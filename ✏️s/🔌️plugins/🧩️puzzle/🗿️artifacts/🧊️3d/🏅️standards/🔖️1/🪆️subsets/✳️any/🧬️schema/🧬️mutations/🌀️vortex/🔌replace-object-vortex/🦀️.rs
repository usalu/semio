//! 🔌 Puzzle3d mutation — `ReplaceObjectVortex`: whole-value swap of one vortex's presentation
//! fields (kind/label/position/direction/radius/hidden/locked together, one property-panel gesture).
use crate::standards::v1::subsets::any::schema::diff::Puzzle3dDiff;
use {crate::Puzzle3dSnapshot, crate::standards::v1::subsets::any::schema::Puzzle3dVortex};

//#region 🔖️Mutation
/// 🔌 `replace-object-vortex` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "replace-object-vortex")]
pub struct ReplaceObjectVortex {
    pub object_id: String,
    pub vortex_id: String,
    #[dsl(block)]
    pub new_vortex: Puzzle3dVortex,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_object_vortex<Op: From<ReplaceObjectVortex>>(object_id: String, vortex_id: String, new_vortex: Puzzle3dVortex) -> Op {
    Op::from(ReplaceObjectVortex { object_id, vortex_id, new_vortex })
}

impl<Op> protocol::MutationKind<Puzzle3dSnapshot, Op> for ReplaceObjectVortex
where
    Op: protocol::Mutation<Puzzle3dSnapshot, Diff = Puzzle3dDiff> + From<ReplaceObjectVortex>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "object-vortex", kind: "replace-object-vortex", record: "ReplacedObjectVortex" };

    fn diff(&self, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace vortex \"{}\" on object \"{}\"", self.vortex_id, self.object_id), &format!("Wirbel \"{}\" an Objekt \"{}\" ersetzen", self.vortex_id, self.object_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.object_id.clone(), self.vortex_id.clone()]
    }
}
//#endregion 🔖️Mutation
