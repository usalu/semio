//! 🧮️ The fixed-slot primitive of the reactor state and its slot counts.


#[doc(hidden)]
pub const REACTOR_TASK_SLOTS: usize = 1_024;
const REACTOR_FIXED_WORDS: usize = REACTOR_TASK_SLOTS / u64::BITS as usize;

#[doc(hidden)]
pub struct ReactorFixedSlots<T> {
    pub values: Box<[std::mem::MaybeUninit<T>]>,
    pub occupied: [u64; REACTOR_FIXED_WORDS],
    pub allocation_admitted: bool,
}

impl<T> ReactorFixedSlots<T> {
    pub fn new() -> Self {
        let mut values = Vec::new();
        let allocation_admitted = values.try_reserve_exact(REACTOR_TASK_SLOTS).is_ok();
        if allocation_admitted {
            values.resize_with(REACTOR_TASK_SLOTS, std::mem::MaybeUninit::uninit);
        }
        Self { values: values.into_boxed_slice(), occupied: [0; REACTOR_FIXED_WORDS], allocation_admitted }
    }

    /// 🈳️ A table with no backing: allocator success at construction is not an admission, so the owner takes the backing with [`Self::admit_backing`] on its first admission that needs a slot.
    pub fn unallocated() -> Self {
        Self { values: Vec::new().into_boxed_slice(), occupied: [0; REACTOR_FIXED_WORDS], allocation_admitted: false }
    }

    /// 🛂️ Takes the one fixed allocation, once; a refused reservation leaves the table exactly as empty as it was.
    pub fn admit_backing(&mut self) -> bool {
        if self.allocation_admitted {
            return true;
        }
        let mut values = Vec::new();
        if values.try_reserve_exact(REACTOR_TASK_SLOTS).is_err() {
            return false;
        }
        values.resize_with(REACTOR_TASK_SLOTS, std::mem::MaybeUninit::uninit);
        self.values = values.into_boxed_slice();
        self.allocation_admitted = true;
        true
    }

    pub fn occupied(&self, index: usize) -> bool {
        self.occupied[index / u64::BITS as usize] & (1u64 << (index % u64::BITS as usize)) != 0
    }

    pub fn set_occupied(&mut self, index: usize, occupied: bool) {
        let word = &mut self.occupied[index / u64::BITS as usize];
        let mask = 1u64 << (index % u64::BITS as usize);
        if occupied {
            *word |= mask;
        } else {
            *word &= !mask;
        }
    }

    /// 🔍️ SAFETY: occupancy is set only after `write` and cleared before `assume_init_read`.
    pub fn get(&self, index: usize) -> Option<&T> {
        if !self.occupied(index) {
            return None;
        }
        self.values.get(index).map(|value| unsafe { value.assume_init_ref() })
    }

    /// 🖊️ SAFETY: occupancy is set only after `write` and cleared before `assume_init_read`.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if !self.occupied(index) {
            return None;
        }
        self.values.get_mut(index).map(|value| unsafe { value.assume_init_mut() })
    }

    pub fn insert(&mut self, index: usize, value: T) -> Result<(), T> {
        if !self.allocation_admitted || self.occupied(index) {
            return Err(value);
        }
        self.values[index].write(value);
        self.set_occupied(index, true);
        Ok(())
    }

    pub fn insert_admitted(&mut self, index: usize, value: T) {
        debug_assert!(self.allocation_admitted && !self.occupied(index));
        self.values[index].write(value);
        self.set_occupied(index, true);
    }

    /// 🫴️ SAFETY: occupancy was checked and is now cleared before the exact initialized read.
    pub fn take(&mut self, index: usize) -> Option<T> {
        if !self.occupied(index) {
            return None;
        }
        self.set_occupied(index, false);
        Some(unsafe { self.values[index].assume_init_read() })
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        (0..REACTOR_TASK_SLOTS).filter_map(|index| self.get(index))
    }
}

impl<T> Drop for ReactorFixedSlots<T> {
    fn drop(&mut self) {}
}

#[doc(hidden)]
pub const PLUGIN_REACTOR_INSTANCE_SLOTS: usize = 1_024;
