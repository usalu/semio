//! 🧱 Remodeling mutation — `ReplaceMeshResult`: whole-value swap of the reconstructed (or
//! placeholder/imported) mesh. Boxed: `RemodelingMesh` is far larger than any sibling payload, and
//! `clippy::large_enum_variant` flags the resulting size disparity across `RemodelingMutation` otherwise.

use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingMesh;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🧱 `replace-mesh-result` payload.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "replace-mesh-result")]
pub struct ReplaceMeshResult {
    #[dsl(block)]
    pub mesh: Box<RemodelingMesh>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_mesh_result<Op: From<ReplaceMeshResult>>(mesh: Box<RemodelingMesh>) -> Op {
    Op::from(ReplaceMeshResult { mesh })
}

impl<Op> protocol::MutationKind<RemodelingSnapshot, Op> for ReplaceMeshResult
where
    Op: protocol::Mutation<RemodelingSnapshot, Diff = RemodelingDiff> + From<ReplaceMeshResult>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "mesh-result", kind: "replace-mesh-result", record: "ReplacedMeshResult" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace mesh result", "Netzergebnis ersetzen")
    }
}

impl<Op> protocol::ColdMutationKind<RemodelingSnapshot, Op> for ReplaceMeshResult
where
    Op: protocol::ColdMutation<RemodelingSnapshot, ReferenceDiff = RemodelingDiff> + From<ReplaceMeshResult>,
{
    fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
