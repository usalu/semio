//! 🎗️ Compile-time-checked UI labels (`Label` / `LabelText` / `LocalizedLabel` / `AppLabels`).
//! Extracted from wgpu target `🦀️.rs` (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE).

// 🎗️ Replaces raw `String` labels on `UiNode` and the app manifest — a `Label` is only constructible
// from `app_labels!`-produced `LabelText` or explicit runtime data (`Label::data`), so a hardcoded
// literal assigned to a label field (`label: "LOD".into()`) does not compile. See ticket
// 26/08/03/COMPILE-TIME-CHECKED-UI-LABELS-ACROSS-LOCALE-TERMINOLOGY-AND-BRAND.
use super::{Locale, Terminology};
// 🌱️ `dsl` is the `semio_framework_os_kernel` extern-crate alias set up crate-wide in the
// `wgpu`-feature-gated `🦀️.rs` root (this file is only ever mounted under that feature — see its
// docstring). `ToValue`/`FromValue` here is the first-party analog of `Serialize`/`Deserialize`
// below, for ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
use dsl::{DslValue, FromValue, ToValue, ValueError};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// 🎗️ A display-ready UI string. No `From<&str>`/`From<String>` on purpose.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(transparent)]
#[value(transparent)]
pub struct Label(String);

impl Label {
    /// 📊️ Genuine runtime data (file names, counts, user content) rendered as a label. Passing a
    /// string literal here is a gate violation (see the Rust twin of `uiDataLabel`'s TS lint).
    pub fn data(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    // 🚫️async: E1 pure accessor consumed by sync-only std call sites (Option::map fn-value) — see R9
    pub fn into_string(self) -> String {
        self.0
    }
}

impl From<LabelText> for Label {
    fn from(text: LabelText) -> Self {
        Self(text.0.to_string())
    }
}

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::borrow::Borrow<str> for Label {
    fn borrow(&self) -> &str {
        &self.0
    }
}

/// 🧵️ A localized static template, produced only by `app_labels!` (never construct directly — the
/// hidden constructor is the macro's, not a public API; committed source calling it is a gate
/// violation, mirroring the TS `__from_app_labels` ban).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LabelText(&'static str);

impl LabelText {
    #[doc(hidden)]
    pub const fn __from_app_labels(text: &'static str) -> Self {
        Self(text)
    }

    pub fn as_str(self) -> &'static str {
        self.0
    }

    /// 🧵️ Named-placeholder runtime fill, e.g. `labels.selected_count.fill(&[("count", &n.to_string())])`
    /// — substitution, not `format!`, so word order never has to match across locales.
    pub fn fill(self, args: &[(&str, &str)]) -> Label {
        let mut out = self.0.to_string();
        for (name, value) in args {
            out = out.replace(&format!("{{{name}}}"), value);
        }
        Label(out)
    }
}

impl From<LabelText> for String {
    fn from(text: LabelText) -> Self {
        text.0.to_string()
    }
}

/// 🗺️ Full locale×terminology matrix for a manifest label, resolved shell-side per active axes —
/// the multilingual replacement for `AppLabelsOverlay`'s stringly-typed per-id maps. TS mirror is
/// the hand-generated `LocalizedLabel` type in `framework/⚡️implementations/🟦️typescript/🤖️generated/🟦️ui-axes.ts`
/// (the wire shape below is explicitly kept in sync with that type by the owned schema metadata).
#[derive(Clone, Debug, PartialEq)]
pub struct LocalizedLabel {
    cells: [[Cow<'static, str>; Locale::COUNT]; Terminology::COUNT],
}

impl LocalizedLabel {
    /// 🗺️ Builds the full matrix from a resolver called once per (terminology, locale) cell.
    // 🚫️async: E1 pure accessor consumed by external-trait impls (Serialize/Deserialize) — see R9
    pub fn from_fn(mut resolve: impl FnMut(Terminology, Locale) -> String) -> Self {
        let cells = std::array::from_fn(|ti| {
            let terminology = Terminology::ALL[ti];
            std::array::from_fn(|li| Cow::Owned(resolve(terminology, Locale::ALL[li])))
        });
        Self { cells }
    }

    /// 📊️ Locale-invariant runtime data (fixture names, proper nouns) broadcast to every cell.
    // 🚫️async: E1 pure accessor consumed by external-trait impls (Serialize/Deserialize) — see R9
    pub fn data(value: impl Into<String>) -> Self {
        let value = value.into();
        Self::from_fn(|_, _| value.clone())
    }

    /// 🌐️ Terminology-invariant framework-owned text (same copy regardless of terminology, real
    /// per-locale translation) — for the framework's own built-in manifest text (history actions,
    /// panel tabs, …), which has no app-declared terminology axis. The exhaustive match on `Locale`
    /// (no catch-all) means adding a locale breaks every call site here until translated.
    // 🚫️async: E1 pure accessor consumed by external-trait impls (Serialize/Deserialize) — see R9
    pub fn native(en: &str, de: &str) -> Self {
        Self::from_fn(|_terminology, locale| {
            match locale {
                Locale::En => en,
                Locale::De => de,
            }
            .to_string()
        })
    }

    // 🚫️async: E1 pure accessor consumed by external-trait impls (Serialize/Deserialize) — see R9
    pub fn resolve(&self, terminology: Terminology, locale: Locale) -> &str {
        &self.cells[terminology.index()][locale.index()]
    }
}

impl Serialize for LocalizedLabel {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut outer = serializer.serialize_map(Some(Terminology::COUNT))?;
        for terminology in Terminology::ALL {
            let inner: std::collections::BTreeMap<&str, &str> = Locale::ALL.iter().map(|&locale| (locale.as_str(), self.resolve(terminology, locale))).collect();
            outer.serialize_entry(terminology.as_str(), &inner)?;
        }
        outer.end()
    }
}

impl<'de> Deserialize<'de> for LocalizedLabel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw: std::collections::HashMap<String, std::collections::HashMap<String, String>> = Deserialize::deserialize(deserializer)?;
        Ok(Self::from_fn(|terminology, locale| raw.get(terminology.as_str()).and_then(|m| m.get(locale.as_str())).cloned().unwrap_or_default()))
    }
}

/// 🌱️ Hand-written, not derived: `#[derive(ToValue, FromValue)]` (`#[value(...)]`) only reads a
/// struct's own named/unnamed fields, and `cells` is a `[[Cow<'static, str>; N]; M]` fixed-size
/// nested array with no field-level shape to annotate — the same reason `Serialize`/`Deserialize`
/// above are hand-written rather than derived. Mirrors those two exactly: the SAME
/// `{terminology.as_str(): {locale.as_str(): text}}` object shape, so the wire format is
/// unchanged and `to_dsl_value(&to_json_value(x)) == x.to_value()` for every `LocalizedLabel`.
/// Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
impl ToValue for LocalizedLabel {
    fn to_value(&self) -> DslValue {
        DslValue::object(Terminology::ALL.into_iter().map(|terminology| {
            let inner = DslValue::object(Locale::ALL.into_iter().map(|locale| (locale.as_str().to_string(), DslValue::String(self.resolve(terminology, locale).to_string()))));
            (terminology.as_str().to_string(), inner)
        }))
    }
}

/// 🌱️ Exact inverse of the `ToValue` impl above — same missing-key-defaults-to-empty-string
/// fallback the hand-written `Deserialize` impl already uses (`raw.get(...).unwrap_or_default()`).
impl FromValue for LocalizedLabel {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        Ok(Self::from_fn(|terminology, locale| value.get(terminology.as_str()).and_then(|inner| inner.get(locale.as_str())).and_then(DslValue::as_str).map(str::to_string).unwrap_or_default()))
    }
}

#[cfg(test)]
mod localized_label_value_round_trip_tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn round_trips_through_to_value_and_from_value() {
        let label = LocalizedLabel::from_fn(|terminology, locale| format!("{}-{}", terminology.as_str(), locale.as_str()));
        let decoded = LocalizedLabel::from_value(label.to_value()).expect("valid DslValue decodes");
        assert_eq!(decoded, label);
    }

    #[semio_framework_async_macros::async_test]
    async fn to_value_uses_the_same_keys_the_hand_written_serde_impl_emits() {
        let label = LocalizedLabel::native("Hello", "Hallo");
        let entries = label.to_value().into_object().expect("LocalizedLabel::to_value is an object");
        for terminology in Terminology::ALL {
            let inner = entries.iter().find(|(key, _)| key == terminology.as_str()).map(|(_, value)| value.clone()).expect("terminology key present").into_object().expect("inner value is an object");
            for locale in Locale::ALL {
                let text = inner.iter().find(|(key, _)| key == locale.as_str()).map(|(_, value)| value.clone()).expect("locale key present");
                assert_eq!(text, DslValue::String(label.resolve(terminology, locale).to_string()));
            }
        }
    }
}

/// 🗣️ Two-axis label set; implement via `semio_framework_plugin::app_labels!` only — the macro emits
/// an exhaustive `match (terminology, locale)` with no catch-all, so a `Locale`/`Terminology` variant
/// added to the generated axes fails every implementor's build until covered.
pub trait AppLabels: Sized + 'static {
    fn labels(locale: Locale, terminology: Terminology) -> &'static Self;
}
