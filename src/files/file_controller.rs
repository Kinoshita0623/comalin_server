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
        let tmp_filename = Uuid::new_v4().to_string();
        let filepath = format!("./tmp/{}", tmp_filename.clone());

        // File::create is blocking operation, use threadpool
        let filepath2 = filepath.clone();
        let mut f: std::fs::File = web::block(|| std::fs::File::create(filepath2))
            .await
            .unwrap();
        
        let mut cap = 0;
        
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data: Bytes = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool

            cap += data.len();
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
        
       
        let kind = match infer::get_from_path(filepath)? {
            Some(kind) => kind,
            None => {
                return Ok(HttpResponse::Ok().into());
            }
        };
        let extenstion = kind.extension();
        let mime_type = kind.mime_type();
        
        println!("extenstion:{}, mimetype:{}, cap:{}", extenstion, mime_type, cap);

    }
    return Ok(HttpResponse::Ok().into());
}