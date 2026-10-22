//! 🧬️ Direct move-animation mutation owner: payload, validation, typed diff, inverse, and outcomes.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections::*;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.move-animation.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/animations"];
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfMoveAnimationPayload {
    pub index: usize,
    pub position: usize,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfMoveAnimationPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if payload.index >= base.document.animations.len() || payload.position >= base.document.animations.len() {
        return Err(reject("gltf.mutation.index-out-of-range", "document/animations", "indices must address items"));
    }
    if payload.index == payload.position {
        return Err(reject("gltf.mutation.no-observable-change", "document/animations", "destination equals source"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfMoveAnimationPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let mut diff = rewire(base, GltfTopLevelFamily::Animations, &mut after_move(p.index, p.position));
    let slot = diff.animations.get_or_insert_with(Default::default);
    slot.removed.push(p.index);
    slot.added.push(GltfAdded { index: p.position, item: base.document.animations[p.index].clone() });
    Ok(diff)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(p: &GltfMoveAnimationPayload, base: &GltfSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::remove_used_extension::RemoveUsedExtensionMutation>,
{
    if validate(p, base).is_err() {
        return Vec::new();
    }
    vec![mutation(GltfMoveAnimationPayload { index: p.position, position: p.index })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum MoveAnimationMutation {
    Apply(GltfMoveAnimationPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation<Op>(payload: GltfMoveAnimationPayload) -> Op
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::move_animation::MoveAnimationMutation>,
{
    Op::from(MoveAnimationMutation::Apply(payload))
}

impl<Op> protocol::MutationKind<GltfSnapshot, Op> for MoveAnimationMutation
where
    Op: protocol::Mutation<GltfSnapshot, Diff = GltfDiff> + From<MoveAnimationMutation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "animation", kind: "move-animation", record: "MovedAnimation" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Move Animation", "Animation verschieben")
    }

    fn target(&self) -> Vec<String> {
        vec!["move-animation".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<GltfSnapshot, Op> for MoveAnimationMutation
where
    Op: protocol::ColdMutation<GltfSnapshot, ReferenceDiff = GltfDiff> + From<MoveAnimationMutation>,
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
