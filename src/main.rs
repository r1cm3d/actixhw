// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::{web, App, HttpServer, Responder};

static ADDRESS: &str = "127.0.0.1";
static PORT: &str = "8080";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/app")
                    .route("/index.html", web::get().to(index)),
            )
    })
        .bind(format!("{}:{}", ADDRESS, PORT))?
        .run()
        .await
}

async fn index() -> impl Responder {
    "Hello world!"
}

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }
//
// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }
//
// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }
