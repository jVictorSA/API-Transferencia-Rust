use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use tide::{Body, Request, Response};
use crate::State;
use crate::cliente::Cliente;
use qrcode_generator::QrCodeEcc;


//resposta com os dados para TED/DOC
//com uma estrutura mais robusta de cliente, talvez houvesse mais/outros dados
#[derive(Debug, Serialize, Deserialize)]
pub struct DadosTed {
  pub nome: String,
  pub cpf: String,
  pub conta: String
}

//resposta com chave pix e nome do cliente
#[derive(Debug, Serialize, Deserialize)]
pub struct DadosPix {
  pub nome: String,
  pub chave_pix: String
}

//exibir os dados necessários para que um terceiro deposite na conta do cliente

pub async fn dados_ted_qr(req: Request<State>) -> tide::Result<tide::Body> {
  let cpf: String = req.param("cpf")?.parse().unwrap(); //pega o parâmetro :cpf na url e passa pra string
    
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

  let dados_cliente_qr = serde_json::to_string(&resposta)?;

  println!("cliente é {}", dados_cliente_qr);
  qrcode_generator::to_png_to_file(dados_cliente_qr.clone(), QrCodeEcc::Low, 1024, "data/QRs/dados_ted.png").unwrap();
  
  return Body::from_json(&dados_cliente_qr);
}

pub async fn dados_pix_qr(req: Request<State>) -> tide::Result<tide::Body> {
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
  
  let resposta = DadosPix{
    nome: cliente_use.nome,
    chave_pix: cliente_use.pix
  };

  let dados_cliente_qr = serde_json::to_string(&resposta)?;

  println!("cliente é {}", dados_cliente_qr);
  qrcode_generator::to_png_to_file(dados_cliente_qr.clone(), QrCodeEcc::Low, 1024, "data/QRs/dados_pix.png").unwrap();
  
  return Body::from_json(&dados_cliente_qr);
}


pub async fn exibir_ted(req: Request<State>) -> tide::Result<tide::Body> {
  //com um cliente melhor estruturado, talvez fosse mais apropriado um ID em vez do CPF
    
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