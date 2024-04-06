use std::{io, process::exit};

fn main() {
    println!(
        "Hello! Sounds like you've got a nasty case of imperial units. I can solve that for you."
    );

    'a: loop {
        println!("Please input your value in degrees Fahrenheight:");

        let mut deg_f = String::new();

        io::stdin().read_line(&mut deg_f).expect("Uh oh!");

        let deg_f: f32 = match deg_f.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error: not a number");
                continue;
            }
        };

        let deg_c = (deg_f - 32.0) * 5.0 / 9.0;

        println!("{deg_f}°F <-> {deg_c}°C");

        loop {
            println!("Convert another? y/n");

            let mut answer = String::new();
            io::stdin().read_line(&mut answer).expect("Uh oh!");

            let answer = answer.trim().to_lowercase();

            if answer == "y" {
                continue 'a;
            } else if answer == "n" {
                break 'a;
            } else {
                println!("Must be y/n");
            }
        }
    }

    exit(0);
}
