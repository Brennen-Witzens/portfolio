use yew::prelude::*;

use crate::{
    AppContext,
    utils::{MenuAction, MenuState},
};

fn change_title(
    name: &'static str,
    menu_state: UseReducerHandle<MenuState>,
) -> Callback<MouseEvent> {
    Callback::from(move |_| match name {
        "AboutMe" => menu_state.dispatch(MenuAction::AboutMe),
        "Projects" => menu_state.dispatch(MenuAction::Projects),
        "Experience" => menu_state.dispatch(MenuAction::Experience),
        _ => todo!(),
    })
}

#[function_component(Menu)]
pub fn menu() -> Html {
    let context = use_context::<AppContext>().expect("Failed to find context");

    html! {
        <>
        <div class="flex flex-col min-w-[350px] max-w-[350px] items-center">
        // Title
        <h1 class="text-4xl"> { "Menu" } </h1>
        // NOTE: change the 100px size
        <div class="flex flex-col items-center max-w-[150px]">
        // List of items
        <div>
        <button onclick={change_title("AboutMe", context.menu_state.clone())}> { "About Me" } </button>
        </div>
        <button onclick={change_title("Projects", context.menu_state.clone())}> { "Projects" } </button>
        <button onclick={change_title("Experience", context.menu_state.clone())}> { "Experience" } </button>
        </div>
        </div>
        </>
    }
}
