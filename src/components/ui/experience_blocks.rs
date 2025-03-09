use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub company_name: Html,
    pub date: Html,
    pub job_name: Html,
    pub description: Html,
}

#[function_component(ExperienceBlock)]
pub fn experience_block(props: &Props) -> Html {
    html! {
        <div class="py-8">
        <div class="bg-blue-200 border border-gray-200 flex flex-wrap gap-4 justify-center p-4 rounded-lg dark:bg-slate-800 dark:border-slate-700">
            <div class="flex flex-col flex-1 grow min-w-[300px] gap-6 self-start max-[850px]:min-w-full">
                <h3 class="text-2xl"> {props.company_name.clone()} </h3>
                <h4 class="text-xl"> {props.date.clone()} </h4>
                <h3> {props.job_name.clone()} </h3>
                <p> {props.description.clone()} </p>
            </div>
            </div>
        </div>
    }
}
