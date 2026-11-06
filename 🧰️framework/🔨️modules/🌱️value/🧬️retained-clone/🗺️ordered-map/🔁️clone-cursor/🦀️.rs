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
    fn progress_from_close(step: RetainedCloneStep, terminal_is_empty: bool, grant: RetainedCloneGrant, context: &'static str) -> Result<RetainedCloneStep, ValueError> {
        let progress = admit_retained_clone_close(grant, step, terminal_is_empty, context)?.progress();
        Ok(RetainedCloneStep::Progress(progress))
    }
}

impl<K:RetainedClone,V:RetainedClone> RetainedOrderedMapCloneCursor<K,V>{
    fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{
        #[cfg(test)] selected_close_tests::record(self as*const Self as usize,body);
        if !self.closing{return Ok(Default::default());}
        if !self.key_cursor.terminal_is_empty(){return nested_retained_clone_close_demands::<K>(&self.key_cursor,body);}
        if !self.value_cursor.terminal_is_empty(){return nested_retained_clone_close_demands::<V>(&self.value_cursor,body);}
        if self.key.is_some(){return self.close.next_owner_demands::<K>(true,&self.source,body);}
        if self.value.is_some(){return self.close.next_owner_demands::<V>(true,&self.source,body);}
        if self.page_output.is_some(){return self.close.next_owner_demands::<Vec<(K,V)>>(true,&self.source,body);}
        if !self.pages.is_empty()||self.pages.capacity()!=0{return self.close.next_owner_demands::<Vec<Vec<(K,V)>>>(true,&self.source,body);}
        self.close.next_owner_demands::<RetainedOrderedMap<K,V>>(self.output.is_some(),&self.source,body)
    }
}
impl<K: RetainedClone, V: RetainedClone> RetainedCloneCursor<RetainedOrderedMap<K, V>> for RetainedOrderedMapCloneCursor<K, V> {
    fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{RetainedOrderedMapCloneCursor::<K,V>::close_demands(self,body)}
    fn output_ready(&self)->bool{self.output.is_some()}
    fn advance_demands(&self,source:RetainedCloneRef<'_,RetainedOrderedMap<K,V>>,body:usize)->Result<crate::RetirementDemand,ValueError>{
        if self.closing||self.spent{return Err(ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"ordered map cannot quote a closing or spent clone"));}
        if self.output.is_some(){return Ok(Default::default());}
        if !source.check_binding(&self.source)?{return Ok(crate::RetirementDemand{copy_bytes:source.binding_copy_bytes(),depth:1,..Default::default()});}
        let value=source.get();let mut demand=crate::RetirementDemand{depth:1,..Default::default()};
        match self.phase{
            0=>demand.capacity_bytes=value.pages.len().checked_add(1).and_then(|count|count.checked_mul(size_of::<Vec<(K,V)>>())).ok_or_else(||ValueError::literal(crate::ValueRefusalKind::OwnershipLimit,"ordered map directory quote overflow"))?,
            1=>{if self.page_output.is_some()&&self.entry==self.source_page_len{demand.copy_bytes=size_of::<Vec<(K,V)>>();}else if self.page==value.pages.len(){demand.copy_bytes=size_of::<RetainedOrderedMap<K,V>>();}else if self.page_output.is_none(){demand.capacity_bytes=RETAINED_ORDERED_MAP_PAGE_CAPACITY.checked_mul(size_of::<(K,V)>()).ok_or_else(||ValueError::literal(crate::ValueRefusalKind::OwnershipLimit,"ordered map page quote overflow"))?;}},
            2=>{if self.key_cursor.output_ready(){return nested_retained_clone_take_demands(&self.key_cursor);}else{let ordinal=self.page*RETAINED_ORDERED_MAP_PAGE_CAPACITY+self.entry;return nested_retained_clone_demands::<K>(&self.key_cursor,source.project(ordinal*2+1,|map|&map.pages[self.page][self.entry].0),body);}},
            3 if !self.key_cursor.terminal_is_empty()=>return nested_retained_clone_close_demands::<K>(&self.key_cursor,body),
            4=>{if self.value_cursor.output_ready(){return nested_retained_clone_take_demands(&self.value_cursor);}else{let ordinal=self.page*RETAINED_ORDERED_MAP_PAGE_CAPACITY+self.entry;return nested_retained_clone_demands::<V>(&self.value_cursor,source.project(ordinal*2+2,|map|&map.pages[self.page][self.entry].1),body);}},
            5 if !self.value_cursor.terminal_is_empty()=>return nested_retained_clone_close_demands::<V>(&self.value_cursor,body),
            6=>demand.copy_bytes=size_of::<(K,V)>(),
            3=>demand.copy_bytes=size_of::<K::Cursor>(),
            5=>demand.copy_bytes=size_of::<V::Cursor>(),
            7=>{},
            _=>return Err(ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"ordered map clone quote state is invalid")),
        }
        Ok(demand)
    }
    fn advance(&mut self, source: RetainedCloneRef<'_, RetainedOrderedMap<K, V>>, body: usize, grant: RetainedCloneGrant) -> crate::OriginalRetirementOutcome {let mut work=0;let result=(|| -> Result<RetainedCloneStep,crate::OriginalRetirementFailure>{
        if self.closing {
            return Err((ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map clone cursor is closing")).into());
        }
        if self.spent {
            return Err((ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map clone cursor is spent")).into());
        }
        let demand=self.advance_demands(source,body)?;
        if !retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
        if self.output.is_some() {
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
        }
        if let Some(progress)={let turn=source.bind(&mut self.source,grant);work+=turn.work;turn.result}?{return Ok(RetainedCloneStep::Progress(progress));}
        let source_value = source.get();
        match self.phase {
            0 => {
                let planned_pages = source_value.pages.len().checked_add(1).ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map directory page count overflow"))?;
                let planned_capacity = planned_pages.checked_mul(size_of::<Vec<(K, V)>>()).ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map directory capacity overflow"))?;
                let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: planned_capacity, released_bytes: 0 };
                if !progress.fits(grant) {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                work+=1;self.pages.try_reserve_exact(planned_pages).map_err(|_| ValueError::literal(crate::ValueRefusalKind::AllocationFailed, "retained ordered-map directory allocation failed"))?;
                let actual = self.pages.capacity().checked_mul(size_of::<Vec<(K, V)>>()).ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map directory actual capacity overflow"))?;
                if actual > grant.maximum_capacity_bytes {
                    return Err((ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map directory allocator exceeded its admitted capacity").with_retained_progress(RetainedCloneProgress{retained_capacity_bytes:actual,..progress})).into());
                }
                self.phase = 1;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { retained_capacity_bytes: actual, ..progress }))
            }
            1 => {
                if self.page_output.is_some() && self.entry == self.source_page_len {
                    work+=1;let page=self.page_output.take().ok_or_else(||ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"ordered map ready page omitted owner"))?;
                    self.pages.push(page);self.page+=1;self.entry=0;
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));
                }
                if self.page == source_value.pages.len() {
                    if grant.maximum_copy_bytes < size_of::<RetainedOrderedMap<K,V>>() {
                        return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                    }
                    work+=1;self.output = Some(RetainedOrderedMap { pages: std::mem::take(&mut self.pages), len: source_value.len });
                    self.phase = 7;
                    return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes:size_of::<RetainedOrderedMap<K,V>>(), ..Default::default() }));
                }
                if self.page_output.is_some() {
                    self.phase = 2;work+=1;
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..Default::default()}));
                }
                let source_page = source_value.pages.get(self.page).ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map source page is missing"))?;
                if source_page.is_empty() || source_page.len() > RETAINED_ORDERED_MAP_PAGE_CAPACITY {
                    return Err((ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map source page violates its fixed capacity")).into());
                }
                let planned = RETAINED_ORDERED_MAP_PAGE_CAPACITY.checked_mul(size_of::<(K, V)>()).ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map page capacity overflow"))?;
                let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: planned, released_bytes: 0 };
                if !progress.fits(grant) {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                let mut page = Vec::new();
                work+=1;page.try_reserve_exact(RETAINED_ORDERED_MAP_PAGE_CAPACITY).map_err(|_| ValueError::literal(crate::ValueRefusalKind::AllocationFailed, "retained ordered-map page allocation failed"))?;
                let actual = page.capacity().checked_mul(size_of::<(K, V)>()).ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map page actual capacity overflow"))?;
                if actual > grant.maximum_capacity_bytes {
                    self.page_output=Some(page);return Err((ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"retained ordered-map page allocator exceeded its admitted capacity").with_retained_progress(RetainedCloneProgress{retained_capacity_bytes:actual,..progress})).into());
                }
                self.source_page_len = source_page.len();
                self.page_output = Some(page);
                self.phase = 2;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { retained_capacity_bytes: actual, ..progress }))
            }
            2 => {
                if self.key_cursor.output_ready() {
                    let nested=nested_retained_clone_grant(grant)?;let turn=self.key_cursor.take(nested);work+=turn.work;let Some((value,progress))=turn.result? else{return Ok(RetainedCloneStep::Progress(Default::default()))};self.key=Some(value);
                    self.key_cursor.begin_close();self.phase=3;
                    return Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(nested,progress,"ordered map key take")?));
                }
                let ordinal=self.page*RETAINED_ORDERED_MAP_PAGE_CAPACITY+self.entry;
                let nested=nested_retained_clone_grant(grant)?;
                let step={let turn=self.key_cursor.advance(source.project(ordinal*2+1,|map|&map.pages[self.page][self.entry].0),body,nested);work+=turn.work;turn.result}?;
                Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(nested,step.progress(),"ordered map key")?))
            }
            3 => {
                if !self.key_cursor.terminal_is_empty() {
                    let step = {let turn=self.key_cursor.close_step(nested_retained_clone_grant(grant)?);work+=turn.work;turn.result}?;
                    return Self::progress_from_close(step, self.key_cursor.terminal_is_empty(), grant, "retained ordered-map key scaffold close");
                }
                if grant.maximum_items == 0 {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                work+=1;self.key_cursor = K::retained_clone_cursor();
                self.phase = 4;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes:demand.copy_bytes, ..Default::default() }))
            }
            4 => {
                if self.value_cursor.output_ready() {
                    let nested=nested_retained_clone_grant(grant)?;let turn=self.value_cursor.take(nested);work+=turn.work;let Some((value,progress))=turn.result? else{return Ok(RetainedCloneStep::Progress(Default::default()))};self.value=Some(value);
                    self.value_cursor.begin_close();self.phase=5;
                    return Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(nested,progress,"ordered map value take")?));
                }
                let ordinal=self.page*RETAINED_ORDERED_MAP_PAGE_CAPACITY+self.entry;
                let nested=nested_retained_clone_grant(grant)?;
                let step={let turn=self.value_cursor.advance(source.project(ordinal*2+2,|map|&map.pages[self.page][self.entry].1),body,nested);work+=turn.work;turn.result}?;
                Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(nested,step.progress(),"ordered map value")?))
            }
            5 => {
                if !self.value_cursor.terminal_is_empty() {
                    let step = {let turn=self.value_cursor.close_step(nested_retained_clone_grant(grant)?);work+=turn.work;turn.result}?;
                    return Self::progress_from_close(step, self.value_cursor.terminal_is_empty(), grant, "retained ordered-map value scaffold close");
                }
                if grant.maximum_items == 0 {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                work+=1;self.value_cursor = V::retained_clone_cursor();
                self.phase = 6;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes:demand.copy_bytes, ..Default::default() }))
            }
            6 => {
                if grant.maximum_copy_bytes < size_of::<(K,V)>() {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                work+=1;self.page_output.as_mut().ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map output page is missing"))?.push((
                    self.key.take().ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map key owner is missing"))?,
                    self.value.take().ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map value owner is missing"))?,
                ));
                self.entry += 1;
                self.phase = 1;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes:size_of::<(K,V)>(), ..Default::default() }))
            }
            7 => Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())),
            _ => Err((ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map clone state is invalid")).into()),
        }
    })();crate::RetainedWorkOutcome{result,work}}

    fn take_demand(&self)->Result<crate::RetirementDemand,ValueError>{Ok(crate::retained_clone::retained_clone_take_demand())}
    fn take(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(RetainedOrderedMap<K, V>,RetainedCloneProgress)>>{if !self.output_ready(){return crate::RetainedWorkOutcome{result:Ok(None),work:0}}let turn=crate::retained_clone::take_retained_clone_output(&mut self.output,grant);if matches!(&turn.result,Ok(Some(_))){self.spent = true;}turn}

    fn begin_close(&mut self) -> bool {
        if self.closing {
            return false;
        }
        self.closing = true;
        true
    }

    

    fn close_step(&mut self, grant: RetainedCloneGrant) ->crate::RetainedWorkOutcome{let mut work=0;let result=(||{
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "ordered map must begin close before granted retirement")); }
                if !retained_clone_grant_admits(grant,retained_clone_close_demands::<RetainedOrderedMap<K,V>>(self,grant.maximum_copy_bytes)?) {return Ok(RetainedCloneStep::Progress(Default::default()));}
if !self.key_cursor.terminal_is_empty() {
            if self.key_cursor.begin_close() {work+=1; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
            let step = {let turn=self.key_cursor.close_step(nested_retained_clone_grant(grant)?);work+=turn.work;turn.result}?;
            return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, self.key_cursor.terminal_is_empty(), "retained ordered-map key close")?.progress()));
        }
        if !self.value_cursor.terminal_is_empty() {
            if self.value_cursor.begin_close() {work+=1; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
            let step = {let turn=self.value_cursor.close_step(nested_retained_clone_grant(grant)?);work+=turn.work;turn.result}?;
            return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, self.value_cursor.terminal_is_empty(), "retained ordered-map value close")?.progress()));
        }
        if !self.close.is_empty() { return {let turn=self.close.step_granted(grant);work+=turn.work;turn.result}; }
        if let Some(step) = {let turn=self.close.begin_granted(&mut self.key, grant);work+=turn.work;turn.result}? { return Ok(step); }
        if let Some(step) = {let turn=self.close.begin_granted(&mut self.value, grant);work+=turn.work;turn.result}? { return Ok(step); }
        if let Some(step) = {let turn=self.close.begin_granted(&mut self.page_output, grant);work+=turn.work;turn.result}? { return Ok(step); }
        if !self.pages.is_empty() || self.pages.capacity() != 0 {
            if let Some(step) = {let turn=self.close.begin_empty_granted(&mut self.pages, grant);work+=turn.work;turn.result}? { return Ok(step); }
        }
        if let Some(step) = {let turn=self.close.begin_granted(&mut self.output, grant);work+=turn.work;turn.result}? { return Ok(step); }
        {let turn=close_retained_binding(&mut self.source, grant);work+=turn.work;turn.result}
    })();crate::RetainedWorkOutcome{result,work}}

    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.depth)}
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demands(body)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.release_bytes)}


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

#[cfg(test)]
#[path="🧪️tests/🎯️dispatch/🦀️.rs"]
mod selected_close_tests;
