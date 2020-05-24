mod database;
mod graphql;
use database::Postgres;

use anyhow::Result;
use async_graphql::http::playground_source;
use async_std::task;
use graphql::schema::{AppState, create_schema};
use sqlx::PgPool;
use std::env;
use tide::{
    http::{headers, mime},
    Request, Response, StatusCode,
};

fn main() -> Result<()> {
    task::block_on(run())
}

async fn graphql(req: Request<AppState>) -> tide::Result<Response> {
    let schema = req.state().schema.clone();
    async_graphql_tide::graphql(req, schema, |query_builder| query_builder).await
}

async fn run() -> Result<()> {
    let pool = PgPool::new("postgres://postgres:postgres@localhost:5432/postgres").await?;

    let database = Postgres::new(pool);
    let schema = create_schema(database);
    let state = AppState { schema };
    let mut app = tide::with_state(state);
    app.at("/graphql").post(graphql).get(graphql);
    app.at("/").get(|_| async move {
        let resp = Response::new(StatusCode::Ok)
            .body_string(playground_source("/graphql", None))
            .set_header(headers::CONTENT_TYPE, mime::HTML.to_string());

        Ok(resp)
    });

    let listen_addr = env::var("LISTEN_ADDR").unwrap_or_else(|_| "localhost:8080".to_owned());
    println!("Playground: http://{}", listen_addr);
    app.listen(listen_addr).await?;
    Ok(())
}
