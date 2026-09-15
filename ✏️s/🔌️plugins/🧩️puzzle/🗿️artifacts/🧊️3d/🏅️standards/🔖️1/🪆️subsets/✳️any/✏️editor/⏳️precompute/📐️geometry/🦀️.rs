//! 📐️ Puzzle 3d play app — the precompute geometry layer: the `semio_framework_3d::{rigid,
//! collision}` adapter (the ONE interface boundary this module depends on), the plain
//! `[f64; 3]`/`[f64; 4]` vector and quaternion math the placement solver builds on, the brush
//! placement pose solver itself, and the collision-body/AABB/overlap primitives the brush and
//! fill lanes gate placements with. Rehomed from the former `⚙️engine/📐️geometry` (ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): this is interactive brush/fill tool
//! behaviour, so it lives with the app, not the artifact. The `nalgebra`/`parry3d` third-party
//! surface this adapter used to wrap moved into the framework (ticket
//! 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS) — see
//! `semio_framework_3d::rigid` (vectors/points/quaternions/isometries) and
//! `semio_framework_3d::collision` (BVH triangle-mesh intersection + winding-number containment).

use crate::standards::v1::subsets::any::schema::{Quat, Vec3, WorldVolumeProps};
use semio_framework_3d::{collision, rigid};
use semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES;
use std::borrow::Borrow;
use std::mem::MaybeUninit;

/// 🗜️ Bookkeeping slots: the default page width for owners the memory census walks and the
/// retirement cursor releases one entry per close grant — the sub-page a spatial cell's member bucket
/// grows by, the retiring hand-off slots. It bounds one interactive close step's
/// retirement work, never how large a document may be: document capacities are the
/// `DOCUMENT_*_SLOTS` constants below.
pub(crate) const FIXED_OWNER_SLOTS: usize = 32;
/// 📏️ Byte ceiling for one bookkeeping owner page.
pub(crate) const FIXED_OWNER_PAGE_BYTES: usize = 16 * 1024;
/// 📐️ Byte ceiling for one document-scale owner page, expressed in bookkeeping pages: the widest
/// declared page is `FixedOwnerVec<FixtureObject, DOCUMENT_OBJECT_SLOTS>` (≈432 KiB), and the fill
/// envelope reserves `FILL_ENVELOPE_MAX_BYTES` (256 × 16 KiB) for every page of one session
/// together, so a single page may claim at most a quarter of that reservation.
pub(crate) const DOCUMENT_OWNER_PAGE_BYTES: usize = 64 * FIXED_OWNER_PAGE_BYTES;
/// 🏢️ Objects the flagship fixture carries — the Nakagin capsule tower, the largest real document
/// this artifact plans against. Every document-scale page is sized from it plus
/// [`DOCUMENT_FILL_HEADROOM_SLOTS`], never from a plan ceiling: the planner has none any more.
pub(crate) const NAKAGIN_DOCUMENT_OBJECTS: usize = 180;
/// 🧊️ Fill placements a document page keeps free above the flagship fixture. It is a DOCUMENT
/// capacity, not a request ceiling — the count the user asks for is unbounded, and a plan that
/// exhausts these slots reports `stall_reason = "artifact-capacity"` and stops, visibly, instead of
/// being clamped or faulting.
pub(crate) const DOCUMENT_FILL_HEADROOM_SLOTS: usize = 1024;
/// 🧊️ Objects one fill session owns: the flagship fixture plus its fill headroom — 1204 — rounded up
/// to the next power of two, 2048, which also bounds `placed`/`placed_lookup`/the spatial entry map.
pub(crate) const DOCUMENT_OBJECT_SLOTS: usize = (NAKAGIN_DOCUMENT_OBJECTS + DOCUMENT_FILL_HEADROOM_SLOTS).next_power_of_two();
/// 🔘️ Vortices one fill session reasons about: Nakagin measures 358 vortices over 180 objects (≈2
/// per object, at most 10 on one object), so two per object slot bounds the blocked-vortex and
/// seen-candidate sets at document scale.
pub(crate) const DOCUMENT_VORTEX_SLOTS: usize = 2 * DOCUMENT_OBJECT_SLOTS;
/// 🧲️ Attractions one fill session owns: the scene's own plus one per placement — Nakagin carries
/// 358 — so it shares the object ceiling.
pub(crate) const DOCUMENT_ATTRACTION_SLOTS: usize = DOCUMENT_OBJECT_SLOTS;
/// 🧱️ Target volumes one fill session owns: a document declares fill regions by hand, so it stays
/// at the kind scale rather than the object scale.
pub(crate) const DOCUMENT_VOLUME_SLOTS: usize = DOCUMENT_KIND_SLOTS;
/// 🗂️ Catalog rows one fill session owns — object/vortex/cable kinds, compatibility rows, kind
/// weight maps, registered mesh urls. Nakagin declares 12 object kinds, 18 vortex kinds and 14
/// compatibility rows, so 256 is an order of magnitude of headroom on the flagship fixture.
pub(crate) const DOCUMENT_KIND_SLOTS: usize = 256;
/// 🎯️ Candidates one target vortex may enumerate: object kinds × their vortex templates, drained
/// again before the next target, so four templates per kind slot bounds the classification maps.
pub(crate) const DOCUMENT_CANDIDATE_SLOTS: usize = 4 * DOCUMENT_KIND_SLOTS;
/// 🗺️ Spatial hash cells one fill session may occupy: an object's AABB straddles up to a handful of
/// 8.0-world-unit cells, so four cells per object slot bounds the cell map; each cell's member bucket is
/// [`DOCUMENT_CELL_MEMBER_SLOTS`] wide.
pub(crate) const DOCUMENT_CELL_SLOTS: usize = 4 * DOCUMENT_OBJECT_SLOTS;
/// 🏙️ Members one spatial hash cell may hold: every object the session owns. A dense document — Nakagin's capsule tower
/// with its bodies' own meshes at 1.5× puts 43 bodies into one 8-unit cell — must never be refused by a cell before the
/// entry map refuses the object itself, so the only density bound is the document's object capacity. The bucket is
/// claimed in [`FIXED_OWNER_SLOTS`]-wide sub-pages, so a sparse cell still costs one bookkeeping page.
pub(crate) const DOCUMENT_CELL_MEMBER_SLOTS: usize = DOCUMENT_OBJECT_SLOTS;
/// 📏️ The sub-page ceiling an owner declares when only the guest's contiguous-request ceiling sizes its sub-pages.
pub(crate) const OWNER_SUB_PAGE_UNBOUNDED: usize = usize::MAX;

//#region 🧯️Reservation
/// 📏️ Slots ONE sub-page of an owner over `T` backs: the largest power of two whose block stays at
/// or under [`GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES`], and never more than the owner's declared
/// width — so a narrow owner keeps its single page and only a document-scale one is split.
///
/// 🧊️ The ceiling is `dlmalloc`'s wasm granularity, the unit `memory.grow` moves in: a request at or
/// under it is served from a small bin, a split of `dv`/`top`, or one page of growth, while a larger
/// one needs a pre-existing large free chunk or a multi-page grow — the FIRST request a fragmented
/// guest refuses. A power of two keeps the index arithmetic a shift and a mask on the hot path.
pub(crate) const fn owner_sub_page_slots<T>(slots: usize) -> usize {
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
pub(crate) const fn owner_sub_page_slots_within<T>(slots: usize, ceiling: usize) -> usize {
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
#[cfg(not(test))]
const fn owner_reservation_admits(_bytes: usize) -> bool {
    true
}

#[cfg(test)]
thread_local! {
    static OWNER_RESERVATION: std::cell::Cell<(usize, usize)> = const { std::cell::Cell::new((usize::MAX, usize::MAX)) };
}

#[cfg(test)]
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
#[cfg(test)]
pub(crate) struct OwnerReservationLimit((usize, usize));

#[cfg(test)]
impl OwnerReservationLimit {
    pub(crate) fn install(ceiling_bytes: usize, grants: usize) -> Self {
        OWNER_RESERVATION.with(|policy| {
            let restored = policy.get();
            policy.set((ceiling_bytes, grants));
            Self(restored)
        })
    }
}

#[cfg(test)]
impl Drop for OwnerReservationLimit {
    fn drop(&mut self) {
        OWNER_RESERVATION.with(|policy| policy.set(self.0));
    }
}
//#endregion 🧯️Reservation

#[derive(Debug)]
pub(crate) struct FixedOwnerVec<T, const N: usize = FIXED_OWNER_SLOTS> {
    pages: Vec<Box<[MaybeUninit<T>]>>,
    sealed: bool,
    len: usize,
}

impl<T, const N: usize> FixedOwnerVec<T, N> {
    /// 📏️ Slots ONE sub-page of this owner backs — derived from `size_of::<T>()` and
    /// [`GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES`], never hand-picked.
    pub(crate) const fn sub_page_slots() -> usize {
        owner_sub_page_slots::<MaybeUninit<T>>(N)
    }

    /// 📏️ Bytes of ONE contiguous sub-page request this owner makes.
    pub(crate) const fn sub_page_bytes() -> usize {
        Self::sub_page_slots() * size_of::<MaybeUninit<T>>()
    }

    const fn sub_pages() -> usize {
        N.div_ceil(Self::sub_page_slots())
    }

    /// 🧱️ Claims the FIRST sub-page and reserves the (pointer-wide) cursor over the rest. The
    /// remaining sub-pages are claimed by [`FixedOwnerVec::try_push`] as `len` crosses their
    /// boundaries, so an owner declared at document width costs one sub-page until it is used and
    /// never asks the guest for a block over the contiguous ceiling.
    pub(crate) fn new() -> Self {
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
    pub(crate) const fn refused() -> Self {
        Self { pages: Vec::new(), sealed: true, len: 0 }
    }

    pub(crate) const fn page_bytes() -> usize {
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

    #[cfg(test)]
    pub(crate) fn backing_ptr(&self) -> Option<*const MaybeUninit<T>> {
        self.pages.first().map(|page| page.as_ptr())
    }

    #[cfg(test)]
    pub(crate) fn backing_credit(&self) -> Option<(usize, usize)> {
        (!self.pages.is_empty()).then(|| (self.pages.len(), self.pages.iter().map(|page| page.len() * size_of::<MaybeUninit<T>>()).sum()))
    }

    pub(crate) fn len(&self) -> usize {
        self.len
    }

    /// 📃️ The one contiguous view a SINGLE-sub-page owner still offers. A document-scale owner is
    /// backed by several sub-pages and has no such view — the const assertion refuses the call at
    /// compile time rather than handing back a prefix, and [`FixedOwnerVec::iter`] is the answer.
    pub(crate) fn as_slice(&self) -> &[T] {
        const { assert!(Self::sub_pages() == 1, "a multi-sub-page owner has no contiguous slice — iterate it instead") };
        let Some(page) = self.pages.first() else { return &[] };
        unsafe { std::slice::from_raw_parts(page.as_ptr().cast::<T>(), self.len) }
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &T> {
        let len = self.len;
        self.pages.iter().enumerate().flat_map(move |(index, page)| {
            let filled = len.saturating_sub(index * Self::sub_page_slots()).min(page.len());
            unsafe { std::slice::from_raw_parts(page.as_ptr().cast::<T>(), filled) }.iter()
        })
    }

    pub(crate) fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        Some(unsafe { self.pages.get(index / Self::sub_page_slots())?.get(index % Self::sub_page_slots())?.assume_init_ref() })
    }

    /// ✍️ In-place mutation of a slot. Production never reaches for it — a fill session appends and
    /// pops its owners and rebuilds a changed object — so only the laws that seed one name it.
    #[cfg(test)]
    pub(crate) fn get_mut(&mut self, index: usize) -> Option<&mut T> {
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
    pub(crate) fn capacity(&self) -> usize {
        if self.sealed { self.backed_slots() } else { N }
    }

    pub(crate) fn try_push(&mut self, value: T) -> Result<(), T> {
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

    pub(crate) fn pop(&mut self) -> Option<T> {
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
    pub(crate) fn retire_backing(&mut self) -> bool {
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

    pub(crate) fn terminal_owners_empty(&self) -> bool {
        self.len == 0 && self.pages.is_empty()
    }
}

impl<T, const N: usize> Drop for FixedOwnerVec<T, N> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

#[derive(Debug)]
pub(crate) struct FixedOwnerMap<K, V, const N: usize = FIXED_OWNER_SLOTS, const S: usize = OWNER_SUB_PAGE_UNBOUNDED> {
    pages: Vec<Box<[Option<(K, V)>]>>,
    sealed: bool,
    len: usize,
}

#[derive(Debug)]
pub(crate) enum FixedOwnerMapInsert<K, V> {
    Inserted,
    Occupied { input_key: K, input_value: V },
}

impl<K, V, const N: usize, const S: usize> FixedOwnerMap<K, V, N, S> {
    /// 📏️ Slots ONE sub-page of this owner backs — see [`FixedOwnerVec::sub_page_slots`], never more than its declared
    /// sub-page ceiling `S`.
    pub(crate) const fn sub_page_slots() -> usize {
        owner_sub_page_slots_within::<Option<(K, V)>>(N, S)
    }

    /// 📏️ Bytes of ONE contiguous sub-page request this owner makes.
    pub(crate) const fn sub_page_bytes() -> usize {
        Self::sub_page_slots() * size_of::<Option<(K, V)>>()
    }

    const fn sub_pages() -> usize {
        N.div_ceil(Self::sub_page_slots())
    }

    /// 🧱️ Claims its first sub-page and grows one sub-page at a time, for the same reason as
    /// [`FixedOwnerVec::new`].
    pub(crate) fn new() -> Self {
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
    pub(crate) const fn refused() -> Self {
        Self { pages: Vec::new(), sealed: true, len: 0 }
    }

    pub(crate) const fn page_bytes() -> usize {
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
    pub(crate) fn capacity(&self) -> usize {
        if self.sealed { self.backed_slots() } else { N }
    }

    #[cfg(test)]
    pub(crate) fn backing_credit(&self) -> Option<(usize, usize)> {
        (!self.pages.is_empty()).then(|| (self.pages.len(), self.pages.iter().map(|page| page.len() * size_of::<Option<(K, V)>>()).sum()))
    }

    #[cfg(test)]
    pub(crate) fn backing_ptr(&self) -> Option<*const Option<(K, V)>> {
        self.pages.first().map(|page| page.as_ptr())
    }

    pub(crate) fn len(&self) -> usize {
        self.len
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn entry(&self, index: usize) -> Option<&Option<(K, V)>> {
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

    pub(crate) fn iter(&self) -> impl Iterator<Item = &(K, V)> {
        let len = self.len;
        self.pages.iter().enumerate().flat_map(move |(index, page)| {
            let filled = len.saturating_sub(index * Self::sub_page_slots()).min(page.len());
            page[..filled].iter()
        })
        .filter_map(Option::as_ref)
    }

    pub(crate) fn keys(&self) -> impl Iterator<Item = &K> {
        self.iter().map(|(key, _)| key)
    }

    #[cfg(test)]
    pub(crate) fn values(&self) -> impl Iterator<Item = &V> {
        self.iter().map(|(_, value)| value)
    }

    fn index_of<Q>(&self, key: &Q) -> Option<usize>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.iter().position(|(candidate, _)| <K as Borrow<Q>>::borrow(candidate) == key)
    }

    pub(crate) fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.entry(self.index_of(key)?)?.as_ref().map(|(_, value)| value)
    }

    pub(crate) fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let index = self.index_of(key)?;
        self.entry_mut(index)?.as_mut().map(|(_, value)| value)
    }

    pub(crate) fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.index_of(key).is_some()
    }

    pub(crate) fn try_insert(&mut self, key: K, value: V) -> Result<FixedOwnerMapInsert<K, V>, (K, V)>
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

    pub(crate) fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
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

    pub(crate) fn pop_first(&mut self) -> Option<(K, V)> {
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
    pub(crate) fn retire_backing(&mut self) -> bool {
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

    pub(crate) fn terminal_owners_empty(&self) -> bool {
        self.len == 0 && self.pages.is_empty()
    }
}

#[derive(Debug)]
pub(crate) struct FixedOwnerSet<K, const N: usize = FIXED_OWNER_SLOTS, const S: usize = OWNER_SUB_PAGE_UNBOUNDED> {
    values: FixedOwnerMap<K, (), N, S>,
}

#[derive(Debug)]
pub(crate) enum FixedOwnerSetInsert<K> {
    Inserted,
    Present { input: K },
}

impl<K, const N: usize, const S: usize> FixedOwnerSet<K, N, S> {
    pub(crate) fn new() -> Self {
        Self { values: FixedOwnerMap::new() }
    }

    /// 🚫️ The owner a guest that could not spare the page hands back — see [`FixedOwnerVec::refused`].
    /// Production reaches the state through [`FixedOwnerSet::new`]'s inner map; only the law names it.
    #[cfg(test)]
    pub(crate) const fn refused() -> Self {
        Self { values: FixedOwnerMap::refused() }
    }

    pub(crate) fn capacity(&self) -> usize {
        self.values.capacity()
    }

    #[cfg(test)]
    pub(crate) fn backing_credit(&self) -> Option<(usize, usize)> {
        self.values.backing_credit()
    }

    #[cfg(test)]
    pub(crate) fn backing_ptr(&self) -> Option<*const Option<(K, ())>> {
        self.values.backing_ptr()
    }

    pub(crate) fn len(&self) -> usize {
        self.values.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &K> {
        self.values.keys()
    }

    pub(crate) fn contains<Q>(&self, value: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.values.contains_key(value)
    }

    pub(crate) fn try_insert(&mut self, value: K) -> Result<FixedOwnerSetInsert<K>, K>
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

    pub(crate) fn remove_entry<Q>(&mut self, value: &Q) -> Option<K>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.values.remove_entry(value).map(|(value, ())| value)
    }

    pub(crate) fn pop_first(&mut self) -> Option<K> {
        self.values.pop_first().map(|(value, ())| value)
    }

    pub(crate) fn retire_backing(&mut self) -> bool {
        self.values.retire_backing()
    }

    pub(crate) fn terminal_owners_empty(&self) -> bool {
        self.values.terminal_owners_empty()
    }
}

//#region 🔒️GeometryAdapter
/// 🔒️ Thin wrappers over `semio_framework_3d::{rigid, collision}` — the one interface boundary
/// this artifact depends on.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Vec3d(rigid::Vector3);

impl Vec3d {
    pub(crate) fn new(x: f32, y: f32, z: f32) -> Self {
        Self(rigid::Vector3::new(x, y, z))
    }
    pub(crate) fn x(&self) -> f32 {
        self.0.x
    }
    pub(crate) fn y(&self) -> f32 {
        self.0.y
    }
    pub(crate) fn z(&self) -> f32 {
        self.0.z
    }
    pub(crate) fn amax(&self) -> f32 {
        self.0.amax()
    }
}

impl std::ops::Add for Vec3d {
    type Output = Vec3d;
    fn add(self, rhs: Self) -> Self {
        Vec3d(self.0 + rhs.0)
    }
}

impl std::ops::Mul<f32> for Vec3d {
    type Output = Vec3d;
    fn mul(self, rhs: f32) -> Self {
        Vec3d(self.0 * rhs)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Point3d(rigid::Point3);

impl Point3d {
    pub(crate) fn new(x: f32, y: f32, z: f32) -> Self {
        Self(rigid::Point3::new(x, y, z))
    }
    pub(crate) fn x(&self) -> f32 {
        self.0.x
    }
    pub(crate) fn y(&self) -> f32 {
        self.0.y
    }
    pub(crate) fn z(&self) -> f32 {
        self.0.z
    }
    pub(crate) fn inf(&self, other: &Self) -> Self {
        Self(self.0.inf(other.0))
    }
    pub(crate) fn sup(&self, other: &Self) -> Self {
        Self(self.0.sup(other.0))
    }
    #[cfg(test)]
    pub(crate) fn coords(&self) -> Vec3d {
        Vec3d(self.0.coords())
    }
    #[cfg(test)]
    pub(crate) fn from_coords(v: Vec3d) -> Self {
        Self(rigid::Point3::from_coords(v.0))
    }
}

impl std::ops::Sub for Point3d {
    type Output = Vec3d;
    fn sub(self, rhs: Self) -> Vec3d {
        Vec3d(self.0 - rhs.0)
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Rotation3d(rigid::UnitQuaternion);

impl Rotation3d {
    pub(crate) fn identity() -> Self {
        Self(rigid::UnitQuaternion::identity())
    }
    /// 🔓️ Builds from CAD's `[i, j, k, w]` quaternion convention.
    pub(crate) fn from_ijkw(i: f32, j: f32, k: f32, w: f32) -> Self {
        Self(rigid::UnitQuaternion::from_quaternion(rigid::Quaternion::new(w, i, j, k)))
    }
    pub(crate) fn to_ijkw(self) -> (f32, f32, f32, f32) {
        let q = self.0.quaternion();
        (q.i, q.j, q.k, q.w)
    }
    pub(crate) fn rotation_between(from: Vec3d, to: Vec3d) -> Option<Self> {
        rigid::UnitQuaternion::rotation_between(from.0, to.0).map(Self)
    }
    pub(crate) fn apply(&self, v: Vec3d) -> Vec3d {
        Vec3d(self.0.apply(v.0))
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Pose3d(rigid::Isometry3);

impl Pose3d {
    pub(crate) fn identity() -> Self {
        Self(rigid::Isometry3::identity())
    }
    pub(crate) fn from_parts(translation: Vec3d, rotation: Rotation3d) -> Self {
        Self(rigid::Isometry3::from_parts(translation.0, rotation.0))
    }
    pub(crate) fn inverse(&self) -> Self {
        Self(self.0.inverse())
    }
    pub(crate) fn transform_point(&self, point: &Point3d) -> Point3d {
        Point3d(self.0.transform_point(point.0))
    }
    pub(crate) fn semio_compose_rs(&self, other: &Self) -> Self {
        Self(self.0.compose(other.0))
    }
}

#[derive(Clone)]
pub(crate) struct CollisionShape {
    shape: std::sync::Arc<collision::TriMesh>,
}

impl CollisionShape {
    pub(crate) fn from_triangle_mesh(vertices: &[Point3d], indices: Vec<[u32; 3]>) -> Self {
        let verts: Vec<rigid::Point3> = vertices.iter().map(|p| p.0).collect();
        Self { shape: std::sync::Arc::new(collision::TriMesh::new(verts, indices)) }
    }
    pub(crate) fn contains_point(&self, pose: &Pose3d, point: &Point3d) -> bool {
        collision::contains_point(pose.0, &self.shape, point.0)
    }
}

fn shapes_intersect(pose_a: &Pose3d, a: &CollisionShape, pose_b: &Pose3d, b: &CollisionShape) -> bool {
    collision::intersection_test(pose_a.0, &a.shape, pose_b.0, &b.shape)
}
//#endregion 🔒️GeometryAdapter

//#region 🔖️Constants
const BRUSH_COLLISION_MESH_MIN_EXTENT: f64 = 2.0;
/// 🕳️ Distance (m) below which a probe is contact, not penetration, whatever the tolerance: single-precision noise of
/// faces that meet exactly (1e-7 m measured on docked cubes) must never read as a collision.
const COLLISION_CONTACT_NOISE_M: f64 = 1e-4;
/// 🕳️ Probes per edge of a clipped face polygon (its first corner included), see `CollisionPenetrationState::step_face`.
const COLLISION_EDGE_PROBES: usize = 8;
/// 🕳️ Smallest and largest half inward offset of a coincident-surface probe (m): a zero tolerance still probes off the
/// surface, and an unbounded measure (an infinite tolerance) still probes a real point instead of one at infinity.
const COLLISION_INSET_MIN_M: f64 = 1e-4;
const COLLISION_INSET_MAX_M: f64 = 0.05;
pub(crate) const BRUSH_PLACEMENT_PARALLEL_TOLERANCE: f64 = 1e-6;
//#endregion 🔖️Constants

//#region 🔖️Vectors
pub(crate) fn normalize_vec3(v: Vec3) -> Vec3 {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len < 1e-9 {
        return [0.0, 0.0, -1.0];
    }
    [v[0] / len, v[1] / len, v[2] / len]
}

pub(crate) fn vec3_dot(a: Vec3, b: Vec3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

pub(crate) fn vec3_cross(a: Vec3, b: Vec3) -> Vec3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

pub(crate) fn vec3_add(a: Vec3, b: Vec3) -> Vec3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

pub(crate) fn vec3_sub(a: Vec3, b: Vec3) -> Vec3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

pub(crate) fn negate_vec3(v: Vec3) -> Vec3 {
    [-v[0], -v[1], -v[2]]
}

pub(crate) fn vec3_scale(v: Vec3, scale: &Option<dsl::DslValue>) -> Vec3 {
    match scale {
        None => v,
        Some(dsl::DslValue::Number(n)) => {
            let s = n.as_f64();
            [v[0] * s, v[1] * s, v[2] * s]
        }
        Some(dsl::DslValue::Array(arr)) if arr.len() >= 3 => {
            let sx = arr[0].as_f64().unwrap_or(1.0);
            let sy = arr[1].as_f64().unwrap_or(1.0);
            let sz = arr[2].as_f64().unwrap_or(1.0);
            [v[0] * sx, v[1] * sy, v[2] * sz]
        }
        _ => v,
    }
}

pub(crate) fn unit_quat_from_cad(q: Quat) -> Rotation3d {
    Rotation3d::from_ijkw(q[0] as f32, q[1] as f32, q[2] as f32, q[3] as f32)
}

pub(crate) fn quat_rotate_vec(q: Quat, v: Vec3) -> Vec3 {
    let uq = unit_quat_from_cad(q);
    let rotated = uq.apply(Vec3d::new(v[0] as f32, v[1] as f32, v[2] as f32));
    [rotated.x() as f64, rotated.y() as f64, rotated.z() as f64]
}

pub(crate) fn quaternion_from_180_degree_axis(axis: Vec3) -> Quat {
    let unit = normalize_vec3(axis);
    [unit[0], unit[1], unit[2], 0.0]
}

pub(crate) fn anti_parallel_brush_orientation(target_dir: Vec3) -> Quat {
    let z_axis: Vec3 = [0.0, 0.0, 1.0];
    if target_dir[2].abs() < BRUSH_PLACEMENT_PARALLEL_TOLERANCE {
        return quaternion_from_180_degree_axis(z_axis);
    }
    let axis = vec3_cross(z_axis, target_dir);
    if (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt() < BRUSH_PLACEMENT_PARALLEL_TOLERANCE {
        return quaternion_from_180_degree_axis([1.0, 0.0, 0.0]);
    }
    quaternion_from_180_degree_axis(axis)
}

pub(crate) fn pose_isometry(origin: Vec3, orientation: Quat, _scale: &Option<dsl::DslValue>) -> Pose3d {
    let q = unit_quat_from_cad(orientation);
    let t = Vec3d::new(origin[0] as f32, origin[1] as f32, origin[2] as f32);
    Pose3d::from_parts(t, q)
}

pub(crate) fn compute_brush_placement_pose(
    source_local_position: Vec3,
    source_local_direction: Vec3,
    scale: &Option<dsl::DslValue>,
    target_world_position: Vec3,
    target_world_direction: Vec3,
    reference_orientation: Option<Quat>,
    use_host_orientation: bool,
) -> (Vec3, Quat) {
    let scaled_local = vec3_scale(source_local_position, scale);
    let local_dir = normalize_vec3(source_local_direction);
    let target_dir = normalize_vec3(target_world_direction);
    if use_host_orientation {
        if let Some(host_orientation) = reference_orientation {
            let world_source_dir = normalize_vec3(quat_rotate_vec(host_orientation, local_dir));
            if vec3_dot(world_source_dir, target_dir) < -BRUSH_PLACEMENT_PARALLEL_TOLERANCE {
                let origin = vec3_sub(target_world_position, quat_rotate_vec(host_orientation, scaled_local));
                return (origin, host_orientation);
            }
        }
    }
    let desired_world_dir = negate_vec3(target_dir);
    let orientation = if vec3_dot(local_dir, desired_world_dir) < -1.0 + BRUSH_PLACEMENT_PARALLEL_TOLERANCE {
        anti_parallel_brush_orientation(target_dir)
    } else {
        let from = Vec3d::new(local_dir[0] as f32, local_dir[1] as f32, local_dir[2] as f32);
        let to = Vec3d::new(desired_world_dir[0] as f32, desired_world_dir[1] as f32, desired_world_dir[2] as f32);
        let q = Rotation3d::rotation_between(from, to).unwrap_or(Rotation3d::identity());
        let (i, j, k, w) = q.to_ijkw();
        [i as f64, j as f64, k as f64, w as f64]
    };
    let origin = vec3_sub(target_world_position, quat_rotate_vec(orientation, scaled_local));
    (origin, orientation)
}
//#endregion 🔖️Vectors

//#region 🔖️Collision
#[derive(Clone)]
pub(crate) struct CollisionMeshPart {
    pub(crate) shape: CollisionShape,
    pub(crate) local_pose: Pose3d,
}

#[derive(Clone)]
pub(crate) struct CollisionBody {
    pub(crate) parts: Vec<CollisionMeshPart>,
    pub(crate) local_bounds_min: Point3d,
    pub(crate) local_bounds_max: Point3d,
}

pub(crate) fn collision_body_from_buffers(positions: &[f32], indices: &[u32]) -> Option<CollisionBody> {
    if positions.len() < 9 || indices.len() < 3 {
        return None;
    }
    let mut verts: Vec<Point3d> = Vec::with_capacity(positions.len() / 3);
    let mut min = Point3d::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
    let mut max = Point3d::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);
    for chunk in positions.as_chunks::<3>().0 {
        let rp = Point3d::new(chunk[0], chunk[1], chunk[2]);
        verts.push(rp);
        min = min.inf(&rp);
        max = max.sup(&rp);
    }
    let extent = (max - min).amax();
    if !extent.is_finite() || extent < BRUSH_COLLISION_MESH_MIN_EXTENT as f32 {
        return None;
    }
    let mut tris: Vec<[u32; 3]> = Vec::with_capacity(indices.len() / 3);
    for chunk in indices.as_chunks::<3>().0 {
        tris.push(*chunk);
    }
    let shape = CollisionShape::from_triangle_mesh(&verts, tris);
    Some(CollisionBody { parts: vec![CollisionMeshPart { shape, local_pose: Pose3d::identity() }], local_bounds_min: min, local_bounds_max: max })
}

pub(crate) fn world_bounds(body: &CollisionBody, world: &Pose3d) -> (Point3d, Point3d) {
    let corners = [
        Point3d::new(body.local_bounds_min.x(), body.local_bounds_min.y(), body.local_bounds_min.z()),
        Point3d::new(body.local_bounds_max.x(), body.local_bounds_min.y(), body.local_bounds_min.z()),
        Point3d::new(body.local_bounds_min.x(), body.local_bounds_max.y(), body.local_bounds_min.z()),
        Point3d::new(body.local_bounds_max.x(), body.local_bounds_max.y(), body.local_bounds_min.z()),
        Point3d::new(body.local_bounds_min.x(), body.local_bounds_min.y(), body.local_bounds_max.z()),
        Point3d::new(body.local_bounds_max.x(), body.local_bounds_min.y(), body.local_bounds_max.z()),
        Point3d::new(body.local_bounds_min.x(), body.local_bounds_max.y(), body.local_bounds_max.z()),
        Point3d::new(body.local_bounds_max.x(), body.local_bounds_max.y(), body.local_bounds_max.z()),
    ];
    let mut min = Point3d::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
    let mut max = Point3d::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);
    for corner in corners {
        let w = world.transform_point(&corner);
        min = min.inf(&w);
        max = max.sup(&w);
    }
    (min, max)
}

pub(crate) fn volume_scale_vec(scale: &Option<dsl::DslValue>) -> [f32; 3] {
    match scale {
        Some(dsl::DslValue::Number(n)) => {
            let s = n.as_f64() as f32;
            [s, s, s]
        }
        Some(dsl::DslValue::Array(values)) if values.len() == 3 => {
            let read = |index: usize| values.get(index).and_then(|v| v.as_f64()).unwrap_or(1.0) as f32;
            [read(0), read(1), read(2)]
        }
        _ => [1.0, 1.0, 1.0],
    }
}

pub(crate) fn world_volumes_contain_aabb(volumes: &[WorldVolumeProps], min: Point3d, max: Point3d) -> bool {
    if volumes.is_empty() {
        return true;
    }
    let corners = [
        min,
        Point3d::new(max.x(), min.y(), min.z()),
        Point3d::new(min.x(), max.y(), min.z()),
        Point3d::new(max.x(), max.y(), min.z()),
        Point3d::new(min.x(), min.y(), max.z()),
        Point3d::new(max.x(), min.y(), max.z()),
        Point3d::new(min.x(), max.y(), max.z()),
        max,
    ];
    for volume in volumes {
        let scale = volume_scale_vec(&volume.scale);
        let world = pose_isometry(volume.origin, volume.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &None);
        let inv = world.inverse();
        let hx = 0.5 + 1e-3;
        let hy = 0.5 + 1e-3;
        let hz = 0.5 + 1e-3;
        let mut inside = true;
        for corner in corners {
            let relative = inv.transform_point(&corner);
            let local = Point3d::new(relative.x() / scale[0], relative.y() / scale[1], relative.z() / scale[2]);
            if local.x().abs() > hx || local.y().abs() > hy || local.z().abs() > hz {
                inside = false;
                break;
            }
        }
        if inside {
            return true;
        }
    }
    false
}

#[cfg(test)]
pub(crate) fn point_inside_body(body: &CollisionBody, world: &Pose3d, point: Point3d) -> bool {
    let local = world.inverse().transform_point(&point);
    for part in &body.parts {
        let part_local = part.local_pose.inverse().transform_point(&local);
        if part.shape.contains_point(&part.local_pose, &part_local) {
            return true;
        }
    }
    false
}

#[cfg(test)]
pub(crate) fn bodies_intersect(a: &CollisionBody, world_a: &Pose3d, b: &CollisionBody, world_b: &Pose3d) -> bool {
    let (amin, amax) = world_bounds(a, world_a);
    let (bmin, bmax) = world_bounds(b, world_b);
    if amax.x() < bmin.x() || bmax.x() < amin.x() || amax.y() < bmin.y() || bmax.y() < amin.y() || amax.z() < bmin.z() || bmax.z() < amin.z() {
        return false;
    }
    for part_a in &a.parts {
        let pose_a = world_a.semio_compose_rs(&part_a.local_pose);
        for part_b in &b.parts {
            let pose_b = world_b.semio_compose_rs(&part_b.local_pose);
            if shapes_intersect(&pose_a, &part_a.shape, &pose_b, &part_b.shape) {
                return true;
            }
        }
    }
    let center = Point3d::from_coords((amin.coords() + amax.coords() + bmin.coords() + bmax.coords()) * 0.25);
    point_inside_body(a, world_a, center) && point_inside_body(b, world_b, center)
}

//#region 🗺️BroadPhase
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub(crate) struct CollisionAabb {
    pub(crate) min: [f32; 3],
    pub(crate) max: [f32; 3],
}

impl CollisionAabb {
    pub(crate) fn from_body(body: &CollisionBody, world: &Pose3d) -> Self {
        let (min, max) = world_bounds(body, world);
        Self { min: [min.x(), min.y(), min.z()], max: [max.x(), max.y(), max.z()] }
    }

    pub(crate) fn intersects(&self, other: &Self) -> bool {
        self.max[0] >= other.min[0] && other.max[0] >= self.min[0] && self.max[1] >= other.min[1] && other.max[1] >= self.min[1] && self.max[2] >= other.min[2] && other.max[2] >= self.min[2]
    }
}

/// 🏙️ One spatial cell's member ids: document-wide ([`DOCUMENT_CELL_MEMBER_SLOTS`]), claimed a bookkeeping page at a time.
pub(crate) type CollisionCellMembers = FixedOwnerSet<String, DOCUMENT_CELL_MEMBER_SLOTS, FIXED_OWNER_SLOTS>;

#[derive(Debug)]
pub(crate) struct CollisionSpatialIndex {
    cell_size: f32,
    entries: FixedOwnerMap<String, CollisionAabb, DOCUMENT_OBJECT_SLOTS>,
    cells: FixedOwnerMap<(i32, i32, i32), CollisionCellMembers, DOCUMENT_CELL_SLOTS>,
    oversized: FixedOwnerSet<String, DOCUMENT_KIND_SLOTS>,
    retiring_key: Option<String>,
    retiring_bucket: Option<CollisionCellMembers>,
}

#[derive(Clone, Debug)]
pub(crate) enum CollisionIndexRejectedOwner {
    Capacity(String),
}

impl CollisionIndexRejectedOwner {
    pub(crate) fn retire_one(&mut self) -> bool {
        match self {
            Self::Capacity(id) if id.capacity() != 0 => {
                drop(std::mem::take(id));
                false
            }
            Self::Capacity(_) => true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct CollisionIndexOwner {
    pub(crate) operation: u64,
    pub(crate) generation: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CollisionCellSpan {
    min: [i32; 3],
    max: [i32; 3],
    count: u64,
}

impl CollisionCellSpan {
    fn new(cell_size: f32, bounds: CollisionAabb) -> Option<Self> {
        let cell = |value: f32| (value / cell_size).floor().clamp(i32::MIN as f32, i32::MAX as f32) as i32;
        let min = [cell(bounds.min[0]), cell(bounds.min[1]), cell(bounds.min[2])];
        let max = [cell(bounds.max[0]), cell(bounds.max[1]), cell(bounds.max[2])];
        let spans = [0, 1, 2].map(|axis| u64::try_from(i64::from(max[axis]) - i64::from(min[axis]) + 1).ok());
        let count = spans.into_iter().try_fold(1_u64, |count, span| count.checked_mul(span?))?;
        (count <= CollisionSpatialIndex::MAX_CELLS_PER_ENTRY).then_some(Self { min, max, count })
    }

    fn cell(self, cursor: u64) -> Option<(i32, i32, i32)> {
        if cursor >= self.count {
            return None;
        }
        let z_span = u64::try_from(i64::from(self.max[2]) - i64::from(self.min[2]) + 1).ok()?;
        let y_span = u64::try_from(i64::from(self.max[1]) - i64::from(self.min[1]) + 1).ok()?;
        let yz = y_span.checked_mul(z_span)?;
        let x = cursor / yz;
        let rest = cursor % yz;
        let y = rest / z_span;
        let z = rest % z_span;
        Some((i32::try_from(i64::from(self.min[0]) + i64::try_from(x).ok()?).ok()?, i32::try_from(i64::from(self.min[1]) + i64::try_from(y).ok()?).ok()?, i32::try_from(i64::from(self.min[2]) + i64::try_from(z).ok()?).ok()?))
    }

    fn contains(self, cell: (i32, i32, i32)) -> bool {
        cell.0 >= self.min[0] && cell.0 <= self.max[0] && cell.1 >= self.min[1] && cell.1 <= self.max[1] && cell.2 >= self.min[2] && cell.2 <= self.max[2]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CollisionMutationStage {
    PreflightNew,
    PreflightOld,
    Remove,
    Insert,
    Commit,
    Complete,
    Rejected,
}

pub(crate) struct CollisionIndexMutation {
    owner: CollisionIndexOwner,
    id: String,
    bounds: CollisionAabb,
    old_bounds: Option<CollisionAabb>,
    old_span: Option<CollisionCellSpan>,
    new_span: Option<CollisionCellSpan>,
    stage: CollisionMutationStage,
    cursor: u64,
    missing_cells: usize,
    reclaimed_cells: usize,
}

#[derive(Debug)]
pub(crate) enum CollisionMutationStep {
    Pending,
    Complete,
    Rejected(CollisionIndexRejectedOwner),
    Stale,
}

/// 🗑️ Resumable withdrawal of one indexed owner — the production counterpart of
/// [`CollisionIndexMutation`], driven one cell per step by the interactive brush lane's incremental
/// scene sync so a deleted object leaves the broad phase without a whole-index rebuild.
pub(crate) struct CollisionIndexRemoval {
    owner: CollisionIndexOwner,
    id: String,
    span: Option<CollisionCellSpan>,
    cursor: u64,
    complete: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CollisionQueryStage {
    Cells,
    Oversized,
    Entries,
    Complete,
}

pub(crate) struct CollisionQueryCursor {
    owner: CollisionIndexOwner,
    bounds: CollisionAabb,
    span: Option<CollisionCellSpan>,
    stage: CollisionQueryStage,
    cell_cursor: u64,
    member_cursor: usize,
    candidates: FixedOwnerSet<String, DOCUMENT_OBJECT_SLOTS>,
    truncated: bool,
    examined_cells: usize,
    examined_members: usize,
    retiring_key: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CollisionQueryStep {
    Pending,
    Complete,
    Stale,
}

impl CollisionQueryCursor {
    pub(crate) fn candidate(&self, index: usize) -> Option<&String> {
        self.candidates.iter().nth(index)
    }

    pub(crate) fn len(&self) -> usize {
        self.candidates.len()
    }

    #[cfg(test)]
    pub(crate) fn truncated(&self) -> bool {
        self.truncated
    }

    /// 📏️ Broad-phase work actually spent by this cursor — `(cells, members)`. The interactive brush
    /// lane publishes it so a document-scale scene can prove the query touched only the cells its own
    /// bounds span instead of every placed object.
    pub(crate) fn examined(&self) -> (usize, usize) {
        (self.examined_cells, self.examined_members)
    }

    pub(crate) fn retire_one_owner(&mut self) -> bool {
        if let Some(key) = self.retiring_key.as_mut() {
            if key.capacity() != 0 {
                drop(std::mem::take(key));
                return false;
            }
            self.retiring_key.take();
            return false;
        }
        if let Some(key) = self.candidates.pop_first() {
            self.retiring_key = Some(key);
            return false;
        }
        if self.candidates.retire_backing() {
            return false;
        }
        true
    }

    pub(crate) fn terminal_owners_empty(&self) -> bool {
        self.candidates.terminal_owners_empty() && self.retiring_key.is_none()
    }
}

impl CollisionIndexMutation {
    pub(crate) fn retire_one_owner(&mut self) -> bool {
        if self.id.capacity() != 0 {
            drop(std::mem::take(&mut self.id));
            return false;
        }
        true
    }
}

impl CollisionIndexRemoval {
    /// ♻️ One owner per close grant, exactly as the replacement cursor: the withdrawn owner id is the
    /// removal's only allocation.
    pub(crate) fn retire_one_owner(&mut self) -> bool {
        if self.id.capacity() != 0 {
            drop(std::mem::take(&mut self.id));
            return false;
        }
        true
    }
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CollisionIndexOwnerCensusStep {
    Pending { items: usize, bytes: usize },
    Complete,
    Rejected,
}

#[cfg(test)]
#[derive(Default)]
pub(crate) struct CollisionIndexOwnerCensusCursor {
    section: u8,
    index: usize,
    inner: usize,
}

#[cfg(test)]
fn collision_index_string_credit(value: &String) -> Option<(usize, usize)> {
    (value.capacity() <= 16 * 1024).then_some((usize::from(value.capacity() != 0), value.capacity()))
}

#[cfg(test)]
/// 📏️ Credit for a fixed page that may already have been handed back: a container whose backing was
/// retired owns nothing and costs nothing. Only a semantic owner over its declared byte cap is a
/// census refusal, so an index walked WHILE it retires (the close census does exactly that) must not
/// read a released page as "over capacity".
fn collision_index_backing_credit(credit: Option<(usize, usize)>) -> (usize, usize) {
    credit.unwrap_or((0, 0))
}

impl CollisionSpatialIndex {
    const MAX_CELLS_PER_ENTRY: u64 = 4_096;

    pub(crate) fn new(cell_size: f32) -> Self {
        assert!(cell_size.is_finite() && cell_size > 0.0);
        Self { cell_size, entries: FixedOwnerMap::new(), cells: FixedOwnerMap::new(), oversized: FixedOwnerSet::new(), retiring_key: None, retiring_bucket: None }
    }

    /// 📐️ World bounds already indexed for one owner id, or `None` when it was never admitted.
    pub(crate) fn entry_bounds(&self, id: &str) -> Option<&CollisionAabb> {
        self.entries.get(id)
    }

    /// 🔑️ Every indexed owner id, in the entries page's own sorted order — the incremental scene sync
    /// walks it one id per step to withdraw owners the new scene no longer carries.
    pub(crate) fn entry_ids(&self) -> impl Iterator<Item = &String> {
        self.entries.keys()
    }

    pub(crate) fn entry_len(&self) -> usize {
        self.entries.len()
    }

    pub(crate) fn begin_replacement(&self, owner: CollisionIndexOwner, id: String, bounds: CollisionAabb) -> CollisionIndexMutation {
        let old_bounds = self.entries.get(id.as_str()).copied();
        CollisionIndexMutation {
            owner,
            id,
            bounds,
            old_bounds,
            old_span: old_bounds.and_then(|value| CollisionCellSpan::new(self.cell_size, value)),
            new_span: CollisionCellSpan::new(self.cell_size, bounds),
            stage: CollisionMutationStage::PreflightNew,
            cursor: 0,
            missing_cells: 0,
            reclaimed_cells: 0,
        }
    }

    pub(crate) fn step_replacement(&mut self, mutation: &mut CollisionIndexMutation, current: CollisionIndexOwner) -> CollisionMutationStep {
        if mutation.owner != current {
            return CollisionMutationStep::Stale;
        }
        match mutation.stage {
            CollisionMutationStage::PreflightNew => {
                if self.entries.get(mutation.id.as_str()).is_none() && self.entries.len() == self.entries.capacity() {
                    return Self::reject_mutation(mutation);
                }
                if let Some(span) = mutation.new_span {
                    if let Some(cell) = span.cell(mutation.cursor) {
                        mutation.cursor += 1;
                        match self.cells.get(&cell) {
                            Some(bucket) if !bucket.contains(mutation.id.as_str()) && bucket.len() == bucket.capacity() => return Self::reject_mutation(mutation),
                            Some(_) => {}
                            None => mutation.missing_cells += 1,
                        }
                        return CollisionMutationStep::Pending;
                    }
                } else if !self.oversized.contains(mutation.id.as_str()) && self.oversized.len() == self.oversized.capacity() {
                    return Self::reject_mutation(mutation);
                }
                mutation.stage = CollisionMutationStage::PreflightOld;
                mutation.cursor = 0;
                CollisionMutationStep::Pending
            }
            CollisionMutationStage::PreflightOld => {
                if let Some(span) = mutation.old_span {
                    if let Some(cell) = span.cell(mutation.cursor) {
                        mutation.cursor += 1;
                        if mutation.new_span.is_none_or(|next| !next.contains(cell)) && self.cells.get(&cell).is_some_and(|bucket| bucket.len() == 1 && bucket.contains(mutation.id.as_str())) {
                            mutation.reclaimed_cells += 1;
                        }
                        return CollisionMutationStep::Pending;
                    }
                }
                if self.cells.len().checked_add(mutation.missing_cells).and_then(|total| total.checked_sub(mutation.reclaimed_cells)).is_none_or(|total| total > self.cells.capacity()) {
                    return Self::reject_mutation(mutation);
                }
                mutation.stage = CollisionMutationStage::Remove;
                mutation.cursor = 0;
                CollisionMutationStep::Pending
            }
            CollisionMutationStage::Remove => {
                if let Some(span) = mutation.old_span {
                    if let Some(cell) = span.cell(mutation.cursor) {
                        mutation.cursor += 1;
                        let empty = self.cells.get_mut(&cell).is_some_and(|bucket| {
                            drop(bucket.remove_entry(mutation.id.as_str()));
                            bucket.is_empty()
                        });
                        if empty {
                            drop(self.cells.remove_entry(&cell));
                        }
                        return CollisionMutationStep::Pending;
                    }
                } else if mutation.old_bounds.is_some() {
                    drop(self.oversized.remove_entry(mutation.id.as_str()));
                }
                mutation.stage = CollisionMutationStage::Insert;
                mutation.cursor = 0;
                CollisionMutationStep::Pending
            }
            CollisionMutationStage::Insert => {
                if let Some(span) = mutation.new_span {
                    if let Some(cell) = span.cell(mutation.cursor) {
                        mutation.cursor += 1;
                        if self.cells.get(&cell).is_none() {
                            let bucket = FixedOwnerSet::new();
                            // 🧊️ The two owners the preflight cannot weigh: a cell's member bucket does
                            // not exist yet, so its first sub-page is claimed HERE, and the cell map's
                            // own next sub-page is claimed by this very insert. A guest that refuses
                            // either hands back what it was given, and the mutation is refused rather
                            // than pushed into an owner that cannot hold it.
                            if bucket.capacity() == 0 {
                                return Self::reject_mutation(mutation);
                            }
                            match self.cells.try_insert(cell, bucket) {
                                Ok(FixedOwnerMapInsert::Inserted) => {}
                                Ok(FixedOwnerMapInsert::Occupied { .. }) => {}
                                Err((_, bucket)) => {
                                    drop(bucket);
                                    return Self::reject_mutation(mutation);
                                }
                            }
                        }
                        let Some(bucket) = self.cells.get_mut(&cell) else { return Self::reject_mutation(mutation) };
                        match bucket.try_insert(mutation.id.clone()) {
                            Ok(FixedOwnerSetInsert::Inserted) => {}
                            Ok(FixedOwnerSetInsert::Present { input }) => drop(input),
                            Err(input) => {
                                drop(input);
                                return Self::reject_mutation(mutation);
                            }
                        }
                        return CollisionMutationStep::Pending;
                    }
                } else {
                    match self.oversized.try_insert(mutation.id.clone()) {
                        Ok(FixedOwnerSetInsert::Inserted) => {}
                        Ok(FixedOwnerSetInsert::Present { input }) => drop(input),
                        Err(input) => {
                            drop(input);
                            return Self::reject_mutation(mutation);
                        }
                    }
                }
                mutation.stage = CollisionMutationStage::Commit;
                CollisionMutationStep::Pending
            }
            CollisionMutationStage::Commit => {
                drop(self.entries.remove_entry(mutation.id.as_str()));
                let id = std::mem::take(&mut mutation.id);
                match self.entries.try_insert(id, mutation.bounds) {
                    Ok(FixedOwnerMapInsert::Inserted) => {}
                    Ok(FixedOwnerMapInsert::Occupied { input_key, input_value: _ }) | Err((input_key, _)) => {
                        mutation.id = input_key;
                        return Self::reject_mutation(mutation);
                    }
                }
                mutation.stage = CollisionMutationStage::Complete;
                CollisionMutationStep::Complete
            }
            CollisionMutationStage::Complete => CollisionMutationStep::Complete,
            CollisionMutationStage::Rejected => CollisionMutationStep::Rejected(CollisionIndexRejectedOwner::Capacity(String::new())),
        }
    }

    fn reject_mutation(mutation: &mut CollisionIndexMutation) -> CollisionMutationStep {
        mutation.stage = CollisionMutationStage::Rejected;
        CollisionMutationStep::Rejected(CollisionIndexRejectedOwner::Capacity(std::mem::take(&mut mutation.id)))
    }

    pub(crate) fn begin_removal(&self, owner: CollisionIndexOwner, id: String) -> Option<CollisionIndexRemoval> {
        let bounds = *self.entries.get(id.as_str())?;
        Some(CollisionIndexRemoval { owner, id, span: CollisionCellSpan::new(self.cell_size, bounds), cursor: 0, complete: false })
    }

    pub(crate) fn step_removal(&mut self, removal: &mut CollisionIndexRemoval, current: CollisionIndexOwner) -> CollisionMutationStep {
        if removal.owner != current {
            return CollisionMutationStep::Stale;
        }
        if removal.complete {
            return CollisionMutationStep::Complete;
        }
        if let Some(span) = removal.span {
            if let Some(cell) = span.cell(removal.cursor) {
                removal.cursor += 1;
                let empty = self.cells.get_mut(&cell).is_some_and(|bucket| {
                    drop(bucket.remove_entry(removal.id.as_str()));
                    bucket.is_empty()
                });
                if empty {
                    drop(self.cells.remove_entry(&cell));
                }
                return CollisionMutationStep::Pending;
            }
        } else {
            drop(self.oversized.remove_entry(removal.id.as_str()));
        }
        drop(self.entries.remove_entry(removal.id.as_str()));
        removal.complete = true;
        CollisionMutationStep::Complete
    }

    pub(crate) fn begin_query(&self, owner: CollisionIndexOwner, bounds: CollisionAabb) -> CollisionQueryCursor {
        let span = CollisionCellSpan::new(self.cell_size, bounds);
        CollisionQueryCursor {
            owner,
            bounds,
            span,
            stage: if span.is_some() { CollisionQueryStage::Cells } else { CollisionQueryStage::Entries },
            cell_cursor: 0,
            member_cursor: 0,
            candidates: FixedOwnerSet::new(),
            truncated: false,
            examined_cells: 0,
            examined_members: 0,
            retiring_key: None,
        }
    }

    pub(crate) fn step_query(&self, query: &mut CollisionQueryCursor, current: CollisionIndexOwner) -> CollisionQueryStep {
        if query.owner != current {
            return CollisionQueryStep::Stale;
        }
        let candidate = match query.stage {
            CollisionQueryStage::Cells => {
                let span = query.span.expect("cell query span");
                let Some(cell) = span.cell(query.cell_cursor) else {
                    query.stage = CollisionQueryStage::Oversized;
                    query.member_cursor = 0;
                    return CollisionQueryStep::Pending;
                };
                let Some(bucket) = self.cells.get(&cell) else {
                    query.cell_cursor += 1;
                    query.examined_cells += 1;
                    return CollisionQueryStep::Pending;
                };
                match bucket.iter().nth(query.member_cursor) {
                    Some(id) => {
                        query.member_cursor += 1;
                        Some(id)
                    }
                    None => {
                        query.cell_cursor += 1;
                        query.member_cursor = 0;
                        query.examined_cells += 1;
                        return CollisionQueryStep::Pending;
                    }
                }
            }
            CollisionQueryStage::Oversized => match self.oversized.iter().nth(query.member_cursor) {
                Some(id) => {
                    query.member_cursor += 1;
                    Some(id)
                }
                None => {
                    query.stage = CollisionQueryStage::Complete;
                    return CollisionQueryStep::Complete;
                }
            },
            CollisionQueryStage::Entries => match self.entries.keys().nth(query.member_cursor) {
                Some(id) => {
                    query.member_cursor += 1;
                    Some(id)
                }
                None => {
                    query.stage = CollisionQueryStage::Complete;
                    return CollisionQueryStep::Complete;
                }
            },
            CollisionQueryStage::Complete => return CollisionQueryStep::Complete,
        };
        if let Some(id) = candidate {
            query.examined_members += 1;
            if self.entries.get(id.as_str()).is_some_and(|entry| entry.intersects(&query.bounds)) && !query.candidates.contains(id.as_str()) {
                match query.candidates.try_insert(id.clone()) {
                    Ok(FixedOwnerSetInsert::Inserted) => {}
                    Ok(FixedOwnerSetInsert::Present { input }) => drop(input),
                    Err(input) => {
                        drop(input);
                        query.truncated = true;
                    }
                }
            }
        }
        CollisionQueryStep::Pending
    }

    pub(crate) fn retire_one_owner(&mut self) -> bool {
        if let Some(key) = self.retiring_key.as_mut() {
            if key.capacity() != 0 {
                drop(std::mem::take(key));
                return false;
            }
            self.retiring_key.take();
            return false;
        }
        if let Some(bucket) = self.retiring_bucket.as_mut() {
            if let Some(id) = bucket.pop_first() {
                self.retiring_key = Some(id);
                return false;
            }
            if bucket.retire_backing() {
                return false;
            }
            self.retiring_bucket.take();
            return false;
        }
        if let Some((key, _)) = self.entries.pop_first() {
            self.retiring_key = Some(key);
            return false;
        }
        if let Some((_, bucket)) = self.cells.pop_first() {
            self.retiring_bucket = Some(bucket);
            return false;
        }
        if let Some(key) = self.oversized.pop_first() {
            self.retiring_key = Some(key);
            return false;
        }
        if self.entries.retire_backing() {
            return false;
        }
        if self.cells.retire_backing() {
            return false;
        }
        if self.oversized.retire_backing() {
            return false;
        }
        true
    }

    pub(crate) fn terminal_owners_empty(&self) -> bool {
        self.entries.terminal_owners_empty() && self.cells.terminal_owners_empty() && self.oversized.terminal_owners_empty() && self.retiring_key.is_none() && self.retiring_bucket.is_none()
    }

    #[cfg(test)]
    pub(crate) fn fixed_backing_witness_for_test(&self) -> [(usize, usize, usize); 3] {
        [
            (self.entries.backing_ptr().map_or(0, |pointer| pointer.cast::<()>() as usize), FixedOwnerMap::<String, CollisionAabb, DOCUMENT_OBJECT_SLOTS>::page_bytes(), self.entries.len()),
            (self.cells.backing_ptr().map_or(0, |pointer| pointer.cast::<()>() as usize), FixedOwnerMap::<(i32, i32, i32), CollisionCellMembers, DOCUMENT_CELL_SLOTS>::page_bytes(), self.cells.len()),
            (self.oversized.backing_ptr().map_or(0, |pointer| pointer.cast::<()>() as usize), FixedOwnerMap::<String, (), DOCUMENT_KIND_SLOTS>::page_bytes(), self.oversized.len()),
        ]
    }

    #[cfg(test)]
    pub(crate) fn census_one_owner(&self, cursor: &mut CollisionIndexOwnerCensusCursor) -> CollisionIndexOwnerCensusStep {
        let credit = match cursor.section {
            0 => Some(collision_index_backing_credit(self.entries.backing_credit())),
            1 => match self.entries.keys().nth(cursor.index) {
                Some(id) => collision_index_string_credit(id).and_then(|(items, bytes)| items.checked_add(1).map(|items| (items, bytes))),
                None => {
                    cursor.section += 1;
                    cursor.index = 0;
                    return CollisionIndexOwnerCensusStep::Pending { items: 0, bytes: 0 };
                }
            },
            2 => Some(collision_index_backing_credit(self.cells.backing_credit())),
            3 => match self.cells.values().nth(cursor.index) {
                Some(bucket) if cursor.inner == 0 => {
                    cursor.inner = 1;
                    { let (items, bytes) = collision_index_backing_credit(bucket.backing_credit()); items.checked_add(1).map(|items| (items, bytes)) }
                }
                Some(bucket) => match bucket.iter().nth(cursor.inner - 1) {
                    Some(id) => collision_index_string_credit(id),
                    None => {
                        cursor.index += 1;
                        cursor.inner = 0;
                        return CollisionIndexOwnerCensusStep::Pending { items: 0, bytes: 0 };
                    }
                },
                None => {
                    cursor.section += 1;
                    cursor.index = 0;
                    cursor.inner = 0;
                    return CollisionIndexOwnerCensusStep::Pending { items: 0, bytes: 0 };
                }
            },
            4 => Some(collision_index_backing_credit(self.oversized.backing_credit())),
            5 => match self.oversized.iter().nth(cursor.index) {
                Some(id) => collision_index_string_credit(id).and_then(|(items, bytes)| items.checked_add(1).map(|items| (items, bytes))),
                None => {
                    cursor.section += 1;
                    cursor.index = 0;
                    return CollisionIndexOwnerCensusStep::Pending { items: 0, bytes: 0 };
                }
            },
            6 => self.retiring_key.as_ref().map_or(Some((0, 0)), collision_index_string_credit),
            7 => {
                let Some(bucket) = &self.retiring_bucket else {
                    cursor.section += 1;
                    return CollisionIndexOwnerCensusStep::Pending { items: 0, bytes: 0 };
                };
                if cursor.inner == 0 {
                    cursor.inner = 1;
                    Some(collision_index_backing_credit(bucket.backing_credit()))
                } else {
                    match bucket.iter().nth(cursor.inner - 1) {
                        Some(id) => collision_index_string_credit(id),
                        None => {
                            cursor.section += 1;
                            Some((0, 0))
                        }
                    }
                }
            }
            _ => return CollisionIndexOwnerCensusStep::Complete,
        };
        let Some((items, bytes)) = credit else { return CollisionIndexOwnerCensusStep::Rejected };
        if matches!(cursor.section, 1 | 5) {
            cursor.index += 1;
        } else if matches!(cursor.section, 3 | 7) && cursor.inner != 0 {
            cursor.inner += 1;
        } else {
            cursor.section += 1;
        }
        CollisionIndexOwnerCensusStep::Pending { items, bytes }
    }

    #[cfg(test)]
    fn install_for_test(&mut self, id: &str, bounds: CollisionAabb) -> bool {
        let owner = CollisionIndexOwner { operation: 1, generation: 1 };
        let mut mutation = self.begin_replacement(owner, id.to_string(), bounds);
        for _ in 0..20_000 {
            match self.step_replacement(&mut mutation, owner) {
                CollisionMutationStep::Pending => {}
                CollisionMutationStep::Complete => return true,
                CollisionMutationStep::Rejected(_) | CollisionMutationStep::Stale => return false,
            }
        }
        false
    }

    #[cfg(test)]
    fn remove_for_test(&mut self, id: &str) -> bool {
        let owner = CollisionIndexOwner { operation: 1, generation: 1 };
        let Some(mut removal) = self.begin_removal(owner, id.to_string()) else {
            return false;
        };
        for _ in 0..20_000 {
            match self.step_removal(&mut removal, owner) {
                CollisionMutationStep::Pending => {}
                CollisionMutationStep::Complete => return true,
                CollisionMutationStep::Rejected(_) | CollisionMutationStep::Stale => return false,
            }
        }
        false
    }

    #[cfg(test)]
    fn candidates_for_test(&self, bounds: CollisionAabb) -> Vec<String> {
        let owner = CollisionIndexOwner { operation: 1, generation: 1 };
        let mut query = self.begin_query(owner, bounds);
        for _ in 0..20_000 {
            match self.step_query(&mut query, owner) {
                CollisionQueryStep::Pending => {}
                CollisionQueryStep::Complete => return query.candidates.iter().cloned().collect(),
                CollisionQueryStep::Stale => return Vec::new(),
            }
        }
        Vec::new()
    }
}
//#endregion 🗺️BroadPhase

//#region ⏳️OverlapStateMachine
pub(crate) trait CollisionStepContext {
    fn is_cancelled(&self) -> bool;
    fn should_yield(&self) -> bool;
    fn consume_fuel(&mut self, units: u64);
}

impl CollisionStepContext for semio_framework_job::StepContext<'_> {
    fn is_cancelled(&self) -> bool {
        semio_framework_job::StepContext::is_cancelled(self)
    }

    fn should_yield(&self) -> bool {
        semio_framework_job::StepContext::should_yield(self)
    }

    fn consume_fuel(&mut self, units: u64) {
        semio_framework_job::StepContext::consume_fuel(self, units);
    }
}

/// 🕳️ Where a [`CollisionPenetrationState`] stands. `A`/`B` name the two bodies; `Vertices*` probe one body's vertices
/// against the other solid, `Faces*` probe the parts of one body's triangles that pass behind the other body's faces.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
pub(crate) enum CollisionPenetrationStage {
    BroadPhase,
    VerticesA,
    VerticesB,
    FacesA,
    FacesB,
    InsetA,
    InsetB,
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum CollisionStepResult {
    Pending,
    Cancelled,
    /// `depth`: how far (m) one body's surface dives into the other solid, measured to that solid's nearest surface —
    /// `0` for bodies that only touch. `rejected_early` when the measure stopped at the first probe past the tolerance.
    Complete { depth: f64, rejected_early: bool },
}

/// 🕳️ Resumable PENETRATION-DEPTH test of one body pair: every step probes one vertex or one triangle, so a fill or brush
/// search interleaves it with its own fuel and cancellation.
///
/// The measure is the deepest point of one body's SURFACE inside the other SOLID, as its distance to that solid's surface.
/// Probes: the vertices of each body that lie inside the other, and the corners of every part of one body's triangle that
/// passes behind a triangle of the other inside that triangle's prism ([`collision::clip_behind_triangle`]) — the exact
/// extremes of a surface diving under a face. A probe only counts when it is farther than `tolerance` from the other
/// surface AND inside the other solid, which also keeps flush contact (the docking of two faces) at depth `0`. Surfaces
/// that COINCIDE (a duplicate at the same pose, a face lying inside another body's face) have no probe off the other
/// surface, so each triangle also probes its centroid moved inward by twice the tolerance: inside its own body, and for a
/// flush-docked neighbour outside the other one, but deep inside a coinciding solid.
///
/// 🐛️ It replaced a volume estimate (512 random points in the bounding-box intersection against an `m³` budget) that
/// accepted a slab 5.5 cm deep inside its neighbour and, because the docking host was skipped, a placement 1 m deep inside
/// its host (ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS, dev directive: only tiny collisions on the touching
/// surfaces).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub(crate) struct CollisionPenetrationState {
    pub(crate) stage: CollisionPenetrationStage,
    part_a: usize,
    part_b: usize,
    cursor: usize,
    tolerance: f64,
    depth: f64,
    rejected_early: bool,
}

impl CollisionPenetrationState {
    pub(crate) fn new(tolerance: f64) -> Self {
        Self { stage: CollisionPenetrationStage::BroadPhase, part_a: 0, part_b: 0, cursor: 0, tolerance, depth: 0.0, rejected_early: false }
    }

    #[cfg(test)]
    pub(crate) fn checkpoint(&self) -> Self {
        self.clone()
    }

    #[cfg(test)]
    pub(crate) fn resume(checkpoint: Self) -> Self {
        checkpoint
    }

    pub(crate) fn step<C: CollisionStepContext>(&mut self, context: &mut C, a: &CollisionBody, world_a: &Pose3d, b: &CollisionBody, world_b: &Pose3d) -> CollisionStepResult {
        if context.is_cancelled() {
            return CollisionStepResult::Cancelled;
        }
        if context.should_yield() {
            return CollisionStepResult::Pending;
        }
        let result = match self.stage {
            CollisionPenetrationStage::BroadPhase => {
                if CollisionAabb::from_body(a, world_a).intersects(&CollisionAabb::from_body(b, world_b)) {
                    self.advance(CollisionPenetrationStage::VerticesA)
                } else {
                    self.finish()
                }
            }
            CollisionPenetrationStage::VerticesA => self.step_vertex(a, world_a, b, world_b, CollisionPenetrationStage::VerticesB),
            CollisionPenetrationStage::VerticesB => self.step_vertex(b, world_b, a, world_a, CollisionPenetrationStage::FacesA),
            CollisionPenetrationStage::FacesA => self.step_face(a, world_a, b, world_b, CollisionPenetrationStage::FacesB),
            CollisionPenetrationStage::FacesB => self.step_face(b, world_b, a, world_a, CollisionPenetrationStage::InsetA),
            CollisionPenetrationStage::InsetA => self.step_inset(a, world_a, b, world_b, CollisionPenetrationStage::InsetB),
            CollisionPenetrationStage::InsetB => self.step_inset(b, world_b, a, world_a, CollisionPenetrationStage::Complete),
            CollisionPenetrationStage::Complete => CollisionStepResult::Complete { depth: self.depth, rejected_early: self.rejected_early },
        };
        context.consume_fuel(1);
        if context.is_cancelled() { CollisionStepResult::Cancelled } else { result }
    }

    fn advance(&mut self, stage: CollisionPenetrationStage) -> CollisionStepResult {
        self.stage = stage;
        self.part_a = 0;
        self.part_b = 0;
        self.cursor = 0;
        if stage == CollisionPenetrationStage::Complete {
            return self.finish();
        }
        CollisionStepResult::Pending
    }

    fn finish(&mut self) -> CollisionStepResult {
        self.stage = CollisionPenetrationStage::Complete;
        CollisionStepResult::Complete { depth: self.depth, rejected_early: self.rejected_early }
    }

    /// 📍️ Records `point` as a depth probe into `solid`'s part, keeping the deepest one; answers `true` once the depth is past
    /// the tolerance, which ends the measure (an infinite tolerance measures the full depth).
    fn probe(&mut self, point: rigid::Point3, solid_bounds: &CollisionAabb, solid_pose: rigid::Isometry3, solid: &CollisionShape) -> bool {
        // 📦️ A point outside the solid's world bounds cannot be inside it: the surface query and the containment test are only
        // paid for probes where the two bodies' bounds meet, and containment (BVH ray parity) only for a probe whose surface
        // distance would deepen the measure.
        let outside = |axis: usize, value: f32| value < solid_bounds.min[axis] || value > solid_bounds.max[axis];
        if outside(0, point.x) || outside(1, point.y) || outside(2, point.z) {
            return false;
        }
        let distance = f64::from(collision::distance_to_surface(solid_pose, &solid.shape, point));
        if distance <= COLLISION_CONTACT_NOISE_M || distance <= self.depth || !collision::contains_point_fast(solid_pose, &solid.shape, point) {
            return false;
        }
        self.depth = distance;
        self.rejected_early = self.depth > self.tolerance;
        self.rejected_early
    }

    /// 🧭️ The current `(probe part, solid part)` pair, advancing the part cursors past exhausted `count`s; `None` once every
    /// pair is done.
    fn parts<'a>(&mut self, probe: &'a CollisionBody, solid: &'a CollisionBody, count: impl Fn(&CollisionShape) -> usize) -> Option<(&'a CollisionMeshPart, &'a CollisionMeshPart)> {
        loop {
            let probe_part = probe.parts.get(self.part_a)?;
            let Some(solid_part) = solid.parts.get(self.part_b) else {
                self.part_a += 1;
                self.part_b = 0;
                self.cursor = 0;
                continue;
            };
            if self.cursor >= count(&probe_part.shape) {
                self.part_b += 1;
                self.cursor = 0;
                continue;
            }
            return Some((probe_part, solid_part));
        }
    }

    fn step_vertex(&mut self, probe: &CollisionBody, probe_world: &Pose3d, solid: &CollisionBody, solid_world: &Pose3d, next: CollisionPenetrationStage) -> CollisionStepResult {
        let Some((probe_part, solid_part)) = self.parts(probe, solid, |shape| shape.shape.vertex_count()) else { return self.advance(next) };
        let probe_pose = probe_world.semio_compose_rs(&probe_part.local_pose).0;
        let solid_pose = solid_world.semio_compose_rs(&solid_part.local_pose).0;
        let point = probe_part.shape.shape.world_vertex(probe_pose, self.cursor);
        self.cursor += 1;
        if self.probe(point, &CollisionAabb::from_body(solid, solid_world), solid_pose, &solid_part.shape) { self.finish() } else { CollisionStepResult::Pending }
    }

    fn step_inset(&mut self, probe: &CollisionBody, probe_world: &Pose3d, solid: &CollisionBody, solid_world: &Pose3d, next: CollisionPenetrationStage) -> CollisionStepResult {
        let Some((probe_part, solid_part)) = self.parts(probe, solid, |shape| shape.shape.triangle_count()) else { return self.advance(next) };
        let probe_pose = probe_world.semio_compose_rs(&probe_part.local_pose).0;
        let solid_pose = solid_world.semio_compose_rs(&solid_part.local_pose).0;
        let [a, b, c] = probe_part.shape.shape.world_triangle_outward(probe_pose, self.cursor);
        self.cursor += 1;
        let Some(outward) = (b - a).cross(c - a).try_normalize(1e-12) else { return CollisionStepResult::Pending };
        let centroid = rigid::Point3::new((a.x + b.x + c.x) / 3.0, (a.y + b.y + c.y) / 3.0, (a.z + b.z + c.z) / 3.0);
        let inset = centroid + outward * -(2.0 * self.tolerance.clamp(COLLISION_INSET_MIN_M, COLLISION_INSET_MAX_M) as f32);
        if self.probe(inset, &CollisionAabb::from_body(solid, solid_world), solid_pose, &solid_part.shape) { self.finish() } else { CollisionStepResult::Pending }
    }

    fn step_face(&mut self, probe: &CollisionBody, probe_world: &Pose3d, solid: &CollisionBody, solid_world: &Pose3d, next: CollisionPenetrationStage) -> CollisionStepResult {
        let Some((probe_part, solid_part)) = self.parts(probe, solid, |shape| shape.shape.triangle_count()) else { return self.advance(next) };
        let probe_pose = probe_world.semio_compose_rs(&probe_part.local_pose).0;
        let solid_pose = solid_world.semio_compose_rs(&solid_part.local_pose).0;
        let triangle = probe_part.shape.shape.world_triangle(probe_pose, self.cursor);
        self.cursor += 1;
        let bounds = CollisionAabb::from_body(solid, solid_world);
        let mut near = Vec::new();
        solid_part.shape.shape.triangles_near(solid_pose, triangle, self.tolerance as f32, &mut near);
        for index in near {
            let face = solid_part.shape.shape.world_triangle_outward(solid_pose, index as usize);
            let clipped = collision::clip_behind_triangle(triangle, face);
            // 📏️ A clipped point sits behind `face` inside its prism, so its distance to the solid's surface is at most its
            // distance to that face's plane: a polygon whose every corner is within the tolerance of the plane cannot probe
            // deeper, and skips the surface queries.
            let normal = (face[1] - face[0]).cross(face[2] - face[0]);
            let Some(unit) = normal.try_normalize(1e-12) else { continue };
            let deepest = clipped.points().iter().map(|point| f64::from(-unit.dot(*point - face[0]))).fold(0.0, f64::max);
            if deepest <= COLLISION_CONTACT_NOISE_M || deepest <= self.depth {
                continue;
            }
            // 📍️ Depth into a solid is the smallest distance over ALL its faces, which peaks inside the polygon (an edge cutting
            // into a face between two other faces reads 0 at both ends and its full depth at the middle), so the polygon's
            // edges and centroid are probed too.
            let points = clipped.points();
            let mut centroid = rigid::Vector3::new(0.0, 0.0, 0.0);
            for (index, point) in points.iter().enumerate() {
                centroid = centroid + point.coords();
                let to = points[(index + 1) % points.len()];
                for step in 0..COLLISION_EDGE_PROBES {
                    let along = *point + (to - *point) * (step as f32 / COLLISION_EDGE_PROBES as f32);
                    if self.probe(along, &bounds, solid_pose, &solid_part.shape) {
                        return self.finish();
                    }
                }
            }
            if self.probe(rigid::Point3::from_coords(centroid * (1.0 / points.len() as f32)), &bounds, solid_pose, &solid_part.shape) {
                return self.finish();
            }
        }
        CollisionStepResult::Pending
    }
}
//#endregion ⏳️OverlapStateMachine
//#endregion 🔖️Collision

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
