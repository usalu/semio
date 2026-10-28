//! 🧾️ Group schema constants, anchors and decision errors: the durable group schema, role and size constants, the anchor, member and prepared outcome records, the three member unsigned record and the decision error. See [`super`].

use super::*;
use semio_framework_dsl_record::{DslField, FieldValue, Shape};

pub const DURABLE_OWNED_GROUP_DECISION_SCHEMA_V1: &str = "semio.store.durable-owned-three-member-decision.v1";

pub const DURABLE_OWNED_GROUP_ANCHOR_SCHEMA_V1: &str = "semio.store.owned-three-member-anchor.v1";

pub const DURABLE_OWNED_GROUP_SHAPE_V1: &str = "parent-drawing-value";

pub const DURABLE_OWNED_GROUP_EVENT_MAX_BYTES: usize = 491_520;

pub const DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES: usize = 162_000;

pub const DURABLE_OWNED_GROUP_STRUCTURAL_MAX_BYTES: usize = 4_096;

pub const DURABLE_OWNED_GROUP_ID_MAX_BYTES: usize = 256;

pub const DURABLE_OWNED_GROUP_UNBOUND_OUTCOME_SCHEMA_V1: &str = "semio.store.unbound-one-item-outcome.v1";

pub const DURABLE_OWNED_GROUP_BOUND_OUTCOME_SCHEMA_V1: &str = "semio.store.bound-one-item-outcome.v1";

pub const DURABLE_OWNED_GROUP_JSON_MAX_DEPTH: usize = 32;

pub const DURABLE_OWNED_GROUP_JSON_MAX_ITEMS: usize = 65_536;

pub const PARENT_ROLE: &str = "parent";

pub const DRAWING_ROLE: &str = "drawing";

pub const VALUE_ROLE: &str = "value";

pub const PARENT_RECOVERY_SCHEMA: &str = "semio.store.one-item-outcome.gis-gismap-v1";

pub const DRAWING_RECOVERY_SCHEMA: &str = "semio.store.one-item-outcome.stdio-drawing-v1";

pub const VALUE_RECOVERY_SCHEMA: &str = "semio.store.one-item-outcome.stdio-value-v1";

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue,semio_framework_value_derive::RetireOwned)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DurableOwnedGroupAnchorV1 {
    pub schema: String,
    pub parent: semio_framework_artifact_reference::ArtifactRef,
    pub shape: String,
}

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue,semio_framework_value_derive::RetireOwned)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DurableOwnedGroupMemberV1 {
    pub role: String,
    pub reference: semio_framework_artifact_reference::ArtifactRef,
    pub owner: Option<OwnerRef>,
    pub expected_generation: u64,
    pub expected_revision: [u8; 32],
    pub recovery_schema: String,
        #[value(with = "semio_framework_value::bytes")]
pub recovery_pack: Vec<u8>,
    pub recovery_pack_sha256: String,
    pub unbound_outcome_sha256: String,
    pub post_generation: u64,
    pub post_revision: [u8; 32],
}

pub struct DurableStorePreparedOutcomesV1 {
    pub parent: DurableStorePreparedOutcomeV1,
    pub drawing: DurableStorePreparedOutcomeV1,
    pub value: DurableStorePreparedOutcomeV1,
}

impl semio_framework_dsl_record::BorrowedDslField for DurableOwnedGroupAnchorV1{
 const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Value;
}

impl semio_framework_dsl_record::BorrowedDslField for DurableOwnedGroupMemberV1{
 const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Value;
}

#[derive(Clone, Debug, PartialEq, Eq, ToValue)]
#[value(rename_all = "camelCase")]
pub(super) struct DurableOwnedThreeMemberUnsignedMemberV1 {
    pub(super) role: String,
    pub(super) reference: semio_framework_artifact_reference::ArtifactRef,
    pub(super) owner: Option<OwnerRef>,
    pub(super) expected_generation: u64,
    pub(super) expected_revision: [u8; 32],
    pub(super) recovery_schema: String,
    pub(super) unbound_outcome_sha256: String,
}

#[derive(Clone, Debug, PartialEq, Eq, ToValue)]
#[value(rename_all = "camelCase")]
pub struct DurableOwnedThreeMemberUnsignedV1 {
    pub(super) schema: String,
    pub(super) anchor: DurableOwnedGroupAnchorV1,
    pub(super) parent: DurableOwnedThreeMemberUnsignedMemberV1,
    pub(super) drawing: DurableOwnedThreeMemberUnsignedMemberV1,
    pub(super) value: DurableOwnedThreeMemberUnsignedMemberV1,
}

impl DurableOwnedThreeMemberUnsignedV1 {
    /// 🖋️ Projects the three signed members of one decision onto the unsigned canonical wire shape.
    pub fn new(schema: String, anchor: DurableOwnedGroupAnchorV1, parent: &DurableOwnedGroupMemberV1, drawing: &DurableOwnedGroupMemberV1, value: &DurableOwnedGroupMemberV1) -> Self {
        Self { schema, anchor, parent: unsigned_member(parent), drawing: unsigned_member(drawing), value: unsigned_member(value) }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DurableOwnedGroupDecisionError {
    InvalidSchema,
    InvalidIdentity,
    InvalidOwner,
    InvalidFrontier,
    InvalidHash,
    InvalidOutcome,
    NonCanonical,
    RecoveryPackTooLarge,
    EventTooLarge,
    Codec(String),
    Retirement(semio_framework_value::ValueError),
    #[cfg(feature = "durable-group-testing")]
    Preparation(semio_framework_value::ValueError),
}

impl std::fmt::Display for DurableOwnedGroupDecisionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSchema => formatter.write_str("durable owned group schema is invalid"),
            Self::InvalidIdentity => formatter.write_str("durable owned group identity is invalid"),
            Self::InvalidOwner => formatter.write_str("durable owned group owner is invalid"),
            Self::InvalidFrontier => formatter.write_str("durable owned group frontier is invalid"),
            Self::InvalidHash => formatter.write_str("durable owned group hash is invalid"),
            Self::InvalidOutcome => formatter.write_str("durable owned group prepared outcome is invalid"),
            Self::NonCanonical => formatter.write_str("durable owned group encoding is not canonical"),
            Self::RecoveryPackTooLarge => formatter.write_str("durable owned group recovery pack exceeds its member bound"),
            Self::EventTooLarge => formatter.write_str("durable owned group decision exceeds its event bound"),
            Self::Codec(message) => formatter.write_str(message),
            Self::Retirement(error) => std::fmt::Display::fmt(error, formatter),
            #[cfg(feature = "durable-group-testing")]
            Self::Preparation(error) => std::fmt::Display::fmt(error, formatter),
        }
    }
}

impl std::error::Error for DurableOwnedGroupDecisionError {}

macro_rules! value_field {
    ($kind:ty, $variant:ident) => {
        impl DslField for $kind {
            fn projection_view(&self, path: &[usize]) -> Result<semio_framework_dsl_record::native_encoding::FieldProjectionView<'_>, semio_framework_value::ValueError> { DurableBorrowedValue::$variant(self).native_view(path) }
            fn projection_key(&self, path: &[usize], index: usize) -> Result<&str, semio_framework_value::ValueError> { DurableBorrowedValue::$variant(self).native_key(path, index) }
            fn shape() -> Shape {
                Shape::Value
            }
            fn to_value(&self) -> FieldValue {
                FieldValue::Value(semio_framework_value::ToValue::to_value(self))
            }
            fn from_value(value: &FieldValue) -> Result<Self, String> {
                match value {
                    FieldValue::Value(value) => semio_framework_value::FromValue::from_value(value.clone()).map_err(|error| error.to_string()),
                    other => Err(format!("expected Value, found {other:?}")),
                }
            }
        }
    };
}

value_field!(DurableOwnedGroupAnchorV1, Anchor);
value_field!(DurableOwnedGroupMemberV1, Member);

impl From<semio_framework_value::ValueError> for DurableOwnedGroupDecisionError {
    fn from(error: semio_framework_value::ValueError) -> Self { Self::Retirement(error) }
}
