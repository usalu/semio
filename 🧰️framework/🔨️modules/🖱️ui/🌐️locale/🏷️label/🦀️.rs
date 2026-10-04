//! 🎗️ Compile-time-checked UI labels (`Label` / `LabelText` / `LocalizedLabel` / `AppLabels`).
//! Extracted from wgpu target `🦀️.rs` (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE).

// 🎗️ Replaces raw `String` labels on `UiNode` and the app manifest — a `Label` is only constructible
// from `app_labels!`-produced `LabelText` or explicit runtime data (`Label::data`), so a hardcoded
// literal assigned to a label field (`label: "LOD".into()`) does not compile. See ticket
// 26/08/03/COMPILE-TIME-CHECKED-UI-LABELS-ACROSS-LOCALE-TERMINOLOGY-AND-BRAND.
use super::{Locale, Terminology};
// 🌱️ `ToValue`/`FromValue` are this crate's own first-party analog of `Serialize`/`Deserialize`
// below, for ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
use semio_framework_value::{DslValue, FromValue, ToValue, ValueError, ValueRefusalKind};
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
/// the generated `LocalizedLabel` type in `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🎚️ui-axes/🟦️.ts`
/// (the wire shape below is explicitly kept in sync with that type by the owned schema metadata).
#[derive(Clone, Debug, Default, PartialEq)]
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

    /// ♻️ Every cell as an owned, mutable string — the retained close cursor pages an oversized
    /// history label cell by cell, exactly as it pages a plain retained `String` field.
    // 🚫️async: E1 pure accessor consumed by the sync close-cursor loop — see R9
    pub fn texts_mut(&mut self) -> impl Iterator<Item = &mut String> {
        self.cells.iter_mut().flatten().map(Cow::to_mut)
    }

    /// 📏️ Retained bytes of the WHOLE matrix. A label is a locale × terminology matrix now, so an
    /// artifact's fixed retained envelope pays for every cell, not for one string: a caller that
    /// used to admit `label.len()` admits this instead, and never under-counts the envelope by
    /// pricing one locale.
    // 🚫️async: E1 pure accessor consumed by the sync retained-footprint admission — see R9
    pub fn retained_bytes(&self) -> usize {
        self.cells.iter().flatten().map(|cell| cell.len()).sum()
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
        struct Row([String; Locale::COUNT]);
        impl<'de> Deserialize<'de> for Row {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                struct RowVisitor;
                impl<'de> serde::de::Visitor<'de> for RowVisitor {
                    type Value = Row;
                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str("the complete declared locale row") }
                    fn visit_map<M: serde::de::MapAccess<'de>>(self, mut map: M) -> Result<Row, M::Error> {
                        let mut cells: [Option<String>; Locale::COUNT] = std::array::from_fn(|_| None);
                        while let Some(key) = map.next_key::<String>()? {
                            let index = Locale::ALL.iter().position(|locale| locale.as_str() == key).ok_or_else(|| serde::de::Error::custom("unknown locale"))?;
                            if cells[index].is_some() { return Err(serde::de::Error::custom("duplicate locale")); }
                            cells[index] = Some(map.next_value()?);
                        }
                        if cells.iter().any(Option::is_none) { return Err(serde::de::Error::custom("missing locale")); }
                        Ok(Row(cells.map(|cell| cell.expect("complete locale row"))))
                    }
                }
                deserializer.deserialize_map(RowVisitor)
            }
        }
        struct MatrixVisitor;
        impl<'de> serde::de::Visitor<'de> for MatrixVisitor {
            type Value = LocalizedLabel;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str("the complete declared terminology matrix") }
            fn visit_map<M: serde::de::MapAccess<'de>>(self, mut map: M) -> Result<LocalizedLabel, M::Error> {
                let mut rows: [Option<Row>; Terminology::COUNT] = std::array::from_fn(|_| None);
                while let Some(key) = map.next_key::<String>()? {
                    let index = Terminology::ALL.iter().position(|terminology| terminology.as_str() == key).ok_or_else(|| serde::de::Error::custom("unknown terminology"))?;
                    if rows[index].is_some() { return Err(serde::de::Error::custom("duplicate terminology")); }
                    rows[index] = Some(map.next_value()?);
                }
                if rows.iter().any(Option::is_none) { return Err(serde::de::Error::custom("missing terminology")); }
                Ok(LocalizedLabel { cells: rows.map(|row| row.expect("complete terminology matrix").0.map(Cow::Owned)) })
            }
        }
        deserializer.deserialize_map(MatrixVisitor)
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
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
        control.scoped_depth(64, |control| control.scoped_stage(|control| {
            control.begin_stage(Terminology::COUNT)?;
            let mut outer = DslValue::object_encoding_controlled(Terminology::COUNT, control)?;
            for terminology in Terminology::ALL {
                let row = control.scoped_depth(64, |control| control.scoped_stage(|control| {
                    control.begin_stage(Locale::COUNT)?;
                    let mut inner = DslValue::object_encoding_controlled(Locale::COUNT, control)?;
                    for locale in Locale::ALL {
                        let text = control.copy_text(self.resolve(terminology, locale)).map(DslValue::String)?;
                        DslValue::push_encoding_controlled(inner.get_mut(), locale.as_str(), text, control)?;
                        control.step()?;
                    }
                    Ok::<DslValue, ValueError>(DslValue::Object(inner.take()))
                }))?;
                DslValue::push_encoding_controlled(outer.get_mut(), terminology.as_str(), row, control)?;
                control.step()?;
            }
            Ok(DslValue::Object(outer.take()))
        }))
    }
    fn to_value(&self) -> DslValue {
        DslValue::object(Terminology::ALL.into_iter().map(|terminology| {
            let inner = DslValue::object(Locale::ALL.into_iter().map(|locale| (locale.as_str().to_string(), DslValue::String(self.resolve(terminology, locale).to_string()))));
            (terminology.as_str().to_string(), inner)
        }))
    }
}

/// 🌱️ Admits exactly the declared complete matrix before copying any cells.
impl FromValue for LocalizedLabel {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let cells = label_cells(&value)?;
        Ok(Self { cells: cells.map(|row| row.map(|text| Cow::Owned(text.to_string()))) })
    }
    fn from_value_controlled(value: &DslValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        control.checkpoint()?;
        control.scoped_depth(64, |control| control.scoped_stage(|control| {
            control.begin_stage(Terminology::COUNT * Locale::COUNT)?;
            let input = label_cells(value)?;
            let mut cells = std::array::from_fn(|_| std::array::from_fn(|_| Cow::Borrowed("")));
            for terminology in Terminology::ALL {
                control.scoped_depth(64, |control| {
                    for locale in Locale::ALL {
                        cells[terminology.index()][locale.index()] = Cow::Owned(control.copy_text(input[terminology.index()][locale.index()])?);
                        control.step()?;
                    }
                    Ok::<(), ValueError>(())
                })?;
            }
            Ok(Self { cells })
        }))
    }
}

fn label_fields<'a>(value: &'a DslValue, expected: &[&str]) -> Result<&'a [(String, DslValue)], ValueError> {
    let DslValue::Object(entries) = value else { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "expected an object")); };
    for (index, (key, _)) in entries.iter().enumerate() {
        if !expected.contains(&key.as_str()) { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "unknown field")); }
        if entries[..index].iter().any(|(previous, _)| previous == key) { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "duplicate field").under(key)); }
    }
    for key in expected { if !entries.iter().any(|(name, _)| name == key) { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "missing field").under(key)); } }
    Ok(entries)
}

fn label_cells(value: &DslValue) -> Result<[[&str; Locale::COUNT]; Terminology::COUNT], ValueError> {
    let rows = label_fields(value, &Terminology::ALL.map(Terminology::as_str))?;
    let mut cells = [[""; Locale::COUNT]; Terminology::COUNT];
    for terminology in Terminology::ALL {
        let row = &rows.iter().find(|(key, _)| key == terminology.as_str()).expect("required terminology").1;
        let fields = label_fields(row, &Locale::ALL.map(Locale::as_str)).map_err(|error| error.under(terminology.as_str()))?;
        for locale in Locale::ALL {
            let value = &fields.iter().find(|(key, _)| key == locale.as_str()).expect("required locale").1;
            cells[terminology.index()][locale.index()] = value.as_str().ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, "expected text").under(locale.as_str()).under(terminology.as_str()))?;
        }
    }
    Ok(cells)
}

#[cfg(test)]
#[path = "../🧪️tests/🏷️label-value/🦀️.rs"]
mod localized_label_value_round_trip_tests;

#[cfg(test)]
#[path = "../🧪️tests/🏷️localized-label-fixture/🦀️.rs"]
mod localized_label_fixture_tests;

/// 🗣️ Two-axis label set; implement via `semio_framework_ui_locale::app_labels!` only — the macro emits
/// an exhaustive `match (terminology, locale)` with no catch-all, so a `Locale`/`Terminology` variant
/// added to the generated axes fails every implementor's build until covered.
pub trait AppLabels: Sized + 'static {
    fn labels(locale: Locale, terminology: Terminology) -> &'static Self;
}
