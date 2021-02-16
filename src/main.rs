use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

static ADDRESS: &str = "127.0.0.1";
static PORT: &str = "8080";

struct AppStateWithCounter {
    counter :Mutex<i32>,
}

async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    format!("Request number: {}", counter)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter{
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .route("/", web::get().to(index))
    })
        .bind(format!("{}:{}", ADDRESS, PORT))?
        .run()
        .await
}