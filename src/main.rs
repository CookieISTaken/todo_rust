use std::fs::OpenOptions;
use std::io::Write;

#[derive(Debug)]
struct Task {
    name: String,
    id: usize,
    // done or not done
    status: bool,
}

impl Task {
    fn new(name: String, id: usize, status: bool) -> Task {
        Task { name, id, status }
    }
    fn change(&mut self, nstatus: bool) {
        self.status = nstatus;
    }
}

struct Todo {
    tasklist: Vec<Task>,
    taskid: usize,
}

impl Todo {
    fn new() -> Self {
        let todo = Todo {
            tasklist: Vec::new(),
            taskid: 1,
        };
        Self::load();
        todo
    }

    fn load() {
        let mut file = match OpenOptions::new()
            .append(true)
            .create(true)
            .open("./todo.txt")
        {
            Ok(file) => file,
            Err(error) => {
                println!("Error opening file: {}", error);
                return;
            }
        };

        match writeln!(file, "Test writing 101") {
            Ok(_) => {}
            Err(error) => println!("Error writing file: {}", error),
        }
    }

    fn add(&mut self, task: String) {
        let nid = self.taskid;
        self.taskid += 1;
        let temp = Task::new(task, nid, false);
        self.tasklist.push(temp);
    }

    fn delete(&mut self, id: usize) {
        match self.tasklist.iter().position(|t| t.id == id) {
            Some(pos) => {
                self.tasklist.swap_remove(pos);
            }
            None => {
                println!("Task not found");
            }
        }
    }

    fn update_status(&mut self, id: usize, done: bool) {
        match self.tasklist.iter_mut().find(|t| t.id == id) {
            Some(task) => {
                task.change(done);
            }
            None => {
                println!("Task not found");
            }
        }
    }

    fn list(&self) {
        for t in self.tasklist.iter() {
            println!("{:?}", t);
        }
    }
}

fn main() {
    let _todo = Todo::new();
}
