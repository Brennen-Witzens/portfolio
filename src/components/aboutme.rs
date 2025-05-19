use yew::prelude::*;

use crate::{
    AppContext,
    components::{experience::Experience, ui::card_block::Card},
};

// TODO:
// - Handle each description based on menu state
fn handle_card_description(app_context: &AppContext) -> Html {
    let html_block = match app_context.menu_state.current_state.as_str() {
        "AboutMe" => {
            html! {
            <>
            <div class="flex flex-col flex-shrink justify-start border p-2 max-w-[1350px]">
                <h1 class="text-3xl"> <strong> { "Welcome!" } </strong> </h1>
                <br/>
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
            // TODO:
            // - Need to get some projects set here
            // -> Need to make components for them
            // - Discord bot
            // - Theme Maker
            html! {}
        }
        "Experience" => {
            // TODO:
            // - Make this centered
            // - Should I make this a carousel?
            // - I think the overall block is "fine" for now though
            // - Maybe a vector to cycle them?
            html! {
                <Experience />
            }
        }
        _ => todo!(),
    };

    html_block
}

#[function_component(AboutMe)]
pub fn aboutme() -> Html {
    let context: AppContext = use_context::<AppContext>().expect("Failed to get app context");
    html! {
        <>
        <Card
            title=""
            description={handle_card_description(&context)}
        />
        </>
    }
}
