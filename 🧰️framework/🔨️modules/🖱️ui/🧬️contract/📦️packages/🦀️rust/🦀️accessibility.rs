//! @emoji ♿️ The `AccessibilitySpec` carried by every node record.
//!
//! ⚠️ SCAFFOLD — owned by packet `contract-layout`. Replace this placeholder wholesale; keep the region
//! structure and the U1 sync rule (no `async fn` in this crate).

use serde::{Deserialize, Serialize};

//#region 🔖️Accessibility

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}

/// 📢️ An ARIA-live-region politeness level, translated by each renderer into its own live-announce
/// mechanism (DOM `aria-live`, the GPU renderer's accessibility snapshot, ...).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Liveness {
    #[default]
    Off,
    Polite,
    Assertive,
}

/// ♿️ The accessibility intent every node carries once, resolved correctly by every renderer. No
/// `role` field: the semantic role is implied by [`crate::Component`] — a `Component::Button` is a
/// button on every renderer, so naming the role again here would just be a second, driftable source
/// of truth.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilitySpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<crate::Label>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<crate::Label>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub live: Liveness,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shortcut: Option<crate::UiText>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub hidden: bool,
}
//#endregion 🔖️Accessibility

#[cfg(test)]
mod tests {
    use super::*;

    fn ui_text(value: &str) -> crate::UiText {
        crate::UiText::try_from_str(value).expect("bounded fixture text")
    }

    #[test]
    fn default_spec_serializes_to_empty_object() {
        let json = serde_json::to_value(AccessibilitySpec::default()).expect("serialize");
        assert_eq!(json, serde_json::json!({}));
    }

    #[test]
    fn liveness_field_omitted_at_default() {
        let json = serde_json::to_value(AccessibilitySpec { live: Liveness::Assertive, ..AccessibilitySpec::default() }).expect("serialize");
        assert_eq!(json, serde_json::json!({ "live": "assertive" }));
    }

    #[test]
    fn hidden_field_omitted_at_default() {
        let json = serde_json::to_value(AccessibilitySpec { hidden: true, ..AccessibilitySpec::default() }).expect("serialize");
        assert_eq!(json, serde_json::json!({ "hidden": true }));
    }

    #[test]
    fn shortcut_roundtrips() {
        let spec = AccessibilitySpec { shortcut: Some(ui_text("Ctrl+S")), live: Liveness::Polite, hidden: false, ..AccessibilitySpec::default() };
        let json = serde_json::to_string(&spec).expect("serialize");
        let back: AccessibilitySpec = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(spec, back);
    }
}
