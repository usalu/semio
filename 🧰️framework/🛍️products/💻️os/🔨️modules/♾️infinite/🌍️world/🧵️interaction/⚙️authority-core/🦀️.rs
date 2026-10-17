//! ⚙️ World3d interaction authority: the settle point, queue coalescing, intent steps and close.

use super::*;

impl WorldInteractionAuthority {
    /// 🧭️ A DRAINED queue is this lane's settle point — React's `CAMERA_SYNC_DEBOUNCE_MS`
    /// trailing debounce without a timer, since the last answered intent of a gesture is
    /// exactly when that debounce would fire. See [`WorldCameraSync`].
    ///
    /// 🧭️ three's `OrbitControls.onPointerUp` dispatches `_endEvent` UNCONDITIONALLY once the
    /// last pointer lifts, and React's `onEnd` always reports the camera — so every release
    /// over a world pane owes one `setCamera`, whatever the button and whether or not the
    /// camera moved. The reference journals exactly that for `pick-instance` and
    /// `context-menu` (`🗑️generated/parity-run-2/steps.json`).
    ///
    /// 🖱️ The MIDDLE button starts no CAMERA gesture of its own: pan is a middle-DRAG, planned by
    /// `plan_world3d_drag` from the moves that follow it. What the press itself answers is the
    /// SELECTION React publishes on any button (`plan_world3d_non_primary_press_pick`); with
    /// neither, the intent used to fall through to the unclaimed-intent fault below, so the FIRST
    /// middle-click on any world surface recorded a frame fault and killed the page
    /// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w9c-behaviour-parity-run-2.md` step 20,
    /// where React simply pans).
    ///
    /// 🖱️ Tracking the click/drag discrimination must NOT swallow the gesture: a right-drag is
    /// also how `plan_world3d_drag` reaches pan (`+shift`) and orbit (`+alt`/`+meta`). Retiring
    /// here unconditionally is what left the wgpu shell with zoom and middle-button pan only.
    ///
    /// 🖱️📋️ A right RELEASE closes the click/drag discrimination and answers nothing else. The
    /// menu itself is the SHELL's (`🐚️Shell/🎯️targets/🧊️wgpu`'s `open_context_menu`, whose
    /// `resolve_context_menu_surface` already fills a World3d surface's `hits`/`selection` from
    /// [`world3d_context_menu_surface`]) and it opens on the PRESS, exactly as React's own
    /// `onContextMenu` does.
    ///
    /// 🩸️ This arm used to run a `WorldContextMenuCursor` and publish `worldContextMenuAt`. No
    /// app declares that verb — React deleted its twin `contextMenuAt` when the target moved onto
    /// the menu REQUEST (`world3dContextMenuSurfaceV1`'s own docstring: "dispatching it anyway
    /// only produced an `undeclaredActionDiagnostic` drop on every right-click"), and the whole
    /// repository had exactly one mention of it left, here. So the wgpu target journalled a verb
    /// the guest refuses where React journals a menu
    /// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w10a-world-interaction-journal-parity.md`).
    ///
    /// 🚚️ A live relocate drag owns every move it receives: it only moves its own ghost and
    /// publishes nothing until the release, so it must not also reach the camera arms below
    /// (React returns early from `onPointerMove` for the same reason).
    ///
    /// 🚚️ The relocate gesture is checked FIRST, exactly as React's `handlePointerDown`
    /// opens with `if (relocateMode && beginRelocateDrag(event)) return;`
    /// (`🌐️World3dHost/🟦️.tsx:6883`). The press publishes nothing; the release publishes the
    /// ONE absolute `worldRelocate`, or nothing at all when the drag never travelled.
    ///
    /// 🚪️ A leave over a surface that publishes no hover has nothing to clear — it is answered,
    /// not refused, wherever it was aimed (the point it left at is usually OUTSIDE this
    /// surface's own rect by construction).
    ///
    /// 🖱️ An intent aimed at a point this surface no longer covers is not this surface's to
    /// answer. Every pick cursor is built through `pointer_in_pick_rect`, which refuses a point
    /// outside the CURRENT pick rect — and a queued intent outlives a relayout: dragging the
    /// dock's split gutter moves the pane out from under moves enqueued against its previous
    /// bounds (measured: a hover at x=534.4 against `pick=…+535.6,54.4`). Faulting on that
    /// refusal is a FRAME fault, which kills the page, so the whole shell tore down mid-drag
    /// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w9c-behaviour-parity-run-2.md` step 15).
    /// A stale aim retires like any other answered intent; a fault stays a fault for a point the
    /// surface DOES cover, which is a real refusal.
    pub fn step(&mut self, session_state: &mut WorldSessionState, camera_state: &mut WorldCameraState, selection_state: &mut WorldSelectionState, gumball_state: &mut WorldGumballState, interaction_state: &mut WorldInteractionState, scene_state: &mut WorldSceneState, generation: u64, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, context: &mut semio_framework_job::StepContext<'_>) -> WorldInteractionAuthorityStep {
        if self.closing || context.should_yield() {
            return WorldInteractionAuthorityStep::Pending;
        }
        if let Some(active) = self.active.take() {
            return self.step_active(active, session_state, camera_state, selection_state, gumball_state, interaction_state, scene_state, generation, input, context);
        }
        if self.faulted {
            return WorldInteractionAuthorityStep::Fault;
        }
        if self.marquee.as_ref().is_some_and(|marquee| marquee.revision != session_state.interaction_revision || marquee.retiring) {
            let complete = self.marquee.as_mut().expect("stale marquee owner").close_step();
            if complete {
                self.marquee = None;
            }
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Stale;
        }
        if self.gumball.as_ref().is_some_and(|gumball| gumball.revision != session_state.interaction_revision) {
            let complete = self.gumball.as_mut().expect("stale gumball owner").close_step();
            if complete {
                self.gumball = None;
            }
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Stale;
        }
        if interaction_state.interaction_objects.revision != session_state.interaction_revision || self.registry.is_some() {
            let mut registry = self.registry.take().unwrap_or_else(|| WorldInteractionRegistryBuildCursor::new(session_state.interaction_revision));
            match registry.step(selection_state, gumball_state, scene_state, context) {
                WorldInteractionStep::Pending => {
                    self.registry = Some(registry);
                    return WorldInteractionAuthorityStep::Pending;
                }
                WorldInteractionStep::Complete => {
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                WorldInteractionStep::Stale => {
                    self.registry = Some(WorldInteractionRegistryBuildCursor::new(session_state.interaction_revision));
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Stale;
                }
                WorldInteractionStep::Fault => {
                    self.registry = Some(registry);
                    self.faulted = true;
                    return WorldInteractionAuthorityStep::Fault;
                }
            }
        }
        if let Some(blocked) = self.blocked.take() {
            match self.queue.push(blocked) {
                Ok(()) => {
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                Err(blocked) => {
                    self.blocked = Some(blocked);
                }
            }
        }
        let Some(intent) = self.queue.front().copied() else {
            if let Some(plan) = plan_world3d_camera_settle(session_state, camera_state, generation) {
                self.active = Some(WorldInteractionActive::Plan { plan, retirement: None });
                context.consume_fuel(1);
                return WorldInteractionAuthorityStep::Pending;
            }
            if let (true, Some([x, y])) = (camera_state.camera_sync.hover_owed, camera_state.camera_sync.at) {
                camera_state.camera_sync.hover_owed = false;
                let mut hover = WorldInteractionIntent::pointer_move(x, y, 0.0, 0.0, false, 0, &PointerModifiers::default());
                hover.generation = self.next_generation;
                if self.queue.push(hover).is_ok() {
                    self.next_generation += 1;
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
            }
            return WorldInteractionAuthorityStep::Idle;
        };
        if !matches!(intent.phase, WorldInteractionPhase::Close | WorldInteractionPhase::PointerLeave | WorldInteractionPhase::Cancel) {
            camera_state.camera_sync.at = Some([intent.x, intent.y]);
            if intent.phase == WorldInteractionPhase::PointerButton && !intent.down {
                camera_state.camera_sync.owed = true;
            }
        }
        if intent.generation < generation {
            self.queue.retire_front(intent.generation);
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Stale;
        }
        if intent.generation > generation {
            self.faulted = true;
            return WorldInteractionAuthorityStep::Fault;
        }
        if intent.phase == WorldInteractionPhase::Cancel {
            let reason = WorldCancelReason::from_button(intent.button);
            if let Some((phase, motion)) = self.gumball.as_ref().and_then(|gesture| gesture.cancel(reason)) {
                let gesture = self.gumball.take().expect("cancelled live gumball gesture");
                let (translate, angle, scale) = motion;
                self.gumball_swallow = true;
                self.active = Some(WorldInteractionActive::GumballCommit { job: WorldGumballCommitJob::new(intent.generation, WorldGumballGesture { translate, angle, scale, ..gesture }, phase), retirement: None });
                context.consume_fuel(1);
                return WorldInteractionAuthorityStep::Pending;
            }
            if let Some(plan) = plan_world3d_paint_abort(session_state, selection_state, generation, reason) {
                self.active = Some(WorldInteractionActive::Plan { plan, retirement: None });
                context.consume_fuel(1);
                return WorldInteractionAuthorityStep::Pending;
            }
            self.queue.retire_front(intent.generation);
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Complete;
        }
        if self.gumball_swallow && intent.button == 0 && (intent.phase == WorldInteractionPhase::PointerMove && intent.down || intent.phase == WorldInteractionPhase::PointerButton) {
            if intent.phase == WorldInteractionPhase::PointerButton && !intent.down {
                self.gumball_swallow = false;
            }
            self.queue.retire_front(intent.generation);
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Complete;
        }
        if intent.phase == WorldInteractionPhase::PointerButton && intent.button == 2 && intent.down {
            self.right_press = Some([intent.x, intent.y]);
            self.right_dragged = false;
            if let Some(active) = plan_world3d_non_primary_press_pick(session_state, camera_state, selection_state, generation, &intent) {
                self.active = Some(active);
                context.consume_fuel(1);
                return WorldInteractionAuthorityStep::Pending;
            }
            self.queue.retire_front(intent.generation);
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Complete;
        }
        if intent.phase == WorldInteractionPhase::PointerButton && intent.button == 1 {
            if intent.down {
                if let Some(active) = plan_world3d_non_primary_press_pick(session_state, camera_state, selection_state, generation, &intent) {
                    self.active = Some(active);
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
            }
            self.queue.retire_front(intent.generation);
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Complete;
        }
        if intent.phase == WorldInteractionPhase::PointerMove && intent.button == 2 && intent.down {
            if let Some(start) = self.right_press {
                let dx = intent.x - start[0];
                let dy = intent.y - start[1];
                self.right_dragged |= (dx * dx + dy * dy).sqrt() > CLICK_DRAG_THRESHOLD_PX;
            }
            let modifiers = PointerModifiers { shift: intent.shift, ctrl: intent.ctrl, alt: intent.alt, meta: intent.meta };
            if let Some(plan) = plan_world3d_drag(session_state, camera_state, generation, intent.dx, intent.dy, intent.button, &modifiers) {
                self.active = Some(WorldInteractionActive::Plan { plan, retirement: None });
                context.consume_fuel(1);
                return WorldInteractionAuthorityStep::Pending;
            }
            self.queue.retire_front(intent.generation);
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Complete;
        }
        if intent.phase == WorldInteractionPhase::PointerButton && intent.button == 2 && !intent.down {
            self.right_press = None;
            self.right_dragged = false;
            self.queue.retire_front(intent.generation);
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Complete;
        }
        if intent.phase == WorldInteractionPhase::PointerButton && intent.button == 0 && intent.down && session_state.active_utility == "select" && self.gumball.is_none() && world3d_offers_transform_gumball(session_state) {
            self.active = Some(WorldInteractionActive::GumballPick { cursor: WorldGumballPickCursor::new(session_state, selection_state, generation, intent.x, intent.y), retirement: None });
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Pending;
        }
        if let (true, Some(gumball)) = (intent.button == 0 && (intent.phase == WorldInteractionPhase::PointerMove && intent.down || intent.phase == WorldInteractionPhase::PointerButton && !intent.down), self.gumball.as_mut()) {
            match gumball.begin_update(intent.generation, intent.x, intent.y) {
                WorldInteractionStep::Stale => return WorldInteractionAuthorityStep::Stale,
                WorldInteractionStep::Fault => {
                    self.faulted = true;
                    return WorldInteractionAuthorityStep::Fault;
                }
                _ => {}
            }
            match gumball.update_step(session_state, camera_state, gumball_state, interaction_state, scene_state) {
                WorldInteractionStep::Pending => {
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                WorldInteractionStep::Stale => return WorldInteractionAuthorityStep::Stale,
                WorldInteractionStep::Fault => {
                    self.faulted = true;
                    return WorldInteractionAuthorityStep::Fault;
                }
                WorldInteractionStep::Complete => {}
            }
            if intent.phase == WorldInteractionPhase::PointerButton {
                let gesture = self.gumball.take().expect("gumball final pointer pose is complete");
                let (phase, (translate, angle, scale)) = gesture.release();
                self.active = Some(WorldInteractionActive::GumballCommit { job: WorldGumballCommitJob::new(intent.generation, WorldGumballGesture { translate, angle, scale, ..gesture }, phase), retirement: None });
                context.consume_fuel(1);
                return WorldInteractionAuthorityStep::Pending;
            }
            if let Some((phase, motion)) = gumball.stream() {
                let tick = gumball.publication(motion);
                self.active = Some(WorldInteractionActive::GumballCommit { job: WorldGumballCommitJob::new(intent.generation, tick, phase), retirement: None });
                context.consume_fuel(1);
                return WorldInteractionAuthorityStep::Pending;
            }
            self.queue.retire_front(intent.generation);
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Complete;
        }
        if intent.phase == WorldInteractionPhase::PointerButton && intent.button == 0 && !intent.down && session_state.active_utility == "brush" {
            match WorldBrushCommitJob::new(session_state, gumball_state, intent.generation) {
                Ok(Some(job)) => {
                    self.active = Some(WorldInteractionActive::BrushCommit { job, retirement: None });
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                Ok(None) => {
                    self.queue.retire_front(intent.generation);
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Complete;
                }
                Err(_) => {
                    self.faulted = true;
                    return WorldInteractionAuthorityStep::Fault;
                }
            }
        }
        let select_marquee_route = session_state.active_utility == "select";
        if intent.phase == WorldInteractionPhase::PointerButton && intent.button == 0 && intent.down && select_marquee_route {
            if self.marquee.is_some() {
                self.faulted = true;
                return WorldInteractionAuthorityStep::Fault;
            }
            self.marquee = Some(WorldMarqueeGesture::new(session_state.interaction_revision, intent.generation, [intent.x, intent.y]));
            self.queue.retire_front(intent.generation);
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Complete;
        }
        if let (true, Some(marquee)) = (intent.phase == WorldInteractionPhase::PointerMove && intent.button == 0 && intent.down, self.marquee.as_mut()) {
            if intent.generation <= marquee.start_generation || !marquee.push([intent.x, intent.y]) {
                self.faulted = true;
                return WorldInteractionAuthorityStep::Fault;
            }
            self.queue.retire_front(intent.generation);
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Complete;
        }
        if intent.phase == WorldInteractionPhase::PointerButton && intent.button == 0 && !intent.down && self.marquee.is_some() {
            let marquee = self.marquee.as_mut().expect("marquee gesture retained above");
            if intent.generation <= marquee.start_generation {
                self.faulted = true;
                return WorldInteractionAuthorityStep::Fault;
            }
            if marquee.is_click([intent.x, intent.y]) {
                marquee.retiring = true;
                context.consume_fuel(1);
                return WorldInteractionAuthorityStep::Pending;
            }
            let marquee = self.marquee.take().expect("non-click marquee owner");
            let Some(cursor) = WorldMarqueePickCursor::new(session_state, camera_state, selection_state, intent.generation, marquee) else {
                self.faulted = true;
                return WorldInteractionAuthorityStep::Fault;
            };
            self.active = Some(WorldInteractionActive::MarqueePick { cursor, retirement: None });
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Pending;
        }
        let active = match intent.phase {
            WorldInteractionPhase::Wheel => plan_world3d_wheel(session_state, camera_state, generation, intent.delta).map(|plan| WorldInteractionActive::Plan { plan, retirement: None }),
            WorldInteractionPhase::PointerLeave => plan_world3d_pointer_leave(session_state, selection_state, generation).map(|plan| WorldInteractionActive::Plan { plan, retirement: None }),
            WorldInteractionPhase::PointerDrag => {
                if world3d_relocate_active(gumball_state) {
                    world3d_update_relocate_drag(session_state, camera_state, gumball_state, scene_state, intent.x, intent.y);
                    self.queue.retire_front(intent.generation);
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Complete;
                }
                let modifiers = PointerModifiers { shift: intent.shift, ctrl: intent.ctrl, alt: intent.alt, meta: intent.meta };
                let Some(plan) = plan_world3d_drag(session_state, camera_state, generation, intent.dx, intent.dy, intent.button, &modifiers) else {
                    self.queue.retire_front(intent.generation);
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Complete;
                };
                Some(WorldInteractionActive::Plan { plan, retirement: None })
            }
            WorldInteractionPhase::PointerMove => {
                if world3d_relocate_active(gumball_state) {
                    world3d_update_relocate_drag(session_state, camera_state, gumball_state, scene_state, intent.x, intent.y);
                    self.queue.retire_front(intent.generation);
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Complete;
                }
                let modifiers = PointerModifiers { shift: intent.shift, ctrl: intent.ctrl, alt: intent.alt, meta: intent.meta };
                if intent.down {
                    if let Some(plan) = plan_world3d_drag(session_state, camera_state, generation, intent.dx, intent.dy, intent.button, &modifiers) {
                        Some(WorldInteractionActive::Plan { plan, retirement: None })
                    } else if intent.button == 0 && selection_state.interaction_mode == "paint" && selection_state.paint_stroke_active {
                        WorldRayPickCursor::new(session_state, camera_state, generation, WorldRayPickPurpose::Paint, intent.x, intent.y).map(|cursor| WorldInteractionActive::Pick { cursor, retirement: None })
                    } else {
                        None
                    }
                } else {
                    if session_state.active_utility == "brush" || (session_state.active_utility == "select" && selection_state.granularity == "vertex") {
                        WorldObjectPickCursor::new(session_state, camera_state, generation, WorldObjectPickPurpose::VortexHover, intent.x, intent.y).map(|cursor| WorldInteractionActive::ObjectPick { cursor, retirement: None })
                    } else if component_mode_active(selection_state) {
                        WorldComponentPickCursor::new(session_state, camera_state, selection_state, generation, WorldComponentPickPurpose::Hover, intent.x, intent.y).map(|cursor| WorldInteractionActive::ComponentPick { cursor, retirement: None })
                    } else {
                        WorldRayPickCursor::new(session_state, camera_state, generation, WorldRayPickPurpose::Hover, intent.x, intent.y).map(|cursor| WorldInteractionActive::Pick { cursor, retirement: None })
                    }
                }
            }
            WorldInteractionPhase::PointerButton => {
                if intent.button == 0 && (world3d_relocate_active(gumball_state) || world3d_relocate_mode(session_state)) {
                    if intent.down {
                        if world3d_begin_relocate_drag(session_state, camera_state, selection_state, gumball_state, scene_state, intent.x, intent.y) {
                            self.queue.retire_front(intent.generation);
                            context.consume_fuel(1);
                            return WorldInteractionAuthorityStep::Complete;
                        }
                    } else if world3d_relocate_active(gumball_state) {
                        let commit = world3d_end_relocate_drag(session_state, camera_state, gumball_state, scene_state, Some((intent.x, intent.y)));
                        match commit.and_then(|(object_id, position)| plan_world3d_relocate(session_state, generation, &object_id, position)) {
                            Some(plan) => {
                                self.active = Some(WorldInteractionActive::Plan { plan, retirement: None });
                                context.consume_fuel(1);
                                return WorldInteractionAuthorityStep::Pending;
                            }
                            None => {
                                self.queue.retire_front(intent.generation);
                                context.consume_fuel(1);
                                return WorldInteractionAuthorityStep::Complete;
                            }
                        }
                    }
                }
                if intent.down && intent.button == 0 && selection_state.interaction_mode == "paint" && !selection_state.paint_stroke_active {
                    // 🖌️ A press opens the paint gesture and dabs where it lands (a `paintAt` stream tick when the ray hits
                    // a mesh); the moves stream the drag's dabs and the release commits them as ONE edit.
                    selection_state.paint_stroke_active = true;
                    session_state.interaction_revision = session_state.interaction_revision.wrapping_add(1);
                    WorldRayPickCursor::new(session_state, camera_state, generation, WorldRayPickPurpose::Paint, intent.x, intent.y).map(|cursor| WorldInteractionActive::Pick { cursor, retirement: None })
                } else if let Some(plan) = (!intent.down).then(|| plan_world3d_paint_release(session_state, selection_state, generation, intent.button)).flatten() {
                    Some(WorldInteractionActive::Plan { plan, retirement: None })
                } else if !intent.down && intent.button == 0 && session_state.active_utility == "surfaceBrush" {
                    WorldRayPickCursor::new(session_state, camera_state, generation, WorldRayPickPurpose::Surface, intent.x, intent.y).map(|cursor| WorldInteractionActive::Pick { cursor, retirement: None })
                } else if intent.down && intent.button == 0 && (session_state.active_utility == "brush" || (session_state.active_utility == "select" && selection_state.granularity == "vertex")) {
                    WorldObjectPickCursor::new(session_state, camera_state, generation, WorldObjectPickPurpose::VortexSelect, intent.x, intent.y).map(|mut cursor| {
                        cursor.merge = world_merge_code(intent.shift, intent.ctrl, intent.meta);
                        WorldInteractionActive::ObjectPick { cursor, retirement: None }
                    })
                } else if !intent.down && intent.button == 0 && session_state.active_utility == "select" && component_mode_active(selection_state) {
                    WorldComponentPickCursor::new(session_state, camera_state, selection_state, generation, WorldComponentPickPurpose::Select, intent.x, intent.y).map(|mut cursor| {
                        cursor.merge = world_merge_code(intent.shift, intent.ctrl, intent.meta);
                        WorldInteractionActive::ComponentPick { cursor, retirement: None }
                    })
                } else if intent.down && intent.button == 0 && session_state.active_utility == "select" && component_mode_active(selection_state) {
                    self.queue.retire_front(intent.generation);
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Complete;
                } else if intent.button == 0 && session_state.active_utility == "select" && !component_mode_active(selection_state) {
                    if intent.down {
                        self.queue.retire_front(intent.generation);
                        context.consume_fuel(1);
                        return WorldInteractionAuthorityStep::Complete;
                    }
                    WorldRayPickCursor::new(session_state, camera_state, generation, WorldRayPickPurpose::Instance, intent.x, intent.y).map(|mut cursor| {
                        cursor.merge = world_merge_code(intent.shift, intent.ctrl, intent.meta);
                        WorldInteractionActive::Pick { cursor, retirement: None }
                    })
                } else {
                    None
                }
            }
            WorldInteractionPhase::Cancel => None,
            WorldInteractionPhase::Close => {
                self.closing = true;
                self.queue.begin_close();
                None
            }
        };
        let Some(active) = active else {
            if intent.phase == WorldInteractionPhase::Close {
                return WorldInteractionAuthorityStep::Pending;
            }
            if intent.phase == WorldInteractionPhase::PointerLeave {
                self.queue.retire_front(intent.generation);
                context.consume_fuel(1);
                return WorldInteractionAuthorityStep::Complete;
            }
            if !world_pick_rect(session_state).contains(intent.x, intent.y) {
                self.queue.retire_front(intent.generation);
                context.consume_fuel(1);
                return WorldInteractionAuthorityStep::Complete;
            }
            self.faulted = true;
            return WorldInteractionAuthorityStep::Fault;
        };
        self.active = Some(active);
        context.consume_fuel(1);
        WorldInteractionAuthorityStep::Pending
    }

    /// 🧲️ A hover that hit NEITHER a mesh nor a sub-object pick target falls through to
    /// the reference-plane tier; one that hit a pick target has its answer already.
    fn step_active(&mut self, active: WorldInteractionActive, session_state: &mut WorldSessionState, camera_state: &mut WorldCameraState, selection_state: &mut WorldSelectionState, gumball_state: &mut WorldGumballState, interaction_state: &mut WorldInteractionState, scene_state: &mut WorldSceneState, generation: u64, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, context: &mut semio_framework_job::StepContext<'_>) -> WorldInteractionAuthorityStep {
        match active {
            WorldInteractionActive::Plan { mut plan, retirement } => {
                if let Some(outcome) = retirement {
                    if plan.close_step() {
                        self.queue.retire_front(plan.generation);
                        return outcome;
                    }
                    self.active = Some(WorldInteractionActive::Plan { plan, retirement: Some(outcome) });
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                match publish_world3d_plan_step(session_state, camera_state, selection_state, scene_state, &mut plan, generation, input, context) {
                    Ok(WorldInteractionStep::Pending) => {
                        self.active = Some(WorldInteractionActive::Plan { plan, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    Ok(WorldInteractionStep::Complete) => {
                        self.queue.retire_front(plan.generation);
                        WorldInteractionAuthorityStep::Complete
                    }
                    Ok(WorldInteractionStep::Stale) => {
                        self.active = Some(WorldInteractionActive::Plan { plan, retirement: Some(WorldInteractionAuthorityStep::Stale) });
                        WorldInteractionAuthorityStep::Pending
                    }
                    Ok(WorldInteractionStep::Fault) => {
                        self.active = Some(WorldInteractionActive::Plan { plan, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                    Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits | ui_wgpu::wgpu::BoundedActionFault::ByteCredits) => {
                        self.active = Some(WorldInteractionActive::Plan { plan, retirement: None });
                        WorldInteractionAuthorityStep::OutputBlocked
                    }
                    Err(_) => {
                        self.active = Some(WorldInteractionActive::Plan { plan, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                }
            }
            WorldInteractionActive::Pick { mut cursor, retirement } => {
                if let Some(outcome) = retirement {
                    if cursor.close_step() {
                        self.queue.retire_front(cursor.generation);
                        return outcome;
                    }
                    self.active = Some(WorldInteractionActive::Pick { cursor, retirement: Some(outcome) });
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                match cursor.step(selection_state, gumball_state, scene_state, generation, context) {
                    WorldInteractionStep::Pending => {
                        self.active = Some(WorldInteractionActive::Pick { cursor, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Complete if cursor.purpose == WorldRayPickPurpose::Hover && cursor.best.is_none() && cursor.best_target.is_none() => {
                        self.active = Some(WorldInteractionActive::ObjectPick { cursor: WorldObjectPickCursor::from_ray(cursor.revision, cursor.generation, WorldObjectPickPurpose::ReferenceHover, cursor.origin, cursor.direction), retirement: None });
                        context.consume_fuel(1);
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Complete if cursor.purpose == WorldRayPickPurpose::Instance && cursor.best.is_none() && cursor.best_target.is_none() => {
                        let mut reference = WorldObjectPickCursor::from_ray(cursor.revision, cursor.generation, WorldObjectPickPurpose::ReferenceSelect, cursor.origin, cursor.direction);
                        reference.merge = cursor.merge;
                        self.active = Some(WorldInteractionActive::ObjectPick { cursor: reference, retirement: None });
                        context.consume_fuel(1);
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Complete => match cursor.finish_plan(session_state, selection_state, interaction_state, scene_state, generation) {
                        Ok(Some(plan)) => {
                            self.active = Some(WorldInteractionActive::Plan { plan, retirement: None });
                            WorldInteractionAuthorityStep::Pending
                        }
                        Ok(None) => {
                            self.queue.retire_front(cursor.generation);
                            WorldInteractionAuthorityStep::Complete
                        }
                        Err(WorldInteractionStep::Pending) => {
                            self.active=Some(WorldInteractionActive::Pick{cursor,retirement:None});
                            context.consume_fuel(1);
                            WorldInteractionAuthorityStep::Pending
                        }
                        Err(outcome) => {
                            let outcome = if outcome == WorldInteractionStep::Stale { WorldInteractionAuthorityStep::Stale } else { WorldInteractionAuthorityStep::Fault };
                            self.active = Some(WorldInteractionActive::Pick { cursor, retirement: Some(outcome) });
                            WorldInteractionAuthorityStep::Pending
                        }
                    },
                    WorldInteractionStep::Stale => {
                        self.active = Some(WorldInteractionActive::Pick { cursor, retirement: Some(WorldInteractionAuthorityStep::Stale) });
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Fault => {
                        self.active = Some(WorldInteractionActive::Pick { cursor, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                }
            }
            WorldInteractionActive::ObjectPick { mut cursor, retirement } => {
                if let Some(outcome) = retirement {
                    if cursor.close_step() {
                        self.queue.retire_front(cursor.generation);
                        return outcome;
                    }
                    self.active = Some(WorldInteractionActive::ObjectPick { cursor, retirement: Some(outcome) });
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                match cursor.step(selection_state, gumball_state, scene_state, generation, context) {
                    WorldInteractionStep::Pending => {
                        self.active = Some(WorldInteractionActive::ObjectPick { cursor, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Complete => match cursor.finish_plan(session_state, selection_state, interaction_state, scene_state, generation) {
                        Ok(Some(plan)) => {
                            self.active = Some(WorldInteractionActive::Plan { plan, retirement: None });
                            WorldInteractionAuthorityStep::Pending
                        }
                        Ok(None) => {
                            self.queue.retire_front(cursor.generation);
                            WorldInteractionAuthorityStep::Complete
                        }
                        Err(outcome) => {
                            let outcome = if outcome == WorldInteractionStep::Stale { WorldInteractionAuthorityStep::Stale } else { WorldInteractionAuthorityStep::Fault };
                            self.active = Some(WorldInteractionActive::ObjectPick { cursor, retirement: Some(outcome) });
                            WorldInteractionAuthorityStep::Pending
                        }
                    },
                    WorldInteractionStep::Stale => {
                        self.active = Some(WorldInteractionActive::ObjectPick { cursor, retirement: Some(WorldInteractionAuthorityStep::Stale) });
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Fault => {
                        self.active = Some(WorldInteractionActive::ObjectPick { cursor, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                }
            }
            WorldInteractionActive::ComponentPick { mut cursor, retirement } => {
                if let Some(outcome) = retirement {
                    if cursor.close_step() {
                        self.queue.retire_front(cursor.generation);
                        return outcome;
                    }
                    self.active = Some(WorldInteractionActive::ComponentPick { cursor, retirement: Some(outcome) });
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                match cursor.step(selection_state, gumball_state, scene_state, generation, context) {
                    WorldInteractionStep::Pending => {
                        self.active = Some(WorldInteractionActive::ComponentPick { cursor, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Complete => match cursor.finish_plan(session_state, selection_state, interaction_state, scene_state, generation) {
                        Ok(Some(plan)) => {
                            self.active = Some(WorldInteractionActive::Plan { plan, retirement: None });
                            WorldInteractionAuthorityStep::Pending
                        }
                        Ok(None) => {
                            self.queue.retire_front(cursor.generation);
                            WorldInteractionAuthorityStep::Complete
                        }
                        Err(outcome) => {
                            let outcome = if outcome == WorldInteractionStep::Stale { WorldInteractionAuthorityStep::Stale } else { WorldInteractionAuthorityStep::Fault };
                            self.active = Some(WorldInteractionActive::ComponentPick { cursor, retirement: Some(outcome) });
                            WorldInteractionAuthorityStep::Pending
                        }
                    },
                    WorldInteractionStep::Stale => {
                        self.active = Some(WorldInteractionActive::ComponentPick { cursor, retirement: Some(WorldInteractionAuthorityStep::Stale) });
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Fault => {
                        self.active = Some(WorldInteractionActive::ComponentPick { cursor, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                }
            }

            WorldInteractionActive::MarqueePick { mut cursor, retirement } => {
                if let Some(outcome) = retirement {
                    if cursor.close_step() {
                        self.queue.retire_front(cursor.generation);
                        return outcome;
                    }
                    self.active = Some(WorldInteractionActive::MarqueePick { cursor, retirement: Some(outcome) });
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                match cursor.step(selection_state, gumball_state, scene_state, generation, context) {
                    WorldInteractionStep::Pending => {
                        self.active = Some(WorldInteractionActive::MarqueePick { cursor, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Complete => {
                        let cursor_generation = cursor.generation;
                        let component_kind = cursor.component_kind;
                        match cursor.finish(session_state, gumball_state, generation) {
                            Ok((gesture, results)) => {
                                let Some(intent) = self.queue.front().copied().filter(|intent| intent.generation == cursor_generation) else {
                                    self.faulted = true;
                                    return WorldInteractionAuthorityStep::Fault;
                                };
                                self.active = if let Some(kind) = component_kind {
                                    Some(WorldInteractionActive::ComponentMarqueePublish { job: WorldComponentMarqueePublishJob::new(cursor_generation, gesture, results, kind, intent.shift, intent.ctrl), retirement: None })
                                } else {
                                    Some(WorldInteractionActive::MarqueePublish { job: WorldMarqueePublishJob::new(cursor_generation, gesture, results, intent.shift, intent.ctrl), retirement: None })
                                };
                                context.consume_fuel(1);
                                WorldInteractionAuthorityStep::Pending
                            }
                            Err((cursor, outcome)) => {
                                let outcome = if outcome == WorldInteractionStep::Stale { WorldInteractionAuthorityStep::Stale } else { WorldInteractionAuthorityStep::Fault };
                                self.active = Some(WorldInteractionActive::MarqueePick { cursor, retirement: Some(outcome) });
                                WorldInteractionAuthorityStep::Pending
                            }
                        }
                    }
                    WorldInteractionStep::Stale => {
                        self.active = Some(WorldInteractionActive::MarqueePick { cursor, retirement: Some(WorldInteractionAuthorityStep::Stale) });
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Fault => {
                        self.active = Some(WorldInteractionActive::MarqueePick { cursor, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                }
            }
            WorldInteractionActive::MarqueePublish { mut job, retirement } => {
                if let Some(outcome) = retirement {
                    if job.close_step(input) {
                        self.queue.retire_front(job.generation);
                        return outcome;
                    }
                    self.active = Some(WorldInteractionActive::MarqueePublish { job, retirement: Some(outcome) });
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                match job.step(selection_state, gumball_state, scene_state, generation, input, context) {
                    Ok(WorldInteractionStep::Pending) => {
                        self.active = Some(WorldInteractionActive::MarqueePublish { job, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    Ok(WorldInteractionStep::Complete) => {
                        self.queue.retire_front(job.generation);
                        WorldInteractionAuthorityStep::Complete
                    }
                    Ok(WorldInteractionStep::Stale) => {
                        self.active = Some(WorldInteractionActive::MarqueePublish { job, retirement: Some(WorldInteractionAuthorityStep::Stale) });
                        WorldInteractionAuthorityStep::Pending
                    }
                    Ok(WorldInteractionStep::Fault) => {
                        self.active = Some(WorldInteractionActive::MarqueePublish { job, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                    Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits | ui_wgpu::wgpu::BoundedActionFault::ByteCredits) if job.prepared.is_none() => {
                        self.active = Some(WorldInteractionActive::MarqueePublish { job, retirement: None });
                        WorldInteractionAuthorityStep::OutputBlocked
                    }
                    Err(_) => {
                        self.active = Some(WorldInteractionActive::MarqueePublish { job, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                }
            }
            WorldInteractionActive::ComponentMarqueePublish { mut job, retirement } => {
                if let Some(outcome) = retirement {
                    if job.close_step(input) {
                        self.queue.retire_front(job.generation);
                        return outcome;
                    }
                    self.active = Some(WorldInteractionActive::ComponentMarqueePublish { job, retirement: Some(outcome) });
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                match job.step(selection_state, gumball_state, scene_state, generation, input, context) {
                    Ok(WorldInteractionStep::Pending) => {
                        self.active = Some(WorldInteractionActive::ComponentMarqueePublish { job, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    Ok(WorldInteractionStep::Complete) => {
                        self.queue.retire_front(job.generation);
                        WorldInteractionAuthorityStep::Complete
                    }
                    Ok(WorldInteractionStep::Stale) => {
                        self.active = Some(WorldInteractionActive::ComponentMarqueePublish { job, retirement: Some(WorldInteractionAuthorityStep::Stale) });
                        WorldInteractionAuthorityStep::Pending
                    }
                    Ok(WorldInteractionStep::Fault) => {
                        self.active = Some(WorldInteractionActive::ComponentMarqueePublish { job, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                    Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits | ui_wgpu::wgpu::BoundedActionFault::ByteCredits) if job.claim.is_none() => {
                        self.active = Some(WorldInteractionActive::ComponentMarqueePublish { job, retirement: None });
                        WorldInteractionAuthorityStep::OutputBlocked
                    }
                    Err(_) => {
                        self.active = Some(WorldInteractionActive::ComponentMarqueePublish { job, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                }
            }
            WorldInteractionActive::GumballPick { mut cursor, retirement } => {
                if let Some(outcome) = retirement {
                    if cursor.close_step() {
                        self.queue.retire_front(cursor.generation);
                        return outcome;
                    }
                    self.active = Some(WorldInteractionActive::GumballPick { cursor, retirement: Some(outcome) });
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                match cursor.step(selection_state, gumball_state, scene_state, generation, context) {
                    WorldInteractionStep::Pending => {
                        self.active = Some(WorldInteractionActive::GumballPick { cursor, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Complete => {
                        let cursor_generation = cursor.generation;
                        let cursor_revision = cursor.revision;
                        let point = [cursor.x, cursor.y];
                        match cursor.finish(session_state, gumball_state, generation) {
                            Ok(Some(gesture)) => {
                                self.gumball = Some(gesture);
                                self.queue.retire_front(cursor_generation);
                                WorldInteractionAuthorityStep::Complete
                            }
                            Ok(None) => {
                                self.marquee = Some(WorldMarqueeGesture::new(cursor_revision, cursor_generation, point));
                                self.queue.retire_front(cursor_generation);
                                WorldInteractionAuthorityStep::Complete
                            }
                            Err((cursor, outcome)) => {
                                let outcome = if outcome == WorldInteractionStep::Stale { WorldInteractionAuthorityStep::Stale } else { WorldInteractionAuthorityStep::Fault };
                                self.active = Some(WorldInteractionActive::GumballPick { cursor, retirement: Some(outcome) });
                                WorldInteractionAuthorityStep::Pending
                            }
                        }
                    }
                    WorldInteractionStep::Stale => {
                        self.active = Some(WorldInteractionActive::GumballPick { cursor, retirement: Some(WorldInteractionAuthorityStep::Stale) });
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Fault => {
                        self.active = Some(WorldInteractionActive::GumballPick { cursor, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                }
            }
            WorldInteractionActive::GumballCommit { mut job, retirement } => {
                if let Some(outcome) = retirement {
                    if job.close_step(input) {
                        self.queue.retire_front(job.generation);
                        return outcome;
                    }
                    self.active = Some(WorldInteractionActive::GumballCommit { job, retirement: Some(outcome) });
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                match job.step(selection_state, gumball_state, scene_state, generation, input, context) {
                    Ok(WorldInteractionStep::Pending) => {
                        self.active = Some(WorldInteractionActive::GumballCommit { job, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    Ok(WorldInteractionStep::Complete) if job.terminal_is_empty() => {
                        self.queue.retire_front(job.generation);
                        WorldInteractionAuthorityStep::Complete
                    }
                    Ok(WorldInteractionStep::Complete) => {
                        self.active = Some(WorldInteractionActive::GumballCommit { job, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    Ok(WorldInteractionStep::Stale) => {
                        self.active = Some(WorldInteractionActive::GumballCommit { job, retirement: Some(WorldInteractionAuthorityStep::Stale) });
                        WorldInteractionAuthorityStep::Pending
                    }
                    Ok(WorldInteractionStep::Fault) => {
                        self.active = Some(WorldInteractionActive::GumballCommit { job, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                    Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits | ui_wgpu::wgpu::BoundedActionFault::ByteCredits) if job.claim.is_none() => {
                        self.active = Some(WorldInteractionActive::GumballCommit { job, retirement: None });
                        WorldInteractionAuthorityStep::OutputBlocked
                    }
                    Err(_) => {
                        self.active = Some(WorldInteractionActive::GumballCommit { job, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                }
            }
            WorldInteractionActive::BrushCommit { mut job, retirement } => {
                if let Some(outcome) = retirement {
                    if job.close_step(input) {
                        self.queue.retire_front(job.generation);
                        return outcome;
                    }
                    self.active = Some(WorldInteractionActive::BrushCommit { job, retirement: Some(outcome) });
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                match job.step(selection_state, gumball_state, scene_state, generation, input, context) {
                    Ok(WorldInteractionStep::Pending) => {
                        self.active = Some(WorldInteractionActive::BrushCommit { job, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    Ok(WorldInteractionStep::Complete) => {
                        self.queue.retire_front(job.generation);
                        WorldInteractionAuthorityStep::Complete
                    }
                    Ok(WorldInteractionStep::Stale) => {
                        self.active = Some(WorldInteractionActive::BrushCommit { job, retirement: Some(WorldInteractionAuthorityStep::Stale) });
                        WorldInteractionAuthorityStep::Pending
                    }
                    Ok(WorldInteractionStep::Fault) => {
                        self.active = Some(WorldInteractionActive::BrushCommit { job, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                    Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits | ui_wgpu::wgpu::BoundedActionFault::ByteCredits) if job.claim.is_none() => {
                        self.active = Some(WorldInteractionActive::BrushCommit { job, retirement: None });
                        WorldInteractionAuthorityStep::OutputBlocked
                    }
                    Err(_) => {
                        self.active = Some(WorldInteractionActive::BrushCommit { job, retirement: Some(WorldInteractionAuthorityStep::Fault) });
                        self.faulted = true;
                        WorldInteractionAuthorityStep::Pending
                    }
                }
            }
        }
    }
}
