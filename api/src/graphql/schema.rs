use super::types::{Issue, NewIssue, NewProject, Project};
use crate::database::Postgres;
use async_graphql::{Context, EmptySubscription, FieldResult, Schema};

pub struct Query;

#[async_graphql::Object]
impl Query {
    async fn issue(&self, ctx: &Context<'_>, id: i32) -> FieldResult<Option<Issue>> {
        let database: &Postgres = ctx.data();
        let issue = database.find_issue(id).await?;
        Ok(issue)
    }
}

pub struct Mutation;

#[async_graphql::Object]
impl Mutation {
    async fn create_issue(&self, ctx: &Context<'_>, new_issue: NewIssue) -> FieldResult<Issue> {
        let database: &Postgres = ctx.data();
        let issue = database.insert_issue(new_issue).await?;
        Ok(issue)
    }

    async fn create_project(
        &self,
        ctx: &Context<'_>,
        new_project: NewProject,
    ) -> FieldResult<Project> {
        let database: &Postgres = ctx.data();
        let project = database.insert_project(new_project).await?;
        Ok(project)
    }
}

pub struct AppState {
    pub schema: Schema<Query, Mutation, EmptySubscription>,
}

pub fn create_schema(database: Postgres) -> Schema<Query, Mutation, EmptySubscription> {
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(database)
        .finish();
    schema
}
