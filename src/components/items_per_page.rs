use crate::app::{GlobalState, GlobalStateStoreFields};
use leptos::prelude::*;
use leptos_router::hooks::{use_params_map, use_query, use_query_map};
use reactive_stores::Store;

#[component]
pub fn ItemsPerPage() -> impl IntoView {
    let params = use_params_map();
    let route_user = move || params.with(|x| x.get("user").unwrap_or_default());
    let query = use_query_map();
    let favourite = move || query.with(|x| x.get("favourites").map(|_| true));
    let global_state = expect_context::<Store<GlobalState>>();

    let per_page: RwSignal<Option<u32>> =
        use_context().expect("per_page context should be available");

    view! {
        <div class="">
            <label for="articlesPerPage" class="text-gray-700 px-1">
                "Items Per Page"
            </label>
            <select
                id="articlesPerPage"
                class="focus:shadow-outline rounded border px-1 py-1 leading-tight text-gray-700 shadow focus:outline-none"
                on:change:target=move |ev| {
                    per_page.set(Some(ev.target().value().parse::<u32>().unwrap()));
                    let pagination = use_query::<crate::models::Pagination>();
                    let navigate = leptos_router::hooks::use_navigate();
                    let page_url = format!(
                        "{}{}{}",
                        if global_state.is_profile().get() {
                            format!("/profile/{}", route_user())
                        } else {
                            "".to_string()
                        },
                        pagination
                            .get()
                            .unwrap_or_default()
                            .reset_page()
                            .set_amount(per_page.get().unwrap())
                            .to_string(),
                        if favourite().unwrap_or_default() { "&favourites=true" } else { "" },
                    );
                    navigate(&page_url, Default::default());
                    global_state.back_url().set(page_url);
                }
            >
                <option value=1 selected=move || per_page.get() == Some(1)>
                    "1"
                </option>
                <option value=5 selected=move || per_page.get() == Some(5)>
                    "5"
                </option>
                <option value=10 selected=move || per_page.get() == Some(10)>
                    "10"
                </option>
                <option value=20 selected=move || per_page.get() == Some(20)>
                    "20"
                </option>
                <option value=100 selected=move || per_page.get() == Some(100)>
                    "100"
                </option>
            </select>
        </div>
    }
}
