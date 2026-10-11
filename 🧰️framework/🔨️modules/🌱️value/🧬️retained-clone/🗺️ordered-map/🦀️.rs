//! 🗺️ Fixed-page ordered owners with resumable native key comparison and insertion.

use super::{RetainedClone, RetainedCloneBinding, RetainedCloneClose, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, admit_retained_clone_close, admit_retained_clone_progress, close_retained_binding};
use crate::{retirement::RetireOwned};
use serde::{Serialize, Serializer, ser::SerializeMap};
use std::{cmp::Ordering, mem::size_of};

pub const RETAINED_ORDERED_MAP_PAGE_CAPACITY: usize = 16;

/// 🗺️ Stores strictly ordered entries in fixed-capacity native pages with resumable directory growth.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RetainedOrderedMap<K, V> {
    pages: Vec<Vec<(K, V)>>,
    len: usize,
}

impl<K, V> Default for RetainedOrderedMap<K, V> {
    fn default() -> Self {
        Self { pages: Vec::new(), len: 0 }
    }
}

impl<K, V> RetainedOrderedMap<K, V> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn page_count(&self) -> usize {
        self.pages.len()
    }

    pub fn get_index(&self, ordinal: usize) -> Option<(&K, &V)> {
        if ordinal >= self.len {
            return None;
        }
        let page = self.pages.get(ordinal / RETAINED_ORDERED_MAP_PAGE_CAPACITY)?;
        let (key, value) = page.get(ordinal % RETAINED_ORDERED_MAP_PAGE_CAPACITY)?;
        Some((key, value))
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.pages.iter().flat_map(|page| page.iter().map(|(key, value)| (key, value)))
    }

    #[cfg(test)]
    pub(crate) fn from_sorted_entries_for_test(entries: Vec<(K, V)>) -> Result<Self, crate::ValueError>
    where
        K: Ord,
    {
        if entries.windows(2).any(|pair| pair[0].0 >= pair[1].0) {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvalidValue, "retained ordered-map fixture entries are not strictly ordered"));
        }
        let len = entries.len();
        let page_count = len.div_ceil(RETAINED_ORDERED_MAP_PAGE_CAPACITY);
        let mut pages = Vec::with_capacity(page_count.checked_add(1).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map fixture page count overflow"))?);
        let mut entries = entries.into_iter();
        for _ in 0..page_count {
            let mut page = Vec::with_capacity(RETAINED_ORDERED_MAP_PAGE_CAPACITY);
            page.extend(entries.by_ref().take(RETAINED_ORDERED_MAP_PAGE_CAPACITY));
            pages.push(page);
        }
        Ok(Self { pages, len })
    }
}

/// 🧊️ Constructs and edits standalone cold dictionaries before retained admission.
impl<K: Ord, V> RetainedOrderedMap<K, V> {
    pub fn new() -> Self { Self::default() }
    pub fn get<Q: Ord + ?Sized>(&self, key: &Q) -> Option<&V> where K: std::borrow::Borrow<Q> {
        let ordinal = self.search(key).ok()?;
        self.get_index(ordinal).map(|(_, value)| value)
    }
    fn search<Q: Ord + ?Sized>(&self, key: &Q) -> Result<usize,usize> where K: std::borrow::Borrow<Q> {
        let mut lower=0;let mut upper=self.len;
        while lower<upper {let middle=lower+(upper-lower)/2;match self.get_index(middle).expect("dense map ordinal").0.borrow().cmp(key){Ordering::Less=>lower=middle+1,Ordering::Greater=>upper=middle,Ordering::Equal=>return Ok(middle)}}
        Err(lower)
    }
    pub fn cold_insert(&mut self, key: K, value: V) -> Option<V> {
        let ordinal=match self.search(&key){Ok(ordinal)=>return Some(std::mem::replace(&mut self.pages[ordinal/RETAINED_ORDERED_MAP_PAGE_CAPACITY][ordinal%RETAINED_ORDERED_MAP_PAGE_CAPACITY].1,value)),Err(ordinal)=>ordinal};
        if self.pages.last().is_none_or(|page|page.len()==RETAINED_ORDERED_MAP_PAGE_CAPACITY){self.pages.push(Vec::with_capacity(RETAINED_ORDERED_MAP_PAGE_CAPACITY));}
        let mut carry=(key,value);let mut slot=ordinal%RETAINED_ORDERED_MAP_PAGE_CAPACITY;
        for page in &mut self.pages[ordinal/RETAINED_ORDERED_MAP_PAGE_CAPACITY..] {
            let next=if page.len()==RETAINED_ORDERED_MAP_PAGE_CAPACITY{page.pop()}else{None};page.insert(slot,carry);
            match next{Some(next)=>{carry=next;slot=0},None=>break}
        }
        self.len+=1;None
    }
    pub fn cold_remove<Q: Ord + ?Sized>(&mut self, key:&Q)->Option<V> where K:std::borrow::Borrow<Q> {
        let ordinal=self.search(key).ok()?;let page=ordinal/RETAINED_ORDERED_MAP_PAGE_CAPACITY;
        let (_,value)=self.pages[page].remove(ordinal%RETAINED_ORDERED_MAP_PAGE_CAPACITY);
        for index in page..self.pages.len()-1 {let next=self.pages[index+1].remove(0);self.pages[index].push(next);}
        self.len-=1;if self.pages.last().is_some_and(Vec::is_empty){self.pages.pop();}Some(value)
    }
}
impl<K:Ord,V> FromIterator<(K, V)> for RetainedOrderedMap<K, V> {
    fn from_iter<I:IntoIterator<Item=(K,V)>>(entries:I)->Self{let mut map=Self::default();for(key,value)in entries{map.cold_insert(key,value);}map}
}
impl<K:Ord,V,const N:usize> From<[(K,V);N]> for RetainedOrderedMap<K,V>{fn from(entries:[(K,V);N])->Self{entries.into_iter().collect()}}
impl<'de,K:Ord+serde::Deserialize<'de>,V:serde::Deserialize<'de>> serde::Deserialize<'de> for RetainedOrderedMap<K, V> {
    fn deserialize<D:serde::Deserializer<'de>>(deserializer:D)->Result<Self,D::Error>{
        struct Visitor<K,V>(std::marker::PhantomData<(K,V)>);
        impl<'de,K:Ord+serde::Deserialize<'de>,V:serde::Deserialize<'de>> serde::de::Visitor<'de> for Visitor<K,V>{
            type Value=RetainedOrderedMap<K,V>;
            fn expecting(&self,formatter:&mut std::fmt::Formatter)->std::fmt::Result{formatter.write_str("an object with unique ordered keys")}
            fn visit_map<M:serde::de::MapAccess<'de>>(self,mut input:M)->Result<Self::Value,M::Error>{let mut map=RetainedOrderedMap::default();while let Some((key,value))=input.next_entry::<K,V>()?{if map.get(&key).is_some(){return Err(serde::de::Error::custom("duplicate ordered-map key"));}map.cold_insert(key,value);}Ok(map)}
        }
        deserializer.deserialize_map(Visitor(std::marker::PhantomData))
    }
}
#[path="🚦️native/🦀️.rs"]
mod native;
impl<K: Serialize, V: Serialize> Serialize for RetainedOrderedMap<K, V> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(self.len))?;
        for (key, value) in self.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}

impl<K: RetireOwned, V: RetireOwned> RetireOwned for RetainedOrderedMap<K, V> {
    fn retirement(self) -> Box<dyn crate::retirement::RetirementCursor> {
        self.pages.retirement()
    }
    fn retirement_birth_bytes(&self) -> Option<usize> { self.pages.retirement_birth_bytes() }
    fn controlled_retirement_supported() -> bool { K::controlled_retirement_supported() && V::controlled_retirement_supported() }
}

pub struct RetainedOrderedMapCloneCursor<K: RetainedClone, V: RetainedClone> {
    pages: Vec<Vec<(K, V)>>,
    page_output: Option<Vec<(K, V)>>,
    key: Option<K>,
    value: Option<V>,
    key_cursor: K::Cursor,
    value_cursor: V::Cursor,
    page: usize,
    entry: usize,
    source_page_len: usize,
    phase: u8,
    source: Option<RetainedCloneBinding>,
    output: Option<RetainedOrderedMap<K, V>>,
    spent: bool,
    closing: bool,
    close: RetainedCloneClose,
}

impl<K: RetainedClone, V: RetainedClone> Default for RetainedOrderedMapCloneCursor<K, V> {
    fn default() -> Self {
        Self {
            pages: Vec::new(),
            page_output: None,
            key: None,
            value: None,
            key_cursor: K::retained_clone_cursor(),
            value_cursor: V::retained_clone_cursor(),
            page: 0,
            entry: 0,
            source_page_len: 0,
            phase: 0,
            source: None,
            output: None,
            spent: false,
            closing: false,
            close: RetainedCloneClose::default(),
        }
    }
}

impl<K: RetainedClone, V: RetainedClone> RetainedOrderedMapCloneCursor<K, V> {
    fn progress_from_close(step: RetainedCloneStep, terminal_is_empty: bool, grant: RetainedCloneGrant, context: &'static str) -> Result<RetainedCloneStep, crate::ValueError> {
        let progress = super::admit_retained_clone_close(grant, step, terminal_is_empty, context)?.progress();
        Ok(RetainedCloneStep::Progress(progress))
    }
}

impl<K: RetainedClone, V: RetainedClone> RetainedCloneCursor<RetainedOrderedMap<K, V>> for RetainedOrderedMapCloneCursor<K, V> {
    fn advance_demands(&self,source:RetainedCloneRef<'_,RetainedOrderedMap<K,V>>,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{
        if self.closing||self.spent{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"ordered map cannot quote a closing or spent clone"));}
        if self.output.is_some(){return Ok(Default::default());}
        if self.source.is_none(){return Ok(crate::RetirementDemand{copy_bytes:source.binding_copy_bytes(),depth:1,..Default::default()});}
        let value=source.get();let mut demand=crate::RetirementDemand{depth:1,..Default::default()};
        match self.phase{
            0=>demand.capacity_bytes=value.pages.len().checked_add(1).and_then(|count|count.checked_mul(size_of::<Vec<(K,V)>>())).ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::OwnershipLimit,"ordered map directory quote overflow"))?,
            1=>{if self.page_output.is_some()&&self.entry==self.source_page_len{demand.copy_bytes=size_of::<Vec<(K,V)>>();}else if self.page==value.pages.len(){demand.copy_bytes=size_of::<RetainedOrderedMap<K,V>>();}else if self.page_output.is_none(){demand.capacity_bytes=RETAINED_ORDERED_MAP_PAGE_CAPACITY.checked_mul(size_of::<(K,V)>()).ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::OwnershipLimit,"ordered map page quote overflow"))?;}},
            2=>{let ordinal=self.page*RETAINED_ORDERED_MAP_PAGE_CAPACITY+self.entry;return self.key_cursor.advance_demands(source.project(ordinal*2+1,|map|&map.pages[self.page][self.entry].0),body);},
            3 if !self.key_cursor.terminal_is_empty()=>return Ok(crate::RetirementDemand{copy_bytes:self.key_cursor.next_close_copy_byte_demand()?,capacity_bytes:self.key_cursor.next_close_capacity_byte_demand(body)?,release_bytes:self.key_cursor.next_close_release_byte_demand()?,depth:self.key_cursor.next_close_depth_demand()?}),
            4=>{let ordinal=self.page*RETAINED_ORDERED_MAP_PAGE_CAPACITY+self.entry;return self.value_cursor.advance_demands(source.project(ordinal*2+2,|map|&map.pages[self.page][self.entry].1),body);},
            5 if !self.value_cursor.terminal_is_empty()=>return Ok(crate::RetirementDemand{copy_bytes:self.value_cursor.next_close_copy_byte_demand()?,capacity_bytes:self.value_cursor.next_close_capacity_byte_demand(body)?,release_bytes:self.value_cursor.next_close_release_byte_demand()?,depth:self.value_cursor.next_close_depth_demand()?}),
            6=>demand.copy_bytes=size_of::<(K,V)>(),
            3|5|7=>{},
            _=>return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"ordered map clone quote state is invalid")),
        }
        Ok(demand)
    }
    fn advance(&mut self, source: RetainedCloneRef<'_, RetainedOrderedMap<K, V>>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if self.closing {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map clone cursor is closing"));
        }
        if self.spent {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map clone cursor is spent"));
        }
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if self.output.is_some() {
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
        }
        if let Some(progress)=source.bind(&mut self.source,grant)?{return Ok(RetainedCloneStep::Progress(progress));}
        let source_value = source.get();
        match self.phase {
            0 => {
                let planned_pages = source_value.pages.len().checked_add(1).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map directory page count overflow"))?;
                let planned_capacity = planned_pages.checked_mul(size_of::<Vec<(K, V)>>()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map directory capacity overflow"))?;
                let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: planned_capacity, released_bytes: 0 };
                if !progress.fits(grant) {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                self.pages.try_reserve_exact(planned_pages).map_err(|_| crate::ValueError::new(crate::ValueRefusalKind::AllocationFailed, "retained ordered-map directory allocation failed"))?;
                let actual = self.pages.capacity().checked_mul(size_of::<Vec<(K, V)>>()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map directory actual capacity overflow"))?;
                if actual > grant.maximum_capacity_bytes {
                    return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map directory allocator exceeded its admitted capacity"));
                }
                self.phase = 1;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { retained_capacity_bytes: actual, ..progress }))
            }
            1 => {
                if let Some(page) = self.page_output.take() {
                    if self.entry != self.source_page_len {
                        self.page_output = Some(page);
                    } else {
                        if grant.maximum_copy_bytes < size_of::<Vec<(K,V)>>() {
                            self.page_output = Some(page);
                            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                        }
                        self.pages.push(page);
                        self.page += 1;
                        self.entry = 0;
                        return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes:size_of::<Vec<(K,V)>>(), ..Default::default() }));
                    }
                }
                if self.page == source_value.pages.len() {
                    if grant.maximum_copy_bytes < size_of::<RetainedOrderedMap<K,V>>() {
                        return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                    }
                    self.output = Some(RetainedOrderedMap { pages: std::mem::take(&mut self.pages), len: source_value.len });
                    self.phase = 7;
                    return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes:size_of::<RetainedOrderedMap<K,V>>(), ..Default::default() }));
                }
                if self.page_output.is_some() {
                    self.phase = 2;
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                let source_page = source_value.pages.get(self.page).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map source page is missing"))?;
                if source_page.is_empty() || source_page.len() > RETAINED_ORDERED_MAP_PAGE_CAPACITY {
                    return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map source page violates its fixed capacity"));
                }
                let planned = RETAINED_ORDERED_MAP_PAGE_CAPACITY.checked_mul(size_of::<(K, V)>()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map page capacity overflow"))?;
                let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: planned, released_bytes: 0 };
                if !progress.fits(grant) {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                let mut page = Vec::new();
                page.try_reserve_exact(RETAINED_ORDERED_MAP_PAGE_CAPACITY).map_err(|_| crate::ValueError::new(crate::ValueRefusalKind::AllocationFailed, "retained ordered-map page allocation failed"))?;
                let actual = page.capacity().checked_mul(size_of::<(K, V)>()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map page actual capacity overflow"))?;
                if actual > grant.maximum_capacity_bytes {
                    return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map page allocator exceeded its admitted capacity"));
                }
                self.source_page_len = source_page.len();
                self.page_output = Some(page);
                self.phase = 2;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { retained_capacity_bytes: actual, ..progress }))
            }
            2 => {
                let ordinal = self.page * RETAINED_ORDERED_MAP_PAGE_CAPACITY + self.entry;
                match self.key_cursor.advance(source.project(ordinal * 2 + 1, |map| &map.pages[self.page][self.entry].0), grant)? {
                    RetainedCloneStep::Progress(progress) => Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(grant, progress, "retained ordered-map key")?)),
                    RetainedCloneStep::Complete(progress) => {
                        let progress = admit_retained_clone_progress(grant, progress, "retained ordered-map key")?;
                        self.key = Some(self.key_cursor.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map key completed without an owner"))?);
                        let _ = self.key_cursor.begin_close();
                        self.phase = 3;
                        Ok(RetainedCloneStep::Progress(progress))
                    }
                }
            }
            3 => {
                if !self.key_cursor.terminal_is_empty() {
                    let step = self.key_cursor.close_step(grant)?;
                    return Self::progress_from_close(step, self.key_cursor.terminal_is_empty(), grant, "retained ordered-map key scaffold close");
                }
                if grant.maximum_items == 0 {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                self.key_cursor = K::retained_clone_cursor();
                self.phase = 4;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
            }
            4 => {
                let ordinal = self.page * RETAINED_ORDERED_MAP_PAGE_CAPACITY + self.entry;
                match self.value_cursor.advance(source.project(ordinal * 2 + 2, |map| &map.pages[self.page][self.entry].1), grant)? {
                    RetainedCloneStep::Progress(progress) => Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(grant, progress, "retained ordered-map value")?)),
                    RetainedCloneStep::Complete(progress) => {
                        let progress = admit_retained_clone_progress(grant, progress, "retained ordered-map value")?;
                        self.value = Some(self.value_cursor.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map value completed without an owner"))?);
                        let _ = self.value_cursor.begin_close();
                        self.phase = 5;
                        Ok(RetainedCloneStep::Progress(progress))
                    }
                }
            }
            5 => {
                if !self.value_cursor.terminal_is_empty() {
                    let step = self.value_cursor.close_step(grant)?;
                    return Self::progress_from_close(step, self.value_cursor.terminal_is_empty(), grant, "retained ordered-map value scaffold close");
                }
                if grant.maximum_items == 0 {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                self.value_cursor = V::retained_clone_cursor();
                self.phase = 6;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
            }
            6 => {
                if grant.maximum_copy_bytes < size_of::<(K,V)>() {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                self.page_output.as_mut().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map output page is missing"))?.push((
                    self.key.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map key owner is missing"))?,
                    self.value.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map value owner is missing"))?,
                ));
                self.entry += 1;
                self.phase = 1;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes:size_of::<(K,V)>(), ..Default::default() }))
            }
            7 => Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())),
            _ => Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map clone state is invalid")),
        }
    }

    fn take(&mut self) -> Option<RetainedOrderedMap<K, V>> {
        let output = self.output.take();
        if output.is_some() {
            self.spent = true;
        }
        output
    }

    fn begin_close(&mut self) -> bool {
        if self.closing {
            return false;
        }
        self.closing = true;
        true
    }

    

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "ordered map must begin close before granted retirement")); }
        if !self.key_cursor.terminal_is_empty() {
            if self.key_cursor.begin_close() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
            let step = self.key_cursor.close_step(grant)?;
            return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, self.key_cursor.terminal_is_empty(), "retained ordered-map key close")?.progress()));
        }
        if !self.value_cursor.terminal_is_empty() {
            if self.value_cursor.begin_close() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
            let step = self.value_cursor.close_step(grant)?;
            return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, self.value_cursor.terminal_is_empty(), "retained ordered-map value close")?.progress()));
        }
        if !self.close.is_empty() { return self.close.step_granted(grant); }
        if let Some(step) = self.close.begin_granted(&mut self.key, grant)? { return Ok(step); }
        if let Some(step) = self.close.begin_granted(&mut self.value, grant)? { return Ok(step); }
        if let Some(step) = self.close.begin_granted(&mut self.page_output, grant)? { return Ok(step); }
        if !self.pages.is_empty() || self.pages.capacity() != 0 {
            if let Some(step) = self.close.begin_default_granted(&mut self.pages, grant)? { return Ok(step); }
        }
        if let Some(step) = self.close.begin_granted(&mut self.output, grant)? { return Ok(step); }
        close_retained_binding(&mut self.source, grant)
    }

    fn next_close_depth_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if !self.key_cursor.terminal_is_empty() { return self.key_cursor.next_close_depth_demand(); }
        if !self.value_cursor.terminal_is_empty() { return self.value_cursor.next_close_depth_demand(); }
        self.close.next_owner_depth_with_binding(self.key.is_some()||self.value.is_some()||self.page_output.is_some()||self.pages.capacity()!=0||self.output.is_some(),&self.source)
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if !self.key_cursor.terminal_is_empty() { return self.key_cursor.next_close_copy_byte_demand(); }
        if !self.value_cursor.terminal_is_empty() { return self.value_cursor.next_close_copy_byte_demand(); }
        self.close.next_copy_with_binding(&self.source)
    }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if !self.key_cursor.terminal_is_empty() { return self.key_cursor.next_close_capacity_byte_demand(body); }
        if !self.value_cursor.terminal_is_empty() { return self.value_cursor.next_close_capacity_byte_demand(body); }
        if !self.close.is_empty() { return self.close.next_capacity_byte_demand(body); }
        if self.key.is_some() { return self.close.next_owner_capacity_with_binding::<K>(true,body,&self.source); }
        if self.value.is_some() { return self.close.next_owner_capacity_with_binding::<V>(true,body,&self.source); }
        if self.page_output.is_some() { return self.close.next_owner_capacity_with_binding::<Vec<(K,V)>>(true,body,&self.source); }
        if !self.pages.is_empty()||self.pages.capacity()!=0 { return self.close.next_owner_capacity_with_binding::<Vec<Vec<(K,V)>>>(true,body,&self.source); }
        self.close.next_owner_capacity_with_binding::<RetainedOrderedMap<K,V>>(self.output.is_some(),body,&self.source)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if !self.key_cursor.terminal_is_empty() { return self.key_cursor.next_close_release_byte_demand(); }
        if !self.value_cursor.terminal_is_empty() { return self.value_cursor.next_close_release_byte_demand(); }
        self.close.next_release_with_binding(&self.source)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.pages.is_empty()
            && self.page_output.is_none()
            && self.key.is_none()
            && self.value.is_none()
            && self.output.is_none()
            && self.key_cursor.terminal_is_empty()
            && self.value_cursor.terminal_is_empty()
            && self.close.is_empty()
            && self.source.is_none()
    }
}

impl<K: RetainedClone, V: RetainedClone> RetainedClone for RetainedOrderedMap<K, V> {
    type Cursor = RetainedOrderedMapCloneCursor<K, V>;
    fn retained_clone_cursor() -> Self::Cursor {
        RetainedOrderedMapCloneCursor::default()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BoundedOrdGrant {
    pub maximum_items: usize,
    pub maximum_bytes: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BoundedOrdProgress {
    pub compared_items: usize,
    pub compared_bytes: usize,
}

impl BoundedOrdProgress {
    pub fn fits(self, grant: BoundedOrdGrant) -> bool {
        self.compared_items <= grant.maximum_items && self.compared_bytes <= grant.maximum_bytes
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoundedOrdStep {
    Authority(RetainedCloneProgress),
    Progress(BoundedOrdProgress),
    Complete { ordering: Ordering, progress: BoundedOrdProgress },
}

pub trait BoundedOrd: Ord + Send + Sync + Sized + 'static {
    type Cursor: BoundedOrdCursor<Self>;
    fn bounded_ord_cursor() -> Self::Cursor;
}

pub trait BoundedOrdCursor<T: BoundedOrd>: Send {
    fn advance_retirement_demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>;
    fn begin_close(&mut self)->bool;
    fn next_close_copy_byte_demand(&self)->Result<usize,crate::ValueError>;
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,crate::ValueError>;
    fn next_close_release_byte_demand(&self)->Result<usize,crate::ValueError>;
    fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>;
    fn terminal_is_empty(&self)->bool;
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,crate::ValueError>;
    fn compare(&mut self, left: RetainedCloneRef<'_, T>, right: RetainedCloneRef<'_, T>, grant: BoundedOrdGrant, retirement:RetainedCloneGrant) -> Result<BoundedOrdStep, crate::ValueError>;
}

pub struct ScalarBoundedOrdCursor<T> {
    closing:bool,
    complete: Option<Ordering>,
    left: Option<RetainedCloneBinding>,
    right: Option<RetainedCloneBinding>,
    marker: std::marker::PhantomData<fn() -> T>,
}

impl<T> Default for ScalarBoundedOrdCursor<T> {
    fn default() -> Self {
        Self { closing:false, complete: None, left: None, right: None, marker: std::marker::PhantomData }
    }
}

impl<T: BoundedOrd + Copy> BoundedOrdCursor<T> for ScalarBoundedOrdCursor<T> {
fn advance_retirement_demands(&self,_body:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(if self.left.is_none()||self.right.is_none(){crate::RetirementDemand{copy_bytes:super::RetainedCloneSource::<u64>::constructor_copy_bytes(),depth:1,..Default::default()}}else{Default::default()})}
fn begin_close(&mut self)->bool {let started=!self.closing;self.closing=true;started}
fn next_close_copy_byte_demand(&self)->Result<usize,crate::ValueError>{RetainedCloneBinding::copy_demand(if self.left.is_some(){&self.left}else{&self.right})}
fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,crate::ValueError>{RetainedCloneBinding::capacity_demand(if self.left.is_some(){&self.left}else{&self.right},body)}
fn next_close_release_byte_demand(&self)->Result<usize,crate::ValueError>{RetainedCloneBinding::release_demand(if self.left.is_some(){&self.left}else{&self.right})}
fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{RetainedCloneBinding::depth_demand(if self.left.is_some(){&self.left}else{&self.right})}
fn terminal_is_empty(&self)->bool{self.left.is_none()&&self.right.is_none()}
fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,crate::ValueError>{if !self.closing{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"comparator must begin close"));}super::close_retained_binding(if self.left.is_some(){&mut self.left}else{&mut self.right},grant)}
    fn compare(&mut self, left: RetainedCloneRef<'_, T>, right: RetainedCloneRef<'_, T>, grant: BoundedOrdGrant, retirement:RetainedCloneGrant) -> Result<BoundedOrdStep, crate::ValueError> {
        if self.closing{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"comparator is closing"));}
        if self.left.is_none()&&grant.maximum_items==0{return Ok(BoundedOrdStep::Progress(Default::default()));}

        if let Some(progress)=left.bind(&mut self.left,retirement)?{return Ok(BoundedOrdStep::Authority(progress));}
        if let Some(progress)=right.bind(&mut self.right,retirement)?{return Ok(BoundedOrdStep::Authority(progress));}
        if let Some(ordering)=self.complete{return Ok(BoundedOrdStep::Complete{ordering,progress:Default::default()});}
        let progress = BoundedOrdProgress { compared_items: 1, compared_bytes: size_of::<T>() };
        if !progress.fits(grant) {
            return Ok(BoundedOrdStep::Progress(BoundedOrdProgress::default()));
        }
        let ordering = left.get().cmp(right.get());
        self.complete = Some(ordering);
        Ok(BoundedOrdStep::Complete { ordering, progress })
    }
}

macro_rules! bounded_ord_scalar {
    ($($type:ty),+ $(,)?) => {$ (
        impl BoundedOrd for $type {
            type Cursor = ScalarBoundedOrdCursor<Self>;
            fn bounded_ord_cursor() -> Self::Cursor { ScalarBoundedOrdCursor::default() }
        }
    )+ };
}

bounded_ord_scalar!(bool, char, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

#[derive(Default)]
pub struct StringBoundedOrdCursor {
    initialized:bool,
    closing:bool,
    left: Option<RetainedCloneBinding>,
    right: Option<RetainedCloneBinding>,
    offset: usize,
    complete: Option<Ordering>,
}

impl BoundedOrdCursor<String> for StringBoundedOrdCursor {
fn advance_retirement_demands(&self,_body:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(if self.left.is_none()||self.right.is_none(){crate::RetirementDemand{copy_bytes:super::RetainedCloneSource::<u64>::constructor_copy_bytes(),depth:1,..Default::default()}}else{Default::default()})}
fn begin_close(&mut self)->bool {let started=!self.closing;self.closing=true;started}
fn next_close_copy_byte_demand(&self)->Result<usize,crate::ValueError>{RetainedCloneBinding::copy_demand(if self.left.is_some(){&self.left}else{&self.right})}
fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,crate::ValueError>{RetainedCloneBinding::capacity_demand(if self.left.is_some(){&self.left}else{&self.right},body)}
fn next_close_release_byte_demand(&self)->Result<usize,crate::ValueError>{RetainedCloneBinding::release_demand(if self.left.is_some(){&self.left}else{&self.right})}
fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{RetainedCloneBinding::depth_demand(if self.left.is_some(){&self.left}else{&self.right})}
fn terminal_is_empty(&self)->bool{self.left.is_none()&&self.right.is_none()}
fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,crate::ValueError>{if !self.closing{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"comparator must begin close"));}super::close_retained_binding(if self.left.is_some(){&mut self.left}else{&mut self.right},grant)}
    fn compare(&mut self, left: RetainedCloneRef<'_, String>, right: RetainedCloneRef<'_, String>, grant: BoundedOrdGrant, retirement:RetainedCloneGrant) -> Result<BoundedOrdStep, crate::ValueError> {
        if self.closing{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"comparator is closing"));}
        if self.left.is_none()&&grant.maximum_items==0{return Ok(BoundedOrdStep::Progress(Default::default()));}

        let first = !self.initialized;
        if first&&grant.maximum_items==0{return Ok(BoundedOrdStep::Progress(Default::default()));}
        if let Some(progress)=left.bind(&mut self.left,retirement)?{return Ok(BoundedOrdStep::Authority(progress));}
        if let Some(progress)=right.bind(&mut self.right,retirement)?{return Ok(BoundedOrdStep::Authority(progress));}
        if let Some(ordering)=self.complete{return Ok(BoundedOrdStep::Complete{ordering,progress:Default::default()});}
        if first {self.initialized=true;
            if grant.maximum_items == 0 {
                return Ok(BoundedOrdStep::Progress(BoundedOrdProgress::default()));
            }
            return Ok(BoundedOrdStep::Progress(BoundedOrdProgress { compared_items: 1, compared_bytes: 0 }));
        }
        let left = left.get().as_bytes();
        let right = right.get().as_bytes();
        let common = left.len().min(right.len());
        if self.offset == common {
            if grant.maximum_items == 0 {
                return Ok(BoundedOrdStep::Progress(BoundedOrdProgress::default()));
            }
            let ordering = left.len().cmp(&right.len());
            self.complete = Some(ordering);
            return Ok(BoundedOrdStep::Complete { ordering, progress: BoundedOrdProgress { compared_items: 1, compared_bytes: 0 } });
        }
        if grant.maximum_bytes == 0 {
            return Ok(BoundedOrdStep::Progress(BoundedOrdProgress::default()));
        }
        let end = common.min(self.offset.saturating_add(grant.maximum_bytes));
        let start = self.offset;
        while self.offset < end {
            let ordering = left[self.offset].cmp(&right[self.offset]);
            self.offset += 1;
            if ordering != Ordering::Equal {
                self.complete = Some(ordering);
                return Ok(BoundedOrdStep::Complete { ordering, progress: BoundedOrdProgress { compared_items: 0, compared_bytes: self.offset - start } });
            }
        }
        Ok(BoundedOrdStep::Progress(BoundedOrdProgress { compared_items: 0, compared_bytes: self.offset - start }))
    }
}

impl BoundedOrd for String {
    type Cursor = StringBoundedOrdCursor;
    fn bounded_ord_cursor() -> Self::Cursor {
        StringBoundedOrdCursor::default()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedOrderedMapLookup {
    Found(usize),
    Missing(usize),
}

pub struct RetainedOrderedMapLookupCursor<K: BoundedOrd> {
    initialized:bool,
    low: usize,
    high: usize,
    middle: usize,
    comparison: Option<K::Cursor>,
    pending: Option<Ordering>,
    source: Option<RetainedCloneBinding>,
    target: Option<RetainedCloneBinding>,
    complete: Option<RetainedOrderedMapLookup>,
    closing:bool,
}

impl<K: BoundedOrd> Default for RetainedOrderedMapLookupCursor<K> {
    fn default() -> Self {
        Self { initialized:false,low: 0, high: 0, middle: 0, comparison: None, pending: None, source: None, target:None, complete: None, closing:false }
    }
}

impl<K:BoundedOrd> RetainedOrderedMapLookupCursor<K>{
    pub fn advance<V>(&mut self,source:RetainedCloneRef<'_,RetainedOrderedMap<K,V>>,target:RetainedCloneRef<'_,K>,grant:BoundedOrdGrant,retirement:RetainedCloneGrant)->Result<(Option<RetainedOrderedMapLookup>,BoundedOrdProgress,RetainedCloneProgress),crate::ValueError>{
        if self.closing{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"lookup is closing"));}
        let first=!self.initialized;
        if first&&grant.maximum_items==0{return Ok((None,Default::default(),Default::default()));}
        if let Some(progress)=source.bind(&mut self.source,retirement)?{return Ok((None,Default::default(),progress));}
        if let Some(progress)=target.bind(&mut self.target,retirement)?{return Ok((None,Default::default(),progress));}
        if let Some(result)=self.complete{return Ok((Some(result),Default::default(),Default::default()));}
        let map=source.get();
        if first{self.initialized=true;self.high=map.len();return Ok((None,BoundedOrdProgress{compared_items:1,compared_bytes:0},Default::default()));}
        if let Some(ordering)=self.pending{
            if let Some(comparison)=self.comparison.as_mut(){
                comparison.begin_close();let step=comparison.close_step(retirement)?;
                if comparison.terminal_is_empty(){self.comparison=None;}
                return Ok((None,Default::default(),step.progress()));
            }
            if grant.maximum_items==0{return Ok((None,Default::default(),Default::default()));}
            self.pending=None;
            match ordering{Ordering::Less=>self.low=self.middle+1,Ordering::Greater=>self.high=self.middle,Ordering::Equal=>unreachable!()}
            return Ok((None,BoundedOrdProgress{compared_items:1,compared_bytes:0},Default::default()));
        }
        if self.low==self.high{
            if grant.maximum_items==0{return Ok((None,Default::default(),Default::default()));}
            let result=RetainedOrderedMapLookup::Missing(self.low);self.complete=Some(result);
            return Ok((Some(result),BoundedOrdProgress{compared_items:1,compared_bytes:0},Default::default()));
        }
        self.middle=self.low+(self.high-self.low)/2;
        let comparison=self.comparison.get_or_insert_with(K::bounded_ord_cursor);
        match comparison.compare(source.project(self.middle+1,|map|map.get_index(self.middle).unwrap().0),target,grant,retirement)?{
            BoundedOrdStep::Authority(progress)=>Ok((None,Default::default(),progress)),
            BoundedOrdStep::Progress(p)=>Ok((None,p,Default::default())),
            BoundedOrdStep::Complete{ordering:Ordering::Equal,progress}=>{let result=RetainedOrderedMapLookup::Found(self.middle);self.complete=Some(result);Ok((Some(result),progress,Default::default()))},
            BoundedOrdStep::Complete{ordering,progress}=>{self.pending=Some(ordering);Ok((None,progress,Default::default()))}
        }
    }
    pub fn advance_retirement_demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{
        if self.complete.is_some(){return Ok(Default::default());}
        if self.source.is_none()||self.target.is_none(){return Ok(crate::RetirementDemand{copy_bytes:super::RetainedCloneSource::<u64>::constructor_copy_bytes(),depth:1,..Default::default()});}
        if self.pending.is_some(){return self.comparison.as_ref().map_or(Ok(Default::default()),|cursor|Ok(crate::RetirementDemand{copy_bytes:cursor.next_close_copy_byte_demand()?,capacity_bytes:cursor.next_close_capacity_byte_demand(body)?,release_bytes:cursor.next_close_release_byte_demand()?,depth:cursor.next_close_depth_demand()?}));}
        if !self.initialized||self.low==self.high{return Ok(Default::default());}
        self.comparison.as_ref().map_or(Ok(crate::RetirementDemand{copy_bytes:super::RetainedCloneSource::<u64>::constructor_copy_bytes(),depth:1,..Default::default()}),|cursor|cursor.advance_retirement_demands(body))
    }
    pub fn begin_close(&mut self)->bool{let started=!self.closing;self.closing=true;if let Some(c)=self.comparison.as_mut(){c.begin_close();}started}
    pub fn next_close_copy_byte_demand(&self)->Result<usize,crate::ValueError>{if let Some(c)=self.comparison.as_ref(){c.next_close_copy_byte_demand()}else{RetainedCloneBinding::copy_demand(if self.source.is_some(){&self.source}else{&self.target})}}
    pub fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,crate::ValueError>{if let Some(c)=self.comparison.as_ref(){c.next_close_capacity_byte_demand(body)}else{RetainedCloneBinding::capacity_demand(if self.source.is_some(){&self.source}else{&self.target},body)}}
    pub fn next_close_release_byte_demand(&self)->Result<usize,crate::ValueError>{if let Some(c)=self.comparison.as_ref(){c.next_close_release_byte_demand()}else{RetainedCloneBinding::release_demand(if self.source.is_some(){&self.source}else{&self.target})}}
    pub fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{if let Some(c)=self.comparison.as_ref(){c.next_close_depth_demand()}else{RetainedCloneBinding::depth_demand(if self.source.is_some(){&self.source}else{&self.target})}}
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,crate::ValueError>{
        if !self.closing{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"lookup must begin close"));}
        if let Some(c)=self.comparison.as_mut(){let step=c.close_step(grant)?;if c.terminal_is_empty(){self.comparison=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
        super::close_retained_binding(if self.source.is_some(){&mut self.source}else{&mut self.target},grant)
    }
    pub fn terminal_is_empty(&self)->bool{self.comparison.is_none()&&self.source.is_none()&&self.target.is_none()}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedOrderedMapInsertStep {
    Progress(RetainedOrderedMapInsertProgress),
    Complete { ordinal: usize, progress: RetainedOrderedMapInsertProgress },
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RetainedOrderedMapInsertGrant {
    pub retirement:RetainedCloneGrant,
    pub comparison: BoundedOrdGrant,
    pub maximum_moved_items: usize,
    pub maximum_moved_bytes: usize,
    pub maximum_capacity_bytes: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RetainedOrderedMapInsertProgress {
    pub retirement:RetainedCloneProgress,
    pub comparison: BoundedOrdProgress,
    pub moved_items: usize,
    pub moved_bytes: usize,
    pub retained_capacity_bytes: usize,
}

impl RetainedOrderedMapInsertProgress {
    pub fn fits(self, grant: RetainedOrderedMapInsertGrant) -> bool {
        self.retirement.fits(grant.retirement) && self.comparison.fits(grant.comparison) && self.moved_items <= grant.maximum_moved_items && self.moved_bytes <= grant.maximum_moved_bytes && self.retained_capacity_bytes <= grant.maximum_capacity_bytes
    }
}

/// 🧯 Owns one exclusive unpublished workspace through completion or bounded cancellation.
#[derive(crate::RetireOwned)]
struct InsertWorkspace<K:BoundedOrd+RetireOwned,V:RetireOwned+Sync>{map:RetainedOrderedMap<K,V>,key:K,value:V}
struct InsertState<K: BoundedOrd + RetireOwned, V: RetireOwned+Sync> {
    map: Option<RetainedOrderedMap<K, V>>,
    output: Option<RetainedOrderedMap<K, V>>,
    key: Option<K>,
    value: Option<V>,
    lookup: RetainedOrderedMapLookupCursor<K>,
    lookup_lease: super::RetainedCloneBorrowAuthority<InsertWorkspace<K,V>>,
    target: Option<usize>,
    shift_page: Option<usize>,
    old_directory: Option<Vec<Vec<(K, V)>>>,
    new_directory: Option<Vec<Vec<(K, V)>>>,
    directory_index: usize,
    phase: u8,
    spent: bool,
    closing: bool,
    close: RetainedCloneClose,
}

pub struct RetainedOrderedMapInsertCursor<K:BoundedOrd+RetireOwned,V:RetireOwned+Sync>{state:std::mem::ManuallyDrop<InsertState<K,V>>}
impl<K: BoundedOrd + RetireOwned, V: RetireOwned+Sync> RetainedOrderedMapInsertCursor<K, V> {
    pub fn constructor_capacity_bytes()->usize{super::RetainedCloneBorrowAuthority::<InsertWorkspace<K,V>>::constructor_capacity_bytes()}
    pub fn constructor_copy_bytes()->usize{super::RetainedCloneBorrowAuthority::<InsertWorkspace<K,V>>::constructor_copy_bytes()}
    pub fn admit(map:RetainedOrderedMap<K,V>,key:K,value:V,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(crate::ValueError,RetainedOrderedMap<K,V>,K,V)>{
        if grant.maximum_items==0{return Err((crate::ValueError::literal(crate::ValueRefusalKind::WorkLimit,"insertion constructor requires an item"),map,key,value));}
        let(lookup_lease,receipt)=match super::RetainedCloneBorrowAuthority::admit(InsertWorkspace{map,key,value},grant){Ok(v)=>v,Err((error,InsertWorkspace{map,key,value}))=>return Err((error,map,key,value))};
        Ok((Self {state:std::mem::ManuallyDrop::new(InsertState {
            map: None,
            output: None,
            key: None,
            value: None,
            lookup: Default::default(),
            lookup_lease,
            target: None,
            shift_page: None,
            old_directory: None,
            new_directory: None,
            directory_index: 0,
            phase: 0,
            spent: false,
            closing: false,
            close: Default::default(),
        })},receipt))
    }

    pub fn advance_retirement_demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{ let state=&*self.state;
        if state.phase!=0{return Ok(Default::default());}
        if !state.lookup.closing{return state.lookup.advance_retirement_demands(body);}
        if !state.lookup.terminal_is_empty(){return Ok(crate::RetirementDemand{copy_bytes:state.lookup.next_close_copy_byte_demand()?,capacity_bytes:state.lookup.next_close_capacity_byte_demand(body)?,release_bytes:state.lookup.next_close_release_byte_demand()?,depth:state.lookup.next_close_depth_demand()?});}
        Ok(crate::RetirementDemand{copy_bytes:state.lookup_lease.next_take_copy_byte_demand()?,capacity_bytes:state.lookup_lease.next_take_capacity_byte_demand(body)?,release_bytes:state.lookup_lease.next_take_release_byte_demand()?,depth:state.lookup_lease.next_take_depth_demand()?})
    }
    pub fn advance(&mut self, grant: RetainedOrderedMapInsertGrant) -> Result<RetainedOrderedMapInsertStep, crate::ValueError> { let state=&mut *self.state;
        if state.closing {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion cursor is closing"));
        }
        if state.spent {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion cursor is spent"));
        }
        match state.phase {
            0 => {
                if state.lookup.closing{
                    if !state.lookup.terminal_is_empty(){let step=state.lookup.close_step(grant.retirement)?;return Ok(RetainedOrderedMapInsertStep::Progress(RetainedOrderedMapInsertProgress{retirement:step.progress(),..Default::default()}));}
                    let demand=crate::RetirementDemand{copy_bytes:state.lookup_lease.next_take_copy_byte_demand()?,capacity_bytes:state.lookup_lease.next_take_capacity_byte_demand(grant.retirement.maximum_copy_bytes)?,release_bytes:state.lookup_lease.next_take_release_byte_demand()?,depth:state.lookup_lease.next_take_depth_demand()?};
                    if grant.retirement.maximum_items==0||demand.copy_bytes>grant.retirement.maximum_copy_bytes||demand.capacity_bytes>grant.retirement.maximum_capacity_bytes||demand.release_bytes>grant.retirement.maximum_release_bytes||demand.depth>grant.retirement.maximum_depth{return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));}
                    match state.lookup_lease.take_authority(grant.retirement)?{
                        super::RetainedCloneSourceTake::Pending(retirement)=>return Ok(RetainedOrderedMapInsertStep::Progress(RetainedOrderedMapInsertProgress{retirement,..Default::default()})),
                        super::RetainedCloneSourceTake::Ready(InsertWorkspace{map,key,value},retirement)=>{state.map=Some(map);state.key=Some(key);state.value=Some(value);match state.lookup.complete.unwrap(){RetainedOrderedMapLookup::Found(_)=>{state.phase=255;},RetainedOrderedMapLookup::Missing(ordinal)=>{state.target=Some(ordinal);state.phase=1;}}return Ok(RetainedOrderedMapInsertStep::Progress(RetainedOrderedMapInsertProgress{retirement,..Default::default()}));}
                    }
                }
                let map_ref=state.lookup_lease.borrow(1,|workspace|&workspace.map);
                let key_ref=state.lookup_lease.borrow(2,|workspace|&workspace.key);
                let(result,comparison,retirement)=state.lookup.advance(map_ref,key_ref,grant.comparison,grant.retirement)?;
                if result.is_some(){state.lookup.begin_close();}
                Ok(RetainedOrderedMapInsertStep::Progress(RetainedOrderedMapInsertProgress{comparison,retirement,..Default::default()}))
            }
            255=>Err(crate::ValueError::literal(crate::ValueRefusalKind::InvalidValue,"retained ordered-map insertion refused a duplicate key")),
            1 => {
                let map = state.map.as_mut().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion workspace is missing"))?;
                if map.len % RETAINED_ORDERED_MAP_PAGE_CAPACITY == 0 {
                    if map.pages.len() == map.pages.capacity() {
                        state.phase = 10;
                        return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                    }
                    let capacity = RETAINED_ORDERED_MAP_PAGE_CAPACITY.checked_mul(size_of::<(K, V)>()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map insertion page capacity overflow"))?;
                    let progress = RetainedOrderedMapInsertProgress { retained_capacity_bytes: capacity, ..Default::default() };
                    if !progress.fits(grant) {
                        return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                    }
                    let mut page = Vec::new();
                    page.try_reserve_exact(RETAINED_ORDERED_MAP_PAGE_CAPACITY).map_err(|_| crate::ValueError::new(crate::ValueRefusalKind::AllocationFailed, "retained ordered-map insertion page allocation failed"))?;
                    let actual = page.capacity().checked_mul(size_of::<(K, V)>()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map insertion page actual capacity overflow"))?;
                    if actual > grant.maximum_capacity_bytes {
                        return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion page allocator exceeded its admitted capacity"));
                    }
                    map.pages.push(page);
                    state.shift_page = map.pages.len().checked_sub(1);
                    state.phase = 2;
                    return Ok(RetainedOrderedMapInsertStep::Progress(RetainedOrderedMapInsertProgress { retained_capacity_bytes: actual, ..progress }));
                }
                if grant.maximum_moved_items == 0 {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                state.shift_page = map.pages.len().checked_sub(1);
                state.phase = 2;
                Ok(RetainedOrderedMapInsertStep::Progress(RetainedOrderedMapInsertProgress::default()))
            }
            2 => {
                let map = state.map.as_mut().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion workspace is missing"))?;
                let ordinal = state.target.ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion target is missing"))?;
                let target_page = ordinal / RETAINED_ORDERED_MAP_PAGE_CAPACITY;
                let shift_page = state.shift_page.ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion shift page is missing"))?;
                if shift_page == target_page {
                    state.phase = 3;
                    return Ok(RetainedOrderedMapInsertStep::Progress(RetainedOrderedMapInsertProgress::default()));
                }
                let (left, right) = map.pages.split_at_mut(shift_page);
                let previous = left.get_mut(shift_page - 1).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion previous page is missing"))?;
                let current = right.first_mut().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion current page is missing"))?;
                if current.len() >= RETAINED_ORDERED_MAP_PAGE_CAPACITY {
                    return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion shift page has no admitted slot"));
                }
                let moved_items = current.len().checked_add(1).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::WorkLimit, "retained ordered-map insertion move count overflow"))?;
                let moved_bytes = moved_items.checked_mul(size_of::<(K, V)>()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::WorkLimit, "retained ordered-map insertion shift overflow"))?;
                let progress = RetainedOrderedMapInsertProgress { moved_items, moved_bytes, ..Default::default() };
                if !progress.fits(grant) {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                let entry = previous.pop().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion previous page is empty"))?;
                current.insert(0, entry);
                state.shift_page = Some(shift_page - 1);
                Ok(RetainedOrderedMapInsertStep::Progress(progress))
            }
            3 => {
                let map = state.map.as_mut().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion workspace is missing"))?;
                let ordinal = state.target.ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion target is missing"))?;
                let page_index = ordinal / RETAINED_ORDERED_MAP_PAGE_CAPACITY;
                let entry_index = ordinal % RETAINED_ORDERED_MAP_PAGE_CAPACITY;
                let page = map.pages.get_mut(page_index).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion target page is missing"))?;
                if page.len() >= RETAINED_ORDERED_MAP_PAGE_CAPACITY {
                    return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion target page has no admitted slot"));
                }
                let moved_items = page.len().saturating_sub(entry_index).checked_add(1).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::WorkLimit, "retained ordered-map insertion move count overflow"))?;
                let moved_bytes = moved_items.checked_mul(size_of::<(K, V)>()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::WorkLimit, "retained ordered-map insertion target shift overflow"))?;
                let progress = RetainedOrderedMapInsertProgress { moved_items, moved_bytes, ..Default::default() };
                if !progress.fits(grant) {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                let new_len = map.len.checked_add(1).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map insertion length overflow"))?;
                page.insert(
                    entry_index,
                    (
                        state.key.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion key owner is missing"))?,
                        state.value.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion value owner is missing"))?,
                    ),
                );
                map.len = new_len;
                state.output = state.map.take();
                state.phase = 4;
                Ok(RetainedOrderedMapInsertStep::Complete { ordinal, progress })
            }
            4 => Ok(RetainedOrderedMapInsertStep::Complete {
                ordinal: state.target.ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion target is missing"))?,
                progress: RetainedOrderedMapInsertProgress::default(),
            }),
            10 => {
                let map = state.map.as_ref().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion workspace is missing"))?;
                let planned = map.pages.len().checked_mul(2).and_then(|value| value.checked_add(1)).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map directory growth overflow"))?;
                let capacity = planned.checked_mul(size_of::<Vec<(K, V)>>()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map directory capacity overflow"))?;
                let progress = RetainedOrderedMapInsertProgress { retained_capacity_bytes: capacity, ..Default::default() };
                if !progress.fits(grant) {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                let mut directory = Vec::new();
                directory.try_reserve_exact(planned).map_err(|_| crate::ValueError::new(crate::ValueRefusalKind::AllocationFailed, "retained ordered-map directory growth allocation failed"))?;
                let actual = directory.capacity().checked_mul(size_of::<Vec<(K, V)>>()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map directory actual capacity overflow"))?;
                if actual > grant.maximum_capacity_bytes {
                    return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map directory allocator exceeded its admitted capacity"));
                }
                state.new_directory = Some(directory);
                state.phase = 11;
                Ok(RetainedOrderedMapInsertStep::Progress(RetainedOrderedMapInsertProgress { retained_capacity_bytes: actual, ..progress }))
            }
            11 => {
                let progress = RetainedOrderedMapInsertProgress { moved_items: 1, moved_bytes: size_of::<Vec<Vec<(K, V)>>>(), ..Default::default() };
                if !progress.fits(grant) {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                let map = state.map.as_mut().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion workspace is missing"))?;
                state.old_directory = Some(std::mem::take(&mut map.pages));
                state.phase = 12;
                Ok(RetainedOrderedMapInsertStep::Progress(progress))
            }
            12 => {
                let old = state.old_directory.as_mut().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map old directory is missing"))?;
                if state.directory_index == old.len() {
                    state.phase = 13;
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                let progress = RetainedOrderedMapInsertProgress { moved_items: 1, moved_bytes: size_of::<Vec<(K, V)>>(), ..Default::default() };
                if !progress.fits(grant) {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                state.new_directory.as_mut().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map new directory is missing"))?.push(std::mem::take(&mut old[state.directory_index]));
                state.directory_index += 1;
                Ok(RetainedOrderedMapInsertStep::Progress(progress))
            }
            13 => {
                let progress = RetainedOrderedMapInsertProgress { moved_items: 1, moved_bytes: size_of::<Vec<Vec<(K, V)>>>(), ..Default::default() };
                if !progress.fits(grant) {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                state.map.as_mut().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion workspace is missing"))?.pages =
                    state.new_directory.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map relocated directory is missing"))?;
                state.directory_index = 0;
                state.phase = 1;
                Ok(RetainedOrderedMapInsertStep::Progress(progress))
            }
            _ => Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion state is invalid")),
        }
    }

    pub fn output_ready(&self)->bool{let state=&*self.state;state.output.is_some()&&!state.closing&&!state.spent}
    pub fn refused_workspace_ready(&self)->bool{let state=&*self.state;state.phase==255&&state.map.is_some()&&!state.closing&&!state.spent}
    pub fn take(&mut self) -> Option<RetainedOrderedMap<K, V>> { let state=&mut *self.state;
        let output = state.output.take();
        if output.is_some() {
            state.spent = true;
        }
        output
    }

    pub fn take_refused_workspace(&mut self) -> Option<RetainedOrderedMap<K, V>> { let state=&mut *self.state;
        if state.phase != 255 {
            return None;
        }
        let map = state.map.take();
        if map.is_some() {
            state.spent = true;
        }
        map
    }

    pub fn begin_close(&mut self) -> bool { let state=&mut *self.state;
        if state.closing {
            return false;
        }
        state.closing = true;
        true
    }

    pub fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> { let state=&*self.state; if !state.lookup.terminal_is_empty(){state.lookup.next_close_copy_byte_demand()}else if !state.close.is_empty(){state.close.next_copy_byte_demand()}else if state.key.is_some()||state.value.is_some()||state.old_directory.is_some()||state.new_directory.is_some()||state.map.is_some()||state.output.is_some(){Ok(0)}else{state.lookup_lease.next_close_copy_byte_demand()} }
    pub fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, crate::ValueError> { let state=&*self.state;
        if !state.lookup.terminal_is_empty(){return state.lookup.next_close_capacity_byte_demand(body);}
        if !state.close.is_empty() { return state.close.next_capacity_byte_demand(body); }
        if state.key.is_some() { return state.close.next_owner_capacity_byte_demand::<K>(true, body); }
        if state.value.is_some() { return state.close.next_owner_capacity_byte_demand::<V>(true, body); }
        if state.old_directory.is_some() || state.new_directory.is_some() { return state.close.next_owner_capacity_byte_demand::<Vec<Vec<(K,V)>>>(true, body); }
        if state.map.is_some()||state.output.is_some(){state.close.next_owner_capacity_byte_demand::<RetainedOrderedMap<K,V>>(true,body)}else{state.lookup_lease.next_close_capacity_byte_demand(body)}
    }
    pub fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> { let state=&*self.state; if !state.lookup.terminal_is_empty(){state.lookup.next_close_release_byte_demand()}else if !state.close.is_empty(){state.close.next_release_byte_demand()}else if state.key.is_some()||state.value.is_some()||state.old_directory.is_some()||state.new_directory.is_some()||state.map.is_some()||state.output.is_some(){Ok(0)}else{state.lookup_lease.next_close_release_byte_demand()} }
    pub fn next_close_depth_demand(&self) -> Result<usize, crate::ValueError> { let state=&*self.state; if !state.lookup.terminal_is_empty(){state.lookup.next_close_depth_demand()}else if !state.close.is_empty(){state.close.next_depth_demand()}else if state.key.is_some()||state.value.is_some()||state.old_directory.is_some()||state.new_directory.is_some()||state.map.is_some()||state.output.is_some(){Ok(1)}else{state.lookup_lease.next_close_depth_demand()} }
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> { let state=&mut *self.state;
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !state.closing { return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "ordered-map insertion must begin close before retiring ownership")); }
        if !state.lookup.terminal_is_empty(){state.lookup.begin_close();return state.lookup.close_step(grant);}
        if !state.close.is_empty() { return state.close.step_granted(grant); }
        if let Some(step) = state.close.begin_granted(&mut state.key, grant)? { return Ok(step); }
        if let Some(step) = state.close.begin_granted(&mut state.value, grant)? { return Ok(step); }
        if let Some(step) = state.close.begin_granted(&mut state.old_directory, grant)? { return Ok(step); }
        if let Some(step) = state.close.begin_granted(&mut state.new_directory, grant)? { return Ok(step); }
        if let Some(step) = state.close.begin_granted(&mut state.map, grant)? { return Ok(step); }
        if let Some(step) = state.close.begin_granted(&mut state.output, grant)? { return Ok(step); }
        state.lookup_lease.close_step(grant)
    }

    pub fn terminal_is_empty(&self) -> bool { let state=&*self.state;
        state.lookup.terminal_is_empty()&&state.lookup_lease.terminal_is_empty()&&state.closing && state.key.is_none() && state.value.is_none() && state.old_directory.is_none() && state.new_directory.is_none() && state.map.is_none() && state.output.is_none() && state.close.is_empty()
    }
}

impl<K:BoundedOrd+RetireOwned,V:RetireOwned+Sync> Drop for RetainedOrderedMapInsertCursor<K,V>{fn drop(&mut self){if self.terminal_is_empty(){unsafe{std::mem::ManuallyDrop::drop(&mut self.state)}}else if !std::thread::panicking(){panic!("insertion original owners require caller-funded closure")}}}
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path="➖️remove/🧪️tests/🦀️.rs"]
mod removal_tests;

#[path="➖️remove/🦀️.rs"]
mod removal;
pub use removal::{RetainedOrderedMapRemoveCursor,RetainedOrderedMapRemoveGrant,RetainedOrderedMapRemoveProgress,RetainedOrderedMapRemoveStep};

#[path="♻️retirement/🦀️.rs"]
mod edit_retirement;
