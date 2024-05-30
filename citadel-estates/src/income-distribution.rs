use near_sdk::{near_bindgen, AccountId, Balance, Promise};
use std::collections::HashMap;
use crate::RoleManagement::{RoleManagement, Role};

#[near_bindgen]
pub struct IncomeDistribution {
  dividends: HashMap<AccountId, Balance>,
  role_management: RoleManagement,
}

impl Default for IncomeDistribution {
  fn default() -> Self {
    Self {
      dividends: HashMap::new(),
      role_management: RoleManagement::new(),
    }
  }
}

#[near_bindgen]
impl IncomeDistribution {
  pub fn distribute_income(&mut self, account: AccountId, amount: Balance) {
    let current_dividend = self.dividends.entry(account.clone()).or_insert(0);
    *current_dividend += amount;
  }

  pub fn distribute_dividends(&mut self, total_income: Balance) {
    assert!(self.role_management.is_admin(&env::predecessor_account_id()), "Only admin can call this function");

    let management_fee = total_income * 5 / 100;
    let distributable_income = total_income - management_fee;
    let total_shares = self.shares.values().sum();
  
    for (investor, shares) in self.shares.iter() {
      let dividend = distributable_income * shares / total_shares;
      self.distribute_income(investor.clone(), dividend);
    }
  }  

  pub fn withdraw_dividend(&mut self) -> Promise {
    let account = near_sdk::env::predecessor_account_id();
    let dividend = self.dividends.remove(&account).expect("No dividend to withdraw");
    Promise::new(account.clone()).transfer(dividend)
  }
}
