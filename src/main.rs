use std::io;

struct TodoItem {
    id: u32,
    name: String,
    completed: bool,
}

fn main() {
    let mut todo_list: Vec<TodoItem> = Vec::new();

    loop {
        println!("\nWhat would you like to do?");
        println!("1. Create a todo item");
        println!("2. Mark existing item completed");
        println!("3. Display items");
        println!("4. Quit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");

        let input = input.trim().parse().expect("invalid input");
        match input {
            1 => {
                println!("Enter the title");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("failed to read");

                let name = name.trim().to_string();

                let id = todo_list.len() as u32 + 1;
                let item = TodoItem {
                    id,
                    name,
                    completed: false,
                };

                todo_list.push(item);
            }
            2 => {
                println!("Enter id of the item you want to mark complete");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("invalid id");

                let id = id.trim().parse().expect("error");

                let item = todo_list.iter_mut().find(|i| i.id == id).unwrap();
                complete_item(item)
            }
            3 => {
                display_items(&todo_list);
            }
            4 => {
                println!("bye bye");
                return;
            }
            _ => {
                println!("invalid choice")
            }
        }
    }
}

fn complete_item(item: &mut TodoItem) {
    item.completed = true;
}

fn display_items(items: &Vec<TodoItem>) {
    for item in items {
        println!("{} - {} ({})", item.id, item.name, item.completed);
    }
}
