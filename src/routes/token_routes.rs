use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use tokenizer::{
    lexer::Lexer,
    token_definition::TokenDefinition,
    token_actions::TokenActions
    };
//cargo update -p tokenizer
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Info 
{
    pub username: String,
    pub fam : String,
}
#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
#[post("/json")]
pub async fn json(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcome {} {}!", info.username, info.fam))
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}