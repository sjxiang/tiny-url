use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use tera::{Tera, Context};
use crate::settings::Settings;


mod api;
mod settings;


#[macro_use]
extern crate lazy_static;
 
lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error: {}", e);
                ::std::process::exit(1);
            }
        };

        tera
    };
}


#[get("/")]
async fn index() -> impl Responder {
    let content = "Generate a tiny code for url";
    let mut data = Context::new();
    
    data.insert("content", content);

    let rendered = TEMPLATES.render("index.html", &data).unwrap();

    HttpResponse::Ok().body(rendered)
}



#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let s = Settings::new().unwrap();
    let ip = s.server.get_ip();

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(api::links::create_link)
            .service(api::links::get_all_links)
            .service(api::links::get_from_link)
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    // .bind(("127.0.0.1", 8080))?
    .bind(&ip)?
    .run()
    .await
}