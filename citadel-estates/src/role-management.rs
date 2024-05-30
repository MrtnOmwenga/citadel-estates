use near_sdk::{near_bindgen, AccountId, BorshStorage, Serialize, Deserialize};
use near_sdk::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub enum Role {
  Admin,
}

pub trait Roles {
  fn get_roles(&self, account_id: &AccountId) -> Vec<Role>;
  fn add_role(&mut self, account_id: &AccountId, role: Role);
  fn remove_role(&mut self, account_id: &AccountId, role: Role);
  fn is_admin(&self, account_id: &AccountId) -> bool;
}

#[derive(BorshStorage, Serialize, Deserialize)]
pub struct RolesManagement {
  roles: HashMap<AccountId, Vec<Role>>,
}

impl RolesManagement {
  pub fn new() -> Self {
    Self {
      roles: HashMap::new(),
    }
  }
}

#[near_bindgen]
impl RolesManagement {
  pub fn get_roles(&self, account_id: &AccountId) -> Vec<Role> {
    self.roles.get(account_id).unwrap_or(&vec![]).to_vec()
  }

  pub fn add_role(&mut self, account_id: &AccountId, role: Role) {
    let mut roles = self.roles.entry(account_id.clone()).or_insert(vec![]);
    roles.push(role);
  }

  pub fn remove_role(&mut self, account_id: &AccountId, role: Role) {
    if let Some(roles) = self.roles.get_mut(account_id) {
      roles.retain(|r| *r != role);
    }
  }

  pub fn is_admin(&self, account_id: &AccountId) -> bool {
    self.roles.get(account_id).map_or(false, |roles| roles.contains(&Role::Admin))
  }
}
