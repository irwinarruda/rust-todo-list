use crate::{
    entities::todo::Todo,
    utils::{are_similar::are_similar, format_bool::format_bool, format_date::format_date},
};
use leptos::*;

#[component]
pub fn TodoList(
    cx: Scope,
    todos: ReadSignal<Vec<Todo>>,
    delete_todo: Action<String, ()>,
) -> impl IntoView {
    let (text_search, set_text_search) = create_signal(cx, "".to_string());
    let filtered_todos: Memo<Vec<Todo>> = create_memo(cx, move |_| {
        return todos.with(move |todos| {
            return todos
                .iter()
                .filter(move |todo| {
                    text_search() == ""
                        || are_similar(&todo.title, &text_search())
                        || are_similar(&todo.description, &text_search())
                })
                .cloned()
                .collect::<Vec<Todo>>();
        });
    });
    let on_change = move |event| {
        set_text_search(event_target_value(&event));
    };
    let on_delete = move |id: &String| {
        let id = id.clone();
        delete_todo.dispatch(id);
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
                <p class="flex-1 font-3xl font-semibold">"Ações"</p>
            </div>
            <div>
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
                                <span>{match todo.deadline { Some(d) => format_date(&d), None => "---".to_string() }}</span>
                            </div>
                            <div class="flex-1">
                                <span>{format_bool(todo.is_completed)}</span>
                            </div>
                            <button class="flex items-center justify-center w-9 h-9 bg-red-500 text-white rounded-full hover:bg-red-700" on:click={move |_| on_delete(&todo.id)}>
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M14.74 9l-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 01-2.244 2.077H8.084a2.25 2.25 0 01-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 00-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 013.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 00-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 00-7.5 0" />
                                </svg>
                            </button>
                        </div>
                    }}}
                />
            </div>
        </div>
    };
}
