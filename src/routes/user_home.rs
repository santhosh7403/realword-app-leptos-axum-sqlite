use crate::app::{GlobalState, GlobalStateStoreFields};
use crate::models::Pagination;
use leptos::{html::Input, prelude::*};
use leptos_meta::*;
use leptos_router::{
    components::A,
    hooks::{use_location, use_query},
    params::ParamsError,
};
use reactive_stores::Store;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct MatchedArticles {
    pub slug: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}

#[server(SearchAction)]
async fn fetch_results(search: String) -> Result<Vec<MatchedArticles>, ServerFnError> {
    let page = 0;
    let amount = 10;
    let offset = page * amount;
    // if search.is_empty() {
    //     Err(ServerFnError::new("Empty search string, hence ignore"))
    // } else {
    //     Ok(
    // crate::models::MatchedArticles::search_articles(search, 0, 10)
    //     .await
    //     .map_err(|x| {
    //         tracing::error!("problem while fetching search articles: {x:?}");
    //         ServerFnError::new("Problem while fetching search articles")
    //     })?,

    sqlx::query!(
            // MatchedArticles,
            r#"
SELECT distinct
a.slug as slug,
snippet(articles_fts,1, '<span class="bg-yellow-300">','</span>','<span class="bg-yellow-300">  ...  </span>',10) as "title: String",
snippet(articles_fts,2, '<span class="bg-yellow-300">','</span>','<span class="bg-yellow-300">  ...  </span>',20) as "description: String",
snippet(articles_fts,3, '<span class="bg-yellow-300">','</span>','<span class="bg-yellow-300">  ...  </span>',20) as "body: String"
FROM Articles_fts AS AFTS
JOIN  Articles AS A  ON A.oid = AFTS.rowid
WHERE Articles_fts MATCH $3
order by rank
LIMIT $1 OFFSET $2"#,
            amount,
            offset,
            search,
        )
        .map(|x| MatchedArticles {
            slug: x.slug,
            title: x.title,
            description: x.description,
            body: x.body,
        })
        .fetch_all(crate::database::get_db())
        .await
        .map_err(|e| ServerFnError::new(format!("Some problem occured in sqlite query - {}", e.to_string())))
    // )
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

    // let search_string: NodeRef<Input> = NodeRef::new();
    // let search_param = RwSignal::new(String::new());
    // let search = move || query.read().get("q").unwrap_or_default();
    let run_search = ServerAction::<SearchAction>::new();
    // let on_search = move || {
    //     search_param.set(
    //         search_string
    //             .get()
    //             .expect("search <Input> has to exist")
    //             .value(),
    //     );
    // };

    // let search_results = OnceResource::new(fetch_results(search));
    // let search_results = Resource::new(search, |s| fetch_results(s));
    // let search_results = LocalResource::new(move || fetch_results(search.get()));
    Effect::new(move || {
        if run_search.value().get().is_some() && !run_search.pending().get() {
            global_state.search_window().set(true);
            leptos::logging::log!("setting window true");
        } else {
            global_state.search_window().set(false);
            leptos::logging::log!("setting window false");
        }
    });

    view! {
        <Title text="Home" />
        <div class="mx-auto max-w-7xl sm:px-6 lg:px-8 bg-gray-200 px-2 py-2 sm:px-0">
            <div class="">
                <div class="flex justify-between">
                    <div>
                        <YourFeedTab username pagination />
                        <GlobalFeedTab pagination />
                    </div>
                    <div>
                        <SearchArticle run_search />
                    // <SearchClear run_search />
                    </div>
                    <ItemsPerPage />
                </div>
                // <Show when=move || global_state.search_window().get()>

                // <div class="flex justify-end">
                // <div class="w-3/5">
                // <div class="mb-2 relative flex justify-end">
                // <input
                // node_ref=search_string
                // class="shadow appearance-none bg-white border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                // name="search"
                // type="text"
                // value=move || { String::new() }
                // placeholder="Search string"
                // required=true
                // />
                // <span class="absolute inset-y-0 right-8 flex items-center pr-3">
                // <i class="fas fa-magnifying-glass"></i>
                // </span>
                // <button
                // class="px-2 cursor-pointer"
                // type="button"
                // on:click=move |_| on_search()
                // >
                // Go
                // </button>
                // </div>
                // </div>
                // </div>
                // <Show when=move || !search_param.get().is_empty()>
                // // <SearchOut article_out=search_results />
                // <SearchResults search_param />
                // </Show>
                // </Show>
                <Show when=move || global_state.search_window().get()>
                    <SearchResults run_search />
                </Show>
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
fn SearchResults(
    // search_param: RwSignal<String>,
    run_search: ServerAction<SearchAction>,
) -> impl IntoView {
    // let results = fetch_results(search.get());
    // let articles_out = LocalResource::new(move || fetch_results(search_param.get()));
    //
    let articles_out = run_search.value().get_untracked();

    // #[component]
    // fn SearchOut(
    //     article_out: Resource<Result<Vec<crate::models::MatchedArticles>, ServerFnError>>,
    // ) -> impl IntoView {
    // let articles_view = move || {
    //     articles_out.with(move |x| {
    //         x.clone().map(move |res| {
    //             view! {
    //                 <Suspense fallback=move || view! { <p>"Loading..."</p> }>
    //                     <For
    //                         each=move || res.clone().unwrap_or_default().into_iter().enumerate()
    //                         key=|(i, _)| *i
    //                         children=move |(_, article)| {
    //                             view! { <SearchView article_res=article /> }
    //                         }
    //                     />
    //                 </Suspense>
    //             }
    //         })
    //     })
    // };

    let articles_view = articles_out.map(move |res| {
        view! {
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                <For
                    each=move || res.clone().unwrap_or_default().into_iter().enumerate()
                    key=|(i, _)| *i
                    children=move |(_, article)| {
                        view! { <SearchView article_res=article /> }
                    }
                />
            </Suspense>
        }
    });
    view! {
        <Suspense fallback=move || view! { <p>"Loading Search Articles"</p> }>
            <ErrorBoundary fallback=|_| {
                view! { <p class="error-messages text-xs-center">"Something went wrong."</p> }
            }>{articles_view}</ErrorBoundary>
        </Suspense>
    }
}

#[component]
fn SearchView(article_res: MatchedArticles) -> impl IntoView {
    view! {
        <div class="mb-2 p-4 bg-white rounded-lg shadow-md">
            <p>
                <span class="font-bold">"Title: "</span>
                <span inner_html=article_res.title></span>
            </p>
            <p>
                <span class="font-bold">"Description: "</span>
                <span inner_html=article_res.description></span>
            </p>
            <p>
                <span class="font-bold">"Body: "</span>
                <span inner_html=article_res.body></span>
            </p>
            <div class="flex justify-end">
                <A href=move || format!("/article/{}", article_res.slug)>
                    <span class="text-blue-600 underline cursor-pointer">"Read more..."</span>
                </A>
            </div>
        </div>
    }
}

#[component]
fn SearchArticle(run_search: ServerAction<SearchAction>) -> impl IntoView {
    // let query = use_query_map();
    // let run_search = ServerAction::<SearchAction>::new();
    let search_string: NodeRef<Input> = NodeRef::new();
    let (search_param, set_search_param) = signal(String::new());
    let global_state = expect_context::<Store<GlobalState>>();

    let search_in = move |ev| set_search_param(event_target_value(&ev));

    let clear_search = move || {
        run_search.clear();
        set_search_param.set(String::new());
    };

    view! {
        <ActionForm action=run_search>
            <div class="flex justify-end">
                <div class="flex justify-end">
                    <input
                        class="shadow appearance-none bg-white border rounded w-full py-1 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                        type="text"
                        name="search"
                        minlength=2
                        placeholder="Search string"
                        required
                        node_ref=search_string
                        prop:value=move||search_param.get()
                        on:input=search_in
                    />
                    <button class="absolute pr-2 cursor-pointer hover:text-blue-500 transition duration-200 py-0.5">
                        <i class="fas fa-magnifying-glass"></i>
                    </button>
                </div>
                // <input class="px-2 cursor-pointer" type="submit" value="Go" />
                <Show when=move || global_state.search_window().get()>
                    <button
                        class="text-blue-400 hover:underline hover:text-blue-500 transition duration-200 cursor-pointer px-2"
                        on:click=move |_| clear_search()
                    >
                        "Clear Search"
                    </button>
                </Show>
            </div>
        </ActionForm>
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
                global_state.search_tab().set(false);
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
                        }) && !global_state.search_tab().get()
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
                        }) && !global_state.search_tab().get()
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
                global_state.search_tab().set(false);
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
