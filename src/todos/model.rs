use sqlx::Pool;
use sqlx::Postgres;

#[derive(Debug, Serialize)]
pub struct Todo {
    id: i64,
    name: String,
    completed: bool,
}

impl Todo {
    pub(crate) async fn create(name: &str, pool: &Pool<Postgres>) -> Result<Todo, sqlx::Error> {
        let todo = sqlx::query_as!(
            Todo,
            r"INSERT INTO todos (name) VALUES ($1) RETURNING *;",
            name
        )
        .fetch_one(pool)
        .await?;

        Ok(todo)
    }

    pub(crate) async fn find(id: i64, pool: &Pool<Postgres>) -> Result<Option<Todo>, sqlx::Error> {
        let todo: Option<Todo> = sqlx::query_as!(
            Todo,
            r#"SELECT *
            FROM todos
            WHERE id = $1"#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(todo)
    }

    pub(crate) async fn load(pool: &Pool<Postgres>) -> Result<Vec<Todo>, sqlx::Error> {
        let todo: Vec<Todo> = sqlx::query_as!(Todo, r"SELECT * FROM todos",)
            .fetch_all(pool)
            .await?;

        Ok(todo)
    }
}
