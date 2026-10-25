//! 🧬️ Direct create-camera mutation owner: payload, validation, typed diff, inverse, and outcomes.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections::*;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::*;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.create-camera.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/cameras"];
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfCreateCameraPayload {
    pub position: usize,
    pub projection: GltfCameraProjection,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub camera: Option<Box<GltfCamera>>,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfCreateCameraPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if payload.position > base.document.cameras.len() {
        return Err(reject("gltf.mutation.insert-out-of-range", "document/cameras", "position must be within the collection"));
    }
    if let Some(record) = &payload.camera {
        if !(record.projection == payload.projection) {
            return Err(reject("gltf.mutation.record-mismatch", "document/cameras", "the record must carry the fields the payload names"));
        }
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfCreateCameraPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let mut diff = rewire(base, GltfTopLevelFamily::Cameras, &mut after_insert(p.position));
    diff.cameras.get_or_insert_with(Default::default).added.push(GltfAdded { index: p.position, item: p.camera.as_deref().cloned().unwrap_or_else(|| GltfCamera { projection: p.projection.clone(), name: None, extensions: None, extras: None }) });
    Ok(diff)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(p: &GltfCreateCameraPayload, base: &GltfSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::delete_camera::DeleteCameraMutation>,
{
    if validate(p, base).is_err() {
        return Vec::new();
    }
    vec![crate::standards::v2_0::subsets::any::schema::mutations::delete_camera::mutation(crate::standards::v2_0::subsets::any::schema::mutations::delete_camera::GltfDeleteCameraPayload { index: p.position })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum CreateCameraMutation {
    Apply(GltfCreateCameraPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation<Op>(payload: GltfCreateCameraPayload) -> Op
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::create_camera::CreateCameraMutation>,
{
    Op::from(CreateCameraMutation::Apply(payload))
}

impl<Op> protocol::MutationKind<GltfSnapshot, Op> for CreateCameraMutation
where
    Op: protocol::Mutation<GltfSnapshot, Diff = GltfDiff> + From<CreateCameraMutation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "camera", kind: "create-camera", record: "CreatedCamera" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Create Camera", "Kamera erstellen")
    }

    fn target(&self) -> Vec<String> {
        vec!["create-camera".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<GltfSnapshot, Op> for CreateCameraMutation
where
    Op: protocol::ColdMutation<GltfSnapshot, ReferenceDiff = GltfDiff> + From<CreateCameraMutation> + From<crate::standards::v2_0::subsets::any::schema::mutations::delete_camera::DeleteCameraMutation>,
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
