use crate::todo::Todo;
use std::io::Write;

const DEFAULT_ID_WIDTH: usize = 3;
const DEFAULT_NAME_WIDTH: usize = 50;
const DEFAULT_COMPLETED_WIDTH: usize = 10;
const DEFAULT_LINE_OFFSET: usize = 10;

pub struct UIManager {
    id_width: usize,
    name_width: usize,
    completed_width: usize,
    line_width: usize,
}

impl UIManager {
    pub fn new() -> Self {
        UIManager {
            id_width: DEFAULT_ID_WIDTH,
            name_width: DEFAULT_NAME_WIDTH,
            completed_width: DEFAULT_COMPLETED_WIDTH,
            line_width: DEFAULT_ID_WIDTH
                + DEFAULT_NAME_WIDTH
                + DEFAULT_COMPLETED_WIDTH
                + DEFAULT_LINE_OFFSET,
        }
    }

    pub fn from(
        id_width: usize,
        name_width: usize,
        completed_width: usize,
        line_offset: usize,
    ) -> Self {
        UIManager {
            id_width,
            name_width,
            completed_width,
            line_width: id_width + name_width + completed_width + line_offset,
        }
    }

    pub fn display_todos(&self, todos: &Vec<&Todo>) {
        println!(
            "{3: <0$} | {4: <1$} | {5: <2$}",
            self.id_width, self.name_width, self.completed_width, "ID", "Name", "Completed"
        );
        println!("{:=<1$}", "", self.line_width);

        for todo in todos {
            let lines = textwrap::wrap(todo.get_name().as_str(), 30);

            for (index, line) in lines.iter().enumerate() {
                if index == 0 {
                    println!(
                        "{3: <0$} | {4: <1$} | {5: <2$}",
                        self.id_width,
                        self.name_width,
                        self.completed_width,
                        todo.get_id(),
                        line,
                        todo.is_completed()
                    );
                } else {
                    println!(
                        "{3: <0$} | {4: <1$} | {5: <2$}",
                        self.id_width, self.name_width, self.completed_width, "", line, ""
                    );
                }
            }
            println!("{:-<1$}", "", self.line_width);
        }
    }

    pub fn get_input(message: &str) -> String {
        print!("{}", message);
        std::io::stdout().flush().expect("Flush failed");

        let mut input = String::new();

        std::io::stdin().read_line(&mut input).expect("Read failed");

        input
    }
}
