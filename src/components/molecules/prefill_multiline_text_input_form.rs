use yew::prelude::*;
use gloo::console::log;
use stylist::{Style, style, yew::styled_component};

use std::collections::HashMap;

use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

use crate::components::atoms::multiline_text_input::MultiLineTextInput;
use crate::components::molecules::button_bar::ButtonBar;

// const STYLE_THEME: &str = include_str!("theme.css");

#[derive(Properties, PartialEq)]
pub struct PrefillMultiLineTextInputFormProps {
	pub prefill_options: HashMap<String, String>,
	pub text_box_text: String,
	pub handle_onchange: Callback<String>,
	pub on_prefill_option_click: Callback<Option<String>>,
	pub on_submit: Callback<()>,
}

#[styled_component(PrefillMultiLineTextInputForm)]
pub fn prefill_multiline_text_input_form(props: &PrefillMultiLineTextInputFormProps) -> Html {
	

	let handle_onchange = props.handle_onchange.clone();
	let onchange = Callback::from(move |event: Event| {
		let value = event.target()
			.unwrap()
			.unchecked_into::<HtmlInputElement>()
			.value();
		handle_onchange.emit(value);
	});

	let on_submit = props.on_submit.clone();
	let button_clicked = Callback::from(move |_| {
		on_submit.emit(());
	});

	html! {
		<div class={"prefill_multiline_text_input_form"}>
			<div class={"text_space"}>
				<ButtonBar prefill_options={props.prefill_options.clone()} onclick={props.on_prefill_option_click.clone()} />
				<textarea class={"multiline_text_input"} onchange={onchange} value={props.text_box_text.clone()} rows=10/>
			</div>
			<div class={"submit_button_space"}>
				<button class={"submit_button"} onclick={button_clicked}>{"Go"}</button>
			</div>
		</div>
	}
}