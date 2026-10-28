//! 📀️ Staged slice of the `ingress` band of the mounted artifact instance: its `instance-children` members (WP6 step S0 of the instance family).
//!
//! 🚪️ Ingress of the mounted artifact instance: framework-reserved job driving, constructors, store-constructor units, envelope ingress and decode, document archive genesis and merge, child members and packs.
use semio_framework_plugin_ownership::bounded_ownership::ARTIFACT_LIVE_OUTPUT_SLOTS;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_artifact_reference::{ArtifactDialect, ArtifactRef};
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
#[cfg(test)]
use semio_framework_plugin_child::ChildAdmissionTestState;
use crate::children_components::ChildComposition;
use semio_framework_plugin_child::{ChildContentRetirement, ChildMemberEntry, ChildMemberRegistrationError, ChildMemberRetirement, ChildStoreAdmission, DocumentClosureSourceView, MemberPath};
use semio_framework_plugin_decl_views::child_view::{ChildContentView, MemberKey, MemberKeyRef};
use semio_framework_plugin_instance_history::history_components::CommandHistory;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::{Fault, FaultCode, FaultFrom, FaultOrigin};
use std::future::Future;
use semio_framework_plugin_instance_stores::stores_components::InstanceStores;
use store::{MemberFactory, Mutation, ReferenceSpaceMember};
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework_ui_contract::*;
use crate::children_instance_root::genesis_member_schema;

impl<M: ReferenceSpaceMember + MemberFactory + 'static> ChildComposition<M> {
    /// 🌱️ Opens every composed child the initial snapshot declares and the app derives
    /// (`ArtifactApp::genesis_child_pack`) through the ordinary restore path, so `children`, the
    /// composition graph and the child content root are complete from the first frame — a saved
    /// archive of a never-edited document then carries its derivable members instead of relying on
    /// the loading side to re-derive them.
    pub async fn seed_genesis_children<A: ArtifactApp>(&mut self, stores: &mut InstanceStores<A>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        let mut genesis = Vec::new();
        {
            let snapshot = stores.store.snapshot_ref();
            let projection = semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| plugin_sdk_fault(format!("initial snapshot child projection failed: {error}")))?;
            for index in 0..projection.len() {
                let Some((slot, fields)) = projection.get(index) else { break };
                let Some(initial_pack) = A::genesis_child_pack(snapshot, slot, fields.child_id).map_err(|error| plugin_sdk_fault(error.into_message()))? else { continue };
                let dialect = ArtifactDialect { artifact_kind: fields.artifact_kind.to_string(), standard: fields.standard.to_string(), subset: fields.subset.to_string() };
                genesis.push((slot.to_string(), fields.child_id.to_string(), ArtifactRef { artifact_id: fields.artifact_id.to_owned(), dialect }, initial_pack));
            }
        }
        let parent = ArtifactRef { artifact_id: stores.store.envelope().id.clone(), dialect: A::DIALECT.into() };
        for (slot, child_id, expected, initial_pack) in genesis {
            let dialect = expected.dialect.clone();
            let schema = genesis_member_schema::<M>(&dialect)?;
            let owner = store::OwnerRef { parent: parent.clone(), slot: slot.clone(), child_id: child_id.clone() };
            let envelope_pack = store::os_store::text_format::genesis_member_envelope_pack(schema, &expected, &owner, &initial_pack).await.map_err(|error| plugin_sdk_fault(format!("genesis child {slot}/{child_id} envelope: {error}")))?;
            self.open_child(stores, slot, child_id, dialect, &envelope_pack, identity).await?;
        }
        Ok(())
    }

    /// 🪆️ Derivable composed children follow their coordinate. A content-addressed child is minted from the
    /// parent's own content, so a parent-lane change can re-point a declared coordinate at a child id no store
    /// holds (P8 `composedChildOrphaned`). This opens every declared child no store holds that
    /// `ArtifactApp::genesis_child_pack` derives — the restore path `seed_genesis_children` uses — and retires
    /// every held child whose slot now names another derived child, so exporters, archives, hub members,
    /// agents and the next verb read the child the parent names. Gated by the parent store's generation and
    /// deferred while a child admission is in flight or while the named child is still retiring (an undo back
    /// to it) — the pass then repeats until that retirement drained. It also waits, before it changes anything,
    /// while the retirements its own publications need are not yet admissible ([`Self::reclaim_child_retirements`]):
    /// the parent already holds every edit in order, the child it names follows once maintenance returned them, and
    /// the instance stays runnable meanwhile (`child_follow_awaits_retirement`).
    pub async fn follow_derivable_children<A: ArtifactApp>(&mut self, stores: &mut InstanceStores<A>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        let generation = stores.store.generation_now();
        if generation == self.followed_parent_generation || self.children.has_admission_in_flight() {
            return Ok(());
        }
        let mut genesis = Vec::new();
        let mut retire = Vec::new();
        let mut deferred = false;
        {
            let snapshot = stores.store.snapshot_ref();
            let projection = semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| plugin_sdk_fault(format!("derivable child projection failed: {error}")))?;
            let mut declared = Vec::with_capacity(projection.len());
            for index in 0..projection.len() {
                let Some((slot, fields)) = projection.get(index) else { break };
                let key = (slot.to_string(), fields.child_id.to_string());
                if self.children.get(&key).is_none() {
                    if semio_framework_plugin_child::member_is_retiring(&self.child_member_retirements, fields.artifact_id) {
                        deferred = true;
                    } else if let Some(initial_pack) = A::genesis_child_pack(snapshot, slot, fields.child_id).map_err(|error| plugin_sdk_fault(error.into_message()))? {
                        let dialect = ArtifactDialect { artifact_kind: fields.artifact_kind.to_string(), standard: fields.standard.to_string(), subset: fields.subset.to_string() };
                        genesis.push((key.0.clone(), key.1.clone(), ArtifactRef { artifact_id: fields.artifact_id.to_owned(), dialect }, initial_pack));
                    }
                }
                declared.push(key);
            }
            for (member, _) in self.children.keyed_entries().filter(|(member, _)| member.owner.is_empty()) {
                let key = (member.slot.to_string(), member.child_id.to_string());
                if !declared.contains(&key) && genesis.iter().any(|(slot, _, _, _)| *slot == key.0) {
                    retire.push(key);
                }
            }
        }
        let publications = retire.len() + genesis.len();
        if !self.reclaim_child_retirements(publications, publications)? {
            self.child_follow_awaits_retirement = true;
            return Ok(());
        }
        for (slot, child_id) in retire {
            self.retire_followed_child(&slot, &child_id).await?;
        }
        let parent = ArtifactRef { artifact_id: stores.store.envelope().id.clone(), dialect: A::DIALECT.into() };
        for (slot, child_id, expected, initial_pack) in genesis {
            let dialect = expected.dialect.clone();
            let schema = genesis_member_schema::<M>(&dialect)?;
            let owner = store::OwnerRef { parent: parent.clone(), slot: slot.clone(), child_id: child_id.clone() };
            let envelope_pack = store::os_store::text_format::genesis_member_envelope_pack(schema, &expected, &owner, &initial_pack).await.map_err(|error| plugin_sdk_fault(format!("followed child {slot}/{child_id} envelope: {error}")))?;
            self.open_child(stores, slot, child_id, dialect, &envelope_pack, identity).await?;
        }
        if !deferred {
            self.followed_parent_generation = generation;
        }
        self.child_follow_awaits_retirement = deferred;
        Ok(())
    }

    /// 🍂️ Moves one held child its coordinate left out of every live authority — the member map, the ownership
    /// graph and the published child-content root (the previous root goes to bounded content retirement) —
    /// into bounded child-member retirement, where it stays the disposer of the snapshot leases that previous
    /// root still holds (`ChildContentRetirement::close_step` finds it there) and closes only after them.
    pub(crate) async fn retire_followed_child(&mut self, slot: &str, child_id: &str) -> Result<(), Fault> {
        let retirement_generation = self.child_member_retirement_generation.checked_add(1).ok_or_else(|| plugin_sdk_fault("child member retirement generation exhausted"))?;
        if !self.reclaim_child_retirements(0, 1)? {
            return Err(plugin_sdk_fault("child member retirement waits on retirements that are all still borrowed").with_retryable(true));
        }
        let publication_generation = self.admit_child_content_publication()?;
        let next = self.child_content_root.without_member(MemberKeyRef::root(slot, child_id))?;
        let entry = self.children.remove(MemberKeyRef::root(slot, child_id))?.ok_or_else(|| plugin_sdk_fault("followed child left the member map before its exact retirement"))?;
        self.composition.graph_mut().await.remove_owns(&entry.reference.artifact_id).await;
        let previous = std::mem::replace(&mut *self.child_content_root, next);
        if !previous.is_empty() {
            self.child_content_retirements.insert_admitted(publication_generation, ChildContentRetirement::new(previous, false));
        }
        self.child_content_generation = publication_generation;
        self.child_member_retirements.insert_admitted(retirement_generation, ChildMemberRetirement::new(entry));
        self.child_member_retirement_generation = retirement_generation;
        Ok(())
    }

    /// ⏳️ A follow pass waits on child retirements and the parent has not been followed since — pending, runnable
    /// work until maintenance returned them (see `child_follow_awaits_retirement`).
    pub fn derivable_follow_awaits_retirement<A: ArtifactApp>(&self, stores: &InstanceStores<A>) -> bool {
        self.child_follow_awaits_retirement && stores.store.generation_now() != self.followed_parent_generation
    }

    /// ♻️ Whether the next `roots` child-content and `members` child-member retirement generations each find a vacant
    /// slot of their fixed registry.
    pub(crate) fn child_retirements_admit(&self, roots: usize, members: usize) -> bool {
        (1..=roots as u64).all(|offset| self.child_content_generation.checked_add(offset).is_some_and(|generation| self.child_content_retirements.can_insert(generation)))
            && (1..=members as u64).all(|offset| self.child_member_retirement_generation.checked_add(offset).is_some_and(|generation| self.child_member_retirements.can_insert(generation)))
    }

    /// ⏳️ Queries the original fixed retirement slots; funded maintenance makes room.
    pub fn reclaim_child_retirements(&self, roots: usize, members: usize) -> Result<bool, Fault> {
        if roots > ARTIFACT_LIVE_OUTPUT_SLOTS || members > ARTIFACT_LIVE_OUTPUT_SLOTS {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-root-retirement-span"), "one child publication span exceeds the fixed retirement authority"));
        }
        Ok(self.child_retirements_admit(roots, members))
    }

    /// 🎟️ Admits the next immutable child-content publication once its retirement slot is vacant — reclaimed first when
    /// an older retirement still holds it ([`Self::reclaim_child_retirements`]).
    pub fn admit_child_content_publication(&mut self) -> Result<u64, Fault> {
        self.admit_child_content_publication_span(1)?;
        Ok(self.child_content_generation + 1)
    }

    /// 🎟️ Admits `count` consecutive child-content publications (see [`Self::admit_child_content_publication`]). Only a
    /// ring of retirements that are every one still borrowed answers the retryable
    /// `interactive-job.child-root-retirement-saturated`.
    pub fn admit_child_content_publication_span(&mut self, count: usize) -> Result<(), Fault> {
        if self.child_content_generation.checked_add(count as u64).is_none() {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-root-generation"), "immutable child-content generation exhausted"));
        }
        if !self.reclaim_child_retirements(count, 0)? {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-root-retirement-saturated"), "immutable child-content publication waits on retirements that are all still borrowed").with_retryable(true));
        }
        Ok(())
    }

    pub async fn publish_child_content_member(&mut self, generation: u64, slot: &str, child_id: &str) -> Result<(), Fault> {
        self.publish_member_content(generation, MemberKeyRef::root(slot, child_id)).await
    }

    /// 📰️ Republishes the immutable child-content root with the member at one exact owner edge captured anew.
    pub async fn publish_member_content(&mut self, generation: u64, key: MemberKeyRef<'_>) -> Result<(), Fault> {
        if generation != self.child_content_generation.saturating_add(1) || !self.child_content_retirements.can_insert(generation) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-root-publication-authority"), "immutable child-content publication lost its exact admitted generation"));
        }
        let entry = self.children.member(key).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-root-member"), "immutable child-content publication lost its exact live member"))?;
        let next = self.child_content_root.with_member(key, &entry.reference, &entry.member).await?;
        let previous = std::mem::replace(&mut *self.child_content_root, next);
        if previous.root.as_ref().is_some_and(|root| root.len != 0) {
            self.child_content_retirements.insert_admitted(generation, ChildContentRetirement::new(previous, false));
        }
        self.child_content_generation = generation;
        Ok(())
    }

    /// 📮️ Sends one member's events on the document's backbone under that member's lane: the owner-path text of the
    /// member that owns it (empty for the document itself) and its step there (design §21.9). A member whose owner is
    /// not live has no lane: its events are refused here, never sent under another lane and never dropped.
    pub async fn send_member_lane<A: ArtifactApp>(&mut self, stores: &mut InstanceStores<A>, lane: &MemberKey, payload: Vec<u8>) -> Result<(), Fault> {
        if payload.is_empty() || stores.store.backbone_ref().is_none() {
            return Ok(());
        }
        let Some(owner) = self.children.path_of(lane.borrowed()).map(|path| path.owner_text()) else {
            return Err(plugin_sdk_fault(format!("the events of member {}/{} of member {} have no lane on the backbone", lane.slot, lane.child_id, lane.owner)));
        };
        stores.store.send_member_mutations(&owner, &lane.slot, &lane.child_id, payload).await.map_err(|error| error.into_fault())
    }

    pub(crate) async fn admit_child_member<A: ArtifactApp>(&mut self, stores: &mut InstanceStores<A>, slot: String, child_id: String, dialect: ArtifactDialect) -> Result<ChildStoreAdmission, Fault> {
        let expected = stores.declared_child_reference(&slot, &child_id, &dialect)?;
        let root_index = self.child_content_root.admit_member(MemberKeyRef::root(&slot, &child_id))?;
        let parent_dialect: ArtifactDialect = A::DIALECT.into();
        if stores.store.envelope().dialect.as_ref() != Some(&parent_dialect) {
            return Err(plugin_sdk_fault("owned member admission requires the parent's exact declared dialect"));
        }
        let generation = self.admit_child_content_publication()?;
        let parent_id = stores.store.envelope().id.clone();
        let graph = self.composition.graph_mut().await.admit_owns(&parent_id, &slot, &expected.artifact_id).await.map_err(|error| plugin_sdk_fault(error.to_string()))?;
        let owner = store::OwnerRef { parent: ArtifactRef { artifact_id: parent_id, dialect: parent_dialect }, slot: slot.clone(), child_id: child_id.clone() };
        let key = MemberKey::root(slot, child_id);
        let member = self.children.admit_identity(key.borrowed())?;
        Ok(ChildStoreAdmission { key, expected, owner, member, graph, root_index, generation })
    }

    pub(crate) async fn prepare_child_member(&self, admission: &ChildStoreAdmission, member: &mut M, restored: bool, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(ChildContentView, Option<usize>), Fault> {
        let member_identity=member.member_identity().ok_or_else(||plugin_sdk_fault("admitted child lacks its original declared dialect"))?;
        if member_identity.artifact_id!=admission.expected.artifact_id||member_identity.dialect!=&admission.expected.dialect||member_identity.owner!=Some(&admission.owner) {
            return Err(plugin_sdk_fault("child member does not match its exact admitted identity and ownership"));
        }
        let pin_index = self.pending_child_pins.iter().position(|pin| pin.child_ref.artifact_id == admission.expected.artifact_id);
        if let Some(index) = pin_index {
            if !restored {
                return Err(plugin_sdk_fault("direct child registration cannot consume a queued restore checkpoint"));
            }
            let pin = &self.pending_child_pins[index];
            if pin.child_ref != admission.expected {
                return Err(plugin_sdk_fault("pending child checkpoint belongs to a different exact dialect"));
            }
            let alternative = member.current_alternative_id().await.ok_or_else(|| plugin_sdk_fault("pending child checkpoint has no live alternative authority"))?;
            member.checkout(&pin.checkpoint_id, &alternative, identity).await.map_err(|error| plugin_sdk_fault(error.to_string()))?;
        }
        let root = self.child_content_root.capture_member_admitted(admission.root_index, admission.key.borrowed(), &admission.expected, member).await?;
        Ok((root, pin_index))
    }

    pub(crate) async fn commit_child_member(&mut self, admission: ChildStoreAdmission, member: M, root: ChildContentView, pin_index: Option<usize>) {
        self.composition.graph_mut().await.commit_owns_admitted(admission.graph);
        self.children.insert_admitted(admission.member, admission.expected, admission.owner, member);
        let previous = std::mem::replace(&mut *self.child_content_root, root);
        if !previous.is_empty() {
            self.child_content_retirements.insert_admitted(admission.generation, ChildContentRetirement::new(previous, false));
        }
        self.child_content_generation = admission.generation;
        if let Some(index) = pin_index {
            self.pending_child_pins.remove(index);
        }
    }

    #[cfg(test)]
    pub fn test_child_admission_state(&self, abort_generation: u64) -> ChildAdmissionTestState {
        ChildAdmissionTestState {
            children_empty: self.children.is_empty(),
            generation: self.child_content_generation,
            abort_generation: self.child_member_retirement_generation,
            abort_empty: self.child_member_retirements.is_empty(),
            requested_abort_retained: self.child_member_retirements.get(abort_generation).is_some(),
            content_empty: self.child_content_root.is_empty(),
            roots_retiring_empty: self.child_content_retirements.is_empty(),
        }
    }

    pub fn child_member_retirement_quote(&self,body:usize,closing:bool)->Result<Option<RetirementDemand>,ValueError>{
        let Some((_,generation))=self.child_member_retirements.next_id_from(self.child_member_retirement_cursor)else{let release_bytes=self.child_member_retirements.empty_backing_byte_demand().unwrap_or(0);return Ok((closing&&release_bytes!=0).then_some(RetirementDemand{copy_bytes:size_of_val(&self.child_member_retirements),release_bytes,depth:1,..Default::default()}))};let retirement=self.child_member_retirements.get(generation).unwrap();let parent_copy=size_of::<usize>();
        if retirement.terminal_is_empty(){return Ok(Some(RetirementDemand{copy_bytes:parent_copy+size_of::<Option<(u64,ChildMemberRetirement<M>)>>(),depth:1,..Default::default()}))}
        ChildContentRetirement::parent_snapshot_demand(retirement.retirement_demands(body.saturating_sub(parent_copy))?,parent_copy).map(Some)
    }

    pub fn child_member_retirement_unit(&mut self,grant:RetainedCloneGrant,closing:bool)->Result<RetainedCloneStep,Fault>{
        let empty=RetainedCloneProgress::default();let Some(demand)=self.child_member_retirement_quote(grant.maximum_copy_bytes, closing).map_err(|error|plugin_sdk_fault(error.into_message()))?else{return Ok(RetainedCloneStep::Progress(empty))};
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(empty))}
        let Some((index, generation)) = self.child_member_retirements.next_id_from(self.child_member_retirement_cursor) else {
            let backing=std::mem::take(&mut self.child_member_retirements.slots);self.child_member_retirements.allocation_admitted=false;drop(backing);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..empty}));
        };
        let retirement = self.child_member_retirements.get_mut(generation).expect("exact child-member retirement remains admitted");
        if retirement.entry.as_ref().is_some_and(|entry| !entry.member.snapshot_read_leases_terminal_is_empty()) {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        if retirement.terminal_is_empty(){drop(self.child_member_retirements.remove(generation));self.child_member_retirement_cursor=(index+1)%ARTIFACT_LIVE_OUTPUT_SLOTS;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}))}
        let parent_copy=size_of::<usize>();let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent_copy,maximum_depth:grant.maximum_depth-1,..grant};let step=retirement.close_step(child)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,retirement.terminal_is_empty(),"original child member registry").map_err(|error|plugin_sdk_fault(error.into_message()))?;
        let mut receipt=step.progress();if receipt!=empty{self.child_member_retirement_cursor=(index+1)%ARTIFACT_LIVE_OUTPUT_SLOTS;receipt.copied_bytes=receipt.copied_bytes.checked_add(parent_copy).expect("original child registry cursor preflight")}
        let receipt=semio_framework_value::retained_clone::admit_retained_clone_progress(grant,receipt,"original child member registry parent").map_err(|error|plugin_sdk_fault(error.into_message()))?;Ok(RetainedCloneStep::Progress(receipt))
    }

    pub(crate) fn validate_parent_child_restore<A: ArtifactApp>(&self, stores: &InstanceStores<A>, slot: &str, child_id: &str, expected: &ArtifactRef) -> Result<(), Fault> {
        let parent_dialect: ArtifactDialect = A::DIALECT.into();
        if stores.store.envelope().dialect.as_ref() != Some(&parent_dialect) {
            return Err(plugin_sdk_fault("child restore requires the parent's exact declared dialect"));
        }
        let parent = ArtifactRef { artifact_id: stores.store.envelope().id.clone(), dialect: parent_dialect };
        let source = DocumentClosureSourceView::new(stores.store.generation_now(), &parent, stores.store.snapshot_ref(), &self.children);
        let projection = store::OwnedDocumentClosureSource::child_projection(&source, None).map_err(|error| plugin_sdk_fault(format!("child restore projection failed: {error:?}")))?;
        if !(0..projection.len()).filter_map(|index| projection.get(index)).any(|(declared, fields)| declared == slot && fields.child_id == child_id && fields.artifact_id == expected.artifact_id && fields.artifact_kind == expected.dialect.artifact_kind && fields.standard == expected.dialect.standard && fields.subset == expected.dialect.subset) {
            return Err(plugin_sdk_fault("child restore is not declared by the loaded parent snapshot"));
        }
        Ok(())
    }

    /// 🌱️ Restores one exact owned member and publishes its graph, map and snapshot together.
    pub async fn open_child<A: ArtifactApp>(&mut self, stores: &mut InstanceStores<A>, slot: impl Into<String>, child_id: impl Into<String>, dialect: ArtifactDialect, envelope_pack: &[u8], identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        let slot = slot.into();
        let child_id = child_id.into();
        let expected = stores.declared_child_reference(&slot, &child_id, &dialect)?;
        self.validate_parent_child_restore(stores, &slot, &child_id, &expected)?;
        let parent_generation = stores.store.generation_now();
        let abort_generation = self.child_member_retirement_generation.checked_add(1).ok_or_else(|| plugin_sdk_fault("failed child retirement generation exhausted"))?;
        if !self.reclaim_child_retirements(0, 1)? {
            return Err(plugin_sdk_fault("failed child retirement waits on retirements that are all still borrowed").with_retryable(true));
        }
        let admission = self.admit_child_member(stores, slot, child_id, dialect).await?;
        let mut member = match M::open(&admission.expected, Some(&admission.owner), envelope_pack, stores.store.local_actor_id().clone()).await {
            Ok(member) => member,
            Err(error) => {
                assert!(self.children.cancel_admission(&admission.member), "failed child open retains its exact admission until cancellation");
                return Err(plugin_sdk_fault(error.to_string()));
            }
        };
        let prepared = if stores.store.generation_now() != parent_generation {
            Err(plugin_sdk_fault("loaded parent changed during child restore"))
        } else {
            match self.validate_parent_child_restore(stores, &admission.key.slot, &admission.key.child_id, &admission.expected) {
                Ok(()) => self.prepare_child_member(&admission, &mut member, true, identity).await,
                Err(fault) => Err(fault),
            }
        };
        let (root, pin_index) = match prepared {
            Ok(prepared) => prepared,
            Err(fault) => {
                assert!(self.children.cancel_admission(&admission.member), "failed child preparation returns its exact map admission");
                self.child_member_retirements.insert_admitted(abort_generation, ChildMemberRetirement::new(ChildMemberEntry { reference: admission.expected, owner: admission.owner, member }));
                self.child_member_retirement_generation = abort_generation;
                return Err(fault);
            }
        };
        self.commit_child_member(admission, member, root, pin_index).await;
        Ok(())
    }

    /// 🔎️ The live child store for `(slot, child_id)`, if adopted/created — the read half of
    /// `open_child`/`register_child`.
    pub async fn child_store(&self, slot: &str, child_id: &str) -> Option<&M> {
        self.children.get(&(slot.to_string(), child_id.to_string())).map(|entry| &entry.member)
    }

    /// 📣️ Announces every live composed member's whole event log on the PARENT's backbone
    /// under that member's own `(slot, child_id)` lane. A composed document owns exactly one
    /// replica endpoint — `ArtifactStore::announce_history` covers the parent lane alone, so
    /// without this a joining replica never learns a single child-lane edit and the two replicas
    /// diverge on the very content a composing plugin keeps in its children (measured on
    /// `🌊️flow`'s `content` child, ticket 26/09/19 `📓️flow.md` §5.3).
    pub async fn announce_member_event_logs<A: ArtifactApp>(&mut self, stores: &mut InstanceStores<A>) -> Result<(), Fault> {
        if self.children.is_empty() {
            return Ok(());
        }
        let lanes: Vec<MemberKey> = self.children.keyed_entries().map(|(key, _)| key.to_key()).collect();
        for lane in lanes {
            let Some(entry) = self.children.member_mut(lane.borrowed()) else { continue };
            let payload = ReferenceSpaceMember::event_log_payload(&entry.member).await.map_err(|error| error.into_fault())?;
            self.send_member_lane(stores, &lane, payload).await?;
        }
        Ok(())
    }

    /// 📤️ Announces the tail edit of every member a composite gesture just touched, under
    /// that member's own lane. Called from the ONE choke point child edits land at
    /// (`dispatch_emit_group`), so a child-lane edit travels exactly like a parent-lane one.
    pub async fn announce_member_tail_edits<A: ArtifactApp>(&mut self, stores: &mut InstanceStores<A>, lanes: &[(String, String)]) -> Result<(), Fault> {
        if stores.store.backbone_ref().is_none() {
            return Ok(());
        }
        for (slot, child_id) in lanes {
            let Some(entry) = self.children.get_mut(&(slot.clone(), child_id.clone())) else { continue };
            let payload = ReferenceSpaceMember::announce_tail_edit_payload(&mut entry.member).await.map_err(|error| error.into_fault())?;
            stores.store.send_member_mutations("", slot, child_id, payload).await.map_err(|error| error.into_fault())?;
        }
        Ok(())
    }

    /// 📥️ Routes received member envelopes to the exact declared child owner.
    pub async fn fold_member_inbound<A: ArtifactApp>(&mut self, stores: &mut InstanceStores<A>, history: &mut CommandHistory<A>) -> Result<usize, Fault> {
        let inbound = stores.store.take_member_inbound();
        if inbound.is_empty() {
            return Ok(0);
        }
        let mut folded = 0usize;
        for (owner, slot, child_id, envelopes) in inbound {
            let lane = MemberPath::under(&owner, slot.as_str(), child_id.as_str()).and_then(|path| self.children.resolve(&path).map(|(key, _)| key));
            let entry = match lane.as_ref() {
                Some(key) => self.children.member_mut(key.borrowed()),
                None => None,
            };
            let Some(entry) = entry else {
                let declared = if owner.is_empty() {
                    let projection = semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(stores.store.snapshot_ref()).map_err(|error| plugin_sdk_fault(format!("member lane projection failed: {error}")))?;
                    (0..projection.len()).filter_map(|index| projection.get(index)).any(|(declared, fields)| declared == slot.as_str() && fields.child_id == child_id.as_str())
                } else {
                    match MemberPath::parse(&owner).and_then(|path| self.children.resolve(&path)) {
                        Some((_, owning)) => {
                            let projection = owning.member.child_restore_projection().map_err(|error| plugin_sdk_fault(format!("member lane projection failed: {error}")))?;
                            (0..projection.len()).filter_map(|index| projection.get(index)).any(|(declared, fields)| declared == slot.as_str() && fields.child_id == child_id.as_str())
                        }
                        None => false,
                    }
                };
                if declared && !self.derivable_follow_awaits_retirement(stores) {
                    let lane = if owner.is_empty() { format!("{slot}/{child_id}") } else { format!("{owner}/{slot}/{child_id}") };
                    return Err(plugin_sdk_fault(format!("backbone member lane {lane} names no live composed member of this replica")));
                }
                continue;
            };
            ReferenceSpaceMember::ingest_remote_payload(&mut entry.member, &envelopes).await.map_err(|error| error.into_fault())?;
            folded += 1;
        }
        history.cache = None;
        Ok(folded)
    }

    /// 🌱️ Adopts an already-live `M` member into the child-store map directly — the
    /// counterpart `dispatch_emit_group` uses to absorb a `store::os_store::composition_coordinator::GroupReceipt::created_children`
    /// entry (a `ChildGenesis`-minted member has no prior caller-held reference an `open_child`
    /// call could have produced it from) and the general escape hatch for a caller that already
    /// holds a constructed member from elsewhere (e.g. a host-level `SpaceHost`).
    /// 🌱️ Adopts an already-built child store under `(slot, child_id)`. Like [`Self::open_child`]
    /// it MUST seed the ownership edge into `self.composition`'s graph: `dispatch_group`'s phase 1
    /// ownership check (`CompositionGraph::owner_of`) is fail-closed, so a child present in
    /// `self.children` but absent from the graph is rejected as an `OwnershipViolation` the first
    /// time anything dispatches against it. Registering in one place and validating from the other
    /// is precisely the inconsistency this seeds away.
    pub async fn register_child<A: ArtifactApp>(&mut self, stores: &mut InstanceStores<A>, slot: impl Into<String>, child_id: impl Into<String>, dialect: ArtifactDialect, mut member: M, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), ChildMemberRegistrationError<M>> {
        let admission = match self.admit_child_member(stores, slot.into(), child_id.into(), dialect).await {
            Ok(admission) => admission,
            Err(fault) => return Err(ChildMemberRegistrationError { member: Some(member), fault }),
        };
        let (root, pin_index) = match self.prepare_child_member(&admission, &mut member, false, identity).await {
            Ok(prepared) => prepared,
            Err(fault) => {
                assert!(self.children.cancel_admission(&admission.member), "failed child preparation returns its exact map admission");
                return Err(ChildMemberRegistrationError { member: Some(member), fault });
            }
        };
        self.commit_child_member(admission, member, root, pin_index).await;
        Ok(())
    }

    /// 📌️ Leaves-first half of a composing document's checkpoint: commits every DIRTY
    /// child (a clean child is already pinned at its current checkpoint, and committing it again
    /// would mint an empty checkpoint per parent commit) and returns the `CompositionPin`s the
    /// parent's own about-to-be-created checkpoint must carry.
    ///
    /// A child with no checkpoint at all after committing contributes no pin rather than
    /// aborting the parent's checkpoint: a pin that named nothing would be worse than an absent
    /// one, and the parent's history is still perfectly valid without it.
    /// 📌️ One checkpoint-cascade unit: commits the child at `slot`/`child_id` when it is dirty,
    /// republishes its content root, and answers the pin its current checkpoint contributes.
    pub async fn checkpoint_child_unit(&mut self, message: &str, authors: &[vcs::Author], key: MemberKeyRef<'_>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<Option<vcs::CompositionPin>, Fault> {
        let publication_generation = self.admit_child_content_publication()?;
        let (reference, checkpoint_id) = {
            let entry = self.children.member_mut(key).ok_or_else(|| plugin_sdk_fault("checkpoint child authority changed during bounded publication"))?;
            if entry.member.is_dirty().await {
                entry.member.commit_checkpoint(message.to_string(), authors.to_vec(), identity).await.map_err(|error| error.into_fault())?;
            }
            (entry.reference.clone(), entry.member.current_checkpoint_id().await)
        };
        self.publish_member_content(publication_generation, key).await?;
        Ok(checkpoint_id.map(|checkpoint_id| vcs::CompositionPin { child_ref: reference, checkpoint_id }))
    }

    /// ⏮️ Checkout half of the cascade: the pins the parent's now-current checkpoint recorded,
    /// each restored by one [`Self::checkout_child_unit`]. A pin naming a child that is not currently
    /// open is QUEUED (`pending_child_pins`) rather than dropped, so a child adopted later still lands
    /// on its pinned state instead of silently staying at head — see `open_child`.
    pub fn checkout_cascade_pins<A: ArtifactApp>(&mut self, stores: &mut InstanceStores<A>) -> Result<Vec<vcs::CompositionPin>, Fault> {
        let Some(checkpoint_id) = stores.store.current_checkpoint_id().map(str::to_string) else { return Ok(Vec::new()) };
        let Some(pins) = stores.store.envelope().vcs.checkpoints.iter().find(|checkpoint| checkpoint.id == checkpoint_id).map(|checkpoint| checkpoint.composition_pins.clone()) else { return Ok(Vec::new()) };
        self.pending_child_pins.clear();
        self.admit_child_content_publication_span(pins.len())?;
        Ok(pins)
    }

    /// ⏮️ One checkout-cascade unit: restores one live child to `pin`, or queues the pin.
    pub async fn checkout_child_unit(&mut self, pin: &vcs::CompositionPin, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        let child_key = self.children.keyed_entries().find(|(_, entry)| entry.reference.artifact_id == pin.child_ref.artifact_id).map(|(key, _)| key.to_key());
        let Some(key) = child_key else {
            self.pending_child_pins.push(pin.clone());
            return Ok(());
        };
        let publication_generation = self.admit_child_content_publication()?;
        let entry = self.children.member_mut(key.borrowed()).ok_or_else(|| plugin_sdk_fault("checkout child authority changed during bounded publication"))?;
        let alternative_id = entry.member.current_alternative_id().await.unwrap_or_default();
        let _ = entry.member.checkout(&pin.checkpoint_id, &alternative_id, identity).await;
        self.publish_member_content(publication_generation, key.borrowed()).await
    }
}
