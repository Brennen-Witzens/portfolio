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
            <div class="w-screen h-screen bg-slate-700 text-white">
            // TODO: Figure out good "width" to have set here for overall placement of things
            <div class="max-w[1200px] m-auto p-4">
            <AboutMe />
            <Experience />
    //        <h1>{ "This is an example" }</h1>
    //        <div>
    //            <h3>{ "Videos to watch" }</h3>
    //            <p>{ "John Doe: Building and breaking things" }</p>
    //            <p>{ "This is a paragraph tag!!" }</p>
    //        </div>
    //        <div>
    //            <h3>{ "Image here" }</h3>
    //            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
            <Footer />
            </div>
            </div>
        }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
