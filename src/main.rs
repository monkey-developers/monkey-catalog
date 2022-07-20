use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, path::Path};

#[derive(Debug, Serialize, Deserialize)]
struct Monkey {
    id: String,
    gender: String,
    specie: String,
    age: u8,
    location: String,
}

#[get("/monkey")]
async fn get_all_projects() -> impl Responder {
    let json_file_path = Path::new("./src/db.json");
    let mut file = File::open(json_file_path).unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();

    let monkeys: Vec<Monkey> = serde_json::from_str(&buff).expect("Not able to parse json file");

    HttpResponse::Ok().body(serde_json::to_string(&monkeys).unwrap())
}

#[get("/monkey/{monkey_id}")]
async fn get_project_by_id(path: web::Path<String>) -> impl Responder {
    let monkey_id = path.into_inner();

    let json_file_path = Path::new("./src/db.json");
    let mut file = File::open(json_file_path).unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();

    let monkeys: Vec<Monkey> = serde_json::from_str(&buff).expect("Not able to parse json file");

    let monkey_by_id: Vec<Monkey> = monkeys
        .into_iter()
        .filter(|monkey| monkey.id == monkey_id)
        .collect::<Vec<Monkey>>();

    HttpResponse::Ok().body(serde_json::to_string(&monkey_by_id.get(0)).unwrap())
}

#[post("/monkey")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_all_projects)
            .service(echo)
            .service(get_project_by_id)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
