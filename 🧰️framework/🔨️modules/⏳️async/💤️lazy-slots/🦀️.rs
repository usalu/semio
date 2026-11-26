//! 💤️ Lazily allocated fixed slot tables that keep `.data` empty.

/// 💤️ A fixed table of `N` optional slots that costs no image bytes until first touched.
///
/// A `static`/`thread_local!` `[Option<T>; N]` is const-initialised with the `None` niche, which is
/// not all-zero for most `T`, so the linker emits the WHOLE table as an active data segment: measured
/// 72.5 MB (94 % zero bytes) in every plugin component for the 1 024-slot request, response, staged
/// event and job tables. The empty `OnceCell<Box<..>>` is all-zero (`.bss`, no bytes in the wasm); the
/// heap table is built on the first access straight onto the heap (`Vec::with_capacity` then `resize_with`), never through a stack temporary.
pub struct LazySlots<T, const N: usize> {
    cell: std::cell::OnceCell<Box<[Option<T>]>>,
}

impl<T, const N: usize> LazySlots<T, N> {
    pub const fn new() -> Self {
        Self { cell: std::cell::OnceCell::new() }
    }

    /// 🔎️ Reads already materialised slots without allocating an unused table.
    pub fn resident_slots(&self) -> &[Option<T>] {
        self.cell.get().map_or(&[], |slots| slots.as_ref())
    }

    /// 🔄️ Takes the first occupied slot in round-robin order, starting at the supplied cursor.
    pub fn take_next_occupied_slot(&mut self, start: usize) -> Option<(usize, T)> {
        let slots = self.cell.get_mut()?;
        if slots.is_empty() {
            return None;
        }
        let start = start % slots.len();
        let index = slots[start..].iter().position(Option::is_some)
            .map(|offset| start + offset)
            .or_else(|| slots[..start].iter().position(Option::is_some))?;
        slots[index].take().map(|owner| (index, owner))
    }

    fn fill() -> Box<[Option<T>]> {
        let mut slots: Vec<Option<T>> = Vec::with_capacity(N);
        slots.resize_with(N, || None);
        slots.into_boxed_slice()
    }
}

impl<T, const N: usize> Default for LazySlots<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> std::ops::Deref for LazySlots<T, N> {
    type Target = [Option<T>];
    fn deref(&self) -> &[Option<T>] {
        self.cell.get_or_init(Self::fill)
    }
}

impl<T, const N: usize> std::ops::DerefMut for LazySlots<T, N> {
    fn deref_mut(&mut self) -> &mut [Option<T>] {
        self.cell.get_or_init(Self::fill);
        self.cell.get_mut().expect("💤️ lazy slot table was just materialised")
    }
}

#[cfg(test)]
mod tests {
    use super::LazySlots;

    #[test]
    fn polling_vacancies_preserves_lazy_allocation_and_rotates_across_live_owners() {
        let mut slots = LazySlots::<Box<[u8; 4096]>, 1024>::new();
        assert!(slots.take_next_occupied_slot(1023).is_none());
        assert!(slots.cell.get().is_none());
        slots[1000] = Some(Box::new([1; 4096]));
        slots[2] = Some(Box::new([2; 4096]));
        let (index, owner) = slots.take_next_occupied_slot(999).unwrap();
        assert_eq!((index, owner[0]), (1000, 1));
        let (index, owner) = slots.take_next_occupied_slot(1001).unwrap();
        assert_eq!((index, owner[0]), (2, 2));
        assert!(slots.take_next_occupied_slot(0).is_none());
        assert!(LazySlots::<u8, 0>::new().take_next_occupied_slot(1).is_none());
    }

    #[test]
    fn empty_table_is_all_zero_and_materialises_n_vacant_slots_on_first_access() {
        const { assert!(size_of::<LazySlots<[u8; 4096], 1024>>() == size_of::<usize>() * 2) };
        let mut slots = LazySlots::<[u8; 4096], 1024>::new();
        assert_eq!(slots.len(), 1024);
        assert!(slots.iter().all(Option::is_none));
        slots[3] = Some([7; 4096]);
        assert_eq!(slots.iter().flatten().count(), 1);
    }
}
