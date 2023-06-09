use crate::State;
use htmlentity::entity::decode;
use lazy_static::lazy_static;
use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Tags {
    title: Option<String>,
    description: Option<String>,
    image: Option<String>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct RootResponse {
    success: bool,
    message: Tags,
}

#[rocket::get("/<domain>")]
pub async fn get_ogp(state: &rocket::State<State>, domain: String) -> Json<RootResponse> {
    let res = state
        .client
        .get(format!("https://{}/", domain))
        .header("Range", "bytes=0-4096")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    lazy_static! {
        static ref TITLE_RE: regex::Regex =
            regex::Regex::new(r#"<meta property="og:title" content=.+?\/>"#).unwrap();
    }
    lazy_static! {
        static ref DESC_RE: regex::Regex =
            regex::Regex::new(r#"<meta property="og:description" content=.+?\/>"#).unwrap();
    }
    lazy_static! {
        static ref IMAGE_RE: regex::Regex =
            regex::Regex::new(r#"<meta property="og:image" content=.+?\/>"#).unwrap();
    }

    let get_match = |tag: &str| -> Option<String> {
        let matches = match tag {
            "title" => TITLE_RE.find(&res),
            "description" => DESC_RE.find(&res),
            "image" => IMAGE_RE.find(&res),
            _ => None,
        };

        match matches {
            Some(m) => Some(decode(
                m.as_str()
                    .to_string()
                    .replace(
                        format!(r#"<meta property="og:{}" content=""#, tag).as_str(),
                        "",
                    )
                    .replace(r#""/>"#, "")
                    .as_str(),
            )),
            None => None,
        }
    };

    let title = get_match("title");
    let description = get_match("description");
    let image = get_match("image");

    let tags = Tags {
        title,
        description,
        image,
    };

    Json(RootResponse {
        success: true,
        message: tags,
    })
}
