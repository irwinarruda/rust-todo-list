use crate::pages::todo_form::*;
use crate::pages::todo_list::*;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn Todo(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    return view! {
        cx,
        <div class="flex flex-col w-full h-screen px-32 py-10 bg-gray-700">
            <h1 class="mb-4 text-5xl text-white text-center">"Todo list app"</h1>
            <div class="flex-1 flex align-top justify-center w-full">
                <TodoForm />
                <TodoList />
            </div>
        </div>
    };
}
