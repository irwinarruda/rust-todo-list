use crate::entities::error::AppError;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, to_string_pretty};

pub struct Json;

#[allow(dead_code)]
impl Json {
    pub fn from_str<'a, T: Deserialize<'a>>(data: &'a str) -> Result<T, AppError> {
        return match from_str(data) {
            Ok(value) => Ok(value),
            Err(err) => Err(AppError::new(
                500,
                "Error deserializing json string",
                err.to_string(),
            )),
        };
    }
    pub fn to_string<T: Serialize + ?Sized>(data: &T) -> Result<String, AppError> {
        return match to_string(data) {
            Ok(value) => Ok(value),
            Err(err) => Err(AppError::new(
                500,
                "Error serializing json string",
                err.to_string(),
            )),
        };
    }
    pub fn to_string_pretty<T: Serialize + ?Sized>(data: &T) -> Result<String, AppError> {
        return match to_string_pretty(data) {
            Ok(value) => Ok(value),
            Err(err) => Err(AppError::new(
                500,
                "Error serializing json string",
                err.to_string(),
            )),
        };
    }
}
