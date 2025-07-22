static DB: std::sync::OnceLock<sqlx::SqlitePool> = std::sync::OnceLock::new();

async fn create_pool() -> sqlx::SqlitePool {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(8)
        .connect("sqlite://realworld-app-leptos.db")
        .await
        .expect("could not connect to database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("migrations failed");

    pool
}

pub async fn init_db() -> Result<(), ()> {
    DB.set(create_pool().await).map_err(|_| ())
}

pub fn get_db() -> &'static sqlx::SqlitePool {
    DB.get().expect("database initialized")
}
