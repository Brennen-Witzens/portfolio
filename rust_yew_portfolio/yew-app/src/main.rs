use yew::prelude::*;

mod components;

use components::AboutMe;

// Struct for job text and blocks
// Struct for "About me"
// Struct for Experience/Projects
// Struct for Link blocks/images

#[function_component(App)]
fn app() -> Html {
    html! {
        <>

        <AboutMe />
        <h1>{ "This is an example" }</h1>
        <div>
            <h3>{ "Videos to watch" }</h3>
            <p>{ "John Doe: Building and breaking things" }</p>
            <p>{ "This is a paragraph tag!!" }</p>
        </div>
        <div>
            <h3>{ "Image here" }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
