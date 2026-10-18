//! 🧵️ Source form and JSON of the glTF snapshot: the wire dialect a snapshot was parsed from and the opaque JSON value with its value codecs. See [`super`].

#[cfg(any(test, feature = "testing"))]
use serde::{Deserialize, Serialize};

//#region 🔖️SourceForm
/// 🧵 Which wire dialect a snapshot was last parsed from -- drives [`serialize_gltf_document`]'s
/// choice of whether a no-`uri` buffer needs re-embedding as a data uri (a `.glb`-sourced buffer
/// serialized back out as plain `.gltf` JSON text has no BIN chunk to lean on).
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, Default, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "testing"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testing"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_dsl_record_derive::DslScalar)]
pub enum GltfSourceForm {
    #[default]
    Json,
    Glb,
}
//#endregion 🔖️SourceForm

//#region 🔖️GltfJson
/// 🌳 A JSON value living inside a glTF `extras`/`extensions` slot -- this artifact's OWN local
/// value enum (structurally similar to `stdio.json`'s `JsonValue`, deliberately NOT imported: the
/// recipe treats "own `JsonValue`-shaped type per artifact" as separate concerns even where the
/// shape coincides). `Object` is a `Vec<(String, GltfJson)>`, never a map, so member insertion
/// order survives decode->encode exactly. `Number` widens to `f64` (glTF extras/extensions are
/// free-form JSON with no `bufferView`-precision requirement, unlike `stdio.json`'s own
/// arbitrary-precision lexeme retention).
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, Default)]
pub enum GltfJson {
    #[default]
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<GltfJson>),
    Object(Vec<(String, GltfJson)>),
}

/// 🌉️ Retains the ordered extension value tree through canonical owned values.
impl semio_framework_value::ToValue for GltfJson {
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_value::DslValue, semio_framework_value::ValueError> {
        control.scoped_depth(64, |control| control.scoped_stage(|control| match self {
            Self::Null => semio_framework_value::ToValue::to_value_controlled(&(), control),
            Self::Bool(value) => semio_framework_value::ToValue::to_value_controlled(value, control),
            Self::Number(value) => semio_framework_value::ToValue::to_value_controlled(value, control),
            Self::String(value) => semio_framework_value::ToValue::to_value_controlled(value, control),
            Self::Array(values) => {
                control.begin_stage(values.len())?;
                let mut output = control.allocate_vec(values.len())?;
                for value in values { output.push(semio_framework_value::ToValue::to_value_controlled(value, control)?); control.step()?; }
                Ok(semio_framework_value::DslValue::Array(output))
            },
            Self::Object(values) => {
                control.begin_stage(values.len())?;
                let mut output = control.allocate_vec(values.len())?;
                for (key, value) in values { output.push((control.copy_text(key)?, semio_framework_value::ToValue::to_value_controlled(value, control)?)); control.step()?; }
                Ok(semio_framework_value::DslValue::Object(output))
            },
        }))
    }
    fn to_value(&self) -> semio_framework_value::DslValue {
        match self {
            GltfJson::Null => semio_framework_value::DslValue::Null,
            GltfJson::Bool(b) => semio_framework_value::DslValue::Bool(*b),
            GltfJson::Number(n) => semio_framework_value::DslValue::float(*n),
            GltfJson::String(s) => semio_framework_value::DslValue::String(s.clone()),
            GltfJson::Array(items) => semio_framework_value::DslValue::Array(items.iter().map(semio_framework_value::ToValue::to_value).collect()),
            GltfJson::Object(members) => semio_framework_value::DslValue::object(members.iter().map(|(k, v)| (k.clone(), semio_framework_value::ToValue::to_value(v)))),
        }
    }
}

impl semio_framework_value::FromValue for GltfJson {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        Ok(match value {
            semio_framework_value::DslValue::Null => GltfJson::Null,
            semio_framework_value::DslValue::Bool(b) => GltfJson::Bool(b),
            semio_framework_value::DslValue::Number(n) => GltfJson::Number(n.as_f64()),
            semio_framework_value::DslValue::String(s) => GltfJson::String(s),
            semio_framework_value::DslValue::Bytes(_) => return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "GLTF extras cannot contain an intrinsic byte value")),
            semio_framework_value::DslValue::Array(items) => GltfJson::Array(items.into_iter().map(semio_framework_value::FromValue::from_value).collect::<Result<Vec<_>, _>>()?),
            semio_framework_value::DslValue::Object(members) => GltfJson::Object(members.into_iter().map(|(k, v)| Ok((k, semio_framework_value::FromValue::from_value(v)?))).collect::<Result<Vec<_>, semio_framework_value::ValueError>>()?),
        })
    }
}
//#endregion 🔖️GltfJson
