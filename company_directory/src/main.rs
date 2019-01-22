use std::io;
use std::collections::HashMap;

fn print_menu() {
    println!("{:=<17} Menu {0:=<17}", "");
    println!("Add <Employe name> to <Department Name>");
    println!("Help -> Print this menu");
    println!("List <Department Name>");
    println!("All -> List all people by department");
    println!("Exit -> Exit program, all data will be lost!!!");
    println!("{:=<40}", "");
}

fn get_user_input() -> String {
    println!("Input a command: ");

    let mut input = String::new();
    let read_bytes = io::stdin().read_line(&mut input).unwrap_or(0);
    if read_bytes > 0 {
        input = input.trim().to_string();
    }

    input
}

enum Command {
    Add(String, String),
    ListDepartment(String),
    ListAll,
    Help,
    Exit,
    Err,
}

fn parse_input(input: &String) -> Command {
    let mut splitted_input = input.split_whitespace();

    let command = match splitted_input.next() {
        Some("Add") => {
            let name = splitted_input.next();
            let _to = splitted_input.next();
            let department = splitted_input.next();

            if name.is_some() && department.is_some() {
                return Command::Add(
                    String::from(name.expect("Name was not given")),
                    String::from(department.expect("Department was not given")),
                );
            } else {
                return Command::Err;
            }
        }
        Some("Help") => Command::Help,
        Some("List") => {
            let department = splitted_input.next();
            match department {
                Some(department_name) => {
                    return Command::ListDepartment(String::from(department_name))
                }
                None => return Command::Err,
            }
        }
        Some("All") => Command::ListAll,
        Some("Exit") => Command::Exit,
        _ => Command::Err,
    };

    command
}

fn print_persons_in_department(department: &String, department_list: &Vec<String>) {
    println!("Persons in {} department: ", department);
    for person in department_list{
        println!("{}", person);
    }
}

fn process_command(command: Command, directory: &mut HashMap<String, Vec<String>>) -> bool {

    match command {
        Command::Add(name, department) => {
            let department_list = directory.entry(department).or_insert(Vec::new());
            department_list.push(name);
        },
        Command::Help => print_menu(),
        Command::ListDepartment(department) => {
            let department_list = directory.entry(department.clone()).or_insert(Vec::new());
            print_persons_in_department(&department, department_list);

        },
        Command::ListAll => {
            for (department, department_list) in directory {
                print_persons_in_department(department, department_list);
            }
        },
        Command::Exit => return false,
        Command::Err => println!("Error with input, please input a valid command"),
    }

    true
}

fn main() {
    print_menu();

    let mut directory : HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let input = get_user_input();
        let command = parse_input(&input);

        if !process_command(command, &mut directory) {
            break;
        }
    }
}
