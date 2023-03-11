use crate::entities::todo::*;
use crate::pages::todo_form::*;
use crate::pages::todo_list::*;
use crate::services::todo::CreateTodoDTO;
use crate::services::todo::TodoService;

use leptos::*;

#[component]
pub fn TodoPage(cx: Scope) -> impl IntoView {
    let (todos, set_todos) = create_signal::<Vec<Todo>>(cx, Vec::new() as Vec<Todo>);

    let get_todos = create_action(cx, move |_| async move {
        if let Ok(got_todo) = TodoService::get_todos().await {
            set_todos(got_todo);
        }
    });

    let create_todo = create_action(cx, move |todo: &CreateTodoDTO| {
        let todo = todo.clone();
        return async move {
            if let Ok(_) = TodoService::create_todo(&todo).await {
                get_todos.dispatch(());
            }
        };
    });

    let delete_todo = create_action(cx, move |id: &String| {
        let id = id.clone();
        return async move {
            if let Ok(_) = TodoService::delete_todo(&id).await {
                get_todos.dispatch(());
            }
        };
    });

    create_effect(cx, move |_| {
        get_todos.dispatch(());
    });

    return view! {
        cx,
        <div class="flex flex-col w-full h-screen px-32 py-10 bg-gray-700">
            <h1 class="mb-4 text-5xl text-white text-center">"Todo list app"</h1>
            <div class="flex-1 flex align-top justify-center w-full">
                <TodoForm create_todo={create_todo} />
                <TodoList todos={todos} delete_todo={delete_todo} />
            </div>
        </div>
    };
}
