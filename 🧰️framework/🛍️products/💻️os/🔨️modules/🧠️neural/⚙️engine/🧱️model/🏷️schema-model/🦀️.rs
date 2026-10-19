//! 🏷️ Schema model: the schema, field specification and reference records, the schema errors and the value type names.
//!
//! Holds the `🔖️Schema`, `⚠️ Errors`, `🔖️SchemaComponent` regions of the module; the root re-exports its items.

use super::*;

// #region 🔖️Schema
pub const SCHEMA_KEY: &str = "$schema";

#[cfg(any(test, feature = "testkit"))]
#[path = "../../🧪️tests/🏷️type/🦀️.rs"]
mod value_type_reference;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(any(test, feature = "testkit"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
pub struct FieldSpec {
    pub key: String,
    #[cfg_attr(any(test, feature = "testkit"), serde(with = "value_type_reference"))]
    pub value: ValueType,
    #[cfg_attr(any(test, feature = "testkit"), serde(skip_serializing_if = "Option::is_none"))]
    pub default: Option<Value>,
    #[cfg_attr(any(test, feature = "testkit"), serde(skip_serializing_if = "Option::is_none"))]
    pub label: Option<String>,
}

impl FieldSpec {
    pub fn new(key: impl Into<String>, value: ValueType) -> Self {
        Self { key: key.into(), value, default: None, label: None }
    }

    pub fn with_default(mut self, default: Value) -> Self {
        self.default.replace(default).retire_cold();
        self
    }

    pub fn decimal(key: impl Into<String>) -> Self {
        Self::new(key, ValueType::Decimal)
    }

    pub fn decimal_default(key: impl Into<String>, default: f64) -> Self {
        Self::decimal(key).with_default(Value::Atom(Atom::Decimal(default)))
    }
}

impl ToValue for FieldSpec {
    fn to_value(&self) -> DslValue {
        DslValue::Object(vec![
            ("key".into(), self.key.to_value()),
            ("value".into(), self.value.to_value()),
            ("default".into(), self.default.to_value()),
            ("label".into(), self.label.to_value()),
        ])
    }
}

impl FromValue for FieldSpec {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(_) = &value else { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object for FieldSpec")) };
        Ok(Self {
            key: value.get("key").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "key"))?,
            value: value.get("value").cloned().map(ValueType::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "value"))?,
            default: value.get("default").cloned().map(Option::<Value>::from_value).transpose()?.unwrap_or_default(),
            label: value.get("label").cloned().map(Option::<String>::from_value).transpose()?.unwrap_or_default(),
        })
    }
}

/// 🔮️ `serde` is TEST-ONLY — see `Dictionary`'s docstring; `fields: Vec<FieldSpec>` fans in `Value` transitively.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(any(test, feature = "testkit"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
pub struct Schema {
    pub id: String,
    pub module: String,
    pub name: String,
    pub icon: String,
    pub summary: String,
    pub fields: Vec<FieldSpec>,
}

impl ToValue for Schema {
    fn to_value(&self) -> DslValue {
        DslValue::Object(vec![
            ("id".into(), self.id.to_value()),
            ("module".into(), self.module.to_value()),
            ("name".into(), self.name.to_value()),
            ("icon".into(), self.icon.to_value()),
            ("summary".into(), self.summary.to_value()),
            ("fields".into(), self.fields.to_value()),
        ])
    }
}

impl FromValue for Schema {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(_) = &value else { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object for Schema")) };
        Ok(Self {
            id: value.get("id").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "id"))?,
            module: value.get("module").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "module"))?,
            name: value.get("name").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "name"))?,
            icon: value.get("icon").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "icon"))?,
            summary: value.get("summary").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "summary"))?,
            fields: value.get("fields").cloned().map(Vec::<FieldSpec>::from_value).transpose()?.unwrap_or_default(),
        })
    }
}

/// 🏷️ Schema id with display metadata for pickers.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(any(test, feature = "testkit"), derive(Serialize, Deserialize))]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
pub struct SchemaRef {
    pub id: String,
    pub name: String,
    pub icon: String,
}

impl ToValue for SchemaRef {
    fn to_value(&self) -> DslValue {
        DslValue::Object(vec![("id".into(), self.id.to_value()), ("name".into(), self.name.to_value()), ("icon".into(), self.icon.to_value())])
    }
}

impl FromValue for SchemaRef {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(_) = &value else { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object for SchemaRef")) };
        Ok(Self {
            id: value.get("id").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "id"))?,
            name: value.get("name").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "name"))?,
            icon: value.get("icon").cloned().map(String::from_value).transpose()?.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "icon"))?,
        })
    }
}

impl Schema {
    pub fn validate(&self, dictionary: &Dictionary) -> Result<(), EvalError> {
        match dictionary.schema() {
            Some(id) if id == self.id => {}
            Some(id) => return Err(EvalError::InvalidInput(format!("schema {id} does not match {}", self.id))),
            None => return Err(EvalError::MissingInput(SCHEMA_KEY.into())),
        }
        for field in &self.fields {
            let Some(value) = dictionary.get(&field.key) else {
                return Err(EvalError::MissingInput(field.key.clone()));
            };
            if !field.value.matches(value.kind()) {
                return Err(EvalError::InvalidInput(format!("field {} does not match {}", field.key, field.value.id())));
            }
        }
        Ok(())
    }

    pub fn default_dictionary(&self) -> Dictionary {
        let mut dictionary = Dictionary::with_schema(self.id.clone());
        for field in &self.fields {
            if let Some(default) = &field.default {
                dictionary = dictionary.insert(field.key.clone(), default.clone());
            }
        }
        dictionary
    }
}
// #endregion 🔖️Schema
//#region ⚠️ Errors
/// 🚨️ Schema field/channel conversion, instance-read, and cardinality-parse failures.
#[derive(Clone, Debug, PartialEq)]
pub enum NeuralEngineError {
    /// 🔢️ Atom value doesn't match the scalar type a schema field declares.
    FieldTypeMismatch { kind: &'static str, reason: &'static str },
    /// 📦️ Field value isn't a dictionary where a list/schema/any field required one.
    FieldNotDictionary,
    /// 🕳️ Channel value is null where a value was required.
    ChannelNull,
    /// 📦️ Channel value isn't the wrapping dictionary its scalar type expects.
    ChannelTypeMismatch { kind: &'static str },
    /// 📦️ Channel value isn't a dictionary where a list/schema/any channel required one.
    ChannelNotDictionary,
    /// 🕳️ Channel dictionary is missing its `value` entry.
    ChannelMissingValue { kind: &'static str },
    /// 🔍️ A schema instance or field channel is absent from the input dictionary.
    Missing(String),
    /// ⚠️ A schema instance dictionary carries the wrong `$schema` tag.
    Invalid(String),
    /// 🔍️ A declared schema field is absent from the constructed dictionary.
    MissingField(String),
    /// 🚫️ Neither an instance nor any field inputs were provided to a schema component.
    NoInputProvided,
    /// 🔢️ A cardinality symbol string didn't parse to a known cardinality.
    InvalidCardinality(String),
    /// 🧬️ The constructed/modified dictionary failed schema validation.
    Validation(EvalError),
}

impl std::fmt::Display for NeuralEngineError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FieldTypeMismatch { kind, reason } => write!(formatter, "{kind} field is not {reason}"),
            Self::FieldNotDictionary => formatter.write_str("field is not a dictionary"),
            Self::ChannelNull => formatter.write_str("channel value is null"),
            Self::ChannelTypeMismatch { kind } => write!(formatter, "{kind} channel is not a dictionary"),
            Self::ChannelNotDictionary => formatter.write_str("channel is not a dictionary"),
            Self::ChannelMissingValue { kind } => write!(formatter, "{kind} channel is missing value"),
            Self::Missing(detail) => write!(formatter, "missing {detail}"),
            Self::Invalid(detail) => write!(formatter, "invalid {detail}"),
            Self::MissingField(field) => write!(formatter, "missing field {field}"),
            Self::NoInputProvided => formatter.write_str("no instance or field inputs provided"),
            Self::InvalidCardinality(value) => write!(formatter, "invalid cardinality: {value}"),
            Self::Validation(error) => std::fmt::Display::fmt(error, formatter),
        }
    }
}

impl std::error::Error for NeuralEngineError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Validation(error) => Some(error),
            _ => None,
        }
    }
}

impl From<EvalError> for NeuralEngineError {
    fn from(error: EvalError) -> Self {
        Self::Validation(error)
    }
}
//#endregion ⚠️ Errors
// #region 🔖️SchemaComponent

/// 🔢️ The port type every numeric channel carries — one `Dictionary::with_schema("number")`.
pub const VALUE_TYPE_NUMBER: &str = "number";

/// 🔤️ The port type every text channel carries.
pub const VALUE_TYPE_TEXT: &str = "text";

/// ☑️ The port type every boolean channel carries.
pub const VALUE_TYPE_BOOLEAN: &str = "boolean";

/// 📃️ The port type every list channel carries — `Dictionary::with_schema("list")`, whatever the items are.
pub const VALUE_TYPE_LIST: &str = "list";

/// 🧭️ The port type a `math` vector carries.
pub const VALUE_TYPE_VECTOR: &str = "vector";

/// 📍️ The port type a `math` point carries.
pub const VALUE_TYPE_POINT: &str = "point";

/// 🔷️ The port type every brep kernel handle carries — `Dictionary::with_schema("geometry")`, whether
/// the handle is a vertex, a wire, a face, a surface, a solid or a compound.
pub const VALUE_TYPE_GEOMETRY: &str = "geometry";
// #endregion 🔖️SchemaComponent
