//! 导游质押准入：按城市/语言/服务类型分档（预留）

use serde::{Deserialize, Serialize};

/// 质押档位，用于门槛与曝光
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StakeTier {
    /// 低质押，低曝光（冷启动用）
    Basic,
    Standard,
    Premium,
}

impl StakeTier {
    /// 最小质押金额（十进制字符串，与链一致）
    pub fn min_stake(tier: &StakeTier) -> &'static str {
        match tier {
            StakeTier::Basic => "100",
            StakeTier::Standard => "500",
            StakeTier::Premium => "2000",
        }
    }
}
