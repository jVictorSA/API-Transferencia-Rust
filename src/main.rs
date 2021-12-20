//declaração da dependência
use actix_web::{web, App, HttpServer,};
//módulo: um contâiner de código
mod handlers;

//função main do nosso servidor
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    //inicializção do servidor
    HttpServer::new(move||{
        //instanciação do Web App
        App::new()
        //rotas do servidor com suas respectivas chamadas à API
        .route("/users", web::get().to(handlers::get_users))
        .route("/users/{id}", web::get().to(handlers::get_users_id))
        .route("/users", web::post().to(handlers::post_users))
        .route("/users/{id}", web::delete().to(handlers::delete_user))
    })
    //IP do servidor (localhost)
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

        
        /*#[get("/")]
        async fn hello() -> impl Responder{
            HttpResponse::Ok().body("Olá server!")
        }
        
        #[post("/echo")]
        async fn echo(req_body: String) -> impl Responder{
            HttpResponse::Ok().body(req_body)
        }
        
        async fn manual_hello() -> impl Responder{
            HttpResponse::Ok().body("Opa, server!")
        }
        
        #[get("/ola")]
        async fn ola () -> impl Responder{
            HttpResponse::Ok().body("Isso aqui é uma requisição")
        }*/