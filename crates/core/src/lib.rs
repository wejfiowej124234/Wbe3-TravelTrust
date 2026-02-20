//! TravelTrust 领域类型与抽象：Registry / Escrow / Staking / Reputation / Dispute
//!
//! 先链下实现，接口设计兼容后续上链。

pub mod escrow;
pub mod reputation;
pub mod staking;
pub mod types;

pub use escrow::{EscrowState, OrderState};
pub use reputation::ReviewWeight;
pub use staking::StakeTier;
pub use types::*;
