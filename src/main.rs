use yew::prelude::*;
use yew::{classes};

#[derive(Debug)]
struct User {
	name: String,
	uid: u64,
	balance: u32,
}


#[function_component]
fn App() -> Html {
	let sender = User {
		name: "Gene Masaka".to_string(),
		uid: 1331997,
		balance: 500,
	};
	
	let recepient = User {
		name: String::from("Vitalik"),
		uid: 1998,
		balance: 0,
	};

	let sender_balance = use_state(|| sender.balance);
	let onclick = {
		let amount: u32 = 33;
		let balance = sender_balance.clone();

		match sender.uid {
		1331997 => {
			let value = *balance - amount;
      		sender_balance.set(value);
			},
			_=> println!("You dont have the correct credentials to transfer tokens"),
		}
		
	
	};


	html! {
		<>
		<div class={classes!("main")}>
		<div class={classes!("card")}>
		<div class={classes!("card-header")}>
			<h3>{"Wallet"}</h3>
		</div>
		<div class={classes!("card-body")}>
		<h5>{"New Balance:"}</h5>
		<p>{ *sender_balance }</p>
		<button class={classes!("btn")} {onclick}>{"Send"}</button>

		</div>
		</div>
		</div>
		</>
	}

}

fn main() {
	yew::Renderer::<App>::new().render();
}