# Microsserviço de transferências bancárias
Este projeto em Rust foi desenvolvido para a disciplina Conceitos de Linguagens de Programação, onde fora solicitado aos alunos desenvolver diversos microsserviços relacionados à um banco digital, o **OxeBank**. O nosso microsserviço é responsável pelas transferências.


## Dependências
A única dependência externa ao projeto é o MongoDB, que utilizamos para armazenar a base de dados dos clientes.

## Como rodar o projeto
Como o projeto é desenvolvido em Rust, é necessário tê-la instalada na sua máquina.
Instale a Rust e suas ferramentas em: https://www.rust-lang.org/pt-BR/learn/get-started

Verifique se a Rust e o gerenciador de pacotes Cargo está instalado rodando a instrução `cargo --version`

Compile o projeto digitando o comando `cargo build` no seu terminal e rode com o comando `cargo run`

O servidor HTTP está configurado na porta 8080 do localhost, então para fazer requisições à API utilize localhost:8080/*endpoint*
