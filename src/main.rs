// Importing the required traits and modules from yew
use yew::classes;
use yew::prelude::*;

// Deriving the Debug trait for the User struct
#[derive(Debug)]
struct User {
    // A field to store the name of the user as a string
    name: String,
    // A field to store the unique identifier of the user as an unsigned 64-bit integer
    uid: u64,
    // A field to store the balance of the user as an unsigned 32-bit integer
    balance: u32,
}

// A functional component for the application
#[function_component]
fn App() -> Html {
    // Creating a sender User instance
    let sender = User {
        name: "Gene Masaka".to_string(),
        uid: 1331997,
        balance: 500,
    };

    // Creating a recipient User instance
    let recepient = User {
        name: String::from("Vitalik"),
        uid: 1998,
        balance: 0,
    };

    // Initializing the sender balance state
    let sender_balance = use_state(|| sender.balance);

    // Rendering the HTML content
    html! {
        <>
        // A main container div with the "main" class
        <div class={classes!("main")}>
            // A card container div with the "card" class
            <div class={classes!("card")}>
                // A header container div with the "card-header" class
                <div class={classes!("card-header")}>
                    // A header element with the text "Wallet"
                    <h3>{"Wallet"}</h3>
                </div>
                // A body container div with the "card-body" class
                <div class={classes!("card-body")}>
                    // A header element with the text "New Balance:"
                    <h5>{"New Balance:"}</h5>
                    // A paragraph element displaying the sender's current balance
                    <p>{ *sender_balance }</p>
                    // A button with the "Send" text and the "btn" class
                    <button class={classes!("btn")} onclick = {move |_| {
                        // A constant representing the amount to be transferred
                        let amount: u32 = 33;
                        // Cloning the sender balance state
                        let balance = sender_balance.clone();
                        // Checking the sender's unique identifier
                        match sender.uid {
                            // If the sender's UID matches, subtract the amount from the balance and update the state
                            1331997 => {
                                let value = *balance - amount;
                                sender_balance.set(value);
                            },
                            // If the UID does not match, print an error message
                            _=> println!("You don't have the correct credentials to transfer tokens"),
                        }
                    }
                    }
                    >{"Send"}</button>

                </div>
            </div>
        </div>
        </>
    }
}

// The main function, where the app is rendered
fn main() {
    yew::Renderer::<App>::new().render();
}
