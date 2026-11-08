//! 🧺️ Immutable ordered string membership over the codebase-owned retained map.

use super::{Iter, LookupCursor, OrderedMap, Retirement, RetirementStep, UpdateCursor, RetainedCloneGrant};


//#region 🧺️OrderedSet
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[must_use = "ordered set ownership must be explicitly retired"]
pub struct OrderedSet { values: OrderedMap<()> }

impl OrderedSet {
    pub fn new() -> Self { Self::default() }
    pub fn from_map(values: OrderedMap<()>) -> Self { Self { values } }
    pub fn len(&self) -> usize { self.values.len() }
    pub fn is_empty(&self) -> bool { self.values.is_empty() }
    pub fn iter(&self) -> SetIter<'_> { SetIter(self.values.iter()) }
    /// 🔎️ Borrows one original string by ranked tree navigation.
    pub fn key_at_rank(&self, index: usize) -> Option<&String> { self.values.entry_at_rank(index).map(|(key, ())| key) }
    /// 🧊️ Cold synchronous lookup; retained membership uses begin_lookup.
    pub fn contains(&self, key: &str) -> bool { self.values.contains_key(key) }
    /// ❄️ Cold synchronous insertion; retained callers use begin_insert.
    pub fn insert(&mut self, key: String) -> bool { let fresh = !self.values.contains_key(&key); self.values.insert(key, ()); fresh }
    /// ➖️ Cold synchronous removal; retained callers use begin_remove.
    pub fn remove(&mut self, key: &str) -> bool { self.values.remove(key).is_some() }
    pub fn begin_insert(&self, key: String) -> UpdateCursor<()> { self.values.begin_set(key, ()) }
    pub fn begin_remove(&self, key: String) -> UpdateCursor<()> { self.values.begin_remove(key) }
    pub fn begin_lookup(&self, key: String) -> LookupCursor<()> { self.values.begin_lookup(key) }
    pub fn retire(self) -> Retirement<()> { self.values.retire() }
    /// 🧹️ Explicit cold cleanup; never called by retained advance or Drop.
    pub fn retire_cold(self) {
        let mut retirement = self.retire();
        loop {
            let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:0,maximum_release_bytes:retirement.next_close_byte_demand().expect("finite ordered-set cold release demand"),maximum_depth:retirement.next_depth_demand()};
            match retirement.advance(grant){RetirementStep::Complete=>break,RetirementStep::Failure(error)=>panic!("ordered set cold release refused: {error}"),_=>{}}
        }
    }
}

pub struct SetIter<'a>(Iter<'a, ()>);
impl<'a> Iterator for SetIter<'a> {
    type Item = &'a String;
    fn next(&mut self) -> Option<Self::Item> { self.0.next().map(|(key, ())| key) }
    fn size_hint(&self) -> (usize, Option<usize>) { self.0.size_hint() }
}
impl DoubleEndedIterator for SetIter<'_> { fn next_back(&mut self) -> Option<Self::Item> { self.0.next_back().map(|(key, ())| key) } }
impl ExactSizeIterator for SetIter<'_> {}
impl<'a> IntoIterator for &'a OrderedSet { type Item = &'a String; type IntoIter = SetIter<'a>; fn into_iter(self) -> Self::IntoIter { self.iter() } }
impl FromIterator<String> for OrderedSet {
    fn from_iter<T: IntoIterator<Item = String>>(values: T) -> Self { let mut set = Self::new(); for value in values { set.insert(value); } set }
}
impl<const N: usize> From<[String; N]> for OrderedSet { fn from(values: [String; N]) -> Self { values.into_iter().collect() } }
//#endregion 🧺️OrderedSet



/// 🔌️ Gated (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/02): `ToValue`/ [Design notes](README.md#source-notes-56).
impl serde::Serialize for OrderedSet {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let mut sequence = serializer.serialize_seq(Some(self.len()))?;
        for value in self { sequence.serialize_element(value)?; }
        sequence.end()
    }
}
impl<'de> serde::Deserialize<'de> for OrderedSet {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = OrderedSet;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { formatter.write_str("an ordered string array") }
            fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
                let mut set = OrderedSet::new();
                loop {
                    match access.next_element::<String>() {
                        Ok(Some(value)) => { set.insert(value); }
                        Ok(None) => return Ok(set),
                        Err(error) => { set.retire_cold(); return Err(error); }
                    }
                }
            }
        }
        deserializer.deserialize_seq(Visitor)
    }
}
//#endregion 🔀️ArrayWire

//#region 🧪️SetLaws
#[cfg(test)]
#[path = "🧪️tests/🧺️set/🦀️.rs"]
mod tests;
//#endregion 🧪️SetLaws

impl semio_framework_value_retained_clone::RetainedAlias for OrderedSet{
 fn alias_demand(&self)->crate::RetirementDemand{crate::RetirementDemand{copy_bytes:std::mem::size_of::<Self>()+std::mem::size_of::<Option<Self>>()+std::mem::size_of::<usize>(),depth:1,..Default::default()}}
 fn alias_work_demand(&self)->u64{1}
 fn admit_alias_into<'r>(&self,output:&mut Option<Self>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'r,crate::RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'r,crate::RetainedCloneStep,crate::OriginalRetirementFailure>,crate::ValueError>{
  if output.is_some(){return Err(crate::ValueError::invariant("ordered alias output is occupied"))}if effect.invocation().is_none(){return Err(crate::ValueError::invariant("ordered alias lacks its original invocation"))}let demand=semio_framework_value_retained_clone::RetainedAlias::alias_demand(self);if !crate::retained_clone::retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<1{return Ok(reply.complete(Ok(crate::RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Ok(reply.complete(Err(crate::ValueError::literal(crate::ValueRefusalKind::WorkLimit,"ordered alias exceeds its original Work receipt").into())))}*output=Some(Self{values:self.values.clone()});Ok(reply.complete(Ok(crate::RetainedCloneStep::Complete(crate::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))))
 }
 fn recover_alias_into<'r>(&self,_output:&mut Option<Self>,_effect:&mut crate::RetainedAllocationEffect<'_>,_reply:semio_framework_value_control_contract::OriginalEffectReply<'r,crate::RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'r,crate::RetainedCloneStep,crate::OriginalRetirementFailure>,crate::ValueError>{Err(crate::ValueError::invariant("native ordered alias has no unresolved callback"))}
}
impl semio_framework_value_retained_clone::RetainedClone for OrderedSet{type Cursor=semio_framework_value_retained_clone::RetainedAliasCursor<Self>;fn retained_clone_cursor()->Self::Cursor{Default::default()}}
