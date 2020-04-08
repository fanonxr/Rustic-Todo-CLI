use std::env;

struct TodoItem {
    name:String,
    completed: char,
}
// interface to create a todo item that will have a blank completeted
impl TodoItem {
    fn new(name: String) -> TodoItem {
        return TodoItem {
            name,
            completed: '_'
        }
    }
}

// Struct to hold the todo list data
struct TodoList {
    list: Vec<TodoItem>
}

impl TodoList {
    fn new() -> TodoList {
        return TodoList{list: Vec::new()};
    }
    // add todo list
    fn add_to_list(&mut self, name: String) {
        // create the todo item to be added
        let todo_item = TodoItem::new(name);
        // pushing in a todo item into the list
        self.list.push(todo_item);
    }

    // function to print the contents within the todo list
    fn print(&self) {
        // we have to reference the list
        for (index, item) in self.list.iter().enumerate() {
            // print all the items and their stats
            println!("{} [{}] - {}", index, item.completed, item.name);
        }
    }

    fn mark_done(&mut self, index: usize) {
        self.list[index].completed = 'x';
    }
}

// creating the ENUM type to match our commands to what they need to do
enum Command {
    Get,
    Add(String),
    Done(usize),
}

fn main() {
    // arguments that will be collected from the command line and stored in this vector
    let arguments:Vec<String> = env::args().collect();
    // let command:String = arguments[1].clone(); // making a deep copy of the first argument

    let command = match arguments[1].as_str() {
        "get" => Command::Get,
        "add" => Command::Add(arguments[2].clone()),
        "done" => Command::Done(arguments[2].parse().expect("Error converting to an integer")),
        _ => panic!("You must provide a proper command line arg")
    };

    // create a list to store all of the todo items
    let mut todo_list = TodoList::new();

    todo_list.add_to_list("Say Hi to X".to_string());
    todo_list.add_to_list("Learn a new thing about RUST".to_string());


    // logic to execute specific commands
    match command {
        Command::Get => todo_list.print(),
        Command::Add(task) => {
            let task = arguments[2].clone();
            todo_list.add_to_list(task);
            todo_list.print();
        },
        Command::Done(index) => {
            todo_list.mark_done(index);
            todo_list.print();
        }
    }

}
