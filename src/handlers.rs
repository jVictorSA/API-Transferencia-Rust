//declaração da dependência
use actix_web::Responder;

//Funções que representam requisições HTTP.
//Atualmente só imprimem no terminal quando
//requisitadas.
pub async fn get_users() -> impl Responder{
    format!("hello from users")
}

pub async fn get_users_id() -> impl Responder{
    format!("hello from get users by id")
}

pub async fn post_users() -> impl Responder{
    format!("hello from post users")
}

pub async fn delete_user() -> impl Responder{
    format!("hello from delete users")
}