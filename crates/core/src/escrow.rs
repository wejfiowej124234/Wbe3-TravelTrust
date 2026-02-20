//! 订单托管状态机：created → escrowed → completed | disputed

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderState {
    Created,   // 下单未支付/未锁定
    Escrowed,  // 资金已锁定
    Completed, // 双方确认，已放款
    Disputed,  // 争议中
    Cancelled,
}

impl Default for OrderState {
    fn default() -> Self {
        Self::Created
    }
}

/// 托管层抽象（链下或链上）
pub trait EscrowState: Send + Sync {
    /// 是否允许对该订单进行评价（仅 completed）
    fn can_review(state: OrderState) -> bool {
        state == OrderState::Completed
    }

    /// 是否允许发起争议（仅 escrowed）
    fn can_dispute(state: OrderState) -> bool {
        state == OrderState::Escrowed
    }
}

pub struct DefaultEscrow;
impl EscrowState for DefaultEscrow {}
