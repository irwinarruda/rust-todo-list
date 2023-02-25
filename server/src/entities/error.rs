use actix_web::{http::header::ContentType, HttpResponse};
use serde::Serialize;
use serde_json::to_string;

#[derive(Serialize)]
pub struct AppError {
    pub public_message: String,
    #[serde(skip_serializing)]
    private_message: String,
}

impl AppError {
    pub fn new(public_message: &str, private_message: String) -> AppError {
        Self {
            public_message: String::from(public_message),
            private_message,
        }
    }
    pub fn print_error(self: &Self) {
        if self.private_message == String::from("") {
            println!("[Error] {}", self.public_message);
            return;
        }
        println!("[Private Error] {}", self.private_message);
        return;
    }
    pub fn to_response_body(&self) -> String {
        self.print_error();
        return match to_string(self) {
            Ok(response_error) => response_error,
            Err(err) => panic!("[Error Serialize] {:?}", err),
        };
    }
    pub fn internal_server_error(err: AppError) -> HttpResponse {
        return HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body(err.to_response_body());
    }
}
