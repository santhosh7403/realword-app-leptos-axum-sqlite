// use crate::app::{GlobalState, GlobalStateStoreFields};
use crate::routes::profile_home::Profile;
use crate::routes::user_home::HomePage;
use leptos::prelude::*;
use leptos_meta::*;
// use leptos_router::nested_router::Outlet;
// use reactive_stores::Store;

/// Renders the home page of your application.
#[component]
pub fn HomeMain(username: crate::auth::UsernameSignal, user_profile: bool) -> impl IntoView {
    tracing::debug!("Starting HomePage component");
    // let global_state = expect_context::<Store<GlobalState>>();

    view! {
        <div class="mx-auto max-w-7xl sm:px-6 lg:px-8 bg-gray-200 px-2 py-2 sm:px-0">
            // <Show when=move || global_state.search_window().get()>
                 // <Outlet />
            // </Show>
            // <Show when=move || {
            //     leptos::logging::log!("search_window is  {}", global_state.search_window().get());
            //     !global_state.search_window().get()
            // }>
                <Show
                    when=move || !user_profile
                    fallback=move || {
                        view! {
                            <Transition fallback=move || view! { <p>"Loading data..."</p> }>
                                <Profile username />
                            </Transition>
                        }
                    }
                >
                    <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                        <Title text="Home" />
                        <HomePage username />
                    </Suspense>
                </Show>
            // </Show>
        </div>
    }
}
