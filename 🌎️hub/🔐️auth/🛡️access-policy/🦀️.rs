//! 🛡️ The hub's one declared access policy (`🔣️.json`, schema `schema://hub.auth/HubAccessPolicyV1`)
//! and the one authority that evaluates it. Every access decision the hub takes — a directory
//! command, a document read, write or Check In, an artifact creation, a space blob — is a question
//! `(roles, action, space kind)` asked here; the only hand-written part is deriving a principal's
//! roles (membership, ownership, share token, operator subject), never deciding what a role may do.
//! Closed by default; an explicit deny overrides every allow.

use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

/// 🎭️ A role a principal holds for one request.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HubAccessRoleV1 {
    Admin,
    Owner,
    Author,
    Spectator,
    Share,
    Authenticated,
}

/// 🎬️ Every access decision the hub takes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum HubAccessActionV1 {
    #[serde(rename = "space.create")]
    SpaceCreate,
    #[serde(rename = "space.rename")]
    SpaceRename,
    #[serde(rename = "space.visibility")]
    SpaceVisibility,
    #[serde(rename = "space.archive")]
    SpaceArchive,
    #[serde(rename = "space.delete")]
    SpaceDelete,
    #[serde(rename = "member.upsert")]
    MemberUpsert,
    #[serde(rename = "member.remove")]
    MemberRemove,
    #[serde(rename = "invite.create")]
    InviteCreate,
    #[serde(rename = "invite.revoke")]
    InviteRevoke,
    #[serde(rename = "document.announce")]
    DocumentAnnounce,
    #[serde(rename = "agent.delegate")]
    AgentDelegate,
    #[serde(rename = "document.read")]
    DocumentRead,
    #[serde(rename = "document.write")]
    DocumentWrite,
    #[serde(rename = "document.check-in")]
    DocumentCheckIn,
    #[serde(rename = "artifact.create")]
    ArtifactCreate,
    #[serde(rename = "blob.read")]
    BlobRead,
    #[serde(rename = "blob.write")]
    BlobWrite,
}

/// ⚖️ Allow or deny.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HubAccessEffectV1 {
    Allow,
    Deny,
}

/// 🎫️ One declared grant; `space_kinds` limits it to spaces of those kinds.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HubAccessGrantV1 {
    pub effect: HubAccessEffectV1,
    pub roles: Vec<HubAccessRoleV1>,
    pub actions: Vec<HubAccessActionV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub space_kinds: Option<Vec<String>>,
}

/// 📜️ The declared policy document.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HubAccessPolicyV1 {
    pub schema: String,
    pub grants: Vec<HubAccessGrantV1>,
}

/// 📛️ The policy document's schema identifier.
pub const HUB_ACCESS_POLICY_SCHEMA_V1: &str = "semio.hub.access-policy/v1";

impl HubAccessPolicyV1 {
    /// 📥️ Parses and validates one policy document: known schema, 1..=64 grants, every grant naming
    /// at least one role and one action, and space kinds only from the directory's own vocabulary.
    pub fn parse(source: &str) -> Result<Self, String> {
        let policy: Self = serde_json::from_str(source).map_err(|error| format!("access policy does not parse: {error}"))?;
        if policy.schema != HUB_ACCESS_POLICY_SCHEMA_V1 || policy.grants.is_empty() || policy.grants.len() > 64 {
            return Err("access policy has an unknown schema or grant count".into());
        }
        for grant in &policy.grants {
            if grant.roles.is_empty() || grant.actions.is_empty() || grant.space_kinds.as_ref().is_some_and(|kinds| kinds.is_empty() || kinds.iter().any(|kind| !matches!(kind.as_str(), "atelier" | "studio" | "archive"))) {
                return Err("access policy grant is empty or names an unknown space kind".into());
            }
        }
        Ok(policy)
    }

    /// 🛡️ The hub's own declared policy, parsed once.
    pub fn declared() -> &'static Self {
        static DECLARED: OnceLock<HubAccessPolicyV1> = OnceLock::new();
        DECLARED.get_or_init(|| Self::parse(include_str!("🔣️.json")).expect("the hub's declared access policy is valid"))
    }

    /// ⚖️ Whether any of `roles` may perform `action` in a space of `space_kind` (`None` for an
    /// instance-wide action such as creating a space). Closed by default; deny overrides allow.
    pub fn permits(&self, roles: &[HubAccessRoleV1], action: HubAccessActionV1, space_kind: Option<&str>) -> bool {
        let mut allowed = false;
        for grant in &self.grants {
            if !grant.actions.contains(&action) || !grant.roles.iter().any(|role| roles.contains(role)) {
                continue;
            }
            if let Some(kinds) = &grant.space_kinds {
                if !space_kind.is_some_and(|kind| kinds.iter().any(|candidate| candidate == kind)) {
                    continue;
                }
            }
            match grant.effect {
                HubAccessEffectV1::Deny => return false,
                HubAccessEffectV1::Allow => allowed = true,
            }
        }
        allowed
    }
}

/// ⚖️ The one hub authority: the declared policy's decision.
pub fn hub_access_permits(roles: &[HubAccessRoleV1], action: HubAccessActionV1, space_kind: Option<&str>) -> bool {
    HubAccessPolicyV1::declared().permits(roles, action, space_kind)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
