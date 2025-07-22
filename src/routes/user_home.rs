use crate::app::{GlobalState, GlobalStateStoreFields};
use crate::models::Pagination;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    hooks::{use_location, use_query},
    params::ParamsError,
};
use reactive_stores::Store;

use crate::components::{
    article_preview::ArticlePreviewList, items_per_page::ItemsPerPage,
    prev_next_buttons::PreviousNextButton,
};

#[tracing::instrument]
#[server(HomeAction, "/api", "GetJson")]
async fn home_articles(
    page: u32,
    amount: u32,
    tag: String,
    my_feed: bool,
) -> Result<Vec<crate::models::Article>, ServerFnError> {
    let page = i64::from(page);
    let amount = i64::from(amount);

    Ok(
        crate::models::Article::for_home_page(page, amount, tag, my_feed)
            .await
            .map_err(|x| {
                tracing::error!("problem while fetching home articles: {x:?}");
                ServerFnError::new("Problem while fetching home articles")
            })?,
    )
}

#[server(GetTagsAction, "/api", "GetJson")]
async fn get_tags() -> Result<Vec<String>, ServerFnError> {
    sqlx::query!("SELECT DISTINCT tag FROM ArticleTags")
        .map(|x| x.tag)
        .fetch_all(crate::database::get_db())
        .await
        .map_err(|x| {
            tracing::error!("problem while fetching tags: {x:?}");
            ServerFnError::ServerError("Problem while fetching tags".into())
        })
}

/// Renders the home page of your application.
#[component]
pub fn HomePage(username: crate::auth::UsernameSignal) -> impl IntoView {
    let per_page: RwSignal<Option<u32>> =
        use_context().expect("per_page context should be available");
    tracing::debug!("Starting HomePage component");
    let pagination = use_query::<crate::models::Pagination>();

    let show_modal: RwSignal<bool> = use_context().expect("show_modal context should be available");
    show_modal.set(false); // Return from any Modal page should bring the home page front

    let articles = Resource::new(
        move || pagination.get().unwrap_or_default(),
        move |pagination| async move {
            tracing::debug!("making another request: {pagination:?}");
            home_articles(
                pagination.get_page(),
                pagination.get_amount(),
                pagination.get_tag().to_string(),
                pagination.get_my_feed(),
            )
            .await
        },
    );

    let global_state = expect_context::<Store<GlobalState>>();
    global_state.is_profile().set(false);

    let curr_location = format!(
        "{}{}",
        use_location().pathname.get_untracked(),
        if use_location().search.get_untracked().is_empty() {
            "".to_string()
        } else {
            format!("?{}", use_location().search.get_untracked())
        }
    );
    global_state.home_url().set(curr_location);
    global_state
        .back_url()
        .set(global_state.home_url().get_untracked());
    Effect::new(move || {
        global_state
            .home_url()
            .set(pagination.get().unwrap_or_default().to_string());
    });

    Effect::new(move || {
        pagination
            .get()
            .unwrap_or_default()
            .set_amount(per_page.get().unwrap());
    });

    view! {
        <Title text="Home" />
        <div class="mx-auto max-w-7xl sm:px-6 lg:px-8 bg-gray-200 px-2 py-2 sm:px-0">
            <div class="">
                <div class="flex justify-between mb-2">
                    <div>
                        <YourFeedTab username pagination />
                        <GlobalFeedTab pagination />
                    </div>
                    <ItemsPerPage />
                </div>
                <Show when=move || !pagination.get().unwrap_or_default().get_my_feed()>
                    <div class="flex gap-1 rounded bg-white mb-2">
                        <span class="font-bold m-1">Popular Tags:</span>
                        <TagList />
                    </div>
                </Show>
                <Show
                    when=move || {
                        articles
                            .with(|x| {
                                x.as_ref()
                                    .map_or(0, |y| y.as_ref().map(Vec::len).unwrap_or_default())
                            }) != 0
                    }
                    fallback=move || {
                        view! {
                            <div>
                                <p>
                                    {if pagination.get().unwrap_or_default().get_my_feed() {
                                        "You are not following any other user!"
                                    } else {
                                        "No articles to list"
                                    }}
                                </p>
                            </div>
                        }
                    }
                >
                    <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                        <ArticlePreviewList username articles />
                    </Suspense>
                </Show>
            </div>
            <div class="flex gap-4">
                <PreviousNextButton articles />
            </div>
        </div>
    }
}

#[component]
fn YourFeedTab(
    username: RwSignal<Option<String>>,
    pagination: Memo<Result<Pagination, ParamsError>>,
) -> impl IntoView {
    let per_page: RwSignal<Option<u32>> =
        use_context().expect("per_page context should be available");
    let global_state = expect_context::<Store<GlobalState>>();

    view! {
        <button
            type="button"
            disabled=move || username.with(Option::is_none)
            on:click=move |_| {
                let navigate = leptos_router::hooks::use_navigate();
                let your_feed_url = format!(
                    "{}",
                    if username.with(Option::is_some)
                        && !pagination
                            .with(|x| {
                                x.as_ref()
                                    .map(crate::models::Pagination::get_my_feed)
                                    .unwrap_or_default()
                            })
                    {
                        pagination
                            .get()
                            .unwrap_or_default()
                            .reset_page()
                            .set_my_feed(true)
                            .set_amount(per_page.get().unwrap())
                            .to_string()
                    } else {
                        String::from("/")
                    },
                );
                global_state.back_url().set(your_feed_url.clone());
                navigate(&your_feed_url, Default::default());
            }
            class=move || {
                format!(
                    "px-1 m-1 font-bold {}",
                    if username.with(Option::is_none) {
                        "cursor-not-allowed bg-gray-200"
                    } else if pagination
                        .with(|x| {
                            x.as_ref()
                                .map(crate::models::Pagination::get_my_feed)
                                .unwrap_or_default()
                        })
                    {
                        "border-b-8 bg-gray-200"
                    } else {
                        "bg-gray-200 cursor-pointer"
                    },
                )
            }
        >
            "Your Feed"
        </button>
    }
}

#[component]
fn GlobalFeedTab(pagination: Memo<Result<Pagination, ParamsError>>) -> impl IntoView {
    let per_page: RwSignal<Option<u32>> =
        use_context().expect("per_page context should be available");
    let global_state = expect_context::<Store<GlobalState>>();

    view! {
        <button
            class=move || {
                format!(
                    "px-1 m-1 font-bold {}",
                    if !pagination
                        .with(|x| {
                            x.as_ref()
                                .map(crate::models::Pagination::get_my_feed)
                                .unwrap_or_default()
                        })
                    {
                        "border-b-8 bg-gray-200"
                    } else {
                        "bg-gray-200 cursor-pointer"
                    },
                )
            }
            on:click=move |_| {
                let navigate = leptos_router::hooks::use_navigate();
                let global_feed_url = pagination
                    .get()
                    .unwrap_or_default()
                    .reset_page()
                    .set_my_feed(false)
                    .set_amount(per_page.get().unwrap())
                    .to_string();
                global_state.back_url().set(global_feed_url.clone());
                navigate(&global_feed_url, Default::default())
            }
        >
            "Global Feed"
        </button>
    }
}

#[component]
fn TagList() -> impl IntoView {
    let pagination = use_query::<crate::models::Pagination>();
    let tag_list = Resource::new(|| (), |_| async { get_tags().await });

    let tag_view = move || {
        let tag_elected = pagination.with(|x| {
            x.as_ref()
                .map(crate::models::Pagination::get_tag)
                .unwrap_or_default()
                .to_string()
        });
        tag_list.get().map(move |ts| {
            ts.map(move |tags| {
                view! {
                    <For
                        each=move || tags.clone().into_iter().enumerate()
                        key=|(i, _)| *i
                        children=move |(_, t): (usize, String)| {
                            let t2 = t.to_string();
                            let same = t2 == tag_elected;
                            view! {
                                <div class="gap-1">
                                    <a
                                        class=move || {
                                            format!(
                                                "rounded px-1 py-0.5 hover:bg-green-300 {}",
                                                if same { "bg-green-200" } else { "bg-gray-200" },
                                            )
                                        }
                                        href=move || {
                                            if same {
                                                pagination.get().unwrap_or_default().set_tag("").to_string()
                                            } else {
                                                pagination
                                                    .get()
                                                    .unwrap_or_default()
                                                    .set_tag(&t2)
                                                    .reset_page()
                                                    .to_string()
                                            }
                                        }
                                    >
                                        {t}
                                    </a>
                                </div>
                            }
                        }
                    />
                }
            })
        })
    };

    view! {
        <div class="flex gap-1 py-1 flex-wrap">
            <Suspense fallback=move || view! { <p>"Loading Tags"</p> }>
                <ErrorBoundary fallback=|_| {
                    view! { <p class="error-messages text-xs-center">"Something went wrong."</p> }
                }>{tag_view}</ErrorBoundary>
            </Suspense>
        </div>
    }
}
