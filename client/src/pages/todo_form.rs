use chrono::{DateTime, Utc};
use leptos::*;

use crate::{
    components::{
        checkbox::{Checkbox, CheckboxProps},
        text_field::{TextField, TextFieldProps},
    },
    services::todo::CreateTodoDTO,
};

#[component]
pub fn TodoForm(cx: Scope, create_todo: Action<CreateTodoDTO, ()>) -> impl IntoView {
    let (title, set_title) = create_signal(cx, "".to_string());
    let (description, set_description) = create_signal(cx, "".to_string());
    let (deadline, set_deadline) = create_signal(cx, "".to_string());
    let (is_completed, set_is_completed) = create_signal(cx, false);

    let on_submit = move |event: ev::SubmitEvent| {
        event.prevent_default();
        let todo_item: CreateTodoDTO;
        if let Ok(date_time_deadline) = format!("{}:00Z", deadline()).parse::<DateTime<Utc>>() {
            todo_item = CreateTodoDTO {
                title: title(),
                description: description(),
                deadline: Some(date_time_deadline),
                is_completed: is_completed(),
            }
        } else {
            todo_item = CreateTodoDTO {
                title: title(),
                description: description(),
                deadline: None,
                is_completed: is_completed(),
            }
        }
        create_todo.dispatch(todo_item);
    };

    return view! {
        cx,
        <div class="flex-1 border-2 mr-10 px-8 pt-6 pb-8 bg-white border-gray-300 rounded-md">
            <h2 class="mb-2 text-3xl text-center font-semibold">"Create/Edit Todo"</h2>
            <form class="h-full" on:submit=on_submit>
                <TextField
                    label="Title"
                    name="title"
                    placeholder="Do something..."
                    value=title
                    set_value=set_title
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
