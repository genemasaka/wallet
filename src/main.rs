use yew::prelude::*;
use yew::{classes};

struct User {
	name: String,
	uid: u64;
	balance: u32;
}

impl User {
	fn send(&self) {
		
	}
}

#[function_component]
fn App() -> Html {
	let sender = User {
		String::from("Gene Masaka"),
		1331997,
		500,
	}
	
	let recepient = User {
		String::from("Vitalik"),
		1998,
		0,
	}

}

fn main() {
	yew::Renderer::<App>::new().render();
}