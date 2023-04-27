use gloo_net::http::Request;
use serde::{Serialize, Deserialize};
use serde_json::json;

#[derive(Serialize,Deserialize)]
pub struct ApiLoginResponse {
	pub text: String,
}

// #[derive(Serialize,Deserialize)]
// struct ApiLoginResponseData {
// 	pub text: String,
// }

pub async fn api_post_test(url: &str, payload: &str) ->  ApiLoginResponse {
	let body = json!({
		"text": payload,
	});
	let response = Request::post(url)
		.header("content-type", "application/json")
		.body(body.to_string())
		.send()
		.await
		.unwrap()
		.json::<ApiLoginResponse>()
		.await
		.unwrap();
	response
}