use crate::entities::todo::*;
use crate::pages::todo_form::*;
use crate::pages::todo_list::*;

use anyhow::Result;
use leptos::*;

async fn get_todos() -> Result<Vec<Todo>> {
    let res = match reqwasm::http::Request::get("http://localhost:8080/todos")
        .send()
        .await
    {
        Ok(res) => res,
        Err(err) => panic!("{:?}", err),
    };
    let todos = match res.json::<Vec<Todo>>().await {
        Ok(res) => res,
        Err(err) => panic!("{:?}", err),
    };
    log!("{:?}", todos);
    return Ok(todos);
}

#[component]
pub fn TodoPage(cx: Scope) -> impl IntoView {
    let (todos, set_todos) = create_signal::<Vec<Todo>>(cx, vec![]);
    let get_todos_action = create_action(cx, move |_| async move {
        if let Ok(got_todo) = get_todos().await {
            set_todos(got_todo);
        }
    });
    create_effect(cx, move |_| {
        get_todos_action.dispatch(());
    });
    return view! {
        cx,
        <div class="flex flex-col w-full h-screen px-32 py-10 bg-gray-700">
            <h1 class="mb-4 text-5xl text-white text-center">"Todo list app"</h1>
            <div class="flex-1 flex align-top justify-center w-full">
                <TodoForm  />
                <TodoList todos={todos} />
            </div>
        </div>
    };
}
