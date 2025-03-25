use yew::prelude::*;

use crate::components::{civs::CIVS, gamedevhq::GamedevHQ, kidstrong::KidStrong};

#[function_component(Experience)]
pub fn experience() -> Html {
    html! {
    <>
    <KidStrong/>
    <CIVS/>
    <GamedevHQ/>
    </>
    }
}
