//! 🧭️ Native ordinal canonical traversal retains real root-backed projections and paged frontier ownership.
use crate::{ArtifactCanonicalJsonNode as Node, ArtifactCanonicalJsonText, ArtifactCanonicalJsonScalarBytes as ScalarBytes, canonical_escaped_byte};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind, list::PagedList, paged::{PagedMap, PagedUtf8}, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetainedOwnedProjection}};
use std::mem::size_of;

fn refusal(reason: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvariantViolated, reason) }

/// 🌲️ Direct native child ordinals preserve original record fields and collection order.
pub trait ArtifactCanonicalJsonTree: Sync + 'static {
    fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError>;
    fn canonical_tree_child(&self, _ordinal: usize) -> Result<&dyn ArtifactCanonicalJsonTree, ValueError> { Err(refusal("canonical tree scalar has no child")) }
    fn canonical_tree_key(&self, _ordinal: usize) -> Result<ArtifactCanonicalJsonText<'_>, ValueError> { Err(refusal("canonical tree node has no key")) }
}

impl<const N: usize> ArtifactCanonicalJsonTree for PagedUtf8<N> {
    fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> { Ok(Node::Text(ArtifactCanonicalJsonText::Native(self))) }
}
impl<T: ArtifactCanonicalJsonTree, const N: usize> ArtifactCanonicalJsonTree for PagedList<T, N> {
    fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> { Ok(Node::Array(self.len())) }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn ArtifactCanonicalJsonTree, ValueError> { self.get(ordinal).map(|value| value as &dyn ArtifactCanonicalJsonTree).ok_or_else(|| refusal("canonical native array ordinal is absent")) }
}
impl<T: ArtifactCanonicalJsonTree, const N: usize> ArtifactCanonicalJsonTree for PagedMap<T, N> {
    fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> { Ok(Node::Object(self.len())) }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn ArtifactCanonicalJsonTree, ValueError> { self.entry_at(ordinal).map(|(_, value)| value as &dyn ArtifactCanonicalJsonTree).ok_or_else(|| refusal("canonical native object ordinal is absent")) }
    fn canonical_tree_key(&self, ordinal: usize) -> Result<ArtifactCanonicalJsonText<'_>, ValueError> { self.entry_at(ordinal).map(|(key, _)| ArtifactCanonicalJsonText::Native(key)).ok_or_else(|| refusal("canonical native object key is absent")) }
}
impl ArtifactCanonicalJsonTree for bool { fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> { Ok(Node::Bool(*self)) } }
/// 🔤️ A `u64` whose canonical JSON wire is its decimal string (`"18446744073709551615"`), for ids beyond 2^53.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct ArtifactCanonicalDecimalU64(pub u64);
impl ArtifactCanonicalDecimalU64 {
    /// 🔗️ Views a native `u64` field in place as its decimal-string role, so `#[canonical_json(decimal_string)]` projects the original field without a copy.
    pub const fn from_ref(value: &u64) -> &Self {
        unsafe { &*(value as *const u64).cast::<Self>() }
    }
}
impl From<u64> for ArtifactCanonicalDecimalU64 { fn from(value: u64) -> Self { Self(value) } }
impl From<ArtifactCanonicalDecimalU64> for u64 { fn from(value: ArtifactCanonicalDecimalU64) -> u64 { value.0 } }
impl ArtifactCanonicalJsonTree for ArtifactCanonicalDecimalU64 { fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> { Ok(Node::U64Text(self.0)) } }
semio_framework_value::artifact_retire_leaf!(ArtifactCanonicalDecimalU64);
impl semio_framework_value::retained_clone::RetainedClone for ArtifactCanonicalDecimalU64 {
    type Cursor = semio_framework_value::retained_clone::ScalarCursor<Self>;
    fn retained_clone_cursor() -> Self::Cursor { Default::default() }
}
impl semio_framework_value::ToValue for ArtifactCanonicalDecimalU64 { fn to_value(&self) -> semio_framework_value::DslValue { semio_framework_value::DslValue::String(self.0.to_string()) } }
impl semio_framework_value::FromValue for ArtifactCanonicalDecimalU64 {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, ValueError> {
        match value {
            semio_framework_value::DslValue::String(text) if !text.is_empty() && text.bytes().all(|byte| byte.is_ascii_digit()) && (text.len() == 1 || !text.starts_with('0')) => text.parse().map(Self).map_err(|_| ValueError::literal(ValueRefusalKind::InvalidValue, "canonical decimal u64 exceeds 64 bits")),
            _ => Err(ValueError::literal(ValueRefusalKind::InvalidValue, "canonical decimal u64 requires a canonical decimal string")),
        }
    }
}
/// 🔤️ A `i64` whose canonical JSON wire is its decimal string (`"-9223372036854775808"`), for exact signed64 values.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct ArtifactCanonicalDecimalI64(pub i64);
impl ArtifactCanonicalDecimalI64 {
    /// 🔗️ Views a native `i64` field in place as its decimal-string role, so `#[canonical_json(decimal_string)]` projects the original field without a copy.
    pub const fn from_ref(value: &i64) -> &Self {
        unsafe { &*(value as *const i64).cast::<Self>() }
    }
}
impl From<i64> for ArtifactCanonicalDecimalI64 { fn from(value: i64) -> Self { Self(value) } }
impl From<ArtifactCanonicalDecimalI64> for i64 { fn from(value: ArtifactCanonicalDecimalI64) -> i64 { value.0 } }
impl ArtifactCanonicalJsonTree for ArtifactCanonicalDecimalI64 { fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> { Ok(Node::I64Text(self.0)) } }
semio_framework_value::artifact_retire_leaf!(ArtifactCanonicalDecimalI64);
impl semio_framework_value::retained_clone::RetainedClone for ArtifactCanonicalDecimalI64 {
    type Cursor = semio_framework_value::retained_clone::ScalarCursor<Self>;
    fn retained_clone_cursor() -> Self::Cursor { Default::default() }
}
impl semio_framework_value::ToValue for ArtifactCanonicalDecimalI64 { fn to_value(&self) -> semio_framework_value::DslValue { semio_framework_value::DslValue::String(self.0.to_string()) } }
impl semio_framework_value::FromValue for ArtifactCanonicalDecimalI64 {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, ValueError> {
        match value {
            semio_framework_value::DslValue::String(text) if !text.is_empty() && text.bytes().all(|byte| byte.is_ascii_digit()) && (text.len() == 1 || !text.starts_with('0')) => text.parse().map(Self).map_err(|_| ValueError::literal(ValueRefusalKind::InvalidValue, "canonical decimal i64 is outside signed 64 bits")),
            _ => Err(ValueError::literal(ValueRefusalKind::InvalidValue, "canonical decimal i64 requires a canonical decimal string")),
        }
    }
}
/// 🔢️ An `f32` whose canonical JSON wire is its IEEE-754 bit pattern as eight lowercase hexadecimal digits in a string (`"3fc00000"`), for exact binary32 words.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(transparent)]
pub struct ArtifactCanonicalHexWordF32(pub f32);
impl ArtifactCanonicalHexWordF32 {
    /// 🔗️ Views a native `f32` field in place as its hex-word role, so `#[canonical_json(hex_word)]` projects the original field without a copy.
    pub const fn from_ref(value: &f32) -> &Self {
        unsafe { &*(value as *const f32).cast::<Self>() }
    }
}
impl From<f32> for ArtifactCanonicalHexWordF32 { fn from(value: f32) -> Self { Self(value) } }
impl From<ArtifactCanonicalHexWordF32> for f32 { fn from(value: ArtifactCanonicalHexWordF32) -> f32 { value.0 } }
impl ArtifactCanonicalJsonTree for ArtifactCanonicalHexWordF32 { fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> { Ok(Node::F32HexWord(self.0)) } }
semio_framework_value::artifact_retire_leaf!(ArtifactCanonicalHexWordF32);
impl semio_framework_value::retained_clone::RetainedClone for ArtifactCanonicalHexWordF32 {
    type Cursor = semio_framework_value::retained_clone::ScalarCursor<Self>;
    fn retained_clone_cursor() -> Self::Cursor { Default::default() }
}
impl semio_framework_value::ToValue for ArtifactCanonicalHexWordF32 { fn to_value(&self) -> semio_framework_value::DslValue { semio_framework_value::DslValue::String(format!("{:08x}", self.0.to_bits())) } }
impl semio_framework_value::FromValue for ArtifactCanonicalHexWordF32 {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, ValueError> {
        match value {
            semio_framework_value::DslValue::String(text) if text.len() == 8 && text.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)) => u32::from_str_radix(&text, 16).map(|bits| Self(f32::from_bits(bits))).map_err(|_| ValueError::literal(ValueRefusalKind::InvalidValue, "canonical hex word exceeds 32 bits")),
            _ => Err(ValueError::literal(ValueRefusalKind::InvalidValue, "canonical hex word requires eight lowercase hexadecimal digits")),
        }
    }
}
/// 🔢️ A fixed `[f32; N]` whose canonical JSON wire is an array of hex-word strings.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct ArtifactCanonicalHexWordArray<const N: usize>(pub [f32; N]);
impl<const N: usize> ArtifactCanonicalHexWordArray<N> {
    /// 🔗️ Views a native `[f32; N]` field in place as its hex-word array role.
    pub const fn from_ref(value: &[f32; N]) -> &Self {
        unsafe { &*(value as *const [f32; N]).cast::<Self>() }
    }
}
impl<const N: usize> ArtifactCanonicalJsonTree for ArtifactCanonicalHexWordArray<N> {
    fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> { Ok(Node::Array(N)) }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn ArtifactCanonicalJsonTree, ValueError> { self.0.get(ordinal).map(|value| ArtifactCanonicalHexWordF32::from_ref(value) as &dyn ArtifactCanonicalJsonTree).ok_or_else(|| refusal("canonical hex word array ordinal is absent")) }
}
/// 🔢️ A `Vec<f32>` whose canonical JSON wire is an array of hex-word strings.
#[derive(Clone, Debug, Default, PartialEq)]
#[repr(transparent)]
pub struct ArtifactCanonicalHexWordList(pub Vec<f32>);
impl ArtifactCanonicalHexWordList {
    /// 🔗️ Views a native `Vec<f32>` field in place as its hex-word list role.
    pub const fn from_ref(value: &Vec<f32>) -> &Self {
        unsafe { &*(value as *const Vec<f32>).cast::<Self>() }
    }
}
impl ArtifactCanonicalJsonTree for ArtifactCanonicalHexWordList {
    fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> { Ok(Node::Array(self.0.len())) }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn ArtifactCanonicalJsonTree, ValueError> { self.0.get(ordinal).map(|value| ArtifactCanonicalHexWordF32::from_ref(value) as &dyn ArtifactCanonicalJsonTree).ok_or_else(|| refusal("canonical hex word list ordinal is absent")) }
}
fn canonical_i64_text(text: &str) -> bool {
    let digits = text.strip_prefix('-').unwrap_or(text);
    !digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit()) && (digits.len() == 1 || !digits.starts_with('0')) && text != "-0"
}
impl ArtifactCanonicalJsonTree for u64 { fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> { Ok(Node::U64(*self)) } }
impl ArtifactCanonicalJsonTree for i64 { fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> { Ok(Node::I64(*self)) } }

impl ArtifactCanonicalJsonTree for String { fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::String(self))} }
impl ArtifactCanonicalJsonTree for semio_framework_value::SharedUtf8 {fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Text(ArtifactCanonicalJsonText::Native(self)))}}
impl ArtifactCanonicalJsonTree for &'static str { fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::String(self))} }
impl<T:ArtifactCanonicalJsonTree> ArtifactCanonicalJsonTree for Vec<T> {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Array(self.len()))}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn ArtifactCanonicalJsonTree,ValueError>{self.get(ordinal).map(|value|value as &dyn ArtifactCanonicalJsonTree).ok_or_else(||refusal("canonical original vector ordinal is absent"))}
}
impl<T:ArtifactCanonicalJsonTree,const N:usize> ArtifactCanonicalJsonTree for [T;N] {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Array(N))}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn ArtifactCanonicalJsonTree,ValueError>{self.get(ordinal).map(|value|value as &dyn ArtifactCanonicalJsonTree).ok_or_else(||refusal("canonical original fixed array ordinal is absent"))}
}
impl<T:ArtifactCanonicalJsonTree> ArtifactCanonicalJsonTree for Option<T> {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{self.as_ref().map_or(Ok(Node::Null),|value|value.canonical_tree_node())}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn ArtifactCanonicalJsonTree,ValueError>{self.as_ref().ok_or_else(||refusal("canonical absent optional field has no child"))?.canonical_tree_child(ordinal)}
 fn canonical_tree_key(&self,ordinal:usize)->Result<ArtifactCanonicalJsonText<'_>,ValueError>{self.as_ref().ok_or_else(||refusal("canonical absent optional field has no key"))?.canonical_tree_key(ordinal)}
}
impl<T:ArtifactCanonicalJsonTree> ArtifactCanonicalJsonTree for Box<T> {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{self.as_ref().canonical_tree_node()}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn ArtifactCanonicalJsonTree,ValueError>{self.as_ref().canonical_tree_child(ordinal)}
 fn canonical_tree_key(&self,ordinal:usize)->Result<ArtifactCanonicalJsonText<'_>,ValueError>{self.as_ref().canonical_tree_key(ordinal)}
}
impl ArtifactCanonicalJsonTree for f64 {fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::F64(*self))}}
impl ArtifactCanonicalJsonTree for i32 {fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::I64(i64::from(*self)))}}
impl ArtifactCanonicalJsonTree for usize {fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::U64(u64::try_from(*self).map_err(|_|refusal("canonical native ordinal exceeds unsigned64"))?))}}
impl ArtifactCanonicalJsonTree for u8 {fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::U64(u64::from(*self)))}}
impl ArtifactCanonicalJsonTree for u16 {fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::U64(u64::from(*self)))}}
impl ArtifactCanonicalJsonTree for u32 {fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::U64(u64::from(*self)))}}
impl ArtifactCanonicalJsonTree for u128 {fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::U128(*self))}}
impl ArtifactCanonicalJsonTree for i8 {fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::I64(i64::from(*self)))}}
impl ArtifactCanonicalJsonTree for i16 {fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::I64(i64::from(*self)))}}
impl ArtifactCanonicalJsonTree for i128 {fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::I128(*self))}}
impl ArtifactCanonicalJsonTree for isize {fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::I64(i64::try_from(*self).map_err(|_|refusal("canonical native ordinal exceeds signed64"))?))}}
impl ArtifactCanonicalJsonTree for f32 {fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::F32(*self))}}
static KIND_BOOLEAN:&str="boolean";static KIND_INTEGER:&str="integer";static KIND_DECIMAL:&str="decimal";static KIND_TEXT:&str="text";static KIND_LIST:&str="list";static KIND_SCHEMA:&str="schema";static KIND_ANY:&str="any";
impl ArtifactCanonicalJsonTree for semio_framework_value::ValueType {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{use semio_framework_value::ValueType;Ok(Node::Object(match self{ValueType::List(_)|ValueType::Schema(_)=>2,_=>1}))}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn ArtifactCanonicalJsonTree,ValueError>{
  use semio_framework_value::ValueType;
  match(ordinal,self){
   (0,ValueType::Boolean)=>Ok(&KIND_BOOLEAN),(0,ValueType::Integer)=>Ok(&KIND_INTEGER),(0,ValueType::Decimal)=>Ok(&KIND_DECIMAL),(0,ValueType::Text)=>Ok(&KIND_TEXT),(0,ValueType::List(_))=>Ok(&KIND_LIST),(0,ValueType::Schema(_))=>Ok(&KIND_SCHEMA),(0,ValueType::Any)=>Ok(&KIND_ANY),
   (1,ValueType::List(inner))=>Ok(inner.as_ref()),(1,ValueType::Schema(identity))=>Ok(identity),
   _=>Err(refusal("canonical value type ordinal is absent")),
  }
 }
 fn canonical_tree_key(&self,ordinal:usize)->Result<ArtifactCanonicalJsonText<'_>,ValueError>{match(ordinal,self){(0,_)=>Ok("kind".into()),(1,semio_framework_value::ValueType::List(_)|semio_framework_value::ValueType::Schema(_))=>Ok("of".into()),_=>Err(refusal("canonical value type key is absent"))}}
}
/// 🗝️ String-like ordered keys render as JSON object members in byte order; ordinal access walks the live ordered map.
impl<K:AsRef<str>+Ord+Sync+'static,V:ArtifactCanonicalJsonTree> ArtifactCanonicalJsonTree for std::collections::BTreeMap<K,V> {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Object(self.len()))}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn ArtifactCanonicalJsonTree,ValueError>{self.values().nth(ordinal).map(|value|value as&dyn ArtifactCanonicalJsonTree).ok_or_else(||refusal("canonical ordered map ordinal is absent"))}
 fn canonical_tree_key(&self,ordinal:usize)->Result<ArtifactCanonicalJsonText<'_>,ValueError>{self.keys().nth(ordinal).map(|key|ArtifactCanonicalJsonText::from(key.as_ref())).ok_or_else(||refusal("canonical ordered map key is absent"))}
}
/// 🗝️ Ordered immutable maps render as JSON objects in their own rank order, matching their `ToValue` wire; ordinal access is one bounded ranked visit.
impl<V:ArtifactCanonicalJsonTree+Send> ArtifactCanonicalJsonTree for semio_framework_value::ordered::OrderedMap<V> {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Object(self.len()))}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn ArtifactCanonicalJsonTree,ValueError>{self.entry_at_rank(ordinal).map(|(_,value)|value as&dyn ArtifactCanonicalJsonTree).ok_or_else(||refusal("canonical ordered root ordinal is absent"))}
 fn canonical_tree_key(&self,ordinal:usize)->Result<ArtifactCanonicalJsonText<'_>,ValueError>{self.entry_at_rank(ordinal).map(|(key,_)|ArtifactCanonicalJsonText::from(key.as_str())).ok_or_else(||refusal("canonical ordered root key is absent"))}
}
/// 🗝️ Ordered immutable string sets render as JSON string arrays in rank order, matching their `ToValue` wire.
impl ArtifactCanonicalJsonTree for semio_framework_value::ordered::OrderedSet {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Array(self.len()))}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn ArtifactCanonicalJsonTree,ValueError>{self.key_at_rank(ordinal).map(|key|key as&dyn ArtifactCanonicalJsonTree).ok_or_else(||refusal("canonical ordered set ordinal is absent"))}
}
fn hashed_entry<K:AsRef<str>+Ord,V,S>(map:&std::collections::HashMap<K,V,S>,ordinal:usize)->Option<(&K,&V)>{
 if ordinal>=map.len(){return None;}
 map.iter().find(|(candidate,_)|map.keys().filter(|other|other.as_ref()<candidate.as_ref()).count()==ordinal)
}
/// 🗝️ Hashed maps render in canonical byte-ordered key order, independent of their hasher, by ranking each key without allocating.
impl<K:AsRef<str>+Ord+std::hash::Hash+Eq+Sync+'static,V:ArtifactCanonicalJsonTree,S:Sync+'static> ArtifactCanonicalJsonTree for std::collections::HashMap<K,V,S> {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Object(self.len()))}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn ArtifactCanonicalJsonTree,ValueError>{hashed_entry(self,ordinal).map(|(_,value)|value as&dyn ArtifactCanonicalJsonTree).ok_or_else(||refusal("canonical hashed map ordinal is absent"))}
 fn canonical_tree_key(&self,ordinal:usize)->Result<ArtifactCanonicalJsonText<'_>,ValueError>{hashed_entry(self,ordinal).map(|(key,_)|ArtifactCanonicalJsonText::from(key.as_ref())).ok_or_else(||refusal("canonical hashed map key is absent"))}
}
macro_rules! canonical_tuple {
 ($length:literal;$($type:ident:$index:tt),+) => {
  impl<$($type:ArtifactCanonicalJsonTree),+> ArtifactCanonicalJsonTree for ($($type,)+) {
   fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Array($length))}
   fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn ArtifactCanonicalJsonTree,ValueError>{match ordinal{$($index=>Ok(&self.$index as&dyn ArtifactCanonicalJsonTree),)+_=>Err(refusal("canonical original tuple ordinal is absent"))}}
  }
 };
}
canonical_tuple!(2;A:0,B:1);
canonical_tuple!(3;A:0,B:1,C:2);
canonical_tuple!(4;A:0,B:1,C:2,D:3);
impl ArtifactCanonicalJsonTree for () {fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Null)}}
impl ArtifactCanonicalJsonTree for semio_framework_value::Number {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(match self{semio_framework_value::Number::UInt(value)=>Node::U64(*value),semio_framework_value::Number::Int(value)=>Node::I64(*value),semio_framework_value::Number::Float(value)=>Node::F64(*value)})}
}
impl ArtifactCanonicalJsonTree for semio_framework_value::DslValue {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{
  use semio_framework_value::DslValue;
  Ok(match self{DslValue::Null=>Node::Null,DslValue::Bool(value)=>Node::Bool(*value),DslValue::Number(number)=>return number.canonical_tree_node(),DslValue::String(text)=>Node::String(text),DslValue::Bytes(bytes)=>Node::Array(bytes.len()),DslValue::Array(items)=>Node::Array(items.len()),DslValue::Object(entries)=>Node::Object(entries.len())})
 }
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn ArtifactCanonicalJsonTree,ValueError>{
  use semio_framework_value::DslValue;
  match self{
   DslValue::Bytes(bytes)=>bytes.get(ordinal).map(|byte|byte as&dyn ArtifactCanonicalJsonTree).ok_or_else(||refusal("canonical dynamic byte ordinal is absent")),
   DslValue::Array(items)=>items.get(ordinal).map(|item|item as&dyn ArtifactCanonicalJsonTree).ok_or_else(||refusal("canonical dynamic array ordinal is absent")),
   DslValue::Object(entries)=>entries.get(ordinal).map(|(_,item)|item as&dyn ArtifactCanonicalJsonTree).ok_or_else(||refusal("canonical dynamic object ordinal is absent")),
   _=>Err(refusal("canonical dynamic scalar has no child")),
  }
 }
 fn canonical_tree_key(&self,ordinal:usize)->Result<ArtifactCanonicalJsonText<'_>,ValueError>{
  match self{
   semio_framework_value::DslValue::Object(entries)=>entries.get(ordinal).map(|(key,_)|ArtifactCanonicalJsonText::from(key.as_str())).ok_or_else(||refusal("canonical dynamic object key is absent")),
   _=>Err(refusal("canonical dynamic node has no key")),
  }
 }
}

/// 🚚️ The smallest output window in which the bulk path is attempted; smaller windows keep the one-event-per-turn walk.
const FLAT_MINIMUM_BYTES: usize = 32;
/// 🚚️ The deepest subtree the bulk path encodes inside one turn.
const FLAT_DEPTH: usize = 16;

fn put(output: &mut [u8], at: &mut usize, byte: u8) -> bool {
    match output.get_mut(*at) { Some(slot) => { *slot = byte; *at += 1; true } None => false }
}

fn flat_text(text: ArtifactCanonicalJsonText<'_>, output: &mut [u8]) -> Result<Option<usize>, ValueError> {
    let (mut at, mut chunk, mut offset) = (0, 0, 0);
    if !put(output, &mut at, b'"') { return Ok(None); }
    while let Some(byte) = text.next_byte(&mut chunk, &mut offset).map_err(refusal)? {
        let mut index = 0;
        while let Some(escaped) = canonical_escaped_byte(byte, index) { if !put(output, &mut at, escaped) { return Ok(None); } index += 1; }
    }
    Ok(put(output, &mut at, b'"').then_some(at))
}

/// 🚚️ Encodes one whole subtree into `output` with exactly the bytes the one-event walk emits, or returns `None` when it does not fit or exceeds the bulk depth, leaving the caller on the walk.
fn flat_encode(tree: &dyn ArtifactCanonicalJsonTree, output: &mut [u8], depth: usize) -> Result<Option<usize>, ValueError> {
    if depth > FLAT_DEPTH { return Ok(None); }
    let mut at = 0;
    match tree.canonical_tree_node()? {
        Node::Array(length) => {
            if !put(output, &mut at, b'[') { return Ok(None); }
            for ordinal in 0..length {
                if ordinal != 0 && !put(output, &mut at, b',') { return Ok(None); }
                match flat_encode(tree.canonical_tree_child(ordinal)?, &mut output[at..], depth + 1)? { Some(written) => at += written, None => return Ok(None) }
            }
            Ok(put(output, &mut at, b']').then_some(at))
        }
        Node::Object(length) => {
            if !put(output, &mut at, b'{') { return Ok(None); }
            for ordinal in 0..length {
                if ordinal != 0 && !put(output, &mut at, b',') { return Ok(None); }
                match flat_text(tree.canonical_tree_key(ordinal)?, &mut output[at..])? { Some(written) => at += written, None => return Ok(None) }
                if !put(output, &mut at, b':') { return Ok(None); }
                match flat_encode(tree.canonical_tree_child(ordinal)?, &mut output[at..], depth + 1)? { Some(written) => at += written, None => return Ok(None) }
            }
            Ok(put(output, &mut at, b'}').then_some(at))
        }
        Node::String(text) => flat_text(text.into(), output),
        Node::Text(text) => flat_text(text, output),
        scalar => {
            let mut bytes = ScalarBytes { bytes: [0; 64], length: 0 };
            bytes.write_node(scalar)?;
            match output.get_mut(..bytes.length) { Some(window) => { window.copy_from_slice(&bytes.bytes[..bytes.length]); Ok(Some(bytes.length)) } None => Ok(None) }
        }
    }
}

/// 🚚️ Moves the frame forward by whole subtrees inside one turn: a node that fits the window completes at once, and an array resumes as a run of children, so contiguous scalar runs cost one turn per window instead of one per element.
fn flat_run(frame: &mut NativeFrame, output: &mut [u8], grant: RetainedCloneGrant) -> Result<Option<usize>, ValueError> {
    let window = output.len().min(grant.maximum_copy_bytes);
    if window < FLAT_MINIMUM_BYTES { return Ok(None); }
    let output = &mut output[..window];
    match frame.phase {
        Phase::Inspect => {
            let written = { let reference = frame.owner.borrow()?; flat_encode(reference.get(), output, 0)? };
            if written.is_some() { frame.phase = Phase::Complete; }
            Ok(written)
        }
        Phase::ArrayNext if frame.ordinal < frame.length => {
            let (mut at, mut count) = (0, 0);
            {
                let reference = frame.owner.borrow()?;
                let array = reference.get();
                while frame.ordinal + count < frame.length {
                    let ordinal = frame.ordinal + count;
                    let start = at;
                    if ordinal != 0 && !put(output, &mut at, b',') { break; }
                    match flat_encode(array.canonical_tree_child(ordinal)?, &mut output[at..], 0)? { Some(written) => { at += written; count += 1; } None => { at = start; break; } }
                }
            }
            frame.ordinal += count;
            Ok((count != 0).then_some(at))
        }
        Phase::Text => {
            let reference = frame.owner.borrow()?;
            let text = match reference.get().canonical_tree_node()? { Node::String(text) => text.into(), Node::Text(text) => text, _ => return Err(refusal("canonical native text role changed")) };
            let mut at = 0;
            while at < output.len() && frame.text.phase != 3 { if let Some(byte) = frame.text.step(text)? { output[at] = byte; at += 1; } }
            if frame.text.phase == 3 { frame.phase = Phase::Complete; }
            Ok((at != 0).then_some(at))
        }
        _ => Ok(None),
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase { Inspect, Scalar, Text, ArrayStart, ArrayNext, ArrayChild, ObjectStart, ObjectNext, ObjectKey, ObjectColon, ObjectChild, Complete }

#[derive(Default)]
struct TextState { chunk: usize, offset: usize, phase: u8, original: u8, escaped: usize }
impl TextState {
    fn step(&mut self, text: ArtifactCanonicalJsonText<'_>) -> Result<Option<u8>, ValueError> {
        match self.phase {
            0 => { self.phase = 1; Ok(Some(b'"')) }
            1 => match text.next_byte(&mut self.chunk, &mut self.offset).map_err(refusal)? {
                Some(byte) => { self.original = byte; self.escaped = 0; self.phase = 2; Ok(None) }
                None => { self.phase = 3; Ok(Some(b'"')) }
            },
            2 => { let byte = canonical_escaped_byte(self.original, self.escaped).ok_or_else(|| refusal("canonical native escape offset is absent"))?; self.escaped += 1; if canonical_escaped_byte(self.original, self.escaped).is_none() { self.phase = 1; } Ok(Some(byte)) }
            _ => Err(refusal("canonical native text advanced after terminal quote")),
        }
    }
}

struct NativeFrame {
    owner: RetainedOwnedProjection<dyn ArtifactCanonicalJsonTree>,
    phase: Phase,
    ordinal: usize,
    length: usize,
    scalar: ScalarBytes,
    offset: usize,
    text: TextState,
}
impl NativeFrame {
    fn new(owner: RetainedOwnedProjection<dyn ArtifactCanonicalJsonTree>) -> Self { Self { owner, phase: Phase::Inspect, ordinal: 0, length: 0, scalar: ScalarBytes { bytes: [0; 64], length: 0 }, offset: 0, text: TextState::default() } }
    fn closure_demand(&self) -> Result<RetirementDemand, ValueError> {
        let copy_bytes = self.owner.next_close_copy_byte_demand()?;
        Ok(RetirementDemand { copy_bytes, capacity_bytes: self.owner.next_close_capacity_byte_demand(copy_bytes)?, release_bytes: self.owner.next_close_release_byte_demand()?, depth: self.owner.next_close_depth_demand()? })
    }
}

/// 📊️ One admitted native action reports only its initialized byte prefix and common ownership receipt.
pub struct ArtifactCanonicalJsonTreeStep { pub ownership: RetainedCloneStep, pub written_bytes: usize }

/// 🧭️ Owns actual immutable-root aliases in a genuinely paged traversal frontier without path rescans.
pub struct ArtifactCanonicalJsonTreeCursor {
    frames: PagedList<NativeFrame, {usize::MAX}>,
    pending: Option<RetainedOwnedProjection<dyn ArtifactCanonicalJsonTree>>,
    closing: bool,
}

impl ArtifactCanonicalJsonTreeCursor {
    pub fn constructor_demand() -> RetirementDemand { RetirementDemand { copy_bytes: size_of::<RetainedOwnedProjection<dyn ArtifactCanonicalJsonTree>>(), depth: 1, ..Default::default() } }
    pub fn admit(owner: RetainedOwnedProjection<dyn ArtifactCanonicalJsonTree>, grant: RetainedCloneGrant) -> Result<(Self, RetainedCloneProgress), (ValueError, RetainedOwnedProjection<dyn ArtifactCanonicalJsonTree>)> {
        let demand = Self::constructor_demand();
        if !Self::admitted(demand, grant) { return Err((ValueError::literal(ValueRefusalKind::WorkLimit, "canonical tree constructor requires original alias transfer admission"), owner)); }
        Ok((Self { frames: PagedList::empty(), pending: Some(owner), closing: false }, RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
    }

    fn admitted(demand: RetirementDemand, grant: RetainedCloneGrant) -> bool { grant.maximum_items != 0 && demand.copy_bytes <= grant.maximum_copy_bytes && demand.capacity_bytes <= grant.maximum_capacity_bytes && demand.release_bytes <= grant.maximum_release_bytes && demand.depth <= grant.maximum_depth }
    fn pending_closure_demand(&self) -> Result<RetirementDemand, ValueError> {
        let owner = self.pending.as_ref().ok_or_else(|| refusal("canonical pending alias is absent"))?;
        if owner.terminal_is_empty() { return Ok(RetirementDemand { copy_bytes: size_of::<RetainedOwnedProjection<dyn ArtifactCanonicalJsonTree>>(), depth: 1, ..Default::default() }); }
        let copy_bytes = owner.next_close_copy_byte_demand()?;
        Ok(RetirementDemand { copy_bytes, capacity_bytes: owner.next_close_capacity_byte_demand(copy_bytes)?, release_bytes: owner.next_close_release_byte_demand()?, depth: owner.next_close_depth_demand()? })
    }

    pub fn next_demand(&self) -> Result<RetirementDemand, ValueError> {
        if self.terminal_is_empty() { return Ok(Default::default()); }
        if self.pending.is_some() {
            if self.closing { return self.pending_closure_demand(); }
            if !self.frames.has_reserved_slot() { return Ok(RetirementDemand { capacity_bytes: self.frames.next_allocation_bytes().map_err(ValueError::from)?, depth: 1, ..Default::default() }); }
            return Ok(RetirementDemand { copy_bytes: size_of::<NativeFrame>(), depth: 1, ..Default::default() });
        }
        if let Some(frame) = self.frames.last() {
            if self.closing || frame.phase == Phase::Complete {
                if frame.owner.terminal_is_empty() { return Ok(RetirementDemand { copy_bytes: size_of::<NativeFrame>(), depth: 1, ..Default::default() }); }
                return frame.closure_demand();
            }
            let copy_bytes = match frame.phase {
                Phase::Inspect => match frame.owner.borrow()?.get().canonical_tree_node()? { Node::Array(_) | Node::Object(_) | Node::String(_) | Node::Text(_) => 0, _ => 64 },
                Phase::ArrayChild | Phase::ObjectChild => size_of::<RetainedOwnedProjection<dyn ArtifactCanonicalJsonTree>>(),
                Phase::ArrayNext if frame.ordinal < frame.length && frame.ordinal == 0 => size_of::<RetainedOwnedProjection<dyn ArtifactCanonicalJsonTree>>(),
                _ => 1,
            };
            return Ok(RetirementDemand { copy_bytes, depth: 1, ..Default::default() });
        }
        Ok(RetirementDemand { release_bytes: self.frames.next_release_allocation_bytes().map_err(ValueError::from)?, depth: 1, ..Default::default() })
    }

    pub fn begin_close(&mut self) { self.closing = true; }
    pub fn terminal_is_empty(&self) -> bool { self.pending.is_none() && self.frames.is_empty() && self.frames.allocated_bytes() == 0 }
    pub fn is_complete(&self) -> bool { !self.closing && self.terminal_is_empty() }

    pub fn advance(&mut self, output: &mut [u8], grant: RetainedCloneGrant) -> Result<ArtifactCanonicalJsonTreeStep, ValueError> {
        if self.terminal_is_empty() { return Ok(ArtifactCanonicalJsonTreeStep { ownership: RetainedCloneStep::Complete(Default::default()), written_bytes: 0 }); }
        if grant.maximum_items == 0 { return Ok(ArtifactCanonicalJsonTreeStep { ownership: RetainedCloneStep::Progress(Default::default()), written_bytes: 0 }); }
        let demand = self.next_demand()?;
        if !Self::admitted(demand, grant) { return Ok(ArtifactCanonicalJsonTreeStep { ownership: RetainedCloneStep::Progress(Default::default()), written_bytes: 0 }); }
        let mut progress = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        let mut written_bytes = 0;
        if self.pending.is_some() {
            if self.closing {
                if self.pending.as_ref().unwrap().terminal_is_empty() { self.pending = None; progress.copied_bytes = demand.copy_bytes; }
                else { progress = self.pending.as_mut().unwrap().close_step(grant)?.progress(); }
            } else if !self.frames.has_reserved_slot() {
                let step = self.frames.reserve_one(grant.maximum_capacity_bytes).map_err(|error| ValueError::from(error.refusal()))?;
                progress.retained_capacity_bytes = step.allocated_bytes;
            } else {
                let frame = NativeFrame::new(self.pending.take().unwrap());
                self.frames.push_reserved(frame).unwrap_or_else(|_| unreachable!("canonical frontier reserved its exact native slot"));
                progress.copied_bytes = demand.copy_bytes;
            }
        } else if let Some(frame) = self.frames.last_mut() {
            if self.closing || frame.phase == Phase::Complete {
                if frame.owner.terminal_is_empty() { drop(self.frames.pop().unwrap()); progress.copied_bytes = demand.copy_bytes; }
                else { progress = frame.owner.close_step(grant)?.progress(); }
            } else if let Some(written) = flat_run(frame, output, grant)? {
                progress.copied_bytes = written;
                written_bytes = written;
            } else {
                let inspected = frame.phase == Phase::Inspect;
                let byte = match frame.phase {
                    Phase::Inspect => {
                        let reference = frame.owner.borrow()?;
                        let node = reference.get().canonical_tree_node()?;
                        match node {
                            Node::Array(length) => { frame.length = length; frame.phase = Phase::ArrayStart; }
                            Node::Object(length) => { frame.length = length; frame.phase = Phase::ObjectStart; }
                            Node::String(_) | Node::Text(_) => frame.phase = Phase::Text,
                            scalar => { progress = frame.scalar.write_node(scalar)?; frame.phase = Phase::Scalar; }
                        }
                        None
                    }
                    Phase::ArrayChild | Phase::ObjectChild | Phase::ArrayNext if frame.phase != Phase::ArrayNext || frame.ordinal == 0 && frame.ordinal < frame.length => {
                        let ordinal = frame.ordinal;
                        frame.owner.borrow()?.get().canonical_tree_child(ordinal)?;
                        let (child, alias_progress) = frame.owner.project(ordinal, |node| node.canonical_tree_child(ordinal).expect("checked immutable native canonical child"), grant)?;
                        self.pending = Some(child);
                        progress = alias_progress;
                        frame.ordinal += 1;
                        frame.phase = if frame.phase == Phase::ObjectChild { Phase::ObjectNext } else { Phase::ArrayNext };
                        progress.copied_bytes = demand.copy_bytes;
                        None
                    }
                    _ if output.is_empty() => { return Ok(ArtifactCanonicalJsonTreeStep { ownership: RetainedCloneStep::Progress(Default::default()), written_bytes: 0 }); }
                    Phase::Scalar => {
                        let byte = frame.scalar.bytes[frame.offset]; frame.offset += 1;
                        if frame.offset == frame.scalar.length { frame.phase = Phase::Complete; }
                        Some(byte)
                    }
                    Phase::Text => {
                        let reference = frame.owner.borrow()?;
                        let text = match reference.get().canonical_tree_node()? { Node::String(text) => text.into(), Node::Text(text) => text, _ => return Err(refusal("canonical native text role changed")) };
                        let byte = frame.text.step(text)?;
                        if frame.text.phase == 3 { frame.phase = Phase::Complete; }
                        byte
                    }
                    Phase::ArrayStart => { frame.phase = Phase::ArrayNext; Some(b'[') }
                    Phase::ArrayNext => { if frame.ordinal == frame.length { frame.phase = Phase::Complete; Some(b']') } else { frame.phase = Phase::ArrayChild; Some(b',') } }
                    Phase::ObjectStart => { frame.phase = Phase::ObjectNext; Some(b'{') }
                    Phase::ObjectNext => {
                        if frame.ordinal == frame.length { frame.phase = Phase::Complete; Some(b'}') }
                        else { frame.phase = Phase::ObjectKey; frame.text = TextState::default(); if frame.ordinal != 0 { Some(b',') } else { None } }
                    }
                    Phase::ObjectKey => {
                        let reference = frame.owner.borrow()?;
                        let text = reference.get().canonical_tree_key(frame.ordinal)?;
                        let byte = frame.text.step(text)?;
                        if frame.text.phase == 3 { frame.phase = Phase::ObjectColon; }
                        byte
                    }
                    Phase::ObjectColon => { frame.phase = Phase::ObjectChild; Some(b':') }
                    _ => return Err(refusal("canonical native traversal phase is invalid")),
                };
                if !inspected && progress.copied_bytes == 0 { progress.copied_bytes = 1; }
                if let Some(byte) = byte { output[0] = byte; written_bytes = 1; }
            }
        } else {
            let step = self.frames.release_empty_page(grant.maximum_release_bytes).map_err(ValueError::from)?;
            progress.released_bytes = step.released_allocation_bytes;
        }
        Ok(ArtifactCanonicalJsonTreeStep { ownership: if self.terminal_is_empty() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) }, written_bytes })
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { self.begin_close(); Ok(self.advance(&mut [], grant)?.ownership) }
}
impl Drop for ArtifactCanonicalJsonTreeCursor { fn drop(&mut self) { assert!(self.terminal_is_empty() || std::thread::panicking(), "canonical native traversal requires exact frontier closure"); } }

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
