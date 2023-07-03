use borsh::{BorshDeserialize, BorshSerialize};
use pchain_sdk::{call, contract, contract_methods};

type Address = [u8; 32];

#[derive(BorshSerialize, BorshDeserialize)]
#[contract]
pub struct LotteryContract {
    ticket_price: u64,
    participants: Vec<Address>,
    total_prize: u64,
    targeted_participants: u32,
}

#[contract_methods]
impl LotteryContract {
    //Function to interact with the contract
    #[call]
    pub fn execute(&mut self) -> Result<String, String> {
        // Check if the contract has already been initialized
        if !self.is_initialized() {
            // Contract has not been initialized, call the `new` method
            self.new(1, 10);
        }
        // Contract has already been initialized, proceed with regular execution

        // Check if the caller is trying to purchase a ticket
        let caller_address = pchain_sdk::transaction::calling_account() as Address;
        let sent_amount = pchain_sdk::transaction::amount() / 100000000; // Convert to TXPLL

        pchain_sdk::log(
            "execute: STARTED".as_bytes(),
            format!(
                "The caller is : \nAdress: {}\nSent Value: {}\nTicket Price: {}\nTotal Participants: {}\nTargeted Participants: {}",
                Self::address_to_string(caller_address),
                &sent_amount,
                &self.ticket_price,
                &self.participants.len(),
                &self.targeted_participants
            )
            .as_bytes(),
        );

        // Ensure the caller has sent the correct ticket price
        if sent_amount < self.ticket_price {
            return Err("Insufficient funds to purchase a ticket".to_string());
        }

        // Add the participant to the list
        self.participants.push(caller_address);

        // Increase the total prize
        self.total_prize += self.ticket_price;

        pchain_sdk::log(
            "execute: ADDED".as_bytes(),
            format!("The participants count: {}", &self.participants.len()).as_bytes(),
        );

        // Check if the lottery should be conducted
        if self.should_conduct_lottery() {
            pchain_sdk::log(
                "execute: CONDUCTING".as_bytes(),
                format!("Selecting winner now...").as_bytes(),
            );

            let winner = self.select_winner();

            pchain_sdk::log(
                "execute: TRANSFERRING".as_bytes(),
                format!(
                    "Transferring the amount of {} to the winner...",
                    &self.total_prize
                )
                .as_bytes(),
            );

            // Distribute the prize to the winner
            pchain_sdk::transfer(winner, self.total_prize);

            // Reset the contract for the next lottery round
            self.participants = Vec::new();
            self.total_prize = u64::default();

            pchain_sdk::log(
                "execute: CONDUCTED".as_bytes(),
                format!("The winner is: {}", Self::address_to_string(winner)).as_bytes(),
            );
        }

        pchain_sdk::log(
            "execute: ENTERED".as_bytes(),
            format!("You entered the lottery!").as_bytes(),
        );

        return Ok("You entered the lottery!".to_string());
    }

    // Function to initialize the contract
    #[call]
    pub fn new(&mut self, ticket_price: u64, targeted_participants: u32) {
        self.ticket_price = ticket_price; // Set the ticket price (1 XPLL)
        self.participants = Vec::new();
        self.total_prize = 0; // Set total_prize to 0
        self.targeted_participants = targeted_participants; // total number of targeted participants
    }

    // Function to check if the contract has been initialized
    fn is_initialized(&self) -> bool {
        // Check if any initialization condition is met
        self.ticket_price > 0
    }

    // Function to convert an address to a string
    fn address_to_string(address: Address) -> String {
        let hex_chars: Vec<String> = address.iter().map(|byte| format!("{:02X}", byte)).collect();

        hex_chars.join("")
    }

    // Function to check if the lottery should be conducted
    #[call]
    fn should_conduct_lottery(&self) -> bool {
        self.participants.len() >= self.targeted_participants as usize
    }

    // Function to select a winner randomly
    #[call]
    fn select_winner(&self) -> Address {
        let random_index = self.select_random_number(0, self.participants.len() as u32 - 1);

        pchain_sdk::log(
            "execute: SHUFFLED".as_bytes(),
            format!("Selected inex is: {}", random_index).as_bytes(),
        );

        return self.participants[random_index];
    }

    // Function to select a random number
    fn select_random_number(&self, start: u32, end: u32) -> usize {
        let timestamp = self.get_timestamp();

        let range = end - start + 1;
        let shuffled_range = self.shuffle_range(start, end, timestamp);

        let random_index = (timestamp % range as u32) as usize;

        return shuffled_range[random_index] as usize;
    }

    fn get_timestamp(&self) -> u32 {
        let timestamp = pchain_sdk::blockchain::timestamp();

        return timestamp;
    }

    fn shuffle_range(&self, start: u32, end: u32, seed: u32) -> Vec<u32> {
        let mut range: Vec<u32> = (start..=end).collect();

        for i in (1..range.len()).rev() {
            let j = (seed % (i as u32 + 1)) as usize;
            range.swap(i, j);
        }

        return range;
    }
}
