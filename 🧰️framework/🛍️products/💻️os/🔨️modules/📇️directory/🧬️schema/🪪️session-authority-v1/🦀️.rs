//! 🪪️ Closed authenticated Directory session authority.

use semio_framework_value_derive::{FromValue, ToValue};

/// 🧯️ Maximum canonical authenticated session response bytes.
pub const DIRECTORY_SESSION_AUTHORITY_MAX_BYTES: usize = 2048;

/// 🧭️ Identity provider class of the authenticated session.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
#[serde(rename_all = "kebab-case")]
#[value(rename_all = "kebab-case")]
pub enum DirectorySessionKindV1 {
    External,
    DevelopmentLocal,
}

/// 🔐️ Public identity plus a digest binding to one exact live server session.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DirectorySessionAuthorityV1 {
    pub schema: String,
    pub session_binding_sha256: String,
    pub authorization_generation: u64,
    pub user_id: String,
    pub email: String,
    pub display_name: String,
    pub expires_at: i64,
    pub session_kind: DirectorySessionKindV1,
}

fn bounded_text(value: &str, maximum: usize) -> bool {
    !value.is_empty() && value.chars().count() <= maximum && value.trim_matches(' ') == value && !value.chars().any(char::is_control)
}

impl DirectorySessionAuthorityV1 {
    /// 🛡️ Withholds secret-bearing, malformed and JavaScript-unsafe identity rows.
    pub fn validate(&self) -> bool {
        self.schema == "semio.directory.session-authority.v1"
            && self.session_binding_sha256.len() == 64
            && self.session_binding_sha256.bytes().any(|byte| byte != b'0')
            && self.session_binding_sha256.bytes().all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
            && (1..=9_007_199_254_740_991).contains(&self.authorization_generation)
            && !self.user_id.is_empty()
            && self.user_id.len() <= 256
            && self.user_id.as_bytes()[0].is_ascii_alphanumeric()
            && self.user_id.bytes().all(|byte| byte.is_ascii_alphanumeric() || b"._:/-".contains(&byte))
            && bounded_text(&self.email, 320)
            && bounded_text(&self.display_name, 128)
            && (1..=9_007_199_254_740_991).contains(&self.expires_at)
    }

    /// 📤️ Serializes only an exact bounded response.
    pub fn canonical_json(&self) -> Option<String> {
        if !self.validate() {
            return None;
        }
        let source = crate::os_pack::json::to_json_string(self);
        (source.len() <= DIRECTORY_SESSION_AUTHORITY_MAX_BYTES).then_some(source)
    }

    /// 📥️ Rejects unknown fields, reordering, padding and oversized responses.
    pub fn parse_canonical_json(source: &str) -> Option<Self> {
        if source.len() > DIRECTORY_SESSION_AUTHORITY_MAX_BYTES {
            return None;
        }
        let value: Self = crate::os_pack::json::from_json_str(source).ok()?;
        (value.validate() && crate::os_pack::json::to_json_string(&value) == source).then_some(value)
    }
}
