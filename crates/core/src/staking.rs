//! 导游质押准入：按 08-3 stakeTierThresholds=500,2000,10000、stakeToOrderCapMap=1,3,10 分档

use serde::{Deserialize, Serialize};

/// 质押档位（与 08-3 SSOT 一致：500 / 2000 / 10000 USDC）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StakeTier {
    /// 500 USDC 起，接单上限 1（08-3 stakeToOrderCapMap）
    Standard,
    /// 2000 USDC 起，接单上限 3
    Premium,
    /// 10000 USDC 起，接单上限 10
    Enterprise,
}

impl StakeTier {
    /// 档位最低质押金额（十进制字符串，与 08-3 stakeTierThresholds 一致）
    pub fn min_stake(tier: &StakeTier) -> &'static str {
        match tier {
            StakeTier::Standard => "500",
            StakeTier::Premium => "2000",
            StakeTier::Enterprise => "10000",
        }
    }

    /// 档位接单上限（与 08-3 stakeToOrderCapMap 一致）
    pub fn order_cap(tier: &StakeTier) -> u32 {
        match tier {
            StakeTier::Standard => 1,
            StakeTier::Premium => 3,
            StakeTier::Enterprise => 10,
        }
    }
}
