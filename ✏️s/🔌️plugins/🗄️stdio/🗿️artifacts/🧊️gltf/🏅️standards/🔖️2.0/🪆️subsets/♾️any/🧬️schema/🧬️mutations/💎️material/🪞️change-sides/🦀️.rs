//! 🧬️ Direct change-material-double-sided mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::modules::mutation_support::material_animation::{index, GltfMaterialAnimationFailure};
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.change-material-double-sided.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/materials/{material}/doubleSided"];
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn touched_paths(payload: &GltfChangeMaterialDoubleSidedPayload) -> Vec<String> {
    vec![format!("document/materials/{}/doubleSided", payload.material)]
}
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct GltfChangeMaterialDoubleSidedRejection {
    pub code: String,
    pub path: String,
    pub detail: String,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn failure(value: GltfMaterialAnimationFailure) -> GltfChangeMaterialDoubleSidedRejection {
    GltfChangeMaterialDoubleSidedRejection { code: value.code.into(), path: value.path, detail: value.detail.into() }
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfChangeMaterialDoubleSidedPayload {
    pub material: usize,
    pub double_sided: bool,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfChangeMaterialDoubleSidedPayload, base: &GltfSnapshot) -> Result<(), GltfChangeMaterialDoubleSidedRejection> {
    index(&base.document.materials, payload.material, "document/materials").map_err(failure)?;
    (base.document.materials[payload.material].double_sided != payload.double_sided).then_some(()).ok_or_else(|| GltfChangeMaterialDoubleSidedRejection {
        code: "gltf.mutation.no-observable-change".into(),
        path: format!("document/materials/{}/doubleSided", payload.material),
        detail: "doubleSided already has that value".into(),
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(snapshot: &mut GltfSnapshot, payload: &GltfChangeMaterialDoubleSidedPayload) -> Result<(), GltfChangeMaterialDoubleSidedRejection> {
    validate(payload, snapshot)?;
    snapshot.document.materials[payload.material].double_sided = payload.double_sided;
    Ok(())
}
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#region 🧬️DirectMutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ChangeMaterialDoubleSidedMutation {
    Apply(GltfChangeMaterialDoubleSidedPayload),
    Restore(Box<crate::schema::diff::GltfDiff>),
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for ChangeMaterialDoubleSidedMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "material-double-sided", kind: "change-material-double-sided", record: "ChangedMaterialDoubleSided" };

    fn diff(&self, base: &GltfSnapshot) -> protocol::MutationOutcome<crate::schema::diff::GltfDiff> {
        match self {
            Self::Apply(payload) => {
                let mut next = base.clone();
                match apply(&mut next, payload) {
                    Ok(()) => protocol::MutationOutcome::new(<crate::schema::diff::GltfDiff as protocol::DiffAlgebra<GltfSnapshot>>::between(base, &next)),
                    Err(error) => rejection_outcome(&error.code, &error.path, error.detail),
                }
            }
            Self::Restore(diff) => match protocol::MutationDiff::apply(diff.as_ref(), base) {
                Ok(_) => protocol::MutationOutcome::new(diff.as_ref().clone()),
                Err(error) => protocol::MutationOutcome::fatal("mutation.invariant", error.to_string(), error.target),
            },
        }
    }

    fn inverse(&self, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
        let outcome = <Self as protocol::MutationKind<GltfSnapshot, super::GltfMutation>>::diff(self, base);
        if !outcome.messages().is_empty() || outcome.diff().is_empty_diff() {
            return Vec::new();
        }
        let inverse = <crate::schema::diff::GltfDiff as protocol::DiffAlgebra<GltfSnapshot>>::inverse(outcome.diff(), base);
        vec![super::GltfMutation::ChangeMaterialDoubleSided(Self::Restore(Box::new(inverse)))]
    }

    fn label(&self) -> String {
        "Change Material Double Sided".to_string()
    }

    fn target(&self) -> Vec<String> {
        vec!["change-material-double-sided".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️direct-leaf/🦀️.rs"]
mod direct_leaf_tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "📜️contract/🧪️tests/🔬️unit/🦀️.rs"]
mod contract;
