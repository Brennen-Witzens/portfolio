use utils::{MenuState, use_menu_context};
use yew::prelude::*;

mod components;
mod utils;
use components::{
    aboutme::AboutMe, civs::CIVS, color_theme::ColorTheme, discord_bot::DiscordBot, footer::Footer,
    gamedevhq::GamedevHQ, kidstrong::KidStrong, ui::menu::Menu,
};

// TODO:
// - Different "welcome" page
// - Have the "welcome" blocks change as the projects and other items are clicked
// - Better introduction
// 1. First on the list, get different colors setup. Get a proper "theming" going
// 2. Welcome block and layout change
//      2a. Layout change -> "Welcome" text under, and then buttons or links to projects, skills,
//      etc.

#[derive(Clone, Debug, PartialEq)]
pub struct AppContext {
    pub menu_state: UseReducerHandle<MenuState>,
    pub experience_blocks: Vec<Html>,
    pub project_blocks: Vec<Html>,
    pub counter: UseStateHandle<i32>,
}

#[function_component(App)]
fn app() -> Html {
    let menu_state: UseReducerHandle<MenuState> = use_menu_context();
    let experience_blocks = vec![html!(<KidStrong/>), html!(<CIVS/>), html!(<GamedevHQ/>)];
    let project_blocks = vec![html!(<ColorTheme/>), html!(<DiscordBot/>)];
    let counter = use_state(|| 0);
    html! {
    // TODO: Change/Make the default About me... programatically
    <ContextProvider<AppContext> context={AppContext {
        menu_state,
        experience_blocks,
        project_blocks,
        counter
        }}>

    <body class="bg-gray-700 flex flex-col h-screen text-black">
    <div class="flex justify-center items-center flex-grow">
    <AboutMe/>
    <Menu />
    </div>
    <div>
    <Footer />
    </div>
    </body>
    </ContextProvider<AppContext>>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
