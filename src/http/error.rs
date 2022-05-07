#[derive(thiserror::Error, Debug)]
pub enum UnafHttpError
{
 #[error("std::io::Error")]
 StdIoError(#[from] std::io::Error),

 #[error("std::string::FromUtf8Error")]
 StdStringFromUtf8Error(#[from] std::string::FromUtf8Error),

 #[error("unknown")]
 Unknown
}

impl actix_web::error::ResponseError for UnafHttpError {}

pub type UnafHttpResponseResult = Result<actix_web::HttpResponse, UnafHttpError>;
