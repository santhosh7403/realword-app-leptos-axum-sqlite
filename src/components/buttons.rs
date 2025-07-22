use crate::routes::article_modal::FollowUser;
use leptos::prelude::*;

#[server(FollowAction, "/api")]
#[tracing::instrument]
pub async fn follow_action(other_user: String) -> Result<bool, ServerFnError> {
    let Some(username) = crate::auth::get_username() else {
        return Err(ServerFnError::ServerError(
            "You need to be authenticated".into(),
        ));
    };
    toggle_follow(username, other_user).await.map_err(|x| {
        tracing::error!("problem while updating the database: {x:?}");
        ServerFnError::ServerError("error while updating the follow".into())
    })
}

#[cfg(feature = "ssr")]
#[tracing::instrument]
async fn toggle_follow(current: String, other: String) -> Result<bool, sqlx::Error> {
    let db = crate::database::get_db();
    match sqlx::query!(
        "SELECT * FROM Follows WHERE follower=$1 and influencer=$2",
        current,
        other
    )
    .fetch_one(db)
    .await
    {
        Ok(_) => sqlx::query!(
            "DELETE FROM Follows WHERE follower=$1 and influencer=$2",
            current,
            other
        )
        .execute(db)
        .await
        .map(|_| false),
        Err(sqlx::error::Error::RowNotFound) => sqlx::query!(
            "INSERT INTO Follows(follower, influencer) VALUES ($1, $2)",
            current,
            other
        )
        .execute(db)
        .await
        .map(|_| true),
        Err(x) => Err(x),
    }
}

#[component]
pub fn ButtonFollow(
    logged_user: crate::auth::UsernameSignal,
    author: ReadSignal<String>,
) -> impl IntoView {
    // let following_signal = use_context::<RwSignal<bool>>();
    let following_signal = use_context::<RwSignal<FollowUser>>();

    let follow = ServerAction::<FollowAction>::new();
    let follow_update = move || {
        follow.dispatch(FollowAction {
            other_user: author.get(),
        });

        following_signal
            .map(|fso| fso.update(|fs| fs.0 = !fs.0))
            .expect("following_signal read from context error")
    };

    let (is_hovered, set_is_hovered) = signal(false);

    let button_text = move || {
        format!(
            " {} ",
            if is_hovered.get() {
                "Unfollow"
            } else {
                "Following"
            }
        )
    };

    view! {
        <Show when=move || logged_user.get().unwrap_or_default() != author.get() fallback=|| ()>
        <form>
                <div class="rounded-md">
                    <button
                        class="btn btn-sm btn-outline-secondary"
                        class=(
                            "text-yellow-500",
                            move || !is_hovered.get() && following_signal.get().unwrap_or(FollowUser(false)).0,
                        )
                        class=(
                            "text-yellow-400",
                            move || is_hovered.get() && !following_signal.get().unwrap_or(FollowUser(false)).0,
                        )
                        disabled=move||follow.pending().get()
                        on:click=move|_|follow_update()
                        on:mouseenter=move |_| set_is_hovered(true)
                        on:mouseleave=move |_| set_is_hovered(false)
                    >
                        <Show
                            when=move || following_signal.get().unwrap_or(FollowUser(false)).0
                            fallback=|| {
                                view! {
                                    <i class="fa-solid fa-person-circle-plus w-4 h-4"></i>
                                    " Follow "
                                }
                            }
                        >
                            <i class=move || {
                                format!(
                                    "{}",
                                    if is_hovered.get() {
                                        "fa-solid fa-person-circle-minus w-4 h-4"
                                    } else {
                                        ""
                                    },
                                )
                            }></i>
                            {button_text}
                        </Show>
                        {move || author.get()}
                    </button>
                </div>
            </form>
        </Show>
    }
}

#[server(FavAction, "/api")]
#[tracing::instrument]
pub async fn fav_action(slug: String) -> Result<bool, ServerFnError> {
    let Some(username) = crate::auth::get_username() else {
        return Err(ServerFnError::ServerError(
            "You need to be authenticated".into(),
        ));
    };
    toggle_fav(slug, username).await.map_err(|x| {
        tracing::error!("problem while updating the database: {x:?}");
        ServerFnError::ServerError("error while updating the follow".into())
    })
}

#[cfg(feature = "ssr")]
#[tracing::instrument]
async fn toggle_fav(slug: String, username: String) -> Result<bool, sqlx::Error> {
    let db = crate::database::get_db();
    match sqlx::query!(
        "SELECT * FROM FavArticles WHERE article=$1 and username=$2",
        slug,
        username
    )
    .fetch_one(db)
    .await
    {
        Ok(_) => sqlx::query!(
            "DELETE FROM FavArticles WHERE article=$1 and username=$2",
            slug,
            username
        )
        .execute(db)
        .await
        .map(|_| false),
        Err(sqlx::error::Error::RowNotFound) => sqlx::query!(
            "INSERT INTO FavArticles(article, username) VALUES ($1, $2)",
            slug,
            username
        )
        .execute(db)
        .await
        .map(|_| true),
        Err(x) => Err(x),
    }
}

#[component]
pub fn ButtonFav(
    username: crate::auth::UsernameSignal,
    article: super::article_preview::ArticleSignal,
) -> impl IntoView {
    let make_fav = ServerAction::<FavAction>::new();

    let change_fav = move || {
        make_fav.dispatch(FavAction {
            slug: article.with(|x| x.slug.to_string()),
        });
    };

    let result_make_fav = make_fav.value();
    let fav_count = move || {
        if let Some(x) = result_make_fav.get() {
            match x {
                Ok(result) => {
                    article.update(move |x| {
                        x.fav = !x.fav;
                        x.favorites_count =
                            (x.favorites_count + if result { 1 } else { -1 }).max(0);
                    });
                }
                Err(err) => {
                    tracing::error!("problem while fav {err:?}");
                }
            }
        }
        article.with(|x| x.favorites_count)
    };

    let non_zero = move || article.with(|x| x.favorites_count) != 0;

    view! {
        <Show
            when=move || username.with(Option::is_some)
            fallback=move || {
                view! {
                    <button>
                        <Show
                            when=move || non_zero()
                            fallback=move || {
                                view! {
                                    <i class="far fa-star"></i>
                                    " Favorites: "
                                    {fav_count}
                                }
                            }
                        >
                            <span class="text-yellow-500">
                                <i class="fas fa-star"></i>
                                " Favorites: "
                                {fav_count}
                            </span>
                        </Show>
                    </button>
                }
            }
        >
            <div class="flex items-center gap-2">
                <form>
                    <button
                        class=(
                            [
                                "text-gray-600",
                                "hover:text-yellow-500",
                                "transition-colors",
                                "duration-200",
                            ],
                            move || {
                                username.get().unwrap()
                                    != article.with(|x| x.author.username.clone())
                            },
                        )
                        disabled=move || {
                            username.get().unwrap() == article.with(|x| x.author.username.clone())
                            || make_fav.pending().get()
                        }
                        on:click=move|_| change_fav()
                    >

                        <Show
                            when=move || article.with(|x| x.fav)
                            fallback=move || {
                                view! {
                                    <span class:cursor-not-allowed=move || {
                                        username.with(Option::is_some)
                                            && username.get().unwrap()
                                                == article.with(|x| x.author.username.clone())
                                    }>
                                        <i class="far fa-star"></i>
                                        " My Favorite"
                                    </span>
                                }
                            }
                        >
                            <span class="text-yellow-500 hover:text-gray-500 transition-colors duration-200">
                                <i class="fas fa-star"></i>
                                " My Favorite"
                            </span>
                        </Show>
                    </button>
                    <span class="px-8" class=(["text-yellow-500"], move || non_zero())>
                        " Favourites: "
                        {fav_count}
                    </span>
                </form>
            </div>
        </Show>
    }
}
