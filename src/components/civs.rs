use yew::prelude::*;

use crate::components::ui::experience_blocks::ExperienceBlock;

#[function_component(CIVS)]
pub fn civs() -> Html {
    html! {
    <>
    <ExperienceBlock
            company_name="Competition Interactive-Virtual Sports"
            date="Nov 2020 - Mar 2022"
            job_title="Unity Developer"
            description= {
                    html!{
                        <>
                        <ul class="list-inside list-disc container">
                        <li> {"Wrote performant code and updated existing projects either alongside other developers or independently" } </li>
                        <li> {"Worked with Git repositories for multiple projects"} </li>
                        <li> {"Worked in Unity to develop iOS, Android, WebGL based applications"} </li>
                        </ul>
                        </>
                    }
            }
        />
    </>
        }
}
