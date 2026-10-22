//! 📐 Note mutation — `ChangeSnapGridSpacing`: sets the snap grid spacing.

use crate::{NoteDiff, NoteSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 📐 `change-snap-grid-spacing` payload — sets the snap grid spacing.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-snap-grid-spacing")]
pub struct ChangeSnapGridSpacing {
    pub new_spacing: Option<f64>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_snap_grid_spacing<Op>(new_spacing: Option<f64>) -> Op
where
    Op: From<crate::standards::v1::subsets::canvas::schema::mutations::change_snap_grid_spacing::ChangeSnapGridSpacing>,
{
    Op::from(ChangeSnapGridSpacing { new_spacing })
}

impl<Op> MutationKind<NoteSnapshot, Op> for ChangeSnapGridSpacing
where
    Op: protocol::Mutation<NoteSnapshot, Diff = NoteDiff> + From<ChangeSnapGridSpacing>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "snap-grid-spacing", kind: "change-snap-grid-spacing", record: "ChangedSnapGridSpacing" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        crate::schema::note_setting_label(("snap grid spacing", "Fangrasterabstand"), self.new_spacing.map(crate::schema::note_label_number))
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl<Op> protocol::ColdMutationKind<NoteSnapshot, Op> for ChangeSnapGridSpacing
where
    Op: protocol::ColdMutation<NoteSnapshot, ReferenceDiff = NoteDiff> + From<ChangeSnapGridSpacing>,
{
    fn diff(&self, base: &NoteSnapshot) -> protocol::MutationOutcome<NoteDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &NoteSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
