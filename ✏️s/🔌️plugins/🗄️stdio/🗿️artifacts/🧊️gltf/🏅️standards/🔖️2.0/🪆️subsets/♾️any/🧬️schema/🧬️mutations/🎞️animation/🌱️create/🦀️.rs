//! 🧬️ Direct create-animation mutation owner: payload, validation, typed diff, inverse, and outcomes.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections::*;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::*;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.create-animation.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/animations"];
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfCreateAnimationPayload {
    pub position: usize,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub animation: Option<Box<GltfAnimation>>,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfCreateAnimationPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if payload.position > base.document.animations.len() {
        return Err(reject("gltf.mutation.insert-out-of-range", "document/animations", "position must be within the collection"));
    }
    let lengths = GltfLengths::after_insert(base, GltfTopLevelFamily::Animations);
    if let Some(record) = &payload.animation {
        check_animation(record, &lengths)?;
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfCreateAnimationPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let mut diff = rewire(base, GltfTopLevelFamily::Animations, &mut after_insert(p.position));
    diff.animations.get_or_insert_with(Default::default).added.push(GltfAdded { index: p.position, item: p.animation.as_deref().cloned().unwrap_or_else(|| GltfAnimation::default()) });
    Ok(diff)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(p: &GltfCreateAnimationPayload, base: &GltfSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::remove_used_extension::RemoveUsedExtensionMutation>,
{
    if validate(p, base).is_err() {
        return Vec::new();
    }
    vec![crate::standards::v2_0::subsets::any::schema::mutations::delete_animation::mutation(crate::standards::v2_0::subsets::any::schema::mutations::delete_animation::GltfDeleteAnimationPayload { index: p.position })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum CreateAnimationMutation {
    Apply(GltfCreateAnimationPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation<Op>(payload: GltfCreateAnimationPayload) -> Op
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::create_animation::CreateAnimationMutation>,
{
    Op::from(CreateAnimationMutation::Apply(payload))
}

impl<Op> protocol::MutationKind<GltfSnapshot, Op> for CreateAnimationMutation
where
    Op: protocol::Mutation<GltfSnapshot, Diff = GltfDiff> + From<CreateAnimationMutation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "animation", kind: "create-animation", record: "CreatedAnimation" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Create Animation", "Animation erstellen")
    }

    fn target(&self) -> Vec<String> {
        vec!["create-animation".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<GltfSnapshot, Op> for CreateAnimationMutation
where
    Op: protocol::ColdMutation<GltfSnapshot, ReferenceDiff = GltfDiff> + From<CreateAnimationMutation>,
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
