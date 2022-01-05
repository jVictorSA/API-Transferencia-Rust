use async_std::stream::StreamExt;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use tide::{Body, Request, Response, StatusCode};
use crate::State;

pub async fn hello(_req: Request<State>) -> tide::Result {
    return Ok("Hello, world!".into()) //into() transforma a str em tide::Result
}

#[derive(Debug, Serialize, Deserialize)]
// The request's body structure
pub struct Item {
  pub name: String,
  pub price: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cliente {
    pub nome: String,
    pub num: i32, //id
    pub cpf: String,
    pub conta: String,
    pub saldo: f64,
    pub pix: String,
}

pub async fn get_cliente(req: Request<State>) -> tide::Result<tide::Body> {
    
    let db = &req.state().db;
  
    let clientes_collection = db.collection_with_type::<Cliente>("clientes");
  
  
    let mut cursor = clientes_collection.find(None, None).await?;
  
    // Create a new empty Vector of Item
    let mut data = Vec::<Cliente>::new();

    while let Some(result) = cursor.next().await {
      if let Ok(cliente) = result {
        data.push(cliente);
      }
    }
  
    // Send the response with the list of items
    return Body::from_json(&data);
}

pub async fn post_cliente(mut req: Request<State>) -> tide::Result {
    
    let cliente = req.body_json::<Cliente>().await?;
  
    let db = &req.state().db;
  
    let clientes_collection = db.collection_with_type::<Cliente>("clientes");
  
    
    clientes_collection
      .insert_one(
        Cliente {
          nome: cliente.nome,
          num: cliente.num,
          cpf: cliente.cpf,
          conta: cliente.conta,
          saldo: cliente.saldo,
          pix: cliente.pix,
        },
        None,
      )
      .await?;
  
    return Ok(Response::new(StatusCode::Ok));
}
  

pub async fn post_item(mut req: Request<State>) -> tide::Result {
  // Read the request's body and transform it into a struct
  let item = req.body_json::<Item>().await?;

  // Recover the database connection handle from the Tide state
  let db = &req.state().db;

  // Get a handle to the "items" collection
  let items_collection = db.collection_with_type::<Item>("items");

  // Insert a new Item in the "items" collection using values
  // from the request's body
  items_collection
    .insert_one(
      Item {
        name: item.name,
        price: item.price,
      },
      None,
    )
    .await?;

  // Return 200 if everything went fine
  return Ok(Response::new(StatusCode::Ok));
}

pub async fn get_items(req: Request<State>) -> tide::Result<tide::Body> {
    // Recover the database connection handle from the Tide state
    let db = &req.state().db;
  
    // Get a handle to the "items" collection
    let items_collection = db.collection_with_type::<Item>("items");
  
    // Find all the documents from the "items" collection
    let mut cursor = items_collection.find(None, None).await?;
  
    // Create a new empty Vector of Item
    let mut data = Vec::<Item>::new();


    // Loop through the results of the find query
    while let Some(result) = cursor.next().await {
      // If the result is ok, add the Item in the Vector
      if let Ok(item) = result {
        data.push(item);
      }
    }
  
    // Send the response with the list of items
    return Body::from_json(&data);
  }

