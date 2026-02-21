//! 页面占位，与 05 §四§五、04 §三 对齐；实现时按 05/06 与 api 模块补齐。

use crate::api;
use yew::prelude::*;

#[function_component(PageHome)]
pub fn page_home() -> Html {
    let loading = use_state(|| true);
    let error = use_state(|| Option::<String>::None);
    let health = use_state(|| Option::<String>::None);

    use_effect_with((), move |_| {
        let loading = loading.clone();
        let error = error.clone();
        let health = health.clone();
        wasm_bindgen_futures::spawn_local(async move {
            loading.set(true);
            error.set(None);
            match api::get_health().await {
                Ok(s) => {
                    health.set(Some(s));
                }
                Err(e) => {
                    error.set(Some(e));
                }
            }
            loading.set(false);
        });
        || ()
    });

    let content = if *loading {
        html! { <p>{ "加载中…" }</p> }
    } else if let Some(ref e) = *error {
        html! { <p class="error">{ "API 错误: " }{ e.as_str() }</p> }
    } else if let Some(ref s) = *health {
        html! { <p>{ "Backend 健康: " }{ s.as_str() }</p> }
    } else {
        html! { <p>{ "TravelTrust · 游客 · 导游 · 仲裁" }</p> }
    };

    html! {
        <section>
            <h2>{ "首页" }</h2>
            { content }
        </section>
    }
}

#[function_component(PageAuth)]
pub fn page_auth() -> Html {
    html! {
        <section>
            <h2>{ "登录 / 注册" }</h2>
            <p>{ "占位：04 §三 auth 对接" }</p>
        </section>
    }
}

#[function_component(PageMe)]
pub fn page_me() -> Html {
    html! {
        <section>
            <h2>{ "个人中心" }</h2>
            <p>{ "占位：GET /api/v1/me、04 §三 3.2" }</p>
        </section>
    }
}

#[function_component(PageOrders)]
pub fn page_orders() -> Html {
    html! {
        <section>
            <h2>{ "订单" }</h2>
            <p>{ "占位：04 §三 orders、01 状态机" }</p>
        </section>
    }
}

#[function_component(PageDisputes)]
pub fn page_disputes() -> Html {
    html! {
        <section>
            <h2>{ "争议" }</h2>
            <p>{ "占位：04 §三 disputes" }</p>
        </section>
    }
}
