//! 🎛️ Modes of the app surface: the mode definition, the non-empty vector with its conversions and the mode and window kind collections. See [`super`].

use super::*;

// #region 🛂️Manifest
//#region 🔖️Manifest
// 🚧️ Kept additive: consumed outside this pass by 🛍️products/💻️os (plugin/renderer modules) and/or ✏️s/🔌️plugins/** while still serde-deriving; ToValue/FromValue added alongside, not replacing, Serialize/Deserialize. Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ModeDefinition {
    pub id: String,
    /// 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
    pub label: LocalizedLabel,
    pub icon_id: IconName,
    /// 🛠️ Tools available while this mode is active — references `AppDefinition.tools` ids.
    #[serde(default)]
    #[value(default)]
    pub tools: Vec<ToolRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub layout_id: Option<String>,
    /// ⌨️ Commands owned by this mode and active only while it is active.
    #[serde(default)]
    #[value(default)]
    pub commands: Vec<CommandDefinition>,
}

/// 🚫️ A non-empty, order-preserving list — construction-time enforcement replaces what used to be a
/// runtime `assert!` deep inside `AppBuilder::build_definition`. The first entry is the implicit
/// fallback default when nothing else specifies one.
#[derive(Clone, Debug, PartialEq)]
pub struct NonEmptyVec<T> {
    pub(super) first: T,
    rest: Vec<T>,
}

// 🚫️async: E1 transitive block — every method here is a pure accessor/iterator with no I/O, and
// `iter`/`iter_mut` must feed directly into std `Iterator` combinators (`find`/`filter`/`map`, an
// external trait) at call sites, so the whole inherent impl stays sync rather than forcing an
// `.await` between every accessor and the combinator chain that consumes it (R9).
impl<T> NonEmptyVec<T> {
    pub fn one(first: T) -> Self {
        Self { first, rest: Vec::new() }
    }

    pub fn new(first: T, rest: Vec<T>) -> Self {
        Self { first, rest }
    }

    pub fn first(&self) -> &T {
        &self.first
    }

    pub fn len(&self) -> usize {
        1 + self.rest.len()
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        std::iter::once(&self.first).chain(self.rest.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        std::iter::once(&mut self.first).chain(self.rest.iter_mut())
    }

    pub fn first_mut(&mut self) -> &mut T {
        &mut self.first
    }

    pub fn push(&mut self, value: T) {
        self.rest.push(value);
    }
}

impl<T> std::ops::Index<usize> for NonEmptyVec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        if index == 0 {
            &self.first
        } else {
            &self.rest[index - 1]
        }
    }
}

impl<'a, T> IntoIterator for &'a NonEmptyVec<T> {
    type Item = &'a T;
    type IntoIter = std::iter::Chain<std::iter::Once<&'a T>, std::slice::Iter<'a, T>>;
    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(&self.first).chain(self.rest.iter())
    }
}

impl<T> TryFrom<Vec<T>> for NonEmptyVec<T> {
    type Error = String;
    fn try_from(mut values: Vec<T>) -> Result<Self, Self::Error> {
        if values.is_empty() {
            return Err("expected a non-empty list, got zero entries".to_string());
        }
        let first = values.remove(0);
        Ok(Self { first, rest: values })
    }
}

impl<T: Clone> From<NonEmptyVec<T>> for Vec<T> {
    fn from(value: NonEmptyVec<T>) -> Self {
        std::iter::once(value.first).chain(value.rest).collect()
    }
}

/// 🌉️ Hand-written, not derived: `#[value(…)]` has no `into`/`try_from` equivalent — this mirrors
/// the retired `#[serde(try_from = "Vec<T>", into = "Vec<T>")]` bridge, wire-shaped as a plain
/// array exactly like `Vec<T>` (see `ToValue`/`FromValue for Vec<T>` in `🌱️value/🔁️codec/🦀️.rs`).
impl<T: ToValue> ToValue for NonEmptyVec<T> {
    fn to_value(&self) -> DslValue {
        DslValue::Array(self.iter().map(ToValue::to_value).collect())
    }
}

impl<T: FromValue> FromValue for NonEmptyVec<T> {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Array(items) = value else {
            return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an array, found {value:?}")));
        };
        let values = items.into_iter().enumerate().map(|(index, item)| T::from_value(item).map_err(|error| error.under(index))).collect::<Result<Vec<T>, ValueError>>()?;
        NonEmptyVec::try_from(values).map_err(|error| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error))
    }
}

// 🚧️ BLOCKED (26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS): `AppDefinition`
// (`modes: Modes`/`window_kinds: WindowKinds`, both `NonEmptyVec<_>`) is itself serde-only — blocked
// on `ui_wgpu`/`interaction`/`🎠️kernel` field types elsewhere in this file — so `NonEmptyVec` must
// keep pace with it. Hand-written (not `#[serde(try_from/into = "Vec<T>")]`, since the derive is
// gone): mirrors the retired attribute via the existing `TryFrom<Vec<T>>`/`From<NonEmptyVec<T>> for
// Vec<T>` bridge above.
impl<T: Clone + Serialize> Serialize for NonEmptyVec<T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let values: Vec<T> = self.clone().into();
        values.serialize(serializer)
    }
}

impl<'de, T: serde::de::DeserializeOwned> Deserialize<'de> for NonEmptyVec<T> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let values = Vec::<T>::deserialize(deserializer)?;
        NonEmptyVec::try_from(values).map_err(serde::de::Error::custom)
    }
}

/// 🧭️ Every app has at least one mode — `playbook/module/procedural` and any other single-purpose app
/// must declare an explicit mode (e.g. `"default"`) instead of the zero-mode state the type system
/// now makes unrepresentable.
pub type Modes = NonEmptyVec<ModeDefinition>;

/// 🪟️ Every app has at least one window kind — mirrors `Modes`, formerly a runtime `assert!` in
/// `AppBuilder::build_definition`.
pub type WindowKinds = NonEmptyVec<WindowKindDefinition>;
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
