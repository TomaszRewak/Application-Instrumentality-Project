use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = "localhost:5123";

    println!("Starting up the app");
    println!("Listening on {}", address);

    HttpServer::new(move || {
        App::new()
            .service(hello)
            .service(echo)
    })
    .bind(address)?
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
