use actix_web::{http::header::ContentType, HttpResponse, HttpResponseBuilder};
use serde::Serialize;
use serde_json::to_string;

#[derive(Serialize)]
pub struct AppError {
    pub message: String,
    #[serde(skip_serializing)]
    status_code: i32,
    #[serde(skip_serializing)]
    private_message: String,
}

impl AppError {
    pub fn new(status_code: i32, message: &str, private_message: String) -> AppError {
        Self {
            status_code,
            message: String::from(message),
            private_message,
        }
    }
    pub fn handle(error: &AppError) -> HttpResponse {
        return error
            .to_response_builder()
            .content_type(ContentType::json())
            .body(error.to_response_body());
    }
    fn to_response_builder(&self) -> HttpResponseBuilder {
        return match self.status_code {
            400 => HttpResponse::BadRequest(),
            401 => HttpResponse::Unauthorized(),
            403 => HttpResponse::Forbidden(),
            404 => HttpResponse::NotFound(),
            _ => HttpResponse::InternalServerError(),
        };
    }
    fn to_response_body(&self) -> String {
        self.print_error();
        return match to_string(self) {
            Ok(response_error) => response_error,
            Err(err) => panic!("[Error Serialize] {:?}", err),
        };
    }
    fn print_error(self: &Self) {
        if self.private_message == String::from("") {
            println!("[Error] {}", self.message);
            return;
        }
        println!("[Private Error] {}", self.private_message);
        return;
    }
}
