use actix_web::{get, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::Executor;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/artem")]
async fn artem() -> &'static str {
    "Hello Artem!"
}

#[get("/liza")]
async fn liza() -> &'static str {
    "Hello Elizaveta!"
}

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres()] pool: sqlx::PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
        cfg.service(artem);
        cfg.service(liza);
    };

    // initialize the database if not already initialized
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;

    Ok(config.into())
}
