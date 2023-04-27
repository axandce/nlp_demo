use yew::prelude::*;
use std::iter;
// use gloo::console::log;
use std::collections::HashMap;
use stylist::{Style, style, yew::styled_component};

#[derive(Properties, PartialEq)]
pub struct ButtonBarProps {
	pub prefill_options: HashMap<String, String>,
	pub onclick: Callback<Option<String>>,
}

#[styled_component(ButtonBar)]
pub fn button_bar(props: &ButtonBarProps) -> Html {

	html! {
		<div class={"button_bar"}>
			{count_to_buttons(&props.prefill_options.clone(), &props.onclick.clone())}
		</div>
	}
}

fn count_to_buttons(prefill_options: &HashMap<String, String>, on_click: &Callback<Option<String>>) -> Html {

	prefill_options.iter().map(|(key, value)|  {
		let key = key.clone();
		let value = value.clone();
		let on_click = on_click.clone();
		let button_clicked = Callback::from(move |_| {
			on_click.emit(Some(value.clone().to_string()));
		});
		html! {
			<button class={"button_bar_button"} onclick={button_clicked}>
				{key.clone()}
			</button>
		}
	}).collect()
}