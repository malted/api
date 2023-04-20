use mongodb::bson::{doc, Document};
use mongodb::Client;
use rocket::serde::{json::Json, Serialize};
use std::env::var;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    success: bool,
}

#[rocket::patch("/visitors/<domain>?<token>")]
pub async fn patch_visitors(domain: String, token: Option<String>) -> Json<Response> {
    if token != Some(var("secret_token").unwrap()) {
        return Json(Response { success: false });
    }

    let client = Client::with_uri_str(var("mongo_uri").unwrap())
        .await
        .unwrap();

    let db_main = client.database("main");

    let collection_visitors: mongodb::Collection<Document> = db_main.collection("visitors");

    // Find the document with the domain name
    let domain_doc_opt: Option<_> = collection_visitors
        .find_one(mongodb::bson::doc! { "domain": &domain }, None)
        .await
        .unwrap();

    // If the document exists, increment the 'visitors' field
    if let Some(doc) = domain_doc_opt {
        let visitors = doc.get_i32("visitors").unwrap() + 1;
        collection_visitors
            .update_one(
                mongodb::bson::doc! { "domain": &domain },
                mongodb::bson::doc! { "$set": { "visitors": visitors } },
                None,
            )
            .await
            .unwrap();
    } else {
        // If the document doesn't exist, create it
        collection_visitors
            .insert_one(
                mongodb::bson::doc! { "domain": &domain, "visitors": 1 },
                None,
            )
            .await
            .unwrap();
    }

    Json(Response { success: true })
}
