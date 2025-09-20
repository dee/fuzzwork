use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::fs::File;

static USERAGENT: &str = "Fuzzworks database updater";
static URL: &str = "https://www.fuzzwork.co.uk/dump/sqlite-latest.sqlite.bz2";

fn create_client() -> reqwest::Client {
    reqwest::Client::builder()
        .user_agent(USERAGENT)
        .build()
        .unwrap()
}

pub async fn get_size() -> u64 {
    let default_length: u64 = 0;
    let client = create_client();

    let resp = client.head(URL).send().await.unwrap();
    let header_str = resp
        .headers()
        .get("Content-Length")
        .unwrap()
        .to_str()
        .unwrap();
    header_str.parse::<u64>().unwrap_or(default_length)
}

pub fn get_dest_fn(p0: &Path) -> PathBuf {
    p0.to_path_buf().with_extension("tar.bz2")
}

pub async fn download_file(p0: &Path) {
    println!("Downloading file: {}", &String::from(URL));
    let expected_size: u64 = get_size().await;

    let client = create_client();
    let mut response = client.get(URL).send().await.unwrap();

    // calculate dest
    let mut target = File::create(get_dest_fn(p0)).unwrap();
    //let mut buffer : [u8; 1024] = [0; 1024];

    let mut actual_size: u64 = 0;
    //prg.start_progress(expected_size);
    while let Some(chunk) = response.chunk().await.unwrap() {
        actual_size += chunk.len() as u64;
        target.write_all(&chunk).unwrap();
        //prg.report_progress(actual_size);
    }
    //prg.report(format!("File downloaded to {}", dest_pb.to_str().unwrap()).as_str());
    //prg.stop_progress();
}
