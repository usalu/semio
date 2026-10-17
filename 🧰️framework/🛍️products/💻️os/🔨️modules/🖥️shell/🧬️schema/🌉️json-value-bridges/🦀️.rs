//! 🌉️ JSON value bridges of the shell schema: the open JSON value and staged argument aliases and the four value bridge modules the wire types name through `value(with)`. See [`super`].

use super::*;

/// 🧩️ Free-form staged/seed argument payload (action args, command args, dialog seed args).
/// The owned schema metadata projects this deliberately open value as TypeScript `unknown`.
pub type JsonValue = serde_json::Value;

/// 🎚️ Arguments indexed by command or action identifier, then argument identifier.
pub type StagedCommandArgs = HashMap<String, HashMap<String, JsonValue>>;

/// 🪟️ Action arguments indexed by window, action, and argument identifier.
pub type StagedActionArgs = HashMap<String, StagedCommandArgs>;

// #region 🌉️ ValueBridges
// 🌉️ `ToValue`/`FromValue` is not implemented for `serde_json::Value` anywhere reachable from this
// crate (implementing it here would be an orphan-rule violation: both the trait, defined in
// `protocol::value`, and the type, defined in `serde_json`, are foreign to this crate) — every
// `JsonValue`/`Option<JsonValue>`/`HashMap<..., JsonValue>` struct field below instead names one of
// these bridges via `#[value(with = "...")]`, built on `DslValue`'s existing bidirectional
// `serde_json::Value` conversions (`🌱️value/🦀️.rs`). ADDITIVE ONLY — round-trips through the exact
// same `DslValue` shape a plain `impl ToValue for serde_json::Value` would produce.
pub(super) mod json_value_bridge {
    pub fn to_value(value: &super::JsonValue) -> dsl_core::DslValue {
        dsl_core::DslValue::from(value.clone())
    }
    #[expect(clippy::unnecessary_wraps, reason = "The value derive custom codec interface requires a fallible decoder signature.")]
    pub fn from_value(value: dsl_core::DslValue) -> Result<super::JsonValue, dsl_core::ValueError> {
        Ok(super::JsonValue::from(value))
    }
    use super::dsl_core;
}

pub(super) mod json_value_option_bridge {
    pub fn to_value(value: &Option<super::JsonValue>) -> dsl_core::DslValue {
        match value {
            Some(v) => dsl_core::DslValue::from(v.clone()),
            None => dsl_core::DslValue::Null,
        }
    }
    #[expect(clippy::unnecessary_wraps, reason = "The value derive custom codec interface requires a fallible decoder signature.")]
    pub fn from_value(value: dsl_core::DslValue) -> Result<Option<super::JsonValue>, dsl_core::ValueError> {
        match value {
            dsl_core::DslValue::Null => Ok(None),
            other => Ok(Some(super::JsonValue::from(other))),
        }
    }
    use super::dsl_core;
}

/// 🌉️ `ShellState::staged_command_args`'s exact shape: `command_id -> arg_id -> value`.
pub(super) mod staged_command_args_bridge {
    pub fn to_value(value: &super::StagedCommandArgs) -> dsl_core::DslValue {
        dsl_core::DslValue::object(value.iter().map(|(k, inner)| {
            (k.clone(), dsl_core::DslValue::object(inner.iter().map(|(k2, v)| (k2.clone(), dsl_core::DslValue::from(v.clone())))))
        }))
    }
    pub fn from_value(value: dsl_core::DslValue) -> Result<super::StagedCommandArgs, dsl_core::ValueError> {
        let dsl_core::DslValue::Object(entries) = value else {
            return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "expected an object for staged_command_args"));
        };
        entries.into_iter().map(|(k, inner)| {
            let dsl_core::DslValue::Object(inner_entries) = inner else {
                return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "expected an object").under(k));
            };
            let inner_map = inner_entries.into_iter().map(|(k2, v)| (k2, super::JsonValue::from(v))).collect();
            Ok((k, inner_map))
        }).collect()
    }
    use super::dsl_core;
}

/// 🌉️ `ShellState::staged_action_args`'s exact shape: `window_id -> action_id -> arg_id -> value`.
pub(super) mod staged_action_args_bridge {
    use std::collections::HashMap;
    pub fn to_value(value: &super::StagedActionArgs) -> dsl_core::DslValue {
        dsl_core::DslValue::object(value.iter().map(|(k, mid)| {
            (k.clone(), dsl_core::DslValue::object(mid.iter().map(|(k2, inner)| {
                (k2.clone(), dsl_core::DslValue::object(inner.iter().map(|(k3, v)| (k3.clone(), dsl_core::DslValue::from(v.clone())))))
            })))
        }))
    }
    pub fn from_value(value: dsl_core::DslValue) -> Result<super::StagedActionArgs, dsl_core::ValueError> {
        let dsl_core::DslValue::Object(entries) = value else {
            return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "expected an object for staged_action_args"));
        };
        entries.into_iter().map(|(k, mid)| {
            let dsl_core::DslValue::Object(mid_entries) = mid else {
                return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "expected an object").under(k));
            };
            let mid_map = mid_entries.into_iter().map(|(k2, inner)| {
                let dsl_core::DslValue::Object(inner_entries) = inner else {
                    return Err(dsl_core::ValueError::new(dsl_core::ValueRefusalKind::InvalidValue, "expected an object").under(k2));
                };
                let inner_map = inner_entries.into_iter().map(|(k3, v)| (k3, super::JsonValue::from(v))).collect();
                Ok((k2, inner_map))
            }).collect::<Result<HashMap<_, _>, dsl_core::ValueError>>()?;
            Ok((k, mid_map))
        }).collect()
    }
    use super::dsl_core;
}
// #endregion 🌉️ ValueBridges
