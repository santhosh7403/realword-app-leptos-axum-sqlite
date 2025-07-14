<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>


This is another leptos demo application that I worked on as part of my learning rust/leptos and carry out experiments. This one is little more complex  compared to the previously shared one [ demo-tools-app-leptos-07-actix-tailwind ](https://github.com/santhosh7403/demo-tools-app-leptos-07-actix-tailwind). I hope this code may help anyone who is seriously considering leptos framework in their next project and wants a hands-on or see it in more realworld working example.

To make it run in few simple steps, made a sqlite version (though, [ recent opinions ](https://dev.to/shayy/everyone-is-wrong-about-sqlite-4gjf) suggest sqlite itself is good enough if your app isn't too many writes ). So to see it, just clone the project and run.

Before proceeding to clone, you may take a look on the screenshots here, that will give a quick good insight into this app and you can decide.

There is a Postgres version of this as well and you can [ find it here ](https://github.com/santhosh7403/realword-app-leptos-07-axum).

This app includes:<br/>
        Leptos<br/>
        axum<br/>
        SSR<br/>
        sqlite<br/>
        Modal Windows<br/>
        argon2(password encrypt)<br/>
        uuid<br/>
        tailwindcss<br/>
        fontawesome icons<br/>

To test it out, clone the repo and run.

`git clone https://github.com/santhosh7403/realword-app-leptos-07-axum-sqlite.git`

`cd realword-app-leptos-07-axum-sqlite`

`cargo leptos watch`  or `cargo leptos serve`

This expects rust toolchains and cargo-leptos are installed already, if you don't have `cargo-leptos` installed you can install it with

`cargo install cargo-leptos --locked`




By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate` etc. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup update` - update the rust toolchains to latest
3. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
4. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)

Now you may run the build.

    `cargo leptos watch`  or `cargo leptos serve`

# Application access

Once application started, access application from you web browser [ localhost:3000 ](http://localhost:3000/)

To showcase and test, some dummy data is populated already. User names 'user1' to 'user5' are available and password is same as username. In case if you want to remove this data, you may delete the 'basedata' files inside migrations folder before build and run.
