use std::io::stdin;
use std::io::Read;
use std::io::ErrorKind;
use std::fs;
use std::fs::File;

const PATH: &str = "./files/data.csv";

pub fn run() {
    println!("===== THIRD =====");


    let mut f = match File::open(PATH) {
        Ok(file) => {
            println!("File opened: {}", PATH);
            file
        },
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


    let mut contents = String::new();

    match f.read_to_string(&mut contents) {
        Ok(size) => size,
        Err(error) => panic!("Failed to read contents from file: {:?}", error)
    };

    let contents = contents.trim();
    let mut lines: Vec<String> = contents.split("\n").map(String::from).collect();

    loop {
        println!("stat loop");
        let mut command: String = String::new();
        if let Err(_) = stdin().read_line(&mut command) {
            println!("Failed to read input");
            continue;
        }

        let input = command.trim().split_whitespace().collect::<Vec<&str>>();
        match input[0] {
            "add" => {
                let mut record = input[1..=2].join(" ");
                record.push_str(",");
                record.push_str(input[3]);
                
                lines.push(record);
            },
            "q" => {
                println!("Quitting");
                break;
            },
            _ => println!("Invalid command")
        }
    };

    for l in &lines {
        let cells = l.split(",").map(String::from).collect::<Vec<String>>();
        println!("{:<10}{:<20}", cells[0], cells[1]);
    }

    fs::write(PATH, lines.join("\n").as_bytes()).expect("sheesh");
}
