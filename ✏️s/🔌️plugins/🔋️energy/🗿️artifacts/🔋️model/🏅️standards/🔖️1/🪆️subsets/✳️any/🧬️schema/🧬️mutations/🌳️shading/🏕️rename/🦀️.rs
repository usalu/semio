//! 🏕️ Energy model mutation — `RenameShadingSurface`: Sets one shading surface's identity field; a name another shading surface already holds is refused.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🏕️ `rename-shading-surface` payload. Sets one shading surface's identity field; a name another shading surface already holds is refused.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "rename-shading-surface")]
pub struct RenameShadingSurface {
    pub id: crate::model::EntityId,
    pub new_name: String,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn rename_shading_surface<Op: From<RenameShadingSurface>>(id: crate::model::EntityId, new_name: String) -> Op {
    RenameShadingSurface { id, new_name }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for RenameShadingSurface
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<RenameShadingSurface>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "shading-surface", kind: "rename-shading-surface", record: "RenamedShadingSurface" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rename shading surface {} to \"{}\"", self.id.0, self.new_name), &format!("Verschattungsfläche {} in \"{}\" umbenennen", self.id.0, self.new_name))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
