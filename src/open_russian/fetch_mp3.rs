use std::{fs::File, io::Write, path::PathBuf};

pub async fn fetch_mp3(russian: &str, path: PathBuf) {
    let encoded = urlencoding::encode(russian);
    let url = format!("https://api.openrussian.org/read/ru/{encoded}");

    println!("{url}");
    let bytes = reqwest::get(url).await.unwrap().bytes().await.unwrap();

    let mut file = File::create(path).unwrap();
    file.write_all(&bytes).unwrap();
}
