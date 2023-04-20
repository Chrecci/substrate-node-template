#![cfg_attr(not(feature = "std"), no_std)]

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
    
    /// Specify ERC-20 error type.
    // #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    // #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    // pub enum Error {
    //     /// Return if the balance cannot fulfill a request.
    //     InsufficientBalance,
    //     InsufficientAllowance,
    //     NotOrigin
        
    // }
    // pub type Result<T> = core::result::Result<T, Error>;

    impl Insurance {
        
        // When instantiating a new contract, and only when instantiating, we can permanently set an owner address. Only this owner address can withdraw funds
        #[ink(constructor)]
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

        // Helper function to retrieve the caller's value stored in map
        #[ink(message)]
        pub fn get_mine(&self) -> i32 {
            let caller = self.env().caller();
            self.premium.get(&caller).unwrap_or_default()
        }

        // Function to retrieve contract's total balance
        #[ink(message)]
        pub fn get_mine(&self) -> i32 {
            let caller = self.env().caller();
            self.premium.get(&caller).unwrap_or_default()
        }

        // Function to retrieve caller's reserve
        #[ink(message)]
        pub fn get_my_reserve(&self) -> i32 {
            let caller = self.env().caller();
            self.reserve.get(&caller).unwrap_or_default()
        }
        pub fn inc_my_reserve(&mut self, by: i32) {
                let caller = self.env().caller();
                let my_reserve = self.get_my_reserve();
                self.reserve.insert(caller, &(my_reserve + by));
            }

        #[ink(message, payable)]
        pub fn deposit(&mut self, value: Balance) {
            let amount = Self::env().transferred_value();


        }


        #[ink(message, payable)]
        pub fn withdraw_all(&mut self) {

            let total = self.env().balance()
            if self.env().transfer(self.owner_account, total).is_err() {
                panic!(
                    "requested transfer failed. this can be the case if the contract does not\
                    have sufficient free funds or if the transfer would have brought the\
                    contract's balance below minimum balance."
                )
            }
        }

        // // Insert 'by' i32 argument into map, adding it onto existing value retrieved using get_mine()
        // #[ink(message)]
        // pub fn inc_mine(&mut self, by: i32) {
        //     let caller = self.env().caller();
        //     let my_value = self.get_mine();
        //     self.my_map.insert(caller, &(my_value + by));
        // }

        // // Helper function to remove a key value pair from map
        // #[ink(message)]
        // pub fn remove_mine(&self) {
        // let caller = self.env().caller();
        // self.my_map.remove(&caller)
        // }
                 
        // /// Increments our value by input 'by'
        // #[ink(message)]
        // pub fn inc(&mut self, by: i32) {
        //     self.value += by;
        // }
        
        // /// Simply returns the current value of our `i32`.
        // #[ink(message)]
        // pub fn get(&self) -> i32 {
        //     self.value
        // }
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
