use yew::prelude::*;

// TODO:
// - Welcome text should change according to what is clicked, default is Welcome and about me text
// - Work on border stuff
#[function_component(AboutMe)]
pub fn aboutme() -> Html {
    html! {
        <>
        <div class="flex flex-col flex-shrink justify-start border p-2 max-w-[1350px]">
        <h1 class="text-3xl"> <strong> { "Welcome!" } </strong> </h1>
        <br/>
        <p>{ "Hello. I'm Brennen Witzens, a software engineer from Arizona." } </p>
        <p>{ "I'm a " } <strong> { "Full Stack-Engineer" } </strong> { ", always looking to improve my skills and learn new things." } </p>
        <p>{ "I'm mostly focused on backend development, as I've recently learned that I enjoy it more than designing UI." }</p>
        <p> { "I started with Unity and C# and have moved over to Rust and Go as my main languages, I've been wanting to learn Zig too! "} </p>
        <p> { "I've been interested in tooling, CLI and trying to better understand the lower level aspects of computers." } </p>
        <p> { "Outside of programming, I'm interested in reading manga, manhwa, manhua and light novels. I also enjoy listening to music and getting suggestions on new music." } </p>
        <p> { " Fun fact, this website is built with " } <a href="https://www.yew.rs" class="underline text-blue-300">
        {"Yew.rs"}</a> {" and Tailwind CSS" } </p>
        </div>
        </>
    }
}
