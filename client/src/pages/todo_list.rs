use crate::entities::todo::Todo;
use leptos::*;

pub fn are_similar(str1: &String, str2: &String) -> bool {
    return str1
        .to_lowercase()
        .replace(" ", "")
        .contains(&str2.to_lowercase().replace(" ", ""));
}

#[component]
pub fn TodoList(cx: Scope, todos: ReadSignal<Vec<Todo>>) -> impl IntoView {
    let (text_search, set_text_search) = create_signal(cx, "".to_string());
    let filtered_todos: Memo<Vec<_>> = create_memo(cx, move |_| {
        return todos.with(move |todos| {
            return todos
                .iter()
                .filter(|todo| {
                    text_search() == ""
                        || are_similar(&todo.title, &text_search())
                        || are_similar(&todo.description, &text_search())
                })
                .cloned()
                .collect();
        });
    });
    let on_change = move |event| {
        set_text_search(event_target_value(&event));
    };
    return view! {
        cx,
        <div class="flex-1 border-2 h-full px-8 pt-6 pb-8 bg-white border-gray-300 rounded-md">
            <h2 class="mb-2 text-3xl text-center font-semibold">"List Todos"</h2>
            <div class="mb-2">
                <input prop:value={move || text_search()} on:input=on_change class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-cyan-500" id="search" type="text" placeholder="Search something..." />
            </div>
            <div class="flex align-top w-full px-4 py-2">
                <p class="flex-1 font-3xl font-semibold">"Title"</p>
                <p class="flex-1 font-3xl font-semibold">"Description"</p>
                <p class="flex-1 font-3xl font-semibold">"Deadline"</p>
                <p class="flex-1 font-3xl font-semibold">"Is Completed"</p>
            </div>
            <For
                each={filtered_todos}
                key={|todo| todo.id.clone()}
                view={move |cx, todo| { view!{ cx,
                    <div class="flex align-top w-full mt-2 p-4 shadow border">
                        <div class="flex-1">
                            <span>{todo.title}</span>
                        </div>
                        <div class="flex-1">
                            <span>{todo.description}</span>
                        </div>
                        <div class="flex-1">
                            <span>"Qualquer"</span>
                        </div>
                        <div class="flex-1">
                            <span>{todo.is_completed}</span>
                        </div>
                    </div>
                }}}
            />
        </div>
    };
}
