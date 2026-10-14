//! 🪆️ Child members and child content: member keys and paths, the member registry, owned document ingress and child content views.
use crate::child_content::mounted_child_group_receipt::mounted_nested_retirement_demand;
use crate::child_content::mounted_child_group_receipt::MOUNTED_RECEIPT_MAXIMUM_BYTES;
use crate::child_content::mounted_child_group_receipt::MountedReceiptBytes;
use crate::child_content::mounted_child_group_receipt_group::command::entry::MountedCommandEntrySource;
use crate::child_content::mounted_child_group_receipt_group::command::entry::MountedCommandEntry;
use crate::child_content::mounted_child_group_receipt_group::command::CommandAppend;
pub(crate) use crate::child_content::mounted_child_group_receipt_group::command::PagedCommandLog;
#[cfg(test)] use crate::child_view::PreparedChildContentEntry;
#[cfg(test)] use crate::child_view::child_content_arc_bytes;
#[cfg(test)] use crate::child_view::ChildContentRoot;
use crate::child_view::PreparedChildContentIdentity;
use crate::child_view::CHILD_CONTENT_SLOTS;
use crate::child_view::MemberKey;
use crate::child_view::MemberKeyRef;
use crate::child_view::ChildContentEntry;
use crate::child_view::ChildContentPage;
use crate::child_view::ChildContentView;
use crate::child_view::ChildContentTake;
use crate::child_view::ChildContentRetainers;
use crate::child_view::ChildContentOwners;
use super::child_emit_preparation::OwnedChildEmit;
use semio_framework_value::DslValue;
use semio_framework::{kernel::{ActorId, ArtifactDiff, ArtifactHandle, ArtifactVersion, EditRef, HybridLogicalTimestamp, InverseMutation, InvocationId, KernelMutation, MutationId, SchemaId, UndoGroup, UndoPolicy}, ActionKind, Fault, FaultCode, FaultFrom, FaultOrigin};
/// 🚪️ `io`'s `ArtifactRef`/`ArtifactKindId` vocabulary is not glob-re-exported at the
/// `semio-framework-os-kernel` crate root (deliberate — see that crate's own glue.rs comment on
/// the `io` mount), so it is named through the `store::io::` path everywhere in this file,
/// exactly like the sibling `🎞️gif` migration leaf (`semio_framework_artifact_reference::ArtifactDialect`) already does.
use semio_framework_artifact_reference::ArtifactRef;
use store::{ MemberFactory, SpaceMember};
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_ui_locale::Terminology;
/// 🏅️🪆️🎯️ Standards/subsets dialect vocabulary (ticket 26/08/10/STDIO-ARTIFACTS-AND-IO phase
/// 2). Defined in `semio_framework` so plugins and the OS product
/// share one definition without an inverted dependency; re-exported here verbatim.
use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_value::{RetirementDemand, ValueError, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use semio_framework_value::retirement::controlled::ControlledRetirement;
#[cfg(test)]
use crate::plugin_app::PluginApp;
#[cfg(any(test, feature = "artifact-app-testing"))]
use crate::typed_operation::PendingPublicationOutcome;
#[cfg(any(test, feature = "artifact-app-testing"))]
use crate::app_contract::ArtifactApp;
#[cfg(any(test, feature = "artifact-app-testing"))]
use crate::app_emission::ActionMeta;
#[cfg(any(test, feature = "artifact-app-testing"))]
use crate::app_emission::Emit;
#[cfg(any(test, feature = "artifact-app-testing"))]
use crate::app_emission::EphemeralEmit;
#[cfg(any(test, feature = "artifact-app-testing"))]
use crate::artifact_instance::VcsArtifactApp;
use crate::bounded_ownership::ARTIFACT_LIVE_OUTPUT_SLOTS;
use crate::bounded_ownership::ArtifactFixedRegistry;
#[cfg(any(test, feature = "artifact-app-testing"))]
use crate::bounded_ownership::PluginLifecycleStep;
use crate::history_view::CommandLogEntry;
use crate::history_view::ShellHistoryReplay;
use crate::sdk_fault::plugin_sdk_fault;
#[cfg(any(test, feature = "artifact-app-testing"))]
use crate::store_publication::MountedTypedCommandFullOperationStage;
#[cfg(any(test, feature = "artifact-app-testing"))]
use crate::tool_cancellation::ArtifactDocumentAuthority;
#[cfg(any(test, feature = "artifact-app-testing"))]
use crate::tool_cancellation::ToolOperationKey;
#[cfg(any(test, feature = "artifact-app-testing"))]
use crate::tool_contract::ArtifactToolCompletionValue;
#[cfg(any(test, feature = "artifact-app-testing"))]
use crate::tool_jobs::INTERACTIVE_TURN_WORKER_WALL_US;
#[cfg(any(test, feature = "artifact-app-testing"))]
use crate::tool_contract::TypedOperationResultLane;
#[cfg(any(test, feature = "artifact-app-testing"))]
use crate::typed_operation::MountedTypedCommandFullOperation;
#[cfg(test)]
use crate::bounded_ownership::DOCUMENT_ARCHIVE_LOAD_SLOTS;
#[cfg(test)]
use crate::command_history::CommandLogAppend;
#[cfg(test)]
use crate::history_view::InverseAction;
#[cfg(test)]
use crate::document_ingress::ActiveDocumentArchiveLoad;
#[cfg(test)]
use crate::document_ingress::CompositionPinsRetirement;
#[cfg(test)]
use crate::document_ingress::advance_original_document_archive_close;
#[cfg(test)]
use crate::document_ingress::original_document_archive_close_demands;
#[cfg(test)]
use crate::store_publication::MountedWorkerFaultCapture;
#[cfg(test)]
use crate::store_publication::advance_original_fault_capture;
#[cfg(test)]
use crate::store_publication::original_fault_capture_demands;
#[cfg(test)]
use crate::tool_jobs::time_travel;

pub(crate) const OWNED_DOCUMENT_MEMBER_PREPARATION_BYTES: usize = store::MEMBER_OPEN_IDENTITY_BYTES * 8;

/// 👣️ One step of a [`MemberPath`]: the slot a member is declared in and its child id there.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord,semio_framework_value::RetireOwned)]
pub struct MemberStep {
    pub slot: String,
    pub child_id: String,
}

/// 🛤️ The owner path of one composed member — every step from the document down to it — as hosts, agents and history
/// rows address it (the `store` of a history row, a tool run's member, an inference dependency): `slot/childId` per
/// step, steps joined by `/`, `%25` for `%` and `%2F` for `/` inside a component. A member of the document whose id
/// holds neither reads exactly `<slot>/<childId>`.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord,semio_framework_value::RetireOwned)]
pub struct MemberPath(Vec<MemberStep>);

impl MemberPath {
    pub(crate) fn take_original(&mut self)->Self{std::mem::replace(self,Self(Vec::new()))}

    /// 🌲️ The path of a member the document itself owns.
    pub fn root(slot: impl Into<String>, child_id: impl Into<String>) -> Self {
        Self(vec![MemberStep { slot: slot.into(), child_id: child_id.into() }])
    }

    /// 🪵️ The path of the member this one owns at `(slot, child_id)`.
    pub fn join(mut self, slot: impl Into<String>, child_id: impl Into<String>) -> Self {
        self.0.push(MemberStep { slot: slot.into(), child_id: child_id.into() });
        self
    }

    /// 🧗️ The text of the owner's path — every step but the member's own; empty for a member of the document.
    pub fn owner_text(&self) -> String {
        Self(self.0[..self.0.len().saturating_sub(1)].to_vec()).to_string()
    }

    /// 🪢️ The path of the member at `(slot, child_id)` under the owner whose path text is `owner` (empty: the document).
    pub fn under(owner: &str, slot: impl Into<String>, child_id: impl Into<String>) -> Option<Self> {
        let owner = if owner.is_empty() { Self(Vec::new()) } else { Self::parse(owner)? };
        Some(owner.join(slot, child_id))
    }

    /// 🥾️ Every step from the document down to the member.
    pub fn steps(&self) -> &[MemberStep] {
        &self.0
    }

    /// 📜️ Reads the canonical text; `None` for anything [`std::fmt::Display`] never writes.
    pub fn parse(text: &str) -> Option<Self> {
        let mut components = Vec::new();
        for component in text.split('/') {
            components.push(Self::unescape(component)?);
        }
        if components.len() % 2 != 0 {
            return None;
        }
        let mut steps = Vec::with_capacity(components.len() / 2);
        let mut components = components.into_iter();
        while let (Some(slot), Some(child_id)) = (components.next(), components.next()) {
            steps.push(MemberStep { slot, child_id });
        }
        Some(Self(steps))
    }

    fn unescape(component: &str) -> Option<String> {
        if component.is_empty() {
            return None;
        }
        let mut text = String::with_capacity(component.len());
        let mut rest = component;
        while let Some(position) = rest.find('%') {
            text.push_str(&rest[..position]);
            match rest.get(position..position + 3)? {
                "%25" => text.push('%'),
                "%2F" => text.push('/'),
                _ => return None,
            }
            rest = &rest[position + 3..];
        }
        text.push_str(rest);
        Some(text)
    }
}

impl std::fmt::Display for MemberPath {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, component) in self.0.iter().flat_map(|step| [step.slot.as_str(), step.child_id.as_str()]).enumerate() {
            if index != 0 {
                formatter.write_str("/")?;
            }
            for character in component.chars() {
                match character {
                    '%' => formatter.write_str("%25")?,
                    '/' => formatter.write_str("%2F")?,
                    other => std::fmt::Write::write_char(formatter, other)?,
                }
            }
        }
        Ok(())
    }
}

pub(crate) struct ChildMemberEntry<M> {
    pub(crate) reference: ArtifactRef,
    pub(crate) owner: store::OwnerRef,
    pub(crate) member: M,
}

pub(crate) struct ChildMemberAdmission {
    index: usize,
    generation: u64,
    nested: bool,
}

pub struct ChildMemberRegistrationError<M> {
    pub member: Option<M>,
    pub fault: Fault,
}

impl<M> std::fmt::Debug for ChildMemberRegistrationError<M> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("ChildMemberRegistrationError").field("owner_returned", &self.member.is_some()).field("fault", &self.fault).finish()
    }
}

pub(crate) struct ChildMemberRegistry<M> {
    pub(crate) slots: Box<[std::mem::MaybeUninit<ChildMemberEntry<M>>]>,
    occupied: [u64; CHILD_CONTENT_SLOTS / 64],
    reserved: [u64; CHILD_CONTENT_SLOTS / 64],
    nested: [u64; CHILD_CONTENT_SLOTS / 64],
    generations: [u64; CHILD_CONTENT_SLOTS],
    pub(crate) dense_slots: [usize; CHILD_CONTENT_SLOTS],
    ordinal_by_slot: [usize; CHILD_CONTENT_SLOTS],
    next_generation: u64,
    len: usize,
    pub(crate) allocation_admitted: bool,
}

impl<M> ChildMemberRegistry<M> {
    pub(crate) fn new() -> Self {
        let mut slots = Vec::new();
        let allocation_admitted = slots.try_reserve_exact(CHILD_CONTENT_SLOTS).is_ok();
        if allocation_admitted {
            slots.resize_with(CHILD_CONTENT_SLOTS, std::mem::MaybeUninit::uninit);
        }
        Self {
            slots: slots.into_boxed_slice(),
            occupied: [0; CHILD_CONTENT_SLOTS / 64],
            reserved: [0; CHILD_CONTENT_SLOTS / 64],
            nested: [0; CHILD_CONTENT_SLOTS / 64],
            generations: [0; CHILD_CONTENT_SLOTS],
            dense_slots: [usize::MAX; CHILD_CONTENT_SLOTS],
            ordinal_by_slot: [usize::MAX; CHILD_CONTENT_SLOTS],
            next_generation: 0,
            len: 0,
            allocation_admitted,
        }
    }

    fn hash(key: MemberKeyRef<'_>) -> Result<usize, Fault> {
        ChildContentView::hash(key)
    }

    fn occupied(&self, index: usize) -> bool {
        self.occupied[index / 64] & (1 << (index % 64)) != 0
    }

    fn reserved(&self, index: usize) -> bool {
        self.reserved[index / 64] & (1 << (index % 64)) != 0
    }

    fn entry(&self, index: usize) -> Option<&ChildMemberEntry<M>> {
        self.occupied(index).then(|| unsafe { self.slots[index].assume_init_ref() })
    }

    fn entry_mut(&mut self, index: usize) -> Option<&mut ChildMemberEntry<M>> {
        if !self.occupied(index) {
            return None;
        }
        Some(unsafe { self.slots[index].assume_init_mut() })
    }

    fn nested(&self, index: usize) -> bool {
        self.nested[index / 64] & (1 << (index % 64)) != 0
    }

    /// 🧷️ The owner edge of the member at one occupied slot.
    fn key_at(&self, index: usize) -> Option<MemberKeyRef<'_>> {
        let entry = self.entry(index)?;
        Some(MemberKeyRef { owner: if self.nested(index) { entry.owner.parent.artifact_id.as_str() } else { "" }, slot: &entry.owner.slot, child_id: &entry.owner.child_id })
    }

    fn locate(&self, key: MemberKeyRef<'_>) -> Result<Result<usize, usize>, Fault> {
        if !self.allocation_admitted {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-member-allocation"), "fixed child-member registry allocation was not pre-admitted"));
        }
        let start = Self::hash(key)?;
        for offset in 0..CHILD_CONTENT_SLOTS {
            let index = (start + offset) % CHILD_CONTENT_SLOTS;
            match self.key_at(index) {
                Some(candidate) if candidate == key => return Ok(Ok(index)),
                Some(_) => {}
                None if self.reserved(index) => {}
                None => return Ok(Err(index)),
            }
        }
        Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-member-capacity"), "fixed child-member registry is saturated"))
    }

    pub(crate) fn admit_identity(&mut self, key: MemberKeyRef<'_>) -> Result<ChildMemberAdmission, Fault> {
        if !self.allocation_admitted {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-member-allocation"), "fixed child-member registry allocation was not pre-admitted"));
        }
        match self.locate(key)? {
            Ok(_) => Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-member-duplicate"), "fixed child-member identity is already occupied")),
            Err(index) => {
                let generation = self.next_generation.checked_add(1).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-member-generation"), "fixed child-member admission generation exhausted"))?;
                self.next_generation = generation;
                self.generations[index] = generation;
                self.reserved[index / 64] |= 1 << (index % 64);
                Ok(ChildMemberAdmission { index, generation, nested: !key.owner.is_empty() })
            }
        }
    }

    pub(crate) fn admit(&mut self, key: &(String, String)) -> Result<ChildMemberAdmission, Fault> {
        self.admit_identity(MemberKeyRef::root(&key.0, &key.1))
    }

    #[expect(clippy::needless_pass_by_value, reason = "This non-Copy token conveys one-use admission authority and must be consumed by the commit, even though its identity fields are scalar.")]
    pub(crate) fn insert_admitted(&mut self, admission: ChildMemberAdmission, reference: ArtifactRef, owner: store::OwnerRef, member: M) {
        assert!(self.reserved(admission.index) && self.generations[admission.index] == admission.generation && !self.occupied(admission.index), "exact child-member admission changed while its exclusive app owner was suspended");
        self.reserved[admission.index / 64] &= !(1 << (admission.index % 64));
        self.slots[admission.index].write(ChildMemberEntry { reference, owner, member });
        self.occupied[admission.index / 64] |= 1 << (admission.index % 64);
        if admission.nested {
            self.nested[admission.index / 64] |= 1 << (admission.index % 64);
        }
        self.dense_slots[self.len] = admission.index;
        self.ordinal_by_slot[admission.index] = self.len;
        self.len += 1;
    }

    /// 🌻️ The member the document itself owns at one `(slot, child_id)` step.
    pub(crate) fn get(&self, key: &(String, String)) -> Option<&ChildMemberEntry<M>> {
        self.member(MemberKeyRef::root(&key.0, &key.1))
    }

    pub(crate) fn get_mut(&mut self, key: &(String, String)) -> Option<&mut ChildMemberEntry<M>> {
        self.member_mut(MemberKeyRef::root(&key.0, &key.1))
    }

    /// 🪆️ The member at one exact owner edge, whatever its depth.
    pub(crate) fn member(&self, key: MemberKeyRef<'_>) -> Option<&ChildMemberEntry<M>> {
        let Ok(Ok(index)) = self.locate(key) else { return None };
        self.entry(index)
    }

    pub(crate) fn member_mut(&mut self, key: MemberKeyRef<'_>) -> Option<&mut ChildMemberEntry<M>> {
        let Ok(Ok(index)) = self.locate(key) else { return None };
        self.entry_mut(index)
    }

    /// 🧾️ Every live member with its owner edge, in admission order.
    pub(crate) fn keyed_entries(&self) -> impl Iterator<Item = (MemberKeyRef<'_>, &ChildMemberEntry<M>)> {
        (0..self.len).filter_map(|ordinal| self.keyed_entry_by_ordinal(ordinal))
    }

    pub(crate) fn keyed_entry_by_ordinal(&self, ordinal: usize) -> Option<(MemberKeyRef<'_>, &ChildMemberEntry<M>)> {
        let index = *self.dense_slots[..self.len].get(ordinal)?;
        Some((self.key_at(index)?, self.entry(index)?))
    }

    /// 🧭️ The member an owner path names: one lookup per step, each under the member the step before resolved.
    pub(crate) fn resolve(&self, path: &MemberPath) -> Option<(MemberKey, &ChildMemberEntry<M>)> {
        let mut owner = "";
        let mut resolved = None;
        for step in path.steps() {
            let entry = self.member(MemberKeyRef { owner, slot: &step.slot, child_id: &step.child_id })?;
            resolved = Some((MemberKey { owner: owner.to_string(), slot: step.slot.clone(), child_id: step.child_id.clone() }, entry));
            owner = entry.reference.artifact_id.as_str();
        }
        resolved
    }

    /// 🗺️ The owner path of the member at `key`: its own step under the steps of every member that owns it; `None`
    /// while a member on the way up is not live.
    pub(crate) fn path_of(&self, key: MemberKeyRef<'_>) -> Option<MemberPath> {
        let mut steps = vec![MemberStep { slot: key.slot.to_string(), child_id: key.child_id.to_string() }];
        let mut owner = key.owner;
        while !owner.is_empty() {
            if steps.len() > self.len {
                return None;
            }
            let (above, _) = self.keyed_entries().find(|(_, entry)| entry.reference.artifact_id == owner)?;
            steps.push(MemberStep { slot: above.slot.to_string(), child_id: above.child_id.to_string() });
            owner = above.owner;
        }
        steps.reverse();
        Some(MemberPath(steps))
    }

    /// 🏷️ Every live member with the text of its owner path — the store id the history wire names it by.
    pub(crate) fn addressed_entries(&self) -> impl Iterator<Item = (String, &ChildMemberEntry<M>)> {
        self.keyed_entries().filter_map(|(key, entry)| Some((self.path_of(key)?.to_string(), entry)))
    }

    pub(crate) fn entries(&self) -> impl Iterator<Item = &ChildMemberEntry<M>> {
        self.dense_slots[..self.len].iter().map(|index| self.entry(*index).expect("dense child-member slot remains occupied"))
    }

    pub(crate) fn entries_physical(&self) -> impl Iterator<Item = &ChildMemberEntry<M>> {
        let occupied = self.occupied;
        self.slots.iter().enumerate().filter(move |(index, _)| occupied[*index / 64] & (1 << (*index % 64)) != 0).map(|(_, slot)| unsafe { slot.assume_init_ref() })
    }

    pub(crate) fn entries_mut_physical(&mut self) -> impl Iterator<Item = &mut ChildMemberEntry<M>> {
        let occupied = self.occupied;
        self.slots.iter_mut().enumerate().filter_map(move |(index, slot)| (occupied[index / 64] & (1 << (index % 64)) != 0).then(|| unsafe { slot.assume_init_mut() }))
    }

    fn entry_by_ordinal(&self, ordinal: usize) -> Option<&ChildMemberEntry<M>> {
        (ordinal < self.len).then(|| self.entry(self.dense_slots[ordinal]).expect("dense child-member slot remains occupied"))
    }

    pub(crate) fn take_at(&mut self, index: usize) -> Option<ChildMemberEntry<M>> {
        if !self.occupied(index) {
            return None;
        }
        self.occupied[index / 64] &= !(1 << (index % 64));
        self.nested[index / 64] &= !(1 << (index % 64));
        self.len -= 1;
        let ordinal = self.ordinal_by_slot[index];
        let moved = self.dense_slots[self.len];
        self.dense_slots[ordinal] = moved;
        self.ordinal_by_slot[moved] = ordinal;
        self.dense_slots[self.len] = usize::MAX;
        self.ordinal_by_slot[index] = usize::MAX;
        Some(unsafe { self.slots[index].assume_init_read() })
    }

    pub(crate) fn cancel_admission(&mut self, admission: &ChildMemberAdmission) -> bool {
        if !self.reserved(admission.index) || self.generations[admission.index] != admission.generation {
            return false;
        }
        self.reserved[admission.index / 64] &= !(1 << (admission.index % 64));
        true
    }

    pub(crate) fn has_admission_in_flight(&self) -> bool {
        self.reserved.iter().any(|word| *word != 0)
    }

    /// 🍂️ Removes one exact member while no admission is in flight and re-seats every later entry of its probe
    /// run (backward shift), so every other member stays addressable by [`Self::locate`].
    pub(crate) fn remove(&mut self, key: MemberKeyRef<'_>) -> Result<Option<ChildMemberEntry<M>>, Fault> {
        if self.has_admission_in_flight() {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-member-removal-admission"), "a child member cannot leave the fixed registry while an admission is in flight"));
        }
        let Ok(mut hole) = self.locate(key)? else { return Ok(None) };
        let entry = self.take_at(hole).expect("located child member remains occupied");
        let mut cursor = (hole + 1) % CHILD_CONTENT_SLOTS;
        while let Some(moved) = self.key_at(cursor) {
            let home = Self::hash(moved)?;
            if (cursor + CHILD_CONTENT_SLOTS - home) % CHILD_CONTENT_SLOTS >= (cursor + CHILD_CONTENT_SLOTS - hole) % CHILD_CONTENT_SLOTS {
                self.relocate(cursor, hole);
                hole = cursor;
            }
            cursor = (cursor + 1) % CHILD_CONTENT_SLOTS;
        }
        Ok(Some(entry))
    }

    fn relocate(&mut self, from: usize, to: usize) {
        let entry = unsafe { self.slots[from].assume_init_read() };
        self.slots[to].write(entry);
        self.occupied[from / 64] &= !(1 << (from % 64));
        self.occupied[to / 64] |= 1 << (to % 64);
        if self.nested(from) {
            self.nested[to / 64] |= 1 << (to % 64);
        } else {
            self.nested[to / 64] &= !(1 << (to % 64));
        }
        self.nested[from / 64] &= !(1 << (from % 64));
        self.generations[to] = self.generations[from];
        let ordinal = self.ordinal_by_slot[from];
        self.dense_slots[ordinal] = to;
        self.ordinal_by_slot[to] = ordinal;
        self.ordinal_by_slot[from] = usize::MAX;
    }

    pub(crate) fn len(&self) -> usize {
        self.len
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<M> Drop for ChildMemberRegistry<M> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.is_empty(), "fixed child-member registry reached Drop before every exact member was transferred to bounded retirement");
        assert!(std::thread::panicking() || self.reserved.iter().all(|word| *word == 0), "fixed child-member registry reached Drop with a reserved owner slot outside exact admission");
        assert!(
            std::thread::panicking() || self.dense_slots.iter().all(|slot| *slot == usize::MAX) && self.ordinal_by_slot.iter().all(|ordinal| *ordinal == usize::MAX),
            "fixed child-member registry reached Drop with a stale dense owner coordinate"
        );
    }
}

/// 📥️ Retains one exact recursive member identity with its admitted bounded open request.
pub struct OwnedDocumentMemberIngress {
    pub(crate) identity: ManuallyDrop<Option<(usize, ArtifactRef, store::OwnerRef)>>,
    pub(crate) request: ManuallyDrop<Option<store::MemberOpenRequest>>,
    pub(crate) identity_field: u8,
}

impl OwnedDocumentMemberIngress {
    /// 🪪️ Binds one closure ordinal and recursive owner to the request's sealed identity.
    #[expect(clippy::result_large_err, reason = "Rejected ingress returns its exact retained page and identity owners unchanged.")]
    pub fn try_new(ordinal: usize, reference: ArtifactRef, owner: store::OwnerRef, request: store::MemberOpenRequest) -> Result<Self, (Fault, Self)> {
        let ingress = Self { identity: ManuallyDrop::new(Some((ordinal, reference, owner))), request: ManuallyDrop::new(Some(request)), identity_field: 0 };
        let (_, reference, owner) = ingress.identity.as_ref().expect("fresh member ingress retains identity");
        let request = ingress.request.as_ref().expect("fresh member ingress retains request");
        let valid = ordinal < store::OWNED_DOCUMENT_MAXIMUM_MEMBERS && request.admitted_expected().is_ok() && request.expected() == reference && request.owner() == Some(owner);
        if valid {
            Ok(ingress)
        } else {
            Err((plugin_sdk_fault("owned document member ingress identity, owner, ordinal, or admitted request does not match"), ingress))
        }
    }

    fn ordinal(&self) -> usize {
        self.identity.as_ref().expect("live member ingress retains identity").0
    }

    pub(crate) fn operation(&self) -> semio_framework_job::OperationId {
        self.request.as_ref().expect("live member ingress retains request").operation()
    }

    pub(crate) fn generation(&self) -> semio_framework_job::Generation {
        self.request.as_ref().expect("live member ingress retains request").generation()
    }

    fn take_request(&mut self) -> Option<store::MemberOpenRequest> {
        self.request.take()
    }

    pub(crate) fn has_request(&self) -> bool {
        self.request.is_some()
    }

    fn return_request(&mut self, request: store::MemberOpenRequest) {
        assert!(self.request.is_none(), "member ingress request can only return to its exact empty slot");
        *self.request = Some(request);
    }

    pub(crate) fn take_identity(&mut self) -> Option<(usize, ArtifactRef, store::OwnerRef)> {
        self.identity.take()
    }

    pub(crate) fn has_identity(&self) -> bool {
        self.identity.is_some()
    }

    fn identity_string_mut(&mut self) -> Option<&mut String> {
        let (_, reference, owner) = self.identity.as_mut()?;
        match self.identity_field {
            0 => Some(&mut reference.artifact_id),
            1 => Some(&mut reference.dialect.artifact_kind),
            2 => Some(&mut reference.dialect.standard),
            3 => Some(&mut reference.dialect.subset),
            4 => Some(&mut owner.parent.artifact_id),
            5 => Some(&mut owner.parent.dialect.artifact_kind),
            6 => Some(&mut owner.parent.dialect.standard),
            7 => Some(&mut owner.parent.dialect.subset),
            8 => Some(&mut owner.slot),
            9 => Some(&mut owner.child_id),
            _ => None,
        }
    }

    /// 📏️ Quotes the next request step, identity string release, or identity handoff.
    pub(crate) fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(request) = self.request.as_ref() {
            if request.terminal_is_empty() { return Ok(RetirementDemand { copy_bytes: size_of_val(&*self.request), depth: 1, ..Default::default() }); }
            return Ok(RetirementDemand { copy_bytes: request.next_copy_byte_demand()?, capacity_bytes: request.next_capacity_byte_demand(body)?, release_bytes: request.next_release_byte_demand()?, depth: request.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original member request parent depth overflow"))? });
        }
        let Some((_, reference, owner)) = self.identity.as_ref() else { return Ok(RetirementDemand::default()) };
        let fields = [&reference.artifact_id, &reference.dialect.artifact_kind, &reference.dialect.standard, &reference.dialect.subset, &owner.parent.artifact_id, &owner.parent.dialect.artifact_kind, &owner.parent.dialect.standard, &owner.parent.dialect.subset, &owner.slot, &owner.child_id];
        Ok(match fields.get(usize::from(self.identity_field)) { Some(field)=>RetirementDemand { copy_bytes: size_of::<String>() + size_of::<u8>(), release_bytes: field.capacity(), depth: 1, ..Default::default() }, None=>RetirementDemand { copy_bytes: size_of_val(&*self.identity), depth: 1, ..Default::default() } })
    }

    pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_items == 0 || grant.maximum_depth < demand.depth || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        if let Some(request) = self.request.as_mut() {
            if request.terminal_is_empty() {
                drop(self.request.take());
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
            }
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = request.close_step(child)?;
            semio_framework_value::retained_clone::admit_retained_clone_close(child, step, request.terminal_is_empty(), "member ingress request")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(field) = self.identity_string_mut() {
            let bytes = field.capacity();
            drop(std::mem::take(field));
            self.identity_field += 1;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: bytes, ..empty }));
        }
        drop(self.identity.take());
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }))
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.identity.is_none() && self.request.is_none()
    }
}

impl Drop for OwnedDocumentMemberIngress {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "owned document member ingress reached Drop before exact handoff or bounded close");
    }
}

const OWNED_DOCUMENT_INGRESS_PAGE_SLOTS: usize = 8;

pub(crate) struct OwnedDocumentMemberIngressRegistry {
    pages: Vec<Option<Box<[std::mem::MaybeUninit<OwnedDocumentMemberIngress>]>>>,
    occupied: [u64; CHILD_CONTENT_SLOTS / 64],
    pub(crate) expected: usize,
    pub(crate) len: usize,
    generation: u64,
    pub(crate) sealed: bool,
    close_cursor: usize,
}

impl OwnedDocumentMemberIngressRegistry {
    pub(crate) fn try_new(expected: usize) -> Result<Self, Fault> {
        if expected > store::OWNED_DOCUMENT_MAXIMUM_MEMBERS {
            return Err(plugin_sdk_fault("owned document member ingress exceeds the recursive closure member limit"));
        }
        let count = expected.div_ceil(OWNED_DOCUMENT_INGRESS_PAGE_SLOTS);
        let mut pages = Vec::new();
        pages.try_reserve_exact(count).map_err(|_| plugin_sdk_fault("owned document member ingress page directory was not admitted"))?;
        for index in 0..count {
            let length = (expected - index * OWNED_DOCUMENT_INGRESS_PAGE_SLOTS).min(OWNED_DOCUMENT_INGRESS_PAGE_SLOTS);
            let mut slots = Vec::new();
            slots.try_reserve_exact(length).map_err(|_| plugin_sdk_fault("owned document member ingress page allocation was not admitted"))?;
            slots.resize_with(length, std::mem::MaybeUninit::uninit);
            pages.push(Some(slots.into_boxed_slice()));
        }
        Ok(Self { pages, occupied: [0; CHILD_CONTENT_SLOTS / 64], expected, len: 0, generation: 0, sealed: false, close_cursor: 0 })
    }

    pub(crate) fn entry(&self, ordinal: usize) -> Option<&OwnedDocumentMemberIngress> {
        if ordinal >= self.expected || self.occupied[ordinal / 64] & (1 << (ordinal % 64)) == 0 { return None; }
        let page = self.pages.get(ordinal / OWNED_DOCUMENT_INGRESS_PAGE_SLOTS)?.as_ref()?;
        Some(unsafe { page[ordinal % OWNED_DOCUMENT_INGRESS_PAGE_SLOTS].assume_init_ref() })
    }

    fn entry_mut(&mut self, ordinal: usize) -> Option<&mut OwnedDocumentMemberIngress> {
        if ordinal >= self.expected || self.occupied[ordinal / 64] & (1 << (ordinal % 64)) == 0 { return None; }
        let page = self.pages.get_mut(ordinal / OWNED_DOCUMENT_INGRESS_PAGE_SLOTS)?.as_mut()?;
        Some(unsafe { page[ordinal % OWNED_DOCUMENT_INGRESS_PAGE_SLOTS].assume_init_mut() })
    }

    fn next_occupied(&self) -> Option<usize> {
        let first = self.close_cursor / 64;
        for index in first..self.occupied.len() {
            let word = self.occupied[index] & if index == first { u64::MAX << (self.close_cursor % 64) } else { u64::MAX };
            if word != 0 { return Some(index * 64 + word.trailing_zeros() as usize); }
        }
        None
    }

    fn empty_backing_byte_demand(&self) -> usize {
        if let Some(page) = self.pages.iter().find_map(Option::as_ref) { return size_of_val(page.as_ref()); }
        self.pages.capacity() * size_of::<Option<Box<[std::mem::MaybeUninit<OwnedDocumentMemberIngress>]>>>()
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the exact retained member ingress owner unchanged.")]
    pub(crate) fn admit(&mut self, ingress: OwnedDocumentMemberIngress) -> Result<(), (Fault, OwnedDocumentMemberIngress)> {
        let ordinal = ingress.ordinal();
        if self.sealed || ordinal >= self.expected || self.occupied[ordinal / 64] & (1 << (ordinal % 64)) != 0 {
            return Err((plugin_sdk_fault("owned document member ingress is sealed, out of range, or duplicates an ordinal"), ingress));
        }
        let Some(generation) = self.generation.checked_add(1) else {
            return Err((plugin_sdk_fault("owned document member ingress generation exhausted"), ingress));
        };
        self.pages[ordinal / OWNED_DOCUMENT_INGRESS_PAGE_SLOTS].as_mut().expect("open ingress ordinal retains its original page")[ordinal % OWNED_DOCUMENT_INGRESS_PAGE_SLOTS].write(ingress);
        self.occupied[ordinal / 64] |= 1 << (ordinal % 64);
        self.len += 1;
        self.generation = generation;
        Ok(())
    }

    pub(crate) fn seal(&mut self) -> Result<(), Fault> {
        if self.sealed || self.len != self.expected {
            return Err(plugin_sdk_fault("owned document member ingress is duplicate-sealed or incomplete"));
        }
        self.sealed = true;
        Ok(())
    }

    pub(crate) fn take(&mut self, ordinal: usize) -> Option<OwnedDocumentMemberIngress> {
        if ordinal >= self.expected || self.occupied[ordinal / 64] & (1 << (ordinal % 64)) == 0 {
            return None;
        }
        self.occupied[ordinal / 64] &= !(1 << (ordinal % 64));
        self.len -= 1;
        Some(unsafe { self.pages[ordinal / OWNED_DOCUMENT_INGRESS_PAGE_SLOTS].as_mut().expect("occupied ingress ordinal retains its original page")[ordinal % OWNED_DOCUMENT_INGRESS_PAGE_SLOTS].assume_init_read() })
    }

    /// ♻️ Retires original occupied slots, indivisible pages, and the separate directory.
    pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, Fault> {
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes).map_err(|error| plugin_sdk_fault(error.to_string()))?;
        if grant.maximum_items == 0 || grant.maximum_depth < demand.depth || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        if !self.sealed { self.sealed = true; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty })); }
        if let Some(ordinal) = self.next_occupied() {
            let active = self.entry_mut(ordinal).expect("occupied ingress keeps its original page");
            if active.terminal_is_empty() {
                drop(self.take(ordinal).ok_or_else(|| plugin_sdk_fault("member ingress close lost its exact occupied ordinal"))?);
                self.close_cursor = ordinal + 1;
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
            }
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = active.close_step(child).map_err(|error| plugin_sdk_fault(error.to_string()))?;
            semio_framework_value::retained_clone::admit_retained_clone_close(child, step, active.terminal_is_empty(), "member ingress").map_err(|error| plugin_sdk_fault(error.to_string()))?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(page) = self.pages.iter_mut().find(|page| page.is_some()) { drop(page.take()); } else { drop(std::mem::take(&mut self.pages)); }
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: demand.release_bytes, ..empty };
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
    }

    pub(crate) fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.terminal_is_empty() { return Ok(RetirementDemand::default()); }
        if !self.sealed { return Ok(RetirementDemand { copy_bytes: size_of::<bool>(), depth: 1, ..Default::default() }); }
        if let Some(active) = self.next_occupied().and_then(|ordinal| self.entry(ordinal)) {
            if active.terminal_is_empty() { return Ok(RetirementDemand { copy_bytes: size_of::<OwnedDocumentMemberIngress>() + size_of::<u64>() + 2 * size_of::<usize>(), depth: 1, ..Default::default() }); }
            let child = active.retirement_demands(body)?;
            return Ok(RetirementDemand { depth: child.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "member ingress registry depth overflow"))?, ..child });
        }
        let copy_bytes=if self.pages.iter().any(Option::is_some) { size_of::<Option<Box<[std::mem::MaybeUninit<OwnedDocumentMemberIngress>]>>>() } else { size_of_val(&self.pages) };
        Ok(RetirementDemand { copy_bytes, release_bytes: self.empty_backing_byte_demand(), depth: 1, ..Default::default() })
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.len == 0 && self.pages.is_empty() && self.pages.capacity() == 0
    }
}

impl Drop for OwnedDocumentMemberIngressRegistry {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "owned document member ingress registry reached Drop before every request owner was handed off or bounded-closed");
    }
}

pub(crate) struct DocumentClosureSourceView<'a, P, M> {
    generation: u64,
    root_reference: &'a ArtifactRef,
    root_snapshot: &'a P,
    members: &'a ChildMemberRegistry<M>,
}

impl<'a, P, M> DocumentClosureSourceView<'a, P, M> {
    pub(crate) fn new(generation: u64, root_reference: &'a ArtifactRef, root_snapshot: &'a P, members: &'a ChildMemberRegistry<M>) -> Self {
        Self { generation, root_reference, root_snapshot, members }
    }
}

impl<P, M> store::OwnedDocumentClosureSource for DocumentClosureSourceView<'_, P, M>
where
    P: semio_framework_schema_composition::ArtifactCompositionFields,
    M: SpaceMember,
{
    fn generation(&self) -> u64 {
        self.generation
    }

    fn root_reference(&self) -> &ArtifactRef {
        self.root_reference
    }

    fn member_count(&self) -> usize {
        self.members.len()
    }

    fn member_reference(&self, index: usize) -> Option<&ArtifactRef> {
        self.members.entry_by_ordinal(index).map(|entry| &entry.reference)
    }

    fn member_owner(&self, index: usize) -> Option<&store::OwnerRef> {
        self.members.entry_by_ordinal(index).map(|entry| &entry.owner)
    }

    fn child_projection(&self, parent: Option<usize>) -> Result<store::ChildRestoreProjection<'_>, store::ChildRestoreProjectionError> {
        match parent {
            None => store::ChildRestoreProjection::from_snapshot(self.root_snapshot),
            Some(index) => self.members.entry_by_ordinal(index).ok_or(store::ChildRestoreProjectionError::IncompleteSet)?.member.child_restore_projection(),
        }
    }
}

#[cfg(test)]
include!("../🧪️tests/🔬️app-child-member-registry/🦀️.rs");

include!("../🧩️composition/🪪️metadata/🦀️.rs");

include!("../🧩️composition/📬️publication/🤝️group/🦀️.rs");

mod mounted_child_group_identity { include!("../🧩️composition/📬️publication/🤝️group/🪟️mounted/🪪️identity/🦀️.rs"); }

pub(crate) use mounted_child_group_identity::{MountedChildGroupIdentityIssuer, MountedChildGroupIdentitySource};

pub(crate) mod mounted_child_group_receipt { use super::*; include!("../🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/🦀️.rs"); }


pub(crate) mod mounted_child_group_receipt_group { use super::*; use super::mounted_child_group_receipt::*; include!("../🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/🦀️.rs"); }

use mounted_child_group_receipt_group::command::CommandPrune;

include!("../🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/📚️command/🪟️mounted/🦀️.rs");

include!("../🧩️composition/📬️publication/🤝️group/🪟️mounted/📦️owner/🦀️.rs");

#[cfg(test)]
include!("../🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/📚️command/🪟️mounted/🧪️tests/🦀️.rs");

pub(crate) struct ChildContentBorrowedOwners<'a> {
    views: [Option<&'a ChildContentView>; ARTIFACT_LIVE_OUTPUT_SLOTS + 2],
    len: usize,
}

impl<'a> ChildContentBorrowedOwners<'a> {
    pub(crate) fn none() -> Self { Self { views: [None; ARTIFACT_LIVE_OUTPUT_SLOTS + 2], len: 0 } }
    fn push(&mut self, view: &'a ChildContentView) {
        assert!(self.len < self.views.len(), "borrowed original child retainers exceed their fixed registry");
        self.views[self.len] = Some(view);
        self.len += 1;
    }
    pub(crate) fn copy_reservation() -> usize {
        size_of::<Self>() + size_of::<Option<&mut ChildContentRetirement>>() + size_of::<u64>()
    }
}

impl ChildContentRetainers for ChildContentBorrowedOwners<'_> {
    fn retains_root(&self, view: &ChildContentView) -> bool { self.views[..self.len].iter().flatten().any(|retainer| retainer.same_owner(view)) }
    fn retains_page(&self, page_index: usize, page: &std::sync::Arc<ChildContentPage>) -> bool { self.views[..self.len].iter().flatten().any(|view| ChildContentOwners::view_retains_page(view, page_index, page)) }
    fn retains_entry(&self, entry: &std::sync::Arc<ChildContentEntry>) -> bool { self.views[..self.len].iter().flatten().any(|view| view.contains_entry_owner(entry)) }
}

/// 🍂️ Whether the member with one exact artifact id is still retiring (it has not closed yet).
pub(crate) fn member_is_retiring<M>(registry: &ArtifactFixedRegistry<ChildMemberRetirement<M>>, artifact_id: &str) -> bool {
    (0..ARTIFACT_LIVE_OUTPUT_SLOTS).any(|index| registry.entry(index).is_some_and(|(_, retirement)| retirement.retires(artifact_id)))
}

/// 🍂️ The retiring member with one exact artifact id, read-only for demand quotes.
fn retiring_member<'a, M>(registry: &'a ArtifactFixedRegistry<ChildMemberRetirement<M>>, artifact_id: &str) -> Option<&'a M> {
    (0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|index| registry.entry(index).and_then(|(_, retirement)| retirement.entry.as_ref().filter(|entry| entry.reference.artifact_id == artifact_id)).map(|entry| &entry.member))
}

/// 🍂️ The retiring member with one exact artifact id — the disposer of the snapshot leases retired roots still hold.
fn retiring_member_mut<'a, M>(registry: &'a mut ArtifactFixedRegistry<ChildMemberRetirement<M>>, artifact_id: &str) -> Option<&'a mut M> {
    let id = (0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|index| registry.entry(index).and_then(|(id, retirement)| retirement.retires(artifact_id).then_some(*id)))?;
    registry.get_mut(id)?.entry.as_mut().map(|entry| &mut entry.member)
}

/// 🪪️ The member that disposes one child snapshot lease: the live member at its owner edge, else the retiring member
/// of that artifact id, which its coordinate left while a retired root still lent its snapshot.
fn child_snapshot_owner<'a, M>(children: &'a mut ChildMemberRegistry<M>, retiring: Option<&'a mut ArtifactFixedRegistry<ChildMemberRetirement<M>>>, key: MemberKeyRef<'_>, artifact_id: &str) -> Option<&'a mut M> {
    if let Some(entry) = children.member_mut(key) {
        return Some(&mut entry.member);
    }
    retiring_member_mut(retiring?, artifact_id)
}

/// 🪪️ Read-only twin of [`child_snapshot_owner`] for demand quotes.
fn child_snapshot_owner_ref<'a, M>(children: &'a ChildMemberRegistry<M>, retiring: Option<&'a ArtifactFixedRegistry<ChildMemberRetirement<M>>>, key: MemberKeyRef<'_>, artifact_id: &str) -> Option<&'a M> {
    if let Some(entry) = children.member(key) {
        return Some(&entry.member);
    }
    retiring_member(retiring?, artifact_id)
}

/// 🪢️ Borrows disjoint initialized registry slots and the exact live preview retainers.
fn original_content_step_owners<'a>(registry: &'a mut ArtifactFixedRegistry<ChildContentRetirement>, stepping: u64, member: Option<&'a ChildContentView>, review: Option<&'a ChildContentView>) -> Option<(&'a mut ChildContentRetirement, ChildContentBorrowedOwners<'a>)> {
    let mut owners = ChildContentBorrowedOwners::none();
    let selected = registry.partition_mut(stepping, |retirement| owners.push(&retirement.view));
    if let Some(view) = member { owners.push(view); }
    if let Some(view) = review { owners.push(view); }
    Some((selected?, owners))
}

include!("../🧩️composition/♻️root/🦀️.rs");

#[derive(semio_framework_value::RetireOwned)]
struct ChildContentMemberMetadata {
    key: MemberKey,
    artifact_id: String,
    dialect: ArtifactDialect,
}

pub(crate) struct ChildContentRetirement {
    view: ManuallyDrop<ChildContentView>,
    pub(crate) pending: ManuallyDrop<Option<ChildContentEntry>>,
    active: ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    active_member: ManuallyDrop<Option<ChildContentMemberMetadata>>,
    metadata_close: ManuallyDrop<Option<ControlledRetirement<ChildContentMemberMetadata>>>,
    require_member_terminal: bool,
}

impl ChildContentRetirement {
    pub(crate) fn new(view: ChildContentView, require_member_terminal: bool) -> Self {
        Self { view: ManuallyDrop::new(view), pending: ManuallyDrop::new(None), active: ManuallyDrop::new(None), active_member: ManuallyDrop::new(None), metadata_close: ManuallyDrop::new(None), require_member_terminal }
    }

    fn snapshot_handoff_copy_bytes()->usize{
        size_of::<Option<ChildContentEntry>>()+size_of::<ChildContentEntry>()+size_of::<Option<store::os_store::snapshot_read::ErasedSnapshotRead>>()+size_of::<Option<ChildContentMemberMetadata>>()+size_of::<Option<Box<dyn store::ErasedSnapshotRetirement>>>()
    }

    pub(crate) fn parent_snapshot_demand(mut child:RetirementDemand,parent_copy:usize)->Result<RetirementDemand,ValueError>{
        child.copy_bytes=child.copy_bytes.checked_add(parent_copy).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original child snapshot header overflow"))?;
        child.depth=child.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original child snapshot depth overflow"))?;Ok(child)
    }

    /// 📏️ Quotes the next disposer step, lease verification, entry disposal, or Arc release against its exact owning member.
    pub(crate) fn retirement_demands<M: SpaceMember>(&self, children: &ChildMemberRegistry<M>, retiring: Option<&ArtifactFixedRegistry<ChildMemberRetirement<M>>>, body: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(active) = self.active.as_ref() { let parent_copy=if active.terminal_is_empty(){size_of_val(&*self.active)}else{0};return Self::parent_snapshot_demand(store::os_store::snapshot_read::artifact_retirement_box_demands(active,body.saturating_sub(parent_copy))?,parent_copy); }
        if let Some(owner)=self.metadata_close.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of_val(&*self.metadata_close),depth:1,..Default::default()})}else{Self::parent_snapshot_demand(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?},0)}}
        if let Some(original) = self.active_member.as_ref() {
            if self.require_member_terminal{
                let Some(member)=child_snapshot_owner_ref(children,retiring,original.key.borrowed(),&original.artifact_id)else{return Ok(RetirementDemand{depth:1,..Default::default()})};
                let returned=member.returned_snapshot_read_retirement_demand()?;
                if returned!=RetirementDemand::default(){return Self::parent_snapshot_demand(returned,size_of_val(&*self.active))}
                if !member.snapshot_read_leases_terminal_is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()})}
            }
            return Ok(RetirementDemand{copy_bytes:size_of_val(&*self.active_member)+size_of_val(&*self.metadata_close),depth:1,..Default::default()});
        }
        if let Some(entry) = self.pending.as_ref() {
            let Some(member) = child_snapshot_owner_ref(children, retiring, entry.key(), &entry.artifact_id) else { return Ok(RetirementDemand { depth: 1, ..Default::default() }); };
            return Self::parent_snapshot_demand(member.snapshot_read_retirement_demand(),Self::snapshot_handoff_copy_bytes());
        }
        if self.terminal_is_empty() { return Ok(RetirementDemand::default()); }
        Ok(RetirementDemand { copy_bytes: self.view.next_take_copy_bytes(), release_bytes: self.view.next_take_release_bytes(), depth: 1, ..Default::default() })
    }

    pub(crate) fn close_step<M: SpaceMember>(
        &mut self,
        children: &mut ChildMemberRegistry<M>,
        retiring: Option<&mut ArtifactFixedRegistry<ChildMemberRetirement<M>>>,
        current: &ChildContentView,
        owners: &dyn ChildContentRetainers,
        grant: RetainedCloneGrant,
    ) -> Result<RetainedCloneStep, Fault> {
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.retirement_demands(children, retiring.as_deref(), grant.maximum_copy_bytes).map_err(|error| plugin_sdk_fault(error.to_string()))?;
        if grant.maximum_depth < demand.depth { return Ok(RetainedCloneStep::Progress(empty)); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        let admit = |progress: RetainedCloneProgress| semio_framework_value::retained_clone::admit_retained_clone_progress(grant, progress, "child snapshot retirement").map_err(|error| plugin_sdk_fault(error.to_string()));
        if self.active.is_some() {
            let parent_copy=if self.active.as_ref().unwrap().terminal_is_empty(){size_of_val(&*self.active)}else{0};
            let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent_copy,maximum_depth:grant.maximum_depth-1,..grant};
            let step = store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active, child)
                .map_err(|reason| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-snapshot-disposer-refused"), format!("child snapshot disposer refused its own bounded step: {reason}")))?;
            let mut receipt=step.progress();receipt.copied_bytes=receipt.copied_bytes.checked_add(parent_copy).expect("original child header preflight");return Ok(RetainedCloneStep::Progress(admit(receipt)?));
        }
        if let Some(owner)=self.metadata_close.as_mut(){
            if owner.terminal_is_empty(){drop(self.metadata_close.take());return Ok(RetainedCloneStep::Progress(admit(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty})?))}
            let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=owner.step(child).map_err(|error|plugin_sdk_fault(error.into_message()))?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,owner.terminal_is_empty(),"original child member metadata").map_err(|error|plugin_sdk_fault(error.into_message()))?;return Ok(RetainedCloneStep::Progress(admit(step.progress())?))
        }
        if let Some(original) = self.active_member.as_ref() {
            if self.require_member_terminal {
                let Some(member) = child_snapshot_owner(children, retiring, original.key.borrowed(), &original.artifact_id) else { return Ok(RetainedCloneStep::Progress(empty)); };
                if member.returned_snapshot_read_retirement_demand().map_err(|error|plugin_sdk_fault(error.into_message()))?!=RetirementDemand::default(){
                    let parent_copy=size_of_val(&*self.active);let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent_copy,maximum_depth:grant.maximum_depth-1,..grant};let(owner,progress)=member.take_returned_snapshot_read_retirement(child).map_err(|error|plugin_sdk_fault(error.into_message()))?;semio_framework_value::retained_clone::admit_retained_clone_progress(child,progress,"original returned child snapshot").map_err(|error|plugin_sdk_fault(error.into_message()))?;
                    let mut receipt=progress;if let Some(owner)=owner{*self.active=Some(owner);receipt.copied_items=receipt.copied_items.max(1);receipt.copied_bytes=receipt.copied_bytes.checked_add(parent_copy).expect("original returned header preflight")};return Ok(RetainedCloneStep::Progress(admit(receipt)?));
                }
                if !member.snapshot_read_leases_terminal_is_empty() { return Ok(RetainedCloneStep::Progress(empty)); }
            }
            if !<ChildContentMemberMetadata as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported(){return Err(plugin_sdk_fault("original child metadata has no controlled producer"))}
            let original=self.active_member.take().unwrap();match ControlledRetirement::new(original){Ok(owner)=>*self.metadata_close=Some(owner),Err((error,original))=>{*self.active_member=Some(original);return Err(plugin_sdk_fault(error.into_message()))}};
            return Ok(RetainedCloneStep::Progress(admit(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty})?));
        }
        if self.pending.is_none() {
            return match self.view.take_one(current, owners) {
                ChildContentTake::Blocked => Ok(RetainedCloneStep::Progress(empty)),
                ChildContentTake::Released(released_bytes) => Ok(RetainedCloneStep::Progress(admit(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes, ..empty })?)),
                ChildContentTake::Snapshot(entry, released_bytes) => {
                    *self.pending = Some(entry);
                    Ok(RetainedCloneStep::Progress(admit(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes, ..empty })?))
                }
                ChildContentTake::Complete(released_bytes) => Ok(RetainedCloneStep::Complete(admit(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes, ..empty })?)),
            };
        }
        let entry=self.pending.as_ref().unwrap();let Some(member)=child_snapshot_owner(children,retiring,entry.key(),&entry.artifact_id)else{return Ok(RetainedCloneStep::Progress(empty))};
        let parent_copy=Self::snapshot_handoff_copy_bytes();let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent_copy,maximum_depth:grant.maximum_depth-1,..grant};
        let entry = self.pending.take().expect("pending child snapshot authority exists");
        let ChildContentEntry { owner: member_owner, slot, child_id, artifact_id, dialect, revision, snapshot } = entry;
        let mut read = Some(snapshot);
        let restore = |read: &mut Option<store::os_store::snapshot_read::ErasedSnapshotRead>, member_owner, slot, child_id, artifact_id, dialect, revision| ChildContentEntry { owner: member_owner, slot, child_id, artifact_id, dialect, revision, snapshot: read.take().expect("refused snapshot read remains with its caller") };
        match member.retire_snapshot_read_erased(&mut read, child) {
            Ok((Some(owner), progress)) => {
                semio_framework_value::retained_clone::admit_retained_clone_progress(child,progress,"original child snapshot handoff").map_err(|error|plugin_sdk_fault(error.into_message()))?;
                *self.active_member = Some(ChildContentMemberMetadata{key:MemberKey { owner: member_owner, slot, child_id }, artifact_id, dialect});
                *self.active = Some(owner);
                Ok(RetainedCloneStep::Progress(admit(RetainedCloneProgress{copied_items:progress.copied_items.max(1),copied_bytes:progress.copied_bytes.checked_add(parent_copy).expect("original child handoff preflight"),..progress})?))
            }
            Ok((None, progress)) => {
                *self.pending = Some(restore(&mut read, member_owner, slot, child_id, artifact_id, dialect, revision));
                semio_framework_value::retained_clone::admit_retained_clone_progress(child,progress,"original refused child snapshot").map_err(|error|plugin_sdk_fault(error.into_message()))?;
                Ok(RetainedCloneStep::Progress(admit(RetainedCloneProgress{copied_items:progress.copied_items.max(1),copied_bytes:progress.copied_bytes.checked_add(parent_copy).expect("original child restore preflight"),..progress})?))
            }
            Err(error) => {
                *self.pending = Some(restore(&mut read, member_owner, slot, child_id, artifact_id, dialect, revision));
                Err(plugin_sdk_fault(error.to_string()))
            }
        }
    }
    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.view.root.is_none() && self.pending.is_none() && self.active.is_none() && self.active_member.is_none() && self.metadata_close.is_none()
    }
}

impl Drop for ChildContentRetirement {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "incomplete child-content retirement reached Drop without bounded disposal");
    }
}

pub(crate) struct SnapshotReadReturnPump {
    active: ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
}

impl SnapshotReadReturnPump {
    pub(crate) fn new() -> Self {
        Self { active: ManuallyDrop::new(None) }
    }

    /// 📏️ Quotes the active disposer's next step, else the caller's quote of the next returned-read admission.
    pub(crate) fn retirement_demands(&self, body: usize, next: impl FnOnce() -> Result<RetirementDemand, ValueError>) -> Result<RetirementDemand, ValueError> {
        let parent_copy=size_of_val(&*self.active);
        match self.active.as_ref(){Some(active)=>{let copied=if active.terminal_is_empty(){parent_copy}else{0};ChildContentRetirement::parent_snapshot_demand(store::os_store::snapshot_read::artifact_retirement_box_demands(active,body.saturating_sub(copied))?,copied)},None=>{let child=next()?;if child==RetirementDemand::default(){Ok(child)}else{ChildContentRetirement::parent_snapshot_demand(child,parent_copy)}}}
    }

    pub(crate) fn drive(&mut self,next_demand:RetirementDemand,next:impl FnOnce(RetainedCloneGrant)->Result<(Option<Box<dyn store::ErasedSnapshotRetirement>>,RetainedCloneProgress),Fault>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,Fault>{
        let empty = RetainedCloneProgress::default();
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(empty))}
        if let Some(active)=self.active.as_ref(){
            let parent_copy=if active.terminal_is_empty(){size_of_val(&*self.active)}else{0};if grant.maximum_copy_bytes<parent_copy{return Ok(RetainedCloneStep::Progress(empty))}
            let demand=ChildContentRetirement::parent_snapshot_demand(store::os_store::snapshot_read::artifact_retirement_box_demands(active,grant.maximum_copy_bytes-parent_copy).map_err(|error|plugin_sdk_fault(error.into_message()))?,parent_copy).map_err(|error|plugin_sdk_fault(error.into_message()))?;
            if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(empty))}
            let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent_copy,maximum_depth:grant.maximum_depth-1,..grant};let step=store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active,child).map_err(|error|plugin_sdk_fault(error.into_message()))?;let mut receipt=step.progress();receipt.copied_bytes=receipt.copied_bytes.checked_add(parent_copy).expect("original returned snapshot shell preflight");let receipt=semio_framework_value::retained_clone::admit_retained_clone_progress(grant,receipt,"original returned snapshot shell").map_err(|error|plugin_sdk_fault(error.into_message()))?;return Ok(RetainedCloneStep::Progress(receipt));
        }
        if next_demand==RetirementDemand::default(){return Ok(RetainedCloneStep::Complete(empty))}
        let parent_copy=size_of_val(&*self.active);let demand=ChildContentRetirement::parent_snapshot_demand(next_demand,parent_copy).map_err(|error|plugin_sdk_fault(error.into_message()))?;if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(empty))}let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent_copy,maximum_depth:grant.maximum_depth-1,..grant};let(owner,progress)=next(child)?;
        semio_framework_value::retained_clone::admit_retained_clone_progress(child, progress, "returned snapshot-read admission").map_err(|error| plugin_sdk_fault(error.to_string()))?;
        match owner {
            Some(owner) => {
                *self.active = Some(owner);
                let mut receipt=progress;receipt.copied_items=receipt.copied_items.max(1);receipt.copied_bytes=receipt.copied_bytes.checked_add(parent_copy).expect("original returned snapshot handoff preflight");let receipt=semio_framework_value::retained_clone::admit_retained_clone_progress(grant,receipt,"original returned snapshot handoff").map_err(|error|plugin_sdk_fault(error.into_message()))?;Ok(RetainedCloneStep::Progress(receipt))
            }
            None if progress == empty => Ok(RetainedCloneStep::Complete(empty)),
            None => Ok(RetainedCloneStep::Progress(progress)),
        }
    }
    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.active.is_none()
    }
}

impl Drop for SnapshotReadReturnPump {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "returned snapshot-read pump reached Drop before its exact disposer was terminal-empty");
    }
}

#[cfg(test)]
pub(super) struct ChildAdmissionTestState {
    pub(super) children_empty: bool,
    pub(super) generation: u64,
    pub(super) abort_generation: u64,
    pub(super) abort_empty: bool,
    pub(super) requested_abort_retained: bool,
    pub(super) content_empty: bool,
    pub(super) roots_retiring_empty: bool,
}

#[derive(semio_framework_value::RetireOwned)]
struct ChildMemberMetadata {
    reference: ArtifactRef,
    owner: store::OwnerRef,
}

pub(crate) struct ChildMemberRetirement<M> {
    pub(crate) entry: ManuallyDrop<Option<ChildMemberEntry<M>>>,
    metadata:ManuallyDrop<Option<ControlledRetirement<ChildMemberMetadata>>>,
}

pub(crate) struct ChildStoreAdmission {
    pub(crate) key: MemberKey,
    pub(crate) expected: ArtifactRef,
    pub(crate) owner: store::OwnerRef,
    pub(crate) member: ChildMemberAdmission,
    pub(crate) graph: store::os_store::composition_coordinator::OwnsAdmission,
    pub(crate) root_index: usize,
    pub(crate) generation: u64,
}

impl<M> ChildMemberRetirement<M> {
    fn retires(&self, artifact_id: &str) -> bool {
        self.entry.as_ref().is_some_and(|entry| entry.reference.artifact_id == artifact_id)
    }
}

impl<M: SpaceMember> ChildMemberRetirement<M> {
    pub(crate) fn new(entry: ChildMemberEntry<M>) -> Self {
        Self { entry: ManuallyDrop::new(Some(entry)),metadata:ManuallyDrop::new(None) }
    }

    pub(crate) fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if let Some(owner)=self.metadata.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of_val(&*self.metadata),depth:1,..Default::default()})}else{ChildContentRetirement::parent_snapshot_demand(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?},0)}}
        let Some(entry)=self.entry.as_ref()else{return Ok(Default::default())};
        if !entry.member.snapshot_read_leases_terminal_is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()})}
        if !entry.member.close_owned_terminal_is_empty(){return ChildContentRetirement::parent_snapshot_demand(entry.member.close_owned_demands(body)?,0)}
        Ok(RetirementDemand{copy_bytes:size_of_val(&*self.entry)+size_of::<ChildMemberEntry<M>>()+size_of_val(&*self.metadata),depth:1,..Default::default()})
    }

    pub(crate) fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,Fault>{
        let empty=RetainedCloneProgress::default();if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty))}
        let demand=self.retirement_demands(grant.maximum_copy_bytes).map_err(|error|plugin_sdk_fault(error.into_message()))?;if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(empty))}
        if let Some(owner)=self.metadata.as_mut(){if owner.terminal_is_empty(){drop(self.metadata.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}))}let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=owner.step(child).map_err(|error|plugin_sdk_fault(error.into_message()))?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,owner.terminal_is_empty(),"original child member metadata").map_err(|error|plugin_sdk_fault(error.into_message()))?;return Ok(RetainedCloneStep::Progress(step.progress()))}
        let entry=self.entry.as_mut().unwrap();let member=&mut entry.member;
        if !member.snapshot_read_leases_terminal_is_empty(){return Ok(RetainedCloneStep::Progress(empty))}
        if !member.close_owned_terminal_is_empty(){let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=member.close_owned_step(child).map_err(|error|plugin_sdk_fault(error.into_message()))?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,member.close_owned_terminal_is_empty(),"original child member store").map_err(|error|plugin_sdk_fault(error.into_message()))?;return Ok(RetainedCloneStep::Progress(step.progress()))}
        if !<ChildMemberMetadata as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported(){return Err(plugin_sdk_fault("original child member metadata requires its native controlled producer"))}
        let ChildMemberEntry{reference,owner,member}=self.entry.take().unwrap();let original=ChildMemberMetadata{reference,owner};match ControlledRetirement::new(original){Ok(owner)=>*self.metadata=Some(owner),Err((error,original))=>{*self.entry=Some(ChildMemberEntry{reference:original.reference,owner:original.owner,member});return Err(plugin_sdk_fault(error.into_message()))}};drop(member);
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}))
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.entry.is_none()&&self.metadata.is_none()
    }
}

impl<M> Drop for ChildMemberRetirement<M> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.entry.is_none()&&self.metadata.is_none()), "child-member retirement reached Drop before its exact owner was terminal-empty");
    }
}

use crate::bounded_ownership::plugin_grant_funds;
