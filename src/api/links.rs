use actix_web::{
    post, 
    get, 
    web::{self, Json, Path}, 
    Responder, 
    HttpResponse, 
    http::header,
};
use serde::{Deserialize, Serialize};
use nanoid::nanoid;
use sqlx::{Pool, MySql};

use crate::api::ApiResult;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
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
async fn create_link(Link: Json<ApiAddLink>, data: web::Data<Pool<MySql>>) -> impl Responder {
    let new_link = Link.0.to_new_link();
    let new_code = new_link.tiny_code.clone();
    
    if let Err(e) = insert_into_tiny_link(data.as_ref().clone(), new_link).await {
        return Json(ApiResult::error(e.to_string()))
    }
    Json(ApiResult::success(Some(new_code)))
}


async fn insert_into_tiny_link(pool: Pool<MySql>, new_link: Link) -> Result<u64, sqlx::Error> {
    let insert_id = sqlx::query(r#"insert into tiny_link(tiny_code, origin_url) values (?, ?)"#)
        .bind(new_link.tiny_code)
        .bind(new_link.origin_url)
        .execute(&pool)
        .await?
        .last_insert_id();

    Ok(insert_id)
}



#[get("/{code}")]
async fn get_from_link(path: Path<String>, data: web::Data<Pool<MySql>>) -> impl Responder {
    let code: String = path.into_inner();
    
    println!("{code}");

    let url = get_origin_url(data.as_ref().clone(), code).await;
    let url = match url {
        Ok(x) => x,
        Err(e) => {
            println!("{}", e);
            return HttpResponse::NotFound().finish();
        },
    };

    HttpResponse::Found()
        .append_header((header::LOCATION, url))  // 重定位
        .finish()
}


async fn get_origin_url(pool: Pool<MySql>, code: String) -> Result<String, sqlx::Error> {
    let row:(String, ) = sqlx::query_as("select origin_url from tiny_link where tiny_code = ?")
        .bind(code)
        .fetch_one(&pool)
        .await?;
    
    Ok(row.0)
}


#[get("/links")]
async fn get_all_links(data: web::Data<Pool<MySql>>) -> impl Responder {
    let links: Result<Vec<Link>, sqlx::Error> = get_links(data.as_ref().clone()).await;
    let links: Vec<Link> = match links {
        Ok(x) => x,
        Err(e) => {
            println!("{}", e);
            return Json(ApiResult::error(e.to_string()));
        }
    };

    Json(ApiResult::success(Some(links)))
}



async fn get_links(pool: Pool<MySql>) -> Result<Vec<Link>, sqlx::Error> {
    let row: Vec<Link> = sqlx::query_as::<_, Link>("select tiny_code, origin_url from tiny_link")
        .fetch_all(&pool)
        .await?;
    
    Ok(row)
}