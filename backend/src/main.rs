use std::fs::File;
//serde
use serde::{Deserialize, Serialize};
use serde_json;
use actix_web::{App, HttpResponse, HttpServer, Responder};



#[derive(Deserialize, Serialize)]
struct BingoWords {
    bingo_words: Vec<String>
}

#[actix_web::get("/api")]
async fn readsendjson() -> impl Responder {
    let path = File::open("words.json").unwrap();
    let word: BingoWords = serde_json::from_reader(&path).unwrap();
    HttpResponse::Ok().json(word)
}


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(move || { App::new().service(readsendjson)}).bind("127.0.0.1:8080").unwrap().workers(4).run().await
}