use image::io::Reader as ImageReader;
use std::cmp::Ordering;
use std::path::Path;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    match args.len().cmp(&3) {
        Ordering::Greater => {
            eprintln!("Too many arguments. expected 2 image files as arguments")
        }
        Ordering::Less => {
            eprintln!("Not enough arguments. expected 2 image files as arguments")
        }
        Ordering::Equal => {
            let first = args.get(1).expect("first arg");
            let second = args.get(2).expect("first arg");

            match are_same(first, second) {
                Ok(true) => {
                    println!("Images are the same!")
                }
                Ok(false) => {
                    println!("Images are NOT the same!")
                }
                Err(e) => {
                    eprint!("An Error occurred: {}", e)
                }
            }
        }
    }
}

fn are_same(first: impl AsRef<Path>, second: impl AsRef<Path>) -> Result<bool, anyhow::Error> {
    let first = ImageReader::open(first)?.decode()?;
    let second = ImageReader::open(second)?.decode()?;
    println!("{:?}", first);
    println!("{:?}", second);
    Ok(false)
}
