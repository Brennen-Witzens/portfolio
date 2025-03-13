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
    <>
        //TODO: work on getting experience block components setup for each of the jobs
    <ExperienceBlock
        company_name="KidStrong"
        date="Mar 2022 - Feb 2025"
        job_title="Software Engineer"
        description={handle_description()}
    />
        //TODO: Clean this up
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
    <ExperienceBlock
        company_name="GameDevHQ"
        date="Jun 2020 - Aug 2020"
        job_title="Independant Developer BootCamp Participant"
        description="A fast paced bootcamp focused on Unity"
    />
    <ExperienceBlock
        company_name="Outback Steakhouse"
        date="Apr 2014 - Oct 2020"
        job_title="Host/Dishwasher/Line cook"
        description="Utilized time-management skills and attention to detail to provide a positive customer experience while maintaining a positive attitude in a fast-paced environment"
    />
    </>
    }
}
