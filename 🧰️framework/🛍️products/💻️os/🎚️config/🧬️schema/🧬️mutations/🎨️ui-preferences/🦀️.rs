//! 🎨️ Cohesive module for the direct OS UI-preference mutation leaves.

macro_rules! optional_setting_impl {
    ($payload:ty, $variant:ident, $field:ident, $kind:literal, $entity:literal, $label:literal, $target:literal) => {
        impl protocol::MutationKind<UiPreferences, UiPreferencesConfigMutation> for $payload {
            const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: $entity, kind: $kind, record: "Set" };
            fn diff(&self, base: &UiPreferences) -> protocol::MutationOutcome<UiPreferencesDiff> {
                if base.$field == self.$field {
                    return protocol::MutationOutcome::new(UiPreferencesDiff(base.clone())).warn("mutation.no-op", concat!($label, " is already selected."));
                }
                let mut next = base.clone();
                next.$field = self.$field.clone();
                protocol::MutationOutcome::new(UiPreferencesDiff(next))
            }
            fn inverse(&self, base: &UiPreferences) -> Vec<UiPreferencesConfigMutation> {
                vec![UiPreferencesConfigMutation::$variant(Self { $field: base.$field.clone() })]
            }
            fn label(&self) -> String {
                format!(concat!("Set ", $label, " to {:?}"), self.$field)
            }
            fn target(&self) -> Vec<String> {
                vec![$target.to_string()]
            }
        }
    };
}

macro_rules! keyed_setting_impl {
    ($payload:ty, $variant:ident, $map:ident, $id:ident, $value:ident, $kind:literal, $entity:literal, $label:literal, $target:literal) => {
        impl protocol::MutationKind<UiPreferences, UiPreferencesConfigMutation> for $payload {
            const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: $entity, kind: $kind, record: "Set" };
            fn diff(&self, base: &UiPreferences) -> protocol::MutationOutcome<UiPreferencesDiff> {
                if base.$map.get(&self.$id) == self.$value.as_ref() {
                    return protocol::MutationOutcome::new(UiPreferencesDiff(base.clone())).warn("mutation.no-op", concat!($label, " already has the requested value."));
                }
                let mut next = base.clone();
                match &self.$value {
                    Some(value) => {
                        next.$map.insert(self.$id.clone(), value.clone());
                    }
                    None => {
                        next.$map.remove(&self.$id);
                    }
                }
                protocol::MutationOutcome::new(UiPreferencesDiff(next))
            }
            fn inverse(&self, base: &UiPreferences) -> Vec<UiPreferencesConfigMutation> {
                vec![UiPreferencesConfigMutation::$variant(Self { $id: self.$id.clone(), $value: base.$map.get(&self.$id).cloned() })]
            }
            fn label(&self) -> String {
                format!(concat!("Set ", $label, " {:?}"), self.$id)
            }
            fn target(&self) -> Vec<String> {
                vec![$target.to_string(), self.$id.clone()]
            }
        }
    };
}

#[path = "../🌗️set-appearance/🦀️.rs"]
pub mod set_appearance;
#[path = "../🚗️set-custom-driver/🦀️.rs"]
pub mod set_custom_driver;
#[path = "../🎨️set-custom-theme/🦀️.rs"]
pub mod set_custom_theme;
#[path = "../🕹️set-driver/🦀️.rs"]
pub mod set_driver;
#[path = "../⌨️set-keybinding-override/🦀️.rs"]
pub mod set_keybinding_override;
#[path = "../📐️set-layout/🦀️.rs"]
pub mod set_layout;
#[path = "../🗣️set-locale/🦀️.rs"]
pub mod set_locale;
#[path = "../📖️set-terminology/🦀️.rs"]
pub mod set_terminology;
#[path = "../🖼️set-theme/🦀️.rs"]
pub mod set_theme;

pub use set_appearance::*;
pub use set_custom_driver::*;
pub use set_custom_theme::*;
pub use set_driver::*;
pub use set_keybinding_override::*;
pub use set_layout::*;
pub use set_locale::*;
pub use set_terminology::*;
pub use set_theme::*;

#[cfg(test)]
#[path = "🧪️tests/🎨️updates-every-os-ui-preference/🦀️.rs"]
mod tests_updates_every_os_ui_preference;
