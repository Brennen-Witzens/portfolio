use yew::prelude::*;

#[function_component(AboutMe)]
pub fn aboutme() -> Html {
    html! {
        <>
        <p>{ "Hey. I'm Brennen Witzens." } </p>
        <p>{ "I'm a " } <strong> { "software engineer " } </strong> { "from Arizona, focusing on backend and tool development." } </p>
        <p>{ "I've also been a Unity developer, using it for both games and applications." } </p>
        // link here for yew
        <p> { "I really enjoy learning new things, trying to understand the 'behind the scenes' of programming. Such as this website was built using " } <a href="https://www.yew.rs" class="underline text-blue-300">
        {"Yew"}</a> {" and Tailwind CSS" } </p>
        </>
    }
}
