use async_std::stream::StreamExt;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use tide::{Body, Request, Response};
use crate::State;
use chrono::Local;
use crate::cliente::Cliente;

pub async fn hello(_req: Request<State>) -> tide::Result {
    return Ok("Hello, world!".into()) //into() transforma a str em tide::Result
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DepositoResult {
  pub mensagem: String,
  pub conta: String,
  pub quantia: f64,
  pub data: String,
  pub hora: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Deposito{
  pub conta: String,
  pub quantia: f64
}


pub async fn auto_deposito(mut req: Request<State>) -> tide::Result {
  
  let requisicao = req.body_json::<Deposito>().await?;
  
  if requisicao.quantia < 0.0 {
    let mut res = Response::new(406);
    
    res.set_body(String::from("A requisição de depósito não pode ter uma quantia negativa!"));
    return Ok(res);
  }

  let db = &req.state().db;
  
  let clientes_collection = db.collection_with_type::<Cliente>("clientes");

  let option_cliente = clientes_collection.find_one(
    doc!{
      "conta": requisicao.conta
    },
    None
  )
  .await?;

  if let None = option_cliente {
    let mut res = Response::new(404);
    println!("Erro: conta não encontrada");

    let response = format!("Conta do depositante não encontrada!");
    res.set_body(String::from("Pix do destinatário não encontrado!"));
    return Ok(res); 
  }

  if remetente.saldo < transferencia.quantia {
    let mut res = Response::new(406);
    
    res.set_body(String::from(response));
    return Ok(res);
  }
    let cliente = option_cliente.unwrap();
    // let conta_copy = cliente.conta;

    let _deposito = clientes_collection.update_one(
      doc!{
        "conta": cliente.conta.clone()

      }, doc! {
        "$inc": {"saldo": requisicao.quantia}
      }, None
    )
    .await?;

  let now = Local::now();
  return Ok(Response::new(201));
}

  let ans = DepositoResult {
    mensagem: "Depósito realizado com sucesso".to_string(),
    conta: cliente.conta,
    quantia: requisicao.quantia,
    data: now.format("%d-%m-%Y").to_string(),
    hora: now.format("%H:%M:%S").to_string()
  };

  let mut res = Response::new(200);
  
  res.set_body(Body::from_json(&ans)?);
    
  return Ok(res);
}