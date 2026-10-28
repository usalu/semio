//! 🪆️ The composed children an app reads: the child content view, its pages, entries and their read retirement.
use semio_framework::Fault;
use semio_framework_artifact_reference::ArtifactRef;
use pack::snapshot::ArtifactPack;
use store::SpaceMember;
use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_value::RetirementDemand;
use semio_framework_value::ValueError;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_value::retained_clone::RetainedCloneStep;
use semio_framework_plugin_ownership::bounded_ownership::ARTIFACT_LIVE_OUTPUT_SLOTS;
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;

/// 🧸️ Read-only access to a composing document's live child stores, keyed `(slot,
/// child_id)` exactly as the parent's `ArtifactChild` handles name them.
///
/// This is the seam that replaces the `thread_local!`/session `HashMap<child_id, content>`
/// caches every composed plugin used to carry. Those caches went STALE the moment anything
/// moved a child's history without going through `ArtifactApp::handle` — store-level undo/redo
/// and checkout do exactly that. `new` clones every live `SpaceMember`'s already-owned snapshot
/// `Arc` and fixed-size revision ONCE, without encoding or graph cloning, so a view built fresh
/// immediately before each `handle`/`render` call (as every {@link VcsArtifactApp} call site
/// does) cannot present state older than the dispatch it belongs to.
///
/// 🚫️async: E1-adjacent by construction, not tagged — `M: SpaceMember` is generic on {@link
/// Self::new} itself, NOT on this struct. `ArtifactApp::render`/`handle`/… fix their `doc`
/// parameter's type at exactly `ArtifactView<'_, Self::Snapshot>` (no member-kind parameter),
/// so `ChildContentView` must stay a single, non-generic, dyn-free type usable from EVERY
/// plugin's fixed trait-method signature — the old `Box<dyn SpaceMember>` erasure existed
/// precisely to buy that uniformity, which is no longer available once `SpaceMember` is AFIT
/// (not object-safe). Pre-extracting each child's immutable snapshot handle at `new()` time (function-generic
/// over the caller's concrete `M`) reproduces the same uniformity without a trait object or a
/// fleet-wide `ArtifactApp::Members` associated-type rollout — see
/// `📓️terra-dedyn-fw-os-spacemember-report.md` §ChildContentView for the rejected alternatives.
/// `Default` (`EMPTY`) so a leaf app's view costs nothing and needs no ceremony.
pub const CHILD_CONTENT_SLOTS: usize = 1_024;

const CHILD_CONTENT_PAGE_SLOTS: usize = 32;

const CHILD_CONTENT_PAGES: usize = CHILD_CONTENT_SLOTS / CHILD_CONTENT_PAGE_SLOTS;

pub const CHILD_CONTENT_ID_BYTES: usize = 256;

/// 🪜️ The owner edge that identifies one composed member inside a document (design §21.9): the artifact id of the
/// MEMBER that owns it — empty when the document itself does — and its `(slot, child_id)` step under that owner. It is
/// the identity a member's `.spr` stamp and archive entry persist (`store::OwnerRef`), so members of any depth share
/// the flat registry and the flat content root without ever colliding on a step. Hosts, agents and history rows
/// address the same member by its [`MemberPath`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord,semio_framework_value::RetireOwned)]
pub struct MemberKey {
    pub owner: String,
    pub slot: String,
    pub child_id: String,
}

impl MemberKey {
    /// 🌰️ A member the document itself owns.
    pub fn root(slot: impl Into<String>, child_id: impl Into<String>) -> Self {
        Self { owner: String::new(), slot: slot.into(), child_id: child_id.into() }
    }

    /// 🤲️ The borrowed form every lookup takes.
    pub fn borrowed(&self) -> MemberKeyRef<'_> {
        MemberKeyRef { owner: &self.owner, slot: &self.slot, child_id: &self.child_id }
    }
}

/// 🗝️ A [`MemberKey`] borrowed for one lookup.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MemberKeyRef<'a> {
    pub owner: &'a str,
    pub slot: &'a str,
    pub child_id: &'a str,
}

impl<'a> MemberKeyRef<'a> {
    /// 🌾️ A member the document itself owns.
    pub fn root(slot: &'a str, child_id: &'a str) -> Self {
        Self { owner: "", slot, child_id }
    }

    /// 🧳️ The owned key.
    pub fn to_key(self) -> MemberKey {
        MemberKey { owner: self.owner.to_string(), slot: self.slot.to_string(), child_id: self.child_id.to_string() }
    }
}

pub struct ChildContentEntry {
    pub owner: String,
    pub slot: String,
    pub child_id: String,
    pub artifact_id: String,
    pub dialect: ArtifactDialect,
    pub revision: [u8; 32],
    pub snapshot: store::os_store::snapshot_read::ErasedSnapshotRead,
}

impl ChildContentEntry {
    pub fn key(&self) -> MemberKeyRef<'_> {
        MemberKeyRef { owner: &self.owner, slot: &self.slot, child_id: &self.child_id }
    }
}

pub struct PreparedChildContentIdentity {
    pub key: MemberKey,
    pub reference: ArtifactRef,
}

pub struct PreparedChildContentEntry {
    #[doc(hidden)]
    pub entry: ManuallyDrop<Option<ChildContentEntry>>,
    pub metadata: ManuallyDrop<Option<[String; 7]>>,
    #[doc(hidden)]
    pub metadata_cursor: usize,
    #[doc(hidden)]
    pub visibility: ManuallyDrop<Option<std::sync::Arc<semio_framework_os_kernel::os_vcs::ArtifactGroupVisibility>>>,
    pub generation: u64,
}

pub struct PreparedChildContentSlot {
    pub index: usize,
    identity: usize,
    visibility: std::sync::Arc<semio_framework_os_kernel::os_vcs::ArtifactGroupVisibility>,
    pub generation: u64,
    taken: bool,
}

impl PreparedChildContentEntry {
    /// 📏️ The original read and parent identity share one item with separate nesting.
    fn preparation_demands() -> RetirementDemand { let mut demand = store::os_store::snapshot_read::ErasedSnapshotRead::prepared_read_demands(); demand.depth += 1; demand }

    /// 🧳️ Moves admitted identity owners into an exact private candidate read.
    pub fn prepare<M: SpaceMember>(identity: &mut Option<PreparedChildContentIdentity>, member: &M, publication: &mut dyn store::os_store::one_item_publication::ErasedMemberStoreOneItemPublication, visibility: &std::sync::Arc<semio_framework_os_kernel::os_vcs::ArtifactGroupVisibility>, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<Option<(Self, RetainedCloneProgress)>, ValueError> {
        if !Self::admits(grant, Self::preparation_demands()) { return Ok(None); }
        let retained = identity.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "prepared child identity has already transferred"))?;
        ChildContentView::identity_hash(retained.key.borrowed())?;
        let child = store::os_store::one_item::ArtifactStoreOneItemGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
        let Some((generation, revision, snapshot, progress)) = member.snapshot_read_erased_for_publication(publication, visibility, child)? else { return Ok(None); };
        let PreparedChildContentIdentity { key, reference } = identity.take().expect("admitted child identity remains caller-owned until the exact read exists");
        Ok(Some((Self { entry: ManuallyDrop::new(Some(ChildContentEntry { owner: key.owner, slot: key.slot, child_id: key.child_id, artifact_id: reference.artifact_id, dialect: reference.dialect, revision, snapshot })), metadata: ManuallyDrop::new(None), metadata_cursor: 0, visibility: ManuallyDrop::new(Some(visibility.clone())), generation }, progress)))
    }

    pub fn next_close_byte_demand(&self) -> usize {
        self.retirement_demands().release_bytes
    }

    pub fn retirement_demands(&self) -> RetirementDemand {
        if self.entry.is_some() {
            let mut demand = store::os_store::snapshot_read::ErasedSnapshotRead::prepared_return_demands();
            demand.copy_bytes += Self::read_return_parent_copy_bytes(); demand.depth += 1; return demand;
        }
        if let Some(fields) = self.metadata.as_ref() {
            return match fields.get(self.metadata_cursor) {
                Some(field) => RetirementDemand { release_bytes: field.capacity(), depth: 1, ..Default::default() },
                None => RetirementDemand { depth: 1, ..Default::default() },
            };
        }
        self.visibility.as_ref().map_or(Default::default(), |_| RetirementDemand { depth: 1, ..Default::default() })
    }

    fn read_return_parent_copy_bytes() -> usize { 0 }

    fn admits(grant: store::os_store::one_item::ArtifactStoreOneItemGrant, demand: RetirementDemand) -> bool { grant.permits_one() && !(grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth) }

    /// 🪃️ Returns the lease to its private member before separately freeing each identity allocation.
    pub fn close_step<M: SpaceMember>(&mut self, member: &M, publication: &mut dyn store::os_store::one_item_publication::ErasedMemberStoreOneItemPublication, visibility_retainer: &std::sync::Arc<semio_framework_os_kernel::os_vcs::ArtifactGroupVisibility>, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<RetainedCloneStep, Fault> {
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if self.visibility.as_ref().is_none_or(|visibility| !std::sync::Arc::ptr_eq(visibility, visibility_retainer)) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let demand = self.retirement_demands();
        if !Self::admits(grant, demand) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(entry) = self.entry.take() {
            let ChildContentEntry { owner, slot, child_id, artifact_id, dialect, revision, snapshot } = entry;
            let parent_copy = Self::read_return_parent_copy_bytes();
            let child = store::os_store::one_item::ArtifactStoreOneItemGrant { maximum_copy_bytes: grant.maximum_copy_bytes - parent_copy, maximum_depth: grant.maximum_depth - 1, ..grant };
            match member.return_prepared_snapshot_read_erased(publication, self.visibility.as_ref().expect("original prepared visibility is retained through read return"), snapshot, child) {
                Ok(step) => {
                    *self.metadata = Some([owner, slot, child_id, artifact_id, dialect.artifact_kind, dialect.standard, dialect.subset]);
                    let mut progress = step.progress(); progress.copied_bytes += parent_copy;
                    let original = RetainedCloneGrant { maximum_items: grant.maximum_items, maximum_copy_bytes: grant.maximum_copy_bytes, maximum_capacity_bytes: grant.maximum_capacity_bytes, maximum_release_bytes: grant.maximum_release_bytes, maximum_depth: grant.maximum_depth };
                    let progress = semio_framework_value::retained_clone::admit_retained_clone_progress(original, progress, "original prepared child read return").map_err(|error| plugin_sdk_fault(error.into_message()))?;
                    return Ok(RetainedCloneStep::Progress(progress));
                }
                Err((error, original)) => {
                    *self.entry = Some(ChildContentEntry { owner, slot, child_id, artifact_id, dialect, revision, snapshot: original });
                    return Err(plugin_sdk_fault(error.to_string()));
                }
            }
        }
        self.close_metadata_step(visibility_retainer, grant)
    }

    pub fn close_metadata_step(&mut self, visibility_retainer: &std::sync::Arc<semio_framework_os_kernel::os_vcs::ArtifactGroupVisibility>, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<RetainedCloneStep, Fault> {
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if self.visibility.as_ref().is_none_or(|visibility| !std::sync::Arc::ptr_eq(visibility, visibility_retainer)) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let demand = self.retirement_demands();
        if !Self::admits(grant, demand) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(fields) = self.metadata.as_mut() {
            if let Some(field) = fields.get_mut(self.metadata_cursor) {
            drop(std::mem::take(field));
            self.metadata_cursor += 1;
            } else { self.metadata.take(); }
        } else {
            drop(self.visibility.take());
        }
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: demand.release_bytes, ..Default::default() };
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
    }

    pub fn terminal_is_empty(&self) -> bool { self.entry.is_none() && self.metadata.is_none() && self.visibility.is_none() }
}

impl Drop for PreparedChildContentEntry {
    fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "prepared child entry requires exact installation or bounded private return"); }
}

pub fn child_content_arc_bytes<T>() -> usize { size_of::<(std::sync::atomic::AtomicUsize, std::sync::atomic::AtomicUsize, T)>() }

#[derive(Clone)]
pub struct ChildContentPage {
    entries: [Option<std::sync::Arc<ChildContentEntry>>; CHILD_CONTENT_PAGE_SLOTS],
}

impl Default for ChildContentPage {
    fn default() -> Self {
        Self { entries: std::array::from_fn(|_| None) }
    }
}

#[derive(Clone)]
pub struct ChildContentRoot {
    pub pages: [Option<std::sync::Arc<ChildContentPage>>; CHILD_CONTENT_PAGES],
    pub len: usize,
}

impl Default for ChildContentRoot {
    fn default() -> Self {
        Self { pages: std::array::from_fn(|_| None), len: 0 }
    }
}

#[derive(Clone, Default)]
pub struct ChildContentView {
    pub root: Option<std::sync::Arc<ChildContentRoot>>,
}

impl ChildContentView {
    /// 🈳️ The view a document with no children gets.
    pub const EMPTY: Self = Self { root: None };

    pub fn prepared_entry_birth_bytes() -> usize { child_content_arc_bytes::<ChildContentRoot>() + child_content_arc_bytes::<ChildContentPage>() + child_content_arc_bytes::<ChildContentEntry>() }

    /// 🏗️ Builds the future immutable root before the common visibility decision.
    pub fn insert_prepared_entry(&self, prepared: &mut PreparedChildContentEntry, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<Option<(Self, PreparedChildContentSlot, RetainedCloneProgress)>, ValueError> {
        let demand = RetirementDemand { capacity_bytes: Self::prepared_entry_birth_bytes(), depth: 3, ..Default::default() };
        if !PreparedChildContentEntry::admits(grant, demand) { return Ok(None); }
        let reject = |reason| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, reason);
        if !prepared.visibility.as_ref().is_some_and(|visibility| visibility.pending()) { return Err(reject("prepared child insertion requires its pending visibility")); }
        let entry = prepared.entry.as_ref().ok_or_else(|| reject("prepared child entry has already transferred"))?;
        let start = Self::identity_hash(entry.key())?;
        let index = (0..CHILD_CONTENT_SLOTS).map(|offset| (start + offset) % CHILD_CONTENT_SLOTS).find(|index| self.root.as_ref().and_then(|root| Self::entry_at(root, *index)).is_none_or(|retained| retained.key() == entry.key())).ok_or_else(|| reject("fixed immutable child-content root is saturated"))?;
        let next = self.insert_entry_admitted(index, prepared.entry.take().unwrap());
        let identity = std::sync::Arc::as_ptr(Self::entry_at(next.root.as_deref().unwrap(), index).unwrap()) as usize;
        Ok(Some((next, PreparedChildContentSlot { index, identity, visibility: prepared.visibility.take().expect("original prepared visibility transfers into its exact slot"), generation: prepared.generation, taken: false }, RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: demand.capacity_bytes, ..Default::default() })))
    }

    pub fn prepared_entry_release_bytes() -> usize { child_content_arc_bytes::<ChildContentEntry>() }

    /// 🪝️ Extracts an unpublished entry under its whole Arc allocation grant, retaining its exact lease owner.
    pub fn take_prepared_entry(&mut self, slot: &mut PreparedChildContentSlot, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<Option<(PreparedChildContentEntry, RetainedCloneStep)>, Fault> {
        if slot.taken { return Ok(None); }
        if slot.visibility.committed() { return Err(plugin_sdk_fault("prepared child extraction cannot change a committed view")); }
        if !grant.permits_one() || grant.maximum_depth < 3 || grant.maximum_release_bytes < Self::prepared_entry_release_bytes() { return Ok(None); }
        let Some(root) = self.root.as_mut().and_then(std::sync::Arc::get_mut) else { return Ok(None); };
        let Some(page) = root.pages[slot.index / CHILD_CONTENT_PAGE_SLOTS].as_mut().and_then(std::sync::Arc::get_mut) else { return Ok(None); };
        let retained = page.entries[slot.index % CHILD_CONTENT_PAGE_SLOTS].as_ref().ok_or_else(|| plugin_sdk_fault("prepared child slot lost its exact entry"))?;
        if std::sync::Arc::as_ptr(retained) as usize != slot.identity { return Err(plugin_sdk_fault("prepared child slot belongs to a different exact entry")); }
        if std::sync::Arc::strong_count(retained) != 1 { return Ok(None); }
        let entry = std::sync::Arc::try_unwrap(page.entries[slot.index % CHILD_CONTENT_PAGE_SLOTS].take().unwrap()).unwrap_or_else(|_| unreachable!("exclusive prepared entry remains exclusive during its bounded move"));
        root.len -= 1;
        slot.taken = true;
        Ok(Some((PreparedChildContentEntry { entry: ManuallyDrop::new(Some(entry)), metadata: ManuallyDrop::new(None), metadata_cursor: 0, visibility: ManuallyDrop::new(Some(slot.visibility.clone())), generation: slot.generation }, RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: Self::prepared_entry_release_bytes(), ..Default::default() }))))
    }

    pub fn prepared_structure_close_byte_demand(&self) -> usize {
        let Some(root) = self.root.as_ref() else { return 0; };
        if std::sync::Arc::strong_count(root) != 1 { return 0; }
        match root.pages.iter().flatten().next() {
            Some(page) if std::sync::Arc::strong_count(page) == 1 && page.entries.iter().all(Option::is_none) => child_content_arc_bytes::<ChildContentPage>(),
            Some(_) => 0,
            None => child_content_arc_bytes::<ChildContentRoot>(),
        }
    }

    pub fn prepared_structure_close_depth_demand(&self) -> usize {
        let Some(root) = self.root.as_ref() else { return 0; };
        if std::sync::Arc::strong_count(root) != 1 { return 1; }
        match root.pages.iter().flatten().next() {
            Some(page) if std::sync::Arc::strong_count(page) == 1 && page.entries.iter().any(Option::is_some) => 3,
            Some(_) => 2,
            None => 1,
        }
    }

    /// 🧭️ Borrows the known view retaining the exact current root, page, or entry frontier.
    pub fn prepared_structure_retainer<'a>(&self, first: &'a Self, second: &'a Self) -> Option<&'a Self> {
        let Some(root) = self.root.as_ref() else { return Some(first); };
        if std::sync::Arc::strong_count(root) != 1 {
            return [first, second].into_iter().find(|view| view.root.as_ref().is_some_and(|retained| std::sync::Arc::ptr_eq(root, retained)));
        }
        let Some(index) = root.pages.iter().position(Option::is_some) else { return Some(first); };
        let page = root.pages[index].as_ref().unwrap();
        if std::sync::Arc::strong_count(page) != 1 {
            return [first, second].into_iter().find(|view| ChildContentOwners::view_retains_page(view, index, page));
        }
        let Some(entry) = page.entries.iter().flatten().next() else { return Some(first); };
        [first, second].into_iter().find(|view| view.contains_entry_owner(entry))
    }

    /// 🪵️ Frees one private view alias or empty scaffold while another exact view retains all remaining entries.
    pub fn close_prepared_structure_step(&mut self, retained: &Self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<RetainedCloneStep, Fault> {
        let yielded = RetainedCloneStep::Progress(RetainedCloneProgress::default());
        let handoff = RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() });
        if self.root.is_none() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if !grant.permits_one() || grant.maximum_depth < self.prepared_structure_close_depth_demand() || grant.maximum_release_bytes < self.prepared_structure_close_byte_demand() { return Ok(yielded); }
        if self.root.as_ref().zip(retained.root.as_ref()).is_some_and(|(root, retained)| std::sync::Arc::ptr_eq(root, retained)) {
            drop(self.root.take());
            return Ok(handoff);
        }
        let Some(root) = self.root.as_mut().and_then(std::sync::Arc::get_mut) else { return Ok(yielded); };
        if let Some(index) = root.pages.iter().position(Option::is_some) {
            let page = root.pages[index].as_mut().unwrap();
            if std::sync::Arc::strong_count(page) != 1 {
                if !ChildContentOwners::view_retains_page(retained, index, page) { return Ok(yielded); }
                root.len -= page.entries.iter().flatten().count();
                drop(root.pages[index].take());
                return Ok(handoff);
            }
            let page = std::sync::Arc::get_mut(page).unwrap();
            if let Some(entry_index) = page.entries.iter().position(Option::is_some) {
                if !retained.contains_entry_owner(page.entries[entry_index].as_ref().unwrap()) { return Ok(yielded); }
                drop(page.entries[entry_index].take());
                root.len -= 1;
                return Ok(handoff);
            }
            drop(root.pages[index].take());
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: child_content_arc_bytes::<ChildContentPage>(), ..Default::default() }));
        }
        assert_eq!(root.len, 0, "private child root closes only after every exact entry transferred");
        drop(self.root.take());
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes: child_content_arc_bytes::<ChildContentRoot>(), ..Default::default() }))
    }

    pub fn hash(key: MemberKeyRef<'_>) -> Result<usize, Fault> {
        Self::identity_hash(key).map_err(|error| plugin_sdk_fault(error.into_message()))
    }

    fn identity_hash(key: MemberKeyRef<'_>) -> Result<usize, ValueError> {
        if key.owner.len() > CHILD_CONTENT_ID_BYTES || key.slot.len() > CHILD_CONTENT_ID_BYTES || key.child_id.len() > CHILD_CONTENT_ID_BYTES {
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "child slot or id exceeds the fixed immutable-root identity bound"));
        }
        let mut hash = 0xcbf29ce484222325u64;
        let owner = key.owner.bytes().chain(std::iter::once(0xfe)).filter(|_| !key.owner.is_empty());
        for byte in owner.chain(key.slot.bytes()).chain(std::iter::once(0xff)).chain(key.child_id.bytes()) {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        Ok(hash as usize % CHILD_CONTENT_SLOTS)
    }

    fn entry_at(root: &ChildContentRoot, index: usize) -> Option<&std::sync::Arc<ChildContentEntry>> {
        root.pages[index / CHILD_CONTENT_PAGE_SLOTS].as_ref()?.entries[index % CHILD_CONTENT_PAGE_SLOTS].as_ref()
    }

    fn locate(root: &ChildContentRoot, key: MemberKeyRef<'_>) -> Result<Result<usize, usize>, Fault> {
        let start = Self::hash(key)?;
        for offset in 0..CHILD_CONTENT_SLOTS {
            let index = (start + offset) % CHILD_CONTENT_SLOTS;
            match Self::entry_at(root, index) {
                Some(entry) if entry.key() == key => return Ok(Ok(index)),
                Some(_) => {}
                None => return Ok(Err(index)),
            }
        }
        Err(plugin_sdk_fault("fixed immutable child-content root is saturated"))
    }

    pub fn admit_member(&self, key: MemberKeyRef<'_>) -> Result<usize, Fault> {
        match self.root.as_deref() {
            Some(root) => Self::locate(root, key).map(|location| match location {
                Ok(index) | Err(index) => index,
            }),
            None => Self::hash(key),
        }
    }

    pub fn insert_entry_admitted(&self, index: usize, entry: ChildContentEntry) -> Self {
        let root = self.root.as_deref().cloned().unwrap_or_default();
        let page_index = index / CHILD_CONTENT_PAGE_SLOTS;
        let entry_index = index % CHILD_CONTENT_PAGE_SLOTS;
        let mut next = root;
        let mut page = next.pages[page_index].as_deref().cloned().unwrap_or_default();
        let added = page.entries[entry_index].is_none();
        page.entries[entry_index] = Some(std::sync::Arc::new(entry));
        next.pages[page_index] = Some(std::sync::Arc::new(page));
        next.len += usize::from(added);
        Self { root: Some(std::sync::Arc::new(next)) }
    }

    fn find(&self, key: MemberKeyRef<'_>) -> Result<Option<&ChildContentEntry>, Fault> {
        let Some(root) = self.root.as_deref() else { return Ok(None) };
        match Self::locate(root, key)? {
            Ok(index) => Ok(Self::entry_at(root, index).map(std::sync::Arc::as_ref)),
            Err(_) => Ok(None),
        }
    }

    fn set_entry(root: &mut ChildContentRoot, index: usize, entry: Option<std::sync::Arc<ChildContentEntry>>) {
        let page_index = index / CHILD_CONTENT_PAGE_SLOTS;
        let mut page = root.pages[page_index].as_deref().cloned().unwrap_or_default();
        page.entries[index % CHILD_CONTENT_PAGE_SLOTS] = entry;
        root.pages[page_index] = page.entries.iter().any(Option::is_some).then(|| std::sync::Arc::new(page));
    }

    /// 🍂️ The same immutable root without one exact child: a copy-on-write removal that re-seats every later
    /// entry of that child's probe run (backward shift), so every other child stays addressable.
    pub fn without_member(&self, key: MemberKeyRef<'_>) -> Result<Self, Fault> {
        let Some(current) = self.root.as_deref() else { return Ok(self.clone()) };
        let Ok(mut hole) = Self::locate(current, key)? else { return Ok(self.clone()) };
        let mut next = current.clone();
        Self::set_entry(&mut next, hole, None);
        next.len -= 1;
        let mut cursor = (hole + 1) % CHILD_CONTENT_SLOTS;
        while let Some(entry) = Self::entry_at(&next, cursor).cloned() {
            let home = Self::hash(entry.key())?;
            if (cursor + CHILD_CONTENT_SLOTS - home) % CHILD_CONTENT_SLOTS >= (cursor + CHILD_CONTENT_SLOTS - hole) % CHILD_CONTENT_SLOTS {
                Self::set_entry(&mut next, hole, Some(entry));
                Self::set_entry(&mut next, cursor, None);
                hole = cursor;
            }
            cursor = (cursor + 1) % CHILD_CONTENT_SLOTS;
        }
        Ok(Self { root: (next.len != 0).then(|| std::sync::Arc::new(next)) })
    }

    /// 🪪 Stable identity of the immutable child-resolution root retained by one job request.
    pub fn identity_digest(&self) -> u64 {
        let Some(root) = self.root.as_deref() else { return 0xcbf2_9ce4_8422_2325 };
        let mut digest = 0xcbf2_9ce4_8422_2325_u64;
        for entry in root.pages.iter().flatten().flat_map(|page| page.entries.iter().flatten()) {
            for byte in entry
                .owner
                .bytes()
                .chain(std::iter::once(0xfb))
                .filter(|_| !entry.owner.is_empty())
                .chain(entry.slot.bytes())
                .chain(std::iter::once(0xff))
                .chain(entry.child_id.bytes())
                .chain(std::iter::once(0xfe))
                .chain(entry.dialect.artifact_kind.bytes())
                .chain(std::iter::once(0xfd))
                .chain(entry.dialect.standard.bytes())
                .chain(std::iter::once(0xfc))
                .chain(entry.dialect.subset.bytes())
                .chain(entry.revision)
            {
                digest = (digest ^ u64::from(byte)).wrapping_mul(0x1000_0000_01b3);
            }
        }
        digest
    }

    pub fn same_owner(&self,other:&Self)->bool{
        match(self.root.as_ref(),other.root.as_ref()){(Some(left),Some(right))=>std::sync::Arc::ptr_eq(left,right),(None,None)=>true,_=>false}
    }

    pub fn contains_entry_owner(&self, entry: &std::sync::Arc<ChildContentEntry>) -> bool {
        self.find(entry.key()).ok().flatten().is_some_and(|candidate| std::ptr::eq(candidate, entry.as_ref()))
    }

    pub fn next_take_copy_bytes(&self)->usize{
        let Some(root)=self.root.as_ref()else{return 0};
        if std::sync::Arc::strong_count(root)!=1{return size_of::<Option<std::sync::Arc<ChildContentRoot>>>()}
        let Some(page)=root.pages.iter().flatten().next()else{return size_of::<Option<std::sync::Arc<ChildContentRoot>>>()};
        if std::sync::Arc::strong_count(page)!=1{return size_of::<Option<std::sync::Arc<ChildContentPage>>>()+size_of::<usize>()}
        match page.entries.iter().flatten().next(){Some(entry)=>size_of::<Option<std::sync::Arc<ChildContentEntry>>>()+size_of::<usize>()+if std::sync::Arc::strong_count(entry)==1{size_of::<ChildContentEntry>()+size_of::<Option<ChildContentEntry>>()}else{0},None=>size_of::<Option<std::sync::Arc<ChildContentPage>>>()}
    }

    /// 📏️ Quotes the exact Arc allocation the next in-place disposal step frees.
    pub fn next_take_release_bytes(&self) -> usize {
        let Some(root) = self.root.as_ref() else { return 0 };
        if std::sync::Arc::strong_count(root) != 1 { return 0; }
        let Some(page) = root.pages.iter().flatten().next() else { return child_content_arc_bytes::<ChildContentRoot>() };
        if std::sync::Arc::strong_count(page) != 1 { return 0; }
        match page.entries.iter().flatten().next() {
            Some(entry) if std::sync::Arc::strong_count(entry) == 1 => child_content_arc_bytes::<ChildContentEntry>(),
            Some(_) => 0,
            None => child_content_arc_bytes::<ChildContentPage>(),
        }
    }

    pub fn take_one(&mut self, current: &ChildContentView, owners: &dyn ChildContentRetainers) -> ChildContentTake {
        if self.root.is_none() { return ChildContentTake::Complete(0); }
        if self.same_owner(current)||owners.retains_root(self){drop(self.root.take());return ChildContentTake::Complete(0)}
        let Some(root) = self.root.as_mut().and_then(std::sync::Arc::get_mut) else { return ChildContentTake::Blocked };
        let Some(page_index) = root.pages.iter().position(Option::is_some) else {
            drop(self.root.take());
            return ChildContentTake::Complete(child_content_arc_bytes::<ChildContentRoot>());
        };
        if std::sync::Arc::get_mut(root.pages[page_index].as_mut().unwrap()).is_none() {
            let shared = root.pages[page_index].as_ref().unwrap();
            if !(ChildContentOwners::view_retains_page(current, page_index, shared) || owners.retains_page(page_index, shared)) { return ChildContentTake::Blocked; }
            root.len = root.len.saturating_sub(shared.entries.iter().flatten().count());
            drop(root.pages[page_index].take());
            return ChildContentTake::Released(0);
        }
        let page = std::sync::Arc::get_mut(root.pages[page_index].as_mut().unwrap()).unwrap();
        let Some(entry_index) = page.entries.iter().position(Option::is_some) else {
            drop(root.pages[page_index].take());
            return ChildContentTake::Released(child_content_arc_bytes::<ChildContentPage>());
        };
        let entry = page.entries[entry_index].take().unwrap();
        match std::sync::Arc::try_unwrap(entry) {
            Ok(entry) => {
                root.len = root.len.saturating_sub(1);
                ChildContentTake::Snapshot(entry, child_content_arc_bytes::<ChildContentEntry>())
            }
            Err(shared) => {
                if current.contains_entry_owner(&shared) || owners.retains_entry(&shared) {
                    drop(shared);
                    root.len = root.len.saturating_sub(1);
                    ChildContentTake::Released(0)
                } else {
                    page.entries[entry_index] = Some(shared);
                    ChildContentTake::Blocked
                }
            }
        }
    }

    /// 🪞️ The same immutable root with one member's `snapshot` read in its place — a history edit's preview of a composed
    /// member (design §12), never the published root.
    pub fn with_member_read(&self, key: MemberKeyRef<'_>, dialect: &ArtifactDialect, revision: [u8; 32], snapshot: store::os_store::snapshot_read::ErasedSnapshotRead) -> Result<Self, Fault> {
        let artifact_id = self.find(key)?.map(|entry| entry.artifact_id.clone()).ok_or_else(|| plugin_sdk_fault(format!("no live child store for slot {} child {}", key.slot, key.child_id)))?;
        let index = self.admit_member(key)?;
        Ok(self.insert_entry_admitted(index, ChildContentEntry { owner: key.owner.to_string(), slot: key.slot.to_string(), child_id: key.child_id.to_string(), artifact_id, dialect: dialect.clone(), revision, snapshot }))
    }

    pub async fn with_member<M: SpaceMember>(&self, key: MemberKeyRef<'_>, reference: &ArtifactRef, member: &M) -> Result<Self, Fault> {
        let index = self.admit_member(key)?;
        self.capture_member_admitted(index, key, reference, member).await
    }

    pub async fn capture_member_admitted<M: SpaceMember>(&self, index: usize, key: MemberKeyRef<'_>, reference: &ArtifactRef, member: &M) -> Result<Self, Fault> {
        let revision = member.content_revision().await;
        let snapshot = member.snapshot_read_erased().await.map_err(|error| plugin_sdk_fault(error.to_string()))?;
        Ok(self.insert_entry_admitted(index, ChildContentEntry { owner: key.owner.to_string(), slot: key.slot.to_string(), child_id: key.child_id.to_string(), artifact_id: reference.artifact_id.clone(), dialect: reference.dialect.clone(), revision, snapshot }))
    }

    /// 🧬️ Fixed-size, restart-stable identity paired with the captured child snapshot.
    pub fn revision(&self, slot: &str, child_id: &str) -> Result<[u8; 32], Fault> {
        self.find(MemberKeyRef::root(slot, child_id))?.map(|entry| entry.revision).ok_or_else(|| plugin_sdk_fault(format!("no live child store for slot {slot} child {child_id}")))
    }

    /// 🧵️ Selects an immutable child snapshot capability without exposing concrete ownership.
    pub fn typed_read<S: ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + Send + Sync + 'static>(&self, slot: &str, child_id: &str) -> Result<store::os_store::snapshot_read::SnapshotReadRef<'_, S>, Fault> {
        self.typed_read_at(MemberKeyRef::root(slot, child_id))
    }

    /// 🪡️ [`Self::typed_read`] of the member at one exact owner edge, whatever its depth.
    pub fn typed_read_at<S: ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + Send + Sync + 'static>(&self, key: MemberKeyRef<'_>) -> Result<store::os_store::snapshot_read::SnapshotReadRef<'_, S>, Fault> {
        let (slot, child_id) = (key.slot, key.child_id);
        let snapshot = &self.find(key)?.ok_or_else(|| plugin_sdk_fault(format!("no live child store for slot {slot} child {child_id}")))?.snapshot;
        snapshot.typed::<S>().ok_or_else(|| plugin_sdk_fault(format!("child snapshot type mismatch for slot {slot} child {child_id}")))
    }

    /// 🎯️ The dialect a child materializes as, for a caller that must route by kind.
    pub fn dialect(&self, slot: &str, child_id: &str) -> Option<ArtifactDialect> {
        self.dialect_at(MemberKeyRef::root(slot, child_id))
    }

    /// 🪪️ Borrows every address coordinate from the same published entry whose snapshot is read.
    pub fn reference_fields(&self, slot: &str, child_id: &str) -> Option<semio_framework_schema_composition::ChildRefFields<'_>> {
        self.reference_fields_at(MemberKeyRef::root(slot, child_id))
    }

    /// 🧷️ Borrows the selected published child identity at one exact owner edge.
    pub fn reference_fields_at(&self, key: MemberKeyRef<'_>) -> Option<semio_framework_schema_composition::ChildRefFields<'_>> {
        let entry = self.find(key).ok().flatten()?;
        Some(semio_framework_schema_composition::ChildRefFields { child_id: &entry.child_id, artifact_id: &entry.artifact_id, artifact_kind: &entry.dialect.artifact_kind, standard: &entry.dialect.standard, subset: &entry.dialect.subset })
    }

    /// 🎳️ [`Self::dialect`] of the member at one exact owner edge, whatever its depth.
    pub fn dialect_at(&self, key: MemberKeyRef<'_>) -> Option<ArtifactDialect> {
        self.find(key).ok().flatten().map(|entry| entry.dialect.clone())
    }

    /// 📋️ Every `(slot, child_id)` the document itself owns and holds live.
    pub fn slots(&self) -> Vec<(String, String)> {
        let Some(root) = self.root.as_deref() else { return Vec::new() };
        let mut slots = Vec::with_capacity(root.len);
        for index in 0..CHILD_CONTENT_SLOTS {
            if let Some(entry) = Self::entry_at(root, index).filter(|entry| entry.owner.is_empty()) {
                slots.push((entry.slot.clone(), entry.child_id.clone()));
            }
        }
        slots
    }

    /// 🗂️ The owner edge of every member held live, whatever its depth.
    pub fn keys(&self) -> Vec<MemberKey> {
        let Some(root) = self.root.as_deref() else { return Vec::new() };
        (0..CHILD_CONTENT_SLOTS).filter_map(|index| Self::entry_at(root, index)).map(|entry| entry.key().to_key()).collect()
    }

    /// 🈳️ Whether this document has any live children at all.
    pub fn is_empty(&self) -> bool {
        self.root.as_ref().is_none_or(|root| root.len == 0)
    }
}

pub enum ChildContentTake {
    Blocked,
    Released(usize),
    Snapshot(ChildContentEntry, usize),
    Complete(usize),
}

/// 👑️ Every OTHER bounded owner of a retained child-content page during one disposal step: the
/// pending sibling retirements of the same registry. The live root is passed separately. A page or
/// entry alias held by an owner is handed over — the owner that ends up exclusive retires it — while
/// an alias held by a mere borrower (an operation view a job still holds) blocks, because a borrower
/// drops without a bounded disposer. Copy-on-write publication makes two consecutive retirements
/// share every untouched page, so without this set both wait on each other forever.
pub trait ChildContentRetainers {
    fn retains_root(&self, view: &ChildContentView) -> bool;
    fn retains_page(&self, page_index: usize, page: &std::sync::Arc<ChildContentPage>) -> bool;
    fn retains_entry(&self, entry: &std::sync::Arc<ChildContentEntry>) -> bool;
}

pub struct ChildContentOwners {
    views: [ChildContentView; ARTIFACT_LIVE_OUTPUT_SLOTS],
    pub len: usize,
}

impl ChildContentRetainers for ChildContentOwners {
    fn retains_root(&self, view: &ChildContentView) -> bool { self.views[..self.len].iter().any(|retainer| retainer.same_owner(view)) }
    fn retains_page(&self, page_index: usize, page: &std::sync::Arc<ChildContentPage>) -> bool { ChildContentOwners::retains_page(self, page_index, page) }
    fn retains_entry(&self, entry: &std::sync::Arc<ChildContentEntry>) -> bool { ChildContentOwners::retains_entry(self, entry) }
}

impl ChildContentOwners {
    /// 🈳️ The owner set of a retirement with no live sibling.
    pub fn none() -> Self {
        Self { views: std::array::from_fn(|_| ChildContentView::EMPTY), len: 0 }
    }

    fn push(&mut self, view: ChildContentView) {
        assert!(self.len < ARTIFACT_LIVE_OUTPUT_SLOTS, "child-content owner set exceeds the fixed retirement registry");
        self.views[self.len] = view;
        self.len += 1;
    }

    pub fn view_retains_page(view: &ChildContentView, page_index: usize, page: &std::sync::Arc<ChildContentPage>) -> bool {
        view.root.as_deref().and_then(|root| root.pages[page_index].as_ref()).is_some_and(|candidate| std::sync::Arc::ptr_eq(candidate, page))
    }

    fn retains_page(&self, page_index: usize, page: &std::sync::Arc<ChildContentPage>) -> bool {
        self.views[..self.len].iter().any(|view| Self::view_retains_page(view, page_index, page))
    }

    fn retains_entry(&self, entry: &std::sync::Arc<ChildContentEntry>) -> bool {
        self.views[..self.len].iter().any(|view| view.contains_entry_owner(entry))
    }
}

use std::mem::ManuallyDrop;
#[path = "🛬️lifted/👁️read/🦀️.rs"]
mod lifted_read;
pub use lifted_read::*;

#[path="🎯️editor/🦀️.rs"]
pub mod editor;

#[path="📖️vector/🦀️.rs"]pub mod vector_read;
