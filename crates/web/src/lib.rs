//! TravelTrust 前端入口（Yew / WASM），与 05/06、04 §三 对齐。
//! 路由：yew-router（05 §三）；首页、auth、me、orders、disputes；api 模块占位；实现时按 05 §四§五、01 §7 钱包签与 04 接口补齐。

mod api;
mod pages;

use pages::{PageAuth, PageDisputes, PageHome, PageMe, PageOrders};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Routable)]
enum AppRoute {
    #[at("/")]
    Home,
    #[at("/auth")]
    Auth,
    #[at("/me")]
    Me,
    #[at("/orders")]
    Orders,
    #[at("/disputes")]
    Disputes,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Home => html! { <PageHome /> },
        AppRoute::Auth => html! { <PageAuth /> },
        AppRoute::Me => html! { <PageMe /> },
        AppRoute::Orders => html! { <PageOrders /> },
        AppRoute::Disputes => html! { <PageDisputes /> },
        AppRoute::NotFound => html! { <h2>{ "404" }</h2> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <main class="app">
                <header>
                    <h1>{ "TravelTrust" }</h1>
                    <nav>
                        <Link<AppRoute> to={AppRoute::Home}>{ "首页" }</Link<AppRoute>>
                        <Link<AppRoute> to={AppRoute::Auth}>{ "登录/注册" }</Link<AppRoute>>
                        <Link<AppRoute> to={AppRoute::Me}>{ "个人中心" }</Link<AppRoute>>
                        <Link<AppRoute> to={AppRoute::Orders}>{ "订单" }</Link<AppRoute>>
                        <Link<AppRoute> to={AppRoute::Disputes}>{ "争议" }</Link<AppRoute>>
                    </nav>
                </header>
                <Switch<AppRoute> render={switch} />
            </main>
        </BrowserRouter>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
