use async_std::stream::StreamExt;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use tide::{Body, Request, Response};
use crate::State;


#[derive(Debug, Serialize, Deserialize)]
pub struct Cliente {
    pub nome: String,
    pub num: String, //id
    pub cpf: String,
    pub conta: String,
    pub saldo: String,
    pub pix: String
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
  
    return Ok(Response::new(201));
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