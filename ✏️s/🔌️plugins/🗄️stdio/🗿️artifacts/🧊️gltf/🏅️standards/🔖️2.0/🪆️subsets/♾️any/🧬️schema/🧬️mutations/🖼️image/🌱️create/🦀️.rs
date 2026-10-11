//! 🧬️ Direct create-image mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::snapshot::*;
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.create-image.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/images"];
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfCreateImagePayload {
    pub position: usize,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<Box<GltfImage>>,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfCreateImagePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if payload.position > base.document.images.len() {
        return Err(reject("gltf.mutation.insert-out-of-range", "document/images", "position must be within the collection"));
    }
    let lengths = GltfLengths::after_insert(base, GltfTopLevelFamily::Images);
    if let Some(record) = &payload.image {
        check_image(record, &lengths)?;
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfCreateImagePayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let mut diff = rewire(base, GltfTopLevelFamily::Images, &mut after_insert(p.position));
    diff.images.get_or_insert_with(Default::default).added.push(GltfAdded { index: p.position, item: p.image.as_deref().cloned().unwrap_or_else(|| GltfImage::default()) });
    Ok(diff)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfCreateImagePayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    vec![super::delete_image::mutation(super::delete_image::GltfDeleteImagePayload { index: p.position })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum CreateImageMutation {
    Apply(GltfCreateImagePayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfCreateImagePayload) -> super::GltfMutation {
    super::GltfMutation::CreateImage(CreateImageMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for CreateImageMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "image", kind: "create-image", record: "CreatedImage" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Create Image", "Bild erstellen")
    }

    fn target(&self) -> Vec<String> {
        vec!["create-image".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🖼️inserts-an-empty-e3b07a/🦀️.rs"]
mod case_inserts_an_empty_e3b07a;
//#endregion 🧪️Tests
