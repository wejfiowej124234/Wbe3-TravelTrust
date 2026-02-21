//! 前端调用 Backend API（04 §三），与 01 §9 业务数据同源一致。
//! 占位：仅 health；实现时补齐 me、orders、guides、disputes、auth，并与 04 接口、幂等/traceId 一致。

/// Backend 基地址；实现时从配置或 env 读取。
pub fn api_base_url() -> String {
    // 开发时默认；生产由 build 或 runtime 注入
    option_env!("VITE_API_BASE_URL")
        .unwrap_or("http://localhost:3000")
        .to_string()
}

/// GET /health
pub async fn get_health() -> Result<String, String> {
    let url = format!("{}/health", api_base_url());
    gloo_net::http::Request::get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())
}

/// GET /api/v1/me — 占位，实现时返回用户与统计（04 §三 3.2）。
pub async fn get_me() -> Result<String, String> {
    let url = format!("{}/api/v1/me", api_base_url());
    gloo_net::http::Request::get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())
}
