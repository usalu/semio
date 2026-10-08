//! ⚠️ Refusal identity and explicit prose ownership survive controlled codec boundaries.
/// 🏷️ Stable semantic failure identity declared by the refusal schema.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ValueRefusalKind { InvalidValue, Canceled, OwnershipLimit, AllocationFailed, WorkLimit, DepthLimit, UnsupportedOwner, InvariantViolated }
impl ValueRefusalKind {
    /// 🔤️ Returns the schema spelling without allocating or inspecting error prose.
    pub const fn as_str(self) -> &'static str {
        match self { Self::InvalidValue => "invalidValue", Self::Canceled => "canceled", Self::OwnershipLimit => "ownershipLimit", Self::AllocationFailed => "allocationFailed", Self::WorkLimit => "workLimit", Self::DepthLimit => "depthLimit", Self::UnsupportedOwner => "unsupportedOwner", Self::InvariantViolated => "invariantViolated" }
    }
}
/// 🚨️ An owned typed refusal with its complete dotted display path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValueError { pub kind: ValueRefusalKind, pub message: std::borrow::Cow<'static, str> }
impl ValueError {
    /// 🧭️ Constructs one refusal with an explicit authority at its actual failure site.
    pub fn new(kind: ValueRefusalKind, message: impl Into<String>) -> Self { Self { kind, message: std::borrow::Cow::Owned(message.into()) } }
    /// 🧱️ Returns immutable refusal prose without spending denied allocation credit.
    pub const fn literal(kind: ValueRefusalKind, message: &'static str) -> Self { Self { kind, message: std::borrow::Cow::Borrowed(message) } }
    /// 🪆️ Adds a parent field or ordinal while retaining the original refusal authority.
    pub fn under(self, segment: impl std::fmt::Display) -> Self { Self::new(self.kind, format!("{segment}.{}", self.message)) }
    /// 🗣️ Moves the prose at a declared terminal text-only boundary.
    pub fn into_message(self) -> String { self.message.into_owned() }
}
impl std::fmt::Display for ValueError { fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { formatter.write_str(&self.message) } }
impl std::error::Error for ValueError {}

#[path = "🔁️codec/🦀️.rs"]
mod controlled_codec;

#[cfg(test)]
#[path = "🔁️codec/🧪️tests/🦀️.rs"]
mod controlled_codec_tests;

#[path = "🔤️utf8/🦀️.rs"]
mod utf8;
