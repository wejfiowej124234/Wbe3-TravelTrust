//! 订单托管状态机（与 01 §1 一致）：created → accepted → escrowed → completed | disputed → refunded/partially_refunded/slashed

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderState {
    Created,            // 下单未支付/未锁定
    Accepted,          // 导游接单，待支付/未锁定
    Escrowed,          // 资金已锁定
    Completed,         // 双方确认，已放款
    Disputed,          // 争议中
    Refunded,          // 全额退款（资金终态）
    PartiallyRefunded, // 部分退款（资金终态）
    Slashed,           // 裁决扣罚导游（资金终态）
    Cancelled,
}

impl Default for OrderState {
    fn default() -> Self {
        Self::Created
    }
}

impl OrderState {
    /// 是否为资金终态（01：仅终态可提交评价）
    pub fn is_final_financial_state(self) -> bool {
        matches!(
            self,
            OrderState::Completed | OrderState::Refunded | OrderState::PartiallyRefunded | OrderState::Slashed
        )
    }
}

/// 托管层抽象（链下或链上）
pub trait EscrowState: Send + Sync {
    /// 是否允许对该订单进行评价（仅资金终态：completed / refunded / partially_refunded / slashed）
    fn can_review(state: OrderState) -> bool {
        state.is_final_financial_state()
    }

    /// 是否允许发起争议（仅 escrowed）
    fn can_dispute(state: OrderState) -> bool {
        state == OrderState::Escrowed
    }
}

pub struct DefaultEscrow;
impl EscrowState for DefaultEscrow {}
