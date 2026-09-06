//! 🌐️ Path-free actor identities; schema and TypeScript twin are adjacent.

use semio_framework_value_derive::{FromValue, ToValue};

/// 📏️ One bounded derived browser actor body.
pub const DOCUMENT_BROWSER_ACTOR_MAX_BYTES: u64 = 67_108_864;

/// 🧮️ Exact bounded integer admission without fractional truncation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DocumentBrowserActorByteLengthV1(u64);

impl DocumentBrowserActorByteLengthV1 {
    /// 🆕️ Admits one nonzero body length.
    pub fn new(value: u64) -> Result<Self, DocumentBrowserActorErrorV1> {
        (1..=DOCUMENT_BROWSER_ACTOR_MAX_BYTES).contains(&value).then_some(Self(value)).ok_or(DocumentBrowserActorErrorV1::InvalidIdentity)
    }

    /// 🔢️ Returns the already-bounded body length.
    pub fn get(self) -> u64 {
        self.0
    }
}

impl crate::ToValue for DocumentBrowserActorByteLengthV1 {
    fn to_value(&self) -> crate::DslValue {
        crate::DslValue::uint(self.0)
    }
}

impl crate::FromValue for DocumentBrowserActorByteLengthV1 {
    fn from_value(value: crate::DslValue) -> Result<Self, crate::ValueError> {
        let number = value
            .as_f64()
            .filter(|number| number.is_finite() && number.fract() == 0.0 && *number >= 1.0 && *number <= DOCUMENT_BROWSER_ACTOR_MAX_BYTES as f64)
            .ok_or_else(|| crate::ValueError::new("expected an exact bounded browser actor length"))?;
        Self::new(number as u64).map_err(|_| crate::ValueError::new("invalid browser actor length"))
    }
}

/// 🛂️ Exact canonical vocabulary admitted by the closed actor compiler.
pub const DOCUMENT_BROWSER_ACTOR_INTERFACES: [&str; 16] = [
    "semio:framework/host-async@1.0.0",
    "semio:framework/pure@1.0.0",
    "wasi:cli/environment@0.2.0",
    "wasi:cli/exit@0.2.0",
    "wasi:cli/stderr@0.2.0",
    "wasi:cli/stdin@0.2.0",
    "wasi:cli/stdout@0.2.0",
    "wasi:cli/terminal-input@0.2.0",
    "wasi:cli/terminal-output@0.2.0",
    "wasi:cli/terminal-stderr@0.2.0",
    "wasi:cli/terminal-stdin@0.2.0",
    "wasi:cli/terminal-stdout@0.2.0",
    "wasi:clocks/monotonic-clock@0.2.0",
    "wasi:io/error@0.2.0",
    "wasi:io/poll@0.2.0",
    "wasi:io/streams@0.2.0",
];

/// 🧷️ Captured package identities supplied independently of the candidate actor.
#[derive(Clone, Copy, Debug)]
pub struct DocumentBrowserActorSourceV1<'a> {
    pub component_sha256: &'a str,
    pub descriptor_byte_sha256: &'a str,
}

/// ⛔️ A structurally invalid, unbound, or renderer-inverted actor identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DocumentBrowserActorErrorV1 {
    InvalidIdentity,
}

/// 🧭️ A plan identity has no length, path, policy source, or byte owner.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "kebab-case", rename_all_fields = "camelCase", deny_unknown_fields)]
pub enum DocumentOpenBrowserActorV1 {
    None,
    ClosedBrowserActor { schema: String, codegen_policy: String, sha256: String, source_component_sha256: String, source_descriptor_byte_sha256: String, policy_sha256: String, import_interfaces: Vec<String> },
}

/// 🔏️ A lease identity additionally binds the selected nonzero bounded length.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "kebab-case", rename_all_fields = "camelCase", deny_unknown_fields)]
pub enum DocumentExecutionTargetBrowserActorV1 {
    None,
    ClosedBrowserActor {
        schema: String,
        codegen_policy: String,
        sha256: String,
        source_component_sha256: String,
        source_descriptor_byte_sha256: String,
        policy_sha256: String,
        import_interfaces: Vec<String>,
        byte_length: DocumentBrowserActorByteLengthV1,
    },
}

struct IdentityRef<'a> {
    schema: &'a str,
    policy: &'a str,
    sha256: &'a str,
    component: &'a str,
    descriptor: &'a str,
    policy_sha256: &'a str,
    interfaces: &'a [String],
}

fn digest(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)) && value.bytes().any(|byte| byte != b'0')
}

fn validate(identity: Option<IdentityRef<'_>>, source: DocumentBrowserActorSourceV1<'_>, renderer: &str) -> Result<(), DocumentBrowserActorErrorV1> {
    let valid = match identity {
        None => matches!(renderer, "react" | "wgpu"),
        Some(row) => {
            renderer == "wasm"
                && row.schema == "semio.os.closed-browser-actor.v1"
                && row.policy == "semio.os.browser-jco-1.27.0-jspi.v1"
                && [row.sha256, row.component, row.descriptor, row.policy_sha256].into_iter().all(digest)
                && row.component == source.component_sha256
                && row.descriptor == source.descriptor_byte_sha256
                && row.interfaces.len() <= DOCUMENT_BROWSER_ACTOR_INTERFACES.len()
                && row.interfaces.iter().all(|entry| DOCUMENT_BROWSER_ACTOR_INTERFACES.contains(&entry.as_str()))
                && row.interfaces.windows(2).all(|pair| pair[0] < pair[1])
        }
    };
    valid.then_some(()).ok_or(DocumentBrowserActorErrorV1::InvalidIdentity)
}

impl DocumentOpenBrowserActorV1 {
    /// ✅️ Enforces schema, source, renderer, and canonical interface bindings.
    pub fn validate(&self, source: DocumentBrowserActorSourceV1<'_>, renderer: &str) -> Result<(), DocumentBrowserActorErrorV1> {
        let row = match self {
            Self::None => None,
            Self::ClosedBrowserActor { schema, codegen_policy, sha256, source_component_sha256, source_descriptor_byte_sha256, policy_sha256, import_interfaces } => {
                Some(IdentityRef { schema, policy: codegen_policy, sha256, component: source_component_sha256, descriptor: source_descriptor_byte_sha256, policy_sha256, interfaces: import_interfaces })
            }
        };
        validate(row, source, renderer)
    }

    /// 🧾️ Revalidates package and renderer bindings before adding the selected length.
    pub fn to_lease(&self, source: DocumentBrowserActorSourceV1<'_>, renderer: &str, byte_length: Option<u64>) -> Result<DocumentExecutionTargetBrowserActorV1, DocumentBrowserActorErrorV1> {
        self.validate(source, renderer)?;
        match (self, byte_length) {
            (Self::None, None) => Ok(DocumentExecutionTargetBrowserActorV1::None),
            (Self::ClosedBrowserActor { schema, codegen_policy, sha256, source_component_sha256, source_descriptor_byte_sha256, policy_sha256, import_interfaces }, Some(byte_length))
                if (1..=DOCUMENT_BROWSER_ACTOR_MAX_BYTES).contains(&byte_length) =>
            {
                Ok(DocumentExecutionTargetBrowserActorV1::ClosedBrowserActor {
                    schema: schema.clone(),
                    codegen_policy: codegen_policy.clone(),
                    sha256: sha256.clone(),
                    source_component_sha256: source_component_sha256.clone(),
                    source_descriptor_byte_sha256: source_descriptor_byte_sha256.clone(),
                    policy_sha256: policy_sha256.clone(),
                    import_interfaces: import_interfaces.clone(),
                    byte_length: DocumentBrowserActorByteLengthV1::new(byte_length)?,
                })
            }
            _ => Err(DocumentBrowserActorErrorV1::InvalidIdentity),
        }
    }
}

impl DocumentExecutionTargetBrowserActorV1 {
    /// ☑️ Enforces all plan identity relations and the independently bounded length.
    pub fn validate(&self, source: DocumentBrowserActorSourceV1<'_>, renderer: &str) -> Result<(), DocumentBrowserActorErrorV1> {
        let row = match self {
            Self::None => None,
            Self::ClosedBrowserActor { schema, codegen_policy, sha256, source_component_sha256, source_descriptor_byte_sha256, policy_sha256, import_interfaces, byte_length } => {
                if !(1..=DOCUMENT_BROWSER_ACTOR_MAX_BYTES).contains(&byte_length.get()) {
                    return Err(DocumentBrowserActorErrorV1::InvalidIdentity);
                }
                Some(IdentityRef { schema, policy: codegen_policy, sha256, component: source_component_sha256, descriptor: source_descriptor_byte_sha256, policy_sha256, interfaces: import_interfaces })
            }
        };
        validate(row, source, renderer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_browser_actor_v1_matches_language_neutral_fixture() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔣️.json")).expect("neutral actor corpus");
        let source = DocumentBrowserActorSourceV1 { component_sha256: fixture["componentSha256"].as_str().unwrap(), descriptor_byte_sha256: fixture["descriptorByteSha256"].as_str().unwrap() };
        for law in fixture["cases"].as_array().unwrap() {
            let mut candidate = if law["kind"] == "none" { serde_json::json!({ "kind": "none" }) } else { fixture["closed"].clone() };
            if law["view"] == "lease" && law["kind"] == "closed" {
                candidate["byteLength"] = fixture["byteLength"].clone();
            }
            if let Some(fields) = law["set"].as_object() {
                candidate.as_object_mut().unwrap().extend(fields.clone());
            }
            if let Some(field) = law["remove"].as_str() {
                candidate.as_object_mut().unwrap().remove(field);
            }
            let json = serde_json::to_string(&candidate).unwrap();
            let renderer = law["renderer"].as_str().unwrap();
            let accepted = if law["view"] == "plan" {
                match crate::os_pack::json::from_json_str::<DocumentOpenBrowserActorV1>(&json) {
                    Ok(value) if value.validate(source, renderer).is_ok() => {
                        assert_eq!(serde_json::from_str::<serde_json::Value>(&crate::os_pack::json::to_json_string(&value)).unwrap(), candidate);
                        true
                    }
                    _ => false,
                }
            } else {
                match crate::os_pack::json::from_json_str::<DocumentExecutionTargetBrowserActorV1>(&json) {
                    Ok(value) if value.validate(source, renderer).is_ok() => {
                        assert_eq!(serde_json::from_str::<serde_json::Value>(&crate::os_pack::json::to_json_string(&value)).unwrap(), candidate);
                        true
                    }
                    _ => false,
                }
            };
            assert_eq!(accepted, law["accepted"].as_bool().unwrap(), "{}", law["id"]);
        }
        let plan: DocumentOpenBrowserActorV1 = crate::os_pack::json::from_json_str(&serde_json::to_string(&fixture["closed"]).unwrap()).unwrap();
        let length = fixture["byteLength"].as_u64().unwrap();
        let lease = plan.to_lease(source, "wasm", Some(length)).unwrap();
        let encoded: serde_json::Value = serde_json::from_str(&crate::os_pack::json::to_json_string(&lease)).unwrap();
        let mut expected = fixture["closed"].clone();
        expected["byteLength"] = fixture["byteLength"].clone();
        assert_eq!(encoded, expected);
        for field in expected.as_object().unwrap().keys() {
            let mut changed = expected.clone();
            if field == "kind" {
                changed = serde_json::json!({ "kind": "none" });
            } else if field == "importInterfaces" {
                changed[field] = serde_json::json!([]);
            } else if field == "byteLength" {
                changed[field] = serde_json::json!(length + 1);
            } else {
                changed[field] = serde_json::json!("different");
            }
            let other: DocumentExecutionTargetBrowserActorV1 = crate::os_pack::json::from_json_str(&serde_json::to_string(&changed).unwrap()).unwrap();
            assert_ne!(lease, other, "equality omitted {field}");
        }
        assert_eq!(DocumentOpenBrowserActorV1::None.to_lease(source, "react", None), Ok(DocumentExecutionTargetBrowserActorV1::None));
        assert!(DocumentOpenBrowserActorV1::None.to_lease(source, "react", Some(1)).is_err());
        assert!(DocumentOpenBrowserActorV1::None.to_lease(source, "wasm", None).is_err());
        for length in [None, Some(0), Some(DOCUMENT_BROWSER_ACTOR_MAX_BYTES + 1)] {
            assert!(plan.to_lease(source, "wasm", length).is_err());
        }
        assert!(plan.to_lease(source, "react", Some(length)).is_err());
        assert!(plan.to_lease(DocumentBrowserActorSourceV1 { component_sha256: source.descriptor_byte_sha256, ..source }, "wasm", Some(length)).is_err());
        assert!(plan.to_lease(DocumentBrowserActorSourceV1 { descriptor_byte_sha256: source.component_sha256, ..source }, "wasm", Some(length)).is_err());
    }
}
