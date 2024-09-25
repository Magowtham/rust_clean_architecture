use async_trait::async_trait;
use diesel::query_dsl::methods::FilterDsl;
use diesel::RunQueryDsl;
use diesel::OptionalExtension;
use std::sync::Arc;
use diesel::ExpressionMethods;

use crate::presentation::handlers::user_handler::NewUser;
use crate::domain::entities::user::User;
use crate::domain::repositories::user_repository::UserRepository;
use crate::infrastructure::db::connection::DBPool;
use crate::infrastructure::db::connection::establish_connection;
use crate::schema;
use crate::schema::users::dsl::users;
use crate::schema::users::email;



#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: DBPool
}

impl PostgresUserRepository {
    pub fn new() -> Self {
        let database_url=std::env::var("DATABASE_URL")
            .expect("Missing DATABASE_URL");

        PostgresUserRepository {
            pool:establish_connection(&database_url)
        }
    }
}

#[async_trait]
impl UserRepository for Arc<PostgresUserRepository> {
    async  fn find_by_email(&self,input_email: String) -> Option<User> {

        users.filter(email.eq(input_email))
            .first::<User>(&mut self.pool.get().unwrap())
            .optional()
            .expect("Error loading user")
            
    }

    async fn save(&self,user: &NewUser) -> Result<(),diesel::result::Error> {
        diesel::insert_into(schema::users::table)
            .values(user)
            .execute(& mut self.pool.get().unwrap())?;

        Ok(())
    }
}
