//! 🧬️ Direct reorder-required-extensions mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.reorder-required-extensions.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/extensionsRequired"];
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfReorderRequiredExtensionsPayload {
    pub order: Vec<String>,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfReorderRequiredExtensionsPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if payload.order.len() != base.document.extensions_required.len()
        || payload.order.iter().collect::<std::collections::BTreeSet<_>>().len() != payload.order.len()
        || payload.order.iter().any(|value| !base.document.extensions_required.contains(value))
    {
        return Err(reject("gltf.mutation.invalid-permutation", "document/extensionsRequired", "order must contain every declaration exactly once"));
    }
    if payload.order == base.document.extensions_required {
        return Err(reject("gltf.mutation.no-observable-change", "document/extensionsRequired", "order already matches"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfReorderRequiredExtensionsPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    Ok(GltfDiff { extensions_required: Some(p.order.clone()), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfReorderRequiredExtensionsPayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    vec![super::reorder_required_extensions::mutation(super::reorder_required_extensions::GltfReorderRequiredExtensionsPayload { order: base.document.extensions_required.clone() })]
}

//#region 🧬️DirectMutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ReorderRequiredExtensionsMutation {
    Apply(GltfReorderRequiredExtensionsPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfReorderRequiredExtensionsPayload) -> super::GltfMutation {
    super::GltfMutation::ReorderRequiredExtensions(ReorderRequiredExtensionsMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for ReorderRequiredExtensionsMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "required-extensions", kind: "reorder-required-extensions", record: "ReorderedRequiredExtensions" };

    fn diff(&self, base: &GltfSnapshot) -> protocol::MutationOutcome<crate::schema::diff::GltfDiff> {
        match self {
            Self::Apply(payload) => match plan(payload, base) {
                Ok(diff) => protocol::MutationOutcome::new(diff),
                Err(error) => rejection_outcome(&error.code, &error.path, error.detail),
            },
        }
    }

    fn inverse(&self, base: &GltfSnapshot) -> Result<Vec<super::GltfMutation>, semio_framework_value::ValueError> {
        match self {
            Self::Apply(payload) => Ok(inverse(payload, base)),
        }
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Reorder Required Extensions", "Erforderliche Erweiterungen umordnen")
    }

    fn target(&self) -> Vec<String> {
        vec!["reorder-required-extensions".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔌️flips-the-two-c3ca1e/🦀️.rs"]
mod case_flips_the_two_c3ca1e;
//#endregion 🧪️Tests
