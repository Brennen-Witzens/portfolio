use yew::prelude::*;

use crate::components::ui::experience_blocks::ExperienceBlock;

#[function_component(Experience)]
pub fn experience() -> Html {
    // TODO: Make different experience component blocks
    fn handle_description() -> Html {
        html! {
            <>
            <ul class="list-inside list-disc container">
                <li> {"Starting out as a Unity developer with a partial customer support role. I worked on two of the app that's allowed the KidStrong classes to run - \"KidStrong Coach\" and \"KidStrong TV\""} </li>
                <li> {"Initially we didn't have a dedicated backend team, so as backend needs were asked the developer tasked would do a \"full-stack\" style development for the feature. This was my first introduction into the backend side of development. After several tasks, I transitioned over to being a full time backend developer."} </li>
                <li> {"Once transitioned over to backend development, I learned more with Typescript. Also learning Rust and Golang. As we transitioned from Typescript to Rust and Golang, we also moved away from the \"traditional\" REST API styling, we used GraphQL and Protobufs, and also looked into OpenAPI specs." } </li>
            </ul>
            </>
        }
    }

    html! {
        // Most recent down
    <ExperienceBlock
        company_name="KidStrong"
        date="Mar 2022 - Feb 2025"
        job_title="Software Engineer"
        description={handle_description()}
    />
    }
}
