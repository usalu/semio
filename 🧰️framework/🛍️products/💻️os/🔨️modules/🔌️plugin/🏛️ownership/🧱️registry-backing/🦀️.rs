//! 🧱️ Exact fixed registry slots partitioned into separately releasable allocations.
use semio_framework_value::{ValueError,ValueRefusalKind};
use std::mem::{MaybeUninit,size_of,size_of_val};

type Slot<T> = MaybeUninit<(u64,T)>;
type Page<T> = Box<[Slot<T>]>;

const PAGE_BYTES:usize=4096;

/// 🧱️ Slot pages are born on the first insert into their residues and released only while resident; a registry of large owners therefore owns its small page table until a slot is used.
pub(super) enum RegistryBacking<T>{
    Flat(Page<T>),
    Paged{page_slots:usize,pages:Vec<Option<Page<T>>>},
}

impl<T> Default for RegistryBacking<T>{
    fn default()->Self{Self::Flat(Box::default())}
}

impl<T> RegistryBacking<T>{
    fn page_slots(capacity:usize)->usize{
        let mut slots=capacity;
        while slots>1&&slots*size_of::<Slot<T>>()>PAGE_BYTES{slots/=2;}
        slots
    }

    fn reserve(count:usize)->Result<Page<T>,ValueError>{
        let mut slots=Vec::new();
        slots.try_reserve_exact(count).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original registry backing allocation refused"))?;
        unsafe{slots.set_len(count)}
        Ok(slots.into_boxed_slice())
    }

    pub(super) fn birth_bytes(capacity:usize)->Result<usize,ValueError>{
        let page_slots=Self::page_slots(capacity);
        let overflow=||ValueError::literal(ValueRefusalKind::OwnershipLimit,"fixed registry backing extent overflow");
        if page_slots==capacity{return capacity.checked_mul(size_of::<Slot<T>>()).ok_or_else(overflow)}
        (capacity/page_slots).checked_mul(size_of::<Option<Page<T>>>()).ok_or_else(overflow)
    }

    pub(super) fn allocate(capacity:usize)->Result<Self,ValueError>{
        let page_slots=Self::page_slots(capacity);
        if page_slots==capacity{return Ok(Self::Flat(Self::reserve(capacity)?))}
        let count=capacity/page_slots;
        let mut pages=Vec::new();
        pages.try_reserve_exact(count).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original registry backing allocation refused"))?;
        pages.resize_with(count,||None);
        Ok(Self::Paged{page_slots,pages})
    }

    pub(super) fn get(&self,index:usize)->Option<&Slot<T>>{
        match self{
            Self::Flat(slots)=>slots.get(index),
            Self::Paged{page_slots,pages}=>pages.get(index/ *page_slots)?.as_ref()?.get(index% *page_slots),
        }
    }

    pub(super) fn get_mut(&mut self,index:usize)->Option<&mut Slot<T>>{
        match self{
            Self::Flat(slots)=>slots.get_mut(index),
            Self::Paged{page_slots,pages}=>pages.get_mut(index/ *page_slots)?.as_mut()?.get_mut(index% *page_slots),
        }
    }

    /// 🎟️ Borrows one slot for its first write, birthing the page that holds it on demand.
    pub(super) fn get_or_birth_mut(&mut self,index:usize)->Option<&mut Slot<T>>{
        match self{
            Self::Flat(slots)=>slots.get_mut(index),
            Self::Paged{page_slots,pages}=>{
                let entry=pages.get_mut(index/ *page_slots)?;
                if entry.is_none(){
                    let page=Self::reserve(*page_slots).ok()?;
                    semio_framework_value::birth_ledger::record_original_birth(size_of_val(page.as_ref()));
                    *entry=Some(page);
                }
                entry.as_mut()?.get_mut(index% *page_slots)
            }
        }
    }

    /// 🔭️ Resident pages with the slot index of their first slot.
    pub(super) fn resident_pages_mut(&mut self)->Box<dyn Iterator<Item=(usize,&mut [Slot<T>])>+'_>{
        match self{
            Self::Flat(slots)=>Box::new(std::iter::once((0,&mut slots[..]))),
            Self::Paged{page_slots,pages}=>{let width= *page_slots;Box::new(pages.iter_mut().enumerate().filter_map(move|(number,page)|page.as_mut().map(|page|(number*width,&mut page[..]))))}
        }
    }

    pub(super) fn resident_pages(&self)->usize{
        match self{Self::Flat(slots)=>usize::from(!slots.is_empty()),Self::Paged{pages,..}=>pages.iter().flatten().count()}
    }

    pub(super) fn next_release(&self)->usize{
        match self{
            Self::Flat(slots)=>size_of_val(slots.as_ref()),
            Self::Paged{pages,..}=>pages.iter().rev().flatten().next().map_or(pages.capacity()*size_of::<Option<Page<T>>>(),|page|size_of_val(page.as_ref())),
        }
    }

    pub(super) fn release_one(&mut self){
        if let Self::Paged{pages,..}=self{
            if let Some(position)=pages.iter().rposition(Option::is_some){drop(pages[position].take());return}
        }
        *self=Self::default();
    }
}


#[cfg(test)]
mod tests{
    use super::super::ArtifactFixedRegistry;
    use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress};
    use semio_framework_trace::observe_heap_allocations_on_this_thread;

    #[test]
    fn paged_registry_preserves_slot_identity_and_disjoint_iteration(){
        let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
        let mut registry=ArtifactFixedRegistry::<[u8;8192]>::new();
        let entries=law["pageBoundaryEntries"].as_array().unwrap().iter().map(|id|id.as_u64().unwrap()).collect::<Vec<_>>();
        let selected=law["partitionSource"].as_u64().unwrap();
        for id in &entries{registry.insert_admitted(*id,[*id as u8;8192]);}
        assert!(!registry.can_insert(64));
        let mut others=Vec::new();
        let loan=registry.partition_mut(selected,|value|others.push(u64::from(value[0]))).unwrap();
        assert_eq!(u64::from(loan[0]),selected);
        loan[0]=255;
        assert_eq!(others,entries.iter().copied().filter(|id|*id!=selected).collect::<Vec<_>>());
        assert_eq!(registry.get(selected).unwrap()[0],255);
        for id in entries{assert!(registry.remove(id).is_some());}
        assert!(registry.is_empty());
    }

    #[test]
    fn large_slots_are_born_on_first_insert_and_only_resident_pages_are_released_one_per_original_grant(){
        const PAYLOAD:usize=128*1024;
        let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
        let number=|key:&str|usize::try_from(law[key].as_u64().unwrap()).unwrap();
        assert_eq!(number("payloadBytes"),PAYLOAD);
        let slots=number("slots");
        let birth=ArtifactFixedRegistry::<[u8;PAYLOAD]>::birth_demands(slots).unwrap().capacity_bytes;
        assert_eq!(birth,slots*std::mem::size_of::<Option<Box<[std::mem::MaybeUninit<(u64,[u8;PAYLOAD])>]>>>());
        let mut registry=ArtifactFixedRegistry::<[u8;PAYLOAD]>::unallocated(slots);
        let grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:birth,maximum_release_bytes:number("maximumReleaseBytes"),maximum_depth:1,..Default::default()};
        let (step,heap)=observe_heap_allocations_on_this_thread(||registry.admit_backing_granted(grant).unwrap());
        assert_eq!(step.progress().retained_capacity_bytes,birth);
        assert_eq!((heap.requested_bytes,heap.released_bytes),(birth,0));
        assert!((0..slots).all(|slot|registry.backing_slot_address(slot).is_none()));
        let used=law["usedSlots"].as_array().unwrap().iter().map(|id|id.as_u64().unwrap()).collect::<Vec<_>>();
        for id in &used{
            let (_,heap)=observe_heap_allocations_on_this_thread(||registry.insert_admitted(*id,[1;PAYLOAD]));
            assert!(heap.requested_bytes>=std::mem::size_of::<(u64,[u8;PAYLOAD])>(),"the first insert births exactly its page");
        }
        assert!(used.iter().all(|id|registry.backing_slot_address(*id as usize).is_some()));
        for id in &used{assert!(registry.remove(*id).is_some());}
        let mut observed=0;
        let mut turns=0;
        while let Some(bytes @ 1..)=registry.empty_backing_byte_demand(){
            let denied=RetainedCloneGrant{maximum_release_bytes:bytes-1,..grant};
            let (step,heap)=observe_heap_allocations_on_this_thread(||registry.close_empty_backing_granted(denied).unwrap());
            assert_eq!(step.progress(),RetainedCloneProgress::default());
            assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
            assert_eq!(registry.empty_backing_byte_demand(),Some(bytes));
            let (step,heap)=observe_heap_allocations_on_this_thread(||registry.close_empty_backing_granted(grant).unwrap());
            assert_eq!(step.progress().released_bytes,bytes);
            assert_eq!((heap.requested_bytes,heap.released_bytes),(0,bytes));
            assert!(!registry.allocation_admitted);
            observed+=bytes;
            turns+=1;
        }
        assert_eq!(turns,used.len()+1,"one release per resident page and one for the page table");
        assert_eq!(observed,birth+used.len()*std::mem::size_of::<(u64,[u8;PAYLOAD])>());
    }
}
