//! 评分权重：仅完成订单可评，权重 = f(金额, 历史, 账户年龄)

use crate::OrderState;
use serde::{Deserialize, Serialize};

/// 评价权重计算（防刷单：高价值订单权重大）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewWeight {
    pub order_amount: f64,
    pub guide_historical_score: f64,
    pub account_age_days: u64,
}

impl ReviewWeight {
    /// 简单权重：金额归一化 + 账户年龄加成
    pub fn weight(&self) -> f64 {
        let amount_factor = (self.order_amount / 1000.0).min(10.0).max(0.1);
        let age_factor = (self.account_age_days as f64 / 365.0).min(3.0).max(0.5);
        amount_factor * age_factor
    }
}

/// 仅当订单为 Completed 时允许评价
pub fn can_submit_review(state: OrderState) -> bool {
    state == OrderState::Completed
}
