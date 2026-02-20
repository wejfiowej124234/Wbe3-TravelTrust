//! 通用领域类型：用户、导游、订单、争议

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Tourist,
    Guide,
    Arbitrator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub role: UserRole,
    pub kyc_status: KycStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum KycStatus {
    #[default]
    None,
    Pending,
    Verified,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guide {
    pub id: Uuid,
    pub user_id: Uuid,
    pub city: String,
    pub country_code: String,
    pub languages: Vec<String>,
    pub service_types: Vec<ServiceType>,
    pub bio: Option<String>,
    pub stake_amount: String, // 十进制字符串，便于与链上一致
    pub status: GuideStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ServiceType {
    WalkingTour,
    CarTour,
    MultiDay,
    Cultural,
    Food,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GuideStatus {
    Pending,   // 未达质押或审核中
    Active,
    Suspended,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub tourist_id: Uuid,
    pub guide_id: Uuid,
    pub amount: String,
    pub currency: String,
    pub state: crate::escrow::OrderState,
    pub escrow_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dispute {
    pub id: Uuid,
    pub order_id: Uuid,
    pub status: DisputeStatus,
    pub evidence_hashes: Vec<String>,
    pub arbitrator_id: Option<Uuid>,
    pub resolution: Option<DisputeResolution>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DisputeStatus {
    Open,
    Assigned,
    Resolved,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisputeResolution {
    /// 0.0 ~ 1.0，退给游客的比例
    pub refund_ratio: f64,
    /// 是否扣罚导游质押
    pub slash_guide: bool,
}
