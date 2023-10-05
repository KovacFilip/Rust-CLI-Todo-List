use std::{
    fs::OpenOptions,
    io::{stdin, stdout, Write},
    path::Path,
    time::{Duration, SystemTime},
};

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    name: String,

    #[arg(short, long)]
    file: Option<String>,
}

enum TodoStatus {
    Todo(i32),
    InProgress(SystemTime),
    Done(Duration),
}

fn main() {
    let args = Args::parse();
    println!("Hello, {}!", args.name);
    let mut my_list: Vec<(TodoStatus, String)> = Vec::new();

    loop {
        print!("> ");
        let _ = stdout().flush();

        let mut buf = String::new();
        let _ = stdin().read_line(&mut buf);

        let buf_trimmed = buf.trim();

        match buf_trimmed.split_once(' ') {
            Some(("start", arg)) => {
                let index = get_index(arg, &my_list);

                if let Some(id) = index {
                   my_list[id].0 = TodoStatus::InProgress(SystemTime::now());
                }

            },
            Some(("finish", arg)) => {
                let index = get_index(arg, &my_list);

                if let Some(id) = index {
                    if let TodoStatus::InProgress(time) = my_list[id].0 {
                        my_list[id].0 = TodoStatus::Done(time.elapsed().unwrap_or(Duration::new(0, 0)))
                    }
                }
            },
            Some(("add", arg)) => {
                let arg_split: Vec<&str> = arg.rsplitn(2, ' ').collect();

                if let Some(priority) = arg_split.first() {
                    match priority.parse::<i32>() {
                        Ok(prio_number) => my_list.push((TodoStatus::Todo(prio_number), arg_split.last().unwrap_or(&"hello").to_string())),
                        Err(_err) => println!("The priority has to be a number"),
                    }
                } else {
                    println!("You need to set priority to new todos")
                }

            }
            Some(("remove", arg)) => match my_list.iter().position(|x| x.1 == arg) {
                Some(index) => {
                    my_list.remove(index);
                }
                None => println!(
                    "'{}' is not in your list. Type `list` to see all Todos.",
                    arg
                ),
            },
            None => match buf_trimmed {
                "exit" => break,
                "list" => {
                    println!("{}'s TODOs", args.name);
                    println!("------------------------");
                    print_todos(&my_list);
                }
                "help" => println!("\n`help` => print help\n`list` => list TODOs\n`add <item>` => add <item> <priority> to your list\n`remove <item>` => removes <item> from your list\n`exit` => exits the program\n`start <item>` => sets <item> as `in progress`\n`finish <item>` => sets <item> as `done`"),
                _ => (),
            },
            _ => (),
        }
    }

    if args.file.is_some() {
        save_todos_to_file(args.file.unwrap(), &my_list)
    }
}

fn status_string(my_todo: &TodoStatus) -> String {
    match my_todo {
        TodoStatus::Done(duration) => format!("[done] in {} seconds", duration.as_secs()),
        TodoStatus::InProgress(_) => "[in progress]".to_string(),
        TodoStatus::Todo(_) => "[todo]".to_string(),
    }
}

fn get_index(arg: &str, todos: &[(TodoStatus, String)]) -> Option<usize> {
    return todos.iter().position(|x| x.1 == arg);
}

fn save_todos_to_file(file_path_arg: String, todos: &Vec<(TodoStatus, String)>) {
    let path = Path::new(&file_path_arg);
    let f = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path);

    match f {
        Ok(mut file) => {
            let mut buffer = String::new();
            for todo in todos {
                buffer.push_str(&status_string(&todo.0));
                buffer.push(' ');
                buffer.push_str(&todo.1);
                buffer.push('\n');
            }

            let _ = file.write_all(buffer.as_bytes());
            let _ = file.sync_all();
        }
        Err(err) => println!("Error happened: {}", err),
    }
}

fn print_todos(my_list: &[(TodoStatus, String)]) {
    let mut todos: Vec<(&TodoStatus, &String)> = Vec::new();
    let mut in_progress: Vec<(&TodoStatus, &String)> = Vec::new();
    let mut dones: Vec<(&TodoStatus, &String)> = Vec::new();

    for (status, todo_string) in my_list {
        match status {
            TodoStatus::Todo(_) => todos.push((status, todo_string)),
            TodoStatus::InProgress(_) => in_progress.push((status, todo_string)),
            TodoStatus::Done(_time) => dones.push((status, todo_string)),
        }
    }

    todos.sort_by(|a, b| match (a, b) {
        ((TodoStatus::Todo(priority1), _), (TodoStatus::Todo(priority2), _)) => {
            priority2.cmp(priority1)
        }
        _ => std::cmp::Ordering::Equal,
    });

    for todo in todos {
        println!("{} {}", status_string(todo.0), todo.1);
    }

    for prog in in_progress {
        println!("{} {}", status_string(prog.0), prog.1);
    }

    for done in dones {
        println!("{} {}", status_string(done.0), done.1);
    }
}
