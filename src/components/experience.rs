use yew::prelude::*;

use crate::components::ui::experience_blocks::ExperienceBlock;

#[function_component(Experience)]
pub fn experience() -> Html {
    html! {
        // Most recent down
    <ExperienceBlock
        company_name="KidStrong"
        date="present"
        job_name="software engineer"
        description="I did a thing"
    />
    }
}
