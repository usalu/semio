//! 🗂️ Saves, replaces or removes one of the user's named window layouts of one app.
use super::super::super::{UiPreferences, UiPreferencesDiff, UserNamedLayout};
use super::super::UiPreferencesConfigMutation;
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::HashMap;
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
            return protocol::MutationOutcome::new(UiPreferencesDiff(base.clone())).warn("mutation.no-op", "named layout already has the requested value.");
        }
        let mut next = base.clone();
        let mut app_layouts: HashMap<String, UserNamedLayout> = next.named_layouts.remove(&self.app_id).unwrap_or_default();
        match &self.layout {
            Some(layout) => {
                app_layouts.insert(self.layout_id.clone(), layout.clone());
            }
            None => {
                app_layouts.remove(&self.layout_id);
            }
        }
        if !app_layouts.is_empty() {
            next.named_layouts.insert(self.app_id.clone(), app_layouts);
        }
        protocol::MutationOutcome::new(UiPreferencesDiff(next))
    }
    fn inverse(&self, base: &UiPreferences) -> Vec<UiPreferencesConfigMutation> {
        vec![set_named_layout(self.app_id.clone(), self.layout_id.clone(), self.saved(base).cloned())]
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
