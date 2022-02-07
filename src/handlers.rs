use async_std::stream::StreamExt;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use tide::{Body, Request, Response};
// use std::path::Path;
use crate::State;

pub async fn hello(_req: Request<State>) -> tide::Result {
    return Ok("Hello, world!".into()) //into() transforma a str em tide::Result
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Deposito{
  pub num: String,
  pub quantia: f64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DadosTed {
  pub nome: String,
  pub cpf: String,
  pub conta: String
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Cliente {
    pub nome: String,
    pub num: String, //id
    pub cpf: String,
    pub conta: String,
    pub saldo: f64,
    pub pix: String
}

pub async fn exibir_pix(req: Request<State>) -> tide::Result<tide::Body>{
  
  let cpf: i32 = req.param("cpf")?.parse().unwrap();
  
  println!("num: {:?}", cpf);

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
    let response = format!("Não foi encontrada chave pix referente ao pedido!");
    return Body::from_json(&response);
  }

  let cliente_use = cliente.as_ref().unwrap();

  return Body::from_json(&format!("chave_pix: {}", cliente_use.pix));
}

pub async fn exibir_ted(req: Request<State>) -> tide::Result<tide::Body> {
  let cpf: i32 = req.param("cpf")?.parse().unwrap();
  
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

pub async fn auto_deposito(mut req: Request<State>) -> tide::Result{
  let requisicao = req.body_json::<Deposito>().await?;

  let db = &req.state().db;
  
  let clientes_collection = db.collection_with_type::<Cliente>("clientes");

  let option_cliente = clientes_collection.find_one(
    doc!{
      "num": requisicao.num
    },
    None
  )
  .await?;

  if let None = option_cliente {
    let mut res = Response::new(404);
    println!("Erro: conta não encontrada");

    let response = format!("Conta do depositante não encontrada!");
    
    res.set_body(String::from(response));
    return Ok(res);
  }else{

    let cliente = option_cliente.unwrap();

    let _deposito = clientes_collection.update_one(
      doc!{
        "num": cliente.num

      }, doc! {
        "$inc": {"saldo": requisicao.quantia}
      }, None
    )
    .await?;
  }

  let mut res = Response::new(200);

  res.set_body(String::from("Depósito concluido"));
    
  return Ok(res);
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

      res.set_body(String::from("Transferencia via pix concluida"));
    
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
