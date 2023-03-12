use crate::entities::todo::*;
use crate::pages::todo_form::*;
use crate::pages::todo_list::*;
use crate::services::todo::CreateTodoDTO;
use crate::services::todo::EditTodoDTO;
use crate::services::todo::TodoService;

use leptos::*;

#[component]
pub fn TodoPage(cx: Scope) -> impl IntoView {
    let (todos, set_todos) = create_signal(cx, Vec::new() as Vec<Todo>);
    let (id_todo, set_id_todo) = create_signal(cx, "".to_string());

    let selected_todo = create_memo(cx, move |_| {
        return todos.with(move |todos| {
            return todos.iter().find(move |todo| todo.id == id_todo()).cloned();
        });
    });

    let get_todos = create_action(cx, move |_| async move {
        if let Ok(got_todo) = TodoService::get_todos().await {
            set_todos(vec![]);
            set_todos(got_todo);
        }
    });

    let reset_selected_todo = create_action(cx, move |()| {
        return async move {
            set_id_todo("".to_string());
        };
    });

    let select_todo = create_action(cx, move |id: &String| {
        let id = id.clone();
        return async move {
            set_id_todo(id);
        };
    });

    let create_todo = create_action(cx, move |todo: &CreateTodoDTO| {
        let todo = todo.clone();
        return async move {
            if let Ok(_) = TodoService::create_todo(&todo).await {
                get_todos.dispatch(());
            }
        };
    });

    let edit_todo = create_action(cx, move |todo: &EditTodoDTO| {
        let todo = todo.clone();
        return async move {
            if let Ok(_) = TodoService::edit_todo(&todo).await {
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
                <TodoForm create_todo={create_todo} edit_todo={edit_todo} selected_todo={selected_todo} reset_selected_todo={reset_selected_todo} />
                <TodoList todos={todos} select_todo={select_todo} delete_todo={delete_todo} />
            </div>
        </div>
    };
}
