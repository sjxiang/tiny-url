use actix_web::{post, get, web::{Json, Path}, Responder, HttpResponse, http::header};
use serde::{Deserialize, Serialize};
use nanoid::nanoid;

use crate::api::ApiResult;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Link {
    tiny_code: String,
    origin_url: String,
}


#[derive(Deserialize, Clone)]
struct ApiAddLink {
    origin_url: String,
}


impl ApiAddLink {
    fn to_new_link(self) -> Link {
        Link {
            tiny_code: nanoid!(5),
            origin_url: self.origin_url,
        }
    }
}


#[post("/create")]
async fn create_link(Link: Json<ApiAddLink>) -> impl Responder {
    let new_link = Link.0.to_new_link();
    let new_code = new_link.tiny_code.clone();
    Json(ApiResult::success(Some((new_code))))
}

#[get("/{code}")]
async fn get_from_link(path: Path<String>) -> impl Responder {
    let code: String = path.into_inner();
    if code == "test" {
        let api_result: ApiResult<String> = ApiResult::error("404".to_string());
        return HttpResponse::Ok().json(api_result);        
    }
    let url = "http://baidu.com";
    HttpResponse::Found().append_header((header::LOCATION, url)).finish()
}


#[get("/links")]
async fn get_all_links() -> impl Responder {
    let mut links: Vec<Link> = Vec::new();
    links.push(Link {
        tiny_code: String::from("111"),
        origin_url: String::from("http://baidu.com"),
    });
    links.push(Link {
        tiny_code: String::from("222"),
        origin_url: String::from("http://google.com"),
    });

    Json(ApiResult::success(Some(links)))
}
