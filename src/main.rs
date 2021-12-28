use async_std::stream::StreamExt;
use dotenv::dotenv;
use mongodb::bson::doc;
use serde::{Deserialize,Serialize};
use std::env;
use tide::{Body, Request, Response, StatusCode};

//mod hellos;

#[derive(Clone)]
pub struct State{
    db: mongodb::Database,
}

pub async fn hello(_req: Request<State>) -> tide::Result{
    return Ok("Hello world".into());
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Item{
    pub name: String,
    pub price: f32,
}

#[derive(Deserialize, Serialize)]
pub struct Cliente{
    nome: String,
    conta: i32,
    saldo: f64,
    pix: i32,
}

async fn post_item(mut req: Request<State>) -> tide::Result{
    let item = req.body_json::<Item>().await?;

    let db = &req.state().db;

    let items_collection = db.collection_with_type::<Item>("items");

    items_collection
        .insert_one(
            Item{
                name: item.name,
                price: item.price,
            },
            None,
        ).await?;

    return Ok(Response::new(StatusCode::Ok));
}

async fn get_items(req: Request<State>) -> tide::Result<tide::Body>{
    let db = &req.state().db;

    let items_collection = db.collection_with_type::<Item>("items");

    let mut cursor = items_collection
            .find(None, None).await?;

    let mut data = Vec::<Item>::new();

    while let Some(result) = cursor.next().await{
        if let Ok(item) = result{
            data.push(item);
        }
    }

    return Body::from_json(&data);
}


#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv().ok();

    let mongodb_client_options = 
        mongodb::options::ClientOptions::parse(&env::var("MONGODB_URI").unwrap()).await?;

    let mongodb_client = mongodb::Client::with_options(mongodb_client_options)?;

    let db = mongodb_client.database("rust-api-example");

    let state = State{db};

    let mut app = tide::with_state(state);
    app.at("/hello").get(hello);
    app.at("/items").get(get_items);
    app.at("/items").post(post_item);
    app.listen("127.0.0.1:8080").await?;

    return Ok(());
}
