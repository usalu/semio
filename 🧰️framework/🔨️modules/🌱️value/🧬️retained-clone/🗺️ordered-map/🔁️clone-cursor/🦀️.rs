//! 🔁️ Clone cursor of the retained ordered map: the resumable cursor that copies a map page by page and its retained clone implementation. See [`super`].

use super::*;

pub struct RetainedOrderedMapCloneCursor<K: RetainedClone, V: RetainedClone> {
    pub(super) pages: Vec<Vec<(K, V)>>,
    page_output: Option<Vec<(K, V)>>,
    pub(super) key: Option<K>,
    pub(super) value: Option<V>,
    key_cursor: K::Cursor,
    value_cursor: V::Cursor,
    pub(super) page: usize,
    pub(super) entry: usize,
    source_page_len: usize,
    pub(super) phase: u8,
    pub(super) source: Option<RetainedCloneBinding>,
    pub(super) output: Option<RetainedOrderedMap<K, V>>,
    pub(super) spent: bool,
    pub(super) closing: bool,
    pub(super) close: RetainedCloneClose,
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
        let progress = admit_retained_clone_close(grant, step, terminal_is_empty, context)?.progress();
        Ok(RetainedCloneStep::Progress(progress))
    }
}

impl<K:RetainedClone,V:RetainedClone> RetainedOrderedMapCloneCursor<K,V>{
    fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{
        if !self.closing{return Ok(Default::default());}
        if !self.key_cursor.terminal_is_empty(){return super::nested_retained_clone_close_demands::<K>(&self.key_cursor,body);}
        if !self.value_cursor.terminal_is_empty(){return super::nested_retained_clone_close_demands::<V>(&self.value_cursor,body);}
        if self.key.is_some(){return self.close.next_owner_demands::<K>(true,&self.source,body);}
        if self.value.is_some(){return self.close.next_owner_demands::<V>(true,&self.source,body);}
        if self.page_output.is_some(){return self.close.next_owner_demands::<Vec<(K,V)>>(true,&self.source,body);}
        if !self.pages.is_empty()||self.pages.capacity()!=0{return self.close.next_owner_demands::<Vec<Vec<(K,V)>>>(true,&self.source,body);}
        self.close.next_owner_demands::<RetainedOrderedMap<K,V>>(self.output.is_some(),&self.source,body)
    }
}
impl<K: RetainedClone, V: RetainedClone> RetainedCloneCursor<RetainedOrderedMap<K, V>> for RetainedOrderedMapCloneCursor<K, V> {
    fn output_ready(&self)->bool{self.output.is_some()}
    fn advance_demands(&self,source:RetainedCloneRef<'_,RetainedOrderedMap<K,V>>,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{
        if self.closing||self.spent{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"ordered map cannot quote a closing or spent clone"));}
        if self.output.is_some(){return Ok(Default::default());}
        if !source.check_binding(&self.source)?{return Ok(crate::RetirementDemand{copy_bytes:source.binding_copy_bytes(),depth:1,..Default::default()});}
        let value=source.get();let mut demand=crate::RetirementDemand{depth:1,..Default::default()};
        match self.phase{
            0=>demand.capacity_bytes=value.pages.len().checked_add(1).and_then(|count|count.checked_mul(size_of::<Vec<(K,V)>>())).ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::OwnershipLimit,"ordered map directory quote overflow"))?,
            1=>{if self.page_output.is_some()&&self.entry==self.source_page_len{demand.copy_bytes=size_of::<Vec<(K,V)>>();}else if self.page==value.pages.len(){demand.copy_bytes=size_of::<RetainedOrderedMap<K,V>>();}else if self.page_output.is_none(){demand.capacity_bytes=RETAINED_ORDERED_MAP_PAGE_CAPACITY.checked_mul(size_of::<(K,V)>()).ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::OwnershipLimit,"ordered map page quote overflow"))?;}},
            2=>{if self.key_cursor.output_ready(){demand.copy_bytes=size_of::<K>();}else{let ordinal=self.page*RETAINED_ORDERED_MAP_PAGE_CAPACITY+self.entry;return super::nested_retained_clone_demands::<K>(&self.key_cursor,source.project(ordinal*2+1,|map|&map.pages[self.page][self.entry].0),body);}},
            3 if !self.key_cursor.terminal_is_empty()=>return super::nested_retained_clone_close_demands::<K>(&self.key_cursor,body),
            4=>{if self.value_cursor.output_ready(){demand.copy_bytes=size_of::<V>();}else{let ordinal=self.page*RETAINED_ORDERED_MAP_PAGE_CAPACITY+self.entry;return super::nested_retained_clone_demands::<V>(&self.value_cursor,source.project(ordinal*2+2,|map|&map.pages[self.page][self.entry].1),body);}},
            5 if !self.value_cursor.terminal_is_empty()=>return super::nested_retained_clone_close_demands::<V>(&self.value_cursor,body),
            6=>demand.copy_bytes=size_of::<(K,V)>(),
            3=>demand.copy_bytes=size_of::<K::Cursor>(),
            5=>demand.copy_bytes=size_of::<V::Cursor>(),
            7=>{},
            _=>return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"ordered map clone quote state is invalid")),
        }
        Ok(demand)
    }
    fn advance(&mut self, source: RetainedCloneRef<'_, RetainedOrderedMap<K, V>>, body: usize, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if self.closing {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map clone cursor is closing"));
        }
        if self.spent {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map clone cursor is spent"));
        }
        let demand=self.advance_demands(source,body)?;
        if !super::retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
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
                if self.page_output.is_some() && self.entry == self.source_page_len {
                    let page=self.page_output.take().ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"ordered map ready page omitted owner"))?;
                    self.pages.push(page);self.page+=1;self.entry=0;
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));
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
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..Default::default()}));
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
                if self.key_cursor.output_ready() {
                    self.key=Some(self.key_cursor.take().ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"ordered map ready key omitted owner"))?);
                    self.key_cursor.begin_close();self.phase=3;
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));
                }
                let ordinal=self.page*RETAINED_ORDERED_MAP_PAGE_CAPACITY+self.entry;
                let nested=super::nested_retained_clone_grant(grant)?;
                let step=self.key_cursor.advance(source.project(ordinal*2+1,|map|&map.pages[self.page][self.entry].0),body,nested)?;
                Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(nested,step.progress(),"ordered map key")?))
            }
            3 => {
                if !self.key_cursor.terminal_is_empty() {
                    let step = self.key_cursor.close_step(super::nested_retained_clone_grant(grant)?)?;
                    return Self::progress_from_close(step, self.key_cursor.terminal_is_empty(), grant, "retained ordered-map key scaffold close");
                }
                if grant.maximum_items == 0 {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                self.key_cursor = K::retained_clone_cursor();
                self.phase = 4;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes:demand.copy_bytes, ..Default::default() }))
            }
            4 => {
                if self.value_cursor.output_ready() {
                    self.value=Some(self.value_cursor.take().ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"ordered map ready value omitted owner"))?);
                    self.value_cursor.begin_close();self.phase=5;
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));
                }
                let ordinal=self.page*RETAINED_ORDERED_MAP_PAGE_CAPACITY+self.entry;
                let nested=super::nested_retained_clone_grant(grant)?;
                let step=self.value_cursor.advance(source.project(ordinal*2+2,|map|&map.pages[self.page][self.entry].1),body,nested)?;
                Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(nested,step.progress(),"ordered map value")?))
            }
            5 => {
                if !self.value_cursor.terminal_is_empty() {
                    let step = self.value_cursor.close_step(super::nested_retained_clone_grant(grant)?)?;
                    return Self::progress_from_close(step, self.value_cursor.terminal_is_empty(), grant, "retained ordered-map value scaffold close");
                }
                if grant.maximum_items == 0 {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                self.value_cursor = V::retained_clone_cursor();
                self.phase = 6;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes:demand.copy_bytes, ..Default::default() }))
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
                if !super::retained_clone_grant_admits(grant,super::retained_clone_close_demands::<RetainedOrderedMap<K,V>>(self,grant.maximum_copy_bytes)?) {return Ok(RetainedCloneStep::Progress(Default::default()));}
if !self.key_cursor.terminal_is_empty() {
            if self.key_cursor.begin_close() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
            let step = self.key_cursor.close_step(super::nested_retained_clone_grant(grant)?)?;
            return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, self.key_cursor.terminal_is_empty(), "retained ordered-map key close")?.progress()));
        }
        if !self.value_cursor.terminal_is_empty() {
            if self.value_cursor.begin_close() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
            let step = self.value_cursor.close_step(super::nested_retained_clone_grant(grant)?)?;
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

    fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{Ok(self.close_demands(0)?.depth)}
    fn next_close_copy_byte_demand(&self)->Result<usize,crate::ValueError>{Ok(self.close_demands(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,crate::ValueError>{Ok(self.close_demands(body)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,crate::ValueError>{Ok(self.close_demands(0)?.release_bytes)}


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
