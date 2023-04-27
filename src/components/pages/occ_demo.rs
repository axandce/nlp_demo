use yew::prelude::*;
use gloo::console::log;
// use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use gloo_net::http::Request;
use stylist::{Style, style, yew::styled_component};


use crate::components::molecules::blurb::{SingleDocumentClassificationBlurb, SummarizerBlurb};
use crate::components::organisms::text_submission_demo::{TextSubmissionDemo, TextSubmissionDemoDataPacket};

const STYLE_THEME: &str = include_str!("dark_theme.css");


#[styled_component(OCCDemo)]
pub fn occ_demo() -> Html {

	let stylesheet_from_file: Style = Style::new(STYLE_THEME).unwrap();


	let amazon_review_prefills : HashMap<String, String> = HashMap::from([
	    ("Book Review Sample".to_owned(), "Hi, I'm the Book Review Sample preview text!".to_owned()),
	    ("Kitchen Gadget Review Sample".to_owned(), "It was made of metal and silicone".to_owned()),
	    ("DvD Review Sample".to_owned(), "It wasn't as good as the book".to_owned()),
	    ("Electronic Gadget Review Sample".to_owned(), "I was hoping this dvd player would read well, but that's a different story.".to_owned()),
	]);

	let one_label_bert_demo_data_packet = TextSubmissionDemoDataPacket{
		url: "http://0.0.0.0:5010".to_string(),
		prefill_options: amazon_review_prefills,
	};


	let research_paper_prefills : HashMap<String, String> = HashMap::from([
	    ("Math and Science Sample Paper".to_owned(), "Hi, I'm the Math and Science Sample Paper preview text!".to_owned()),
	    ("Econ Research Sample Paper".to_owned(), "Hi, I'm the Econ Research Sample Paper preview text!".to_owned()),
	    ("Computer Science Sample Paper".to_owned(), "Hi, I'm the Computer Science Sample Paper preview text!".to_owned()),
	    ("Math and Statistics Sample Paper".to_owned(), "Hi, I'm the Math and Statistics Sample Paper preview text!".to_owned()),
	]);

	let multi_label_bert_demo_data_packet = TextSubmissionDemoDataPacket{
		url: "http://0.0.0.0:5030".to_string(),
		prefill_options: research_paper_prefills,
	};


	html! {
		<div class={stylesheet_from_file}>
			<div class={"demo-tile"}>
				<h1 class={"title-box"}>{ "Automating file systems with BERT" }</h1>
			</div>
			<div class={"demo-tile"}>
				<TextSubmissionDemo data_packet={one_label_bert_demo_data_packet}><SingleDocumentClassificationBlurb /></TextSubmissionDemo>
			</div>
			<div class={"demo-tile"}>
				<TextSubmissionDemo data_packet={multi_label_bert_demo_data_packet}><SummarizerBlurb /></TextSubmissionDemo>
			</div>
			// <TextSubmissionDemo data_packet={bert_packet}/>
		</div>
	}
}