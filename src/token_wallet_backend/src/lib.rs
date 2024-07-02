use ic_cdk::update;
use ic_cdk::query;

use candid::Principal;
use std::collections::HashMap;

thread_local! {
    static STATE: std::cell::RefCell<State> = std::cell::RefCell::new(State::default());
}

#[derive(Default)]
struct State {
    balances: HashMap<Principal, u64>,
}

#[update]
fn set_initial_balance(address: Principal, amount: u64) {
    if cfg!(debug_assertions) {
        
        if address.to_text().is_empty() {
            panic!("Invalid address: Address cannot be empty");
        }

        if amount == 0 {
            panic!("Invalid amount: Amount must be greater than zero");
        }

        STATE.with(|state| {
            let mut state = state.borrow_mut();
            state.balances.insert(address, amount);
        });
    }
}

#[query]
fn get_balance(address: Principal) -> u64 {
    STATE.with(|state| {
        let state = state.borrow();
        *state.balances.get(&address).unwrap_or(&0)
    })
}

#[update]
fn send_token(from: Principal, to: Principal, amount: u64) {
    STATE.with(|state| {
        let mut state_ref = state.borrow_mut();

        // Borrow from state_ref instead of state again
        let from_balance = state_ref.balances.entry(from).or_insert(0);
        if *from_balance >= amount {
            *from_balance -= amount;
        }

        // No need to borrow mutably again here
        let to_balance = state_ref.balances.entry(to).or_insert(0);
        *to_balance += amount;
    });
}


// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_balance() {
        let principal = Principal::anonymous();
        set_initial_balance(principal, 1000);
        assert_eq!(get_balance(principal), 1000);
    }

    #[test]
    fn test_send_token() {
        // Setup initial state
        let from = Principal::anonymous();
        let to = Principal::anonymous();
        let initial_balance_from = 1000;
        let initial_balance_to = 0;
        
        STATE.with(|state| {
            let mut state_ref = state.borrow_mut();
            state_ref.balances.insert(from, initial_balance_from);
            state_ref.balances.insert(to, initial_balance_to);
        });
        
        // Call send_token
        send_token(from, to, 500);
        
        // Assert expected balances
        let expected_balance_from = 500;
        let expected_balance_to = 500;
        
        STATE.with(|state| {
            let state_ref = state.borrow();
            let actual_balance_from = *state_ref.balances.get(&from).unwrap_or(&0);
            let actual_balance_to = *state_ref.balances.get(&to).unwrap_or(&0);
            
            assert_eq!(actual_balance_from, expected_balance_from);
            assert_eq!(actual_balance_to, expected_balance_to);
        });
    }
}

