//! 📤️ Original durable semantic fields expose finite ordinals to shared JSON and Pack producers.
use super::*;
use semio_framework_os_store_canonical_json::{ArtifactCanonicalJson, ArtifactCanonicalJsonNode as Node, ArtifactCanonicalJsonText};
use semio_framework_dsl_record::native_encoding::FieldProjectionView as View;
use semio_framework_value::{Number, ValueError, ValueRefusalKind};

/// 🧩️ Immutable decision fields keep canonical wire projection independent of planning ownership.
#[derive(Clone,Copy)]
pub struct DurableBorrowedDecisionFields<'a>{pub schema:&'a str,pub anchor:&'a DurableOwnedGroupAnchorV1,pub anchor_sha256:&'a str,pub decision_sha256:&'a str,pub parent:&'a DurableOwnedGroupMemberV1,pub drawing:&'a DurableOwnedGroupMemberV1,pub value:&'a DurableOwnedGroupMemberV1}

#[derive(Clone, Copy)]
pub enum DurableBorrowedValue<'a> {
    Anchor(&'a DurableOwnedGroupAnchorV1),
    Member(&'a DurableOwnedGroupMemberV1),
    UnsignedMember(&'a DurableOwnedGroupMemberV1),
    Decision(DurableBorrowedDecisionFields<'a>),
    Unsigned(DurableBorrowedDecisionFields<'a>),
    Reference(&'a semio_framework_artifact_reference::ArtifactRef),
    Dialect(&'a semio_framework_artifact_reference::ArtifactDialect),
    Owner(&'a OwnerRef),
    Text(&'a str),
    UInt(u64),
    Octets(&'a [u8]),
    NativeOctets(&'a [u8]),
    Null,
}

fn absent() -> ValueError { ValueError::literal(ValueRefusalKind::InvariantViolated, "durable original semantic ordinal is absent") }

impl<'a> DurableBorrowedValue<'a> {
    fn keys(self) -> &'static [&'static str] {
        match self {
            Self::Anchor(_) => &["schema", "parent", "shape"],
            Self::Member(_) => &["role", "reference", "owner", "expectedGeneration", "expectedRevision", "recoverySchema", "recoveryPack", "recoveryPackSha256", "unboundOutcomeSha256", "postGeneration", "postRevision"],
            Self::UnsignedMember(_) => &["role", "reference", "owner", "expectedGeneration", "expectedRevision", "recoverySchema", "unboundOutcomeSha256"],
            Self::Decision(_) => &["schema", "anchor", "anchorSha256", "decisionSha256", "parent", "drawing", "value"],
            Self::Unsigned(_) => &["schema", "anchor", "parent", "drawing", "value"],
            Self::Reference(_) => &["artifactId", "dialect"],
            Self::Dialect(_) => &["artifactKind", "standard", "subset"],
            Self::Owner(_) => &["parent", "slot", "childId"],
            _ => &[],
        }
    }

    fn child(self, ordinal: usize) -> Result<Self, ValueError> {
        Ok(match self {
            Self::Anchor(value) => match ordinal { 0 => Self::Text(&value.schema), 1 => Self::Reference(&value.parent), 2 => Self::Text(&value.shape), _ => return Err(absent()) },
            Self::Member(value) => match ordinal { 0 => Self::Text(&value.role), 1 => Self::Reference(&value.reference), 2 => value.owner.as_ref().map_or(Self::Null, Self::Owner), 3 => Self::UInt(value.expected_generation), 4 => Self::Octets(&value.expected_revision), 5 => Self::Text(&value.recovery_schema), 6 => Self::NativeOctets(&value.recovery_pack), 7 => Self::Text(&value.recovery_pack_sha256), 8 => Self::Text(&value.unbound_outcome_sha256), 9 => Self::UInt(value.post_generation), 10 => Self::Octets(&value.post_revision), _ => return Err(absent()) },
            Self::UnsignedMember(value) => match ordinal { 0 => Self::Text(&value.role), 1 => Self::Reference(&value.reference), 2 => value.owner.as_ref().map_or(Self::Null, Self::Owner), 3 => Self::UInt(value.expected_generation), 4 => Self::Octets(&value.expected_revision), 5 => Self::Text(&value.recovery_schema), 6 => Self::Text(&value.unbound_outcome_sha256), _ => return Err(absent()) },
            Self::Decision(value) => match ordinal { 0 => Self::Text(&value.schema), 1 => Self::Anchor(value.anchor), 2 => Self::Text(&value.anchor_sha256), 3 => Self::Text(&value.decision_sha256), 4 => Self::Member(value.parent), 5 => Self::Member(value.drawing), 6 => Self::Member(value.value), _ => return Err(absent()) },
            Self::Unsigned(value) => match ordinal { 0 => Self::Text(&value.schema), 1 => Self::Anchor(value.anchor), 2 => Self::UnsignedMember(value.parent), 3 => Self::UnsignedMember(value.drawing), 4 => Self::UnsignedMember(value.value), _ => return Err(absent()) },
            Self::Reference(value) => match ordinal { 0 => Self::Text(&value.artifact_id), 1 => Self::Dialect(&value.dialect), _ => return Err(absent()) },
            Self::Dialect(value) => match ordinal { 0 => Self::Text(&value.artifact_kind), 1 => Self::Text(&value.standard), 2 => Self::Text(&value.subset), _ => return Err(absent()) },
            Self::Owner(value) => match ordinal { 0 => Self::Reference(&value.parent), 1 => Self::Text(&value.slot), 2 => Self::Text(&value.child_id), _ => return Err(absent()) },
            Self::Octets(value) | Self::NativeOctets(value) => Self::UInt(u64::from(*value.get(ordinal).ok_or_else(absent)?)),
            _ => return Err(absent()),
        })
    }

    fn at(mut self, path: &[usize]) -> Result<Self, ValueError> { for ordinal in path { self = self.child(*ordinal)?; } Ok(self) }
    pub fn native_view(self, path: &[usize]) -> Result<View<'a>, ValueError> {
        let value = self.at(path)?;
        Ok(match value { Self::Text(text) => View::IntrinsicText(text), Self::UInt(value) => View::IntrinsicNumber(Number::UInt(value)), Self::Octets(bytes) => View::IntrinsicArray(bytes.len()), Self::NativeOctets(bytes) => View::IntrinsicBytes(bytes), Self::Null => View::IntrinsicNull, _ => View::IntrinsicObject(value.keys().len()) })
    }
    pub fn native_key(self, path: &[usize], index: usize) -> Result<&'a str, ValueError> { self.at(path)?.keys().get(index).copied().ok_or_else(absent) }
}

impl ArtifactCanonicalJson for DurableBorrowedValue<'_> {
    fn canonical_json_node(&self, path: &[usize]) -> Result<Node<'_>, ValueError> {
        let value = self.at(path)?;
        Ok(match value { Self::Text(text) => Node::String(text), Self::UInt(value) => Node::U64(value), Self::Octets(bytes) | Self::NativeOctets(bytes) => Node::Array(bytes.len()), Self::Null => Node::Null, _ => Node::Object(value.keys().len()) })
    }
    fn canonical_json_key(&self, path: &[usize], index: usize) -> Result<ArtifactCanonicalJsonText<'_>, ValueError> { self.native_key(path, index).map(Into::into) }
}
