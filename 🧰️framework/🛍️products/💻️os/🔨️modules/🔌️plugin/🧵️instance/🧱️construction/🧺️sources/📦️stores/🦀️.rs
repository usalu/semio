//! 📦️ Original store fields move directly from private source storage into the final mounted frame.

use super::*;
use std::{mem::MaybeUninit, ptr::{addr_of_mut, copy_nonoverlapping}};

macro_rules! store_fields {
    ($apply:ident) => {
        $apply! {
            0 store = lanes.document;
            1 config_store = lanes.config;
            2 draft_store = lanes.draft;
            3 interaction_store = lanes.interaction;
            4 presence_store = live.presence;
            5 transient_store = live.transient;
            6 window_config_store = live.window_config;
            7 window_transient_store = live.window_transient;
            8 retired_window_transient_stores = live.retired_window_transient;
            9 artifact_one_item_factory = live.artifact_factory;
            10 config_one_item_factory = live.config_factory;
            11 draft_one_item_factory = live.draft_factory;
            12 presence_one_item_factory = live.presence_factory;
            13 transient_one_item_factory = live.transient_factory;
            14 presence_local_root_retirement_factory = live.presence_retirement;
            15 transient_local_root_retirement_factory = live.transient_retirement;
            16 unsupported_publication_contracts = live.unsupported;
        }
    };
}

pub(crate) struct OriginalInstanceStoreFields<A: ArtifactApp> {
    lanes: MaybeUninit<OriginalArtifactStores<A>>,
    live: MaybeUninit<OriginalInstanceLiveStores<A>>,
}

impl<A: ArtifactApp> OriginalInstanceStoreFields<A> {
    pub(super) const FIELDS: usize = {
        macro_rules! count {
            ($($index:literal $target:ident = $source:ident.$donor:ident;)*) => { [$(stringify!($target)),*].len() };
        }
        store_fields!(count)
    };

    pub(crate) fn new(lanes: OriginalArtifactStores<A>, live: OriginalInstanceLiveStores<A>) -> Self {
        Self { lanes: MaybeUninit::new(lanes), live: MaybeUninit::new(live) }
    }

    pub(super) fn demand(field: usize, offset: usize, body: usize) -> Result<RetirementDemand, ValueError> {
        fn extent<S, T>(_: fn(&S) -> &T) -> usize { size_of::<T>() }
        macro_rules! quote {
            ($($index:literal $target:ident = $source:ident.$donor:ident;)*) => {
                $(if field == $index {
                    let size = extent::<InstanceStores<A>, _>(|stores| &stores.$target);
                    let copy_bytes = semio_framework_value::initialization::absent_chunk_bytes(size, offset, body.max(1))?;
                    return Ok(RetirementDemand { copy_bytes, depth: 1, ..Default::default() });
                })*
                let _: fn(&InstanceStores<A>) = |stores| {
                    let InstanceStores { $($target: _,)* } = stores;
                };
            };
        }
        store_fields!(quote);
        Err(invalid("original store field escaped its initialization inventory"))
    }

    /// 🔁️ Transfers a typed field into exclusive uninitialized final storage, one exact extent at a time.
    /// # Safety
    /// The original caller must preserve the monotone field and byte cursor and publish only after all fields transfer.
    pub(super) unsafe fn place(&mut self, target: *mut InstanceStores<A>, field: usize, offset: usize, count: usize) -> Result<bool, ValueError> {
        macro_rules! place {
            ($($index:literal $target:ident = $source:ident.$donor:ident;)*) => {
                $(if field == $index {
                    let source = self.$source.as_mut_ptr();
                    return unsafe { move_field(addr_of_mut!((*source).$donor), addr_of_mut!((*target).$target), offset, count) };
                })*
            };
        }
        store_fields!(place);
        Err(invalid("original store field escaped its initialization inventory"))
    }
}

unsafe fn move_field<T>(source: *mut T, target: *mut T, offset: usize, count: usize) -> Result<bool, ValueError> {
    let size = size_of::<T>();
    if semio_framework_value::initialization::absent_chunk_bytes(size, offset, count)? != count {
        return Err(invalid("original store transfer exceeds its field extent"));
    }
    unsafe {
        copy_nonoverlapping(source.cast::<MaybeUninit<u8>>().add(offset), target.cast::<MaybeUninit<u8>>().add(offset), count);
    }
    Ok(offset + count == size)
}
