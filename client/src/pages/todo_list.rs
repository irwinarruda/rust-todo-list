use leptos::*;
use leptos_meta::*;

#[component]
pub fn TodoList(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    return view! {
        cx,
        <div class="flex-1 border-2 h-full px-8 pt-6 pb-8 bg-white border-gray-300 rounded-md">
            <h2 class="mb-2 text-3xl text-center font-semibold">"List Todos"</h2>
            <div class="mb-2">
                <input class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-cyan-500" id="search" type="text" placeholder="Search something..." />
            </div>
            <div class="flex align-top w-full px-4 py-2">
                <p class="flex-1 font-3xl font-semibold">"Title"</p>
                <p class="flex-1 font-3xl font-semibold">"Description"</p>
                <p class="flex-1 font-3xl font-semibold">"Deadline"</p>
                <p class="flex-1 font-3xl font-semibold">"Is Completed"</p>
            </div>
            <div class="flex align-top w-full p-4 shadow border">
                <div class="flex-1">
                    <span>"Todo 1"</span>
                </div>
                <div class="flex-1">
                    <span>"Description of todo 1"</span>
                </div>
                <div class="flex-1">
                    <span>"01/03/2023 20:00"</span>
                </div>
                <div class="flex-1">
                    <span>"No"</span>
                </div>
            </div>
        </div>
    };
}
