//! 🧺️ Standard hashed and ordered collections clone entry by entry; the standard table and node backing is not priced, only entry scaffolds and moves.
//!
//! The backing is invisible to the allocator contract, exactly as in the `RetireOwned` implementations of the same types.

use super::{RetainedClone, RetainedCloneBinding, RetainedCloneClose, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, admit_retained_clone_close, admit_retained_clone_progress, close_retained_binding};
use std::{collections::{BTreeMap, BTreeSet, HashMap, HashSet}, hash::Hash, mem::size_of};

/// ❄️ Exposes one standard collection as a dense, immutable entry sequence plus a cold constructor.
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

/// 🖱️ Clones one entry per key, scaffold-close and insertion turn without pricing the standard backing.
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
        #[cfg(test)] selected_close_tests::record(self as*const Self as usize,body);
        if !self.closing{return Ok(Default::default());}
        if !self.key_cursor.terminal_is_empty(){return super::nested_retained_clone_close_demands::<M::Key>(&self.key_cursor,body);}
        if !self.value_cursor.terminal_is_empty(){return super::nested_retained_clone_close_demands::<M::Value>(&self.value_cursor,body);}
        if self.key.is_some(){return self.close.next_owner_demands::<M::Key>(true,&self.source,body);}
        if self.value.is_some(){return self.close.next_owner_demands::<M::Value>(true,&self.source,body);}
        self.close.next_owner_demands::<M>(self.building.is_some()||self.output.is_some(),&self.source,body)
    }
}
impl<M: RetainedCloneEntries> RetainedCloneCursor<M> for RetainedCloneEntriesCursor<M> {
    fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{RetainedCloneEntriesCursor::<M>::close_demands(self,body)}
    fn begin_close_demand(&self)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::retained_clone::retained_close_start_demand(self.closing))}
fn begin_close_work_demand(&self)->Result<u64,crate::ValueError>{Ok(crate::retained_clone::retained_close_start_work_demand(self.closing))}
fn begin_close_paid(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<crate::retained_clone::RetainedCloneProgress>{crate::retained_clone::start_retained_close(&mut self.closing,effect)}
fn close_started(&self)->bool{self.closing}
fn output_ready(&self)->bool{self.output.is_some()}
    fn advance_demands(&self, source: RetainedCloneRef<'_, M>, body: usize,maximum_work_units:u64) -> Result<crate::RetirementDemand, crate::ValueError> {
        if self.closing || self.spent { return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "standard collection clone cannot quote a closing or spent owner")); }
        if !source.check_binding(&self.source)? { return Ok(crate::RetirementDemand { copy_bytes: source.binding_copy_bytes(), depth: 1, ..Default::default() }); }
        if self.output_ready() { return Ok(Default::default()); }
        let index = self.index;
        let mut demand = crate::RetirementDemand { depth: 1, ..Default::default() };
        match self.phase {
            SELECT => { if index == source.get().entry_count() { demand.copy_bytes = size_of::<M>(); } }
            KEY if self.key_cursor.output_ready() => return super::nested_retained_clone_take_demands(&self.key_cursor),
            KEY => return super::nested_retained_clone_demands::<M::Key>(&self.key_cursor,source.project(index * 2 + 1, |map| map.entry_at(index).expect("validated standard collection entry").0),body,maximum_work_units),
            KEY_CLOSE if !self.key_cursor.terminal_is_empty() => return super::nested_retained_clone_close_demands::<M::Key>(&self.key_cursor,body),
            VALUE if self.value_cursor.output_ready() => return super::nested_retained_clone_take_demands(&self.value_cursor),
            VALUE => return super::nested_retained_clone_demands::<M::Value>(&self.value_cursor,source.project(index * 2 + 2, |map| map.entry_at(index).expect("validated standard collection entry").1),body,maximum_work_units),
            VALUE_CLOSE if !self.value_cursor.terminal_is_empty() => return super::nested_retained_clone_close_demands::<M::Value>(&self.value_cursor,body),
            INSERT => return Ok(M::insertion_demands(self.building.as_ref(), self.key.as_ref().ok_or_else(|| crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "collection insertion key missing"))?)),
            KEY_CLOSE => demand.copy_bytes=size_of::<<M::Key as RetainedClone>::Cursor>().checked_add(if M::HAS_IMPLICIT_VALUE{size_of::<M::Value>()}else{0}).ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::WorkLimit,"standard collection cursor reset extent overflow"))?,
            VALUE_CLOSE => demand.copy_bytes=size_of::<<M::Value as RetainedClone>::Cursor>(),
            DONE => {}
            _ => return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "standard collection clone quote state is invalid")),
        }
        Ok(demand)
    }

    fn advance(&mut self, source: RetainedCloneRef<'_, M>, body: usize,maximum_work_units:u64, effect:&mut crate::RetainedAllocationEffect<'_>) ->crate::OriginalRetirementOutcome{let grant=effect.grant();let maximum_work_units=maximum_work_units.min(effect.maximum_work_units());let mut work=0;let result=(|| -> Result<RetainedCloneStep,crate::OriginalRetirementFailure>{
        if self.closing { return Err((crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained standard collection clone cursor is closing")).into()); }
        if self.spent { return Err((crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained standard collection clone cursor is spent")).into()); }
        let demand=self.advance_demands(source,body,maximum_work_units)?;
        if self.output_ready() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if !super::retained_clone_grant_admits(grant,demand) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(progress) = {let turn=source.bind(&mut self.source, grant);work+=turn.work;turn.result}? { return Ok(RetainedCloneStep::Progress(progress)); }
        let index = self.index;
        match self.phase {
            SELECT => {
                let count = source.get().entry_count();
                if index > count { return Err((crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained standard collection clone index passed its source")).into()); }
                if index == count {
                    let bytes = size_of::<M>();
                    if grant.maximum_copy_bytes < bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
                    work+=1;self.output = Some(self.building.take().unwrap_or_else(M::empty));
                    self.phase = DONE;
                    return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() }));
                }
                self.phase = KEY;work+=1;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
            }
            KEY => {
                if self.key_cursor.output_ready() {
                    let nested=super::nested_retained_clone_grant(grant)?;let turn=self.key_cursor.take(nested);work+=turn.work;let Some((value,progress))=turn.result? else{return Ok(RetainedCloneStep::Progress(Default::default()))};self.key=Some(value);
                    
                    self.phase=KEY_CLOSE;
                    return Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(nested,progress,"standard collection key take")?));
                }
                let nested=super::nested_retained_clone_grant(grant)?;
                let step={let turn=self.key_cursor.advance(source.project(index*2+1,|map|map.entry_at(index).expect("validated standard collection entry").0),body,maximum_work_units,&mut effect.reborrow(nested,maximum_work_units)?);work+=turn.work;turn.result}?;
                Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(nested,step.progress(),"standard collection key")?))
            }
            KEY_CLOSE => {
                if !self.key_cursor.terminal_is_empty() {
                    if !self.key_cursor.close_started(){let turn=self.key_cursor.begin_close_paid(grant);work+=turn.work;return turn.result.map(RetainedCloneStep::Progress).map_err(Into::into)}
let step = {let turn=self.key_cursor.close_step(&mut effect.reborrow(super::nested_retained_clone_grant(grant)?,effect.maximum_work_units().saturating_sub(work))?);work+=turn.work;turn.result}?;
                    return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, self.key_cursor.terminal_is_empty(), "retained standard collection key scaffold close")?.progress()));
                }
                work+=1;self.key_cursor = <M::Key as RetainedClone>::retained_clone_cursor();
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
                    let nested=super::nested_retained_clone_grant(grant)?;let turn=self.value_cursor.take(nested);work+=turn.work;let Some((value,progress))=turn.result? else{return Ok(RetainedCloneStep::Progress(Default::default()))};self.value=Some(value);
                    
                    self.phase=VALUE_CLOSE;
                    return Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(nested,progress,"standard collection value take")?));
                }
                let nested=super::nested_retained_clone_grant(grant)?;
                let step={let turn=self.value_cursor.advance(source.project(index*2+2,|map|map.entry_at(index).expect("validated standard collection entry").1),body,maximum_work_units,&mut effect.reborrow(nested,maximum_work_units)?);work+=turn.work;turn.result}?;
                Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(nested,step.progress(),"standard collection value")?))
            }
            VALUE_CLOSE => {
                if !self.value_cursor.terminal_is_empty() {
                    if !self.value_cursor.close_started(){let turn=self.value_cursor.begin_close_paid(grant);work+=turn.work;return turn.result.map(RetainedCloneStep::Progress).map_err(Into::into)}
let step = {let turn=self.value_cursor.close_step(&mut effect.reborrow(super::nested_retained_clone_grant(grant)?,effect.maximum_work_units().saturating_sub(work))?);work+=turn.work;turn.result}?;
                    return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, self.value_cursor.terminal_is_empty(), "retained standard collection value scaffold close")?.progress()));
                }
                work+=1;self.value_cursor = <M::Value as RetainedClone>::retained_clone_cursor();
                self.phase = INSERT;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes:demand.copy_bytes, ..Default::default() }))
            }
            INSERT => {
                let demand = M::insertion_demands(self.building.as_ref(), self.key.as_ref().ok_or_else(|| crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "collection insertion key missing"))?);
                let bytes = demand.copy_bytes;
                if grant.maximum_copy_bytes < bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth { return Ok(RetainedCloneStep::Progress(Default::default())); }
                work+=1;let key = self.key.take().ok_or_else(|| crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained standard collection key owner is missing"))?;
                let value = match self.value.take() {
                    Some(value) => value,
                    None => { self.key = Some(key); return Err((crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained standard collection value owner is missing")).into()); }
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
                        Err((crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained standard collection clone produced a duplicate key")).into())
                    }
                }
            }
            DONE => Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())),
            _ => Err((crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained standard collection clone state is invalid")).into()),
        }
    })();crate::RetainedWorkOutcome{result,work}}

    fn take_demand(&self)->Result<crate::RetirementDemand,crate::ValueError>{crate::retained_clone::retained_clone_cursor_take_demand::<M,bool>()}
    fn take<'reply>(&mut self,output:&mut Option<M>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>{crate::retained_clone::take_retained_clone_cursor_output!(self.output_ready(),self.output,output,self.spent,true,effect,reply,M,bool)}

    

    fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementOutcome{let grant=effect.grant();let mut work=0;let result=(||{
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "standard collection must begin close before granted retirement").into()); }
        if !super::retained_clone_grant_admits(grant,super::retained_clone_close_demands::<M>(self,grant.maximum_copy_bytes)?) {return Ok(RetainedCloneStep::Progress(Default::default()));}
        if !self.key_cursor.terminal_is_empty() {
            if !self.key_cursor.close_started(){let turn=self.key_cursor.begin_close_paid(grant);work+=turn.work;return turn.result.map(RetainedCloneStep::Progress).map_err(Into::into)}
            if !self.key_cursor.close_started(){let turn=self.key_cursor.begin_close_paid(grant);work+=turn.work;return turn.result.map(RetainedCloneStep::Progress).map_err(Into::into)}
let step = {let turn=self.key_cursor.close_step(&mut effect.reborrow(super::nested_retained_clone_grant(grant)?,effect.maximum_work_units()).expect("retirement child exceeds original effect"));work+=turn.work;turn.result}?;
            return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, self.key_cursor.terminal_is_empty(), "retained standard collection key close")?.progress()));
        }
        if !self.value_cursor.terminal_is_empty() {
            if !self.value_cursor.close_started(){let turn=self.value_cursor.begin_close_paid(grant);work+=turn.work;return turn.result.map(RetainedCloneStep::Progress).map_err(Into::into)}
            if !self.value_cursor.close_started(){let turn=self.value_cursor.begin_close_paid(grant);work+=turn.work;return turn.result.map(RetainedCloneStep::Progress).map_err(Into::into)}
let step = {let turn=self.value_cursor.close_step(&mut effect.reborrow(super::nested_retained_clone_grant(grant)?,effect.maximum_work_units()).expect("retirement child exceeds original effect"));work+=turn.work;turn.result}?;
            return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, self.value_cursor.terminal_is_empty(), "retained standard collection value close")?.progress()));
        }
        if !self.close.is_empty() { return {let turn=self.close.step_granted(effect);work+=turn.work;turn.result}; }
        if let Some(step) = {let turn=self.close.begin_granted(&mut self.key,effect);work+=turn.work;turn.result}? { return Ok(step); }
        if let Some(step) = {let turn=self.close.begin_granted(&mut self.value,effect);work+=turn.work;turn.result}? { return Ok(step); }
        if let Some(step) = {let turn=self.close.begin_granted(&mut self.building,effect);work+=turn.work;turn.result}? { return Ok(step); }
        if let Some(step) = {let turn=self.close.begin_granted(&mut self.output,effect);work+=turn.work;turn.result}? { return Ok(step); }
        {let turn=close_retained_binding(&mut self.source,effect);work+=turn.work;turn.result}
    })();crate::RetainedWorkOutcome{result,work}}

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

#[cfg(test)]
#[path="🧪️tests/🎯️dispatch/🦀️.rs"]
mod selected_close_tests;
