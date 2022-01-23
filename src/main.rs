use dotenv::dotenv;
use std::env;
use mongodb;

mod handlers;

#[derive(Clone, Debug)]
pub struct State{ //para passar conexão com os controladores do db
    db: mongodb::Database,
}

#[async_std::main] //isso torna a main assíncrona
async fn main() -> tide::Result<()> { //tide::result retorna Ok(()) ou um erro
    dotenv().ok(); //ler o .env e adicionar as variáveis do environment

    //criar as opções do cliente MongoDB com a string de conexão das variáveis de environment
    let mongodb_client_options = mongodb::options::ClientOptions::parse(&env::var("MONGODB_URI").unwrap()).await?;
    
    //inicializa o cliente mongodb
    let mongodb_client = mongodb::Client::with_options(mongodb_client_options)?;

    let db = mongodb_client.database("rust-api-exemplo");
    
    let state = State {db};

    let mut app = tide::with_state(state);

    app.at("/hello").get(handlers::hello); //endpoint GET/hello. Rode o programa e dê um curl 127.0.0.1:8080/hello: receberás um hello world
    app.at("/items").get(handlers::get_items);
    app.at("/items").post(handlers::post_item);
    app.at("/clientes").get(handlers::get_cliente);
    app.at("/clientes").post(handlers::post_cliente);
    app.at("/transf_conta").post(handlers::transf_conta);
    app.at("/transf_pix").post(handlers::transf_pix);
    app.listen("127.0.0.1:8080").await?; //ouça na porta 8080

    return Ok(());
}