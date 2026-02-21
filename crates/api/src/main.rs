//! TravelTrust API 入口：Axum + CORS，路由与 04 §三 对齐
//!
//! SSOT：Backend 启动时从 env SSOT_VERSION 读取；STRICT_SSOT=1 时未设置则拒绝启动。见 08-5 §4、Runbook §10、04 §四。
//! traceId：响应头 x-request-id 由请求头带入或自动生成，与 01 §9 贯通 requestId→txHash→logIndex 一致。
//! 路由：/health、/api/v1/guides 为占位实现；其余为 501 占位，实现时按 04 §三 与 01 §10 17 条（幂等、traceId）补齐。
//! 幂等：请求头 Idempotency-Key / X-Idempotency-Key 在中间件透传并回写；对 POST/PUT 做 key 去重与结果复用（01 §10 #14），缓存键=method+path+key，最多 1000 条。
//! 环境变量：PORT（默认 3000）、CORS_ORIGINS（逗号分隔的允许 origin，未设则开发态允许任意；生产应设置）。

use axum::{
    body::Body,
    extract::Path,
    http::{header::HeaderName, header::HeaderValue, Method, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    routing::{get, post, put},
    Json, Router,
};
use bytes::Bytes;
use http_body_util::BodyExt;
use serde_json::json;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use std::env;
use tokio::sync::RwLock;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::timeout::TimeoutLayer;

const IDEMPOTENCY_CACHE_MAX: usize = 1000;

fn main() {
    if let Err(e) = run() {
        eprintln!("TravelTrust API 启动失败: {}", e);
        std::process::exit(1);
    }
}

#[tokio::main]
async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ssot_version = env::var("SSOT_VERSION").unwrap_or_else(|_| "unset".to_string());
    if env::var("STRICT_SSOT").as_deref() == Ok("1") && ssot_version == "unset" {
        eprintln!("STRICT_SSOT=1: SSOT_VERSION 未设置，拒绝启动");
        std::process::exit(1);
    }
    println!("SSOT_VERSION={} (08-3 约定版本；实现时校验一致后启动)", ssot_version);

    let cors: CorsLayer = match env::var("CORS_ORIGINS") {
        Ok(s) if !s.trim().is_empty() => {
            let origins: Result<Vec<HeaderValue>, _> = s
                .split(',')
                .map(|o| HeaderValue::try_from(o.trim()))
                .collect();
            match origins {
                Ok(list) if !list.is_empty() => CorsLayer::new()
                    .allow_origin(AllowOrigin::list(list))
                    .allow_methods(Any)
                    .allow_headers(Any),
                _ => {
                    eprintln!("CORS_ORIGINS 解析失败或为空，使用允许任意 origin（仅建议用于开发）");
                    CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any)
                }
            }
        }
        _ => CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any),
    };

    let idem_cache: Arc<RwLock<IdempotencyCache>> = Arc::new(RwLock::new(IdempotencyCache::default()));
    let idem_cache_clone = Arc::clone(&idem_cache);

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/v1/guides", get(guides_list_placeholder).post(not_impl_v1))
        .route("/api/v1/guides/:id", get(not_impl_guides_id))
        .route("/api/v1/guides/:id/stake", post(not_impl_v1))
        .route("/api/v1/me", get(not_impl_me).put(not_impl_v1))
        .route("/api/v1/me/stats", get(not_impl_me)) // 04 §三 可选：统计摘要，与 /api/v1/me 二选一或并存
        .route("/api/v1/me/password", put(not_impl_v1))
        .route("/api/v1/orders", get(not_impl_orders).post(not_impl_v1))
        .route("/api/v1/orders/:id", get(not_impl_orders_id))
        .route("/api/v1/orders/:id/accept", post(not_impl_v1))
        .route("/api/v1/orders/:id/cancel", post(not_impl_v1))
        .route("/api/v1/orders/:id/confirm-completion", post(not_impl_v1))
        .route("/api/v1/orders/:id/reviews", get(not_impl_v1).post(not_impl_v1))
        .route("/api/v1/orders/:id/evidence", get(not_impl_evidence).post(not_impl_v1)) // 04 §三 证据路径占位，实现时与 01 §6 定稿
        .route("/api/v1/orders/:id/dispute", post(not_impl_v1))
        .route("/api/v1/disputes", get(not_impl_disputes))
        .route("/api/v1/disputes/:id", get(not_impl_disputes_id))
        .route("/api/v1/disputes/:id/resolve", post(not_impl_v1))
        .route("/auth/register", post(not_impl_auth))
        .route("/auth/login", post(not_impl_auth))
        .route("/auth/logout", post(not_impl_auth))
        .route("/auth/refresh", post(not_impl_auth))
        .route("/auth/verify-email", post(not_impl_auth))
        .route("/auth/forgot-password", post(not_impl_auth))
        .route("/auth/reset-password", post(not_impl_auth))
        .layer(TimeoutLayer::new(Duration::from_secs(30))) // 04 §四 请求超时，实现时可从配置读取
        .layer(RequestBodyLimitLayer::new(1024 * 1024)) // 1MB，与 04 风控一致
        .layer(cors)
        .layer(axum::middleware::from_fn(request_id_layer))
        .layer(axum::middleware::from_fn(move |req, next| idempotency_key_layer(idem_cache_clone.clone(), req, next)))
        .layer(axum::middleware::from_fn(auth_placeholder_layer));

    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("TravelTrust API listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

/// 鉴权占位：当前透传；实现时在此校验 JWT/session，需登录路由返回 401。04 §三 多处「需登录」。
async fn auth_placeholder_layer(req: Request<axum::body::Body>, next: Next<axum::body::Body>) -> Response {
    next.run(req).await
}

#[derive(Default)]
struct IdempotencyCache {
    store: HashMap<String, (StatusCode, Vec<u8>)>,
}

impl IdempotencyCache {
    fn get(&self, k: &str) -> Option<(StatusCode, Vec<u8>)> {
        self.store.get(k).cloned()
    }
    fn insert(&mut self, k: String, v: (StatusCode, Vec<u8>)) {
        if self.store.len() >= IDEMPOTENCY_CACHE_MAX {
            if let Some(first) = self.store.keys().next().cloned() {
                self.store.remove(&first);
            }
        }
        self.store.insert(k, v);
    }
}

/// 幂等（01 §10 #14）：POST/PUT 时按 Idempotency-Key 去重并复用缓存的响应；否则透传并回写 key。
async fn idempotency_key_layer(
    cache: Arc<RwLock<IdempotencyCache>>,
    req: Request<Body>,
    next: Next<Body>,
) -> Response {
    let key = req
        .headers()
        .get("Idempotency-Key")
        .or_else(|| req.headers().get("X-Idempotency-Key"))
        .and_then(|v| v.to_str().ok())
        .map(String::from);
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    let is_write = method == Method::POST || method == Method::PUT;

    if is_write {
        if let Some(ref k) = key {
            let cache_key = format!("{}:{}:{}", method, path, k);
            {
                let guard = cache.read().await;
                if let Some((status, body)) = guard.get(&cache_key) {
                    let req_id = req
                        .headers()
                        .get("x-request-id")
                        .and_then(|v| v.to_str().ok())
                        .map(String::from)
                        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
                    let mut res = (status, Body::from(Bytes::from(body))).into_response();
                    if let (Ok(n1), Ok(v1)) = (
                        HeaderName::try_from("x-request-id"),
                        HeaderValue::try_from(req_id.as_str()),
                    ) {
                        res.headers_mut().insert(n1, v1);
                    }
                    if let (Ok(n2), Ok(v2)) = (
                        HeaderName::try_from("X-Idempotency-Key"),
                        HeaderValue::try_from(k.as_str()),
                    ) {
                        res.headers_mut().insert(n2, v2);
                    }
                    return res;
                }
            }
        }
    }

    let res = next.run(req).await;

    if is_write {
        if let Some(ref k) = key {
            let cache_key = format!("{}:{}:{}", method, path, k);
            let (parts, body) = res.into_parts();
            match BodyExt::collect(body).await {
                Ok(collected) => {
                    let bytes = collected.to_bytes();
                    let status = parts.status;
                    let body_bytes = bytes.to_vec();
                    cache.write().await.insert(cache_key, (status, body_bytes.clone()));
                    let mut out = Response::from_parts(parts, Body::from(Bytes::from(body_bytes)));
                    if let (Ok(n), Ok(v)) = (
                        HeaderName::try_from("X-Idempotency-Key"),
                        HeaderValue::try_from(k.as_str()),
                    ) {
                        out.headers_mut().insert(n, v);
                    }
                    return out;
                }
                Err(_) => {
                    return Response::from_parts(parts, Body::empty());
                }
            }
        }
    } else if let Some(ref k) = key {
        let mut res = res;
        if let (Ok(n), Ok(v)) = (
            HeaderName::try_from("X-Idempotency-Key"),
            HeaderValue::try_from(k.as_str()),
        ) {
            res.headers_mut().insert(n, v);
        }
        return res;
    }

    res
}

/// traceId：与 01 §9 贯通 requestId→txHash→logIndex 一致；响应头 x-request-id 供审计与资损排查。可观测：每请求打印 request_id + path + status（实现时可按 01 §9 SLO 接入结构化日志）。
async fn request_id_layer(req: Request<axum::body::Body>, next: Next<axum::body::Body>) -> Response {
    let id = req
        .headers()
        .get("x-request-id")
        .and_then(|v| v.to_str().ok())
        .map(String::from)
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    let path = req.uri().path().to_string();
    let mut res = next.run(req).await;
    eprintln!("[req] x-request-id={} path={} status={}", id, path, res.status().as_u16());
    if let (Ok(name), Ok(val)) = (
        HeaderName::try_from("x-request-id"),
        axum::http::header::HeaderValue::try_from(id.as_str()),
    ) {
        res.headers_mut().insert(name, val);
    }
    res
}

async fn health() -> &'static str {
    "ok"
}

async fn guides_list_placeholder() -> &'static str {
    "[]"
}

fn not_impl_json(path: &str) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "status": "not_implemented",
            "path": path,
            "doc": "04 §三"
        })),
    )
}

async fn not_impl_me() -> impl IntoResponse {
    not_impl_json("/api/v1/me")
}
async fn not_impl_orders() -> impl IntoResponse {
    not_impl_json("/api/v1/orders")
}
async fn not_impl_orders_id(Path(id): Path<String>) -> impl IntoResponse {
    not_impl_json(&format!("/api/v1/orders/{}", id))
}
async fn not_impl_disputes() -> impl IntoResponse {
    not_impl_json("/api/v1/disputes")
}
async fn not_impl_disputes_id(Path(id): Path<String>) -> impl IntoResponse {
    not_impl_json(&format!("/api/v1/disputes/{}", id))
}
async fn not_impl_guides_id(Path(id): Path<String>) -> impl IntoResponse {
    not_impl_json(&format!("/api/v1/guides/{}", id))
}
async fn not_impl_evidence(Path(id): Path<String>) -> impl IntoResponse {
    not_impl_json(&format!("/api/v1/orders/{}/evidence", id))
}
async fn not_impl_auth() -> impl IntoResponse {
    not_impl_json("/auth/*")
}
async fn not_impl_v1() -> impl IntoResponse {
    not_impl_json("/api/v1/*")
}
