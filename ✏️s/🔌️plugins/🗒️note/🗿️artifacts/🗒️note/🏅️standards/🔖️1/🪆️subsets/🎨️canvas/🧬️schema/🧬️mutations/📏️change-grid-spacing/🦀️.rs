//! 📏 Note mutation — `ChangeGridSpacing`: sets grid spacing.

use crate::{NoteDiff, NoteSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 📏 `change-grid-spacing` payload — sets grid spacing.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-grid-spacing")]
pub struct ChangeGridSpacing {
    pub new_spacing: Option<f64>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_grid_spacing<Op>(new_spacing: Option<f64>) -> Op
where
    Op: From<crate::standards::v1::subsets::canvas::schema::mutations::change_grid_spacing::ChangeGridSpacing>,
{
    Op::from(ChangeGridSpacing { new_spacing })
}

impl<Op> MutationKind<NoteSnapshot, Op> for ChangeGridSpacing
where
    Op: protocol::Mutation<NoteSnapshot, Diff = NoteDiff> + From<ChangeGridSpacing>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "grid-spacing", kind: "change-grid-spacing", record: "ChangedGridSpacing" };

    fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        crate::schema::note_setting_label(("grid spacing", "Rasterabstand"), self.new_spacing.map(crate::schema::note_label_number))
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Mutation
