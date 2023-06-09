use crate::{err_response, ok_response, verify_token, SimpleResponse, SimpleResponseStructure};

#[rocket::get("/?<token>")]
pub fn template(_state: &rocket::State<crate::State>, token: Option<String>) -> SimpleResponse {
    verify_token!(token);

    if true {
        ok_response!("")
    } else {
        err_response!("")
    }
}
