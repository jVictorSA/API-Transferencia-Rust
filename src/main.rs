use dotenv::dotenv;
use std::env;
use mongodb;

mod handlers;
mod cliente;
mod exibir;
mod transf;

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

    app.at("/clientes").get(cliente::get_cliente);
    app.at("/registrar").post(cliente::post_cliente);
    app.at("/transferencia/conta").post(transf::transf_conta);
    app.at("/transferencia/pix").post(transf::transf_pix);
    app.at("/exibir/ted/:cpf").get(exibir::exibir_ted);
    app.at("/exibir/pix/:cpf").get(exibir::exibir_pix);
    app.at("/deposito").post(handlers::auto_deposito);

    app.at("/hello").get(handlers::hello);
    
    app.listen("127.0.0.1:8080").await?; //ouça na porta 8080

    return Ok(());
}