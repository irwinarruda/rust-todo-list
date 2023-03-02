use leptos::*;
use leptos_meta::*;

#[component]
pub fn TextField(
    cx: Scope,
    name: &'static str,
    label: &'static str,
    placeholder: &'static str,
    variant: &'static str,
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
) -> impl IntoView {
    provide_meta_context(cx);
    let on_click_handler = move |event| {
        set_value(event_target_value(&event));
    };

    return view! {
        cx,
        <div class="mb-4">
            <label class="block text-gray-700 text-sm font-bold mb-2" for=String::from(name)>
                {String::from(label)}
            </label>
            {match variant {
                "textarea" => view!{cx,<><textarea
                    class="shadow appearance-none border rounded w-full h-32 py-2 px-3 resize-none text-gray-700 leading-tight focus:outline-none focus:shadow-cyan-500"
                    id=String::from(name)
                    name=String::from(name)
                    placeholder=String::from(placeholder)
                    value=value
                    on:input=on_click_handler
                    type="textarea"
                /></>},
                val => view!{cx,<><input
                    class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-cyan-500"
                    id=String::from(name)
                    name=String::from(name)
                    placeholder=String::from(placeholder)
                    value=value
                    on:input=on_click_handler
                    type=val
                /></>}
            }}
        </div>
    };
}
