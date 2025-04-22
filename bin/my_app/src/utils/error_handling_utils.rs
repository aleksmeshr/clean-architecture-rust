use std::error::Error;

use domain::error::ApiError;

pub fn application_error(error_message: &str, error: Option<Box<dyn Error>>) -> ApiError {
    log_error(error_message, &error);
    ApiError { code: 400, message: String::from(error_message), error }
}

pub fn unauthorized_error() -> ApiError {
    let unauthorized_message = "Error: not authenticated or token expired";
    log_error(unauthorized_message, &None);
    ApiError { code: 401, message: String::from(unauthorized_message), error: None }
}

pub fn forbidden_error() -> ApiError {
    let forbdden_message = "Error: resource not allowed";
    log_error(forbdden_message, &None);
    ApiError { code: 403, message: String::from(forbdden_message), error: None }
}

fn log_error(message: &str, err: &Option<Box<dyn Error>>) {
    println!("Error: {}", message);
    if let Some(error) = err {
        println!("Stack: {}", error.to_string());
    }
}
