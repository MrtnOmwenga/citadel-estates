use near_sdk::{near_bindgen, AccountId, Balance};
use std::collections::HashMap;

#[near_bindgen]
pub struct ShareManagement {
  shares: HashMap<AccountId, Balance>,
}

impl Default for ShareManagement {
  fn default() -> Self {
    Self {
      shares: HashMap::new(),
    }
  }
}

#[near_bindgen]
impl ShareManagement {
  pub fn invest(&mut self, investor: AccountId, amount: Balance) {
    assert!(amount > 0, "Investment amount must be greater than zero");
    
    let investor_shares = self.shares.entry(investor.clone()).or_insert(0);
    *investor_shares += amount;

    Promise::new(env::current_account_id()).transfer(amount);
  }

  pub fn withdraw_shares(&mut self, investor: AccountId, amount: Balance) {
    let investor_shares = self.shares.get_mut(&investor).expect("Investor not found");
    assert!(*investor_shares >= amount, "Insufficient shares to withdraw");
    
    *investor_shares -= amount;
    
    Promise::new(investor.clone()).transfer(amount);
  }

  pub fn get_investor_shares(&self, investor: AccountId) -> Balance {
    *self.shares.get(&investor).unwrap_or(&0)
  }
}
