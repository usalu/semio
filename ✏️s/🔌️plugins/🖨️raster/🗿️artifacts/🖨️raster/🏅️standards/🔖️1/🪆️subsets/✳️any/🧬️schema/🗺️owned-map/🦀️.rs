//! 🗺️ The bounded page-backed owned map behind the raster assets and adjustment parameters.

pub const RASTER_OWNED_MAP_CAPACITY: usize = 64;

pub const RASTER_OWNED_MAP_PAGE_CAPACITY: usize = 8;

pub const RASTER_OWNED_MAP_PAGE_BACKING_BYTES: usize = 16 * 1024;

const RASTER_OWNED_MAP_PAGE_COUNT: usize = RASTER_OWNED_MAP_CAPACITY / RASTER_OWNED_MAP_PAGE_CAPACITY;

struct RasterOwnedMapPage<V> {
    entries: [Option<(String, V)>; RASTER_OWNED_MAP_PAGE_CAPACITY],
}

impl<V> RasterOwnedMapPage<V> {
    pub(super) fn new() -> Self {
        Self { entries: std::array::from_fn(|_| None) }
    }
}

pub struct RasterOwnedMapPageBacking<V> {
    page: Option<Box<RasterOwnedMapPage<V>>>,
}

impl<V> RasterOwnedMapPageBacking<V> {
    /// 🧮 The credit one owned page costs — the EXACT size of the boxed page, not the declared
    /// ceiling [`RASTER_OWNED_MAP_PAGE_BACKING_BYTES`] the admission guards compare against.
    /// `Box<RasterOwnedMapPage<V>>` allocates exactly `size_of::<RasterOwnedMapPage<V>>()`, so this is
    /// an exact figure and not an allocator-layout observation. Reporting the 16 KiB ceiling instead
    /// starved every close ladder whose per-turn byte grant is the framework's 4 KiB
    /// `ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES` — `artifact_app_laws::close_registered_fixture_app` and
    /// the runtime's own `ArtifactDocumentStoreDisposer` both grant exactly that, so a POPULATED
    /// raster document could never reach terminal-empty: the page release answered `Pending { 0, 0 }`
    /// forever (measured 2026-09-21, `renders_layers_tree` /
    /// `document_tree_binds_the_layers_interaction_domain`).
    pub fn conservative_credit_bytes(&self) -> usize {
        size_of::<RasterOwnedMapPage<V>>()
    }

    pub fn release(mut self) {
        drop(self.page.take());
    }
}

/// 🔒️ Fail-closed, but never a SECOND panic while an earlier one unwinds: a destructor that panics
/// during cleanup is a non-unwinding abort that kills the whole process (SIGABRT) and erases every
/// other test's result. Same `std::thread::panicking()` guard the framework's `OrderedMap`,
/// `ValueRetirement`, `Dictionary` and `MutationDag` destructors carry.
impl<V> Drop for RasterOwnedMapPageBacking<V> {
    fn drop(&mut self) {
        assert!(self.page.is_none() || std::thread::panicking(), "Raster owned map page backing reached Drop before exact release");
    }
}

#[derive(Debug)]
pub struct RasterOwnedMapRejected<V> {
    pub key: String,
    pub value: V,
    pub reason: &'static str,
}

pub struct RasterOwnedMapEntry<V> {
    pub(super) owner: std::mem::ManuallyDrop<Option<(String, V)>>,
}

impl<V> RasterOwnedMapEntry<V> {
    pub(super) fn new(key: String, value: V) -> Self {
        Self { owner: std::mem::ManuallyDrop::new(Some((key, value))) }
    }

    pub fn take(&mut self) -> (String, V) {
        self.owner.take().expect("Raster owned map entry remains available exactly once")
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.owner.is_none()
    }
}

impl<V> Drop for RasterOwnedMapEntry<V> {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "Raster owned map entry reached Drop before exact pair handback");
    }
}

pub enum RasterOwnedMapInsert<V> {
    Inserted,
    Replaced(RasterOwnedMapEntry<V>),
}

pub struct RasterOwnedMap<V> {
    pages: std::mem::ManuallyDrop<[Option<Box<RasterOwnedMapPage<V>>>; RASTER_OWNED_MAP_PAGE_COUNT]>,
    pub(super) order: [u8; RASTER_OWNED_MAP_CAPACITY],
    length: usize,
}

impl<V> RasterOwnedMap<V> {
    pub fn new() -> Self {
        Self { pages: std::mem::ManuallyDrop::new(std::array::from_fn(|_| None)), order: [0; RASTER_OWNED_MAP_CAPACITY], length: 0 }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn allocated_page_count(&self) -> usize {
        self.pages.iter().filter(|page| page.is_some()).count()
    }

    /// 🧮 The credit one page costs — see [`RasterOwnedMapPageBacking::conservative_credit_bytes`],
    /// whose figure this must equal for preflight and release to agree.
    pub fn conservative_page_credit_bytes() -> usize {
        size_of::<RasterOwnedMapPage<V>>()
    }

    pub(super) fn entry(&self, slot: usize) -> Option<&(String, V)> {
        self.pages.get(slot / RASTER_OWNED_MAP_PAGE_CAPACITY)?.as_ref()?.entries.get(slot % RASTER_OWNED_MAP_PAGE_CAPACITY)?.as_ref()
    }

    fn entry_mut(&mut self, slot: usize) -> Option<&mut (String, V)> {
        self.pages.get_mut(slot / RASTER_OWNED_MAP_PAGE_CAPACITY)?.as_mut()?.entries.get_mut(slot % RASTER_OWNED_MAP_PAGE_CAPACITY)?.as_mut()
    }

    fn ordered_slot(&self, index: usize) -> Option<usize> {
        (index < self.length).then(|| self.order[index] as usize)
    }

    pub fn entry_at(&self, index: usize) -> Option<(&String, &V)> {
        let slot = self.ordered_slot(index)?;
        let (key, value) = self.entry(slot)?;
        Some((key, value))
    }

    fn ordered_position(&self, key: &str) -> Result<usize, usize> {
        let mut left = 0;
        let mut right = self.length;
        while left < right {
            let middle = left + (right - left) / 2;
            let slot = self.order[middle] as usize;
            match self.entry(slot).expect("Raster owned map order addresses an occupied slot").0.as_str().cmp(key) {
                std::cmp::Ordering::Less => left = middle + 1,
                std::cmp::Ordering::Greater => right = middle,
                std::cmp::Ordering::Equal => return Ok(middle),
            }
        }
        Err(left)
    }

    fn free_slot(&self) -> Option<usize> {
        (0..RASTER_OWNED_MAP_CAPACITY).find(|slot| self.entry(*slot).is_none() && self.pages[*slot / RASTER_OWNED_MAP_PAGE_CAPACITY].is_some())
    }

    pub fn page_required_for_insert(&self, key: &str) -> bool {
        self.ordered_position(key).is_err() && self.free_slot().is_none() && self.length < RASTER_OWNED_MAP_CAPACITY
    }

    pub fn validate_additional_unique_entries(&self, count: usize) -> Result<(), &'static str> {
        if count > RASTER_OWNED_MAP_CAPACITY.saturating_sub(self.length) {
            return Err("raster-map.item-capacity");
        }
        if count > 0 && size_of::<RasterOwnedMapPage<V>>() > RASTER_OWNED_MAP_PAGE_BACKING_BYTES {
            return Err("raster-map.page-backing-capacity");
        }
        Ok(())
    }

    pub fn admit_one_page(&mut self) -> Result<(), &'static str> {
        if size_of::<RasterOwnedMapPage<V>>() > RASTER_OWNED_MAP_PAGE_BACKING_BYTES {
            return Err("raster-map.page-backing-capacity");
        }
        let page = self.pages.iter_mut().find(|page| page.is_none()).ok_or("raster-map.page-capacity")?;
        *page = Some(Box::new(RasterOwnedMapPage::new()));
        Ok(())
    }

    pub fn insert_pre_admitted(&mut self, key: String, value: V) -> Result<RasterOwnedMapInsert<V>, RasterOwnedMapRejected<V>> {
        match self.ordered_position(&key) {
            Ok(position) => {
                let slot = self.order[position] as usize;
                let entry = self.entry_mut(slot).expect("Raster owned map replacement slot remains occupied");
                let (previous_key, previous_value) = std::mem::replace(entry, (key, value));
                Ok(RasterOwnedMapInsert::Replaced(RasterOwnedMapEntry::new(previous_key, previous_value)))
            }
            Err(position) => {
                if self.length >= RASTER_OWNED_MAP_CAPACITY {
                    return Err(RasterOwnedMapRejected { key, value, reason: "raster-map.item-capacity" });
                }
                let Some(slot) = self.free_slot() else {
                    return Err(RasterOwnedMapRejected { key, value, reason: "raster-map.page-not-admitted" });
                };
                for index in (position..self.length).rev() {
                    self.order[index + 1] = self.order[index];
                }
                self.pages[slot / RASTER_OWNED_MAP_PAGE_CAPACITY].as_mut().expect("Raster owned map page is admitted").entries[slot % RASTER_OWNED_MAP_PAGE_CAPACITY] = Some((key, value));
                self.order[position] = slot as u8;
                self.length += 1;
                Ok(RasterOwnedMapInsert::Inserted)
            }
        }
    }

    pub fn insert(&mut self, key: String, value: V) -> Result<(), RasterOwnedMapRejected<V>> {
        if self.contains_key(&key) {
            return Err(RasterOwnedMapRejected { key, value, reason: "raster-map.duplicate-key" });
        }
        if self.page_required_for_insert(&key) && self.admit_one_page().is_err() {
            return Err(RasterOwnedMapRejected { key, value, reason: "raster-map.page-capacity" });
        }
        match self.insert_pre_admitted(key, value)? {
            RasterOwnedMapInsert::Inserted => Ok(()),
            RasterOwnedMapInsert::Replaced(_) => unreachable!("Raster unique insert cannot replace an occupied key"),
        }
    }

    pub fn get(&self, key: &str) -> Option<&V> {
        let position = self.ordered_position(key).ok()?;
        Some(&self.entry(self.order[position] as usize)?.1)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut V> {
        let position = self.ordered_position(key).ok()?;
        Some(&mut self.entry_mut(self.order[position] as usize)?.1)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.ordered_position(key).is_ok()
    }

    pub fn remove_entry(&mut self, key: &str) -> Option<RasterOwnedMapEntry<V>> {
        let position = self.ordered_position(key).ok()?;
        let slot = self.order[position] as usize;
        let entry = self.pages[slot / RASTER_OWNED_MAP_PAGE_CAPACITY].as_mut()?.entries[slot % RASTER_OWNED_MAP_PAGE_CAPACITY].take()?;
        for index in position + 1..self.length {
            self.order[index - 1] = self.order[index];
        }
        self.length -= 1;
        Some(RasterOwnedMapEntry::new(entry.0, entry.1))
    }

    pub fn take_last_entry(&mut self) -> Option<(String, V)> {
        let slot = self.ordered_slot(self.length.checked_sub(1)?)?;
        let entry = self.pages[slot / RASTER_OWNED_MAP_PAGE_CAPACITY].as_mut()?.entries[slot % RASTER_OWNED_MAP_PAGE_CAPACITY].take()?;
        self.length -= 1;
        Some(entry)
    }

    pub fn take_empty_page_backing(&mut self) -> Option<RasterOwnedMapPageBacking<V>> {
        let page = self.pages.iter_mut().find(|page| page.as_ref().is_some_and(|page| page.entries.iter().all(Option::is_none)))?;
        Some(RasterOwnedMapPageBacking { page: page.take() })
    }

    pub fn iter(&self) -> RasterOwnedMapIter<'_, V> {
        RasterOwnedMapIter { map: self, index: 0 }
    }

    pub fn keys(&self) -> RasterOwnedMapKeys<'_, V> {
        RasterOwnedMapKeys { inner: self.iter() }
    }

    /// 🧹️ Empties this map entry by entry and releases every page backing, leaving it in the only
    /// state its `Drop` guard admits. Every route that can abandon a half-built map (a codec whose
    /// next entry refuses, a layer forest that is not planted) closes it through here rather than
    /// hand-rolling the same two loops.
    pub fn retire(&mut self) {
        while self.take_last_entry().is_some() {}
        while let Some(page) = self.take_empty_page_backing() {
            page.release();
        }
    }
}

impl<V> Default for RasterOwnedMap<V> {
    fn default() -> Self {
        Self::new()
    }
}

/// 🔒️ Fail-closed on a bare drop, but silent while another panic unwinds — a second panic inside a
/// destructor during cleanup aborts the process instead of failing one test (that is exactly how one
/// red raster test used to take the whole binary down with SIGABRT). The pages are still handed back
/// to the allocator on the unwinding path so nothing leaks.
impl<V> Drop for RasterOwnedMap<V> {
    fn drop(&mut self) {
        assert!((self.length == 0 && self.pages.iter().all(Option::is_none)) || std::thread::panicking(), "Raster owned map reached Drop before every entry and page backing was explicitly retired");
        unsafe { std::mem::ManuallyDrop::drop(&mut self.pages) };
    }
}

/// 🧬️ A real deep copy: the map is a FIXED-capacity paged owner (64 entries over 8 pages), so
/// copying it is bounded by construction and needs no retained page authority. It used to answer an
/// EMPTY map for a populated source, which silently deleted every asset and every adjustment
/// parameter on any `RasterSnapshot::clone` — and `ArtifactStore` clones the open document.
impl<V: Clone> Clone for RasterOwnedMap<V> {
    fn clone(&self) -> Self {
        let mut copy = Self::new();
        for (key, value) in self.iter() {
            copy.insert(key.clone(), value.clone()).map_err(|rejected| rejected.reason).expect("a Raster owned map copy fits the capacity its source already fits");
        }
        copy
    }
}

impl<V: std::fmt::Debug> std::fmt::Debug for RasterOwnedMap<V> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_map().entries(self.iter()).finish()
    }
}

impl<V: PartialEq> PartialEq for RasterOwnedMap<V> {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<V: Eq> Eq for RasterOwnedMap<V> {}

pub struct RasterOwnedMapIter<'a, V> {
    map: &'a RasterOwnedMap<V>,
    pub(super) index: usize,
}

impl<'a, V> Iterator for RasterOwnedMapIter<'a, V> {
    type Item = (&'a String, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        let slot = self.map.ordered_slot(self.index)?;
        self.index += 1;
        let (key, value) = self.map.entry(slot)?;
        Some((key, value))
    }
}

pub struct RasterOwnedMapKeys<'a, V> {
    inner: RasterOwnedMapIter<'a, V>,
}

impl<'a, V> Iterator for RasterOwnedMapKeys<'a, V> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(key, _)| key)
    }
}

impl<'a, V> IntoIterator for &'a RasterOwnedMap<V> {
    type Item = (&'a String, &'a V);
    type IntoIter = RasterOwnedMapIter<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// 🗂️ The map's real DSL projection, key-ordered exactly as [`RasterOwnedMap::iter`] walks it.
impl<V: semio_framework_dsl_record::DslField> semio_framework_dsl_record::DslField for RasterOwnedMap<V> {
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Map(Box::new(V::shape()))
    }

    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::Map(self.iter().map(|(key, value)| (key.clone(), value.to_value())).collect())
    }

    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        let semio_framework_dsl_record::FieldValue::Map(entries) = value else { return Err(format!("expected Map, found {value:?}")) };
        let mut out = Self::new();
        for (key, item) in entries {
            let admitted = V::from_value(item).and_then(|item| out.insert(key.clone(), item).map_err(|rejected| rejected.reason.to_string()));
            if let Err(error) = admitted {
                out.retire();
                return Err(error);
            }
        }
        Ok(out)
    }
}

/// 🗂️ The same real projection on `dsl::ToValue`/`dsl::FromValue`, so a `RasterOwnedMap` field
/// carries its true contents inside a `#[derive(dsl::ToValue, dsl::FromValue)]` type.
///
/// 🩸️ Both directions used to refuse a populated map ("interactive production routes require the
/// retained page output authority") — a retained page authority this artifact never grew. Every
/// whole-document route therefore trapped or silently emptied the document: `ArtifactStore`'s own
/// `encode_pack` ("default-options pack encode is infallible"), the `.raster` DSL printer whose
/// committed demo carrier is itself populated, the `s.stdio.json` export/import serializers, and the
/// `apply_raster_mutation_json` oracle bridge whose committed before-documents carry both layers and
/// assets. The map is FIXED-capacity (64 entries over 8 pages of 16 KiB), so a whole-map projection
/// is bounded by construction and no paging authority is owed.
impl<V: semio_framework_value::ToValue> semio_framework_value::ToValue for RasterOwnedMap<V> {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::Object(self.iter().map(|(key, value)| (key.clone(), value.to_value())).collect())
    }
}

impl<V: semio_framework_value::FromValue> semio_framework_value::FromValue for RasterOwnedMap<V> {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(entries) = value else { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected an object for a Raster owned map")) };
        let mut out = Self::new();
        for (key, item) in entries {
            let admitted = V::from_value(item).and_then(|item| out.insert(key, item).map_err(|rejected| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, rejected.reason)));
            if let Err(error) = admitted {
                out.retire();
                return Err(error);
            }
        }
        Ok(out)
    }
}

/// ♻️ Retires one fixed-capacity map entry by entry, then each exact page allocation, through the framework's admitted cursor protocol.
struct RasterOwnedMapRetirement<V: semio_framework_value::retirement::RetireOwned> {
    map: std::mem::ManuallyDrop<RasterOwnedMap<V>>,
}

impl<V: semio_framework_value::retirement::RetireOwned> RasterOwnedMapRetirement<V> {
    fn last_entry(&self) -> Option<&(String, V)> {
        let slot = self.map.ordered_slot(self.map.length.checked_sub(1)?)?;
        self.map.entry(slot)
    }

    fn page_bytes(&self) -> usize {
        if self.map.allocated_page_count() == 0 { 0 } else { size_of::<RasterOwnedMapPage<V>>() }
    }
}

impl<V: semio_framework_value::retirement::RetireOwned> semio_framework_value::retirement::RetirementCursor for RasterOwnedMapRetirement<V> {
    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_value::retirement::RetirementStep {
        use semio_framework_value::retirement::{RetireOwned, RetirementStep};
        if grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return RetirementStep::BudgetExhausted;
        }
        if self.map.length > 0 {
            let entry = self.map.take_last_entry().expect("Raster owned map retains the entry it quoted");
            return RetirementStep::Child(entry.retirement());
        }
        let bytes = self.page_bytes();
        if bytes == 0 {
            return RetirementStep::Complete;
        }
        if bytes > grant.maximum_release_bytes {
            return RetirementStep::BudgetExhausted;
        }
        let page = self.map.take_empty_page_backing().expect("Raster owned map retains the page it quoted");
        page.release();
        RetirementStep::Bytes(bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        self.map.length == 0 && self.map.allocated_page_count() == 0
    }

    fn next_close_byte_demand(&self) -> Option<usize> {
        Some(if self.map.length == 0 { self.page_bytes() } else { 0 })
    }

    fn next_birth_bytes(&self, _maximum_bytes: usize) -> Option<usize> {
        if self.map.length == 0 {
            return Some(0);
        }
        semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(self.last_entry()?)
    }

    fn terminal_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}

impl<V: semio_framework_value::retirement::RetireOwned> Drop for RasterOwnedMapRetirement<V> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || semio_framework_value::retirement::RetirementCursor::terminal_is_empty(self), "Raster owned map retirement abandoned before terminal-empty");
        if semio_framework_value::retirement::RetirementCursor::terminal_is_empty(self) {
            unsafe { std::mem::ManuallyDrop::drop(&mut self.map) };
        }
    }
}

impl<V: semio_framework_value::retirement::RetireOwned> semio_framework_value::retirement::RetireOwned for RasterOwnedMap<V> {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        Box::new(RasterOwnedMapRetirement { map: std::mem::ManuallyDrop::new(self) })
    }

    fn retirement_birth_bytes(&self) -> Option<usize> {
        Some(size_of::<RasterOwnedMapRetirement<V>>())
    }

    fn controlled_retirement_supported() -> bool {
        true
    }
}

impl<V: semio_framework_pack_json::ArtifactCanonicalJsonTree> semio_framework_pack_json::ArtifactCanonicalJsonTree for RasterOwnedMap<V> {
    fn canonical_tree_node(&self) -> Result<semio_framework_pack_json::ArtifactCanonicalJsonNode<'_>, semio_framework_value::ValueError> {
        Ok(semio_framework_pack_json::ArtifactCanonicalJsonNode::Object(self.len()))
    }

    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn semio_framework_pack_json::ArtifactCanonicalJsonTree, semio_framework_value::ValueError> {
        self.entry_at(ordinal).map(|(_, value)| value as &dyn semio_framework_pack_json::ArtifactCanonicalJsonTree).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "canonical Raster map ordinal is absent"))
    }

    fn canonical_tree_key(&self, ordinal: usize) -> Result<semio_framework_pack_json::ArtifactCanonicalJsonText<'_>, semio_framework_value::ValueError> {
        self.entry_at(ordinal).map(|(key, _)| key.as_str().into()).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "canonical Raster map key is absent"))
    }
}
