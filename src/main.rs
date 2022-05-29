pub mod todo;
pub mod todo_manager;
pub mod ui_manager;

use std::{env, fs};
use todo::Todo;
use todo_manager::TodoManager;
use ui_manager::UIManager;

fn main() {
    let data_path = env::temp_dir().join("rust-todo-app-data.json");

    let ui = UIManager::new();

    let mut manager = {
        match data_path.exists() {
            true => {
                let data = fs::read_to_string(&data_path).unwrap();

                serde_json::from_str::<TodoManager>(&data).unwrap()
            }
            false => TodoManager::new(vec![]),
        }
    };

    println!("TODO APP\n");
    println!(
        "1.Add todo\n2.Mark todo as completed\n3.Delete todo\n4.Show completed todo\n5.Show remaining todo\n6.Exit"
    );

    loop {
        let choice = UIManager::get_input("\nEnter choice :");
        let choice = choice.trim();

        match choice {
            "1" => {
                let name = UIManager::get_input("Enter todo name :");

                let todo = manager.add_todo(name.trim());

                println!("Added new todo with id = {}", todo.get_id());
            }
            "2" => {
                let id = UIManager::get_input("Enter todo id :");
                let id: usize = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid ID");
                        continue;
                    }
                };

                match manager.mark_todo(id) {
                    Ok(_) => println!("Marked completed for todo with id = {}", id),
                    Err(_) => println!("Invalid ID"),
                };
            }
            "3" => {
                let id = UIManager::get_input("Enter todo id :");
                let id: usize = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid ID");
                        continue;
                    }
                };

                match manager.delete_todo(id) {
                    Ok(_) => println!("Deleted todo with id = {}", id),
                    Err(_) => println!("Invalid ID"),
                };
            }
            "4" => {
                let completed: Vec<&Todo> = manager
                    .get_todos()
                    .iter()
                    .filter(|todo| todo.is_completed())
                    .collect();

                if completed.len() != 0 {
                    ui.display_todos(&completed);
                } else {
                    println!("No completed todos");
                }
            }
            "5" => {
                let remaining: Vec<&Todo> = manager
                    .get_todos()
                    .iter()
                    .filter(|todo| !todo.is_completed())
                    .collect();

                if remaining.len() != 0 {
                    ui.display_todos(&remaining);
                } else {
                    println!("No remaining todos");
                }
            }
            "6" => break,
            _ => println!("Invalid option"),
        }

        manager.save(&data_path);
    }
}
