//! 🧱️ Exact fixed registry slots partitioned into separately releasable allocations.
use semio_framework_value::{ValueError,ValueRefusalKind};
use std::mem::{MaybeUninit,size_of,size_of_val};

type Slot<T> = MaybeUninit<(u64,T)>;
type Page<T> = Box<[Slot<T>]>;

const PAGE_BYTES:usize=256*1024;

pub(super) enum RegistryBacking<T>{
    Flat(Page<T>),
    Paged{page_slots:usize,pages:Vec<Page<T>>},
}

impl<T> Default for RegistryBacking<T>{
    fn default()->Self{Self::Flat(Box::default())}
}

impl<T> RegistryBacking<T>{
    fn page_slots(capacity:usize)->usize{
        let mut slots=capacity;
        while slots>1&&slots>PAGE_BYTES/size_of::<Slot<T>>(){slots/=2;}
        slots
    }

    pub(super) fn birth_bytes(capacity:usize)->Result<usize,ValueError>{
        let page_slots=Self::page_slots(capacity);
        let table=if page_slots==capacity{0}else{capacity/page_slots*size_of::<Page<T>>()};
        capacity.checked_mul(size_of::<Slot<T>>()).and_then(|bytes|bytes.checked_add(table))
            .ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"fixed registry backing extent overflow"))
    }

    pub(super) fn allocate(capacity:usize)->Result<Self,ValueError>{
        let refused=|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original registry backing allocation refused");
        let reserve=|count|->Result<Page<T>,ValueError>{
            let mut slots=Vec::new();
            slots.try_reserve_exact(count).map_err(refused)?;
            unsafe{slots.set_len(count)}
            Ok(slots.into_boxed_slice())
        };
        let page_slots=Self::page_slots(capacity);
        if page_slots==capacity{return Ok(Self::Flat(reserve(capacity)?))}
        let count=capacity/page_slots;
        let mut pages=Vec::new();
        pages.try_reserve_exact(count).map_err(refused)?;
        for _ in 0..count{pages.push(reserve(page_slots)?);}
        Ok(Self::Paged{page_slots,pages})
    }

    pub(super) fn get(&self,index:usize)->Option<&Slot<T>>{
        match self{
            Self::Flat(slots)=>slots.get(index),
            Self::Paged{page_slots,pages}=>pages.get(index/ *page_slots)?.get(index% *page_slots),
        }
    }

    pub(super) fn get_mut(&mut self,index:usize)->Option<&mut Slot<T>>{
        match self{
            Self::Flat(slots)=>slots.get_mut(index),
            Self::Paged{page_slots,pages}=>pages.get_mut(index/ *page_slots)?.get_mut(index% *page_slots),
        }
    }

    pub(super) fn slices_mut(&mut self)->&mut[Page<T>]{
        match self{
            Self::Flat(slots)=>std::slice::from_mut(slots),
            Self::Paged{pages,..}=>pages.as_mut_slice(),
        }
    }

    pub(super) fn next_release(&self)->usize{
        match self{
            Self::Flat(slots)=>size_of_val(slots.as_ref()),
            Self::Paged{pages,..}=>pages.last().map_or(pages.capacity()*size_of::<Page<T>>(),|page|size_of_val(page.as_ref())),
        }
    }

    pub(super) fn release_one(&mut self){
        if let Self::Paged{pages,..}=self{
            if let Some(page)=pages.pop(){drop(page);return}
        }
        *self=Self::default();
    }
}


#[cfg(test)]
mod tests{
    use super::super::ArtifactFixedRegistry;
    use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress};
    use semio_framework_trace::{HeapWitness,observe_heap_allocations_on_this_thread};

    #[global_allocator]
    static HEAP:HeapWitness=HeapWitness;

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
    fn large_empty_slots_are_physically_released_one_page_per_original_grant(){
        const PAYLOAD:usize=128*1024;
        let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
        let number=|key:&str|usize::try_from(law[key].as_u64().unwrap()).unwrap();
        assert_eq!(number("payloadBytes"),PAYLOAD);
        let slots=number("slots");
        let birth=ArtifactFixedRegistry::<[u8;PAYLOAD]>::birth_demands(slots).unwrap().capacity_bytes;
        let mut registry=ArtifactFixedRegistry::<[u8;PAYLOAD]>::unallocated(slots);
        let grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:birth,maximum_release_bytes:number("maximumReleaseBytes"),maximum_depth:1,..Default::default()};
        let (step,heap)=observe_heap_allocations_on_this_thread(||registry.admit_backing_granted(grant).unwrap());
        assert_eq!(step.progress().retained_capacity_bytes,birth);
        assert_eq!((heap.requested_bytes,heap.released_bytes),(birth,0));
        assert!((0..slots).all(|slot|registry.backing_slot_address(slot).is_some()));
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
        assert_eq!((observed,turns),(birth,number("pageReleases")+number("descriptorReleases")));
    }
}
