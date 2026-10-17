//! 📚️ Dictionary: the immutable key-value dictionary with its value and atom vocabulary.
//!
//! Holds the `🔖️Dictionary` region of the module; the root re-exports its items.

use super::*;

// #region 🔖️Dictionary
/// 📚️ Immutable, unordered, collision-free key-value collection. `serde` is TEST-ONLY
/// (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/01, tenth-seam pass — see
/// `📓️orderedmap-tenth-seam.md`): production now routes through the hand-written `ToValue`/
/// `FromValue` below, mirroring `OrderedMap<V>`'s own `#[cfg(test)]`-gated `Serialize` in
/// `🌱️value/🗂️ordered/🦀️.rs`. All three former blockers moved off serde in this pass — the `Value`
/// enum's derive, `Neuron`'s derive, and the `serde_json::to_string(&merged)` call in the
/// evaluator's pending-extension branch (now `pack::json::to_json_string(&merged.to_value())`).
/// No `#[derive(Serialize)]` here (not even test-gated): `OrderedMap<Value>`'s own `Serialize` is
/// `#[cfg(test)]`-gated inside `replication`'s OWN compilation unit — cfg(test) never crosses a
/// crate boundary, so it stays invisible when `replication` is pulled in as this crate's ordinary
/// (non-test) dependency, even while THIS crate's tests run. The oracle `Serialize` below is
/// hand-written instead, over `Value: Serialize` (local to this crate, genuinely test-cfg'd here).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Dictionary {
    #[doc(hidden)] pub pairs: OrderedMap<Value>,
}

impl Dictionary {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_schema(schema: impl Into<String>) -> Self {
        Self::new().insert(SCHEMA_KEY, Value::Atom(Atom::String(schema.into())))
    }

    /// 🧊️ Explicit synchronous dictionary construction; retained callers use immutable sharing and typed update cursors.
    pub fn insert(self, key: impl Into<String>, value: Value) -> Self {
        let mut builder = ColdDictionaryBuilder::from_dictionary(self);
        builder.insert(key.into(), value);
        builder.finish()
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.pairs.get(key)
    }
    /// 🪪️ Borrows the exact immutable source root without issuing another ownership alias.
    pub fn original_root_identity(&self)->Option<usize>{self.pairs.original_root_identity()}

    /// 🔗️ Retains only the exact immutable value of one ranked entry.
    pub fn value_shared_at_rank(&self,index:usize)->Option<semio_framework_value::ordered::SharedOwner<Value>> {self.pairs.value_shared_at_rank(index)}

    /// 📍️ Borrows an original ranked entry from the existing ordered dictionary owner.
    pub fn entry_at_rank(&self,index:usize)->Option<(&String,&Value)> {self.pairs.entry_at_rank(index)}

    pub fn schema(&self) -> Option<&str> {
        self.get(SCHEMA_KEY).and_then(|v| v.as_atom()).and_then(|a| a.as_str())
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.pairs.keys()
    }

    /// 🔎️ Borrows ordered entries without cloning nested values.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = (&String, &Value)> + ExactSizeIterator {
        self.pairs.iter()
    }

    /// 📤️ Moves exact dictionary ownership into nested byte-aware retirement without cloning values.
    pub fn into_retirement(self) -> ValueRetirement { ValueRetirement::from_dictionary(self) }

    /// 🧬️ Borrows the ordered owner so the budgeted merge hands shared roots over without cloning values.
    pub fn pairs(&self) -> &OrderedMap<Value> {
        &self.pairs
    }

    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }

    pub fn merge(&self, other: &Dictionary) -> Dictionary {
        let mut builder = ColdDictionaryBuilder::from_dictionary(self.clone());
        for (k, v) in &other.pairs {
            builder.insert(k.clone(), v.clone());
        }
        builder.finish()
    }
}

/// 🧊️ Test-only oracle mirror of the hand-written `Deserialize` below; manual (not derived) for the
/// same reason `Deserialize` is hand-written rather than delegating to `OrderedMap`'s own — see
/// `Dictionary`'s struct docstring above. Serializes each entry directly over `Value: Serialize`.
#[cfg(test)]
impl Serialize for Dictionary {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (key, value) in self.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}

/// 🧊️ Cold decoding stages every replacement and partial failure in a domain-aware builder.
/// 🔮️ Test-only — production decoding routes through `FromValue` below.
#[cfg(test)]
impl<'de> Deserialize<'de> for Dictionary {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Dictionary;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { formatter.write_str("a neural dictionary") }
            fn visit_map<A: serde::de::MapAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
                let mut builder = ColdDictionaryBuilder::new();
                while let Some((key, value)) = access.next_entry::<String, Value>()? { builder.insert(key, value); }
                Ok(builder.finish())
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}

/// 🔁️ First-party analog of the hand-written `Deserialize` above — routes through the same
/// `ColdDictionaryBuilder` retirement contract, never constructs `Dictionary` fields directly.
impl ToValue for Dictionary {
    fn to_value(&self) -> DslValue {
        DslValue::Object(self.iter().map(|(key, value)| (key.clone(), value.to_value())).collect())
    }
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
        control.scoped_depth(64, |control| control.scoped_stage(|control| {
            control.begin_stage(self.len())?;
            let mut entries = Vec::<(String, DslValue)>::guard_decoded(control.allocate_vec(self.len())?);
            for (key, value) in self.iter() {
                entries.get_mut().push((control.copy_text(key)?, value.to_value_controlled(control)?));
                control.step()?;
            }
            Ok(DslValue::Object(entries.take()))
        }))
    }
}

impl semio_framework_pack_json::JsonWriteSource for Dictionary {
    fn node_at_path(&self,path:&[usize])->Result<semio_framework_pack_json::JsonWriteNode<'_>,ValueError> {
        use semio_framework_pack_json::JsonWriteNode;
        let mut dictionary=self;
        for (depth,index) in path.iter().enumerate() {
            let (_,value)=dictionary.entry_at_rank(*index).ok_or_else(||ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"JSON dictionary path is absent"))?;
            if depth+1==path.len() {return Ok(match value {Value::Dictionary(value)=>JsonWriteNode::Object(value.len()),Value::Atom(Atom::Null)=>JsonWriteNode::Null,Value::Atom(Atom::Boolean(value))=>JsonWriteNode::Bool(*value),Value::Atom(Atom::Integer(value))=>JsonWriteNode::Number(Number::Int(*value)),Value::Atom(Atom::Decimal(value))=>JsonWriteNode::Number(Number::Float(*value)),Value::Atom(Atom::String(value))=>JsonWriteNode::String(value)});}
            dictionary=value.as_dictionary().ok_or_else(||ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"JSON dictionary path crosses a scalar"))?;
        }
        Ok(JsonWriteNode::Object(dictionary.len()))
    }
    fn object_key_at_path(&self,path:&[usize],index:usize)->Result<&str,ValueError> {
        let mut dictionary=self;
        for position in path {dictionary=dictionary.entry_at_rank(*position).and_then(|(_,value)|value.as_dictionary()).ok_or_else(||ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"JSON dictionary member path is absent"))?;}
        dictionary.entry_at_rank(index).map(|(key,_)|key.as_str()).ok_or_else(||ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"JSON dictionary member is absent"))
    }
}

impl FromValue for Dictionary {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let mut input=retirement::RetainedDictionaryInput::new(value);
        loop {
            let copy=4096.max(input.next_copy_byte_demand().expect("cold dictionary input copy demand"));
            let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:input.next_capacity_byte_demand(copy).expect("cold dictionary input capacity demand"),maximum_release_bytes:input.next_release_byte_demand().expect("cold dictionary input release demand"),maximum_depth:input.next_depth_demand().expect("cold dictionary input depth demand")};
            match input.step(grant) {
                Ok(retirement::DictionaryInputStep {dictionary:Some(dictionary),..})=>return Ok(dictionary),
                Ok(_)=>{},
                Err(error)=>{
                    input.cancel();
                    while !input.terminal_is_empty() {
                        let copy=4096.max(input.next_close_copy_byte_demand().expect("cold input close copy demand"));
                        let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:input.next_close_capacity_byte_demand(copy).expect("cold input close capacity demand"),maximum_release_bytes:input.next_close_release_byte_demand().expect("cold input close release demand"),maximum_depth:input.next_close_depth_demand().expect("cold input close depth demand")};
                        input.close_step(grant).expect("cold dictionary input closure");
                    }
                    return Err(error);
                },
            }
        }
    }
    fn from_value_controlled(value: &DslValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        let DslValue::Object(entries) = value else { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object for Dictionary")); };
        control.scoped_depth(64, |control| control.scoped_stage(|control| {
            control.begin_stage(entries.len())?;
            let mut builder = ColdDictionaryBuilder::new();
            for (key, value) in entries {
                let key = control.copy_text(key)?;
                let value = Value::from_value_controlled(value, control)?;
                builder.insert_controlled(key, value, control)?;
                control.step()?;
            }
            Ok(builder.finish())
        }))
    }
    fn default_value_controlled(control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        control.scoped_stage(|control| { control.begin_stage(1)?; control.step()?; Ok(Self::new()) })
    }
    fn retire_decoded(self) { self.retire_cold(); }
}

/// 🔑️ Dot-separated camelCase segment path.
pub type Key = String;

/// 💎️ Atom or nested dictionary. `serde` is TEST-ONLY — see `Dictionary`'s docstring above.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(untagged))]
pub enum Value {
    Atom(Atom),
    Dictionary(Dictionary),
}

impl Value {
    pub fn null() -> Self {
        Self::Atom(Atom::Null)
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Self::Atom(Atom::Null))
    }

    /// 🔎️ Borrows schema classification from this evaluator-owned value.
    pub fn kind(&self) -> ValueKind<'_> {
        match self {
            Value::Atom(Atom::Null) => ValueKind::Null,
            Value::Atom(Atom::Boolean(_)) => ValueKind::Boolean,
            Value::Atom(Atom::Integer(_)) => ValueKind::Integer,
            Value::Atom(Atom::Decimal(_)) => ValueKind::Decimal,
            Value::Atom(Atom::String(_)) => ValueKind::Text,
            Value::Dictionary(dictionary) => ValueKind::Dictionary(dictionary.schema()),
        }
    }

    pub fn as_atom(&self) -> Option<&Atom> {
        match self {
            Value::Atom(a) => Some(a),
            _ => None,
        }
    }

    pub fn as_dictionary(&self) -> Option<&Dictionary> {
        match self {
            Value::Dictionary(d) => Some(d),
            _ => None,
        }
    }
}

/// 🔁️ Mirrors `#[serde(untagged)]` above: an object decodes as `Dictionary`, anything else as `Atom`.
impl ToValue for Value {
    fn to_value(&self) -> DslValue {
        match self {
            Value::Atom(atom) => atom.to_value(),
            Value::Dictionary(dictionary) => dictionary.to_value(),
        }
    }
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
        match self { Self::Atom(value) => value.to_value_controlled(control), Self::Dictionary(value) => value.to_value_controlled(control) }
    }
}

impl FromValue for Value {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            object @ DslValue::Object(_) => Ok(Value::Dictionary(Dictionary::from_value(object)?)),
            other => Ok(Value::Atom(Atom::from_value(other)?)),
        }
    }
    fn from_value_controlled(value: &DslValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        match value { DslValue::Object(_) => Dictionary::from_value_controlled(value, control).map(Self::Dictionary), _ => Atom::from_value_controlled(value, control).map(Self::Atom) }
    }
    fn retire_decoded(self) { self.retire_cold(); }
}

/// ⚛️ Immutable non-dictionary value. `serde` is TEST-ONLY — see `Dictionary`'s docstring above.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(untagged))]
pub enum Atom {
    Null,
    Boolean(bool),
    Integer(i64),
    Decimal(f64),
    String(String),
}

impl Atom {
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Atom::Boolean(b) => Some(*b),
            Atom::Integer(i) => Some(*i != 0),
            Atom::Decimal(d) => Some(*d != 0.0),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Atom::Boolean(b) => Some(if *b { 1.0 } else { 0.0 }),
            Atom::Integer(i) => Some(*i as f64),
            Atom::Decimal(d) => Some(*d),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Atom::String(s) => Some(s),
            _ => None,
        }
    }
}

/// 🔁️ `DslValue::Number` carries its own exact variant, so each one maps to exactly one `Atom`:
/// `Int`/`UInt` to `Atom::Integer` and `Float` to `Atom::Decimal`.
///
/// 🐛️ `Float` used to take a whole-valued shortcut back to `Integer`, which made the round trip
/// LOSSY at the one hop that is supposed to preserve the carrier: `Atom::Decimal(42.0)` prints as
/// `42.0` (`pack::json`'s `write_float`), parses back as `Number::Float(42.0)` — the JSON layer
/// distinguishes `42` from `42.0` by the text precisely so it does not have to guess — and then
/// became `Atom::Integer(42)` here. Every `flowEvalResolve` seeds a node cache from exactly this
/// JSON (`seed_flow_eval_node_cache`), so a decimal output came back from an extension hop as an
/// integer and compared unequal to the value the kernel had produced
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
impl ToValue for Atom {
    fn to_value(&self) -> DslValue {
        match self {
            Atom::Null => DslValue::Null,
            Atom::Boolean(b) => DslValue::Bool(*b),
            Atom::Integer(i) => DslValue::Number(Number::Int(*i)),
            Atom::Decimal(d) => DslValue::Number(Number::Float(*d)),
            Atom::String(s) => DslValue::String(s.clone()),
        }
    }
    fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
        match self { Self::Null => ().to_value_controlled(control), Self::Boolean(value) => value.to_value_controlled(control), Self::Integer(value) => value.to_value_controlled(control), Self::Decimal(value) => value.to_value_controlled(control), Self::String(value) => value.to_value_controlled(control) }
    }
}

impl FromValue for Atom {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::Null => Ok(Atom::Null),
            DslValue::Bool(b) => Ok(Atom::Boolean(b)),
            DslValue::Number(Number::Int(value)) => Ok(Atom::Integer(value)),
            DslValue::Number(Number::UInt(value)) => i64::try_from(value).map(Atom::Integer).map_err(|_| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "unsigned atom exceeds the signed integer range")),
            DslValue::Number(Number::Float(value)) => Ok(Atom::Decimal(value)),
            DslValue::String(s) => Ok(Atom::String(s)),
            DslValue::Bytes(_) => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an atom, found bytes")),
            DslValue::Array(_) | DslValue::Object(_) => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an atom, found an array or object")),
        }
    }
    fn from_value_controlled(value: &DslValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        match value {
            DslValue::Null => <()>::from_value_controlled(value, control).map(|()| Self::Null),
            DslValue::Bool(_) => bool::from_value_controlled(value, control).map(Self::Boolean),
            DslValue::Number(Number::Int(_) | Number::UInt(_)) => i64::from_value_controlled(value, control).map(Self::Integer),
            DslValue::Number(Number::Float(_)) => f64::from_value_controlled(value, control).map(Self::Decimal),
            DslValue::String(_) => String::from_value_controlled(value, control).map(Self::String),
            _ => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an atom")),
        }
    }
}

/// 🌲️ Canonical projection follows the `ToValue` wire shape: dictionaries are rank-ordered objects, atoms are JSON scalars.
impl semio_framework_pack_json::ArtifactCanonicalJsonTree for Atom {
    fn canonical_tree_node(&self) -> Result<semio_framework_pack_json::ArtifactCanonicalJsonNode<'_>, ValueError> {
        use semio_framework_pack_json::ArtifactCanonicalJsonNode as Node;
        Ok(match self { Atom::Null => Node::Null, Atom::Boolean(value) => Node::Bool(*value), Atom::Integer(value) => Node::I64(*value), Atom::Decimal(value) => Node::F64(*value), Atom::String(value) => Node::String(value) })
    }
}

impl semio_framework_pack_json::ArtifactCanonicalJsonTree for Dictionary {
    fn canonical_tree_node(&self) -> Result<semio_framework_pack_json::ArtifactCanonicalJsonNode<'_>, ValueError> { Ok(semio_framework_pack_json::ArtifactCanonicalJsonNode::Object(self.len())) }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn semio_framework_pack_json::ArtifactCanonicalJsonTree, ValueError> {
        self.entry_at_rank(ordinal).map(|(_, value)| value as &dyn semio_framework_pack_json::ArtifactCanonicalJsonTree).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "canonical neural dictionary ordinal is absent"))
    }
    fn canonical_tree_key(&self, ordinal: usize) -> Result<semio_framework_pack_json::ArtifactCanonicalJsonText<'_>, ValueError> {
        self.entry_at_rank(ordinal).map(|(key, _)| key.as_str().into()).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "canonical neural dictionary key is absent"))
    }
}

impl semio_framework_pack_json::ArtifactCanonicalJsonTree for Value {
    fn canonical_tree_node(&self) -> Result<semio_framework_pack_json::ArtifactCanonicalJsonNode<'_>, ValueError> {
        use semio_framework_pack_json::ArtifactCanonicalJsonTree as CanonicalTree;
        match self { Value::Atom(value) => CanonicalTree::canonical_tree_node(value), Value::Dictionary(value) => CanonicalTree::canonical_tree_node(value) }
    }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn semio_framework_pack_json::ArtifactCanonicalJsonTree, ValueError> {
        use semio_framework_pack_json::ArtifactCanonicalJsonTree as CanonicalTree;
        match self { Value::Atom(value) => CanonicalTree::canonical_tree_child(value, ordinal), Value::Dictionary(value) => CanonicalTree::canonical_tree_child(value, ordinal) }
    }
    fn canonical_tree_key(&self, ordinal: usize) -> Result<semio_framework_pack_json::ArtifactCanonicalJsonText<'_>, ValueError> {
        use semio_framework_pack_json::ArtifactCanonicalJsonTree as CanonicalTree;
        match self { Value::Atom(value) => CanonicalTree::canonical_tree_key(value, ordinal), Value::Dictionary(value) => CanonicalTree::canonical_tree_key(value, ordinal) }
    }
}
// #endregion 🔖️Dictionary
