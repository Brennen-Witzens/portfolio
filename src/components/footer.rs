use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
    <>
    <div class="grid gap-1">
    <hr />
    // TODO: Setup Links for these
    // Take a look at https://gitlab.com/marcempunkt/maeurerdev/-/blob/master/src/components/socials.rs?ref_type=heads for more details/example
    <div class="flex justify-end gap-2 align-end">
        <img class="w=12 h-12 shrink-0" src="./../assets/icons/email.svg" alt="Email" />
        <img class="w-12 h-12 shrink-0" src="./../assets/icons/GitHub.svg" alt="GitHub" />
        <img class="w-11 h-11 shrink-0" src="./../assets/icons/LinkedIn.svg" alt="LinkedIn" />
    </div>
    </div>
    </>
    }
}
