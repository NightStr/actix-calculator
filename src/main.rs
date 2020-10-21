use actix_web::{web, App, HttpServer};
use tera::{Tera, Context};
use serde::Deserialize;
use reverse;


struct AppState {
    template: tera::Tera
}

#[derive(Deserialize)]
struct CalculatorForm {
    expression: String
}


async fn index_get(data: web::Data<AppState>) -> web::HttpResponse {
    let mut context = Context::new();
    context.insert("input_value", "");
    web::HttpResponse::Ok()
        .content_type("text/html")
        .body(data.template.render("index.html", &context).unwrap())
}

async fn index_post(data: web::Data<AppState>, post_data: web::Form<CalculatorForm>) -> web::HttpResponse {
    let mut context = Context::new();
    let result = match reverse::eval(&post_data.expression) {
        Ok(n) => n.to_string(),
        Err(e) => e
    };
    context.insert("result", &result);
    context.insert("input_value", &post_data.expression);
    web::HttpResponse::Ok()
        .content_type("text/html")
        .body(data.template.render("index.html", &context).unwrap())
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(AppState {template: Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap()})
            .service(
                web::scope("/")
                    .route("", web::get().to(index_get))
                    .route("", web::post().to(index_post))
            )
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
