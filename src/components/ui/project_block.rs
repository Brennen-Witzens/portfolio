use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub project_name: Html,
    pub repo_link: Html,
    pub description: Html,
    // repo link?
}

#[function_component(ProjectBlock)]
pub fn project_block(props: &Props) -> Html {
    html! {
        <div class="py-2">
        // TODO: Figure out color scheme to use
        <div class="bg-slate-400 flex p-4 rounded-lg">
            <div class="flex flex-col flex-grow">
                <div class="flex flex-row justify-between">
                    <h2 class="text-2xl"> <strong> {props.project_name.clone()} </strong> </h2>
                </div>
                //Repo Link
                // TODO: make a 'link' component
                {props.repo_link.clone()}
                //Description
                {props.description.clone()}
            </div>
        </div>
        </div>
    }
}
