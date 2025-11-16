use crate::auth::*;
use leptos::prelude::*;
use leptos_router::components::*;

#[component]
pub(crate) fn NavItems(
    login: ServerAction<LoginAction>,
    logout: ServerAction<LogoutAction>,
    username: UsernameSignal,
) -> impl IntoView {
    let profile_label = move || username.get().unwrap_or_default();
    let profile_href = move || format!("/profile/{}", profile_label());

    let navigate_login = move |_| {
        let navigate = leptos_router::hooks::use_navigate();
        navigate("/login", Default::default());
    };

    view! {
        <div class="bg-gray-800 text-white shadow-lg md:relative md:top-0 md:left-0 md:right-auto md:w-full
        rounded-b-xl px-4 py-3 md:py-4">
            <div class="flex justify-around items-center">
                <A href="/">
                    <div class="group navitem">
                        <i class="fas fa-home navitem-icon"></i>
                        <span class="text-xs md:text-base mt-1 font-semibold">Home</span>
                    </div>
                </A>
                <Show
                    when=move || username.with(Option::is_none)
                    fallback=move || {
                        view! {
                            <A href="/editor">
                                <div class="group navitem">
                                    <i class="fa-solid fa-square-plus navitem-icon"></i>
                                    <span class="text-xs md:text-base mt-1 font-semibold">
                                        New Article
                                    </span>
                                </div>
                            </A>

                            <A href="/settings">
                                <div class="group navitem">
                                    <i class="fa-solid fa-gear navitem-icon"></i>
                                    <span class="text-xs md:text-base mt-1 font-semibold">
                                        Settings
                                    </span>
                                </div>
                            </A>
                            <A href=profile_href.clone()>
                                <div class="group navitem">
                                    <i class="fa-regular fa-circle-user navitem-icon"></i>
                                    <span class="text-xs md:text-base mt-1 font-semibold">
                                        {profile_label}
                                    </span>
                                </div>
                            </A>

                            <ActionForm action=logout>
                                <button
                                    class="items-center border-none bg-transparent
                                    focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                                    on:click=move |_| login.clear()
                                >
                                    <div class="group navitem">
                                        <i class="fa-solid fa-right-from-bracket navitem-icon"></i>
                                        <span class="text-xs md:text-base mt-1 font-semibold">
                                            Logout
                                        </span>
                                    </div>
                                </button>
                            </ActionForm>
                        }
                    }
                >
                    <A href="/signup">
                        <div class="group navitem">
                            <i class="fa-solid fa-user-plus navitem-icon"></i>
                            <span class="text-xs md:text-base mt-1 font-semibold">Sign up</span>
                        </div>
                    </A>

                    <button
                        on:click=navigate_login
                        class="focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                    >
                        <div class="group navitem">
                            <i class="fa-solid fa-right-to-bracket navitem-icon"></i>
                            <span class="text-xs md:text-base mt-1 font-semibold">Login</span>
                        </div>
                    </button>
                </Show>
            </div>
        </div>
    }
}
