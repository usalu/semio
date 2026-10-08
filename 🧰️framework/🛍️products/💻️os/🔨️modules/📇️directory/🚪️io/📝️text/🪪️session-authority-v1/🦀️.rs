//! 🚪️ Physical Directory wire admission, canonical emission and bounded receipts.

use super::super::super::schema::session_authority::*;

impl DirectorySessionAuthorityV1 {
    /// 📤️ Canonical json.
    pub fn canonical_json(&self) -> Option<String> {
        if !self.validate() {
            return None;
        }
        let source = semio_framework_pack_json::to_json_string(self);
        (source.len() <= DIRECTORY_SESSION_AUTHORITY_MAX_BYTES).then_some(source)
    }

    /// 📥️ Parse canonical json.
    pub fn parse_canonical_json(source: &str) -> Option<Self> {
        if source.len() > DIRECTORY_SESSION_AUTHORITY_MAX_BYTES {
            return None;
        }
        let value: Self = semio_framework_pack_json::from_json_str(source, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()?;
        (value.validate() && semio_framework_pack_json::to_json_string(&value) == source).then_some(value)
    }
}

