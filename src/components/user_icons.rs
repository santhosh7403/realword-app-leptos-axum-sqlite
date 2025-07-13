use crate::models::{Comment, User};

use super::article_preview::ArticleSignal;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn AuthorUserIcon(article_signal: ArticleSignal) -> impl IntoView {
    let profile_ref = move || {
        format!(
            "/profile/{}",
            article_signal.with_untracked(|x| x.author.username.to_string())
        )
    };

    view! {
        <div class="flex gap-4 text-gray-600">
            <A href=profile_ref>
                <img
                    src=move || {
                        article_signal
                            .with_untracked(|x| { x.author.image.clone().unwrap_or_default() })
                    }
                    class="w-10 h-10 rounded-full"
                />
            </A>
            <div class="flex items-center">
                <A href=profile_ref>
                    <i class="fa-solid fa-user w-4 h-4"></i>
                    <span class="font-medium">
                        {move || {
                            article_signal.with_untracked(|x| x.author.username.to_string())
                        }}
                    </span>
                </A>
            </div>
        </div>
    }
}

#[component]
pub fn CommentUserIcon(comment: RwSignal<Comment>) -> impl IntoView {
    let user_link = move || {
        format!(
            "/profile/{}",
            comment.with_untracked(|x| x.username.to_string())
        )
    };
    let user_image = move || comment.with_untracked(|x| x.user_image.clone().unwrap_or_default());

    view! {
        <div class="flex gap-4 text-gray-600">
            <div>
                <A href=user_link>
                    <img src=user_image class="w-10 h-10 rounded-full" />
                </A>
            </div>
            <div>
                <i class="fa-solid fa-user w-4 h-4"></i>
                <A href=user_link>
                    <span class="font-medium">
                        {move || comment.with(|x| x.username.to_string())}
                    </span>
                </A>
            </div>
        </div>
    }
}

#[component]
pub fn CurrentUserIcon(user_signal: RwSignal<Option<User>>) -> impl IntoView {
    let user_link = move || {
        format!(
            "/profile/{}",
            user_signal
                .get_untracked()
                .as_ref()
                .map(|u| { u.username().clone() })
                .unwrap_or_default() // .username()
        )
    };

    let user_image = user_signal
        .get_untracked()
        .as_ref()
        .map(|u| u.image().unwrap_or_default())
        .unwrap_or_default();

    view! {
        <div class="flex gap-4 text-gray-600">
            <div>
                <A href=user_link>
                    <img src=user_image class="w-10 h-10 rounded-full" />
                </A>
            </div>
            <div>
                <i class="fa-solid fa-user w-4 h-4"></i>
                <A href=user_link>
                    <span class="font-medium">
                        {move || {
                            user_signal
                                .get_untracked()
                                .as_ref()
                                .map(|u| { u.username().clone() })
                                .unwrap_or_default()
                        }}
                    </span>
                </A>
            </div>

        </div>
    }
}
