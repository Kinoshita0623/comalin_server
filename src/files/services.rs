use crate::files::module::AppFileModule;
use crate::files::entities::*;
use crate::errors::service_error::ServiceError;
use std::fs;
use uuid::Uuid;
use blake2::{Blake2b, Digest};
use std::io;
use log::error;
use validator::Validate;
use crate::config::AppConfig;
use validator::ValidationError;


#[derive(Debug, Validate)]
pub struct MultipartFile {
    pub tmp_filepath: String,

    #[validate(custom(function="within_capacity", arg="&'v_a AppConfig"))]
    pub capacity: usize,
    pub extenstion: String,

    #[validate(custom(function="is_video_or_image"))]
    pub mime_type: String,
    pub raw_filename: String,
}   

pub fn is_video_or_image(value: &str) -> Result<(), ValidationError> {
    if value.starts_with("video") || value.starts_with("image"){
        return Ok(());
    }
    return Err(ValidationError::new("now_allowed_file_types"));
}

pub fn within_capacity(value: usize, arg: &AppConfig) -> Result<(), ValidationError> {
    if value < arg.max_file_capacity {
        return Ok(());
    }
    return Err(ValidationError::new("now_allowed_file_capacity"));
}

#[derive(Debug)]
pub struct MultipartText {
    pub body: String
}

#[derive(Debug)]
pub enum MultipartField {
    File (MultipartFile),
    Text {
        text: String
    }
}

#[derive(Debug)]
pub struct MultipartFields {

}

pub trait AppFileService {
    fn save(&self, file: &MultipartFile) -> Result<AppFile, ServiceError>;
}

pub struct AppFileServiceImpl {
    pub file_module: Box<dyn AppFileModule>
}

impl AppFileService for AppFileServiceImpl {
    fn save(&self, file: &MultipartFile) -> Result<AppFile, ServiceError> {

        
        let mut f = match fs::File::open(&file.tmp_filepath) {
            Ok(f) => f,
            Err(_) => return Err(
                ServiceError::InternalError {
                    body: None
                }
            )
        };
        let mut hasher = Blake2b::new();
        if let Err(e) = io::copy(&mut f, &mut hasher) {
            error!("ファイルのハッシュ値取得に失敗: {}", e.to_string());
            return Err(
                ServiceError::InternalError { body: None }
            );
        }
        let hash = hasher.finalize();

        let str_hash = hash.iter().map(|n| format!("{:02X}",n)).collect::<String>();

        let e = match self.file_module.app_file_reposiitory().find_by_hash(&str_hash) {
            Ok(f) => {
                // NOTE: 一時ファイルを削除する
                // TODO: 応答速度向上のためイベントを発行し別スレッドで実行できるようにする
                return match fs::remove_file(file.tmp_filepath.clone()) {
                    Ok(_) => Ok(f),
                    Err(_) => Err(
                        ServiceError::InternalError { body: None }
                    )
                };
            },
            Err(e) => e
        };

        // NOTE: NotFoundの時はまだ登録されていないファイルというなので正常
        if e != ServiceError::NotFoundError {
            return Err(e);
        }

        let filename = Uuid::new_v4().to_string() + &file.extenstion;
        let path = format!("./storage/public/{}", filename);

        if let Err(e) = fs::rename(file.tmp_filepath.clone(), path) {
            return Err(
                ServiceError::InternalError {
                    body: Some(e.to_string())
                }
            );
        };


        let new_file = NewAppFile {
            id: Uuid::new_v4(),
            filename: filename.to_string(),
            mime_type: file.mime_type.clone(),
            raw_name: file.raw_filename.clone(),
            hash: str_hash.to_string()
        };
        let file = match self.file_module.app_file_reposiitory().create(&new_file) {
            Ok(file) => file,
            Err(e) => {
                return Err(e.into())
            }
        };
        
        return Ok(file);
    }
}