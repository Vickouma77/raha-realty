use sqlx::PgPool;
use std::env;

pub async fn setup_database() -> PgPool {
    dotenv::dotenv().ok();
    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.unwrap();

    // Run migrations
    sqlx::migrate!("db/migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}

pub async fn teardown_database(pool: PgPool) {
    // Clean up the database after tests
    sqlx::query("DELETE FROM properties")
        .execute(&pool)
        .await
        .expect("Failed to clean up properties table");
}
