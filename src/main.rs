use std::io;
struct User {
    name: String,
    age: u32,
    profession: String,
}

fn main() {

    let mut list_of_workers: Vec<User> = Vec::new();

    loop {
        let mut direction = String::new();
        println!("Enter 'add', 'list' or 'quit' to exit: ");
        io::stdin()
            .read_line(&mut direction)
            .expect("Enter 'add', 'list' or 'quit' ");


        match direction.as_str().trim() {
            "add" => {
                
                let mut name: String = String::new();
                let mut profession: String = String::new();


                println!("Name: ");
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read name.");


                let age: u32 = loop {
                    println!("Age: ");
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read age.");
                    
                    match input.trim().parse() {
                        Ok(age) => break age,
                        Err(_) =>{
                            println!("Invalid age. Please enter a number.");
                            continue;
                        }
                    }
                };

                println!("Profession: ");
                io::stdin()
                    .read_line(&mut profession)
                    .expect("Failed to read profession");

                let name: String = name.trim().to_string();
                let profession: String = profession.trim().to_string();

                list_of_workers.push(User {
                    name,
                    age,
                    profession,
                });
            

            }
            "list" => {
                for val in &list_of_workers {
                    println!("\n===========\nName: {}\nAge: {}\nProfession: {}\n===========\n", val.name, val.age, val.profession);
                }
                continue;
            }
            "quit" => {
                break;
            }
            _ => {
                println!("Invalid command.");
            }
        }




    }
    



}