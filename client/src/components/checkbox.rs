use leptos::*;

#[component]
pub fn Checkbox(
    cx: Scope,
    name: &'static str,
    label: &'static str,
    value: ReadSignal<bool>,
    set_value: WriteSignal<bool>,
) -> impl IntoView {
    let on_change = move |event| {
        set_value(event_target_checked(&event));
    };
    return view! {
        cx,
        <div class="flex items-center mb-6">
            <input
                class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                id=String::from(name)
                type="checkbox"
                prop:checked={value}
                on:change=on_change
            />
            <label for=String::from(name) class="ml-2 text-sm font-medium text-gray-900 dark:text-gray-300">{label}</label>
        </div>
    };
}
