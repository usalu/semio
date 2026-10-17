//! 🧱️ Fixed owners of the precompute geometry layer: the bookkeeping slot constants, the sub-page reservation and the fixed-capacity vector, map and set owners.

use super::*;

/// 🗜️ Bookkeeping slots: the default page width for owners the memory census walks and the
/// retirement cursor releases one entry per close grant — the sub-page a spatial cell's member bucket
/// grows by, the retiring hand-off slots. It bounds one interactive close step's
/// retirement work, never how large a document may be: document capacities are the
/// `DOCUMENT_*_SLOTS` constants below.
pub const FIXED_OWNER_SLOTS: usize = 32;

/// 📏️ Byte ceiling for one bookkeeping owner page.
pub const FIXED_OWNER_PAGE_BYTES: usize = 16 * 1024;

/// 📐️ Byte ceiling for one document-scale owner page, expressed in bookkeeping pages: the widest
/// declared page is `FixedOwnerVec<EngineSceneObject, DOCUMENT_OBJECT_SLOTS>` (≈432 KiB), and the fill
/// envelope reserves `FILL_ENVELOPE_MAX_BYTES` (256 × 16 KiB) for every page of one session
/// together, so a single page may claim at most a quarter of that reservation.
pub const DOCUMENT_OWNER_PAGE_BYTES: usize = 64 * FIXED_OWNER_PAGE_BYTES;

/// 🏢️ Objects the flagship scene_snapshot carries — the Nakagin capsule tower, the largest real document
/// this artifact plans against. Every document-scale page is sized from it plus
/// [`DOCUMENT_FILL_HEADROOM_SLOTS`], never from a plan ceiling: the planner has none any more.
pub const NAKAGIN_DOCUMENT_OBJECTS: usize = 180;

/// 🧊️ Fill placements a document page keeps free above the flagship scene_snapshot. It is a DOCUMENT
/// capacity, not a request ceiling — the count the user asks for is unbounded, and a plan that
/// exhausts these slots reports `stall_reason = "artifact-capacity"` and stops, visibly, instead of
/// being clamped or faulting.
pub const DOCUMENT_FILL_HEADROOM_SLOTS: usize = 1024;

/// 🧊️ Objects one fill session owns: the flagship scene_snapshot plus its fill headroom — 1204 — rounded up
/// to the next power of two, 2048, which also bounds `placed`/`placed_lookup`/the spatial entry map.
pub const DOCUMENT_OBJECT_SLOTS: usize = (NAKAGIN_DOCUMENT_OBJECTS + DOCUMENT_FILL_HEADROOM_SLOTS).next_power_of_two();

/// 🔘️ Vortices one fill session reasons about: Nakagin measures 358 vortices over 180 objects (≈2
/// per object, at most 10 on one object), so two per object slot bounds the blocked-vortex and
/// seen-candidate sets at document scale.
pub const DOCUMENT_VORTEX_SLOTS: usize = 2 * DOCUMENT_OBJECT_SLOTS;

/// 🧲️ Attractions one fill session owns: the scene's own plus one per placement — Nakagin carries
/// 358 — so it shares the object ceiling.
pub const DOCUMENT_ATTRACTION_SLOTS: usize = DOCUMENT_OBJECT_SLOTS;

/// 🧱️ Target volumes one fill session owns: a document declares fill regions by hand, so it stays
/// at the kind scale rather than the object scale.
pub const DOCUMENT_VOLUME_SLOTS: usize = DOCUMENT_KIND_SLOTS;

/// 🗂️ Catalog rows one fill session owns — object/vortex/cable kinds, compatibility rows, kind
/// weight maps, registered mesh urls. Nakagin declares 12 object kinds, 18 vortex kinds and 14
/// compatibility rows, so 256 is an order of magnitude of headroom on the flagship scene_snapshot.
pub const DOCUMENT_KIND_SLOTS: usize = 256;

/// 🎯️ Candidates one target vortex may enumerate: object kinds × their vortex templates, drained
/// again before the next target, so four templates per kind slot bounds the classification maps.
pub const DOCUMENT_CANDIDATE_SLOTS: usize = 4 * DOCUMENT_KIND_SLOTS;

/// 🗺️ Spatial hash cells one fill session may occupy: an object's AABB straddles up to a handful of
/// 8.0-world-unit cells, so four cells per object slot bounds the cell map; each cell's member bucket is
/// [`DOCUMENT_CELL_MEMBER_SLOTS`] wide.
pub const DOCUMENT_CELL_SLOTS: usize = 4 * DOCUMENT_OBJECT_SLOTS;

/// 🏙️ Members one spatial hash cell may hold: every object the session owns. A dense document — Nakagin's capsule tower
/// with its bodies' own meshes at 1.5× puts 43 bodies into one 8-unit cell — must never be refused by a cell before the
/// entry map refuses the object itself, so the only density bound is the document's object capacity. The bucket is
/// claimed in [`FIXED_OWNER_SLOTS`]-wide sub-pages, so a sparse cell still costs one bookkeeping page.
pub const DOCUMENT_CELL_MEMBER_SLOTS: usize = DOCUMENT_OBJECT_SLOTS;

/// 📏️ The sub-page ceiling an owner declares when only the guest's contiguous-request ceiling sizes its sub-pages.
pub const OWNER_SUB_PAGE_UNBOUNDED: usize = usize::MAX;

//#region 🧯️Reservation
/// 📏️ Slots ONE sub-page of an owner over `T` backs: the largest power of two whose block stays at
/// or under [`GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES`], and never more than the owner's declared
/// width — so a narrow owner keeps its single page and only a document-scale one is split.
///
/// 🧊️ The ceiling is `dlmalloc`'s wasm granularity, the unit `memory.grow` moves in: a request at or
/// under it is served from a small bin, a split of `dv`/`top`, or one page of growth, while a larger
/// one needs a pre-existing large free chunk or a multi-page grow — the FIRST request a fragmented
/// guest refuses. A power of two keeps the index arithmetic a shift and a mask on the hot path.
pub const fn owner_sub_page_slots<T>(slots: usize) -> usize {
    let width = size_of::<T>();
    if width == 0 || slots == 0 {
        return if slots == 0 { 1 } else { slots };
    }
    let fit = GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES / width;
    if fit >= slots {
        return slots;
    }
    if fit <= 1 {
        return 1;
    }
    let mut sub_page = 1;
    while sub_page * 2 <= fit {
        sub_page *= 2;
    }
    sub_page
}

/// 📏️ Slots ONE sub-page of an owner over `T` declared `slots` wide backs when it also declares a `ceiling` in slots:
/// [`owner_sub_page_slots`] over the narrower of the two, so a document-wide owner that is usually sparse grows in
/// bookkeeping-sized steps instead of claiming its guest-ceiling sub-page up front.
pub const fn owner_sub_page_slots_within<T>(slots: usize, ceiling: usize) -> usize {
    owner_sub_page_slots::<T>(if ceiling < slots { ceiling } else { slots })
}

/// 🧯️ The ONE heap request a fixed owner ever makes: one SUB-PAGE of `slots` elements. Every owner
/// page in this module is claimed here, so the guest's contiguous-request budget is enforced in a
/// single place and a law can refuse a request the way a fragmented guest does.
///
/// 🧊️ `Box::new(std::array::from_fn(..))` materializes the whole array as a stack temporary first,
/// which at document capacity is hundreds of kilobytes per owner — enough to overflow a test
/// thread's stack and far past any wasm guest's. `try_reserve_exact` keeps allocation failure a
/// handled `None` (a refused sub-page) instead of an abort.
fn claim_owner_sub_page<T>(slots: usize, fill: impl FnMut() -> T) -> Option<Box<[T]>> {
    if !owner_reservation_admits(slots.checked_mul(size_of::<T>())?) {
        return None;
    }
    let mut page: Vec<T> = Vec::new();
    page.try_reserve_exact(slots).ok()?;
    page.resize_with(slots, fill);
    Some(page.into_boxed_slice())
}

/// 🧯️ A guest whose linear memory is not fragmented serves every sub-page: production has no policy
/// and the check folds away. [`OwnerReservationLimit`] is how a law reaches the fragmented guest a
/// native suite cannot produce — a 512 MiB linear memory cannot be exhausted from a test.
#[cfg(not(any(test, feature = "testkit")))]
const fn owner_reservation_admits(_bytes: usize) -> bool {
    true
}

#[cfg(any(test, feature = "testkit"))]
thread_local! {
    static OWNER_RESERVATION: std::cell::Cell<(usize, usize)> = const { std::cell::Cell::new((usize::MAX, usize::MAX)) };
}

#[cfg(any(test, feature = "testkit"))]
fn owner_reservation_admits(bytes: usize) -> bool {
    OWNER_RESERVATION.with(|policy| {
        let (ceiling, grants) = policy.get();
        if bytes > ceiling || grants == 0 {
            return false;
        }
        policy.set((ceiling, grants - 1));
        true
    })
}

/// 🧯️ The fragmented guest, installed on THIS thread for the lifetime of the guard: every sub-page
/// request over `ceiling_bytes`, and every request past the `grants`th, is refused exactly the way
/// `dlmalloc` refuses a block it cannot serve out of a memory that never shrinks.
#[cfg(any(test, feature = "testkit"))]
pub struct OwnerReservationLimit(pub(super) (usize, usize));

#[cfg(any(test, feature = "testkit"))]
impl OwnerReservationLimit {
    pub fn install(ceiling_bytes: usize, grants: usize) -> Self {
        OWNER_RESERVATION.with(|policy| {
            let restored = policy.get();
            policy.set((ceiling_bytes, grants));
            Self(restored)
        })
    }
}

#[cfg(any(test, feature = "testkit"))]
impl Drop for OwnerReservationLimit {
    fn drop(&mut self) {
        OWNER_RESERVATION.with(|policy| policy.set(self.0));
    }
}
//#endregion 🧯️Reservation

#[derive(Debug)]
pub struct FixedOwnerVec<T, const N: usize = FIXED_OWNER_SLOTS> {
    pages: Vec<Box<[MaybeUninit<T>]>>,
    sealed: bool,
    pub(super) len: usize,
}

impl<T, const N: usize> FixedOwnerVec<T, N> {
    /// 📏️ Slots ONE sub-page of this owner backs — derived from `size_of::<T>()` and
    /// [`GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES`], never hand-picked.
    pub const fn sub_page_slots() -> usize {
        owner_sub_page_slots::<MaybeUninit<T>>(N)
    }

    /// 📏️ Bytes of ONE contiguous sub-page request this owner makes.
    pub const fn sub_page_bytes() -> usize {
        Self::sub_page_slots() * size_of::<MaybeUninit<T>>()
    }

    const fn sub_pages() -> usize {
        N.div_ceil(Self::sub_page_slots())
    }

    /// 🧱️ Claims the FIRST sub-page and reserves the (pointer-wide) cursor over the rest. The
    /// remaining sub-pages are claimed by [`FixedOwnerVec::try_push`] as `len` crosses their
    /// boundaries, so an owner declared at document width costs one sub-page until it is used and
    /// never asks the guest for a block over the contiguous ceiling.
    pub fn new() -> Self {
        const { assert!(Self::sub_page_bytes() <= GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES, "a fixed owner sub-page is over the guest contiguous-request ceiling") };
        const { assert!(Self::page_bytes() <= DOCUMENT_OWNER_PAGE_BYTES, "a fixed owner page is over the document page ceiling") };
        let mut pages = Vec::new();
        if pages.try_reserve_exact(Self::sub_pages()).is_err() {
            return Self::refused();
        }
        let mut owner = Self { pages, sealed: false, len: 0 };
        if !owner.claim_sub_page() {
            owner.sealed = true;
        }
        owner
    }

    /// 🚫️ The owner a guest that could not spare a page hands back: no backing, zero capacity,
    /// every insert refused. Production reaches it through [`FixedOwnerVec::new`]'s refused first
    /// sub-page; a law reaches it directly, because a native suite cannot exhaust a 512 MiB linear
    /// memory to get there.
    pub const fn refused() -> Self {
        Self { pages: Vec::new(), sealed: true, len: 0 }
    }

    pub const fn page_bytes() -> usize {
        size_of::<[MaybeUninit<T>; N]>()
    }

    fn claim_sub_page(&mut self) -> bool {
        let base = self.pages.len() * Self::sub_page_slots();
        if self.sealed || base >= N {
            return false;
        }
        let slots = if N - base < Self::sub_page_slots() { N - base } else { Self::sub_page_slots() };
        let Some(page) = claim_owner_sub_page(slots, MaybeUninit::uninit) else { return false };
        self.pages.push(page);
        true
    }

    fn backed_slots(&self) -> usize {
        let backed = self.pages.len() * Self::sub_page_slots();
        if backed > N { N } else { backed }
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn backing_ptr(&self) -> Option<*const MaybeUninit<T>> {
        self.pages.first().map(|page| page.as_ptr())
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn backing_credit(&self) -> Option<(usize, usize)> {
        (!self.pages.is_empty()).then(|| (self.pages.len(), self.pages.iter().map(|page| page.len() * size_of::<MaybeUninit<T>>()).sum()))
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// 📃️ The one contiguous view a SINGLE-sub-page owner still offers. A document-scale owner is
    /// backed by several sub-pages and has no such view — the const assertion refuses the call at
    /// compile time rather than handing back a prefix, and [`FixedOwnerVec::iter`] is the answer.
    pub fn as_slice(&self) -> &[T] {
        const { assert!(Self::sub_pages() == 1, "a multi-sub-page owner has no contiguous slice — iterate it instead") };
        let Some(page) = self.pages.first() else { return &[] };
        unsafe { std::slice::from_raw_parts(page.as_ptr().cast::<T>(), self.len) }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        let len = self.len;
        self.pages.iter().enumerate().flat_map(move |(index, page)| {
            let filled = len.saturating_sub(index * Self::sub_page_slots()).min(page.len());
            unsafe { std::slice::from_raw_parts(page.as_ptr().cast::<T>(), filled) }.iter()
        })
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        Some(unsafe { self.pages.get(index / Self::sub_page_slots())?.get(index % Self::sub_page_slots())?.assume_init_ref() })
    }

    /// ✍️ In-place mutation of a slot. Production never reaches for it — a fill session appends and
    /// pops its owners and rebuilds a changed object — so only the laws that seed one name it.
    #[cfg(test)]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            return None;
        }
        Some(unsafe { self.pages.get_mut(index / Self::sub_page_slots())?.get_mut(index % Self::sub_page_slots())?.assume_init_mut() })
    }

    /// 📏️ Slots this owner can actually hold. An owner whose sub-page the guest REFUSED reports
    /// what it is really backed by, never the declared width — reporting `N` there is the lie that
    /// turned an out-of-memory refusal into a guest trap (`live fixed owner page`, ticket 26/09/02
    /// build #29). Until a refusal every declared slot is still reachable, because the sub-pages a
    /// growing owner still needs have not been asked for yet.
    pub fn capacity(&self) -> usize {
        if self.sealed { self.backed_slots() } else { N }
    }

    pub fn try_push(&mut self, value: T) -> Result<(), T> {
        if self.len == self.capacity() {
            return Err(value);
        }
        if self.len == self.backed_slots() && !self.claim_sub_page() {
            self.sealed = true;
            return Err(value);
        }
        let (page, offset) = (self.len / Self::sub_page_slots(), self.len % Self::sub_page_slots());
        let Some(slot) = self.pages.get_mut(page).and_then(|page| page.get_mut(offset)) else { return Err(value) };
        slot.write(value);
        self.len += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        let (page, offset) = (self.len / Self::sub_page_slots(), self.len % Self::sub_page_slots());
        Some(unsafe { self.pages.get_mut(page)?.get_mut(offset)?.assume_init_read() })
    }

    /// ♻️ Gives ONE sub-page back per close grant, last claimed first, and seals the owner so a
    /// retired page is never silently re-claimed. The retirement cursor spends one grant per call,
    /// so a document-scale owner retires over as many grants as it holds sub-pages.
    pub fn retire_backing(&mut self) -> bool {
        if self.len != 0 {
            return false;
        }
        self.sealed = true;
        let released = self.pages.pop().is_some();
        if self.pages.is_empty() {
            self.pages = Vec::new();
        }
        released
    }

    pub fn terminal_owners_empty(&self) -> bool {
        self.len == 0 && self.pages.is_empty()
    }
}

impl<T, const N: usize> Drop for FixedOwnerVec<T, N> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

#[derive(Debug)]
pub struct FixedOwnerMap<K, V, const N: usize = FIXED_OWNER_SLOTS, const S: usize = OWNER_SUB_PAGE_UNBOUNDED> {
    pages: Vec<Box<[Option<(K, V)>]>>,
    sealed: bool,
    pub(super) len: usize,
}

#[derive(Debug)]
pub enum FixedOwnerMapInsert<K, V> {
    Inserted,
    Occupied { input_key: K, input_value: V },
}

impl<K, V, const N: usize, const S: usize> FixedOwnerMap<K, V, N, S> {
    /// 📏️ Slots ONE sub-page of this owner backs — see [`FixedOwnerVec::sub_page_slots`], never more than its declared
    /// sub-page ceiling `S`.
    pub const fn sub_page_slots() -> usize {
        owner_sub_page_slots_within::<Option<(K, V)>>(N, S)
    }

    /// 📏️ Bytes of ONE contiguous sub-page request this owner makes.
    pub const fn sub_page_bytes() -> usize {
        Self::sub_page_slots() * size_of::<Option<(K, V)>>()
    }

    const fn sub_pages() -> usize {
        N.div_ceil(Self::sub_page_slots())
    }

    /// 🧱️ Claims its first sub-page and grows one sub-page at a time, for the same reason as
    /// [`FixedOwnerVec::new`].
    pub fn new() -> Self {
        const { assert!(Self::sub_page_bytes() <= GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES, "a fixed owner sub-page is over the guest contiguous-request ceiling") };
        const { assert!(Self::page_bytes() <= DOCUMENT_OWNER_PAGE_BYTES, "a fixed owner page is over the document page ceiling") };
        let mut pages = Vec::new();
        if pages.try_reserve_exact(Self::sub_pages()).is_err() {
            return Self::refused();
        }
        let mut owner = Self { pages, sealed: false, len: 0 };
        if !owner.claim_sub_page() {
            owner.sealed = true;
        }
        owner
    }

    /// 🚫️ The owner a guest that could not spare a page hands back — see [`FixedOwnerVec::refused`].
    pub const fn refused() -> Self {
        Self { pages: Vec::new(), sealed: true, len: 0 }
    }

    pub const fn page_bytes() -> usize {
        size_of::<[Option<(K, V)>; N]>()
    }

    fn claim_sub_page(&mut self) -> bool {
        let base = self.pages.len() * Self::sub_page_slots();
        if self.sealed || base >= N {
            return false;
        }
        let slots = if N - base < Self::sub_page_slots() { N - base } else { Self::sub_page_slots() };
        let Some(page) = claim_owner_sub_page(slots, || None) else { return false };
        self.pages.push(page);
        true
    }

    fn backed_slots(&self) -> usize {
        let backed = self.pages.len() * Self::sub_page_slots();
        if backed > N { N } else { backed }
    }

    /// 📏️ Slots this owner can actually hold — what it is really backed by once a sub-page was
    /// refused, for the same reason as [`FixedOwnerVec::capacity`]. Every collision-mutation
    /// preflight compares `len()` against this, so an honest answer turns an exhausted guest into a
    /// refused mutation instead of an `unreachable!` two frames later.
    pub fn capacity(&self) -> usize {
        if self.sealed { self.backed_slots() } else { N }
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn backing_credit(&self) -> Option<(usize, usize)> {
        (!self.pages.is_empty()).then(|| (self.pages.len(), self.pages.iter().map(|page| page.len() * size_of::<Option<(K, V)>>()).sum()))
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn backing_ptr(&self) -> Option<*const Option<(K, V)>> {
        self.pages.first().map(|page| page.as_ptr())
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub(super) fn entry(&self, index: usize) -> Option<&Option<(K, V)>> {
        self.pages.get(index / Self::sub_page_slots())?.get(index % Self::sub_page_slots())
    }

    fn entry_mut(&mut self, index: usize) -> Option<&mut Option<(K, V)>> {
        self.pages.get_mut(index / Self::sub_page_slots())?.get_mut(index % Self::sub_page_slots())
    }

    fn take_at(&mut self, index: usize) -> Option<(K, V)> {
        self.entry_mut(index)?.take()
    }

    fn put_at(&mut self, index: usize, entry: Option<(K, V)>) {
        if let Some(slot) = self.entry_mut(index) {
            *slot = entry;
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &(K, V)> {
        let len = self.len;
        self.pages.iter().enumerate().flat_map(move |(index, page)| {
            let filled = len.saturating_sub(index * Self::sub_page_slots()).min(page.len());
            page[..filled].iter()
        })
        .filter_map(Option::as_ref)
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.iter().map(|(key, _)| key)
    }

    #[cfg(test)]
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.iter().map(|(_, value)| value)
    }

    fn index_of<Q>(&self, key: &Q) -> Option<usize>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.iter().position(|(candidate, _)| <K as Borrow<Q>>::borrow(candidate) == key)
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.entry(self.index_of(key)?)?.as_ref().map(|(_, value)| value)
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let index = self.index_of(key)?;
        self.entry_mut(index)?.as_mut().map(|(_, value)| value)
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.index_of(key).is_some()
    }

    pub fn try_insert(&mut self, key: K, value: V) -> Result<FixedOwnerMapInsert<K, V>, (K, V)>
    where
        K: Ord,
    {
        if self.index_of(&key).is_some() {
            return Ok(FixedOwnerMapInsert::Occupied { input_key: key, input_value: value });
        }
        if self.len == self.capacity() {
            return Err((key, value));
        }
        if self.len == self.backed_slots() && !self.claim_sub_page() {
            self.sealed = true;
            return Err((key, value));
        }
        let insert_at = self.iter().position(|(candidate, _)| candidate > &key).unwrap_or(self.len);
        for index in (insert_at..self.len).rev() {
            let moved = self.take_at(index);
            self.put_at(index + 1, moved);
        }
        self.put_at(insert_at, Some((key, value)));
        self.len += 1;
        Ok(FixedOwnerMapInsert::Inserted)
    }

    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let index = self.index_of(key)?;
        let entry = self.take_at(index)?;
        for cursor in index + 1..self.len {
            let moved = self.take_at(cursor);
            self.put_at(cursor - 1, moved);
        }
        self.len -= 1;
        Some(entry)
    }

    pub fn pop_first(&mut self) -> Option<(K, V)> {
        if self.len == 0 {
            return None;
        }
        let entry = self.take_at(0);
        for cursor in 1..self.len {
            let moved = self.take_at(cursor);
            self.put_at(cursor - 1, moved);
        }
        self.len -= 1;
        entry
    }

    /// ♻️ Gives ONE sub-page back per close grant — see [`FixedOwnerVec::retire_backing`].
    pub fn retire_backing(&mut self) -> bool {
        if self.len != 0 {
            return false;
        }
        self.sealed = true;
        let released = self.pages.pop().is_some();
        if self.pages.is_empty() {
            self.pages = Vec::new();
        }
        released
    }

    pub fn terminal_owners_empty(&self) -> bool {
        self.len == 0 && self.pages.is_empty()
    }
}

#[derive(Debug)]
pub struct FixedOwnerSet<K, const N: usize = FIXED_OWNER_SLOTS, const S: usize = OWNER_SUB_PAGE_UNBOUNDED> {
    pub(super) values: FixedOwnerMap<K, (), N, S>,
}

#[derive(Debug)]
pub enum FixedOwnerSetInsert<K> {
    Inserted,
    Present { input: K },
}

impl<K, const N: usize, const S: usize> FixedOwnerSet<K, N, S> {
    pub fn new() -> Self {
        Self { values: FixedOwnerMap::new() }
    }

    /// 🚫️ The owner a guest that could not spare the page hands back — see [`FixedOwnerVec::refused`].
    /// Production reaches the state through [`FixedOwnerSet::new`]'s inner map; only the law names it.
    #[cfg(test)]
    pub const fn refused() -> Self {
        Self { values: FixedOwnerMap::refused() }
    }

    pub fn capacity(&self) -> usize {
        self.values.capacity()
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn backing_credit(&self) -> Option<(usize, usize)> {
        self.values.backing_credit()
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn backing_ptr(&self) -> Option<*const Option<(K, ())>> {
        self.values.backing_ptr()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &K> {
        self.values.keys()
    }

    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.values.contains_key(value)
    }

    pub fn try_insert(&mut self, value: K) -> Result<FixedOwnerSetInsert<K>, K>
    where
        K: Ord,
    {
        self.values
            .try_insert(value, ())
            .map(|outcome| match outcome {
                FixedOwnerMapInsert::Inserted => FixedOwnerSetInsert::Inserted,
                FixedOwnerMapInsert::Occupied { input_key, input_value: () } => FixedOwnerSetInsert::Present { input: input_key },
            })
            .map_err(|(value, ())| value)
    }

    pub fn remove_entry<Q>(&mut self, value: &Q) -> Option<K>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.values.remove_entry(value).map(|(value, ())| value)
    }

    pub fn pop_first(&mut self) -> Option<K> {
        self.values.pop_first().map(|(value, ())| value)
    }

    pub fn retire_backing(&mut self) -> bool {
        self.values.retire_backing()
    }

    pub fn terminal_owners_empty(&self) -> bool {
        self.values.terminal_owners_empty()
    }
}
