#![cfg_attr(not(feature = "std"), no_std)]
/// Two major features for the future: 1. Oracles, not currently in substrate natively. 2. Scheduled monthly payment for users

/*  
How this should work: We have a reserve map. Users can deposit into this reserve, verifies they send native currency to the contract, then stores equivalent 
value into reserve map. When users want, they can also withdraw from the contract, up to the amount in their reserves. When the time comes, automatically subtract
value owed from reserve, and at the EXACT same time, withdraw that equivalent amount from contract to contract-owner address (should transfer first, 
then subtract from reserve map upon confirmation of successful transaction). Basically, the map is a record
of transactions, a record of record of transactions heh. This way, we can do automatic payments
*/
#[ink::contract]
mod insurance {
    use ink::storage::Mapping;
    use ink_prelude::string::String;
    use chrono::prelude::*;

    /// Defines storage of the contract. In this case, we want to map a user's account ID with an integer value
    #[ink(storage)]
    pub struct Insurance {
        owner_account: AccountId,
        // premium and deductible can be decimal values
        premium: Mapping<AccountId, i32>,
        deductible: Mapping<AccountId, i32>,
        legal_name: Mapping<AccountId, String>,
        // payment schedule in terms of days
        payment_schedule: Mapping<AccountId, u32>,
        reserve: Mapping<AccountId, Balance>,

    }

    impl Insurance {
        
        // When instantiating a new contract, and only when instantiating, we can permanently set an owner address. Only this owner address can withdraw funds
        // payable allows native tokens to be sent to contract
        #[ink(constructor, payable)]
        pub fn new_insurance(premium_init: i32, deductible_init: i32, legal_name_init: String, payment_schedule_init: u32) -> Self {

            let is_origin = Self::env().caller_is_origin();

            // if !is_origin {
            //     return Err(Error::NotOrigin)
            // }

            let mut premium = Mapping::default();
            let mut deductible = Mapping::default();
            let mut legal_name = Mapping::default();
            let mut payment_schedule = Mapping::default();
            let mut reserve = Mapping::default();
            // Always returns contract caller. Different than origin
            // If a contract calls another contract, this returns first contract, not original caller
            let caller = Self::env().caller();
            // & in rust is known as references. Allows us to borrow values instead of dropping them as rust normally does when you put variable into function
            premium.insert(&caller, &premium_init);
            deductible.insert(&caller, &deductible_init);
            legal_name.insert(&caller, &legal_name_init);
            payment_schedule.insert(&caller, &payment_schedule_init);
            reserve.insert(&caller, &0);

            Self {
                owner_account: caller,
                premium,
                deductible,
                legal_name,
                payment_schedule,
                reserve
            }
        }

        // Default initialize contract storage
        #[ink(constructor)]
        pub fn default() -> Self {
            let caller = Self::env().caller();
            Self {
                owner_account: caller,
                premium: Mapping::default(),
                deductible: Mapping::default(),
                legal_name: Mapping::default(),
                payment_schedule: Mapping::default(),
                reserve: Mapping::default(),
            }
        }

        /// Credit more money to the contract.
        #[ink(message, payable)]
        pub fn deposit_to_contract(&mut self) {
            let caller = self.env().caller();
            let balance = self.reserve.get(caller).unwrap_or(0);

            // retrieves paid value
            let endowment = self.env().transferred_value();
            self.reserve.insert(caller, &(balance + endowment));
        }

        /// Withdraw all your balance from the contract.
        #[ink(message)]
        pub fn withdraw_all_from_reserve(&mut self) {
            let caller = self.env().caller();
            let balance = self.reserve.get(caller).unwrap();
            if self.env().transfer(caller, balance).is_err() {
                panic!(
                    "Could not withdraw funds from contract reserve"
                )
            } else {
                self.reserve.remove(caller);
            }
        }

        //  Call that automatically triggers transfer out of contract into owner account
        #[ink(message)]
        pub fn make_payment(&mut self) {
            let caller = self.env().caller();
            let user_premium = self.premium.get(caller).unwrap();
            let available_balance = self.reserve.get(caller).unwrap();
            if user_premium as u128 <= available_balance {
                self.env().transfer(self.owner_account, user_premium as u128);
            } else {
                panic!(
                    "Can not make payment at this time. Your reserve balance is too low. Please restock with deposit_to_contract call."
                )
            }
            // If panic! doesn't trigger above then everything is ok. we can lower user reserve value
            self.reserve.insert(&caller, &(available_balance - user_premium as u128));
        }

        // Function for allowing a caller to change their contract details. In the future, this should be approved by contract owner
        #[ink(message)]
        pub fn set_contract_info(&mut self, premium_init: i32, deductible_init: i32, legal_name_init: String, payment_schedule_init: u32) {
            let caller = self.env().caller();
            self.premium.insert(caller, &premium_init);
            self.deductible.insert(caller, &deductible_init);
            self.legal_name.insert(caller, &legal_name_init);
            self.payment_schedule.insert(caller, &payment_schedule_init);
        }

        /// See who the contract owner is. This is also whoever is the first to deploy this contract
        #[ink(message)]
        pub fn get_contract_owner(&self) -> AccountId {
            self.owner_account
        }

        // Helper functions to retrieve the caller's value stored in maps
        // Retrieve the balance of the caller.
        
        #[ink(message)]
        pub fn get_contract_info_premium(&self) -> i32 {
            let caller = self.env().caller();
            self.premium.get(&caller).unwrap_or_default()
        }
        #[ink(message)]
        pub fn get_contract_info_deductible(&self) -> i32 {
            let caller = self.env().caller();
            self.deductible.get(&caller).unwrap_or_default()
        }
        #[ink(message)]
        pub fn get_contract_info_payment_schedule(&self) -> u32 {
            let caller = self.env().caller();
            self.payment_schedule.get(&caller).unwrap_or_default()
        }
        #[ink(message)]
        pub fn get_contract_info_legal_name(&self) -> String {
            let caller = self.env().caller();
            self.legal_name.get(&caller).unwrap_or_default()
        }
        // A user's available reserve
        #[ink(message)]
        pub fn get_contract_info_reserve(&self) -> u128 {
            let caller = self.env().caller();
            self.reserve.get(&caller).unwrap_or_default()
        }

        // Function to retrieve contract's total balance
        #[ink(message)]
        pub fn get_total_contract_balance(&self) -> u128 {
            self.env().balance()
        }

        // WARNING. Function for contract owner to empty all funds. Should not exist in prod
        #[ink(message, payable)]
        pub fn withdraw_all_owner(&mut self) {
            let caller = self.env().caller();
            let total = self.env().balance();
            if caller == self.owner_account {
                if self.env().transfer(self.owner_account, total).is_err() {
                    panic!(
                        "requested transfer failed. this can be the case if the contract does not\
                        have sufficient free funds or if the transfer would have brought the\
                        contract's balance below minimum balance."
                    )
                }

            }
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        // /// We test if the default constructor does its job.
        // #[ink::test]
        // fn default_works() {
        //     let insurance = Insurance::default();
        //     assert_eq!(insurance.get(), 0);
        // }
        // #[ink::test]
        // fn my_map_works() {
        //     let contract = Insurance::new(11);
        //     assert_eq!(contract.get(), 11);
        //     assert_eq!(contract.get_mine(), 0);
        // }

        // #[ink::test]
        // fn inc_mine_works() {
        //     let mut contract = Insurance::new(11);
        //     assert_eq!(contract.get_mine(), 0);
        //     contract.inc_mine(5);
        //     assert_eq!(contract.get_mine(), 5);
        //     contract.inc_mine(5);
        //     assert_eq!(contract.get_mine(), 10);
        // }

        // #[ink::test]
        // fn remove_mine_works() {
        //     let mut contract = Insurance::new(11);
        //     assert_eq!(contract.get_mine(), 0);
        //     contract.inc_mine(5);
        //     assert_eq!(contract.get_mine(), 5);
        //     contract.remove_mine();
        //     assert_eq!(contract.get_mine(), 0);
        // }

        // /// We test a simple use case of our contract.
        // #[ink::test]
        // fn it_works() {
        //     let mut contract = Insurance::new(42);
        //     assert_eq!(contract.get(), 42);
        //     contract.inc(5);
        //     assert_eq!(contract.get(), 47);
        //     contract.inc(-50);
        //     assert_eq!(contract.get(), -3);
        // }
         
    }
}
