use std::io;
use super::todo_app::{App, TodoList, Todo, Restriction};
use crate::DisplayArgs;

#[inline]
pub fn run(app: &mut App) -> io::Result<()>{
    let app = CliApp::new(app);
    app.print()?;
    Ok(())
}

pub struct CliApp<'a> {
    todo_app: &'a App,
}

impl <'a>CliApp <'a>{
    #[inline]
    pub fn new(app: &'a mut App) -> Self {
        for message in app.args.append_todo.clone() {
            app.append(message);
        }
        for message in app.args.prepend_todo.clone() {
            app.prepend(message);
        }
        if let Some(path) = app.args.append_file.clone() {
            app.append_list_from_path(path)
        }
        app.do_commands_on_selected();
        let _ = app.write();
        CliApp {
            todo_app: app,
        }
    }

    #[inline]
    fn print_list(&self) {
        for display in self.todo_app.display_current() {
            println!("{}", display);
        }
    }

    #[inline]
    pub fn print(&self) -> io::Result<()>{
        if !self.todo_app.args.search_and_select.is_empty() {
            self.todo_app.print_selected();
            return Ok(())
        }
        if self.todo_app.args.stdout {
            self.todo_app.print()?;
            return Ok(())
        }
        if self.todo_app.is_tree() {
            let mut print_todo = PrintTodoTree::new(self.todo_app.args.minimal_tree);
            print_todo.print_list(&self.todo_app.todo_list, &self.todo_app.args.display_args, self.todo_app.restriction());
        } else {
            self.print_list()
        }
        Ok(())
    }
}

// TODO: Use traverse_tree instead of this struct for printing todo tree. 
#[derive(Clone)]
struct PrintTodoTree {
    last_stack: Vec<bool>,
    should_print_indention: bool,
    is_last: bool,
    depth: usize,
}

impl PrintTodoTree {
    #[inline]
    pub fn new(should_print_indention:bool) -> Self {
        PrintTodoTree {
            last_stack: vec![],
            is_last: false,
            depth: 0,
            should_print_indention,
        }
    }

    #[inline]
    pub fn tree_child(&self, what_to_push: bool) -> Self {
        let mut child = self.clone();
        child.depth+=1;
        child.last_stack.push(!self.is_last && what_to_push);

        child
    }

    #[inline]
    pub fn print_list(&mut self, todo_list: &TodoList, display_args: &DisplayArgs, restriction: Restriction) {
        let todos = todo_list.todos(restriction.clone());

        for (index, todo) in todos.iter().enumerate() {
            self.is_last = index == todos.len() - 1;
            if !self.last_stack.is_empty() {
                self.print_indention();
            }
            self.print_todo(todo, display_args);

            if let Some(todo_list) = todo.dependency.todo_list() {
                let popped = self.last_stack.last();
                let what_to_push = popped.is_some() && !self.last_stack.is_empty();
                let mut tree_child = self.tree_child(what_to_push);
                tree_child.print_list(todo_list, display_args, restriction.clone());
            }

            if let Some(note) = todo.dependency.note() {
                self.print_note(note)
            }
        }
    }

    #[inline]
    fn print_todo(&self, todo: &Todo, display_args: &DisplayArgs) {
        println!("{}", todo.display(&display_args));
    }

    #[inline]
    fn print_note(&mut self, note: &String) {
        let mut last_stack = self.last_stack.clone();
        last_stack.push(!self.is_last);

        for line in note.lines() {
            self.print_prenote(last_stack.clone());
            println!("{}", line);
        }
    }

    #[inline]
    fn print_prenote(&self, last_stack: Vec<bool>) {
        for x in last_stack {
            if x {
                print!("│   ")
            } else {
                print!("    ")
            }
        }
        print!("    ")
    }

    #[inline]
    fn print_indention(&self) {
        if self.should_print_indention {
            return
        }
        for x in self.last_stack.clone() {
            if x {
                print!("│   ")
            } else {
                print!("    ")
            }
        }
        if self.is_last {
            print!("└── ");
        } else {
            print!("├── ");
        }
    }
}
