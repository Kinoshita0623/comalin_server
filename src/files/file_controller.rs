use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use actix_web::web::Bytes;
use uuid::Uuid;
use std::io::Write;
use crate::app_module::AppModule;
use std::vec::Vec;
use std::str;
use crate::files::entities::AppFile;
use crate::errors::service_error::ServiceError;

use actix_web::{web, Error, HttpResponse};
use crate::files::services::*;


pub async fn upload(data: web::Data<AppModule>, mut payload: Multipart) -> Result<HttpResponse, Error>{
    let mut fields: Vec<MultipartField> = Vec::new();
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = match field.content_disposition() {
            Some(cd) => cd,
            None => continue
        };

        // Textとファイルの判定をしている。
        match content_disposition.get_filename() {
            Some(filename) => {
                let tmp_filename = Uuid::new_v4().to_string();
                let filepath = format!("./storage/tmp/{}", tmp_filename.clone());
                let filepath2 = filepath.clone();
                let mut f: std::fs::File = web::block(|| std::fs::File::create(filepath2))
                    .await
                    .unwrap();
                let mut cap = 0;
    
                while let Some(chunk) = field.next().await {
                    let data: Bytes = chunk.unwrap();
        
                    cap += data.len();
                    f = web::block(move || f.write_all(&data).map(|_| f)).await?;
                }
                let kind = match infer::get_from_path(filepath.clone())? {
                    Some(kind) => kind,
                    None => {
                        return Ok(HttpResponse::Ok().into());
                    }
                };
                let extenstion = kind.extension();
                let mime_type = kind.mime_type();
    
                fields.push(
                    MultipartField::File(
                        MultipartFile {
                            tmp_filepath: filepath,
                            capacity: cap,
                            extenstion: extenstion.to_string(),
                            mime_type: mime_type.to_string(),
                            raw_filename: filename.to_string()
                        }
                    )
                );
            },
            None => {
                let mut text: Vec<u8> = Vec::new();
                while let Some(chunk) = field.next().await {
                    let data: Bytes = chunk.unwrap();
                    for v in data {
                        text.push(v);
                    }
                }
                let t: &str = str::from_utf8(&text)?;
                fields.push(
                    MultipartField::Text {
                    text: t.to_string()
                    }
                );
            }
        };

    }
    let results: Vec<Result<AppFile, ServiceError>> = fields.iter().filter_map(|mf| match mf {
        MultipartField::File(f) => Some(f),
        MultipartField::Text{ text: _ } => None
    })
    .map(|mf| data.as_ref().file_module().app_file_service().save(mf))
    .collect();

    let files: Vec<AppFile> = results.iter().filter_map(|result| match result {
        Ok(file) => Some(file.clone()),
        Err(_) => None,
    }).collect();

    // TODO: エラー処理を適切に実装すること
    /*for field in fields {
        match field {
            MultipartField::File(f) => f,
            MultipartField::Text { text: _ } => {
                ServiceError::
            }
        }
    }*/
    
    /*let files: Vec<&AppFile> = results.iter().filter_map(|result| match result {
        Ok(file) => Some(file),
        Err(_) => None
    }).collect();
*/
    /*if results.len() != files.len() {
        return 
    }

    for result in reuslts {
        match result {
            Ok()
        }
    }*/
        

    return Ok(
        HttpResponse::Ok().json(files)
    );
        
    println!("fields:{:?}", fields);
    return Ok(HttpResponse::Ok().into());
}

