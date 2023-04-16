#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod incrementer {
    use ink::storage::Mapping;

    /// Defines storage of the contract. In this case, we want to map a user's account ID with an integer value
    #[ink(storage)]
    pub struct Incrementer {
        value: i32,
        my_map: Mapping<AccountId, i32>,
     }     

    impl Incrementer {
        
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            let mut my_map = Mapping::default();
            // Always returns contract caller. Different than origin
            // If a contract calls another contract, this returns first contract, nort original caller
            let caller = Self::env().caller();

            // & in rust is known as references. Allows us to borrow values instead of dropping them as rust normally does when you put variable into function
            my_map.insert(&caller, &0);

            Self {
                value: init_value,
                my_map,
            }
        }

        // Initialize contract storage
        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                value: 0,
                my_map: Mapping::default(),
            }
        }

        // Helper function to retrieve the caller's value stored in map
        #[ink(message)]
        pub fn get_mine(&self) -> i32 {
            let caller = self.env().caller();
            self.my_map.get(&caller).unwrap_or_default()
        }

        // Insert 'by' i32 argument into map, adding it onto existing value retrieved using get_mine()
        #[ink(message)]
        pub fn inc_mine(&mut self, by: i32) {
            let caller = self.env().caller();
            let my_value = self.get_mine();
            self.my_map.insert(caller, &(my_value + by));
        }

        // Helper function to remove a key value pair from map
        #[ink(message)]
        pub fn remove_mine(&self) {
        let caller = self.env().caller();
        self.my_map.remove(&caller)
        }
                 
        /// Increments our value by input 'by'
        #[ink(message)]
        pub fn inc(&mut self, by: i32) {
            self.value += by;
        }
        
        /// Simply returns the current value of our `i32`.
        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.value
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let incrementer = Incrementer::default();
            assert_eq!(incrementer.get(), 0);
        }
        #[ink::test]
        fn my_map_works() {
            let contract = Incrementer::new(11);
            assert_eq!(contract.get(), 11);
            assert_eq!(contract.get_mine(), 0);
        }

        #[ink::test]
        fn inc_mine_works() {
            let mut contract = Incrementer::new(11);
            assert_eq!(contract.get_mine(), 0);
            contract.inc_mine(5);
            assert_eq!(contract.get_mine(), 5);
            contract.inc_mine(5);
            assert_eq!(contract.get_mine(), 10);
        }

        #[ink::test]
        fn remove_mine_works() {
            let mut contract = Incrementer::new(11);
            assert_eq!(contract.get_mine(), 0);
            contract.inc_mine(5);
            assert_eq!(contract.get_mine(), 5);
            contract.remove_mine();
            assert_eq!(contract.get_mine(), 0);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut contract = Incrementer::new(42);
            assert_eq!(contract.get(), 42);
            contract.inc(5);
            assert_eq!(contract.get(), 47);
            contract.inc(-50);
            assert_eq!(contract.get(), -3);
        }
         
    }
}
