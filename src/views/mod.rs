pub mod views {
    use crate::todos::Todos;
    use askama::Template;

    #[derive(Template)]
    #[template(path = "index.html")]
    pub struct IndexTemplate {}

    #[derive(Template)]
    #[template(path = "pageNotFound.html")]
    pub struct PageNotFoundTemplate {}

    #[derive(Template)]
    #[template(path = "addToDo.html")]
    pub struct AddToDosFormTemplate {}

    #[derive(Template)]
    #[template(path = "toDoList.html")]
    pub struct TodoListTemplate<'a> {
        pub todos: &'a Vec<Todos>,
    }
}
