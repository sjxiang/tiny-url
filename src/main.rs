use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use sqlx::mysql::MySqlPoolOptions;
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
async fn main() -> Result<(), sqlx::Error> {

    let s = Settings::new().unwrap();
    let ip = s.server.get_ip();
    let url = s.database.url;
    let pool_size = s.database.pool_size;

    let pool = MySqlPoolOptions::new()
        .max_connections(pool_size)
        .connect(&url)
        .await?;


    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))  // 把 db 连接池移到 HTTP server 中管理
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
    .await?;

    Ok(())
}