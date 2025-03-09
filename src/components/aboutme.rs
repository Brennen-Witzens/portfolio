use yew::prelude::*;

#[function_component(AboutMe)]
pub fn aboutme() -> Html {
    html! {
        <>
        <p>{ "Hey. I'm Brennen Witzens." } </p>
        <p>{ "I'm a " } <strong> { "software engineer " } </strong> { "from Arizona. Focusing on backend and tool development" } </p>
        <p>{ "I've also used Unity before for both applications and games" } </p>
        // link here for yew
        <p> { "I enjoy understanding the 'behind the scenes' of programming, so always excited to learn new things. Such as I built this website using Yew!" } </p>
        </>
    }
}
