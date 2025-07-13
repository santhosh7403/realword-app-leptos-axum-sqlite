use leptos::prelude::RwSignal;
mod api;
#[cfg(feature = "ssr")]
mod server;
pub use api::*;
#[cfg(feature = "ssr")]
pub use server::*;
pub type UsernameSignal = RwSignal<Option<String>>;
