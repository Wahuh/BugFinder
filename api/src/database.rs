use crate::graphql::types::{Issue, NewIssue, NewProject, Project};
use anyhow::Result;
use sqlx::PgPool;

pub struct Postgres {
    pool: PgPool,
}

impl Postgres {
    pub fn new(pool: PgPool) -> Self {
        Postgres { pool }
    }

    pub async fn find_issue(&self, id: i32) -> Result<Option<Issue>> {
        let issue = sqlx::query_as!(
            Issue,
            "
            SELECT *
            FROM issues
            WHERE id = $1
        ",
            id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(Some(issue))
    }

    pub async fn insert_issue(&self, new_issue: NewIssue) -> Result<Issue> {
        let issue = sqlx::query_as!(
            Issue,
            "
                INSERT INTO issues (title, description, created_at)
                VALUES ($1, $2, timezone('UTC', now()))
                RETURNING *
            ",
            &new_issue.title,
            &new_issue.description
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(issue)
    }

    pub async fn insert_project(&self, new_project: NewProject) -> Result<Project> {
      let project = sqlx::query_as!(
        Project,
        "
          INSERT INTO projects (title, description, created_at)
          VALUES ($1, $2, timezone('UTC', now()))
          RETURNING *
        ",
        &new_project.title,
        &new_project.description
      )
      .fetch_one(&self.pool)
      .await?;

      Ok(project)
    }
}
