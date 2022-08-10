use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Deserialize;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    format!("Index changed! {}", data.app_name)
}

#[get("/users/{user_id}/{friend}")]
async fn handler_with_path(data: web::Data<AppState>, path: web::Path<(u32, String)>) -> impl Responder {
    let (user_id, friend) = path.into_inner();
    format!("Index changed! {} {user_id} {friend}", data.app_name)
}

#[derive(Deserialize)]
struct Info {
    user_id: u32,
    friend: String
}

#[get("/users/{user_id}/{friend}")]
async fn handler_with_path_struct(data: web::Data<AppState>, path: web::Path<Info>) -> Result<String> {
    let user : Info = path.into_inner();
    let fmt = format!("Index changed! {} {} {}", data.app_name, user.user_id, user.friend);
    Ok(fmt)
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

fn foobar () -> String {
    "ok".to_string()
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
            .route("/echo", web::post().to(echo))
            .service(index)
            .service(handler_with_path)
            .service(web::scope("/app")
               // .guard(guard::Header("x-foo", "lol"))
                .app_data(web::Data::new(AppState {
                    app_name: String::from("Actix Web down")
                }))
                .service(handler_with_path_struct)
                .service(index)
            )
            .service(web::scope("/scope").configure(scoped_config))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        http::{self, header::ContentType},
        test,
    };

    #[actix_web::test]
    async fn test_index_ok() {
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_http_request();
        let resp = echo("the-body".to_string()).await;
        assert_eq!(resp.respond_to(&req).status(), http::StatusCode::OK);
    }
}