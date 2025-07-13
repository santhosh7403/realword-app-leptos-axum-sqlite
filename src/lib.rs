pub mod app;
pub mod auth;
pub mod components;
#[cfg(feature = "ssr")]
pub mod database;
pub mod models;
pub(crate) mod routes;
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
