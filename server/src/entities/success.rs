use actix_web::{http::header::ContentType, HttpResponse, HttpResponseBuilder};
use serde::Serialize;
use serde_json::to_string;

pub struct AppSuccess {}

#[allow(dead_code)]
impl AppSuccess {
    pub fn ok<T: Serialize>(data: &T) -> HttpResponse {
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(AppSuccess::to_response_body(data));
    }
    pub fn status<T: Serialize>(status: i32, data: &T) -> HttpResponse {
        return AppSuccess::to_response_builder(status)
            .content_type(ContentType::json())
            .body(AppSuccess::to_response_body(data));
    }
    fn to_response_builder(status: i32) -> HttpResponseBuilder {
        return match status {
            201 => HttpResponse::Created(),
            202 => HttpResponse::Accepted(),
            204 => HttpResponse::NoContent(),
            _ => HttpResponse::Ok(),
        };
    }
    fn to_response_body<T: Serialize>(data: &T) -> String {
        return match to_string(data) {
            Ok(data) => data,
            Err(err) => panic!("[Error Serialize] {:?}", err),
        };
    }
}
