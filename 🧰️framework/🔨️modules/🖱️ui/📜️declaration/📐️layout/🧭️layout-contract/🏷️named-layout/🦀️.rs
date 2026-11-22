//! 🏷️ Named layouts: a catalogue entry wrapping one contract [`WindowLayout`] (the schema-owned window tree lives in `semio-ui-contract`, `presentation/layout`). See [`super`].

use super::*;

/// 🗂️ One selectable layout in the shell's layout catalogue.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct NamedLayout {
    pub id: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<IconName>,
    pub layout: WindowLayout,
    pub origin: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub group_path: Option<Vec<String>>,
}

pub fn create_named_layout(id: impl Into<String>, label: impl Into<String>, layout: WindowLayout, origin: impl Into<String>, icon_id: Option<IconName>, group_path: Option<Vec<String>>) -> NamedLayout {
    NamedLayout { id: id.into(), label: label.into(), icon_id, layout, origin: origin.into(), group_path }
}

pub fn merge_named_layouts(base: &[NamedLayout], extension: &[NamedLayout]) -> Vec<NamedLayout> {
    let mut merged: HashMap<String, NamedLayout> = HashMap::new();
    for entry in base {
        merged.insert(entry.id.clone(), entry.clone());
    }
    for entry in extension {
        merged.insert(entry.id.clone(), entry.clone());
    }
    merged.into_values().collect()
}
