use async_trait::async_trait;

use crate::presentation::handlers::user_handler::NewUser;
use crate::domain::entities::user::User;


#[async_trait]
pub trait UserRepository {
    async fn find_by_email(&self,email: String) -> Option<User>;

    async fn save(&self,user: &NewUser) -> Result<(),diesel::result::Error>;
}