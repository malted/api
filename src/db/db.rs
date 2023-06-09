use crate::{err_response, ok_response, verify_token, SimpleResponse, SimpleResponseStructure};

#[rocket::get("/<table>/<key>/<value>?<token>")]
pub fn set_kv(
    state: &rocket::State<crate::State>,
    table: &str,
    key: &str,
    value: &str,
    token: Option<String>,
) -> SimpleResponse {
    verify_token!(token);

    let conn = state.db_connection.lock().expect("lock db connection");

    let query = "UPDATE "
    conn.execute(query, (domain, 1)).expect("an insertion");

    // for (key, value) in rest.0 {

    //     println!("{}: {}", key, value);
    // }

    // Test url:

    if true {
        ok_response!("")
    } else {
        err_response!("")
    }
}
