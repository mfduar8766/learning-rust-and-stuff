mod todos {
    use serde_derive::{Deserialize, Serialize};
    use std::io::{Error, ErrorKind};
    use uuid::Uuid;

    #[derive(Debug, Serialize, Deserialize, Default, Clone)]
    pub struct Todos {
        pub id: Uuid,
        pub name: String,
        pub completed: bool,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct TodoList {
        todos: Vec<Todos>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct AddTodos {
        pub todo: String,
    }

    impl Todos {
        pub fn new(name: &str) -> Self {
            return Self {
                id: Uuid::new_v4(),
                name: name.to_string(),
                completed: false,
            };
        }
    }

    impl TodoList {
        pub fn new() -> Self {
            return Self {
                todos: vec![
                    Todos {
                        id: Uuid::new_v4(),
                        name: String::from("Learn Rust"),
                        completed: false,
                    },
                    Todos {
                        id: Uuid::new_v4(),
                        name: String::from("Learn HTMK"),
                        completed: false,
                    },
                    Todos {
                        id: Uuid::new_v4(),
                        name: String::from("Learn C++"),
                        completed: false,
                    },
                ],
            };
        }
        pub fn _get_todos(&self) -> &Vec<Todos> {
            return &self.todos;
        }
        pub fn get_todos_as_mut(&mut self) -> &mut Vec<Todos> {
            return &mut self.todos;
        }
        pub fn add_todo(&mut self, name: &str) -> Todos {
            let todo = Todos::new(name);
            self.todos.push(todo);
            return Todos::new(name);
        }
        pub fn delete_todo(&mut self, id: Uuid) {
            self.todos.retain(|todo| todo.id != id);
        }
        pub fn update_todo_status(&mut self, id: Uuid, status: bool) -> Result<&mut Todos, Error> {
            let index = self.get_todo_by_id(id);
            return match index {
                Some(i) => {
                    if i < usize::MIN {
                        return Err(Error::new(ErrorKind::NotFound, "todo not in list"));
                    } else if self.todos[i].id == id {
                        let update = &mut self.todos[i];
                        update.completed = status;
                        return Ok(update);
                    } else {
                        return Err(Error::new(ErrorKind::NotFound, "todo does not exist"));
                    }
                }
                None => Err(Error::new(ErrorKind::Other, "InternalServerError")),
            };
        }
        fn get_todo_by_id(&self, id: Uuid) -> std::option::Option<usize> {
            return self.todos.iter().position(|r| r.id == id);
        }
    }
}

pub use todos::{TodoList, Todos};
pub fn init_todods() -> todos::TodoList {
    return todos::TodoList::new();
}
