//! 🎗️ Compile-time-checked UI labels (`Label` / `LabelText` / `LocalizedLabel` / `AppLabels`).
//! Extracted from wgpu target `📦️lib.rs` (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE).

// 🎗️ Replaces raw `String` labels on `UiNode` and the app manifest — a `Label` is only constructible
// from `app_labels!`-produced `LabelText` or explicit runtime data (`Label::data`), so a hardcoded
// literal assigned to a label field (`label: "LOD".into()`) does not compile. See ticket
// 26/08/03/COMPILE-TIME-CHECKED-UI-LABELS-ACROSS-LOCALE-TERMINOLOGY-AND-BRAND.
use super::{Locale, Terminology};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// 🎗️ A display-ready UI string. No `From<&str>`/`From<String>` on purpose.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
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
/// (not ts-rs-derived — the wire shape below is manually kept in sync with that type).
#[derive(Clone, Debug, PartialEq)]
pub struct LocalizedLabel {
    cells: [[Cow<'static, str>; Locale::COUNT]; Terminology::COUNT],
}

impl LocalizedLabel {
    /// 🗺️ Builds the full matrix from a resolver called once per (terminology, locale) cell.
    pub fn from_fn(mut resolve: impl FnMut(Terminology, Locale) -> String) -> Self {
        let cells = std::array::from_fn(|ti| {
            let terminology = Terminology::ALL[ti];
            std::array::from_fn(|li| Cow::Owned(resolve(terminology, Locale::ALL[li])))
        });
        Self { cells }
    }

    /// 📊️ Locale-invariant runtime data (fixture names, proper nouns) broadcast to every cell.
    pub fn data(value: impl Into<String>) -> Self {
        let value = value.into();
        Self::from_fn(|_, _| value.clone())
    }

    /// 🌐️ Terminology-invariant framework-owned text (same copy regardless of terminology, real
    /// per-locale translation) — for the framework's own built-in manifest text (history actions,
    /// panel tabs, …), which has no app-declared terminology axis. The exhaustive match on `Locale`
    /// (no catch-all) means adding a locale breaks every call site here until translated.
    pub fn native(en: &str, de: &str) -> Self {
        Self::from_fn(|_terminology, locale| {
            match locale {
                Locale::En => en,
                Locale::De => de,
            }
            .to_string()
        })
    }

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

/// 🗣️ Two-axis label set; implement via `semio_framework_plugin::app_labels!` only — the macro emits
/// an exhaustive `match (terminology, locale)` with no catch-all, so a `Locale`/`Terminology` variant
/// added to the generated axes fails every implementor's build until covered.
pub trait AppLabels: Sized + 'static {
    fn labels(locale: Locale, terminology: Terminology) -> &'static Self;
}
