//! TravelTrust 前端入口（Yew / WASM）

use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <main class="app">
            <h1>{ "TravelTrust" }</h1>
            <p>{ "Decentralized Travel Reputation & Escrow" }</p>
            <p>{ "游客 · 导游 · 仲裁" }</p>
        </main>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
