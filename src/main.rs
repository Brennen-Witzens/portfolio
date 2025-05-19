use yew::prelude::*;

mod components;
use components::{aboutme::AboutMe, experience::Experience, footer::Footer, ui::menu::Menu};

// TODO:
// - Different "welcome" page
// - Have the "welcome" blocks change as the projects and other items are clicked
// - Better introduction
// 1. First on the list, get different colors setup. Get a proper "theming" going
// 2. Welcome block and layout change
//      2a. Layout change -> "Welcome" text under, and then buttons or links to projects, skills,
//      etc.

#[function_component(App)]
fn app() -> Html {
    html! {
    <body class="bg-slate-700 flex flex-col h-screen text-white">
    <div class="flex justify-center items-center flex-grow">
    <AboutMe/>
    <Menu />
    </div>
    <div>
    <Footer />
    </div>
    </body>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
