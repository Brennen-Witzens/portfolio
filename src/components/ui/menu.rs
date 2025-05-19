use yew::prelude::*;

// TODO:
// - Do I need properties?
// - Menu block has what?
// - What is in the list
//      1. "Landing"/About Me
//      2. Projects
//      3. Experience
//      4. Skills
//      5. ???
// - Each item in the list is to be a link + change to 'welcome' text?

// TODO:
// 1. Press button, change out the "text block" with the one pressed.
// Ex: Press Projects while 'About me' is active will "animate" the card
// and change it over to the projects information (card/component)
fn change_title(title: &str) -> Callback<MouseEvent> {
    Callback::from(move |_| {})
}

#[function_component(Menu)]
pub fn menu() -> Html {
    html! {
        <>
        <div class="flex flex-col min-w-[350px] max-w-[350px] items-center">
        // Title
        // TODO: make the title bigger
        <h1 class="text-4xl"> { "Menu" } </h1>
        // NOTE: change the 100px size
        <div class="flex flex-col items-center border border-gray-200 max-w-[150px]">
        // List of items
        <div>
        <p> { "About Me" } </p>
        </div>
        <button onclick={change_title("changed")}> { "Projects" } </button>
        <p> { "Experience" } </p>
        </div>
        </div>
        </>




    }
}
