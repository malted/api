use crate::{err_response, ok_response, verify_token, SimpleResponse, SimpleResponseStructure};

#[rocket::get("/visitors/<domain>")]
pub fn get_visitors(state: &rocket::State<crate::State>, domain: &str) -> SimpleResponse {
    let conn = state.db_connection.lock().expect("lock db connection");

    // Get the count
    let stmt = conn.prepare("SELECT count FROM visitors WHERE domain = ?1");

    let mut stmt = match stmt {
        Ok(stmt) => stmt,
        Err(_) => {
            return err_response!("This domain is not set up with visitor tracking.");
        }
    };

    let rows = stmt
        .query_map([domain], |row| row.get(0))
        .expect("a query map")
        .map(|row| row.unwrap())
        .collect::<Vec<i64>>();

    match rows.get(0) {
        Some(count) => ok_response!(*count),
        None => err_response!("This domain is not set up with visitor tracking."),
    }
}

#[rocket::patch("/visitors/<domain>?<token>")]
pub fn increment_visitors(
    state: &rocket::State<crate::State>,
    domain: &str,
    token: Option<String>,
) -> SimpleResponse {
    verify_token!(token);

    let conn = state.db_connection.lock().expect("lock db connection");

    // Create table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS visitors (
            domain TEXT PRIMARY KEY,
            count INTEGER NOT NULL DEFAULT 0
        )",
        (),
    )
    .expect("create table");

    let query = "INSERT INTO visitors (domain, count) VALUES (?1, ?2) ON CONFLICT(domain) DO UPDATE SET count = count + 1";
    conn.execute(query, (domain, 1)).expect("an insertion");

    let mut stmt = conn
        .prepare("SELECT count FROM visitors WHERE domain = ?1")
        .expect("a select");

    let rows = stmt
        .query_map([domain], |row| row.get(0))
        .expect("a query map");

    ok_response!(
        *(rows
            .map(|row| row.unwrap())
            .collect::<Vec<i64>>()
            .get(0)
            .unwrap())
    )
}
