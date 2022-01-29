use async_std::stream::StreamExt;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use tide::{Body, Request, Response, StatusCode};
// use std::path::Path;
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
pub struct TransfConta{
  pub conta_remetente: i32,
  pub quantia: f64,
  pub conta_destinatario: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransfPix{
  pub pix_remetente: i32,
  pub quantia: f64,
  pub pix_destinatario: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cliente {
    pub nome: String,
    pub num: i32, //id
    pub cpf: i32,
    pub conta: i32,
    pub saldo: f64,
    pub pix: i32,
}

pub async fn deposito_ted(req: Request<State>) -> tide::Result<tide::Body> {
  let n_conta: i32 = req.param("num")?.parse().unwrap();
  println!("num: {:?}", n_conta);
  let db = &req.state().db;
  
  let clientes_collection = db.collection_with_type::<Cliente>("clientes");
  
  let cliente = clientes_collection.find_one(
    doc! {
        "conta": n_conta
      },
      None
    )
    .await?;
      
    return Body::from_json(&cliente);
}
    // Ok(Response::new(StatusCode::Ok))
    
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

pub async fn transf_conta(mut req: Request<State>) -> tide::Result{
//verify if account was find
  let transferencia = req.body_json::<TransfConta>().await?;

  let db = &req.state().db;
  
  let clientes_collection = db.collection_with_type::<Cliente>("clientes");

  let rem = clientes_collection.find_one(
    doc! {
      "conta": transferencia.conta_remetente
    },
    None
  )
  .await?;
  
  if let None = rem {
    let mut res = Response::new(404);
    println!("Erro: conta remetente não encontrada");

    let response = format!("Conta do remetente {} não encontrada!", transferencia.conta_remetente);
    
    res.set_body(String::from(response));
    return Ok(res);
  }

  let remetente = rem.unwrap();

  let destinatario = clientes_collection.find_one(
    doc! {
      "conta": transferencia.conta_destinatario
    },
    None
  )
  .await?;
  if let None = destinatario {
    let mut res = Response::new(404);
    println!("Erro: conta destinatária não encontrada");

    let response = format!("Conta do destinatário {} não encontrada!", transferencia.conta_destinatario);
    res.set_body(String::from(response));
    return Ok(res); 
  }

  if remetente.saldo < transferencia.quantia {
    let mut res = Response::new(403);
    
    res.set_body(String::from("O saldo da conta é insuficiente"));
    return Ok(res);
  }
  else {
    clientes_collection.update_one(
      doc!{
        "conta": transferencia.conta_remetente,
      }, doc!{
        "$inc": {"saldo": -transferencia.quantia}
      }, None
    )
    .await?;
  
    clientes_collection.update_one(
      doc!{
        "conta": transferencia.conta_destinatario,
      }, doc!{
        "$inc": {"saldo": transferencia.quantia}
      }, None
    )
    .await?;
    
    println!("Transferencia por conta concluida");
    let mut res = Response::new(200);

    res.set_body(String::from("Transferencia por conta concluida"));
    
    return Ok(res);
  }
}

pub async fn transf_pix(mut req: Request<State>) -> tide::Result{

  let transferencia = req.body_json::<TransfPix>().await?;

  let db = &req.state().db;
  
  let clientes_collection = db.collection_with_type::<Cliente>("clientes");

  let rem = clientes_collection.find_one(
    doc! {
      "pix": transferencia.pix_remetente
    },
    None
  )
  .await?;

  if let None = rem {
    let mut res = Response::new(404);
    println!("Erro: pix remetente não encontrado");
    
    res.set_body(String::from("Pix do remetente não encontrado!"));
    return Ok(res);
  }

  let remetente = rem.unwrap();

  let destinatario = clientes_collection.find_one(
    doc! {
      "pix": transferencia.pix_destinatario
    },
    None
  )
  .await?;

  if let None = destinatario {
    let mut res = Response::new(404);
    println!("Erro: Pix destinatário não encontrada");

    res.set_body(String::from("Pix do destinatário não encontrado!"));
    return Ok(res); 
  }

  if remetente.saldo < transferencia.quantia {
    let mut res = Response::new(403);
    
    res.set_body(String::from("O saldo da conta é insuficiente"));
    return Ok(res);
  }

  else {
      clientes_collection.update_one( //update o remetente -> ele perde dinheiro
        doc!{
          "pix": transferencia.pix_remetente,
        }, doc!{
          "$inc": {"saldo": -transferencia.quantia}
        }, None
      )
      .await?;
    
      clientes_collection.update_one( //update o destinatario -> ele aumenta dinheiro
        doc!{
          "pix": transferencia.pix_destinatario,
        }, doc!{
          "$inc": {"saldo": transferencia.quantia}
        }, None
      )
      .await?;

      println!("Transferencia via pix concluida");
      let mut res = Response::new(200);

      res.set_body(String::from("Transferencia por conta concluida"));
    
      return Ok(res);    
    }
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

  println!("{}", item.name.as_str());
  // Recover the database connection handle from the Tide state
  let db = &req.state().db;

  // Get a handle to the "items" collection
  let items_collection = db.collection_with_type::<Item>("items");
  
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
