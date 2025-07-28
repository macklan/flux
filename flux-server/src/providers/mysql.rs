use crate::models::user::User;
use crate::services::user_service::UserService;
use crate::traits::db_provider::DbProvider;
use async_trait::async_trait;
use mysql_async::{prelude::Queryable, Conn, Error, Pool};

pub struct MySQLProvider {
    pub pool: Pool,
}

impl MySQLProvider {
    pub fn new(database_url: &str) -> Self {
        let pool = Pool::new(database_url);
        MySQLProvider { pool }
    }

    pub async fn connect(&self) -> Result<Conn, Error> {
        self.pool.get_conn().await
    }

    pub fn pool(&self) -> &Pool {
        &self.pool
    }
}

#[async_trait]
impl DbProvider for MySQLProvider {
    async fn execute_query(
        &self,
        query: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = self.connect().await?;
        conn.query_drop(query).await?;
        Ok(())
    }

    async fn debug_tables(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = self.connect().await?;
        let tables: Vec<(String,)> = conn.query("SHOW TABLES").await?;
        println!("Tables in DB!");
        for (table_name,) in tables {
            println!(" - {}", table_name);
        }
        Ok(())
    }
}

#[async_trait]
impl UserService for MySQLProvider {
    async fn create_user(
        &self,
        name: String,
        email: String,
        password: String,
    ) -> Result<User, String> {
        let password_hash = User::hash_password(&password);
        let mut conn = self.connect().await.map_err(|e| e.to_string())?;
        let result = conn
            .exec_drop(
                "INSERT INTO users (name, email, password_hash) VALUES (?, ?, ?)",
                (name.clone(), email.clone(), password_hash.clone()),
            )
            .await;
        match result {
            Ok(_) => Ok(User {
                id: 0,
                name,
                email,
                password_hash,
            }), // You can fetch the inserted id if needed
            Err(e) => Err(e.to_string()),
        }
    }

    async fn find_user_by_username(&self, name: &str) -> Option<User> {
        let mut conn = self.connect().await.ok()?;
        let row: Option<(i32, String, String, String)> = conn
            .exec_first(
                "SELECT id, name, email, password_hash FROM users WHERE name = ?",
                (name,),
            )
            .await
            .ok()?;
        row.map(|(id, name, email, password_hash)| User {
            id,
            name,
            email,
            password_hash,
        })
    }
}
