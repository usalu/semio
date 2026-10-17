//! 🪪️ App role: the closed role of an app with its parsing and value, serde and DSL conversions. See [`super`].

use super::*;

// #region 🛂️Manifest
//#region 🔖️Manifest
//#region 🔖️Surface
/// 👁️✏️ Whether a surface may change the artifact it is bound to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, semio_framework_value::RetireOwned)]
pub enum AppRole {
    Viewer,
    Editor,
}

impl AppRole {
    /// 🔤️ Wire spelling — exactly `"viewer"`/`"editor"`, shared by serde, TS, JSON schema and the
    /// `SEMIO_APP_ROLE`/`VITE_SEMIO_APP_ROLE` env values.
    pub fn as_str(&self) -> &'static str {
        match self {
            AppRole::Viewer => "viewer",
            AppRole::Editor => "editor",
        }
    }
}

impl std::str::FromStr for AppRole {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "viewer" => Ok(AppRole::Viewer),
            "editor" => Ok(AppRole::Editor),
            other => Err(format!("unknown app role {other:?}, expected \"viewer\" or \"editor\"")),
        }
    }
}

/// 🌉️ Hand-written, not derived: `#[derive(ToValue, FromValue)]`'s enum path only supports
/// internally-tagged representations, but `AppRole` is a plain unit-only "string enum" — serde's
/// own default (untagged bare-string) representation. Delegates to the existing `as_str`/`FromStr`
/// so the wire spelling never drifts from the serde/TS/JSON-schema one.
impl ToValue for AppRole {
    fn to_value(&self) -> DslValue {
        DslValue::String(self.as_str().to_string())
    }
}

impl FromValue for AppRole {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::String(s) => s.parse::<AppRole>().map_err(|error| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error)),
            other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string, found {other:?}"))),
        }
    }
}

// 🚧️ Needed in serde form too: referenced by a `🚧️ BLOCKED` serde-only manifest type — hand-written
// (not derived, same reasoning as the `ToValue`/`FromValue` pair above) via the same `as_str`/
// `FromStr` delegation.
impl Serialize for AppRole {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for AppRole {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse::<AppRole>().map_err(serde::de::Error::custom)
    }
}
//#endregion 🔖️Surface
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
