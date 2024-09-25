
use actix_web::{post,get};
use actix_web::web;
use actix_web::HttpResponse;
use diesel::prelude::Insertable;
use log::error;
use serde::Deserialize;

use crate::application::use_cases::get_user::GetUserUseCase;
use crate::application::use_cases::register_user::RegisterUseCase;
use crate::infrastructure::repositories::postgres_user_repository::PostgresUserRepository;
use crate::schema::users;




#[derive(Debug,Clone,Deserialize,Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String
}

#[post("/")]
pub async fn register_user_handler(
    repo: web::Data<PostgresUserRepository>,
    input: web::Json<NewUser>
) -> HttpResponse {

    match RegisterUseCase::new(repo.into_inner())
        .execute(input.into_inner()).await  {

            Ok(_) => HttpResponse::Ok().finish(),
            Err(err) => {
                error!("Error registering user! {:?}",err);
                HttpResponse::InternalServerError().body("failed to register user")
            }

    }
}

#[get("/{email}")]
pub async fn get_by_email(
    repo: web::Data<PostgresUserRepository>,
    path: web::Path<(String,)>
) -> HttpResponse {

    match GetUserUseCase::new(repo.into_inner()).get(path.into_inner().0).await {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("user not found") 
    }

}
