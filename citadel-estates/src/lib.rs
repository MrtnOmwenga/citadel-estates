use crate::contracts::{PropertyManagement, ShareManagement, IncomeDistribution};
use near_sdk::Balance;
use near_sdk::near_bindgen;

pub struct REIT {
  property_management: PropertyManagement,
  share_management: ShareManagement,
  income_distribution: IncomeDistribution,
  management_fee_percentage: Balance,
  property_owner_share_percentage: Balance,
  property_value: Balance,
  constant_share_price: Balance,
}

impl Default for REIT {
  fn default() -> Self {
    Self {
      property_management: PropertyManagement::default(),
      share_management: ShareManagement::default(),
      income_distribution: IncomeDistribution::default(),
      management_fee_percentage: 0,
      property_owner_share_percentage: 0,
      property_value: 0,
      constant_share_price: 0,
    }
  }
}
