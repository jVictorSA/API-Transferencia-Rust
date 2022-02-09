use async_std::stream::StreamExt;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use tide::{Body, Request, Response};
use crate::State;
// use chrono::Local;
use crate::cliente::Cliente;

#[derive(Debug, Serialize, Deserialize)]
pub struct DadosTed {
  pub nome: String,
  pub cpf: String,
  pub conta: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DadosPix {
  pub nome: String,
  pub chave_pix: String
}

pub async fn exibir_ted(req: Request<State>) -> tide::Result<tide::Body> {
    let cpf: String = req.param("cpf")?.parse().unwrap();
    
    let db = &req.state().db;
    
    let clientes_collection = db.collection_with_type::<Cliente>("clientes");
    
    let cliente = clientes_collection.find_one(
      doc! {
          "cpf": cpf
        },
        None
      )
      .await?;
     
    if let None = cliente{
      let response = format!("Não foi encontrada conta referente ao pedido!");
    
      return Body::from_json(&response);
    }
  
    let cliente_use = cliente.unwrap();
    
    let resposta = DadosTed{
      nome: cliente_use.nome,
      cpf: cliente_use.cpf,
      conta: cliente_use.conta,
    };
    
    return Body::from_json(&resposta);
}  

pub async fn exibir_pix(req: Request<State>) -> tide::Result{
  let cpf: String = req.param("cpf")?.parse().unwrap();
  
  let db = &req.state().db;

  let clientes_collection = db.collection_with_type::<Cliente>("clientes");

  let cliente = clientes_collection.find_one(
    doc!{
      "cpf": cpf
    },
    None
  )
  .await?;

  if let None = cliente{

    let mut res = Response::new(404);
    res.set_body(String::from("Nenhuma chave pix foi encontrada"));

    return Ok(res);
  }

  let cliente_use = cliente.unwrap();
  let ans = DadosPix {
    nome: cliente_use.nome,
    chave_pix: cliente_use.pix
  };

  let mut res = Response::new(200);
  
  res.set_body(Body::from_json(&ans)?);
  return Ok(res);
}