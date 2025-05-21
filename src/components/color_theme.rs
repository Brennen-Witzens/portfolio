use crate::components::ui::project_block::ProjectBlock;
use yew::prelude::*;

#[function_component(ColorTheme)]
pub fn color_theme() -> Html {
    fn handle_description() -> Html {
        html! {
            <>
            <ul class="list-inside list-disc container">
                <li> {"This was a project related to Image and Color Quantization. Using different algorithms, I was trying to make a color theme based on an image passed in."} </li>
            </ul>
            </>
        }
    }

    html! {
    <>
    <ProjectBlock
        project_name="Color Theme Maker"
        repo_link={
            html!{
                <>
                <a href="https://github.com/Brennen-Witzens/theme_maker" class="underline text-blue-700 dark:text-blue-400" target="_blank"> { "GitHub Repo Link" } </a>
                </>
            }
        }
        description={handle_description()}
    />
    </>
    }
}
