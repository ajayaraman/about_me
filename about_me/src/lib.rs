use rocket::{get, fs::NamedFile};
use std::path::Path;

#[get("/")]
pub async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).await.ok()
}