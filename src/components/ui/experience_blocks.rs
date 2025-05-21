use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub company_name: Html,
    pub date: Html,
    pub job_title: Html,
    pub description: Html,
    // TODO: Add vec of skills/technology associated with job here
    // IE - Unity, Rust, Go, Cloud Run, Terraform, Google Cloud, Typescript, etc.
}

#[function_component(ExperienceBlock)]
pub fn experience_block(props: &Props) -> Html {
    html! {
        <div class="py-2">
        // TODO: Figure out color scheme to use
        <div class="bg-slate-400 flex p-4 rounded-lg">
            <div class="flex flex-col flex-grow">
                <div class="flex flex-row justify-between">
                    <h2 class="text-2xl"> <strong> {props.company_name.clone()} </strong> </h2>
                    // Date div
                    <div class="h-8 flex rounded-xl bg-slate-600 p-2 shadow-lg">
                        <p class="self-center text-base"> {props.date.clone()} </p>
                    </div>
                </div>
                <h4 class="text-base"> {props.job_title.clone()} </h4>
                //Description
                {props.description.clone()}
            </div>
        </div>
        </div>
    }
}
