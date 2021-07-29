use serde::{Deserialize, Serialize};
use actix_web::{
    HttpResponse,
    http::StatusCode,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    pub email: String,
    pub username: String,
    pub token: String,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct RegisterResponse {
//     pub message: String,
// }

// EROR RESPONSE
#[derive(Debug)]
pub struct ServiceError {
    pub http_status: StatusCode,
    pub body: ResponseBody<String>,
}

impl ServiceError {
    pub fn new(http_status: StatusCode, message: String) -> ServiceError {
        ServiceError {
            http_status,
            body: ResponseBody {
                message,
                data: String::new(),
            }
        }
    }

    pub fn response(&self) -> HttpResponse {
        HttpResponse::build(self.http_status).json(&self.body)
    }
}

// RESPONSE BODY
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody<T> {
    pub message: String,
    pub data: T,
}

impl<T> ResponseBody<T> {
    pub fn new(message: &str, data: T) -> ResponseBody<T> {
        ResponseBody {
            message: message.to_string(),
            data,
        }
    }
}
