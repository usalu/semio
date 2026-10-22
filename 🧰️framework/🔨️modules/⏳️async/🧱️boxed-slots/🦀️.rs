//! 🧱️ Boxed fixed-slot constructors, `FixedSlotTableBudget` and the bounded-stack runner.

//#region 🗺️ShardedMap
#[path = "../🗺️sharded-map/🦀️.rs"]
mod sharded_map;
pub use sharded_map::ShardedMap;
//#endregion 🗺️ShardedMap

//#region 🎰️FixedSlotPages
#[path = "../🎰️slots/📄️pages/🦀️.rs"]
mod original_fixed_slot_pages;
pub use original_fixed_slot_pages::OriginalFixedSlotPages;
//#endregion 🎰️FixedSlotPages

//#region 🧱️BoxedSlots
/// 🧱️ Fills `len` slots straight onto the heap, never through a stack temporary.
///
/// `Box::new([const { None }; N])` and `Box::new(core::array::from_fn(fill))` are heap
/// *destinations* with a **stack source**: the array expression is a value, materialised in the
/// caller's frame and only then moved into the allocation. At `opt-level = 0` — `dev`, `test`,
/// where LLVM neither colours nor reuses stack slots — that costs
/// `len * size_of::<T>()` of stack in the constructing frame, however large. Measured instances of
/// exactly that defect: 5 670 928 B for `[Option<AdmittedSurfaceEntry<World3dState>>; 256]`,
/// 16 779 328 B for `[EngineSurfaceSlot; 256]`, 11 929 152 B for `[Option<UiSurfaceSlot>; 64]` — on
/// OS threads that own 2 MiB (`WorkerPool`'s workers take Rust's default) and on a wasm shadow
/// stack of 16 MiB.
///
/// `Vec::with_capacity` takes the allocation first and `resize_with` writes each slot directly into
/// it, so the deepest live temporary is **one** `T`. The produced value is identical to the array
/// literal's, byte for byte.
///
/// See also [`boxed_fixed_slots`] for the `Box<[T; N]>` (statically-sized) shape.
pub fn boxed_slots<T>(len: usize, fill: impl FnMut() -> T) -> Box<[T]> {
    let mut slots: Vec<T> = Vec::with_capacity(len);
    slots.resize_with(len, fill);
    slots.into_boxed_slice()
}

/// 🧱️ [`boxed_slots`] for a statically-sized slot table: `Box<[T; N]>`, so callers keep the exact
/// array type (and its indexing/credit invariants) the stack-materialising literal gave them.
///
/// `Box<[T]> → Box<[T; N]>` is `TryFrom`, a pointer cast with no copy, and `N` is the same constant
/// the allocation was sized with, so the conversion cannot fail.
pub fn boxed_fixed_slots<T, const N: usize>(fill: impl FnMut() -> T) -> Box<[T; N]> {
    let slots = boxed_slots(N, fill);
    slots.try_into().ok().expect("🧱️ fixed slot table seals into its own array — allocated with exactly N slots")
}

/// 🧵️ Runs `build` on a thread that owns exactly `stack_bytes`, propagating its panic.
///
/// `Builder::stack_size` **overrides** `RUST_MIN_STACK`, which is the whole point: the repo runner
/// floors that variable at 128 MiB, so a constructor that re-materialises its slot table in the
/// caller's frame stays green in every gate until a lane like this one pins the real budget. The
/// budget to pin is the one `WorkerPool`'s workers actually get — `WORKER_STACK_BYTES`, which
/// `native_pool` now states explicitly instead of inheriting Rust's 2 MiB default.
/// 🧱️ One row of the [fixed-slot fixture](🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json), measured or declared — see
/// [`assert_fixed_slot_tables`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FixedSlotTableBudget {
    pub owner: String,
    pub capacity: usize,
    pub element_bytes: usize,
    pub owner_bytes: usize,
}

impl FixedSlotTableBudget {
    pub fn new(owner: &str, capacity: usize, element_bytes: usize, owner_bytes: usize) -> Self {
        Self { owner: owner.to_string(), capacity, element_bytes, owner_bytes }
    }
}

/// 🧱️ The whole `boxed_fixed_slots` law, in one place: the crate's measured rows equal the
/// committed fixture's rows, every listed table is big enough to be worth listing
/// (`capacity * element_bytes` over the conversion threshold), every owner is **smaller than the
/// table it owns** — which is exactly what an inline `[T; N]` field cannot be, so it is the
/// structural proof that the slots live on the heap — and `build` (the owners' `Default`/`new`) runs
/// to completion on a thread holding only `stack_bytes`.
///
/// Callers supply `declared` from the [fixed-slot fixture](🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json) and `measured` from `size_of` at the
/// one site where the private slot types are nameable; each caller owns its physical descriptor.
/// This function owns the reusable assertions without enumerating callers.
#[cfg(not(target_arch = "wasm32"))]
pub fn assert_fixed_slot_tables(guard: &str, stack_bytes: usize, threshold_bytes: usize, declared: &[FixedSlotTableBudget], measured: &[FixedSlotTableBudget], build: impl FnOnce() + Send + 'static) {
    assert!(!declared.is_empty(), "🧱️ guard '{guard}' owns no row in the committed fixed-slot-table budget");
    assert_eq!(measured, declared, "🧱️ guard '{guard}': measured slot tables differ from the committed budget");
    for row in measured {
        let table_bytes = row.capacity.checked_mul(row.element_bytes).expect("🧱️ slot table size fits usize");
        assert!(table_bytes > threshold_bytes, "🧱️ {}: {table_bytes} B is under the {threshold_bytes} B conversion threshold — drop the row instead of carrying it", row.owner);
        assert!(row.owner_bytes < table_bytes, "🧱️ {}: the owner is {} B against a {table_bytes} B slot table, so the table is still inline — build it through boxed_fixed_slots", row.owner, row.owner_bytes);
    }
    on_bounded_stack(guard, stack_bytes, build);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn on_bounded_stack(name: &str, stack_bytes: usize, build: impl FnOnce() + Send + 'static) {
    let lane = std::thread::Builder::new().name(name.to_string()).stack_size(stack_bytes).spawn(build).expect("🧵️ the bounded-stack lane spawns");
    if let Err(panic) = lane.join() {
        std::panic::resume_unwind(panic);
    }
}
//#endregion 🧱️BoxedSlots
