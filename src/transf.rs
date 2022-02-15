use async_std::stream::StreamExt;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use tide::{Body, Request, Response};
use crate::State;
use chrono::Local;
use crate::cliente::Cliente;

#[derive(Debug, Serialize, Deserialize)]
pub struct Resposta {
  pub mensagem: String,
  pub remetente: String,
  pub destinatario: String,
  pub quantia: f64,
  pub data: String,
  pub hora: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransfConta{
  pub conta_remetente: String,
  pub quantia: f64,
  pub conta_destinatario: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransfPix{
  pub pix_remetente: String,
  pub quantia: f64,
  pub pix_destinatario: String
}

pub async fn transf_conta(mut req: Request<State>) -> tide::Result{
  //verify if account was find
  let transferencia = req.body_json::<TransfConta>().await?;

  let db = &req.state().db;
  
  let clientes_collection = db.collection_with_type::<Cliente>("clientes");

  let rem = clientes_collection.find_one(
    doc! {
      "conta": transferencia.conta_remetente.clone()
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
      "conta": transferencia.conta_destinatario.clone()
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
    let mut res = Response::new(406);
    
    res.set_body(String::from("O saldo da conta é insuficiente"));
    return Ok(res);
  }
  else if transferencia.quantia < 0.0{
    let mut res = Response::new(406);
    
    res.set_body(String::from("A requisição de transferência não pode ter uma quantia negativa!"));
    return Ok(res);
  }
  else {
    clientes_collection.update_one(
      doc!{
        "conta": transferencia.conta_remetente.clone(),
      }, doc!{
        "$inc": {"saldo": -transferencia.quantia}
      }, None
    )
    .await?;
  
    clientes_collection.update_one(
      doc!{
        "conta": transferencia.conta_destinatario.clone(),
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
      "pix": transferencia.pix_remetente.clone()
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
      "pix": transferencia.pix_destinatario.clone()
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
    let mut res = Response::new(406);
    
    res.set_body(String::from("O saldo da conta é insuficiente"));
    return Ok(res);
  }
  else if transferencia.quantia < 0.0{
    let mut res = Response::new(406);
    
    res.set_body(String::from("A requisição de transferência não pode ter uma quantia negativa!"));
    return Ok(res);
  }
  else {
      clientes_collection.update_one( //update o remetente -> ele perde dinheiro
        doc!{
          "pix": transferencia.pix_remetente.clone(),
        }, doc!{
          "$inc": {"saldo": -transferencia.quantia}
        }, None
      )
      .await?;
    
      clientes_collection.update_one( //update o destinatario -> ele aumenta dinheiro
        doc!{
          "pix": transferencia.pix_destinatario.clone(),
        }, doc!{
          "$inc": {"saldo": transferencia.quantia}
        }, None
      )
      .await?;
      let now = Local::now();
      // pub struct Resposta {
      //   pub mensagem: String,
      //   pub remetente: String,
      //   pub destinatario: String,
      //   pub quantia: f64,
      //   pub data: String,
      //   pub hora: String
      // }
      let ans = Resposta {
        mensagem: "Transferência via pix concluída".to_string(),
        remetente: transferencia.pix_remetente,
        destinatario: transferencia.pix_destinatario,
        quantia: transferencia.quantia,
        data: now.format("%d-%m-%Y").to_string(),
        hora: now.format("%H:%M:%S").to_string()
      };

      println!("Transferencia via pix concluida");
      let mut res = Response::new(200);

      res.set_body(Body::from_json(&ans)?);
      return Ok(res);    
    }
}
