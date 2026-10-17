//! 🧵️ Interactive fill: target and candidate selection, the broad and narrow phase, acceptance, the resets and the planner transition under any step context.

use super::*;

//#region 🧵️InteractiveFillJob
impl FillBuilder {
    /// 🌀️ Builds the free vortex pool one bounded unit per transition: forget the previous blocked set, block every vortex
    /// an attraction already occupies, then add every open vortex with its distribution weight.
    fn prepare_targets(&mut self) {
        match self.target_prepare_phase {
            TargetPreparePhase::Reset => {
                if let Some(value) = self.blocked_vortex_ids.pop_first() {
                    drop(value);
                } else {
                    self.target_attraction_cursor = 0;
                    self.target_prepare_phase = TargetPreparePhase::Blocked;
                }
            }
            TargetPreparePhase::Blocked => {
                if let Some((attracting, attracted)) = self.scene_attraction(self.target_attraction_cursor).map(|attraction| (attraction.attracting.clone(), attraction.attracted.clone())) {
                    for id in [attracting, attracted] {
                        match self.blocked_vortex_ids.try_insert(id) {
                            Ok(FixedOwnerSetInsert::Inserted) => {}
                            Ok(FixedOwnerSetInsert::Present { input }) => drop(input),
                            Err(value) => {
                                self.fixed_rejection = Some(FillRetiredOwner::String(value));
                                return;
                            }
                        }
                    }
                    self.target_attraction_cursor += 1;
                } else {
                    self.target_object_cursor = 0;
                    self.target_vortex_cursor = 0;
                    self.target_prepare_phase = TargetPreparePhase::Enumerate;
                }
            }
            TargetPreparePhase::Enumerate => {
                let Some(object) = self.scene_object(self.target_object_cursor) else {
                    self.targets_ready = true;
                    self.stage = FillJobStage::SelectTarget;
                    return;
                };
                let Some(vortex) = object.vortices.get(self.target_vortex_cursor) else {
                    self.target_object_cursor += 1;
                    self.target_vortex_cursor = 0;
                    return;
                };
                let full_id = puzzle3d_vortex_full_id(&object.id, &vortex.id);
                let target = BrushFillVortexTarget { full_id, object_id: object.id.clone(), object_kind: object.object_kind.clone(), vortex_kind: vortex.vortex_kind.clone(), vortex_index: self.target_vortex_cursor };
                self.target_vortex_cursor += 1;
                if !self.blocked_vortex_ids.contains(&target.full_id) {
                    self.push_target(target);
                }
            }
        }
    }

    /// ➕️ Adds one free vortex to the pool; a vortex kind the distribution weighs zero is never drawn, so it is not kept.
    fn push_target(&mut self, target: BrushFillVortexTarget) {
        let weight = retained_fill_vortex_target_weight(&target, &self.weights);
        if weight <= 0.0 {
            return;
        }
        if self.targets.len() >= DOCUMENT_VORTEX_SLOTS {
            self.collection_over_capacity = true;
            return;
        }
        self.targets.push(target);
        self.target_weights.push(weight);
        fenwick_push(&mut self.target_tree, weight);
    }

    /// 🚫️ Takes one vortex out of the draw for good (consumed by a placement, or marked).
    fn retire_target_weight(&mut self, index: usize) {
        if let Some(weight) = self.target_weights.get_mut(index) {
            let removed = std::mem::replace(weight, 0.0);
            fenwick_add(&mut self.target_tree, index, -removed);
        }
    }

    /// 🛑️ Why the pool ran dry before the requested count: no free vortex ever, no vortex with a compatible kind, or every
    /// tested pose collided.
    fn exhausted_targets_stall(&self) -> FillStall {
        if self.targets.is_empty() {
            FillStall::NoOpenVortex
        } else if self.ever_constructed {
            FillStall::NoFreePlacement
        } else {
            FillStall::NoCompatibleKind
        }
    }

    /// 🎲️ Draws one free vortex by its distribution weight; a pool whose every vortex is consumed or marked ends the plan.
    fn select_target(&mut self) {
        let Some(index) = weighted_draw(&self.target_weights, &self.target_tree, &mut self.rng_state) else {
            self.stall(self.exhausted_targets_stall());
            return;
        };
        self.current_target = self.targets.get(index).cloned();
        self.current_target_index = Some(index);
        self.reset_candidate_preparation();
        self.stage = FillJobStage::PrepareCandidates;
    }

    /// 🧩️ Lists the compatible vortices of the drawn vortex and orders them by weighted random draws without replacement,
    /// so testing them in order is the same as drawing a fresh random compatible vortex after every collision.
    fn prepare_candidates(&mut self) {
        let Some(target) = self.current_target.clone() else {
            self.mark_target("missing-target");
            return;
        };
        let target_context = AttractionVortexContext { object_kind: target.object_kind.clone(), vortex_kind: target.vortex_kind.clone() };
        match self.candidate_prepare_phase {
            CandidatePreparePhase::Reset => {
                if let Some(value) = self.candidate_seen.pop_first() {
                    drop(value);
                } else {
                    self.candidate_prepare_phase = CandidatePreparePhase::Enumerate;
                }
            }
            CandidatePreparePhase::Enumerate => {
                let Some(kind) = self.catalogs.objects.get(self.candidate_kind_cursor) else {
                    self.candidate_remaining = self.candidate_raw.len();
                    self.candidate_prepare_phase = CandidatePreparePhase::Order;
                    return;
                };
                if self.candidate_vortex_cursor >= kind.vortices.len() {
                    self.candidate_kind_cursor += 1;
                    self.candidate_vortex_cursor = 0;
                    return;
                }
                let vortex_index = self.candidate_vortex_cursor;
                self.candidate_vortex_cursor += 1;
                let Some((candidate, _)) = brush_fill_candidate_at(&target_context, &self.catalogs, self.kind_compatibility.as_slice(), &self.host_rules, self.candidate_kind_cursor, vortex_index) else { return };
                let weight = retained_candidate_suggestion_weight(&candidate, &self.weights, &self.catalogs);
                if weight <= 0.0 {
                    return;
                }
                let key = format!("{}\u{1}{}", candidate.object_kind_id, candidate.source_vortex_index);
                match self.candidate_seen.try_insert(key) {
                    Ok(FixedOwnerSetInsert::Inserted) => {
                        self.candidate_raw.push(candidate);
                        self.candidate_weights.push(weight);
                        fenwick_push(&mut self.candidate_tree, weight);
                    }
                    Ok(FixedOwnerSetInsert::Present { input }) => drop(input),
                    Err(key) => self.fixed_rejection = Some(FillRetiredOwner::String(key)),
                }
            }
            CandidatePreparePhase::Order => {
                if let Some(index) = weighted_pick(&mut self.candidate_weights, &mut self.candidate_tree, self.candidate_remaining, &mut self.rng_state) {
                    self.candidates.push(self.candidate_raw[index].clone());
                    self.candidate_remaining -= 1;
                } else {
                    self.candidate_prepare_phase = CandidatePreparePhase::Finish;
                }
            }
            CandidatePreparePhase::Finish => {
                self.candidate_cursor = 0;
                if self.candidates.is_empty() {
                    self.mark_target("no-compatible-candidate");
                } else {
                    self.stage = FillJobStage::SelectCandidate;
                }
            }
        }
    }

    fn select_candidate(&mut self) {
        if self.candidate_cursor >= self.candidates.len() {
            self.mark_target("candidates-exhausted");
            return;
        }
        self.stage = FillJobStage::ConstructPreview;
    }

    fn construct_preview(&mut self) {
        let Some(target) = &self.current_target else {
            self.mark_target("missing-target");
            return;
        };
        let Some(candidate) = self.candidates.get(self.candidate_cursor) else {
            self.mark_target("missing-candidate");
            return;
        };
        let Some(host) = self.base.objects.iter().chain(&self.appended_objects).find(|object| object.id == target.object_id) else {
            self.mark_target("missing-host");
            return;
        };
        let Some((position, direction)) = vortex_world_from_object(host, target.vortex_index) else {
            self.reject_candidate("invalid-target-pose");
            return;
        };
        let context = AttractionVortexContext { object_kind: target.object_kind.clone(), vortex_kind: target.vortex_kind.clone() };
        let world = TargetVortexWorld { position, direction, reference_orientation: host.orientation };
        let scene_snapshot = self.scene_view();
        let Some(preview) = brush_preview_from_candidate(&target.full_id, candidate, &context, world, &self.catalogs, &scene_snapshot) else {
            self.reject_candidate("preview-unavailable");
            return;
        };
        self.ever_constructed = true;
        if self.run_events.is_some() {
            self.run_candidate_live = true;
            self.run_event(FillRunEvent::Constructed { mesh_url: preview.mesh_url.clone(), origin: preview.origin, orientation: preview.orientation, scale: fill_run_scale(&preview.scale) });
        }
        let Some(body) = self.meshes.get(&preview.mesh_url) else {
            self.reject_candidate("mesh-unavailable");
            return;
        };
        if self.placed.len() != self.base.objects.len() + self.appended_objects.len() {
            self.reject_candidate("placed-mesh-unavailable");
            return;
        }
        let preview_world = pose_isometry(preview.origin, preview.orientation, &preview.scale);
        let (min, max) = world_bounds(body, &preview_world);
        if !world_volumes_contain_aabb(self.base.target_volumes.as_slice(), min, max) {
            self.reject_candidate("outside-target-volume");
            return;
        }
        self.current_preview = Some(preview);
        self.last_rejection = None;
        self.stage = FillJobStage::QueryBroadPhase;
    }

    fn query_broad_phase(&mut self) {
        let Some(_target) = &self.current_target else {
            self.mark_target("missing-target");
            return;
        };
        let Some(preview) = &self.current_preview else {
            self.reject_candidate("missing-preview");
            return;
        };
        let Some(body) = self.meshes.get(&preview.mesh_url) else {
            self.reject_candidate("mesh-unavailable");
            return;
        };
        if self.broad_phase_query.is_none() {
            let world = pose_isometry(preview.origin, preview.orientation, &preview.scale);
            let bounds = CollisionAabb::from_body(body, &world);
            self.broad_phase_bounds = Some(bounds);
            self.broad_phase_query = Some(self.spatial_index.begin_query(self.collision_owner(), bounds));
            self.broad_phase_cursor = 0;
            return;
        }
        let owner = self.collision_owner();
        let query = self.broad_phase_query.as_mut().expect("broad phase query");
        match self.spatial_index.step_query(query, owner) {
            CollisionQueryStep::Pending => return,
            CollisionQueryStep::Stale => {
                self.reject_candidate("stale-spatial-query");
                return;
            }
            CollisionQueryStep::Complete => {}
        }
        if query.truncated() {
            self.reject_candidate("broad-phase-entry-missing");
            return;
        }
        self.broad_phase_cursor = 0;
        self.collision = None;
        self.stage = FillJobStage::TestCollision;
    }

    fn test_collision<C: FillStepContext>(&mut self, context: &mut C) -> Option<JobTurn> {
        let Some(pair_id) = self.broad_phase_query.as_ref().and_then(|query| query.candidate(self.broad_phase_cursor)).cloned() else {
            self.stage = FillJobStage::AcceptCandidate;
            return None;
        };
        let Some(preview) = &self.current_preview else {
            self.reject_candidate("missing-preview");
            return None;
        };
        let Some(preview_body) = self.meshes.get(&preview.mesh_url) else {
            self.reject_candidate("mesh-unavailable");
            return None;
        };
        let Some(entry) = self.placed_lookup.get(&pair_id).and_then(|index| self.placed.get(*index)) else {
            self.reject_candidate("broad-phase-entry-missing");
            return None;
        };
        let Some(other) = self.meshes.get(&entry.mesh_url) else {
            self.reject_candidate("placed-mesh-unavailable");
            return None;
        };
        let preview_world = pose_isometry(preview.origin, preview.orientation, &preview.scale);
        let collision = self.collision.get_or_insert_with(|| CollisionPenetrationState::new(self.contact_tolerance));
        let result = collision.step(context, preview_body, &preview_world, other, &entry.world);
        match result {
            CollisionStepResult::Pending => {}
            CollisionStepResult::Cancelled => return Some(JobTurn::Cancelled),
            CollisionStepResult::Complete { depth, .. } if depth > self.contact_tolerance => {
                self.reject_candidate("solid-overlap");
            }
            CollisionStepResult::Complete { .. } => {
                self.broad_phase_cursor += 1;
                self.collision = None;
            }
        }
        None
    }

    fn accept_candidate(&mut self) -> JobTurn {
        match self.accept_phase {
            AcceptPhase::Validate => {
                let Some(preview) = self.current_preview.clone() else {
                    self.reject_candidate("missing-preview");
                    return JobTurn::Yield;
                };
                let payload = BrushPlacePayload {
                    target_vortex_full_id: preview.target_vortex_full_id.clone(),
                    object_kind_id: preview.object_kind_id.clone(),
                    source_vortex_index: preview.source_vortex_index,
                    origin: preview.origin,
                    orientation: preview.orientation,
                    scale: preview.scale,
                };
                let Some(kind) = self.catalogs.objects.iter().find(|kind| kind.id == payload.object_kind_id) else {
                    self.reject_candidate("placement-kind-missing");
                    return JobTurn::Yield;
                };
                if kind.vortices.get(payload.source_vortex_index).is_none() {
                    self.reject_candidate("placement-vortex-missing");
                    return JobTurn::Yield;
                }
                let scene_snapshot = self.scene_view();
                let Some(mesh_url) = resolve_object_kind_mesh_url(&payload.object_kind_id, &self.catalogs, &scene_snapshot) else {
                    self.reject_candidate("placement-mesh-missing");
                    return JobTurn::Yield;
                };
                let object_id = brush_object_id(&scene_snapshot, &payload);
                let source_vortex_id = format!("{object_id}:v{}", payload.source_vortex_index);
                let attracted = puzzle3d_vortex_full_id(&object_id, &source_vortex_id);
                self.pending_attraction = Some(AttractionProps {
                    id: format!("attraction-{}-{attracted}", payload.target_vortex_full_id),
                    attracting: payload.target_vortex_full_id.clone(),
                    attracted,
                    gap: 0.0,
                    shift: 0.0,
                    rise: 0.0,
                    rotation: 0.0,
                    turn: 0.0,
                    tilt: 0.0,
                    x: 0.0,
                    y: 0.0,
                });
                self.pending_object = Some(EngineSceneObject {
                    id: object_id,
                    object_kind: Some(kind.id.clone()),
                    anchor: Default::default(),
                    mesh_url: Some(mesh_url),
                    origin: payload.origin,
                    orientation: Some(payload.orientation),
                    scale: payload.scale.clone().or(kind.scale.clone()),
                    vortices: Vec::new(),
                });
                self.pending_payload = Some(payload);
                self.accept_attraction_cursor = 0;
                self.accept_vortex_cursor = 0;
                self.accept_phase = AcceptPhase::CheckAttractions;
                JobTurn::Yield
            }
            AcceptPhase::CheckAttractions => {
                let Some((pending_attracting, pending_attracted)) = self.pending_attraction.as_ref().map(|pending| (pending.attracting.clone(), pending.attracted.clone())) else {
                    self.reject_candidate("placement-state-missing");
                    return JobTurn::Yield;
                };
                if let Some(rejected) = self.scene_attraction(self.accept_attraction_cursor).map(|attraction| attraction.attracting == pending_attracting || attraction.attracted == pending_attracted) {
                    self.accept_attraction_cursor += 1;
                    if rejected {
                        self.reject_candidate("placement-rejected");
                    }
                    return JobTurn::Yield;
                }
                self.accept_phase = AcceptPhase::BuildVortices;
                JobTurn::Yield
            }
            AcceptPhase::BuildVortices => {
                let Some(payload) = self.pending_payload.as_ref() else {
                    self.reject_candidate("placement-state-missing");
                    return JobTurn::Yield;
                };
                let Some(kind) = self.catalogs.objects.iter().find(|kind| kind.id == payload.object_kind_id) else {
                    self.reject_candidate("placement-kind-missing");
                    return JobTurn::Yield;
                };
                if let Some(template) = kind.vortices.get(self.accept_vortex_cursor) {
                    let object_id = self.pending_object.as_ref().expect("pending object").id.clone();
                    let index = self.accept_vortex_cursor;
                    self.accept_vortex_cursor += 1;
                    self.pending_object.as_mut().expect("pending object").vortices.push(VortexProps { id: format!("{object_id}:v{index}"), vortex_kind: template.vortex_kind.clone(), position: template.point, direction: template.direction });
                    return JobTurn::Yield;
                }
                self.accept_phase = AcceptPhase::BeginSpatial;
                JobTurn::Yield
            }
            AcceptPhase::BeginSpatial => {
                let Some(object) = self.pending_object.as_ref() else {
                    self.reject_candidate("placement-state-missing");
                    return JobTurn::Yield;
                };
                let Some(mesh_url) = object.mesh_url.as_ref() else {
                    self.accept_phase = AcceptPhase::Commit;
                    return JobTurn::Yield;
                };
                let Some(body) = self.meshes.get(mesh_url) else {
                    self.reject_candidate("placement-mesh-missing");
                    return JobTurn::Yield;
                };
                let world = pose_isometry(object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &object.scale);
                self.pending_spatial = Some(self.spatial_index.begin_replacement(self.collision_owner(), object.id.clone(), CollisionAabb::from_body(body, &world)));
                self.accept_phase = AcceptPhase::StepSpatial;
                JobTurn::Yield
            }
            AcceptPhase::StepSpatial => {
                let owner = self.collision_owner();
                let Some(mutation) = self.pending_spatial.as_mut() else {
                    self.reject_candidate("placement-spatial-state-missing");
                    return JobTurn::Yield;
                };
                match self.spatial_index.step_replacement(mutation, owner) {
                    CollisionMutationStep::Pending => {}
                    CollisionMutationStep::Complete => {
                        self.pending_spatial = None;
                        self.accept_phase = AcceptPhase::InstallLookup;
                    }
                    CollisionMutationStep::Rejected(rejected) => self.fixed_rejection = Some(FillRetiredOwner::Spatial(rejected)),
                    CollisionMutationStep::Stale => self.reject_candidate("stale-spatial-mutation"),
                }
                JobTurn::Yield
            }
            AcceptPhase::InstallLookup => {
                let Some(object) = self.pending_object.as_ref() else {
                    self.reject_candidate("placement-state-missing");
                    return JobTurn::Yield;
                };
                let Some(mesh_url) = object.mesh_url.as_ref() else {
                    self.accept_phase = AcceptPhase::Commit;
                    return JobTurn::Yield;
                };
                let index = self.placed.len();
                match self.placed_lookup.try_insert(object.id.clone(), index) {
                    Ok(FixedOwnerMapInsert::Inserted) => {}
                    Ok(FixedOwnerMapInsert::Occupied { input_key, input_value: _ }) | Err((input_key, _)) => {
                        self.fixed_rejection = Some(FillRetiredOwner::String(input_key));
                        return JobTurn::Yield;
                    }
                }
                self.placed.push(PlacedCollisionEntry { object_id: object.id.clone(), mesh_url: mesh_url.clone(), world: pose_isometry(object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &object.scale) });
                self.accept_vortex_cursor = 0;
                self.accept_phase = AcceptPhase::AppendTargets;
                JobTurn::Yield
            }
            AcceptPhase::AppendTargets => {
                let (Some(object), Some(payload)) = (self.pending_object.as_ref(), self.pending_payload.as_ref()) else {
                    self.reject_candidate("placement-state-missing");
                    return JobTurn::Yield;
                };
                let index = self.accept_vortex_cursor;
                let Some(vortex) = object.vortices.get(index) else {
                    self.accept_phase = AcceptPhase::Commit;
                    return JobTurn::Yield;
                };
                self.accept_vortex_cursor += 1;
                if index != payload.source_vortex_index {
                    let target = BrushFillVortexTarget { full_id: puzzle3d_vortex_full_id(&object.id, &vortex.id), object_id: object.id.clone(), object_kind: object.object_kind.clone(), vortex_kind: vortex.vortex_kind.clone(), vortex_index: index };
                    self.push_target(target);
                }
                JobTurn::Yield
            }
            AcceptPhase::Commit => {
                let Some(payload) = self.pending_payload.take() else {
                    self.reject_candidate("placement-state-missing");
                    return JobTurn::Yield;
                };
                let Some(placed_object) = self.pending_object.take() else {
                    self.reject_candidate("placement-state-missing");
                    return JobTurn::Yield;
                };
                let Some(attraction) = self.pending_attraction.take() else {
                    self.reject_candidate("placement-state-missing");
                    return JobTurn::Yield;
                };
                self.sequence.push(payload);
                self.appended_objects.push(placed_object);
                self.appended_attractions.push(attraction);
                if std::mem::take(&mut self.run_candidate_live) {
                    self.run_event(FillRunEvent::Accepted);
                }
                if let Some(index) = self.current_target_index {
                    self.retire_target_weight(index);
                }
                self.reset_candidate();
                self.stage = self.next_round_stage();
                if matches!(self.stage, FillJobStage::Complete(_)) {
                    return self.complete();
                }
                JobTurn::Checkpoint { applied_progress: self.applied_count as u64, state: Vec::new() }
            }
        }
    }

    /// 🛑️ Stops the planner below its requested count with a reason the HUD can name, instead of
    /// leaving the user with a silently finished plan.
    pub fn stall(&mut self, stall: FillStall) {
        if std::mem::take(&mut self.run_candidate_live) {
            self.run_event(FillRunEvent::Refused(stall.reason()));
        }
        self.stage = FillJobStage::Complete(FillPlanEnd::Stalled(stall));
    }

    fn reject_candidate(&mut self, reason: &str) {
        self.refuse_run_candidate(reason);
        self.last_rejection = Some(reason.to_string());
        self.rejected_count += 1;
        self.candidate_cursor += 1;
        self.reset_acceptance();
        self.reset_collision();
        self.stage = FillJobStage::SelectCandidate;
    }

    /// 🎯️ Marks the drawn vortex: every compatible candidate was refused (or it has none), so it leaves the draw for good,
    /// shows as a danger marker, and the next round draws another free vortex.
    fn mark_target(&mut self, reason: &str) {
        self.refuse_run_candidate(reason);
        self.last_rejection = Some(reason.to_string());
        self.rejected_count += 1;
        if let Some(index) = self.current_target_index.take() {
            self.retire_target_weight(index);
        }
        if self.run_events.is_some() {
            let host = self.current_target.as_ref().and_then(|target| self.base.objects.iter().chain(&self.appended_objects).find(|object| object.id == target.object_id).and_then(|host| vortex_world_from_object(host, target.vortex_index)));
            if let Some((position, _)) = host {
                self.run_event(FillRunEvent::VortexMarked { position });
            }
        }
        self.current_target = None;
        self.reset_candidate_preparation();
        self.reset_acceptance();
        self.reset_collision();
        self.stage = FillJobStage::SelectTarget;
    }

    fn reset_collision(&mut self) {
        self.current_preview = None;
        self.broad_phase_query = None;
        self.broad_phase_cursor = 0;
        self.broad_phase_bounds = None;
        self.collision = None;
    }

    /// 🔄️ Ends one vortex round: the drawn vortex, its candidates, the acceptance and the collision state. The pool stays.
    pub(super) fn reset_candidate(&mut self) {
        self.current_target = None;
        self.current_target_index = None;
        self.reset_candidate_preparation();
        self.reset_acceptance();
        self.last_rejection = None;
        self.reset_collision();
    }

    /// 🌀️ Drops the free vortex pool so the next round rebuilds it from the (retracted) document; marks are forgotten
    /// because fewer placed objects can free a vortex again.
    pub(super) fn reset_targets(&mut self) {
        self.targets.clear();
        self.target_weights.clear();
        self.target_tree = vec![0.0];
        self.targets_ready = false;
        self.target_prepare_phase = TargetPreparePhase::Reset;
        self.target_attraction_cursor = 0;
        self.target_object_cursor = 0;
        self.target_vortex_cursor = 0;
    }

    fn reset_candidate_preparation(&mut self) {
        self.candidates.clear();
        self.candidate_cursor = 0;
        self.candidate_prepare_phase = CandidatePreparePhase::Reset;
        self.candidate_kind_cursor = 0;
        self.candidate_vortex_cursor = 0;
        self.candidate_prepare_cursor = 0;
        self.candidate_raw.clear();
        self.candidate_weights.clear();
        self.candidate_tree = vec![0.0];
        self.candidate_remaining = 0;
    }

    fn reset_acceptance(&mut self) {
        self.accept_phase = AcceptPhase::Validate;
        self.accept_attraction_cursor = 0;
        self.accept_vortex_cursor = 0;
        self.pending_payload = None;
        self.pending_object = None;
        self.pending_attraction = None;
        self.pending_spatial = None;
    }

    /// 🛰️ Records one observation when a fill run job is listening.
    pub(super) fn run_event(&mut self, event: FillRunEvent) {
        if let Some(events) = self.run_events.as_mut() {
            events.push(event);
        }
    }

    fn refuse_run_candidate(&mut self, reason: &str) {
        if std::mem::take(&mut self.run_candidate_live) {
            self.run_event(FillRunEvent::Refused(FillRunReason::of_refusal(reason)));
        }
    }

    /// 🫥️ A retarget or replan drops the candidate under test before it reached a verdict.
    pub(super) fn abandon_run_candidate(&mut self) {
        if std::mem::take(&mut self.run_candidate_live) {
            self.run_event(FillRunEvent::Abandoned);
        }
    }

    /// 🛰️ Starts recording observations for a fill run job.
    pub fn observe_run(&mut self) {
        self.run_events.get_or_insert_with(|| Vec::with_capacity(4));
    }

    /// 📤️ Moves the recorded observations into `into` (cleared first), keeping both buffers' capacity.
    pub fn swap_run_events(&mut self, into: &mut Vec<FillRunEvent>) {
        into.clear();
        if let Some(events) = self.run_events.as_mut() {
            std::mem::swap(events, into);
        }
    }

    /// 🚧️ A document too large for the planner's fixed pages ends the run where the user can see it: the
    /// preparation preflight's refusal publishes one `danger` step through the run's tick writer and then
    /// faults the run (nothing was changed). Without a writer the refusal faults at once. A page that fills
    /// mid-plan is [`FillBuilder::stall_on_capacity`] instead.
    pub fn capacity_refusal<C: FillStepContext>(&mut self, _context: &mut C, writer: Option<&mut ToolRunTickWriter>) -> Option<JobTurn> {
        if let Some(refusal) = self.preparation_capacity_refusal.as_mut() {
            if let Some(writer) = writer.filter(|_| !refusal.published) {
                refusal.published = true;
                let _ = writer.step(ToolRunStepKind::Danger, FillRunStage::Prepare.index(), FillRunReason::ArtifactCapacity.code(), None, &[ToolRunStepArg::Unsigned(refusal.omitted_index as u64)]);
                return Some(JobTurn::Yield);
            }
            return Some(JobTurn::Fault(refusal.diagnostic().as_bytes().to_vec()));
        }
        None
    }

    /// 📄️ A fixed page that refused an owner mid-plan stalls the plan as `artifact-capacity` over what was already
    /// placed; answers whether the plan is over capacity.
    pub fn stall_on_capacity(&mut self) -> bool {
        if self.collection_over_capacity || self.fixed_rejection.is_some() {
            if !matches!(self.stage, FillJobStage::Complete(_)) {
                self.stall(FillStall::ArtifactCapacity);
            }
            return true;
        }
        false
    }

    pub(super) fn complete(&self) -> JobTurn {
        JobTurn::Complete
    }

    pub fn stage_label(&self) -> &'static str {
        match self.stage {
            FillJobStage::RetractTail => "retract-tail",
            FillJobStage::PrepareScene => "prepare-scene_snapshot",
            FillJobStage::PrepareCatalogs => "prepare-catalogs",
            FillJobStage::PrepareMeshes => "prepare-meshes",
            FillJobStage::PrepareEntries => "prepare-entries",
            FillJobStage::PrepareSpatial => "prepare-spatial",
            FillJobStage::PrepareLookup => "prepare-lookup",
            FillJobStage::PrepareConfiguration => "prepare-configuration",
            FillJobStage::PrepareTargets => "prepare-targets",
            FillJobStage::SelectTarget => "select-target",
            FillJobStage::PrepareCandidates => "prepare-candidates",
            FillJobStage::SelectCandidate => "select-candidate",
            FillJobStage::ConstructPreview => "construct-preview",
            FillJobStage::QueryBroadPhase => "query-broad-phase",
            FillJobStage::TestCollision => "test-collision",
            FillJobStage::AcceptCandidate => "accept-candidate",
            FillJobStage::Complete(_) => "complete",
        }
    }
}

impl FillBuilder {
    /// 🪜️ One bounded planner transition under any [`FillStepContext`].
    pub fn advance<C: FillStepContext>(&mut self, context: &mut C) -> JobTurn {
        if context.is_cancelled() {
            return JobTurn::Cancelled;
        }
        if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
            return JobTurn::Fault(b"stale-fill-operation".to_vec());
        }
        if let Some(outcome) = self.capacity_refusal(context, None) {
            return outcome;
        }
        if self.stall_on_capacity() {
            return self.complete();
        }
        if context.should_yield() {
            return JobTurn::Yield;
        }
        context.set_stage(self.stage_label());
        let stage = self.stage;
        let outcome = match stage {
            FillJobStage::RetractTail => {
                if self.discard_tail_one().is_err() {
                    return JobTurn::Fault(b"stale-spatial-index".to_vec());
                }
                None
            }
            FillJobStage::PrepareScene | FillJobStage::PrepareCatalogs | FillJobStage::PrepareMeshes | FillJobStage::PrepareEntries | FillJobStage::PrepareSpatial | FillJobStage::PrepareLookup | FillJobStage::PrepareConfiguration => {
                if self.prepare_one().is_err() {
                    return JobTurn::Fault(b"stale-spatial-index".to_vec());
                }
                None
            }
            FillJobStage::PrepareTargets => {
                self.prepare_targets();
                None
            }
            FillJobStage::SelectTarget => {
                self.select_target();
                None
            }
            FillJobStage::PrepareCandidates => {
                self.prepare_candidates();
                None
            }
            FillJobStage::SelectCandidate => {
                self.select_candidate();
                None
            }
            FillJobStage::ConstructPreview => {
                self.construct_preview();
                None
            }
            FillJobStage::QueryBroadPhase => {
                self.query_broad_phase();
                None
            }
            FillJobStage::TestCollision => self.test_collision(context),
            FillJobStage::AcceptCandidate => Some(self.accept_candidate()),
            FillJobStage::Complete(_) => return self.complete(),
        };
        self.transition_count += 1;
        precompute_work(1);
        if stage != FillJobStage::TestCollision {
            context.consume_fuel(1);
        }
        if context.is_cancelled() {
            return JobTurn::Cancelled;
        }
        if stage == self.stage
            && matches!(
                stage,
                FillJobStage::RetractTail
                    | FillJobStage::PrepareScene
                    | FillJobStage::PrepareCatalogs
                    | FillJobStage::PrepareMeshes
                    | FillJobStage::PrepareEntries
                    | FillJobStage::PrepareSpatial
                    | FillJobStage::PrepareLookup
                    | FillJobStage::PrepareConfiguration
                    | FillJobStage::PrepareTargets
                    | FillJobStage::PrepareCandidates
                    | FillJobStage::QueryBroadPhase
            )
        {
            return JobTurn::Yield;
        }
        outcome.unwrap_or(JobTurn::Yield)
    }
}

impl FillBuilder {
    pub fn begin_close(&mut self) {
        self.closing = true;
    }

    /// ♻️ Retires one planner owner per granted turn; a turn needs one owner page of release credit.
    pub fn close_turn(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.closing = true;
        if grant.maximum_items == 0 || grant.maximum_release_bytes < FILL_BUILDER_OWNER_PAGE_BYTES {
            return semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() };
        }
        if self.retire_one_close_owner() {
            semio_framework_job::InteractiveJobCloseStep::Complete { progress: Default::default() }
        } else {
            semio_framework_job::InteractiveJobCloseStep::Pending { progress: semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, released_bytes: FILL_BUILDER_OWNER_PAGE_BYTES, ..Default::default() } }
        }
    }

    /// 📏️ Quotes the next close turn: one owner page of release credit until the planner holds nothing.
    pub fn close_demands(&self) -> semio_framework_value::RetirementDemand {
        if self.terminal_is_empty() {
            return Default::default();
        }
        semio_framework_value::RetirementDemand { release_bytes: FILL_BUILDER_OWNER_PAGE_BYTES, depth: 1, ..Default::default() }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.close_current.is_none() && self.terminal_owners_empty()
    }
}
//#endregion 🧵️InteractiveFillJob
