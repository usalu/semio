//! 🛡️ The directory's one declared access policy (`🔣️.json`, schema `schema://os.directory.access-policy/DirectoryAccessPolicyV1`)
//! and the one authority that evaluates it. Every access decision the hub takes — a directory
//! command, a document read, write or Check In, an artifact creation, a space blob — is a question
//! `(roles, action, space kind)` asked here; the only hand-written part is deriving a principal's
//! roles (membership, ownership, share token, operator subject), never deciding what a role may do.
//! Closed by default; an explicit deny overrides every allow.

use serde::{Deserialize, Serialize};

/// 🎭️ A role a principal holds for one request.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DirectoryAccessRoleV1 {
    Admin,
    Owner,
    Author,
    Spectator,
    Share,
    Authenticated,
    /// 🤖️ An agent session under a `read` delegation, in its delegation's one space.
    AgentReader,
    /// 🤖️ An agent session under an `edit` delegation, in its delegation's one space.
    AgentEditor,
}

/// 🎬️ Every access decision the hub takes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum DirectoryAccessActionV1 {
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
    #[serde(rename = "preference.record")]
    PreferenceRecord,
    #[serde(rename = "preference.read")]
    PreferenceRead,
}

/// ⚖️ Allow or deny.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DirectoryAccessEffectV1 {
    Allow,
    Deny,
}

/// 🎫️ One declared grant; `space_kinds` limits it to spaces of those kinds.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DirectoryAccessGrantV1 {
    pub effect: DirectoryAccessEffectV1,
    pub roles: Vec<DirectoryAccessRoleV1>,
    pub actions: Vec<DirectoryAccessActionV1>,
    #[serde(default, deserialize_with = "deserialize_space_kinds", skip_serializing_if = "Option::is_none")]
    pub space_kinds: Option<Vec<String>>,
}

fn deserialize_space_kinds<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Option<Vec<String>>, D::Error> {
    Vec::<String>::deserialize(deserializer).map(Some)
}

/// 📜️ The declared policy document.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DirectoryAccessPolicyV1 {
    pub schema: String,
    pub grants: Vec<DirectoryAccessGrantV1>,
}

/// 📛️ The policy document's schema identifier.
pub const DIRECTORY_ACCESS_POLICY_SCHEMA_V1: &str = "semio.os.directory.access-policy/v1";

impl DirectoryAccessPolicyV1 {
    

    

    /// ⚖️ Whether any of `roles` may perform `action` in a space of `space_kind` (`None` for an
    /// instance-wide action such as creating a space). Closed by default; deny overrides allow.
    pub fn permits(&self, roles: &[DirectoryAccessRoleV1], action: DirectoryAccessActionV1, space_kind: Option<&str>) -> bool {
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
                DirectoryAccessEffectV1::Deny => return false,
                DirectoryAccessEffectV1::Allow => allowed = true,
            }
        }
        allowed
    }
}



#[cfg(test)]
#[path = "../🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
