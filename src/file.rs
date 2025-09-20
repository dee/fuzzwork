use bzip2::Decompress;
use std::fs::{File, create_dir_all};
use std::io::{Read, Write};
use std::path::Path;
use std::fs;

pub fn check_file(p: &Path) -> bool {
    let path = &p.to_str().unwrap().to_string();
    println!("Checking: {}", path);
    create_dir_all(p.to_path_buf().parent().unwrap()).unwrap();
    if !p.exists() {
        println!("Path does not exist: {}", path);
        return false;
    }
    let md = fs::metadata(p).unwrap();
    println!("File length is: {}", &md.len().to_string());
    md.len() > 1000000
}

fn read_into_buffer(p: &Path) -> Vec<u8> {
    let mut f = File::open(p).unwrap();
    let metadata = fs::metadata(p).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read_exact(&mut buffer).unwrap();
    buffer
}

pub fn unpack(target: &Path) {
    let source = target.to_path_buf().with_extension("tar.bz2");
    println!("Unpacking {}", &String::from(source.to_str().unwrap()));

    let mut decoder = Decompress::new(false);
    let buf: Vec<u8> = read_into_buffer(&source);
    //println!("Read {} bytes from source", buf.len());

    if target.exists() {
        fs::remove_file(target).unwrap();
    }

    let mut output = File::create(target).unwrap();

    //TODO: how to calculate resulting size?
    let mut out_buf: Vec<u8> = Vec::with_capacity(5 * buf.len());
    decoder.decompress_vec(&buf, &mut out_buf).unwrap();
    let _sz = output.write(out_buf.as_ref()).unwrap();
    //println!("Wrote {} bytes", sz);
    println!("Decompress finished!");

    fs::remove_file(source).unwrap();
}
