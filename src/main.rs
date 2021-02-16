use actix_web::{get, web, App, HttpServer};

static ADDRESS: &str = "127.0.0.1";
static PORT: &str = "8080";

struct AppState {
    app_name :String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;

    format!("Hello {}!", app_name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(AppState {
                app_name: String::from("Actix-web")
            })
            .service(index)
    })
        .bind(format!("{}:{}", ADDRESS, PORT))?
        .run()
        .await
}