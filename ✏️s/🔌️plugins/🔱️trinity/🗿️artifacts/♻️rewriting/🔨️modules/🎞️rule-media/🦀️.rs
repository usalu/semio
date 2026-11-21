//! 🎞️ The portable rule and the parameter bindings used to execute it.

use crate::Rule;
use semio_framework_graph_manifest::PropertyBag;
use semio_framework_pack_json::{JsonMemberPolicy, from_json_str};
use semio_framework_value::{DslValue, FromValue, NativeDecodeControl, NativeEncodeControl, ToValue, ValueError, ValueRefusalKind};
use semio_plugin::{MediaClass, MediaForm, MediaType};

pub const REWRITE_MEDIA_SCHEMA: &str = "trinity.rewrite.invocation";
pub const REWRITE_MEDIA_TYPE: MediaType = MediaType { class: MediaClass::Computation, form: MediaForm::Value };
pub const REWRITE_MEDIA_MAX_BYTES: usize = 64 * 1024;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(deny_unknown_fields)]
pub struct RewriteInvocation {
    pub rule: Rule,
    pub bindings: PropertyBag,
}

/// 📥️ Decodes a bounded invocation and refuses ambiguous JSON members.
pub fn decode_invocation(json: &str) -> Result<RewriteInvocation, ValueError> {
    if json.len() > REWRITE_MEDIA_MAX_BYTES - REWRITE_MEDIA_SCHEMA.len() {
        return Err(ValueError::new(ValueRefusalKind::InvalidValue, "rewrite invocation exceeds the media size limit"));
    }
    from_json_str(json, JsonMemberPolicy::Reject)
}

/// 📦️ Reads the same invocation from a host-admitted intrinsic media value.
pub fn decode_invocation_value(value: &DslValue) -> Result<RewriteInvocation, ValueError> {
    let mut accept = |_| true;
    let mut control = NativeDecodeControl::new(REWRITE_MEDIA_MAX_BYTES, &mut accept);
    RewriteInvocation::from_value_controlled(value, &mut control)
}

/// 📡️ Writes the invocation as a bounded intrinsic media value.
pub fn encode_invocation_value(invocation: &RewriteInvocation) -> Result<DslValue, ValueError> {
    let mut accept = |_| true;
    let mut control = NativeEncodeControl::new(REWRITE_MEDIA_MAX_BYTES, &mut accept);
    invocation.to_value_controlled(&mut control)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INVOCATION: &str = include_str!("🧪️tests/🔣️.json");

    #[test]
    fn invocation_matches_independent_json() {
        let invocation = decode_invocation(INVOCATION).unwrap();
        let actual = semio_framework_pack_json::to_json_string(&invocation);
        let oracle = |json: &str| serde_json::from_str::<serde_json::Value>(json).unwrap();
        assert_eq!(oracle(&actual), oracle(INVOCATION));
        assert_eq!(decode_invocation(&actual).unwrap(), invocation);
        assert_eq!(decode_invocation_value(&encode_invocation_value(&invocation).unwrap()).unwrap(), invocation);
    }

    #[test]
    fn invocation_rejects_duplicate_members_and_excess_bytes() {
        let duplicate = INVOCATION.replace("\"bindings\":", "\"bindings\": {}, \"bindings\":");
        assert!(decode_invocation(&duplicate).is_err());
        let unknown = INVOCATION.replace("\"bindings\":", "\"unknown\": null, \"bindings\":");
        assert!(decode_invocation(&unknown).is_err());
        assert!(decode_invocation(&" ".repeat(REWRITE_MEDIA_MAX_BYTES)).is_err());
    }
}
