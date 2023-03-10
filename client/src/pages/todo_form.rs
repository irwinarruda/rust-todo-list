use leptos::*;

use crate::{
    components::{
        checkbox::{Checkbox, CheckboxProps},
        text_field::{TextField, TextFieldProps},
    },
    entities::todo::Todo,
    services::todo::{CreateTodoDTO, EditTodoDTO},
    utils::parse_date::parse_date,
};

#[component]
pub fn TodoForm(
    cx: Scope,
    create_todo: Action<CreateTodoDTO, ()>,
    edit_todo: Action<EditTodoDTO, ()>,
    selected_todo: Memo<Option<Todo>>,
    reset_selected_todo: Action<(), ()>,
) -> impl IntoView {
    let (title, set_title) = create_signal(cx, "".to_string());
    let (description, set_description) = create_signal(cx, "".to_string());
    let (deadline, set_deadline) = create_signal(cx, "".to_string());
    let (is_completed, set_is_completed) = create_signal(cx, false);

    let is_editing = create_memo(cx, move |_| match selected_todo() {
        Some(_) => true,
        None => false,
    });

    let on_submit = move |event: ev::SubmitEvent| {
        event.prevent_default();
        if is_editing() {
            let todo_item = EditTodoDTO {
                id: match selected_todo() {
                    Some(todo) => todo.id,
                    None => "".to_string(),
                },
                description: description(),
                is_completed: is_completed(),
            };
            edit_todo.dispatch(todo_item);
        } else {
            let todo_item = CreateTodoDTO {
                title: title(),
                description: description(),
                deadline: parse_date(&deadline()),
                is_completed: is_completed(),
            };
            create_todo.dispatch(todo_item);
        }
    };

    let on_reset = move |_| {
        reset_selected_todo.dispatch(());
    };

    create_effect(cx, move |_| {
        match selected_todo() {
            Some(todo) => {
                set_title(todo.title);
                set_description(todo.description);
                set_deadline(match todo.deadline {
                    Some(d) => d.format("%Y-%m-%dT%H:%M").to_string(),
                    None => "".to_string(),
                });
                set_is_completed(todo.is_completed);
            }
            None => {
                set_title("".to_string());
                set_description("".to_string());
                set_deadline("".to_string());
                set_is_completed(false);
            }
        };
    });

    return view! {
        cx,
        <div class="flex-1 border-2 mr-10 px-8 pt-6 pb-8 bg-white border-gray-300 rounded-md">
            <div class="flex items-center justify-center">
                <h2 class="mb-2 text-3xl text-center font-semibold">{move || if is_editing() { "Edit Todo" } else { "Create Todo" }}</h2>
                {move || if is_editing() { view!{cx, <><button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline" type="button" on:click={on_reset}>"Create Todo"</button></>} } else { view!{cx, <></>} }}
            </div>
            <form class="h-full" on:submit=on_submit>
                <TextField
                    label="Title"
                    name="title"
                    placeholder="Do something..."
                    value=title
                    set_value=set_title
                    is_disabled=is_editing()
                />
                <TextField
                    label="Description"
                    name="description"
                    variant="textarea"
                    placeholder="I should do something like..."
                    value=description
                    set_value=set_description
                />
                <TextField
                    label="Deadline"
                    name="deadline"
                    variant="datetime-local"
                    placeholder="I should do something like..."
                    value=deadline
                    set_value=set_deadline
                    is_disabled=is_editing()
                />
                <Checkbox
                    label="Is completed?"
                    name="is_completed"
                    value=is_completed
                    set_value=set_is_completed
                />
                <div class="flex items-center justify-between">
                    <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold w-full py-2 px-4 rounded focus:outline-none focus:shadow-outline" type="submit">
                        "Create Todo"
                    </button>
                </div>
            </form>
        </div>
    };
}
