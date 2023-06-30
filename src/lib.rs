use borsh::{BorshDeserialize, BorshSerialize};
use pchain_sdk::{call, contract, contract_methods};

type Address = [u8; 32];

#[derive(BorshSerialize, BorshDeserialize)]
#[contract]
pub struct LotteryContract {
    ticket_price: u64,
    participants: Vec<Address>,
    total_prize: u64,
}

#[contract_methods]
impl LotteryContract {
    #[call]
    pub fn hello() {
        pchain_sdk::log("topic: Hello".as_bytes(), "Hello, Contract".as_bytes());
    }

    #[call]
    pub fn hello_from(name: String) -> u32 {
        pchain_sdk::log(
            "topic: Hello From".as_bytes(),
            format!("Hello, Contract. From: {}", name).as_bytes(),
        );
        name.len() as u32
    }

    #[call]
    fn new() -> Self {
        LotteryContract {
            ticket_price: u64::from_be(1), // Set the ticket price (1 XPLL)
            participants: Vec::new(),
            total_prize: u64::default(), // Set total_prize to 0
        }
    }

    #[call]
    pub fn execute(&mut self, caller: Address, sent_value: u64) -> Result<String, String> {
        pchain_sdk::log(
            "execute: STARTED".as_bytes(),
            format!(
                "The caller is : \nAdress: {}\nSent Value: {}",
                Self::address_to_string(caller),
                &sent_value
            )
            .as_bytes(),
        );

        // Check if the caller is trying to purchase a ticket
        if caller != Address::default() {
            // Ensure the caller has sent the correct ticket price
            if sent_value < self.ticket_price {
                return Err("Insufficient funds to purchase a ticket".to_string());
            }

            // Add the participant to the list
            self.participants.push(caller);

            // Increase the total prize
            self.total_prize += self.ticket_price;

            // Deduct the ticket price from the caller's balance
            Self::create_deposit(caller, sent_value - self.ticket_price, false);
        }

        pchain_sdk::log(
            "execute: ADDED".as_bytes(),
            format!("The participants count: {}", &self.participants.len()).as_bytes(),
        );

        // Check if the lottery should be conducted
        if self.should_conduct_lottery() {
            let winner = self.select_winner();

            // Distribute the prize to the winner
            pchain_sdk::transfer(winner, self.total_prize);

            // Reset the contract for the next lottery round
            self.participants = Vec::new();
            self.total_prize = u64::default();

            pchain_sdk::log(
                "execute: CONDUCTED".as_bytes(),
                format!("The winner: {}", Self::address_to_string(winner)).as_bytes(),
            );
        }

        return Ok("You entered the lottery!".to_string());
    }

    #[call]
    fn create_deposit(caller: Address, balance: u64, auto_stake_rewards: bool) {
        pchain_sdk::network::defer_create_deposit(caller, balance, auto_stake_rewards)
    }

    fn address_to_string(address: Address) -> String {
        let hex_chars: Vec<String> = address.iter().map(|byte| format!("{:02X}", byte)).collect();

        hex_chars.join("")
    }

    // Function to check if the lottery should be conducted
    #[call]
    fn should_conduct_lottery(&self) -> bool {
        self.participants.len() >= 2
    }

    // Function to select a winner randomly
    #[call]
    fn select_winner(&self) -> Address {
        let random_index = Self::select_random_number(0, self.participants.len() as u32);
        return self.participants[random_index];
    }

    fn select_random_number(start: u32, end: u32) -> usize {
        let timestamp = pchain_sdk::blockchain::timestamp();
        // Generate a random index based on the timestamp
        let range = end - start + 1;
        let random_index = (timestamp % range) + start;
        return random_index as usize;
    }
}
