use std::io;

// input: Temperature, Unit eg. 100 -c
fn main() {
    let delimiter: char = ' ';

    loop {
        println!("Please enter Temperature and desired temperature unit");
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input from User");

        input = input.trim().to_owned();

        let input: Vec<&str> = input.split(delimiter).collect();

        if input.len() != 2 {
            println!("Missing Input Arguments");
            continue;
        }

        let temp:f32 = match input[0].trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
        let unit:&str = input[1].trim();
    
    match unit {
        "-c" => println!("{}",convert_c(temp)),
        "-f" => println!("{}", convert_f(temp)),
        _ => println!("use -f or -c"),
    };
    }
}

fn convert_c(num:f32) -> f32{
    (num * (9 as f32 /5 as f32)) + 32 as f32
}

fn  convert_f(num:f32) ->f32 {
    (num - 32 as f32) * (5 as f32 / 9 as f32)
}
