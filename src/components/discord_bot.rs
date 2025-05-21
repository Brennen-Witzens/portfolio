use crate::components::ui::project_block::ProjectBlock;
use yew::prelude::*;

#[function_component(DiscordBot)]
pub fn discord_bot() -> Html {
    fn handle_description() -> Html {
        html! {
            <>
            <ul class="list-inside list-disc container">
                <li> {"This was a project where I used the Serenity Rust create to make a discord bot. The main focus of this was to better understand the Discord API but also for fun."} </li>
                <li> {"The idea behind the project was to make a randomized character set idea for a game called Naraka Bladepoint."} </li>
            </ul>
            </>
        }
    }

    html! {
    <>
    <ProjectBlock
        project_name="Ultimate Bravery Discord Bot"
        repo_link={
            html!{
                <>
                <a href="https://github.com/Brennen-Witzens/narakaultimatebravery" class="underline text-blue-700 dark:text-blue-400" target="_blank"> { "GitHub Repo Link" } </a>
                </>
            }
        }
        description={handle_description()}
    />
    </>
    }
}
