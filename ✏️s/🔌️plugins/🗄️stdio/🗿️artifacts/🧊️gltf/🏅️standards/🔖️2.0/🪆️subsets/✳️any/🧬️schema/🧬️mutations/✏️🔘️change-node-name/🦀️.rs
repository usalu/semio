//! 🧬️ Direct change-node-name mutation owner.
use crate::artifacts::gltf::schema::modules::mutation_support::structure_geometry::checked_index;
use crate::artifacts::gltf::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use crate::artifacts::gltf::GltfSnapshot;
use dsl::DslValue;

//#region 🔖️Payload
pub const ID: &str = "s.stdio.gltf.mutation.change-node-name.v1";

/// 🕳️ `value`/`before`/`after` need no `#[value(...)]` attribute at all (not even `deserialize_with
/// = "required_option"`, the `serde` equivalent needed here): `serde`'s derive implicitly treats
/// EVERY `Option<T>` field as if `#[serde(default)]` were present (silently defaults to `None` on
/// a missing key) unless overridden — `required_option` existed only to defeat that leniency. This
/// derive has the OPPOSITE default: a field with no `#[value(default)]` is required regardless of
/// its type, so a bare `Option<String>` field already rejects a missing key while still decoding a
/// present `null` as `None` (the blanket `impl<T: FromValue> FromValue for Option<T>` handles that
/// distinction on its own) — exactly the semantics `required_option` was hand-rolled to get.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GltfChangeNodeNamePayload {
    pub node: u32,
    pub value: Option<String>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GltfChangeNodeNameRestore {
    pub node: u32,
    pub before: Option<String>,
    pub after: Option<String>,
}
//#endregion 🔖️Payload

//#region 🔗️Facade
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GltfChangeNodeNameFacadeError {
    pub code: &'static str,
    pub path: String,
}

impl std::fmt::Display for GltfChangeNodeNameFacadeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{} at {}", self.code, self.path)
    }
}

impl std::error::Error for GltfChangeNodeNameFacadeError {}

type FacadeResult<T> = Result<T, GltfChangeNodeNameFacadeError>;

fn facade_error<T>(code: &'static str, path: impl Into<String>) -> FacadeResult<T> {
    Err(GltfChangeNodeNameFacadeError { code, path: path.into() })
}

fn exact_object<'a>(value: &'a DslValue, allowed: &[&str], path: &str) -> FacadeResult<&'a [(String, DslValue)]> {
    let object = match value.as_object() {
        Some(object) => object,
        None => return facade_error("object", path),
    };
    if object.iter().any(|(key, _)| !allowed.contains(&key.as_str())) {
        return facade_error("unknown", path);
    }
    if object.iter().enumerate().any(|(index, (key, _))| object[..index].iter().any(|(prior, _)| prior == key)) {
        return facade_error("duplicate", path);
    }
    Ok(object)
}

fn object_field<'a>(object: &'a [(String, DslValue)], key: &str) -> Option<&'a DslValue> {
    object.iter().find(|(name, _)| name == key).map(|(_, value)| value)
}

fn json_node(value: &DslValue, path: &str) -> FacadeResult<u32> {
    match value.as_f64() {
        Some(node) if node.is_finite() && node.fract() == 0.0 && (0.0..=f64::from(u32::MAX)).contains(&node) => Ok(node as u32),
        _ => facade_error("node", path),
    }
}

fn graphql_optional(value: &DslValue, path: &str) -> FacadeResult<Option<String>> {
    let object = exact_object(value, &["present", "absent"], path)?;
    match (object_field(object, "present"), object_field(object, "absent")) {
        (Some(DslValue::String(value)), None) => Ok(Some(value.clone())),
        (None, Some(DslValue::Bool(true))) => Ok(None),
        _ => facade_error("nullable", path),
    }
}

fn proto_optional(value: &DslValue, path: &str) -> FacadeResult<Option<String>> {
    let object = exact_object(value, &["present", "absent"], path)?;
    match (object_field(object, "present"), object_field(object, "absent")) {
        (Some(DslValue::String(value)), None) => Ok(Some(value.clone())),
        (None, Some(absent)) if exact_object(absent, &[], path)?.is_empty() => Ok(None),
        _ => facade_error("nullable", path),
    }
}

fn decode_object(value: &DslValue, optional: fn(&DslValue, &str) -> FacadeResult<Option<String>>, path: &str) -> FacadeResult<ChangeNodeNameMutation> {
    let root = exact_object(value, &["apply", "restore"], path)?;
    match (object_field(root, "apply"), object_field(root, "restore")) {
        (Some(apply), None) => {
            let apply = exact_object(apply, &["node", "value"], &format!("{path}.apply"))?;
            let node = match object_field(apply, "node") {
                Some(node) => json_node(node, &format!("{path}.apply.node"))?,
                None => return facade_error("node", format!("{path}.apply.node")),
            };
            let value = match object_field(apply, "value") {
                Some(value) => optional(value, &format!("{path}.apply.value"))?,
                None => return facade_error("nullable", format!("{path}.apply.value")),
            };
            Ok(ChangeNodeNameMutation::Apply(GltfChangeNodeNamePayload { node, value }))
        }
        (None, Some(restore)) => {
            let restore = exact_object(restore, &["node", "before", "after"], &format!("{path}.restore"))?;
            let node = match object_field(restore, "node") {
                Some(node) => json_node(node, &format!("{path}.restore.node"))?,
                None => return facade_error("node", format!("{path}.restore.node")),
            };
            let before = match object_field(restore, "before") {
                Some(value) => optional(value, &format!("{path}.restore.before"))?,
                None => return facade_error("nullable", format!("{path}.restore.before")),
            };
            let after = match object_field(restore, "after") {
                Some(value) => optional(value, &format!("{path}.restore.after"))?,
                None => return facade_error("nullable", format!("{path}.restore.after")),
            };
            Ok(ChangeNodeNameMutation::Restore(GltfChangeNodeNameRestore { node, before, after }))
        }
        _ => facade_error("phase", path),
    }
}

pub fn decode_gltf_change_node_name_graphql(value: &DslValue) -> FacadeResult<ChangeNodeNameMutation> {
    decode_object(value, graphql_optional, "graphql")
}

pub fn decode_gltf_change_node_name_proto(value: &DslValue) -> FacadeResult<ChangeNodeNameMutation> {
    decode_object(value, proto_optional, "proto")
}

struct FacadeProtobufReader<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> FacadeProtobufReader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    fn done(&self) -> bool {
        self.position == self.bytes.len()
    }

    fn varint(&mut self, path: &str) -> FacadeResult<u64> {
        let start = self.position;
        let mut value = 0_u64;
        for shift in 0..10 {
            let byte = match self.bytes.get(self.position) {
                Some(byte) => *byte,
                None => return facade_error("truncated", path),
            };
            self.position += 1;
            if shift == 9 && byte > 1 {
                return facade_error("varint", path);
            }
            value |= u64::from(byte & 0x7f) << (shift * 7);
            if byte & 0x80 == 0 {
                let width = if value == 0 { 1 } else { ((64 - value.leading_zeros() + 6) / 7) as usize };
                if self.position - start != width {
                    return facade_error("nonminimal", path);
                }
                return Ok(value);
            }
        }
        facade_error("varint", path)
    }

    fn key(&mut self, path: &str) -> FacadeResult<(u32, u8)> {
        let key = self.varint(path)?;
        let field = u32::try_from(key >> 3).map_err(|_| GltfChangeNodeNameFacadeError { code: "field", path: path.to_string() })?;
        if field == 0 {
            return facade_error("field", path);
        }
        Ok((field, (key & 7) as u8))
    }

    fn bytes(&mut self, path: &str) -> FacadeResult<&'a [u8]> {
        let length = self.varint(path)?;
        let length = usize::try_from(length).map_err(|_| GltfChangeNodeNameFacadeError { code: "length", path: path.to_string() })?;
        let end = self.position.checked_add(length).ok_or_else(|| GltfChangeNodeNameFacadeError { code: "length", path: path.to_string() })?;
        let bytes = match self.bytes.get(self.position..end) {
            Some(bytes) => bytes,
            None => return facade_error("truncated", path),
        };
        self.position = end;
        Ok(bytes)
    }

    fn message(&mut self, path: &str) -> FacadeResult<FacadeProtobufReader<'a>> {
        Ok(FacadeProtobufReader::new(self.bytes(path)?))
    }
}

fn protobuf_optional(reader: &mut FacadeProtobufReader<'_>, path: &str) -> FacadeResult<Option<String>> {
    let (field, wire) = reader.key(path)?;
    let value = match (field, wire) {
        (1, 2) => match String::from_utf8(reader.bytes(path)?.to_vec()) {
            Ok(value) => Some(value),
            Err(_) => return facade_error("utf8", path),
        },
        (2, 2) => {
            if !reader.message(path)?.done() {
                return facade_error("absent", path);
            }
            None
        }
        (1 | 2, _) => return facade_error("wire", path),
        _ => return facade_error("unknown", path),
    };
    if !reader.done() {
        return facade_error("duplicate", path);
    }
    Ok(value)
}

fn protobuf_apply(reader: &mut FacadeProtobufReader<'_>, path: &str) -> FacadeResult<ChangeNodeNameMutation> {
    let mut node = None;
    let mut value = None;
    while !reader.done() {
        let (field, wire) = reader.key(path)?;
        match field {
            1 if wire == 0 && node.is_none() => node = Some(u32::try_from(reader.varint(path)?).map_err(|_| GltfChangeNodeNameFacadeError { code: "node", path: path.to_string() })?),
            2 if wire == 2 && value.is_none() => {
                let mut nullable = reader.message(path)?;
                value = Some(protobuf_optional(&mut nullable, path)?);
            }
            1 | 2 if matches!(wire, 0 | 2) => return facade_error("duplicate", path),
            1 | 2 => return facade_error("wire", path),
            _ => return facade_error("unknown", path),
        }
    }
    match (node, value) {
        (Some(node), Some(value)) => Ok(ChangeNodeNameMutation::Apply(GltfChangeNodeNamePayload { node, value })),
        (None, _) => facade_error("node", path),
        (_, None) => facade_error("nullable", path),
    }
}

fn protobuf_restore(reader: &mut FacadeProtobufReader<'_>, path: &str) -> FacadeResult<ChangeNodeNameMutation> {
    let mut node = None;
    let mut before = None;
    let mut after = None;
    while !reader.done() {
        let (field, wire) = reader.key(path)?;
        match field {
            1 if wire == 0 && node.is_none() => node = Some(u32::try_from(reader.varint(path)?).map_err(|_| GltfChangeNodeNameFacadeError { code: "node", path: path.to_string() })?),
            2 if wire == 2 && before.is_none() => {
                let mut nullable = reader.message(path)?;
                before = Some(protobuf_optional(&mut nullable, path)?);
            }
            3 if wire == 2 && after.is_none() => {
                let mut nullable = reader.message(path)?;
                after = Some(protobuf_optional(&mut nullable, path)?);
            }
            1 | 2 | 3 if matches!(wire, 0 | 2) => return facade_error("duplicate", path),
            1 | 2 | 3 => return facade_error("wire", path),
            _ => return facade_error("unknown", path),
        }
    }
    match (node, before, after) {
        (Some(node), Some(before), Some(after)) => Ok(ChangeNodeNameMutation::Restore(GltfChangeNodeNameRestore { node, before, after })),
        (None, _, _) => facade_error("node", path),
        (_, None, _) | (_, _, None) => facade_error("nullable", path),
    }
}

pub fn decode_gltf_change_node_name_protobuf(bytes: &[u8]) -> FacadeResult<ChangeNodeNameMutation> {
    let mut reader = FacadeProtobufReader::new(bytes);
    let (field, wire) = reader.key("protobuf.phase")?;
    if !matches!(field, 1 | 2) {
        return facade_error("phase", "protobuf.phase");
    }
    if wire != 2 {
        return facade_error("wire", "protobuf.phase");
    }
    let mut phase = reader.message("protobuf.phase")?;
    let mutation = match field {
        1 => protobuf_apply(&mut phase, "protobuf.apply")?,
        2 => protobuf_restore(&mut phase, "protobuf.restore")?,
        _ => unreachable!(),
    };
    if !phase.done() {
        return facade_error("duplicate", "protobuf.phase");
    }
    if !reader.done() {
        return facade_error("duplicate", "protobuf.phase");
    }
    Ok(mutation)
}
//#endregion 🔗️Facade

//#region ⚙️Validation
fn node_path(node: u32) -> String {
    format!("document/nodes/{node}/name")
}

fn node_index(node: u32, base: &GltfSnapshot) -> Result<usize, GltfTopLevelMutationRejection> {
    let index = usize::try_from(node).map_err(|_| reject("gltf.mutation.index-out-of-range", "document/nodes", format!("index {node} is not representable on this platform")))?;
    checked_index(index, base.document.nodes.len(), "document/nodes")?;
    Ok(index)
}

pub fn validate(payload: &GltfChangeNodeNamePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    let index = node_index(payload.node, base)?;
    if base.document.nodes[index].name == payload.value {
        return Err(reject("gltf.mutation.no-observable-change", node_path(payload.node), "name already has the requested presence and value"));
    }
    Ok(())
}

pub fn apply(payload: &GltfChangeNodeNamePayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> {
    validate(payload, base)?;
    let mut next = base.clone();
    next.document.nodes[node_index(payload.node, base)?].name = payload.value.clone();
    Ok(next)
}

pub fn validate_restore(restore: &GltfChangeNodeNameRestore, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    let index = node_index(restore.node, base)?;
    if base.document.nodes[index].name != restore.after {
        return Err(reject("gltf.mutation.stale-inverse", node_path(restore.node), "current name does not equal the inverse after witness"));
    }
    if restore.before == restore.after {
        return Err(reject("gltf.mutation.no-observable-change", node_path(restore.node), "inverse before and after witnesses are equal"));
    }
    Ok(())
}

pub fn apply_restore(restore: &GltfChangeNodeNameRestore, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> {
    validate_restore(restore, base)?;
    let mut next = base.clone();
    next.document.nodes[node_index(restore.node, base)?].name = restore.before.clone();
    Ok(next)
}
//#endregion ⚙️Validation

//#region 🧬️Operation
/// 🛡️ `deny_unknown_fields` here IS now enforced at this enum's own `{"phase": …, "value": …}`
/// level too (adjacently tagged: the outer object's keys are checked against `{"phase",
/// "value"}`), not just for the nested payload structs above — see `🌱️value/✨️derive`'s module
/// docs (`deny_unknown_fields` enum-container enforcement, ticket
/// `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(tag = "phase", content = "value", rename_all = "camelCase", deny_unknown_fields)]
pub enum ChangeNodeNameMutation {
    Apply(GltfChangeNodeNamePayload),
    Restore(GltfChangeNodeNameRestore),
}

fn rejection_outcome(code: String, path: String, detail: String) -> protocol::MutationOutcome<crate::artifacts::gltf::schema::diff::GltfDiff> {
    let target = path.split('/').filter(|part| !part.is_empty()).map(str::to_string).collect::<Vec<_>>();
    if code.contains("no-observable-change") {
        return protocol::MutationOutcome::new(Default::default()).warn("mutation.no-op", detail);
    }
    if code.contains("duplicate") {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", detail, target);
    }
    if code.contains("out-of-range") || code.contains("missing") || code.contains("not-found") {
        return protocol::MutationOutcome::error("mutation.target-missing", detail, target);
    }
    protocol::MutationOutcome::fatal("mutation.invariant", format!("{code}: {detail}"), target)
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for ChangeNodeNameMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node-name", kind: "change-node-name", record: "ChangedNodeName" };

    fn diff(&self, base: &GltfSnapshot) -> protocol::MutationOutcome<crate::artifacts::gltf::schema::diff::GltfDiff> {
        let next = match self {
            Self::Apply(payload) => apply(payload, base),
            Self::Restore(restore) => apply_restore(restore, base),
        };
        match next {
            Ok(next) => protocol::MutationOutcome::new(<crate::artifacts::gltf::schema::diff::GltfDiff as protocol::DiffAlgebra<GltfSnapshot>>::between(base, &next)),
            Err(error) => rejection_outcome(error.code, error.path, error.detail),
        }
    }

    fn inverse(&self, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
        let outcome = <Self as protocol::MutationKind<GltfSnapshot, super::GltfMutation>>::diff(self, base);
        if !outcome.messages().is_empty() || outcome.diff().is_empty_diff() {
            return Vec::new();
        }
        let inverse = match self {
            Self::Apply(payload) => Self::Restore(GltfChangeNodeNameRestore { node: payload.node, before: base.document.nodes[node_index(payload.node, base).expect("validated node")].name.clone(), after: payload.value.clone() }),
            Self::Restore(restore) => Self::Restore(GltfChangeNodeNameRestore { node: restore.node, before: restore.after.clone(), after: restore.before.clone() }),
        };
        vec![super::GltfMutation::ChangeNodeName(inverse)]
    }

    fn label(&self) -> String {
        "Change Node Name".to_string()
    }
    fn target(&self) -> Vec<String> {
        vec![node_path(match self {
            Self::Apply(payload) => payload.node,
            Self::Restore(restore) => restore.node,
        })]
    }
}
//#endregion 🧬️Operation

//#region 🧪️Tests
#[cfg(test)]
mod direct_leaf_tests {
    use super::*;
    use protocol::{Mutation, MutationDiff, MutationKind, MutationLeaf, OpBinary, OpText};

    fn value(entries: impl IntoIterator<Item = (&'static str, DslValue)>) -> DslValue {
        DslValue::object(entries.into_iter().map(|(key, value)| (key.to_string(), value)))
    }

    fn snapshot(name: Option<&str>) -> GltfSnapshot {
        let name = name.map(|value| format!(r#""name":{value:?}"#)).unwrap_or_default();
        serde_json::from_str(&format!(r#"{{"schema":"stdio.gltf","document":{{"asset":{{"version":"2.0"}},"nodes":[{{{}}}]}},"buffers":[],"sourceForm":"json"}}"#, name)).expect("minimal glTF snapshot decodes")
    }

    fn apply_mutation(base: &GltfSnapshot, mutation: &super::super::GltfMutation) -> GltfSnapshot {
        let outcome = <super::super::GltfMutation as Mutation<GltfSnapshot>>::diff(mutation, base);
        assert!(outcome.messages().is_empty(), "mutation must apply: {:?}", outcome.messages());
        <crate::artifacts::gltf::schema::diff::GltfDiff as MutationDiff<GltfSnapshot>>::apply(outcome.diff(), base).expect("aggregate diff applies")
    }

    #[test]
    fn canonical_leaf_metadata_matches_descriptor_and_provenance() {
        let expected: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).expect("valid canonical node-name descriptor");
        assert_eq!(serde_json::to_value(<ChangeNodeNameMutation as MutationLeaf>::DESCRIPTOR).expect("serializable descriptor"), expected);
        let provenance = <ChangeNodeNameMutation as MutationLeaf>::PROVENANCE;
        assert_eq!(provenance.mutation_root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
        assert_eq!(provenance.owner, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️🔘️change-node-name");
        assert_eq!(provenance.source_path, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️🔘️change-node-name/🦀️.rs");
        assert_eq!(provenance.descriptor_path, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️🔘️change-node-name/🔣️.json");
        assert_eq!(provenance.taxonomy_path, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");
        assert!(provenance.workspace_token.iter().any(|byte| *byte != 0));
    }

    #[test]
    fn concrete_inverse_round_trips_names_and_actual_aggregate_codecs() {
        let base = snapshot(Some("Root"));
        let mutation = super::super::GltfMutation::ChangeNodeName(ChangeNodeNameMutation::Apply(GltfChangeNodeNamePayload { node: 0, value: Some("Pivot".into()) }));
        let inverse = <super::super::GltfMutation as Mutation<GltfSnapshot>>::inverse(&mutation, &base);
        assert_eq!(inverse, vec![super::super::GltfMutation::ChangeNodeName(ChangeNodeNameMutation::Restore(GltfChangeNodeNameRestore { node: 0, before: Some("Root".into()), after: Some("Pivot".into()) }))]);
        let mut current = apply_mutation(&base, &mutation);
        assert_eq!(current.document.nodes[0].name.as_deref(), Some("Pivot"));
        assert_eq!(super::super::GltfMutation::parse_op(&mutation.print_op()).expect("text codec decodes"), mutation);
        assert_eq!(super::super::GltfMutation::decode_op(&mutation.encode_op().expect("binary codec encodes")).expect("binary codec decodes"), mutation);
        assert_eq!(super::super::GltfMutation::parse_op(&inverse[0].print_op()).expect("inverse text codec decodes"), inverse[0]);
        assert_eq!(super::super::GltfMutation::decode_op(&inverse[0].encode_op().expect("inverse binary codec encodes")).expect("inverse binary codec decodes"), inverse[0]);
        let redo = <super::super::GltfMutation as Mutation<GltfSnapshot>>::inverse(&inverse[0], &current);
        assert_eq!(redo, vec![super::super::GltfMutation::ChangeNodeName(ChangeNodeNameMutation::Restore(GltfChangeNodeNameRestore { node: 0, before: Some("Pivot".into()), after: Some("Root".into()) }))]);
        assert_eq!(super::super::GltfMutation::parse_op(&redo[0].print_op()).expect("redo text codec decodes"), redo[0]);
        assert_eq!(super::super::GltfMutation::decode_op(&redo[0].encode_op().expect("redo binary codec encodes")).expect("redo binary codec decodes"), redo[0]);
        current = apply_mutation(&current, &inverse[0]);
        assert_eq!(current, base);
        assert_eq!(apply_mutation(&current, &redo[0]), snapshot(Some("Pivot")));
        let stale_redo = <ChangeNodeNameMutation as MutationKind<GltfSnapshot, super::super::GltfMutation>>::diff(
            match &redo[0] {
                super::super::GltfMutation::ChangeNodeName(value) => value,
                _ => unreachable!(),
            },
            &snapshot(Some("Other")),
        );
        assert_eq!(stale_redo.messages()[0].code, protocol::FaultCode::new("mutation.invariant"));
    }

    #[test]
    fn none_normalization_stale_guards_and_noops_emit_no_inverse() {
        let absent = snapshot(None);
        assert_eq!(absent.document.nodes[0].name, None);
        let restore = ChangeNodeNameMutation::Restore(GltfChangeNodeNameRestore { node: 0, before: Some("Root".into()), after: None });
        let restored = apply_restore(
            match &restore {
                ChangeNodeNameMutation::Restore(value) => value,
                _ => unreachable!(),
            },
            &absent,
        )
        .expect("absent node matches null after witness");
        assert_eq!(restored.document.nodes[0].name.as_deref(), Some("Root"));
        assert!(<ChangeNodeNameMutation as MutationKind<GltfSnapshot, super::super::GltfMutation>>::inverse(&ChangeNodeNameMutation::Apply(GltfChangeNodeNamePayload { node: 0, value: None }), &absent).is_empty());
        assert!(<ChangeNodeNameMutation as MutationKind<GltfSnapshot, super::super::GltfMutation>>::inverse(&restore, &snapshot(Some("Other"))).is_empty());
        let stale = <ChangeNodeNameMutation as MutationKind<GltfSnapshot, super::super::GltfMutation>>::diff(&restore, &snapshot(Some("Pivot")));
        assert_eq!(stale.messages()[0].code, protocol::FaultCode::new("mutation.invariant"));
        let missing = <ChangeNodeNameMutation as MutationKind<GltfSnapshot, super::super::GltfMutation>>::diff(&ChangeNodeNameMutation::Apply(GltfChangeNodeNamePayload { node: 1, value: Some("Pivot".into()) }), &snapshot(Some("Root")));
        assert_eq!(missing.messages()[0].code, protocol::FaultCode::new("mutation.target-missing"));
    }

    #[test]
    fn restore_wire_requires_nullable_witnesses_and_excludes_document_diffs() {
        assert!(pack::from_json_str::<ChangeNodeNameMutation>(r#"{"phase":"restore","value":{"node":0,"after":"Pivot"}}"#).is_err());
        assert!(pack::from_json_str::<ChangeNodeNameMutation>(r#"{"phase":"apply","value":{"node":0}}"#).is_err());
        assert!(pack::from_json_str::<ChangeNodeNameMutation>(r#"{"phase":"restore","value":{"node":0,"before":null,"after":"Pivot","sourceForm":"glb"}}"#).is_err());
        let wire = pack::to_json_string(&ChangeNodeNameMutation::Restore(GltfChangeNodeNameRestore { node: 0, before: None, after: Some("Pivot".into()) }));
        assert_eq!(pack::parse_json(&wire).expect("restore encodes valid json"), pack::json!({ "phase": "restore", "value": { "node": 0, "before": null, "after": "Pivot" } }));
    }

    #[test]
    fn facade_decoders_construct_canonical_mutations_and_reject_malformed_boundaries() {
        let apply = ChangeNodeNameMutation::Apply(GltfChangeNodeNamePayload { node: 0, value: Some("Pivot".into()) });
        let present = value([("present", DslValue::String("Pivot".into()))]);
        let graphql = value([("apply", value([("node", DslValue::uint(0)), ("value", present.clone())]))]);
        assert_eq!(decode_gltf_change_node_name_graphql(&graphql).expect("graphql apply decodes"), apply);
        assert_eq!(decode_gltf_change_node_name_proto(&graphql).expect("proto apply decodes"), apply);
        assert_eq!(decode_gltf_change_node_name_protobuf(&[0x0a, 0x0b, 0x08, 0x00, 0x12, 0x07, 0x0a, 0x05, b'P', b'i', b'v', b'o', b't']).expect("protobuf apply decodes"), apply);
        let bom = ChangeNodeNameMutation::Apply(GltfChangeNodeNamePayload { node: 0, value: Some("\u{feff}Pivot".into()) });
        assert_eq!(decode_gltf_change_node_name_protobuf(&[0x0a, 0x0e, 0x08, 0x00, 0x12, 0x0a, 0x0a, 0x08, 0xef, 0xbb, 0xbf, b'P', b'i', b'v', b'o', b't']).expect("protobuf preserves bom"), bom);
        let false_absent = value([("apply", value([("node", DslValue::uint(0)), ("value", value([("absent", DslValue::Bool(false))]))]))]);
        assert_eq!(decode_gltf_change_node_name_graphql(&false_absent).expect_err("graphql false absent rejects").code, "nullable");
        let nonempty_absent = value([("apply", value([("node", DslValue::uint(0)), ("value", value([("absent", value([("extra", DslValue::Bool(true))]))]))]))]);
        assert_eq!(decode_gltf_change_node_name_proto(&nonempty_absent).expect_err("proto nonempty absent rejects").code, "unknown");
        let duplicate = DslValue::Object(vec![("apply".into(), value([("node", DslValue::uint(0)), ("value", present.clone())])), ("apply".into(), value([("node", DslValue::uint(0)), ("value", present)]))]);
        assert_eq!(decode_gltf_change_node_name_graphql(&duplicate).expect_err("duplicate graphql key rejects").code, "duplicate");
        assert_eq!(decode_gltf_change_node_name_protobuf(&[]).expect_err("protobuf missing phase rejects").code, "truncated");
        assert_eq!(decode_gltf_change_node_name_protobuf(&[0x18, 0x00]).expect_err("protobuf unknown phase rejects").code, "phase");
        assert_eq!(decode_gltf_change_node_name_protobuf(&[0x0a, 0x04, 0x08, 0x00, 0x08, 0x00]).expect_err("protobuf duplicate node rejects").code, "duplicate");
    }

    #[test]
    fn semantic_identity_matches_the_language_neutral_descriptor() {
        assert_eq!(<ChangeNodeNameMutation as MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "change-node-name");
        let mutation = ChangeNodeNameMutation::Apply(GltfChangeNodeNamePayload { node: 7, value: Some("Pivot".into()) });
        assert_eq!(mutation.target(), vec!["document/nodes/7/name"]);
    }
}
//#endregion 🧪️Tests
