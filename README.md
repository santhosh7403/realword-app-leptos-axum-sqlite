A near realworld leptos web app with axum sqlite backend
=======

<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>


This is another Leptos demo application that I worked on as part of my learning Rust/Leptos and carrying out experiments. This one is little more complex  compared to the previously shared one [ demo-tools-app-leptos-07-actix-tailwind ](https://github.com/santhosh7403/demo-tools-app-leptos-07-actix-tailwind). I hope this code may help anyone who is seriously considering Leptos framework in their next project and wants a hands-on or a more realworld working example.

To make it run in few simple steps, made a sqlite version (though, [ recent opinions ](https://dev.to/shayy/everyone-is-wrong-about-sqlite-4gjf) suggest sqlite itself is good enough if your app isn't too many writes ). So to see it, just clone the project and run.

Before proceeding to clone, you may take a look on the [ screenshots here ](https://github.com/santhosh7403/realword-app-leptos-axum-sqlite/blob/main/App_Screenshots.md), that will give a quick good insight into this app and you can decide.

There is a Postgres version of this as well and you can [ find it here ](https://github.com/santhosh7403/realword-app-leptos-axum).

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

`git clone https://github.com/santhosh7403/realword-app-leptos-axum-sqlite.git`

`cd realword-app-leptos-axum-sqlite`

`source .env`  - set the DATABASE_URL env variable

`cargo leptos watch`  or `cargo leptos serve`

Above command expects rust toolchains and cargo-leptos are installed already, if you don't have `cargo-leptos` installed you can install it with

`cargo install cargo-leptos --locked`


By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate` etc. If you run into any trouble, you may need to install one or more of these tools. Please refer [ rustup here ](https://rustup.rs).

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup update` - update the rust toolchains to latest
3. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
4. `cargo install cargo-generate` - install cargo-generate binary

Now you may run the build.

  `cargo leptos watch`  or `cargo leptos serve`

# Application access

Once application started, access application from you web browser [ localhost:3000 ](http://localhost:3000/)


The application screen looks like this 
<img width="1809" height="1030" alt="image" src="https://github.com/user-attachments/assets/0c7d129d-6146-4d91-80f5-4659c58c117b" />


More screenshots are [ available here ](https://github.com/santhosh7403/realword-app-leptos-axum-sqlite/blob/main/App_Screenshots.md)

To showcase the app and test, some sample users and data are populated. User names 'user1' to 'user5' are available and password is same as username. In case if you want to remove this data, you may delete the 'basedata' files inside migrations folder and setup database as explained in [ database readme ](https://github.com/santhosh7403/realword-app-leptos-axum-sqlite/blob/main/README_DATABASE.md).


# Inspiration and Thanks

The base of this app is from [ here ](https://github.com/Bechma/realworld-leptos), though there may be other original versions some where else, not sure.

I initially started this as leptos06 to 07 change in this app (though, above reference repo also seems updated now!) as my learning and got interest to try out more experiments. Overall user interface changed, some with modal windows, tailwindcss and fontawesome icons, re-wired pages, some functionality changes etc. Now looking to add the FTS (full text search) and so on.

