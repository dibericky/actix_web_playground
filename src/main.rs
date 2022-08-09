use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, guard};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    format!("Index changed! {}", data.app_name)
}

fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| async { HttpResponse::Ok().body("test") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/api")
            .route(web::get().to(|| async { HttpResponse::Ok().body("app") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

struct AppState {
    app_name: String
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web up")
            }))
            .configure(config)
            .service(index)
            .service(web::scope("/app")
                .guard(guard::Header("x-foo", "lol"))
                .app_data(web::Data::new(AppState {
                    app_name: String::from("Actix Web down")
                }))
                .service(index)
            )
            .service(web::scope("/scope").configure(scoped_config))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
