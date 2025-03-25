use yew::prelude::*;

use crate::components::ui::experience_blocks::ExperienceBlock;

#[function_component(GamedevHQ)]
pub fn gamedevhq() -> Html {
    fn description() -> Html {
        html! {
            <>
            <ul class="list-inside list-disc container">
                <li> {"A fast paced bootcamp that focused on Unity"}</li>
            </ul>
            </>
        }
    }

    html! {
        <>
        <ExperienceBlock
            company_name="GameDevHQ"
            date="Jun 2020 - Aug 2020"
            job_title="Independant Developer BootCamp Participant"
            description={description()}
        />
        </>
    }
}
