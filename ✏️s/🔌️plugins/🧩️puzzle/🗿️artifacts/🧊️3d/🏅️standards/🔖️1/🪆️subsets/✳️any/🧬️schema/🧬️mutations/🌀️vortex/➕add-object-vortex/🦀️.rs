//! ➕ Puzzle3d mutation — `AddObjectVortex`: attaches a new rim vortex to an object.
use crate::standards::v1::subsets::any::schema::mutations::RemoveObjectVortex;
use crate::standards::v1::subsets::any::schema::diff::Puzzle3dDiff;
use {crate::Puzzle3dSnapshot, crate::standards::v1::subsets::any::schema::Puzzle3dVortex};

//#region 🔖️Mutation
/// ➕ `add-object-vortex` payload — owner object id + new vortex payload at an optional
/// FINAL-state `index` (`None` appends). A duplicate `vortex.id` on the same object is a no-op.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "add-object-vortex")]
pub struct AddObjectVortex {
    pub object_id: String,
    #[dsl(block)]
    pub vortex: Puzzle3dVortex,
    pub index: Option<usize>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn add_object_vortex<Op: From<AddObjectVortex>>(object_id: String, vortex: Puzzle3dVortex, index: Option<usize>) -> Op {
    Op::from(AddObjectVortex { object_id, vortex, index })
}

impl<Op> protocol::MutationKind<Puzzle3dSnapshot, Op> for AddObjectVortex
where
    Op: protocol::Mutation<Puzzle3dSnapshot, Diff = Puzzle3dDiff> + From<AddObjectVortex> + From<RemoveObjectVortex>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "object-vortex", kind: "add-object-vortex", record: "AddedObjectVortex" };

    fn diff(&self, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Add vortex \"{}\" to object \"{}\"", self.vortex.id, self.object_id), &format!("Wirbel \"{}\" zu Objekt \"{}\" hinzufügen", self.vortex.id, self.object_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.object_id.clone(), self.vortex.id.clone()]
    }
}
//#endregion 🔖️Mutation
