use crate::{AppContext, components::ui::card_block::Card};
use yew::prelude::*;

fn handle_next_block(counter: UseStateHandle<i32>, blocks: &Vec<Html>) -> Callback<MouseEvent> {
    let counter = counter.clone();
    if (*counter as usize) < blocks.len() - 1 {
        Callback::from(move |_: MouseEvent| counter.set(*counter + 1))
    } else {
        Callback::from(move |_: MouseEvent| counter.set(0))
    }
}

fn handle_previous_block(counter: UseStateHandle<i32>, blocks: &Vec<Html>) -> Callback<MouseEvent> {
    let counter = counter.clone();
    let length: i32 = blocks.len() as i32;
    if (*counter as usize) == 0 {
        Callback::from(move |_: MouseEvent| counter.set(length - 1))
    } else {
        Callback::from(move |_: MouseEvent| counter.set(*counter - 1))
    }
}

fn handle_card_description(app_context: &AppContext) -> Html {
    let html_block = match app_context.menu_state.current_state.as_str() {
        "AboutMe" => {
            html! {
            <>
            <div class="flex flex-col flex-shrink justify-start p-2 max-w-[1350px]">
                //<h1 class="text-3xl"> <strong> { "Welcome!" } </strong> </h1>
                //<br/>
                <p>{ "Hello. I'm Brennen Witzens, a software engineer from Arizona." } </p>
                <p>{ "I'm a " } <strong> { "Full Stack-Engineer" } </strong> { ", always looking to improve my skills and learn new things." } </p>
                <p>{ "I'm mostly focused on backend development, as I've recently learned that I enjoy it more than designing UI." }</p>
                <p> { "I started with Unity and C# and have moved over to Rust and Go as my main languages, I've been wanting to learn Zig too! "} </p>
                <p> { "I've been interested in tooling, CLI and trying to better understand the lower level aspects of computers." } </p>
                <p> { "Outside of programming, I'm interested in reading manga, manhwa, manhua and light novels. I also enjoy listening to music and getting suggestions on new music." } </p>
                <p> { " Fun fact, this website is built with " } <a href="https://www.yew.rs" class="underline text-blue-300">
                    {"Yew.rs"}</a> {" and Tailwind CSS" } </p>
            </div>
            </>
            }
        }
        "Projects" => {
            html! {
                <>
                <div class="flex flex-shrink justify-start p-2 max-w-[1350px]">
                <div class="flex flex-shrink justify-center min-w-[75px]">
                <button onclick={handle_previous_block(app_context.counter.clone(), &app_context.project_blocks)}>
                    <img src="./../assets/icons/arrow-square-left-svgrepo-com.svg" alt="Previous" style="width:50px; height: 50px"/>
                </button>
                </div>
                    {app_context.project_blocks[*app_context.counter as usize].clone()}
                <div class="flex flex-shrink justify-center min-w-[75px]">
                <button onclick={handle_next_block(app_context.counter.clone(), &app_context.project_blocks)}>
                    <img src="./../assets/icons/arrow-square-right-svgrepo-com.svg" alt="Next" style="width:50px; height: 50px"/>
                </button>
                </div>
                </div>
                // TODO:
                // - At some point try to add indicators
                // Indicators go here
                </>
            }
        }
        "Experience" => {
            html! {
                <>
                <div class="flex flex-shrink justify-start p-2 max-w-[1350px]">
                <div class="flex flex-shrink justify-center min-w-[75px]">
                <button onclick={handle_previous_block(app_context.counter.clone(), &app_context.experience_blocks)}>
                    <img src="./../assets/icons/arrow-square-left-svgrepo-com.svg" alt="Previous" style="width:50px; height: 50px"/>
                </button>
                </div>
                    {app_context.experience_blocks[*app_context.counter as usize].clone()}
                <div class="flex flex-shrink justify-center min-w-[75px]">
                <button onclick={handle_next_block(app_context.counter.clone(), &app_context.experience_blocks)}>
                    <img src="./../assets/icons/arrow-square-right-svgrepo-com.svg" alt="Next" style="width:50px; height: 50px"/>
                </button>
                </div>
                </div>
                // TODO:
                // - At some point try to add indicators
                // Indicators go here
                </>
            }
        }
        // TODO: really shouldn't have this
        _ => todo!(),
    };

    html_block
}

fn handle_title(context: &AppContext) -> Html {
    let title = match context.menu_state.current_state.as_str() {
        "AboutMe" => "Welcome!",
        "Projects" => "Projects",
        "Experience" => "Experience",
        _ => todo!(),
    };

    html! {<><h1 class="text-3xl"> <strong> { title } </strong> </h1></>}
}

#[function_component(AboutMe)]
pub fn aboutme() -> Html {
    let context: AppContext = use_context::<AppContext>().expect("Failed to get app context");

    html! {
        <>
        <Card
            title={handle_title(&context)}
            description={handle_card_description(&context)}
        />
        </>
    }
}
