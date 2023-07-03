# Lottery Smart Contract

The Lottery Smart Contract is a decentralized application (DApp) built on the PChain blockchain. It allows participants to purchase lottery tickets and have a chance to win the total prize. The contract randomly selects a winner when the number of participants reaches the targeted number.

## Features

- Participants can enter the lottery by purchasing a ticket.
- The contract ensures that participants have sent the correct ticket price.
- The total prize increases with each ticket purchase.
- When the number of participants reaches the targeted number, the contract conducts the lottery and selects a random winner.
- The winner receives the total prize amount.
- The contract resets for the next lottery round.

## Getting Started

To deploy and interact with the Lottery Smart Contract, follow these steps:

1. Install the required dependencies and set up the development environment.
2. Deploy the smart contract on the PChain blockchain.
3. Interact with the deployed contract by calling the `execute` method.

## Usage

1. Initialize the contract by calling the `new` method:

   - Set the ticket price (in XPLL).
   - Set the targeted number of participants.

2. Participants can enter the lottery by calling the `execute` method:

   - Ensure the ticket price is sent along with the transaction.
   - The participant's address will be automatically recorded.
   - The total prize will increase by the ticket price.

3. Once the number of participants reaches the targeted number:
   - The contract will conduct the lottery and select a random winner.
   - The total prize amount will be transferred to the winner's address.
   - The contract will reset for the next lottery round.

## Development

To contribute to the Lottery Smart Contract development, follow these steps:

1. Clone the repository and navigate to the project directory.
2. Install the required dependencies.
3. Make changes and add new features.
4. Run tests to ensure the contract functions correctly.
5. Submit a pull request with your changes.

## Testing

The smart contract includes unit tests to verify its functionality. To run the tests:

1. Set up the development environment and install dependencies.
2. Run the test suite using the provided testing framework.
3. Check the test results to ensure all tests pass successfully.

## License

The Lottery Smart Contract is open-source and distributed under the [MIT License](LICENSE).

## Contact

For any questions or inquiries, please contact [your-email@example.com](mailto:your-email@example.com).
