use yew::prelude::*;
// use gloo::console::log;
// use serde::{Serialize, Deserialize};
use stylist::{Style, style, yew::styled_component};

mod components;

use components::pages::occ_demo::OCCDemo;

#[styled_component(App)]
pub fn app() -> Html {
	html! {
		<OCCDemo/>
	}
}