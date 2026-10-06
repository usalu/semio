//! 🧬️ Direct change-node-name mutation owner.
use crate::schema::modules::mutation_support::structure_geometry::checked_index;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use crate::GltfSnapshot;

//#region 🔖️Payload
pub const ID: &str = "s.stdio.gltf.mutation.change-node-name.v1";

/// 🕳️ `value`/`before`/`after` carry `#[value(required)]` (the `serde` equivalent this leaf once
/// hand-rolled as `deserialize_with = "required_option"`): this derive, like `serde`'s, decodes a
/// MISSING `Option<T>` key as `None` unless the field says otherwise, and `required` is exactly
/// the opt-out — a present wire key is mandatory even for an `Option<T>`. That is what makes a
/// rename's nullable witnesses honest: `{"before": null}` (the name genuinely was absent) and a
/// wire that simply forgot to record `before` must not decode to the same value. A present `null`
/// still decodes as `None` through the blanket `impl<T: FromValue> FromValue for Option<T>`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GltfChangeNodeNamePayload {
    pub node: u32,
    #[value(required)]
    pub value: Option<String>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GltfChangeNodeNameRestore {
    pub node: u32,
    #[value(required)]
    pub before: Option<String>,
    #[value(required)]
    pub after: Option<String>,
}
//#endregion 🔖️Payload



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
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase", deny_unknown_fields)]
pub enum ChangeNodeNameMutation {
    Apply(GltfChangeNodeNamePayload),
    Restore(GltfChangeNodeNameRestore),
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for ChangeNodeNameMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node-name", kind: "change-node-name", record: "ChangedNodeName" };

    fn diff(&self, base: &GltfSnapshot) -> protocol::MutationOutcome<crate::schema::diff::GltfDiff> {
        let next = match self {
            Self::Apply(payload) => apply(payload, base),
            Self::Restore(restore) => apply_restore(restore, base),
        };
        match next {
            Ok(next) => protocol::MutationOutcome::new(<crate::schema::diff::GltfDiff as protocol::DiffAlgebra<GltfSnapshot>>::between(base, &next)),
            Err(error) => rejection_outcome(&error.code, &error.path, error.detail),
        }
    }

    fn inverse(&self, base: &GltfSnapshot) -> Result<Vec<super::GltfMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        let outcome = <Self as protocol::MutationKind<GltfSnapshot, super::GltfMutation>>::diff(self, base);
        if !outcome.messages().is_empty() || outcome.diff().is_empty_diff() {
            return Vec::new();
        }
        let inverse = match self {
            Self::Apply(payload) => Self::Restore(GltfChangeNodeNameRestore { node: payload.node, before: base.document.nodes[node_index(payload.node, base).expect("validated node")].name.clone(), after: payload.value.clone() }),
            Self::Restore(restore) => Self::Restore(GltfChangeNodeNameRestore { node: restore.node, before: restore.after.clone(), after: restore.before.clone() }),
        };
        vec![super::GltfMutation::ChangeNodeName(inverse)]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change Node Name", "Knotenname ändern")
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
#[path = "🧪️tests/✏️renames-the-root-f1e002/🦀️.rs"]
mod case_renames_the_root_f1e002;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "📜️contract/🧪️tests/🔬️unit/🦀️.rs"]
mod contract;
