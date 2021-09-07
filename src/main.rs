use std::cmp::Ordering;

mod compare;

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

            match compare::are_same(first, second) {
                Ok(true) => {
                    println!("Pictures are the same")
                }
                Ok(false) => {
                    println!("Pictures are different")
                }
                Err(e) => {
                    eprint!("An Error occurred: {}", e)
                }
            }
        }
    }
}
