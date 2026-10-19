//! 🧺️ Standard hashed and ordered collections clone entry by entry; the standard table and node backing is not priced, only entry scaffolds and moves.
//!
//! The backing is invisible to the allocator contract, exactly as in the `RetireOwned` implementations of the same types.

use super::{RetainedClone, RetainedCloneBinding, RetainedCloneClose, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, admit_retained_clone_close, admit_retained_clone_progress, close_retained_binding};
use std::{collections::{BTreeMap, BTreeSet, HashMap, HashSet}, hash::Hash, mem::size_of};

/// 🧺️ Exposes one standard collection as a dense, immutable entry sequence plus a cold constructor.
pub trait RetainedCloneEntries: RetainedClone {
    type Key: RetainedClone;
    type Value: RetainedClone;
    fn entry_count(&self) -> usize;
    fn entry_at(&self, index: usize) -> Option<(&Self::Key, &Self::Value)>;
    fn empty() -> Self;
    fn insert_entry(&mut self, key: Self::Key, value: Self::Value) -> Result<(), (Self::Key, Self::Value)>;
    const HAS_IMPLICIT_VALUE:bool=false;
    fn implicit_value() -> Option<Self::Value> { None }
    /// 🪙️ Quotes the original collection backing and pair movement before an entry is inserted.
    fn insertion_demands(_building: Option<&Self>, _key: &Self::Key) -> crate::RetirementDemand { crate::RetirementDemand { copy_bytes: size_of::<(Self::Key, Self::Value)>(), depth: 1, ..Default::default() } }
}

impl<K: RetainedClone + Hash + Eq, V: RetainedClone> RetainedCloneEntries for HashMap<K, V> {
    type Key = K;
    type Value = V;
    fn entry_count(&self) -> usize { self.len() }
    fn entry_at(&self, index: usize) -> Option<(&K, &V)> { self.iter().nth(index) }
    fn empty() -> Self { Self::new() }
    fn insert_entry(&mut self, key: K, value: V) -> Result<(), (K, V)> {
        if self.contains_key(&key) { return Err((key, value)); }
        self.insert(key, value);
        Ok(())
    }
}

impl<K: RetainedClone + Ord, V: RetainedClone> RetainedCloneEntries for BTreeMap<K, V> {
    type Key = K;
    type Value = V;
    fn entry_count(&self) -> usize { self.len() }
    fn entry_at(&self, index: usize) -> Option<(&K, &V)> { self.iter().nth(index) }
    fn empty() -> Self { Self::new() }
    fn insert_entry(&mut self, key: K, value: V) -> Result<(), (K, V)> {
        if self.contains_key(&key) { return Err((key, value)); }
        self.insert(key, value);
        Ok(())
    }
}

impl<T: RetainedClone + Hash + Eq> RetainedCloneEntries for HashSet<T> {
    type Key = T;
    type Value = ();
    fn entry_count(&self) -> usize { self.len() }
    fn entry_at(&self, index: usize) -> Option<(&T, &())> { self.iter().nth(index).map(|key| (key, &())) }
    fn empty() -> Self { Self::new() }
    fn insert_entry(&mut self, key: T, value: ()) -> Result<(), (T, ())> {
        if self.contains(&key) { return Err((key, value)); }
        self.insert(key);
        Ok(())
    }
    const HAS_IMPLICIT_VALUE:bool=true;
    fn implicit_value() -> Option<()> { Some(()) }
}

impl<T: RetainedClone + Ord> RetainedCloneEntries for BTreeSet<T> {
    type Key = T;
    type Value = ();
    fn entry_count(&self) -> usize { self.len() }
    fn entry_at(&self, index: usize) -> Option<(&T, &())> { self.iter().nth(index).map(|key| (key, &())) }
    fn empty() -> Self { Self::new() }
    fn insert_entry(&mut self, key: T, value: ()) -> Result<(), (T, ())> {
        if self.contains(&key) { return Err((key, value)); }
        self.insert(key);
        Ok(())
    }
    const HAS_IMPLICIT_VALUE:bool=true;
    fn implicit_value() -> Option<()> { Some(()) }
}

/// 🧺️ Clones one entry per key, scaffold-close and insertion turn without pricing the standard backing.
pub struct RetainedCloneEntriesCursor<M: RetainedCloneEntries> {
    key_cursor: <M::Key as RetainedClone>::Cursor,
    value_cursor: <M::Value as RetainedClone>::Cursor,
    key: Option<M::Key>,
    value: Option<M::Value>,
    building: Option<M>,
    output: Option<M>,
    index: usize,
    phase: u8,
    source: Option<RetainedCloneBinding>,
    spent: bool,
    closing: bool,
    close: RetainedCloneClose,
}

impl<M: RetainedCloneEntries> Default for RetainedCloneEntriesCursor<M> {
    fn default() -> Self {
        Self {
            key_cursor: <M::Key as RetainedClone>::retained_clone_cursor(),
            value_cursor: <M::Value as RetainedClone>::retained_clone_cursor(),
            key: None,
            value: None,
            building: None,
            output: None,
            index: 0,
            phase: 1,
            source: None,
            spent: false,
            closing: false,
            close: RetainedCloneClose::default(),
        }
    }
}

const SELECT: u8 = 1;
const KEY: u8 = 2;
const KEY_CLOSE: u8 = 3;
const VALUE: u8 = 4;
const VALUE_CLOSE: u8 = 5;
const INSERT: u8 = 6;
const DONE: u8 = 7;

impl<M:RetainedCloneEntries> RetainedCloneEntriesCursor<M>{
    fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{
        if !self.closing{return Ok(Default::default());}
        if !self.key_cursor.terminal_is_empty(){return super::nested_retained_clone_close_demands::<M::Key>(&self.key_cursor,body);}
        if !self.value_cursor.terminal_is_empty(){return super::nested_retained_clone_close_demands::<M::Value>(&self.value_cursor,body);}
        if self.key.is_some(){return self.close.next_owner_demands::<M::Key>(true,&self.source,body);}
        if self.value.is_some(){return self.close.next_owner_demands::<M::Value>(true,&self.source,body);}
        self.close.next_owner_demands::<M>(self.building.is_some()||self.output.is_some(),&self.source,body)
    }
}
impl<M: RetainedCloneEntries> RetainedCloneCursor<M> for RetainedCloneEntriesCursor<M> {
    fn output_ready(&self)->bool{self.output.is_some()}
    fn advance_demands(&self, source: RetainedCloneRef<'_, M>, body: usize) -> Result<crate::RetirementDemand, crate::ValueError> {
        if self.closing || self.spent { return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "standard collection clone cannot quote a closing or spent owner")); }
        if !source.check_binding(&self.source)? { return Ok(crate::RetirementDemand { copy_bytes: source.binding_copy_bytes(), depth: 1, ..Default::default() }); }
        if self.output_ready() { return Ok(Default::default()); }
        let index = self.index;
        let mut demand = crate::RetirementDemand { depth: 1, ..Default::default() };
        match self.phase {
            SELECT => { if index == source.get().entry_count() { demand.copy_bytes = size_of::<M>(); } }
            KEY if self.key_cursor.output_ready() => demand.copy_bytes = size_of::<M::Key>(),
            KEY => return super::nested_retained_clone_demands::<M::Key>(&self.key_cursor,source.project(index * 2 + 1, |map| map.entry_at(index).expect("validated standard collection entry").0),body),
            KEY_CLOSE if !self.key_cursor.terminal_is_empty() => return super::nested_retained_clone_close_demands::<M::Key>(&self.key_cursor,body),
            VALUE if self.value_cursor.output_ready() => demand.copy_bytes = size_of::<M::Value>(),
            VALUE => return super::nested_retained_clone_demands::<M::Value>(&self.value_cursor,source.project(index * 2 + 2, |map| map.entry_at(index).expect("validated standard collection entry").1),body),
            VALUE_CLOSE if !self.value_cursor.terminal_is_empty() => return super::nested_retained_clone_close_demands::<M::Value>(&self.value_cursor,body),
            INSERT => return Ok(M::insertion_demands(self.building.as_ref(), self.key.as_ref().ok_or_else(|| crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "collection insertion key missing"))?)),
            KEY_CLOSE => demand.copy_bytes=size_of::<<M::Key as RetainedClone>::Cursor>().checked_add(if M::HAS_IMPLICIT_VALUE{size_of::<M::Value>()}else{0}).ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::WorkLimit,"standard collection cursor reset extent overflow"))?,
            VALUE_CLOSE => demand.copy_bytes=size_of::<<M::Value as RetainedClone>::Cursor>(),
            DONE => {}
            _ => return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "standard collection clone quote state is invalid")),
        }
        Ok(demand)
    }

    fn advance(&mut self, source: RetainedCloneRef<'_, M>, body: usize, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained standard collection clone cursor is closing")); }
        if self.spent { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained standard collection clone cursor is spent")); }
        let demand=self.advance_demands(source,body)?;
        if self.output_ready() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if !super::retained_clone_grant_admits(grant,demand) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(progress) = source.bind(&mut self.source, grant)? { return Ok(RetainedCloneStep::Progress(progress)); }
        let index = self.index;
        match self.phase {
            SELECT => {
                let count = source.get().entry_count();
                if index > count { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained standard collection clone index passed its source")); }
                if index == count {
                    let bytes = size_of::<M>();
                    if grant.maximum_copy_bytes < bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
                    self.output = Some(self.building.take().unwrap_or_else(M::empty));
                    self.phase = DONE;
                    return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() }));
                }
                self.phase = KEY;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
            }
            KEY => {
                if self.key_cursor.output_ready() {
                    self.key=Some(self.key_cursor.take().ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"standard collection ready key omitted owner"))?);
                    self.key_cursor.begin_close();
                    self.phase=KEY_CLOSE;
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));
                }
                let nested=super::nested_retained_clone_grant(grant)?;
                let step=self.key_cursor.advance(source.project(index*2+1,|map|map.entry_at(index).expect("validated standard collection entry").0),body,nested)?;
                Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(nested,step.progress(),"standard collection key")?))
            }
            KEY_CLOSE => {
                if !self.key_cursor.terminal_is_empty() {
                    let step = self.key_cursor.close_step(super::nested_retained_clone_grant(grant)?)?;
                    return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, self.key_cursor.terminal_is_empty(), "retained standard collection key scaffold close")?.progress()));
                }
                self.key_cursor = <M::Key as RetainedClone>::retained_clone_cursor();
                if let Some(value) = M::implicit_value() {
                    self.value = Some(value);
                    self.phase = INSERT;
                } else {
                    self.phase = VALUE;
                }
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes:demand.copy_bytes, ..Default::default() }))
            }
            VALUE => {
                if self.value_cursor.output_ready() {
                    self.value=Some(self.value_cursor.take().ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"standard collection ready value omitted owner"))?);
                    self.value_cursor.begin_close();
                    self.phase=VALUE_CLOSE;
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));
                }
                let nested=super::nested_retained_clone_grant(grant)?;
                let step=self.value_cursor.advance(source.project(index*2+2,|map|map.entry_at(index).expect("validated standard collection entry").1),body,nested)?;
                Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(nested,step.progress(),"standard collection value")?))
            }
            VALUE_CLOSE => {
                if !self.value_cursor.terminal_is_empty() {
                    let step = self.value_cursor.close_step(super::nested_retained_clone_grant(grant)?)?;
                    return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, self.value_cursor.terminal_is_empty(), "retained standard collection value scaffold close")?.progress()));
                }
                self.value_cursor = <M::Value as RetainedClone>::retained_clone_cursor();
                self.phase = INSERT;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes:demand.copy_bytes, ..Default::default() }))
            }
            INSERT => {
                let demand = M::insertion_demands(self.building.as_ref(), self.key.as_ref().ok_or_else(|| crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "collection insertion key missing"))?);
                let bytes = demand.copy_bytes;
                if grant.maximum_copy_bytes < bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth { return Ok(RetainedCloneStep::Progress(Default::default())); }
                let key = self.key.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained standard collection key owner is missing"))?;
                let value = match self.value.take() {
                    Some(value) => value,
                    None => { self.key = Some(key); return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained standard collection value owner is missing")); }
                };
                match self.building.get_or_insert_with(M::empty).insert_entry(key, value) {
                    Ok(()) => {
                        self.index += 1;
                        self.phase = SELECT;
                        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, retained_capacity_bytes: demand.capacity_bytes, released_bytes: demand.release_bytes }))
                    }
                    Err((key, value)) => {
                        self.key = Some(key);
                        self.value = Some(value);
                        Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained standard collection clone produced a duplicate key"))
                    }
                }
            }
            DONE => Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())),
            _ => Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained standard collection clone state is invalid")),
        }
    }

    fn take(&mut self) -> Option<M> {
        let output = self.output.take();
        if output.is_some() { self.spent = true; }
        output
    }

    fn begin_close(&mut self) -> bool {
        if self.closing { return false; }
        self.closing = true;
        true
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "standard collection must begin close before granted retirement")); }
        if !super::retained_clone_grant_admits(grant,super::retained_clone_close_demands::<M>(self,grant.maximum_copy_bytes)?) {return Ok(RetainedCloneStep::Progress(Default::default()));}
        if !self.key_cursor.terminal_is_empty() {
            if self.key_cursor.begin_close() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
            let step = self.key_cursor.close_step(super::nested_retained_clone_grant(grant)?)?;
            return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, self.key_cursor.terminal_is_empty(), "retained standard collection key close")?.progress()));
        }
        if !self.value_cursor.terminal_is_empty() {
            if self.value_cursor.begin_close() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
            let step = self.value_cursor.close_step(super::nested_retained_clone_grant(grant)?)?;
            return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, self.value_cursor.terminal_is_empty(), "retained standard collection value close")?.progress()));
        }
        if !self.close.is_empty() { return self.close.step_granted(grant); }
        if let Some(step) = self.close.begin_granted(&mut self.key, grant)? { return Ok(step); }
        if let Some(step) = self.close.begin_granted(&mut self.value, grant)? { return Ok(step); }
        if let Some(step) = self.close.begin_granted(&mut self.building, grant)? { return Ok(step); }
        if let Some(step) = self.close.begin_granted(&mut self.output, grant)? { return Ok(step); }
        close_retained_binding(&mut self.source, grant)
    }

    fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{Ok(self.close_demands(0)?.depth)}
    fn next_close_copy_byte_demand(&self)->Result<usize,crate::ValueError>{Ok(self.close_demands(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,crate::ValueError>{Ok(self.close_demands(body)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,crate::ValueError>{Ok(self.close_demands(0)?.release_bytes)}
    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.key.is_none()
            && self.value.is_none()
            && self.building.is_none()
            && self.output.is_none()
            && self.key_cursor.terminal_is_empty()
            && self.value_cursor.terminal_is_empty()
            && self.close.is_empty()
            && self.source.is_none()
    }
}

impl<K: RetainedClone + Hash + Eq, V: RetainedClone> RetainedClone for HashMap<K, V> {
    type Cursor = RetainedCloneEntriesCursor<Self>;
    fn retained_clone_cursor() -> Self::Cursor { Default::default() }
}
impl<K: RetainedClone + Ord, V: RetainedClone> RetainedClone for BTreeMap<K, V> {
    type Cursor = RetainedCloneEntriesCursor<Self>;
    fn retained_clone_cursor() -> Self::Cursor { Default::default() }
}
impl<T: RetainedClone + Hash + Eq> RetainedClone for HashSet<T> {
    type Cursor = RetainedCloneEntriesCursor<Self>;
    fn retained_clone_cursor() -> Self::Cursor { Default::default() }
}
impl<T: RetainedClone + Ord> RetainedClone for BTreeSet<T> {
    type Cursor = RetainedCloneEntriesCursor<Self>;
    fn retained_clone_cursor() -> Self::Cursor { Default::default() }
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
