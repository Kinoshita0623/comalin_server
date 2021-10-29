use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use actix_web::Responder;
use actix_web::web::Bytes;
use uuid::Uuid;
use actix_web::web::Buf;
use std::io::Write;

use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};

pub async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error>{
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        println!("filename:{}", filename);
        println!("mime:{:?}", content_type);
        let filepath = format!("./tmp/{}", Uuid::new_v4().to_string());

        // File::create is blocking operation, use threadpool
        let mut f: std::fs::File = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();
        
            

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data: Bytes = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }
    return Ok(HttpResponse::Ok().into());
}