use std::{collections::HashMap, io};

extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
struct Company {
    structure: HashMap<String, Vec<String>>,
}

impl Company {
    fn new() -> Self {
        Company {
            structure: HashMap::new(),
        }
    }

    fn add_employee(self: &mut Self, department: &str, name: &str) {
        self.structure
            .entry(department.to_string())
            .or_insert(Vec::new())
            .push(name.to_string());
    }

    fn get_employee(self: &Self, department: &str, name: &str) {
        let dept_employees = self.structure.get(department);

        let dept_employees = match dept_employees {
            Some(dept_employees) => dept_employees,
            None => {
                println!("Department does not exist.");
                return;
            }
        };

        let is_employee = dept_employees.contains(&name.to_string());

        match is_employee {
            true => {
                println!("Found!");
            }
            false => {
                println!("Not found!");
            }
        }
    }
}

enum Command {
    Add,
    Get,
    Print,
    Invalid,
}

fn add(company: &mut Company, args: &Vec<&str>) {
    // Expect args to be "<name>" "to" "<department>"
    let dept = args.get(2).expect("Expected a department name");
    let employee = args.get(0).expect("Expected an employee name");

    println!("Adding {employee} to {dept}");
    company.add_employee(dept, employee);
}

fn get(company: &mut Company, args: &Vec<&str>) {
    // Expect args to be "<name>" "from" "<department>"
    let dept = args.get(2).expect("Expected a department name");
    let employee = args.get(0).expect("Expected an employee name");

    println!("Checking {employee} from {dept}");
    company.get_employee(dept, employee);
}

fn handle_command(company: &mut Company, command: &str) {
    let mut command_args = command.unicode_words();
    let root = command_args.next();

    let root_cmd = match root.expect("Couldn't get root cmd").to_lowercase().as_str() {
        "add" => Command::Add,
        "get" => Command::Get,
        "print" => Command::Print,
        _ => Command::Invalid,
    };

    let args = command_args.collect::<Vec<&str>>();

    // println!("{:#?}", args);

    match root_cmd {
        Command::Add => add(company, &args),
        Command::Get => get(company, &args),
        Command::Print => {
            println!("Current company: {:#?}", company);
        }
        Command::Invalid => {
            println!("Invalid command!");
        }
    }
}

fn print_usage() {
    println!("Usage:");
    println!("    Add <name> to <department>");
    println!("    Get <name> from <department>");
    println!("    print");
    println!("    quit/exit/q");
    println!("=====================================");
}

fn main() {
    let mut company = Company::new();

    println!("Build your company!");
    print_usage();

    loop {
        println!("Command:");
        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Cannot read in command!");

        let quit = match command.to_lowercase().trim() {
            "quit" => true,
            "exit" => true,
            "q" => true,
            _ => false,
        };

        if quit {
            println!("Later gator!");
            break;
        } else {
            handle_command(&mut company, &command);
        }
    }
}
