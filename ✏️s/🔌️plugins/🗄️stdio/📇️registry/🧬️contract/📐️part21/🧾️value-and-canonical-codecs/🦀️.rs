//! 🧾️ Value and canonical-tree codecs of the Part-21 model: the value projections of the header, document, value and instance and the canonical JSON trees. See [`super`].

use super::*;

//#region 🔖️ValueCodec
/// 🔎️ Takes one member out of a Part-21 JSON record, naming the record and the member when it is missing.

impl FromValue for Part21Decimal{
 fn from_value_controlled(value:&DslValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,ValueError>{controlled::input::decode_decimal(value,control)}
 fn from_value(value:DslValue)->Result<Self,ValueError>{let mut entries=value.into_object()?;let negative=bool::from_value(part21_member(&mut entries,"decimal","negative")?)?;let scale=u32::from_value(part21_member(&mut entries,"decimal","scale")?)?;let exponent=entries.iter().position(|(key,_)|key=="exponent").map(|index|Option::<i32>::from_value(entries.swap_remove(index).1)).transpose()?.flatten();let coefficient=String::from_value(part21_member(&mut entries,"decimal","coefficient")?)?;part21_exhausted(&entries,"decimal")?;Ok(Self{negative,coefficient,scale,exponent})}
 fn edit_value_at_path(&mut self,path:&[&str],edit:ValueEdit)->Result<(),ValueError>{edit_through_value(self,path,edit)}
}

impl FromValue for Part21Header{
 fn from_value_controlled(value:&DslValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,ValueError>{controlled::input::decode_header(value,control)}
 fn from_value(value:DslValue)->Result<Self,ValueError>{let mut entries=value.into_object()?;let file_description=Vec::from_value(part21_member(&mut entries,"header","fileDescription")?)?;let file_name=Vec::from_value(part21_member(&mut entries,"header","fileName")?)?;let file_schema=Vec::from_value(part21_member(&mut entries,"header","fileSchema")?)?;part21_exhausted(&entries,"header")?;Ok(Self{file_description,file_name,file_schema})}
 fn edit_value_at_path(&mut self,path:&[&str],edit:ValueEdit)->Result<(),ValueError>{edit_through_value(self,path,edit)}
}

impl FromValue for Part21Document{
 fn from_value_controlled(value:&DslValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,ValueError>{controlled::input::decode_document(value,control)}
 fn from_value(value:DslValue)->Result<Self,ValueError>{let mut entries=value.into_object()?;let header=Part21Header::from_value(part21_member(&mut entries,"document","header")?)?;let instances=Vec::from_value(part21_member(&mut entries,"document","instances")?)?;part21_exhausted(&entries,"document")?;Ok(Self{header,instances})}
 fn edit_value_at_path(&mut self,path:&[&str],edit:ValueEdit)->Result<(),ValueError>{edit_through_value(self,path,edit)}
}

fn part21_member(entries: &mut Vec<(String, DslValue)>, record: &str, key: &str) -> Result<DslValue, ValueError> {
    let index = entries.iter().position(|(candidate, _)| candidate == key).ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("a Part-21 {record} carries `{key}`")))?;
    Ok(entries.remove(index).1)
}

/// 🚫️ Refuses a member a Part-21 JSON record does not declare.
fn part21_exhausted(entries: &[(String, DslValue)], record: &str) -> Result<(), ValueError> {
    entries.first().map_or(Ok(()), |(key, _)| Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("a Part-21 {record} carries no `{key}`"))))
}

/// 🌱️ The canonical JSON projection of a Part-21 value: kind-tagged, the payload under `value` (a scalar, a reference, a
/// decimal), `values` (a list) or `typeName` + `values` (a defined-type wrapper); `unset`/`derived` carry the tag alone.
impl ToValue for Part21Value {
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
        controlled::output::encode_value(self, control)
    }

    fn to_value(&self) -> DslValue {
        let tagged = |kind: &str, payload: Vec<(&str, DslValue)>| DslValue::object(std::iter::once(("kind".to_string(), DslValue::String(kind.to_string()))).chain(payload.into_iter().map(|(key, value)| (key.to_string(), value))));
        match self {
            Self::Ref(id) => tagged("ref", vec![("value", id.to_value())]),
            Self::Str(text) => tagged("str", vec![("value", text.to_value())]),
            Self::Enum(text) => tagged("enum", vec![("value", text.to_value())]),
            Self::Int(value) => tagged("int", vec![("value", value.to_value())]),
            Self::Real(decimal) => tagged("real", vec![("value", decimal.to_value())]),
            Self::List(items) => tagged("list", vec![("values", items.to_value())]),
            Self::Typed { name, items } => tagged("typed", vec![("typeName", name.to_value()), ("values", items.to_value())]),
            Self::Unset => tagged("unset", Vec::new()),
            Self::Derived => tagged("derived", Vec::new()),
        }
    }
}

/// 🔀️ Decodes exactly the projection [`ToValue`] emits; an unknown kind or member is refused.
impl FromValue for Part21Value {
    fn from_value_controlled(value: &DslValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        controlled::input::decode_value(value, control)
    }


    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let mut entries = value.into_object()?;
        let kind = String::from_value(part21_member(&mut entries, "value", "kind")?)?;
        let record = format!("{kind} value");
        let decoded = match kind.as_str() {
            "ref" => Self::Ref(u64::from_value(part21_member(&mut entries, &record, "value")?)?),
            "str" => Self::Str(String::from_value(part21_member(&mut entries, &record, "value")?)?),
            "enum" => Self::Enum(String::from_value(part21_member(&mut entries, &record, "value")?)?),
            "int" => Self::Int(i64::from_value(part21_member(&mut entries, &record, "value")?)?),
            "real" => Self::Real(Part21Decimal::from_value(part21_member(&mut entries, &record, "value")?)?),
            "list" => Self::List(Vec::from_value(part21_member(&mut entries, &record, "values")?)?),
            "typed" => Self::Typed { name: String::from_value(part21_member(&mut entries, &record, "typeName")?)?, items: Vec::from_value(part21_member(&mut entries, &record, "values")?)? },
            "unset" => Self::Unset,
            "derived" => Self::Derived,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown Part-21 value kind `{other}`"))),
        };
        part21_exhausted(&entries, &record)?;
        Ok(decoded)
    }

    fn edit_value_at_path(&mut self, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
        edit_through_value(self, path, edit)
    }
}

/// 🌱️ The canonical JSON projection of an instance: its `id` and its `entities` as `{typeName, arguments}` records — a
/// complex instance keeps every entity, in order.
impl ToValue for Part21Instance {
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
        controlled::output::encode_instance(self, control)
    }

    fn to_value(&self) -> DslValue {
        let entities = self.entities.iter().map(|(name, arguments)| DslValue::object([("typeName".to_string(), name.to_value()), ("arguments".to_string(), arguments.to_value())])).collect();
        DslValue::object([("id".to_string(), self.id.to_value()), ("entities".to_string(), DslValue::Array(entities))])
    }
}

/// 🔀️ Decodes exactly the projection [`ToValue`] emits; an unknown member is refused.
impl FromValue for Part21Instance {
    fn from_value_controlled(value: &DslValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        controlled::input::decode_instance(value, control)
    }


    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let mut entries = value.into_object()?;
        let id = u64::from_value(part21_member(&mut entries, "instance", "id")?)?;
        let DslValue::Array(records) = part21_member(&mut entries, "instance", "entities")? else {
            return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "a Part-21 instance carries its entities as an array"));
        };
        part21_exhausted(&entries, "instance")?;
        let entities = records
            .into_iter()
            .map(|record| {
                let mut entity = record.into_object()?;
                let name = String::from_value(part21_member(&mut entity, "entity", "typeName")?)?;
                let arguments = Vec::from_value(part21_member(&mut entity, "entity", "arguments")?)?;
                part21_exhausted(&entity, "entity")?;
                Ok((name, arguments))
            })
            .collect::<Result<Vec<_>, ValueError>>()?;
        Ok(Self { id, entities })
    }

    fn edit_value_at_path(&mut self, path: &[&str], edit: ValueEdit) -> Result<(), ValueError> {
        edit_through_value(self, path, edit)
    }
}
//#endregion 🔖️ValueCodec

//#region 🌲️CanonicalTree
static PART21_KINDS: [&str; 9] = ["ref", "str", "enum", "int", "real", "list", "typed", "unset", "derived"];

fn part21_tree_refusal(reason: &'static str) -> ValueError {
    ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, reason)
}

/// 🌲️ Projects the kind-tagged value exactly as [`ToValue`] emits it: `kind`, then `value`, `values` or `typeName` + `values`.
impl ArtifactCanonicalJsonTree for Part21Value {
    fn canonical_tree_node(&self) -> Result<ArtifactCanonicalJsonNode<'_>, ValueError> {
        Ok(ArtifactCanonicalJsonNode::Object(match self {
            Self::Unset | Self::Derived => 1,
            Self::Typed { .. } => 3,
            _ => 2,
        }))
    }

    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn ArtifactCanonicalJsonTree, ValueError> {
        let kind = match self {
            Self::Ref(_) => 0,
            Self::Str(_) => 1,
            Self::Enum(_) => 2,
            Self::Int(_) => 3,
            Self::Real(_) => 4,
            Self::List(_) => 5,
            Self::Typed { .. } => 6,
            Self::Unset => 7,
            Self::Derived => 8,
        };
        match (self, ordinal) {
            (_, 0) => Ok(&PART21_KINDS[kind]),
            (Self::Ref(id), 1) => Ok(id),
            (Self::Str(text) | Self::Enum(text), 1) => Ok(text),
            (Self::Int(value), 1) => Ok(value),
            (Self::Real(decimal), 1) => Ok(decimal),
            (Self::List(items), 1) => Ok(items),
            (Self::Typed { name, .. }, 1) => Ok(name),
            (Self::Typed { items, .. }, 2) => Ok(items),
            _ => Err(part21_tree_refusal("canonical Part-21 value ordinal is absent")),
        }
    }

    fn canonical_tree_key(&self, ordinal: usize) -> Result<ArtifactCanonicalJsonText<'_>, ValueError> {
        let key = match (self, ordinal) {
            (_, 0) => "kind",
            (Self::List(_), 1) => "values",
            (Self::Typed { .. }, 1) => "typeName",
            (Self::Typed { .. }, 2) => "values",
            (Self::Ref(_) | Self::Str(_) | Self::Enum(_) | Self::Int(_) | Self::Real(_), 1) => "value",
            _ => return Err(part21_tree_refusal("canonical Part-21 value key is absent")),
        };
        Ok(ArtifactCanonicalJsonText::from(key))
    }
}

#[repr(transparent)]
struct Part21Entity((String, Vec<Part21Value>));

#[repr(transparent)]
struct Part21Entities(Vec<(String, Vec<Part21Value>)>);

impl ArtifactCanonicalJsonTree for Part21Entity {
    fn canonical_tree_node(&self) -> Result<ArtifactCanonicalJsonNode<'_>, ValueError> {
        Ok(ArtifactCanonicalJsonNode::Object(2))
    }

    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn ArtifactCanonicalJsonTree, ValueError> {
        match ordinal {
            0 => Ok(&self.0 .0),
            1 => Ok(&self.0 .1),
            _ => Err(part21_tree_refusal("canonical Part-21 entity ordinal is absent")),
        }
    }

    fn canonical_tree_key(&self, ordinal: usize) -> Result<ArtifactCanonicalJsonText<'_>, ValueError> {
        match ordinal {
            0 => Ok(ArtifactCanonicalJsonText::from("typeName")),
            1 => Ok(ArtifactCanonicalJsonText::from("arguments")),
            _ => Err(part21_tree_refusal("canonical Part-21 entity key is absent")),
        }
    }
}

impl ArtifactCanonicalJsonTree for Part21Entities {
    fn canonical_tree_node(&self) -> Result<ArtifactCanonicalJsonNode<'_>, ValueError> {
        Ok(ArtifactCanonicalJsonNode::Array(self.0.len()))
    }

    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn ArtifactCanonicalJsonTree, ValueError> {
        let entity = self.0.get(ordinal).ok_or_else(|| part21_tree_refusal("canonical Part-21 entity list ordinal is absent"))?;
        // SAFETY: `Part21Entity` is `repr(transparent)` over exactly the borrowed tuple, so the reference keeps its layout and lifetime.
        Ok(unsafe { &*(entity as *const (String, Vec<Part21Value>) as *const Part21Entity) })
    }
}

/// 🌲️ Projects an instance as [`ToValue`] emits it: `id`, then `entities` as `{typeName, arguments}` records.
impl ArtifactCanonicalJsonTree for Part21Instance {
    fn canonical_tree_node(&self) -> Result<ArtifactCanonicalJsonNode<'_>, ValueError> {
        Ok(ArtifactCanonicalJsonNode::Object(2))
    }

    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn ArtifactCanonicalJsonTree, ValueError> {
        match ordinal {
            0 => Ok(&self.id),
            // SAFETY: `Part21Entities` is `repr(transparent)` over exactly the borrowed vector.
            1 => Ok(unsafe { &*(&self.entities as *const Vec<(String, Vec<Part21Value>)> as *const Part21Entities) }),
            _ => Err(part21_tree_refusal("canonical Part-21 instance ordinal is absent")),
        }
    }

    fn canonical_tree_key(&self, ordinal: usize) -> Result<ArtifactCanonicalJsonText<'_>, ValueError> {
        match ordinal {
            0 => Ok(ArtifactCanonicalJsonText::from("id")),
            1 => Ok(ArtifactCanonicalJsonText::from("entities")),
            _ => Err(part21_tree_refusal("canonical Part-21 instance key is absent")),
        }
    }
}
//#endregion 🌲️CanonicalTree
