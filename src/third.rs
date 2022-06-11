use std::io::{Error, ErrorKind};
use std::fs::File;

const PATH: &str = "./files/data.csv";

pub fn run() {
    println!("===== THIRD =====");


    let f = match File::open(PATH) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(PATH) {
                Ok(file_created) => {
                    println!("File created: {}", PATH);
                    file_created
                },
                Err(create_error) => panic!("File creation failed: {:?}", create_error)
            },
            other_error => {
                panic!("File open failed: {:?}", other_error);
            }
        }
    };
}
