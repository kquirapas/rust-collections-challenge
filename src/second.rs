use std::io::stdin;

pub fn run() {
    println!("===== SECOND =====");
    // let mut input_string = String::new();
    // println!("Enter input string:");
    // stdin().read_line(&mut input_string).expect("Did not enter a correct string");
    
    // temporary
    let input_string = String::from("first second third apple");
    println!("input string: {}", input_string);


    let slices = input_string.trim().split_whitespace();

    let mut output_string_vec = Vec::new();
    for s in slices {
        let mut new_string = String::new();

        match &s[0..1] {
            "a" | "e" | "i" | "o" | "u" => {
                new_string.push_str(&s[..]);
                new_string.push_str("-hay");
            },
            _ => {
                new_string.push_str(&s[1..]);
                new_string.push_str("-");
                new_string.push_str(&s[0..1]);
                new_string.push_str("ay");
            }
        }

        output_string_vec.push(new_string);
    }
    
    println!("{}", output_string_vec.join(" "));
}
