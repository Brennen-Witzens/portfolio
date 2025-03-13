use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
    <>
    <div class="py-2 grid gap-1">
    <hr />
    // TODO: Setup Links for these
    // Take a look at https://gitlab.com/marcempunkt/maeurerdev/-/blob/master/src/components/socials.rs?ref_type=heads for more details/example
    <div class="flex justify-end gap-2 align-end">
        <a href="mailto:bwitzens@gmail.com">
        <img class="w=12 h-12 shrink-0" src="./../assets/icons/email.svg" alt="Email" />
        </a>
        <a href="https://github.com/Brennen-Witzens/Spikepk" target="_blank">
        <img class="w-12 h-12 shrink-0" src="./../assets/icons/GitHub.svg" alt="GitHub" />
        </a>
        <a href="https://www.linkedin.com/in/brennen-w-42412a1ab/" target="_blank">
        <img class="w-11 h-11 shrink-0" src="./../assets/icons/LinkedIn.svg" alt="LinkedIn" />
        </a>
    </div>
    </div>
    </>
    }
}
