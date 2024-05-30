use near_sdk::{near_bindgen, AccountId, Balance, assert, assert_eq};
use crate::RoleManagement::{RoleManagement, Role};
use std::collections::HashMap;

#[near_bindgen]
pub struct PropertyManagement {
  next_property_id: u128,
  properties: HashMap<u128, Property>,
  property_owners: HashMap<u128, AccountId>,
  property_owner_shares: HashMap<u128, Balance>,
  role_management: RoleManagement,
}

impl Default for PropertyManagement {
  fn default() -> Self {
    Self {
      next_property_id: 1,
      properties: HashMap::new(),
      property_owners: HashMap::new(),
      property_owner_shares: HashMap::new(),
      role_management: RoleManagement::new(),
    }
  }
}

#[near_bindgen]
impl PropertyManagement {
  pub fn submit_property(
    &mut self,
    value: Balance,
    rental_income: Balance,
    ownership_doc: String,
    proof_of_income: String,
    images: Vec<String>,
    sale_percentage: Balance,
    property_owner: AccountId,
  ) -> u128 {
    assert!(sale_percentage > 0, "Sale percentage must be positive");

    let property_id = self.next_property_id;
    let property = Property {
      id: property_id,
      value,
      rental_income,
      ownership_doc,
      proof_of_income,
      images,
      sale_percentage,
      status: ListingStatus::Pending,
      owner: property_owner,
    };
    self.properties.insert(property_id, property);
    self.next_property_id += 1;
    property_id
  }

  pub fn review_property(&mut self, id: String, status: ListingStatus, purchase_percentage: Balance) {
    let property_id = id.parse().expect("Invalid property ID format");
    let mut property = self.properties.get_mut(&property_id).expect("Property not found");
    assert!(self.role_management.is_admin(&env::predecessor_account_id()), "Only admin can call this function");

    match status {
      ListingStatus::Approved => {
        assert!(property.status == ListingStatus::Pending, "Property cannot be approved, current status is {:?}", property.status);
        assert!(purchase_percentage > 0 && purchase_percentage <= property.sale_percentage, "Approved status requires purchase percentage between 1 and sales percentage");

        self.property_owners.insert(property_id.clone(), property.owner);
        self.property_owner_shares.insert(property_id, sale_percentage);

        let property_owner = self.property_owners.get(&property_id).expect("Property owner not found");
        let price_to_pay = property.value * purchase_percentage / 100;
        Promise::new(property.owner.clone()).transfer(price_to_pay);
      },
      ListingStatus::Pending => {
        assert!(false, "Cannot change to pending a property that is already pending");
      },
    }

    property.status = status;
  }

  pub fn get_all_properties(&self) -> Vec<Property> {
    self.properties.values().cloned().collect()
  }

  pub fn get_properties_by_status(&self, status: ListingStatus) -> Vec<Property> {
    self.properties
      .values()
      .filter(|property| property.status == status)
      .cloned()
      .collect()
  }
}

#[derive(Debug)]
pub enum ListingStatus {
  Pending,
  Listed,
  Rejected,
}

#[derive(Default)]
pub struct Property {
  id: u128,
  value: Balance,
  rental_income: Balance,
  ownership_doc: String,
  proof_of_income: String,
  images: Vec<String>,
  sale_percentage: Balance,
  status: ListingStatus,
  owner: AccountId,
}
