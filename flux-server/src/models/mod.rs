use crate::models::user::User;
use crate::traits::db_provider::DbProvider;

pub mod user;

pub async fn create_models<P: DbProvider>(provider: &P) {
    let user_schema = User::schema();
    let user_create_sql = user_schema.to_create_table_sql();
    provider.execute_query(&user_create_sql).await.unwrap();

    provider.debug_tables().await.unwrap();
}
