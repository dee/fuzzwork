use std::path::PathBuf;
use std::env;

mod file;
mod http;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub async fn update() {
    println!("Updating database.");
    let path = get_database_path();
    let p0 = path.as_path();
    if !file::check_file(&p0) {
        http::download_file(p0).await;
        file::unpack(p0);
    }
    println!("Database update finished.");
}

//TODO: Linux
fn program_data() -> PathBuf {
    let p = env::var("ProgramData").unwrap_or(String::from("C://ProgramData"));
    PathBuf::from(p)
}

fn get_database_path() -> PathBuf {
    program_data()
        .join("fuzzworks")
        .join("sqlite-latest.sqlite")
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/