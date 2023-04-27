use yew::prelude::*;
// use gloo::console::log;
use stylist::{Style, style, yew::styled_component};


#[derive(Properties, PartialEq)]
pub struct MultiLineTextInputProps {
	pub name: String,
	pub value: String,
}

#[styled_component(MultiLineTextInput)]
pub fn multiline_text_input(props: &MultiLineTextInputProps) -> Html {
	html! {
		<textarea class={"multiline_text_input"} name={props.name.clone()} value={props.value.clone()} rows=10/>
	}
}