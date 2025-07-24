use std::ffi::{CString, CStr};
//use std::fmt::{Displa};
use std::os::raw::c_char;
use reqwest::blocking::Client;
/*
#[derive(Debug)]
enum HttpError {
    ReqwestError(Box<ReqwestError>),
    Other(String),
}

impl Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        match self {
            HttpError::ReqwestError(err) => write!(f, "Reqwest error: {}", err),
            HttpError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}
*/
#[no_mangle]
pub extern "C" fn send_request(
    method: *const c_char, 
    url: *const c_char, 
    body: *const c_char) -> *mut c_char
{
    // cpnvertir los punteros C a string de rust 
    let method = unsafe {CStr::from_ptr(method).to_str().unwrap().to_owned()};
    let url = unsafe {CStr::from_ptr(url).to_str().unwrap().to_owned()};
    let body = unsafe {CStr::from_ptr(body).to_str().unwrap().to_owned()};

    //realizar las solicitudes http usando request 
    let result = match method.as_str(){
        "GET" => Client::new().get(&url).send(),
        "POST"=> Client::new()
            .post(&url)
            .body(body)
            .send(),
        "PUT" => Client::new()
            .put(&url)
            .body(body)
            .send(),
        "DELETE" => Client::new()
            .delete(&url)
            .send(),

        _ => return CString::new("Error: Invalid HTTP method").unwrap().into_raw(),

        //_ => Err(reqwest::Error::new(reqwest::error::kind::Reqwest, Some("Invalid HTTP method"))),

    };

    //convertir la respuesta o error en string
    let response = match result {
        Ok(response) => match response.text() {
            Ok(text) => text,
            Err(err) => format!("Error reading response {}", err),
        },
        Err(err) => format!("Error {}", err),
    };

    CString::new(response).unwrap().into_raw()

}









