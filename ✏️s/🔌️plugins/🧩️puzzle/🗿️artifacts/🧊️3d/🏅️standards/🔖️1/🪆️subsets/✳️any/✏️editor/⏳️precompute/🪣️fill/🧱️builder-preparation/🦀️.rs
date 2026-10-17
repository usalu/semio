//! 🧱️ Builder preparation: `begin_preparation`, the close accounting and the cooperative preparation transitions that install the scene, catalogs, meshes, entries, spatial index and lookups.

use super::*;

impl FillBuilder {
    #[cfg(any(test, feature = "testing"))]
    pub fn preparation_refusal_owner_for_test(&self) -> Option<(&'static str, usize, String, Option<f64>)> {
        let refusal = self.preparation_capacity_refusal?;
        let roots = self.preparation_roots.as_ref()?;
        let (owner, weight) = match refusal.branch {
            PreparationCapacityBranch::EngineSceneObjects => (roots.scene.scene_snapshot.objects.get(refusal.omitted_index)?.id.clone(), None),
            PreparationCapacityBranch::SceneAttractions => (roots.scene.scene_snapshot.attractions.get(refusal.omitted_index)?.id.clone(), None),
            PreparationCapacityBranch::SceneTargetVolumes => (roots.scene.scene_snapshot.target_volumes.get(refusal.omitted_index)?.id.clone(), None),
            PreparationCapacityBranch::Meshes => (roots.meshes.keys().nth(refusal.omitted_index)?.clone(), None),
            PreparationCapacityBranch::CatalogObjects => (roots.scene.kind_catalogs.as_ref()?.objects.get(refusal.omitted_index)?.id.clone(), None),
            PreparationCapacityBranch::CatalogVortices => (roots.scene.kind_catalogs.as_ref()?.vortices.get(refusal.omitted_index)?.id.clone(), None),
            PreparationCapacityBranch::CatalogCables => (roots.scene.kind_catalogs.as_ref()?.cables.get(refusal.omitted_index)?.id.clone(), None),
            PreparationCapacityBranch::KindCompatibility => (roots.scene.kind_compatibility.get(refusal.omitted_index)?.source.clone(), None),
            PreparationCapacityBranch::ObjectWeights => {
                let (key, value) = roots.scene.weights.object_weights.iter().nth(refusal.omitted_index)?;
                (key.clone(), Some(*value))
            }
            PreparationCapacityBranch::VortexWeights => {
                let (key, value) = roots.scene.weights.vortex_weights.iter().nth(refusal.omitted_index)?;
                (key.clone(), Some(*value))
            }
        };
        Some((refusal.branch.label(), refusal.omitted_index, owner, weight))
    }

    /// 🔎️ The FIRST retained-owner clause this builder still fails, BY NAME. A close ladder that wedges
    /// has to be able to say which owner nothing drains, instead of only that the builder is "not empty":
    /// the close walk's terminal arm returns `false` forever in that case, so the whole teardown spins
    /// without progress (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B42).
    pub fn terminal_owner_debt(&self) -> Option<&'static str> {
        [
            ("scene_terminal_owners_empty base", scene_terminal_owners_empty(&self.base)),
            ("sequence.is_empty", self.sequence.is_empty()),
            ("sequence.capacity 0", self.sequence.capacity() == 0),
            ("appended_objects.is_empty", self.appended_objects.is_empty()),
            ("appended_objects.capacity 0", self.appended_objects.capacity() == 0),
            ("appended_attractions.is_empty", self.appended_attractions.is_empty()),
            ("appended_attractions.capacity 0", self.appended_attractions.capacity() == 0),
            ("placed.is_empty", self.placed.is_empty()),
            ("placed.capacity 0", self.placed.capacity() == 0),
            ("placed_lookup.is_empty", self.placed_lookup.is_empty()),
            ("candidate_cache.is_empty", self.candidate_cache.is_empty()),
            ("catalogs.objects.terminal_owners_empty", self.catalogs.objects.terminal_owners_empty()),
            ("catalogs.vortices.terminal_owners_empty", self.catalogs.vortices.terminal_owners_empty()),
            ("catalogs.cables.terminal_owners_empty", self.catalogs.cables.terminal_owners_empty()),
            ("weights.object_weights.is_empty", self.weights.object_weights.is_empty()),
            ("weights.vortex_weights.is_empty", self.weights.vortex_weights.is_empty()),
            ("kind_compatibility.terminal_owners_empty", self.kind_compatibility.terminal_owners_empty()),
            ("meshes.is_empty", self.meshes.is_empty()),
            ("spatial_index.terminal_owners_empty", self.spatial_index.terminal_owners_empty()),
            ("targets.is_empty", self.targets.is_empty()),
            ("targets.capacity 0", self.targets.capacity() == 0),
            ("blocked_vortex_ids.is_empty", self.blocked_vortex_ids.is_empty()),
            ("target_weights.is_empty", self.target_weights.is_empty()),
            ("target_weights.capacity 0", self.target_weights.capacity() == 0),
            ("target_tree.is_empty", self.target_tree.is_empty()),
            ("target_tree.capacity 0", self.target_tree.capacity() == 0),
            ("current_target.is_none", self.current_target.is_none()),
            ("candidates.is_empty", self.candidates.is_empty()),
            ("candidates.capacity 0", self.candidates.capacity() == 0),
            ("candidate_seen.is_empty", self.candidate_seen.is_empty()),
            ("candidate_raw.is_empty", self.candidate_raw.is_empty()),
            ("candidate_raw.capacity 0", self.candidate_raw.capacity() == 0),
            ("candidate_weights.is_empty", self.candidate_weights.is_empty()),
            ("candidate_weights.capacity 0", self.candidate_weights.capacity() == 0),
            ("candidate_tree.is_empty", self.candidate_tree.is_empty()),
            ("candidate_tree.capacity 0", self.candidate_tree.capacity() == 0),
            ("current_preview.is_none", self.current_preview.is_none()),
            ("broad_phase_query.as_ref .is_none_or CollisionQueryCursor terminal_owners_empty", self.broad_phase_query.as_ref().is_none_or(CollisionQueryCursor::terminal_owners_empty)),
            ("collision.is_none", self.collision.is_none()),
            ("pending_payload.is_none", self.pending_payload.is_none()),
            ("pending_object.is_none", self.pending_object.is_none()),
            ("pending_attraction.is_none", self.pending_attraction.is_none()),
            ("pending_spatial.is_none", self.pending_spatial.is_none()),
            ("preparation_spatial.is_none", self.preparation_spatial.is_none()),
            ("tail_removal.is_none", self.tail_removal.is_none()),
            ("preparation_roots.is_none", self.preparation_roots.is_none()),
            ("preparation_capacity_refusal.is_none", self.preparation_capacity_refusal.is_none()),
            ("last_rejection.is_none", self.last_rejection.is_none()),
            ("fixed_rejection.is_none", self.fixed_rejection.is_none()),
            ("collection_over_capacity", !self.collection_over_capacity),
            ("placed_lookup.terminal_owners_empty", self.placed_lookup.terminal_owners_empty()),
            ("candidate_cache.terminal_owners_empty", self.candidate_cache.terminal_owners_empty()),
            ("weights.object_weights.terminal_owners_empty", self.weights.object_weights.terminal_owners_empty()),
            ("weights.vortex_weights.terminal_owners_empty", self.weights.vortex_weights.terminal_owners_empty()),
            ("meshes.terminal_owners_empty", self.meshes.terminal_owners_empty()),
            ("blocked_vortex_ids.terminal_owners_empty", self.blocked_vortex_ids.terminal_owners_empty()),
            ("candidate_seen.terminal_owners_empty", self.candidate_seen.terminal_owners_empty()),
            ("run_events.is_none", self.run_events.is_none()),
        ]
        .into_iter()
        .find_map(|(owner, empty)| (!empty).then_some(owner))
    }

    pub fn terminal_owners_empty(&self) -> bool {
        self.terminal_owner_debt().is_none()
    }

    /// 🎯️ Seeds a planner that targets exactly what was requested — there is no hidden ceiling it
    /// races toward in the background, so every placement the user sees was asked for.
    pub fn begin_preparation(roots: FillPreparationRoots, operation: Operation, requested_count: usize) -> Self {
        let seed = roots.scene.seed;
        let preparation_capacity_refusal = preparation_capacity_refusal(&roots);
        Self {
            base: FixedSceneOwner::new(),
            preparation_roots: Some(roots),
            preparation_cursor: 0,
            preparation_inner_cursor: 0,
            preparation_spatial: None,
            tail_removal: None,
            preparation_capacity_refusal,
            applied_count: 0,
            sequence: Vec::new(),
            appended_objects: Vec::new(),
            appended_attractions: Vec::new(),
            placed: Vec::new(),
            placed_lookup: FixedOwnerMap::new(),
            candidate_cache: FixedOwnerMap::new(),
            rng_state: seed,
            max_count: requested_count,
            tail_floor: 0,
            ever_constructed: false,
            operation,
            stage: FillJobStage::PrepareScene,
            catalogs: FixedCatalogOwner::new(),
            weights: RetainedBrushKindWeights::new(),
            kind_compatibility: FixedOwnerVec::new(),
            host_rules: BrushHostRules::default(),
            contact_tolerance: 0.0,
            meshes: FixedOwnerMap::new(),
            spatial_index: CollisionSpatialIndex::new(8.0),
            targets: Vec::new(),
            target_weights: Vec::new(),
            target_tree: vec![0.0],
            targets_ready: false,
            current_target_index: None,
            target_prepare_phase: TargetPreparePhase::Reset,
            blocked_vortex_ids: FixedOwnerSet::new(),
            target_attraction_cursor: 0,
            target_object_cursor: 0,
            target_vortex_cursor: 0,
            current_target: None,
            candidates: Vec::new(),
            candidate_cursor: 0,
            candidate_prepare_phase: CandidatePreparePhase::Reset,
            candidate_kind_cursor: 0,
            candidate_vortex_cursor: 0,
            candidate_prepare_cursor: 0,
            candidate_seen: FixedOwnerSet::new(),
            candidate_raw: Vec::new(),
            candidate_weights: Vec::new(),
            candidate_tree: vec![0.0],
            candidate_remaining: 0,
            current_preview: None,
            broad_phase_query: None,
            broad_phase_cursor: 0,
            broad_phase_bounds: None,
            collision: None,
            accept_phase: AcceptPhase::Validate,
            accept_attraction_cursor: 0,
            accept_vortex_cursor: 0,
            pending_payload: None,
            pending_object: None,
            pending_attraction: None,
            pending_spatial: None,
            last_rejection: None,
            fixed_rejection: None,
            collection_over_capacity: false,
            transition_count: 0,
            rejected_count: 0,
            close_field: 0,
            close_current: None,
            closing: false,
            run_events: None,
            run_candidate_live: false,
        }
    }

    pub(super) fn retire_one_close_owner(&mut self) -> bool {
        if let Some(current) = self.close_current.as_mut() {
            if retire_retained_owner(current) {
                self.close_current = None;
            }
            return false;
        }
        let mut current = None;
        let retired = match self.close_field {
            0 => take_scene_owner(&mut self.base, &mut current),
            1 => false,
            2 => take_sequence_owner(self, &mut current),
            3 => take_lookup_owner(self, &mut current),
            4 => take_catalog_owner(self, &mut current),
            5 => take_weight_mesh_owner(self, &mut current),
            6 => take_target_owner(self, &mut current),
            7 => take_target_weight_owner(self),
            8 => take_candidate_owner(self, &mut current),
            9 => take_candidate_order_owner(self),
            10 => match self.broad_phase_query.as_mut() {
                Some(query) => {
                    if query.retire_one_owner() {
                        self.broad_phase_query.take();
                    }
                    true
                }
                None => false,
            },
            11 => self.pending_payload.take().is_some_and(|value| {
                current = Some(FillRetiredOwner::Payload(value));
                true
            }),
            12 => self.pending_object.take().is_some_and(|value| {
                current = Some(FillRetiredOwner::EngineSceneObject(value));
                true
            }),
            13 => self.pending_attraction.take().is_some_and(|value| {
                current = Some(FillRetiredOwner::Attraction(value));
                true
            }),
            14 => self.current_target.take().is_some_and(|value| {
                current = Some(FillRetiredOwner::Target(value));
                true
            }),
            15 => self.current_preview.take().is_some_and(|value| {
                current = Some(FillRetiredOwner::PreviewState(value));
                true
            }),
            16 => self.last_rejection.take().is_some_and(|value| {
                current = Some(FillRetiredOwner::String(value));
                true
            }),
            17 => self.collision.take().is_some(),
            18 => match self.fixed_rejection.as_mut() {
                Some(rejected) => {
                    if retire_retained_owner(rejected) {
                        self.fixed_rejection.take();
                    }
                    true
                }
                None => false,
            },
            19 => retire_fixed_collection_backing(self),
            20 => !self.spatial_index.retire_one_owner(),
            21 if self.collection_over_capacity => {
                self.collection_over_capacity = false;
                true
            }
            21 => false,
            22 => match self.preparation_spatial.as_mut() {
                Some(mutation) => {
                    if mutation.retire_one_owner() {
                        self.preparation_spatial.take();
                    }
                    true
                }
                None => false,
            },
            23 => match self.pending_spatial.as_mut() {
                Some(mutation) => {
                    if mutation.retire_one_owner() {
                        self.pending_spatial.take();
                    }
                    true
                }
                None => false,
            },
            24 => match self.tail_removal.as_mut() {
                Some(removal) => {
                    if removal.retire_one_owner() {
                        self.tail_removal.take();
                    }
                    true
                }
                None => false,
            },
            25 if self.preparation_roots.take().is_some() => true,
            25 => false,
            26 if self.preparation_capacity_refusal.take().is_some() => true,
            26 => false,
            27 if self.run_events.take().is_some() => true,
            27 => false,
            _ => return self.terminal_owners_empty() && self.close_current.is_none(),
        };
        self.close_current = current;
        if !retired {
            self.close_field = self.close_field.saturating_add(1);
        }
        false
    }

    pub(super) fn collision_owner(&self) -> CollisionIndexOwner {
        CollisionIndexOwner { operation: self.operation.operation.0, generation: self.operation.generation.0 }
    }

    pub(super) fn scene_object(&self, index: usize) -> Option<&EngineSceneObject> {
        self.base.objects.get(index).or_else(|| self.appended_objects.get(index.saturating_sub(self.base.objects.len())))
    }

    pub(super) fn scene_attraction(&self, index: usize) -> Option<&AttractionProps> {
        self.base.attractions.get(index).or_else(|| self.appended_attractions.get(index.saturating_sub(self.base.attractions.len())))
    }

    pub(super) fn scene_view(&self) -> FillSceneView<'_> {
        FillSceneView { base: &self.base, appended: &self.appended_objects }
    }

    pub(super) fn label_catalog_snapshot(&self) -> Puzzle3dSceneSnapshot {
        let mut scene_snapshot = empty_scene_snapshot();
        if let Some(roots) = self.preparation_roots.as_ref() {
            if let Some(catalogs) = roots.scene.kind_catalogs.as_ref() {
                scene_snapshot.meta.kind_catalogs = Some(semio_framework_value::ToValue::to_value(catalogs));
            }
        }
        scene_snapshot
    }

    pub(super) fn label_peers(&self) -> Vec<Puzzle3dObject> {
        let mut peers = Vec::new();
        for object in self.base.objects.iter() {
            if let Ok(peer) = semio_framework_value::FromValue::from_value(semio_framework_value::ToValue::to_value(object)) {
                peers.push(peer);
            }
        }
        for object in self.appended_objects.iter().take(self.appended_objects.len().saturating_sub(1)) {
            if let Ok(peer) = semio_framework_value::FromValue::from_value(semio_framework_value::ToValue::to_value(object)) {
                peers.push(peer);
            }
        }
        peers
    }

    /// 🏁️ How the plan ended, `None` while it still runs.
    pub fn end(&self) -> Option<FillPlanEnd> {
        match self.stage {
            FillJobStage::Complete(end) => Some(end),
            _ => None,
        }
    }

    /// 🔁️ Where a round goes after it placed or retracted: complete as reached once the plan holds what was asked for,
    /// else the next vortex draw — from the resident pool, or after rebuilding it when a retraction dropped it.
    pub(super) fn next_round_stage(&self) -> FillJobStage {
        if self.sequence.len() >= self.max_count {
            FillJobStage::Complete(FillPlanEnd::Reached)
        } else if self.targets_ready {
            FillJobStage::SelectTarget
        } else {
            FillJobStage::PrepareTargets
        }
    }

    /// 🎯️ What the user is asking for right now.
    pub fn requested_count(&self) -> usize {
        self.max_count
    }

    /// 🎚️ Retargets the live plan without ever rewinding its RNG stream. Raising simply lifts the
    /// ceiling and wakes a planner that had reached it, so the longer plan keeps the shorter one as
    /// its exact prefix. Lowering hands the surplus tail back through the same one-placement-per-turn
    /// [`FillJobStage::RetractTail`] walk that a weight replan uses, stopping at whatever is already
    /// applied to the document.
    pub fn set_requested_count(&mut self, requested: usize) {
        if requested == self.max_count {
            return;
        }
        let raising = requested > self.max_count;
        self.max_count = requested;
        if raising {
            if matches!(self.stage, FillJobStage::Complete(_)) && self.sequence.len() < self.max_count {
                self.stage = self.next_round_stage();
            }
            return;
        }
        if self.sequence.len() <= self.max_count {
            return;
        }
        self.abandon_run_candidate();
        self.applied_count = self.applied_count.min(self.sequence.len());
        self.tail_floor = self.max_count.max(self.applied_count);
        self.stage = FillJobStage::RetractTail;
    }

    /// ♻️ One unapplied placement per turn: withdraw its spatial owner, drop its lookup and placement
    /// rows, then pop the plan row itself. Reaching the applied prefix rewinds the planner.
    pub(super) fn discard_tail_one(&mut self) -> Result<(), StaleSpatialIndex> {
        let owner = self.collision_owner();
        if let Some(removal) = self.tail_removal.as_mut() {
            match self.spatial_index.step_removal(removal, owner) {
                CollisionMutationStep::Pending => {}
                CollisionMutationStep::Complete => self.tail_removal = None,
                CollisionMutationStep::Rejected(rejected) => {
                    self.fixed_rejection = Some(FillRetiredOwner::Spatial(rejected));
                    self.collection_over_capacity = true;
                }
                CollisionMutationStep::Stale => return Err(StaleSpatialIndex),
            }
            return Ok(());
        }
        if self.appended_objects.len() <= self.tail_floor {
            self.reset_targets();
            self.reset_candidate();
            self.stage = self.next_round_stage();
            return Ok(());
        }
        let Some(object) = self.appended_objects.pop() else {
            self.reset_targets();
            self.stage = FillJobStage::PrepareTargets;
            return Ok(());
        };
        self.sequence.pop();
        self.appended_attractions.pop();
        self.run_event(FillRunEvent::Discarded);
        if let Some((id, index)) = self.placed_lookup.remove_entry(object.id.as_str()) {
            if index + 1 == self.placed.len() {
                self.placed.pop();
            }
            drop(id);
        }
        self.tail_removal = self.spatial_index.begin_removal(owner, object.id.clone());
        drop(object);
        Ok(())
    }

    pub fn prepare_one(&mut self) -> Result<(), StaleSpatialIndex> {
        if self.collection_over_capacity {
            self.last_rejection = Some("preparation-capacity".into());
            self.stall(FillStall::ArtifactCapacity);
            return Ok(());
        }
        match self.stage {
            FillJobStage::PrepareScene => self.prepare_scene_one(),
            FillJobStage::PrepareCatalogs => self.prepare_catalog_one(),
            FillJobStage::PrepareMeshes => self.prepare_mesh_one(),
            FillJobStage::PrepareEntries => self.prepare_entry_one(),
            FillJobStage::PrepareSpatial => return self.prepare_spatial_one(),
            FillJobStage::PrepareLookup => self.prepare_lookup_one(),
            FillJobStage::PrepareConfiguration => self.prepare_configuration_one(),
            _ => {}
        }
        Ok(())
    }

    fn prepare_scene_one(&mut self) {
        let roots = self.preparation_roots.as_ref().expect("preparation roots");
        let scene_snapshot = &roots.scene.scene_snapshot;
        let value = match self.preparation_inner_cursor {
            0 => scene_snapshot.attractions.get(self.preparation_cursor).map(|value| {
                if let Err(owner) = self.base.attractions.try_push(value.clone()) {
                    self.fixed_rejection = Some(FillRetiredOwner::Attraction(owner));
                }
            }),
            1 => scene_snapshot.objects.get(self.preparation_cursor).map(|value| {
                if let Err(owner) = self.base.objects.try_push(value.clone()) {
                    self.fixed_rejection = Some(FillRetiredOwner::EngineSceneObject(owner));
                }
            }),
            _ => scene_snapshot.target_volumes.get(self.preparation_cursor).map(|value| {
                if let Err(owner) = self.base.target_volumes.try_push(value.clone()) {
                    self.fixed_rejection = Some(FillRetiredOwner::WorldVolume(owner));
                }
            }),
        };
        if value.is_some() {
            self.preparation_cursor += 1;
            return;
        }
        self.preparation_cursor = 0;
        self.preparation_inner_cursor += 1;
        if self.preparation_inner_cursor == 3 {
            self.preparation_inner_cursor = 0;
            self.stage = FillJobStage::PrepareCatalogs;
        }
    }

    fn prepare_catalog_one(&mut self) {
        let roots = self.preparation_roots.as_ref().expect("preparation roots");
        let catalogs = roots.scene.kind_catalogs.as_ref();
        let value = match self.preparation_inner_cursor {
            0 => catalogs.and_then(|value| value.objects.get(self.preparation_cursor)).map(|value| {
                if let Err(owner) = self.catalogs.objects.try_push(value.clone()) {
                    self.fixed_rejection = Some(FillRetiredOwner::ObjectKind(owner));
                }
            }),
            1 => catalogs.and_then(|value| value.vortices.get(self.preparation_cursor)).map(|value| {
                if let Err(owner) = self.catalogs.vortices.try_push(value.clone()) {
                    self.fixed_rejection = Some(FillRetiredOwner::VortexKind(owner));
                }
            }),
            _ => catalogs.and_then(|value| value.cables.get(self.preparation_cursor)).map(|value| {
                if let Err(owner) = self.catalogs.cables.try_push(value.clone()) {
                    self.fixed_rejection = Some(FillRetiredOwner::CableKind(owner));
                }
            }),
        };
        if value.is_some() {
            self.preparation_cursor += 1;
            return;
        }
        self.preparation_cursor = 0;
        self.preparation_inner_cursor += 1;
        if self.preparation_inner_cursor == 3 {
            self.preparation_inner_cursor = 0;
            self.stage = FillJobStage::PrepareMeshes;
        }
    }

    fn prepare_mesh_one(&mut self) {
        let roots = self.preparation_roots.as_ref().expect("preparation roots");
        let Some((url, body)) = roots.meshes.iter().nth(self.preparation_cursor) else {
            self.preparation_cursor = 0;
            self.stage = FillJobStage::PrepareEntries;
            return;
        };
        self.preparation_cursor += 1;
        match self.meshes.try_insert(url.clone(), body.clone()) {
            Ok(FixedOwnerMapInsert::Inserted) => {}
            Ok(FixedOwnerMapInsert::Occupied { input_key: url, input_value: body }) | Err((url, body)) => {
                self.fixed_rejection = Some(FillRetiredOwner::Mesh(url, body));
                self.collection_over_capacity = true;
            }
        }
    }

    fn prepare_entry_one(&mut self) {
        let Some(object) = self.base.objects.get(self.preparation_cursor) else {
            self.preparation_cursor = 0;
            self.stage = FillJobStage::PrepareSpatial;
            return;
        };
        self.preparation_cursor += 1;
        let scene_snapshot = FillSceneView { base: &self.base, appended: &self.appended_objects };
        let Some(mesh_url) = resolve_placed_object_mesh_url(object, &self.catalogs, &scene_snapshot) else {
            return;
        };
        if self.meshes.get(&mesh_url).is_none() {
            return;
        }
        self.placed.push(PlacedCollisionEntry { object_id: object.id.clone(), mesh_url, world: pose_isometry(object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &object.scale) });
    }

    fn prepare_spatial_one(&mut self) -> Result<(), StaleSpatialIndex> {
        let owner = self.collision_owner();
        if let Some(mutation) = self.preparation_spatial.as_mut() {
            match self.spatial_index.step_replacement(mutation, owner) {
                CollisionMutationStep::Pending => {}
                CollisionMutationStep::Complete => {
                    self.preparation_spatial = None;
                    self.preparation_cursor += 1;
                }
                CollisionMutationStep::Rejected(rejected) => {
                    self.fixed_rejection = Some(FillRetiredOwner::Spatial(rejected));
                    self.collection_over_capacity = true;
                }
                CollisionMutationStep::Stale => return Err(StaleSpatialIndex),
            }
            return Ok(());
        }
        let Some(entry) = self.placed.get(self.preparation_cursor) else {
            self.preparation_cursor = 0;
            self.stage = FillJobStage::PrepareLookup;
            return Ok(());
        };
        let Some(body) = self.meshes.get(&entry.mesh_url) else {
            self.preparation_cursor += 1;
            return Ok(());
        };
        self.preparation_spatial = Some(self.spatial_index.begin_replacement(owner, entry.object_id.clone(), CollisionAabb::from_body(body, &entry.world)));
        Ok(())
    }

    fn prepare_lookup_one(&mut self) {
        let Some(entry) = self.placed.get(self.preparation_cursor) else {
            self.preparation_cursor = 0;
            self.stage = FillJobStage::PrepareConfiguration;
            return;
        };
        let index = self.preparation_cursor;
        self.preparation_cursor += 1;
        match self.placed_lookup.try_insert(entry.object_id.clone(), index) {
            Ok(FixedOwnerMapInsert::Inserted) => {}
            Ok(FixedOwnerMapInsert::Occupied { input_key, input_value: _ }) | Err((input_key, _)) => {
                self.fixed_rejection = Some(FillRetiredOwner::String(input_key));
                self.collection_over_capacity = true;
            }
        }
    }

    fn prepare_configuration_one(&mut self) {
        let roots = self.preparation_roots.as_ref().expect("preparation roots");
        let value = match self.preparation_inner_cursor {
            0 => roots.scene.weights.object_weights.iter().nth(self.preparation_cursor).map(|(id, weight)| {
                let _ = self.weights.object_weights.try_insert(id.clone(), *weight);
            }),
            1 => roots.scene.weights.vortex_weights.iter().nth(self.preparation_cursor).map(|(id, weight)| {
                let _ = self.weights.vortex_weights.try_insert(id.clone(), *weight);
            }),
            _ => roots.scene.kind_compatibility.get(self.preparation_cursor).map(|value| {
                if let Err(owner) = self.kind_compatibility.try_push(value.clone()) {
                    self.fixed_rejection = Some(FillRetiredOwner::Compat(owner));
                }
            }),
        };
        if value.is_some() {
            self.preparation_cursor += 1;
            return;
        }
        self.preparation_cursor = 0;
        self.preparation_inner_cursor += 1;
        if self.preparation_inner_cursor == 3 {
            self.host_rules = roots.scene.host_rules.clone();
            self.contact_tolerance = roots.scene.contact_tolerance;
            self.preparation_roots = None;
            self.preparation_inner_cursor = 0;
            self.stage = FillJobStage::PrepareTargets;
        }
    }
}
