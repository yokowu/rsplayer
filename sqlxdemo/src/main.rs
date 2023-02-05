use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect("postgres://yoko:yoko@localhost/yoko")
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(())
}
