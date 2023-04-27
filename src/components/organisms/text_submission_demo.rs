use yew::prelude::*;
use gloo::console::log;
use stylist::{Style, style, yew::styled_component};
use std::collections::HashMap;

use crate::components::organisms::api::api_post_test;
use crate::components::molecules::prefill_multiline_text_input_form::PrefillMultiLineTextInputForm;
// use crate::components::molecules::blurb::Blurb;

#[derive(PartialEq)]
pub struct TextSubmissionDemoDataPacket {
	pub url: String,
	pub prefill_options: HashMap<String, String>,

}

#[derive(Properties, PartialEq)]
pub struct TextSubmissionDemoProps {
	pub data_packet: TextSubmissionDemoDataPacket,
	pub children: Children,
}

#[derive(Clone)]
struct OutputData {
	active: bool,
	text: String,
}

#[styled_component(TextSubmissionDemo)]
pub fn text_submission_demo(props: &TextSubmissionDemoProps) -> Html {

	let input_text_state = use_state(|| "".to_owned());
	let output_state = use_state(|| OutputData{active: false, text: "".to_owned()});

	let cloned_input_text_state = input_text_state.clone();
	let text_changed = Callback::from(move |text| {
		cloned_input_text_state.set(text);
	});

	let cloned_output_state = output_state.clone();
	let cloned_input_text_state = input_text_state.clone();
	let url = props.data_packet.url.clone();
	let submit_button_clicked = Callback::from(move |_| {
		let url = url.clone();
		let cloned_output_state = cloned_output_state.clone();
		let cloned_input_text_state = cloned_input_text_state.clone();
		wasm_bindgen_futures::spawn_local(async move {
			
			let url = url.clone();
			let cloned_output_state = cloned_output_state.clone();
			let cloned_input_text_state = cloned_input_text_state.clone();
			let payload = &(*cloned_input_text_state).clone();
			let response = api_post_test(&url, payload).await;
			cloned_output_state.set({
				OutputData {
					active: true,
					text: response.text,
				}
			});

		});

	});

	let cloned_input_text_state = input_text_state.clone();
	let prefill_option_clicked = Callback::from(move |prefill_text : Option<String>| {
		match prefill_text {
			Some(s) => cloned_input_text_state.set((*s).to_string()),
			None => log!("Still in default"),
			};
	});
	
	html! {
		<div class={"text_submission_demo"}>
			{props.children.clone()}
			<PrefillMultiLineTextInputForm on_prefill_option_click={prefill_option_clicked} on_submit={submit_button_clicked} handle_onchange={text_changed} prefill_options={props.data_packet.prefill_options.clone()} text_box_text={(*input_text_state).clone()}/>
			{get_output((*output_state).clone())}
		</div>
	}
}

fn get_output(output_state: OutputData) -> Html {
	if output_state.active {
		html! {
			<div class={"output_space"}>
				<h6 class="solid"/>
				<div class={"output_tile"}>
					<p>{output_state.text}</p>
				</div>
			</div>
		}
	} else {
		html! {}
	}
}

fn call_api(text: &str) -> String {
	format!("Calling API ON: <{}>", text)
}