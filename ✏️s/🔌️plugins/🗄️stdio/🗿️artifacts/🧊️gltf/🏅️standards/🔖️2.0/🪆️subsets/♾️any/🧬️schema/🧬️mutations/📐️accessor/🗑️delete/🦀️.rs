//! 🧬️ Direct delete-accessor mutation owner: payload, validation, typed diff, inverse, and outcomes.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections::*;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.delete-accessor.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/accessors"];
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfDeleteAccessorPayload {
    pub index: usize,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfDeleteAccessorPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if payload.index >= base.document.accessors.len() {
        return Err(reject("gltf.mutation.index-out-of-range", "document/accessors", "index must address an item"));
    }
    require_unreferenced(base, GltfTopLevelFamily::Accessors, payload.index)?;
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfDeleteAccessorPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let mut diff = rewire(base, GltfTopLevelFamily::Accessors, &mut after_delete(p.index));
    let slot = diff.accessors.get_or_insert_with(Default::default);
    slot.removed.push(p.index);
    Ok(diff)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(p: &GltfDeleteAccessorPayload, base: &GltfSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::remove_used_extension::RemoveUsedExtensionMutation>,
{
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let mut rows = vec![crate::standards::v2_0::subsets::any::schema::mutations::create_accessor::mutation(crate::standards::v2_0::subsets::any::schema::mutations::create_accessor::GltfCreateAccessorPayload { position: p.index, component_type: base.document.accessors[p.index].component_type, count: base.document.accessors[p.index].count, kind: base.document.accessors[p.index].kind, accessor: Some(Box::new(base.document.accessors[p.index].clone())) })];
    for (mesh, entry) in base.document.meshes.iter().enumerate() {
        for (primitive, item) in entry.primitives.iter().enumerate() {
            if item.indices == Some(p.index) {
                rows.push(crate::standards::v2_0::subsets::any::schema::mutations::bind_primitive_indices::mutation(crate::standards::v2_0::subsets::any::schema::mutations::bind_primitive_indices::GltfBindPrimitiveIndicesPayload { mesh, primitive, accessor: p.index }));
            }
            let dropped: Vec<usize> = item.attributes.iter().enumerate().filter(|(_, (_, accessor))| *accessor == p.index).map(|(position, _)| position).collect();
            let kept = item.attributes.len() - dropped.len();
            for (offset, position) in dropped.iter().enumerate() {
                rows.push(crate::standards::v2_0::subsets::any::schema::mutations::bind_primitive_attribute::mutation(crate::standards::v2_0::subsets::any::schema::mutations::bind_primitive_attribute::GltfBindPrimitiveAttributePayload { mesh, primitive, semantic: item.attributes[*position].0.clone(), accessor: p.index }));
                if kept + offset != *position {
                    rows.push(crate::standards::v2_0::subsets::any::schema::mutations::move_primitive_attribute::mutation(crate::standards::v2_0::subsets::any::schema::mutations::move_primitive_attribute::GltfMovePrimitiveAttributePayload { mesh, primitive, semantic: item.attributes[*position].0.clone(), position: *position }));
                }
            }
            for (target, morph) in item.targets.iter().enumerate() {
                let dropped: Vec<usize> = morph.0.iter().enumerate().filter(|(_, (_, accessor))| *accessor == p.index).map(|(position, _)| position).collect();
                let kept = morph.0.len() - dropped.len();
                for (offset, position) in dropped.iter().enumerate() {
                    rows.push(crate::standards::v2_0::subsets::any::schema::mutations::bind_morph_target_attribute::mutation(crate::standards::v2_0::subsets::any::schema::mutations::bind_morph_target_attribute::GltfBindMorphTargetAttributePayload { mesh, primitive, target, semantic: morph.0[*position].0.clone(), accessor: p.index }));
                    if kept + offset != *position {
                        rows.push(crate::standards::v2_0::subsets::any::schema::mutations::move_morph_target_attribute::mutation(crate::standards::v2_0::subsets::any::schema::mutations::move_morph_target_attribute::GltfMoveMorphTargetAttributePayload { mesh, primitive, target, semantic: morph.0[*position].0.clone(), position: *position }));
                    }
                }
            }
        }
    }
    rows.reverse();
    rows
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum DeleteAccessorMutation {
    Apply(GltfDeleteAccessorPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation<Op>(payload: GltfDeleteAccessorPayload) -> Op
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::delete_accessor::DeleteAccessorMutation>,
{
    Op::from(DeleteAccessorMutation::Apply(payload))
}

impl<Op> protocol::MutationKind<GltfSnapshot, Op> for DeleteAccessorMutation
where
    Op: protocol::Mutation<GltfSnapshot, Diff = GltfDiff> + From<DeleteAccessorMutation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "accessor", kind: "delete-accessor", record: "DeletedAccessor" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Delete Accessor", "Accessor löschen")
    }

    fn target(&self) -> Vec<String> {
        vec!["delete-accessor".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<GltfSnapshot, Op> for DeleteAccessorMutation
where
    Op: protocol::ColdMutation<GltfSnapshot, ReferenceDiff = GltfDiff> + From<DeleteAccessorMutation>,
{
    fn diff(&self, base: &GltfSnapshot) -> protocol::MutationOutcome<GltfDiff> {
        match self {
            Self::Apply(payload) => match plan(payload, base) {
                Ok(diff) => protocol::MutationOutcome::new(diff),
                Err(error) => rejection_outcome(&error.code, &error.path, error.detail),
            },
        }
    }
    fn inverse(&self, base: &GltfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        match self {
            Self::Apply(payload) => Ok(inverse(payload, base)),
        }
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests


//#endregion 🧪️Tests
