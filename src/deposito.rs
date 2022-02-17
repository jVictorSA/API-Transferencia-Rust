use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use tide::{Body, Request, Response};
use crate::State;
use chrono::Local; //pra pegar data/hora
use crate::cliente::Cliente; //nossa struct Cliente 

#[derive(Debug, Serialize, Deserialize)]
pub struct DepositoResult { //resposta dada ao fim da função
  pub mensagem: String,
  pub conta: String,
  pub quantia: f64,
  pub data: String,
  pub hora: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Deposito { //requisição do depósito
  //com uma estrutura robusta de clientes, talvez houvesse mais dados aqui
  pub conta: String,
  pub quantia: f64
}

//a ideia é o cliente depositar na própria conta, como se pagasse um boleto-depósito
//o dinheiro cai automaticamente
pub async fn auto_deposito(mut req: Request<State>) -> tide::Result {
  
  //requisição <JSON> passada pra struct <Deposito>
  let requisicao = req.body_json::<Deposito>().await?; 

  //verificar se a quantia é menor do que 0
  if requisicao.quantia < 0.0 {
    let mut res = Response::new(406); // Response::new(código-padrão http)
    
    //com o set_body, a resposta tem, além do erro, o que estiver dentro dos parênteses
    res.set_body(String::from("A requisição de depósito não pode ter uma quantia negativa!"));
    return Ok(res);
  }

  let db = &req.state().db;
  
  let clientes_collection = db.collection_with_type::<Cliente>("clientes");

  //procurar o cliente que tenha a conta passada na requisição
  let option_cliente = clientes_collection.find_one(
    doc!{
      "conta": requisicao.conta
    },
    None
  )
  .await?;

  //verificar se nenhum cliente foi encontrado
  if let None = option_cliente {
    let mut res = Response::new(404);
    println!("Erro: conta não encontrada");

    let response = format!("Conta do depositante não encontrada!");
    
    res.set_body(String::from(response));
    return Ok(res);
  }
    //option_cliente é uma option, um lance de Rust
    //pra usar como struct cliente, passamos o option com a função unwrap(),
    //que é uma espécie de desempacotamento
    //depois disso, ele vira a struct cliente
    let cliente = option_cliente.unwrap();

    //aumentando o saldo do cliente
    let _deposito = clientes_collection.update_one(
      doc!{
        "conta": cliente.conta.clone()

      }, doc! {
        "$inc": {"saldo": requisicao.quantia}
      }, None
    )
    .await?;

  let now = Local::now(); //data e hora, logo após o update

  //resposta JSON
  //primeiro é a struct DepositoResult
  let ans = DepositoResult {
    mensagem: "Depósito realizado com sucesso".to_string(), //to_string() é necessária porque em Rust str != String
    conta: cliente.conta,
    quantia: requisicao.quantia,
    //é assim que se formata data e hora usando Chrono:
    data: now.format("%d-%m-%Y").to_string(), //dia-mês-ano
    hora: now.format("%H:%M:%S").to_string() //hora-minuto-segundo
  };

  //código 200: tudo certo
  let mut res = Response::new(200);
  
  //envia o código com o DepositoResult passado pra Json
  res.set_body(Body::from_json(&ans)?);
    
  return Ok(res);
}