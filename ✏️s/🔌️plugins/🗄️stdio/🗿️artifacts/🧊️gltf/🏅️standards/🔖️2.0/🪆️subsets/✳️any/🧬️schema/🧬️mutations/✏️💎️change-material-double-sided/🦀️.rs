//! 🧬️ Direct change-material-double-sided mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::artifacts::gltf::schema::modules::mutation_support::material_animation::{index, GltfMaterialAnimationFailure};
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};
pub const ID: &str = "s.stdio.gltf.mutation.change-material-double-sided.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/materials/{material}/doubleSided"];
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn touched_paths(payload: &GltfChangeMaterialDoubleSidedPayload) -> Vec<String> {
    vec![format!("document/materials/{}/doubleSided", payload.material)]
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct GltfChangeMaterialDoubleSidedRejection {
    pub code: String,
    pub path: String,
    pub detail: String,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn failure(value: GltfMaterialAnimationFailure) -> GltfChangeMaterialDoubleSidedRejection {
    GltfChangeMaterialDoubleSidedRejection { code: value.code.into(), path: value.path, detail: value.detail.into() }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
mod tests {
    use super::*;
    #[semio_framework_async_macros::async_test]
    async fn applies_and_rejects_identity() {
        let mut snapshot = GltfSnapshot::default();
        snapshot.document.materials.push(Default::default());
        let payload = GltfChangeMaterialDoubleSidedPayload { material: 0, double_sided: true };
        apply(&mut snapshot, &payload).unwrap();
        assert!(snapshot.document.materials[0].double_sided);
        assert!(apply(&mut snapshot, &payload).is_err());
    }
}

//#region 🧬️DirectMutation
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ChangeMaterialDoubleSidedMutation {
    Apply(GltfChangeMaterialDoubleSidedPayload),
    Restore(crate::artifacts::gltf::schema::diff::GltfDiff),
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

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for ChangeMaterialDoubleSidedMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "material-double-sided", kind: "change-material-double-sided", record: "ChangedMaterialDoubleSided" };

    fn diff(&self, base: &GltfSnapshot) -> protocol::MutationOutcome<crate::artifacts::gltf::schema::diff::GltfDiff> {
        match self {
            Self::Apply(payload) => { let mut next = base.clone(); match apply(&mut next, payload) { Ok(()) => protocol::MutationOutcome::new(<crate::artifacts::gltf::schema::diff::GltfDiff as protocol::DiffAlgebra<GltfSnapshot>>::between(base, &next)), Err(error) => rejection_outcome(error.code, error.path, error.detail) } }
            Self::Restore(diff) => match protocol::MutationDiff::apply(diff, base) {
                Ok(_) => protocol::MutationOutcome::new(diff.clone()),
                Err(error) => protocol::MutationOutcome::fatal("mutation.invariant", error.to_string(), error.target),
            },
        }
    }

    fn inverse(&self, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
        let outcome = <Self as protocol::MutationKind<GltfSnapshot, super::GltfMutation>>::diff(self, base);
        if !outcome.messages().is_empty() || outcome.diff().is_empty_diff() {
            return Vec::new();
        }
        let inverse = <crate::artifacts::gltf::schema::diff::GltfDiff as protocol::DiffAlgebra<GltfSnapshot>>::inverse(outcome.diff(), base);
        vec![super::GltfMutation::ChangeMaterialDoubleSided(Self::Restore(inverse))]
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
mod direct_leaf_tests {
    use super::*;

    #[test]
    fn semantic_identity_matches_the_language_neutral_descriptor() {
        assert_eq!(<ChangeMaterialDoubleSidedMutation as protocol::MutationKind<GltfSnapshot, super::super::GltfMutation>>::SEMANTICS.kind, "change-material-double-sided");
    }
}
//#endregion 🧪️Tests
