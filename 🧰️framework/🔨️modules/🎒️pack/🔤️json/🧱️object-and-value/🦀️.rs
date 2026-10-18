//! 🧱️ JSON object and value: the ordered object, the value enum, its indexing, comparisons and conversions, and the order-insensitive equality. See [`super`].

use super::*;

//#region 🔖️Value
/// 🗂️ An insertion-order-preserving JSON object. Re-inserting an existing key overwrites
/// its value in place (last-value-wins) rather than moving it to the end — the same externally
/// observable behaviour as `serde_json::Map`'s default `BTreeMap` backing, just order-preserving
/// for the common no-duplicate case instead of key-sorted.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Object(pub(super) Vec<(String, Value)>);

impl Object {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.0.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        self.0.iter_mut().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    pub fn insert(&mut self, key: impl Into<String>, value: Value) -> Option<Value> {
        let key = key.into();
        if let Some(slot) = self.0.iter_mut().find(|(k, _)| *k == key) {
            return Some(std::mem::replace(&mut slot.1, value));
        }
        self.0.push((key, value));
        None
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// 🗑️ Removes `key` and returns its value, preserving the insertion order of the rest.
    ///
    /// Mirrors `serde_json::Map::remove` so a recursive key-at-a-time retirement walk can be
    /// expressed against this type without reaching for serde_json.
    pub fn remove(&mut self, key: &str) -> Option<Value> {
        let index = self.0.iter().position(|(name, _)| name == key)?;
        Some(self.0.remove(index).1)
    }

    /// ✏️ Mutable iteration in insertion order, for in-place rewrites of nested values.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&str, &mut Value)> {
        self.0.iter_mut().map(|(name, value)| (name.as_str(), value))
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &Value)> {
        self.0.iter().map(|(k, v)| (k.as_str(), v))
    }

    /// 🪪️ Borrows the last owned member and its actual key allocation before a bounded retirement grant.
    pub fn last_member(&self) -> Option<(&String, &Value)> {
        self.0.last().map(|(name, value)| (name, value))
    }

    /// 🌿️ Borrows the last descendant without transferring or cloning its owner.
    pub fn last_member_mut(&mut self) -> Option<(&str, &mut Value)> {
        self.0.last_mut().map(|(name, value)| (name.as_str(), value))
    }

    /// 🫴️ Transfers the last member's real key and value allocations in constant time.
    pub fn pop_member(&mut self) -> Option<(String, Value)> {
        self.0.pop()
    }

    /// 📏️ The actual retained member-vector backing, excluding independently owned descendants.
    pub fn member_storage_bytes(&self) -> usize {
        self.0.capacity().saturating_mul(size_of::<(String, Value)>())
    }

    /// 🫴️ Transfers the actual owned member storage in insertion order without copying keys or descendants.
    pub fn into_entries(self) -> Vec<(String, Value)> {
        self.0
    }
}

impl<'a> IntoIterator for &'a Object {
    type Item = (&'a str, &'a Value);
    type IntoIter = Box<dyn Iterator<Item = (&'a str, &'a Value)> + 'a>;
    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.iter())
    }
}

impl FromIterator<(String, Value)> for Object {
    fn from_iter<T: IntoIterator<Item = (String, Value)>>(iter: T) -> Self {
        let mut object = Object::new();
        for (key, value) in iter {
            object.insert(key, value);
        }
        object
    }
}

/// 🌳️ An owned JSON value tree — the `serde_json::Value` replacement. Every framework
/// consumer of dynamically-shaped JSON (schema leaves, protocol probes) reads/writes this type.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(Object),
}

impl Value {
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(v) => Some(v.as_str()),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<&Number> {
        match self {
            Value::Number(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        self.as_number().map(Number::as_f64)
    }

    pub fn as_i64(&self) -> Option<i64> {
        self.as_number().and_then(Number::as_i64)
    }

    pub fn as_u64(&self) -> Option<u64> {
        self.as_number().and_then(Number::as_u64)
    }

    /// 🔎️ `serde_json::Value::as_array`'s own signature (`Option<&Vec<Value>>`, not a bare slice) —
    /// on purpose: `Vec<Value>: Clone` lets `.and_then(Value::as_array).cloned()` call sites that
    /// used to target `serde_json::Value` keep compiling unchanged (`[Value]` alone is unsized and
    /// has no `Clone`).
    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Array(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Value>> {
        match self {
            Value::Array(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&Object> {
        match self {
            Value::Object(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_object_mut(&mut self) -> Option<&mut Object> {
        match self {
            Value::Object(v) => Some(v),
            _ => None,
        }
    }

    /// 🔎️ Object-field lookup, mirroring `serde_json::Value::get` — `None` on any non-object.
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.as_object().and_then(|object| object.get(key))
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        self.as_object_mut().and_then(|object| object.get_mut(key))
    }

    /// 🔎️ Array-element lookup by position, mirroring `serde_json::Value::get(usize)` —
    /// `None` on any non-array or out-of-bounds index.
    pub fn get_index(&self, index: usize) -> Option<&Value> {
        self.as_array().and_then(|array| array.get(index))
    }

    /// 🧭️ RFC 6901 JSON Pointer lookup, mirroring `serde_json::Value::pointer` — the empty string
    /// resolves to `self`; a non-empty pointer must start with `/`, and each `/`-separated segment
    /// is unescaped (`~1` -> `/`, `~0` -> `~`) before being tried as an object key, then as an
    /// array index. `None` on a malformed pointer, a missing key, or an out-of-range index.
    pub fn pointer(&self, pointer: &str) -> Option<&Value> {
        if pointer.is_empty() {
            return Some(self);
        }
        if !pointer.starts_with('/') {
            return None;
        }
        pointer.split('/').skip(1).try_fold(self, |current, raw_segment| {
            let segment = raw_segment.replace("~1", "/").replace("~0", "~");
            match current {
                Value::Object(_) => current.get(&segment),
                Value::Array(_) => segment.parse::<usize>().ok().and_then(|index| current.get_index(index)),
                _ => None,
            }
        })
    }
}

/// 🗝️ `value["key"]`, mirroring `serde_json::Value`'s own `Index<&str>` — panics if `self` is
/// not an object, returns [`Value::Null`] for a missing key (never panics on a missing key,
/// matching `serde_json`'s own permissive lookup semantics for assertions/fixtures).
impl std::ops::Index<&str> for Value {
    type Output = Value;
    fn index(&self, key: &str) -> &Value {
        static NULL: Value = Value::Null;
        match self.get(key) {
            Some(value) => value,
            None => &NULL,
        }
    }
}

/// 🗝️ `value[index]`, mirroring `serde_json::Value`'s own `Index<usize>` — panics if `self` is
/// not an array, returns [`Value::Null`] for an out-of-bounds index.
impl std::ops::Index<usize> for Value {
    type Output = Value;
    fn index(&self, index: usize) -> &Value {
        static NULL: Value = Value::Null;
        match self.get_index(index) {
            Some(value) => value,
            None => &NULL,
        }
    }
}

/// 🪞️ Cross-type equality against Rust primitives, mirroring `serde_json::Value`'s own
/// `impl_value_eq!` family — lets `assert_eq!(value["key"], "literal")`/`value == 3.0` read
/// exactly like the `serde_json` call sites they replace, with no `.as_str()`/`.as_f64()` unwrap
/// noise at the assertion site.
macro_rules! impl_value_partial_eq {
    ($($ty:ty => $variant_check:expr),+ $(,)?) => {
        $(
            impl PartialEq<$ty> for Value {
                fn eq(&self, other: &$ty) -> bool {
                    ($variant_check)(self, other)
                }
            }
            impl PartialEq<Value> for $ty {
                fn eq(&self, other: &Value) -> bool {
                    ($variant_check)(other, self)
                }
            }
        )+
    };
}

impl_value_partial_eq! {
    str => |value: &Value, other: &str| value.as_str() == Some(other),
    String => |value: &Value, other: &String| value.as_str() == Some(other.as_str()),
    bool => |value: &Value, other: &bool| value.as_bool() == Some(*other),
    f64 => |value: &Value, other: &f64| value.as_f64() == Some(*other),
    i64 => |value: &Value, other: &i64| value.as_i64() == Some(*other),
    u64 => |value: &Value, other: &u64| value.as_u64() == Some(*other),
    i32 => |value: &Value, other: &i32| value.as_i64() == Some(*other as i64),
    u32 => |value: &Value, other: &u32| value.as_u64() == Some(*other as u64),
    usize => |value: &Value, other: &usize| value.as_u64() == Some(*other as u64),
}

impl PartialEq<&str> for Value {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == Some(*other)
    }
}

impl PartialEq<Value> for &str {
    fn eq(&self, other: &Value) -> bool {
        other.as_str() == Some(*self)
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Value::Bool(v)
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value::String(v.to_string())
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Value::String(v)
    }
}

impl From<u64> for Value {
    fn from(v: u64) -> Self {
        Value::Number(Number::UInt(v))
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Value::Number(Number::Int(v))
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::Number(Number::Float(v))
    }
}

impl From<u32> for Value {
    fn from(v: u32) -> Self {
        Value::Number(Number::UInt(v as u64))
    }
}

impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Value::Number(Number::Int(v as i64))
    }
}

impl From<usize> for Value {
    fn from(v: usize) -> Self {
        Value::Number(Number::UInt(v as u64))
    }
}

impl From<i8> for Value {
    fn from(v: i8) -> Self {
        Value::Number(Number::Int(v as i64))
    }
}

impl From<Vec<Value>> for Value {
    fn from(v: Vec<Value>) -> Self {
        Value::Array(v)
    }
}

impl From<Object> for Value {
    fn from(v: Object) -> Self {
        Value::Object(v)
    }
}

impl<T: Into<Value>> From<Option<T>> for Value {
    fn from(v: Option<T>) -> Self {
        match v {
            Some(inner) => inner.into(),
            None => Value::Null,
        }
    }
}

/// 🏗️ Builds a [`Value::Array`] from any iterator of values.
pub fn array(items: impl IntoIterator<Item = Value>) -> Value {
    Value::Array(items.into_iter().collect())
}

/// 🏗️ Builds a [`Value::Object`] from any iterator of `(key, value)` pairs.
pub fn object(pairs: impl IntoIterator<Item = (String, Value)>) -> Value {
    Value::Object(pairs.into_iter().collect())
}

/// ⚖️ Structural equality that ignores object key order — [`Object`]'s derived `PartialEq`
/// (insertion-order `Vec`-backed) is order-sensitive, unlike `serde_json::Map`'s default
/// key-sorted `BTreeMap` backing; a decode→re-encode round trip through [`crate::json::to_json_string`]
/// naturally reorders fields to Rust struct declaration order, so a "committed JSON is already
/// canonical" style assertion needs this, not `==`, to match the old serde-era test semantics.
pub fn value_eq_ignoring_object_order(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Object(x), Value::Object(y)) => {
            x.len() == y.len() && x.iter().all(|(key, value)| y.get(key).is_some_and(|other| value_eq_ignoring_object_order(value, other)))
        }
        (Value::Array(x), Value::Array(y)) => x.len() == y.len() && x.iter().zip(y.iter()).all(|(l, r)| value_eq_ignoring_object_order(l, r)),
        _ => a == b,
    }
}
//#endregion 🔖️Value
