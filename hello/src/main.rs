use gloo::console::log;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Deserialize, Serialize)]
struct Object {
    name: String,
    age: u32,
}

#[function_component(App)]
fn app() -> Html {
    let name: &str = "John";
    let age: u32 = 32;

    let obj = Object {
        name: String::from("hello"),
        age: 32,
    };

    log!("name", name);
    log!("age", age);

    log!("obj", serde_json::to_string(&obj).unwrap());

    html! {
        <>
            <div class="flex flex-col w-screen h-screen items-center bg-red-300">
                <div class="text-black">{ "Hello, world!" }</div>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
