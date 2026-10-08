//! 🗂️ Saves, replaces or removes one of the user's named window layouts of one app.
use super::super::super::{NamedLayoutEdit, UiPreferences, UiPreferencesDiff, UserNamedLayout};
use super::super::UiPreferencesConfigMutation;
use semio_framework_value_derive::{FromValue, ToValue};
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetNamedLayout {
    pub app_id: String,
    pub layout_id: String,
    pub layout: Option<UserNamedLayout>,
}
pub fn set_named_layout(app_id: impl Into<String>, layout_id: impl Into<String>, layout: Option<UserNamedLayout>) -> UiPreferencesConfigMutation {
    UiPreferencesConfigMutation::SetNamedLayout(SetNamedLayout { app_id: app_id.into(), layout_id: layout_id.into(), layout })
}

impl SetNamedLayout {
    fn saved<'a>(&self, base: &'a UiPreferences) -> Option<&'a UserNamedLayout> {
        base.named_layouts.get(&self.app_id).and_then(|layouts| layouts.get(&self.layout_id))
    }
}

impl protocol::MutationKind<UiPreferences, UiPreferencesConfigMutation> for SetNamedLayout {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "named-layout", kind: "set-named-layout", record: "Set" };
    fn diff(&self, base: &UiPreferences) -> protocol::MutationOutcome<UiPreferencesDiff> {
        if self.saved(base) == self.layout.as_ref() {
            return protocol::MutationOutcome::new(UiPreferencesDiff::default()).warning("mutation.no-op", "named layout already has the requested value.");
        }
        protocol::MutationOutcome::new(UiPreferencesDiff { named_layouts: vec![NamedLayoutEdit { app_id: self.app_id.clone(), layout_id: self.layout_id.clone(), value: self.layout.clone() }], ..UiPreferencesDiff::default() })
    }
    fn inverse(&self, base: &UiPreferences) -> Result<Vec<UiPreferencesConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![set_named_layout(self.app_id.clone(), self.layout_id.clone(), self.saved(base).cloned())]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        match self.layout {
            Some(_) => semio_framework_ui_locale::LocalizedLabel::native(&format!("Save layout {:?}", self.layout_id), &format!("Layout {:?} speichern", self.layout_id)),
            None => semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove layout {:?}", self.layout_id), &format!("Layout {:?} entfernen", self.layout_id)),
        }
    }
    fn target(&self) -> Vec<String> {
        vec!["named-layouts".to_string(), self.app_id.clone(), self.layout_id.clone()]
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/✏️sets-named-layout/🦀️.rs"]
mod tests_sets_named_layout;

#[cfg(test)]
#[path = "🧪️tests/🟰️keeps-named-layout/🦀️.rs"]
mod tests_keeps_named_layout;
//#endregion 🧪️Tests
