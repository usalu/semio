//! 🗿️ Pure artifact identities and exact first-party value admission.

use semio_framework_value::{FromValue,ToValue};
use semio_framework_value::serde::{Deserialize,Serialize};

#[path="♻️retirement/🦀️.rs"]
mod retirement;

/// 🏅️ A compile-time standard identity.
#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)]
pub struct StandardId(pub &'static str);

/// 🪆️ A compile-time subset identity.
#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)]
pub struct SubsetId(pub &'static str);
impl SubsetId { pub const ANY:Self=Self("*"); }

/// 🎯️ A compile-time artifact dialect identity.
#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)]
pub struct Dialect { pub artifact_kind:&'static str,pub standard:StandardId,pub subset:SubsetId }

/// 🧭️ An owned artifact dialect with three exact identity fields.
#[derive(Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash,Serialize,Deserialize,ToValue,FromValue)]
#[serde(crate="semio_framework_value::serde",rename_all="camelCase")]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct ArtifactDialect { pub artifact_kind:String,pub standard:String,pub subset:String }
impl From<Dialect> for ArtifactDialect {
 fn from(value:Dialect)->Self {Self{artifact_kind:value.artifact_kind.into(),standard:value.standard.0.into(),subset:value.subset.0.into()}}
}

/// 🪪️ Canonical artifact-kind id. Grammar: exactly three dot-separated ASCII segments,
/// `<domain>.<plugin>.<artifact>`, with each segment in lowercase ASCII kebab form.
#[derive(Clone, Debug, PartialEq, Eq, Hash, ToValue, FromValue)]
#[value(transparent)]
pub struct ArtifactKindId(String);

impl ArtifactKindId {
    /// 🧵️ Parses and validates the canonical grammar, failing with a message that names which
    /// rule broke.
    pub fn parse(s: &str) -> Result<Self, String> {
        if !is_canonical_artifact_kind(s) {
            return Err(format!("artifact kind {s:?} must use `<domain>.<plugin>.<artifact>` with three lowercase ASCII kebab segments"));
        }
        Ok(ArtifactKindId(s.to_string()))
    }

    /// 🔍️ Borrows the complete canonical artifact kind.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// 🌐️ Returns the declaring domain namespace.
    pub fn domain(&self) -> &str {
        self.0.split('.').next().expect("ArtifactKindId invariant: three segments")
    }

    /// 🔌️ Second segment — the owning plugin slug.
    pub fn plugin(&self) -> &str {
        self.0.split('.').nth(1).expect("ArtifactKindId invariant: exactly 3 dot-separated segments")
    }

    /// 🗿️ Third segment — the artifact slug within the plugin.
    pub fn artifact(&self) -> &str {
        self.0.split('.').nth(2).expect("ArtifactKindId invariant: exactly 3 dot-separated segments")
    }
}

impl std::fmt::Display for ArtifactKindId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// ✅️ Standalone canonical-grammar predicate behind `ArtifactKindId::parse`.
pub fn is_canonical_artifact_kind(kind: &str) -> bool {
    let mut segments = kind.split('.');
    let Some(first) = segments.next() else { return false };
    let Some(plugin) = segments.next() else { return false };
    let Some(artifact) = segments.next() else { return false };
    if segments.next().is_some() {
        return false;
    }
    is_kebab_segment(first) && is_kebab_segment(plugin) && is_kebab_segment(artifact)
}

/// 🔡️ One canonical-grammar segment: non-empty lowercase-ASCII `[a-z0-9-]`, no leading/trailing
/// hyphen, no doubled hyphen.
fn is_kebab_segment(segment: &str) -> bool {
    if segment.is_empty() || segment.starts_with('-') || segment.ends_with('-') || segment.contains("--") {
        return false;
    }
    segment.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}


/// 🔗️ One artifact identity and its owned dialect.
#[derive(Clone,Debug,PartialEq,Eq,Hash,ToValue,FromValue)]
#[value(rename_all="camelCase",deny_unknown_fields)]
pub struct ArtifactRef {pub artifact_id:String,pub dialect:ArtifactDialect}


