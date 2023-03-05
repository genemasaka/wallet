# wallet

Rust code that utilizes the Yew web framework to create a simple wallet application. The application has a sender and a recipient, represented by instances of the User struct, which holds their respective names, unique identifiers, and balances.

The balance states of the sender and recipient are initialized using the use_state hook provided by Yew. The sender's current balance is displayed in a paragraph element, and the recipient's current balance is also displayed in a separate paragraph element.

The application also has a button labeled "Send", which, when clicked, subtracts a constant value of 33 from the sender's balance and adds it to the recipient's balance. The transaction only goes through if the sender's unique identifier matches a specific value (1331997), and an error message is printed otherwise.

## Requirements

Rust language installed

## How to run

Clone the repository using git clone https://github.com/<username>/<repository>.git or download the code as a zip file and extract it.
Navigate to the project directory using a command prompt or terminal.
Run the command cargo run.

Open a web browser and navigate to http://localhost:8000. The wallet application will be displayed.

## Dependencies

yew v0.17.1

## Contributing

If you would like to contribute to this code, please create a pull request.
