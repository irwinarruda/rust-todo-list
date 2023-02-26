use crate::entities::error::AppError;
use std::{
    fs::{read_to_string, write},
    path::Path,
};

pub struct File;
impl File {
    pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> Result<(), AppError> {
        return match write(path, contents) {
            Ok(_) => Ok(()),
            Err(err) => Err(AppError::new(
                500,
                "Error writing to file system",
                err.to_string(),
            )),
        };
    }
    pub fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String, AppError> {
        return match read_to_string(path) {
            Ok(data) => Ok(data),
            Err(err) => Err(AppError::new(
                500,
                "Error reading json file",
                err.to_string(),
            )),
        };
    }
}
