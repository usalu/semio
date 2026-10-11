//! 🧱 Remodeling mutation — `ReplaceMeshResult`: whole-value swap of the reconstructed (or
//! placeholder/imported) mesh. Boxed: `RemodelingMesh` is far larger than any sibling payload, and
//! `clippy::large_enum_variant` flags the resulting size disparity across `RemodelingMutation` otherwise.

use crate::diff::RemodelingDiff;
use crate::mutations::RemodelingMutation;
use crate::{RemodelingMesh, RemodelingSnapshot};
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
pub fn replace_mesh_result(mesh: Box<RemodelingMesh>) -> RemodelingMutation {
    RemodelingMutation::ReplaceMeshResult(ReplaceMeshResult { mesh })
}

impl protocol::MutationKind<RemodelingSnapshot, RemodelingMutation> for ReplaceMeshResult {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "mesh-result", kind: "replace-mesh-result", record: "ReplacedMeshResult" };

    fn diff(&self, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RemodelingSnapshot) -> Result<Vec<RemodelingMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace mesh result", "Netzergebnis ersetzen")
    }
}
//#endregion 🔖️Mutation
