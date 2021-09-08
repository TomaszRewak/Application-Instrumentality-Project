mod proto;
mod workspace_manager;
mod workstation;

use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use proto::workspace_manager_server::WorkspaceManagerServer;
use tonic::transport::Server;
use workspace_manager::WorkspaceManager;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let management_address = "[::1]:5122".parse().unwrap();
    let workspace_manager = WorkspaceManager::new();

    Server::builder()
        .add_service(WorkspaceManagerServer::new(workspace_manager))
        .serve(management_address)
        .await
        .unwrap();

    let web_address = "localhost:5123";

    println!("Starting up the app");
    println!("Listening on {}", web_address);

    HttpServer::new(move || App::new().service(hello).service(echo))
        .bind(web_address)?
        .run()
        .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello bright new world 2!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
