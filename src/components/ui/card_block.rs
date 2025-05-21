use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub title: Html,
    pub description: Html,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    html! {
        <>
        <div>
        <div class="bg-slate-400">
        {props.title.clone()}
        {props.description.clone()}
        </div>
        </div>
        </>
    }
}
