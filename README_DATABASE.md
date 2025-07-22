
# Setup Database

Let us add the sqlx-cli command-line utility that will help us to drop/create/reset the DB in the DATABASE_URL string inside .env file. It is important to set DATABASE_URL ENV variable before running sqlx commands below as it operates on the value of it. It will be set by running  `source .env` command from the project root folder.

To install sqlx-cli, run below command. However, it all expects you have rust toolchains installed already,if not, refer the Rust toolchain section in main readme to install.

`cargo install sqlx-cli`  - this installs sqlx utility

Now from the root folder you can run below commands to create a DB and run the initialize sql scripts inside the 'migrations' folder.

`cd realworld-app-leptos-axum`

`source .env`

`sqlx database setup` - DB create and run the migrations
