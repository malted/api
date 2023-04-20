use rand::prelude::SliceRandom;
use rocket::fs::NamedFile;

struct Dinosaur;

impl Dinosaur {
    async fn random() -> NamedFile {
        let mut dir = std::env::current_dir().expect("the current directory");
        dir.push("data");
        dir.push("dinosaurs");

        // Get a random file ending in .png from the directory
        let file_name = std::fs::read_dir(&dir)
            .expect("the dinosaur directory")
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().map(|t| t.is_file()).unwrap_or(false))
            .filter(|entry| {
                entry
                    .path()
                    .extension()
                    .map(|ext| ext == "png")
                    .unwrap_or(false)
            })
            .map(|entry| entry.file_name().into_string().unwrap())
            .collect::<Vec<_>>()
            .choose(&mut rand::thread_rng())
            .expect("a random dinosaur")
            .to_string();

        // let img = ImageReader::open(&dir.join(&file_name))
        //     .expect("the dinosaur image")
        //     .decode() // fails on bat_kryptonyte_dino.png
        //     .expect("the decoded image");

        NamedFile::open(dir.join(&file_name))
            .await
            .expect("the dinosaur image")
    }
}

#[rocket::get("/random")]
pub async fn random() -> NamedFile {
    Dinosaur::random().await
}
