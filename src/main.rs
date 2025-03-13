use yew::prelude::*;

mod components;

use components::{aboutme::AboutMe, experience::Experience, footer::Footer};

// Struct for job text and blocks
// Struct for "About me"
// Struct for Experience/Projects
// Struct for Link blocks/images

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="w-full h-full bg-slate-700 text-white">
        // TODO: Figure out good "width" to have set here for overall placement of things
        <div class="max-w[1200px] m-auto p-4">
        <AboutMe />
        <Experience />
        <Footer />
        </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
