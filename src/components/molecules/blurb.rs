use yew::prelude::*;
// use gloo::console::log;
use stylist::{Style, style, yew::styled_component};

#[styled_component(SingleDocumentClassificationBlurb)]
pub fn single_document_classification_blurb() -> Html {
	html! {
		<div class={"blurb"}>
			<h1>{"Document Classification and Tagging"}</h1>
			<h3>{"Unstructured Data"}</h3>
			<p>{"Large collections of unstructured text based data are everywhere.  For these collections to be useful for future use, a system must be in place to organize the data in a way that can be queried.  Such a system may be as straightforward as organizing files into a folder hierarchy, or it can be more dynamic with a tagging system."}</p>
			<p>{"Such collections include legal documents, medical records, testimonies, and reviews."}</p>
			<h3>{"Implementing systems to structure unstructured Data"}</h3>
			<p>{"If a collection of data does not have a system in place, our options of querying the document are limited by keyword searching, and metadata analysis. However, this is only really effective if each class has a heavy but distinctive jargon that the other classes don't share."}</p>
			<p>{"When our data shares common language, what distinguishes them apart is their context."}</p>
			<h2>{"Leveraging BERT to implement a system"}</h2>
			<p>{"As a toy example to illustrate the capabilities of these models, below is a BERT model that has been fine-tuned to classify the subject matter of Amazon reviews"}</p>
			<h6 class="solid"/>
		</div>
	}
}

#[styled_component(SummarizerBlurb)]
pub fn summarizer_blurb() -> Html {
	html! {
		<div class={"blurb"}>
			<h1>{"Summarization"}</h1>
			<h3>{"Using Summarization to Speed Data Collection"}</h3>
			<p>{"The BERT model above was trained on 22,000 data points.  Many times, access to large enough datasets fit for training are scarce.  Unfortunately, there isn't a way to avoid the data requirement involved with any machine learning strategy."}</p>

			<p>{"If the alternative involves a person to open, skim, and deliberate over the contents of each file, there's an immediate use case for summarization with transformers"}</p>

			<h2>{"Leveraging T5 for Summarization"}</h2>
			<p>{"This is an out-of-the-box open sourced summarizer pipeline from HuggingFace"}</p>
			<h6 class="solid"/>
		</div>
	}
}

