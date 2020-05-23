use anyhow::Result;
use async_graphql::{http::playground_source, Context, EmptySubscription, FieldResult, Schema};
use async_std::task;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::env;
use tide::{
    http::{headers, mime},
    Request, Response, StatusCode,
};

#[async_graphql::InputObject]
struct NewIssue {
    pub title: String,
    pub description: String,
}

#[async_graphql::SimpleObject]
struct Issue {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub category: Option<String>,
}

struct Query;

#[async_graphql::Object]
impl Query {
    async fn issue(&self, ctx: &Context<'_>, id: i32) -> FieldResult<Option<Issue>> {
        let pool: &PgPool = ctx.data();

        let issue = sqlx::query_as!(
            Issue,
            "
                SELECT *
                FROM issue
                WHERE id = $1
            ",
            id
        )
        .fetch_one(pool)
        .await?;
        Ok(Some(issue))
    }
}

struct Mutation;

#[async_graphql::Object]
impl Mutation {
    async fn create_issue(&self, ctx: &Context<'_>, new_issue: NewIssue) -> FieldResult<Issue> {
        let pool: &PgPool = ctx.data();

        let issue = sqlx::query_as!(
            Issue,
            "
                INSERT INTO issue (title, description, created_at)
                VALUES ($1, $2, timezone('UTC', now()))
                RETURNING *
            ",
            &new_issue.title,
            &new_issue.description
        )
        .fetch_one(pool)
        .await?;

        Ok(issue)
    }
}

struct AppState {
    schema: Schema<Query, Mutation, EmptySubscription>,
}

fn main() -> Result<()> {
    task::block_on(run())
}

async fn graphql(req: Request<AppState>) -> tide::Result<Response> {
    let schema = req.state().schema.clone();
    async_graphql_tide::graphql(req, schema, |query_builder| query_builder).await
}

async fn run() -> Result<()> {
    let pool = PgPool::new("postgres://postgres:postgres@localhost:5432/postgres").await?;

    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(pool)
        .finish();

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
