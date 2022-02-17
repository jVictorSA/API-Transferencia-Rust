use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use tide::{Body, Request, Response};
use crate::State;
use chrono::Local;
use crate::cliente::Cliente;

#[derive(Debug, Serialize, Deserialize)] 
pub struct Resposta { //resposta enviada depois que a transferência é concluída 
  pub mensagem: String,
  pub remetente: String,
  pub destinatario: String,
  pub quantia: f64,
  pub data: String,
  pub hora: String
}

//em um mundo ideal, mais dados seriam inseridos aqui, feito Agência, código do banco...
#[derive(Debug, Serialize, Deserialize)]
pub struct TransfConta{ //requisição para transferência TED/DOC
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

  let transferencia = req.body_json::<TransfConta>().await?; //pega o json que foi enviado na requisição

  let db = &req.state().db; //chama o db
  
  let clientes_collection = db.collection_with_type::<Cliente>("clientes");
  //pega a nossa coleção de clientes, para realizar a transferência entre as instâncias 

  let rem = clientes_collection.find_one(
     //find_one() é função do mongodb, para achar uma instância com os dados especificados no primeiro parâmetro
    doc! {
      "conta": transferencia.conta_remetente.clone() //precisamos encontrar o remetente
    },
    None
  )
  .await?;
  
  if let None = rem { //caso o remetente não seja encontrado:
    let mut res = Response::new(404);
    println!("Erro: conta remetente não encontrada");

    let response = format!("Conta do remetente {} não encontrada!", transferencia.conta_remetente);
    
    res.set_body(String::from(response));
    return Ok(res);
  }

  let remetente = rem.unwrap(); //com esse comando, remetente vira uma instancia da struct Cliente

  let destinatario = clientes_collection.find_one(
    doc! {
      "conta": transferencia.conta_destinatario.clone() //precisamos encontrar o destinatário
    },
    None
  )
  .await?;
  if let None = destinatario { //caso o destinatário não seja encontrado:
    let mut res = Response::new(404);
    println!("Erro: conta destinatária não encontrada");

    let response = format!("Conta do destinatário {} não encontrada!", transferencia.conta_destinatario);
    res.set_body(String::from(response));
    return Ok(res); 
  }

  if remetente.saldo < transferencia.quantia { //o remetente não pode enviar uma quantia que não tem
    let mut res = Response::new(406);
    
    res.set_body(String::from("O saldo da conta é insuficiente"));
    return Ok(res);
  }
  else if transferencia.quantia < 0.0{ //o remetente não pode enviar uma quantia negativa
    let mut res = Response::new(406);
    
    res.set_body(String::from("A requisição de transferência não pode ter uma quantia negativa!"));
    return Ok(res);
  }
  else { //caso esteja tudo certo:
    clientes_collection.update_one( 
      //update_one() é do MongoDB: encontra o cliente com os dados do primeiro parâmetro
      //e atualiza os dados do segundo
      //aqui nós tiramos o dinheiro do remetente, então diminuímos o saldo da quantia
      doc!{
        "conta": transferencia.conta_remetente.clone(),
      }, doc!{
        "$inc": {"saldo": -transferencia.quantia} //tire o dinheiro do remetente
      }, None
    )
    .await?;
  
    clientes_collection.update_one(
      doc!{
        "conta": transferencia.conta_destinatario.clone(),
      }, doc!{
        "$inc": {"saldo": transferencia.quantia} //soma o dado do destinatário pela quantia
      }, None
    )
    .await?;
    let now = Local::now();
    
    //resposta JSON ao fim da função
    let ans = Resposta {
      mensagem: "Transferência via conta concluída".to_string(),
      remetente: transferencia.conta_remetente,
      destinatario: transferencia.conta_destinatario,
      quantia: transferencia.quantia,
      //é assim que se formata data e hora usando Chrono:
      data: now.format("%d-%m-%Y").to_string(), //dia-mês-ano
      hora: now.format("%H:%M:%S").to_string() //hora-minuto-segundo
    };

    println!("Transferencia via conta concluida"); //confirma no terminal que a função foi bem sucedida
    let mut res = Response::new(200);

    res.set_body(Body::from_json(&ans)?);    
    
    return Ok(res);

    //a função de pix é análoga
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
