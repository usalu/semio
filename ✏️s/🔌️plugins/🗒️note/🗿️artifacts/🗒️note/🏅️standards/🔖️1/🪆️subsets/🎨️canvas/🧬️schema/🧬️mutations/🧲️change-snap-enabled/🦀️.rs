//! 🧲 Note mutation — `ChangeSnapEnabled`: sets snap-to-grid enabled.

use crate::{NoteDiff, NoteSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🧲 `change-snap-enabled` payload — sets snap-to-grid enabled.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-snap-enabled")]
pub struct ChangeSnapEnabled {
    pub new_enabled: Option<bool>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_snap_enabled<Op>(new_enabled: Option<bool>) -> Op
where
    Op: From<crate::standards::v1::subsets::canvas::schema::mutations::change_snap_enabled::ChangeSnapEnabled>,
{
    Op::from(ChangeSnapEnabled { new_enabled })
}

impl<Op> MutationKind<NoteSnapshot, Op> for ChangeSnapEnabled
where
    Op: protocol::Mutation<NoteSnapshot, Diff = NoteDiff> + From<ChangeSnapEnabled>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "snap-enabled", kind: "change-snap-enabled", record: "ChangedSnapEnabled" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        match self.new_enabled {
            Some(true) => semio_framework_ui_locale::LocalizedLabel::native("Enable snapping", "Fangfunktion einschalten"),
            Some(false) => semio_framework_ui_locale::LocalizedLabel::native("Disable snapping", "Fangfunktion ausschalten"),
            None => crate::schema::note_setting_label(("snapping", "Fangfunktion"), None),
        }
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl<Op> protocol::ColdMutationKind<NoteSnapshot, Op> for ChangeSnapEnabled
where
    Op: protocol::ColdMutation<NoteSnapshot, ReferenceDiff = NoteDiff> + From<ChangeSnapEnabled>,
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
