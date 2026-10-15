//! ⚙️ World interaction authority: mesh helpers, intent enqueue and step, the authority impl, plan fns and plan publication.

use super::*;

//#region 🧵️WorldInteractionTransaction
pub(in crate::world) fn world_mesh_vertex(mesh: Mesh3dLease, index: u32) -> Option<Vec3> {
    let point = mesh.vec3(Mesh3dField::Positions, index).ok()?;
    Some(Vec3::new(point[0], point[1], point[2]))
}

pub(in crate::world) fn world_mesh_triangle(mesh: Mesh3dLease, triangle: u32) -> Option<[u32; 3]> {
    let base = triangle.checked_mul(3)?;
    Some([mesh.u32(Mesh3dField::Indices, base).ok()?, mesh.u32(Mesh3dField::Indices, base + 1).ok()?, mesh.u32(Mesh3dField::Indices, base + 2).ok()?])
}

pub(in crate::world) fn world_mesh_component_id(mesh: Mesh3dLease, field: Mesh3dField, index: u32) -> u32 {
    mesh.u32(field, index).unwrap_or(if field == Mesh3dField::VertexIds { u32::MAX } else { index })
}

pub(super) fn world_ray_triangle_barycentric(origin: Vec3, direction: Vec3, a: Vec3, b: Vec3, c: Vec3) -> Option<(f32, f32, f32)> {
    let edge1 = b.sub(a);
    let edge2 = c.sub(a);
    let h = direction.cross(edge2);
    let determinant = edge1.dot(h);
    if determinant.abs() < 1e-8 {
        return None;
    }
    let inverse = 1.0 / determinant;
    let offset = origin.sub(a);
    let u = inverse * offset.dot(h);
    if !(0.0..=1.0).contains(&u) {
        return None;
    }
    let q = offset.cross(edge1);
    let v = inverse * direction.dot(q);
    if v < 0.0 || u + v > 1.0 {
        return None;
    }
    let distance = inverse * edge2.dot(q);
    (distance > 1e-6).then_some((distance, u, v))
}

pub fn enqueue_world3d_intent(state: &mut World3dState, intent: WorldInteractionIntent) -> Result<(), WorldInteractionIntent> {
    let Some(authority) = state.interaction_authority.as_mut() else {
        return Err(intent);
    };
    if authority.faulted || authority.closing || authority.blocked.is_some() || authority.next_generation == u64::MAX || intent.generation != authority.next_generation {
        return Err(intent);
    }
    match authority.queue.push(intent) {
        Ok(()) => {
            authority.next_generation += 1;
            Ok(())
        }
        Err(intent) if authority.blocked.is_none() => {
            authority.blocked = Some(intent);
            authority.next_generation += 1;
            Ok(())
        }
        Err(intent) => Err(intent),
    }
}

/** 🫧 Whether this surface's pointer MOVES may be folded onto one another
 * ([`WorldInteractionIntentQueue::coalesce_tail_pointer_move`]).
 *
 * Two gestures read the move SAMPLES rather than the latest point plus the accumulated delta, and
 * folding would change what they produce, which is the one thing the fold may never do:
 * - a LASSO marquee is its own path — `marquee_is_crossing_from_path` and the painted outline both
 *   walk every point, where a rectangle needs only its start and its newest corner;
 * - a live PAINT stroke raycasts once per move (`WorldRayPickPurpose::Paint`), so a folded move
 *   would leave a gap in the painted run.
 *
 * Everything else — hover, orbit/pan/zoom, the rectangle marquee, the vertex/brush object hover —
 * reads only `(x, y)` and the summed `(dx, dy)`, which the fold preserves exactly. */
pub fn world3d_pointer_move_folds(state: &World3dState) -> bool {
    state.selection_method != "lasso" && !(state.interaction_mode == "paint" && state.paint_stroke_active)
}

pub fn enqueue_world3d_event(state: &mut World3dState, mut intent: WorldInteractionIntent) -> Result<u64, WorldInteractionIntent> {
    let folds = world3d_pointer_move_folds(state);
    let Some(authority) = state.interaction_authority.as_mut() else {
        return Err(intent);
    };
    if folds && !authority.faulted && !authority.closing && authority.blocked.is_none() {
        if let Some(generation) = authority.queue.coalesce_tail_pointer_move(&intent) {
            return Ok(generation);
        }
    }
    intent.generation = authority.next_generation;
    let generation = intent.generation;
    enqueue_world3d_intent(state, intent).map(|()| generation)
}

pub fn enqueue_world3d_events<const N: usize>(state: &mut World3dState, mut intents: [WorldInteractionIntent; N]) -> Result<[u64; N], [WorldInteractionIntent; N]> {
    let Some(authority) = state.interaction_authority.as_mut() else {
        return Err(intents);
    };
    let available = if authority.blocked.is_some() { 0 } else { WORLD_INTERACTION_INTENT_CAPACITY - usize::from(authority.queue.len) + 1 };
    if N == 0 || N > 4 || authority.faulted || authority.closing || authority.next_generation.checked_add(N as u64).is_none() || N > available {
        return Err(intents);
    }
    let mut generations = [0; N];
    for index in 0..N {
        intents[index].generation = authority.next_generation;
        generations[index] = authority.next_generation;
        match authority.queue.push(intents[index]) {
            Ok(()) => {}
            Err(intent) if authority.blocked.is_none() => authority.blocked = Some(intent),
            Err(_) => unreachable!("world intent batch preflight owns exact fixed credits"),
        }
        authority.next_generation += 1;
    }
    Ok(generations)
}

/** 🎟️ The generation the frame driver must step this surface at, or `None` when it owes nothing.
 *
 * 🧭️ A drained queue is NOT idle while a camera settle is owed: the debounced `setCamera` (and the
 * `noteWorldNavigation` ahead of it, and the re-published hover behind it) are published by
 * `WorldInteractionAuthority::step` with no intent of their own, so they need a generation to be
 * stepped at. The LAST minted one is that generation — no queued intent can ever carry it again, so
 * the settle plan's `retire_front` is a no-op and a fresh intent supersedes it as `Stale`, which is
 * what a gesture that resumed mid-settle deserves. See [`WorldCameraSync`]. */
pub fn world3d_interaction_front_generation(state: &World3dState) -> Option<u64> {
    let authority = state.interaction_authority.as_ref()?;
    if let Some(intent) = authority.queue.front() {
        return Some(intent.generation);
    }
    let settle_owed = !authority.closing && (state.camera_sync.owed || (state.camera_sync.hover_owed && state.camera_sync.at.is_some()));
    (authority.active.is_some() || settle_owed).then(|| authority.next_generation.wrapping_sub(1))
}

pub fn step_world3d_interaction(state: &mut World3dState, generation: u64, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, context: &mut semio_framework_job::StepContext<'_>) -> WorldInteractionAuthorityStep {
    let Some(mut authority) = state.interaction_authority.take() else {
        return WorldInteractionAuthorityStep::Fault;
    };
    let step = authority.step(state, generation, input, context);
    state.interaction_authority = Some(authority);
    step
}

pub fn begin_world3d_interaction_close(state: &mut World3dState) {
    if let Some(authority) = state.interaction_authority.as_mut() {
        authority.closing = true;
        authority.queue.begin_close();
    }
}

pub fn close_world3d_interaction_step(state: &mut World3dState, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, context: &mut semio_framework_job::StepContext<'_>) -> bool {
    if context.should_yield() {
        return false;
    }
    let Some(authority) = state.interaction_authority.as_mut() else {
        return true;
    };
    authority.closing = true;
    authority.queue.begin_close();
    if let Some(active) = authority.active.as_mut() {
        let complete = match active {
            WorldInteractionActive::Plan { plan, .. } => plan.close_step(),
            WorldInteractionActive::Pick { cursor, .. } => cursor.close_step(),
            WorldInteractionActive::ObjectPick { cursor, .. } => cursor.close_step(),
            WorldInteractionActive::ComponentPick { cursor, .. } => cursor.close_step(),
            WorldInteractionActive::MarqueePick { cursor, .. } => cursor.close_step(),
            WorldInteractionActive::MarqueePublish { job, .. } => job.close_step(input),
            WorldInteractionActive::ComponentMarqueePublish { job, .. } => job.close_step(input),
            WorldInteractionActive::GumballPick { cursor, .. } => cursor.close_step(),
            WorldInteractionActive::GumballCommit { job, .. } => job.close_step(input),
            WorldInteractionActive::BrushCommit { job, .. } => job.close_step(input),
        };
        context.consume_fuel(1);
        if complete {
            authority.active = None;
        }
        return false;
    }
    if let Some(registry) = authority.registry.as_mut() {
        if registry.close_step(context) {
            authority.registry = None;
        }
        return false;
    }
    if authority.blocked.take().is_some() {
        context.consume_fuel(1);
        return false;
    }
    if authority.right_press.take().is_some() || authority.right_dragged {
        authority.right_dragged = false;
        context.consume_fuel(1);
        return false;
    }
    if let Some(marquee) = authority.marquee.as_mut() {
        if marquee.close_step() {
            authority.marquee = None;
        }
        context.consume_fuel(1);
        return false;
    }
    if let Some(gumball) = authority.gumball.as_mut() {
        if gumball.close_step() {
            authority.gumball = None;
        }
        context.consume_fuel(1);
        return false;
    }
    let complete = authority.queue.close_step();
    context.consume_fuel(1);
    complete
}

pub fn world3d_interaction_terminal_is_empty(state: &World3dState) -> bool {
    state.interaction_authority.as_ref().is_none_or(|authority| {
        authority.closing
            && authority.registry.is_none()
            && authority.active.is_none()
            && authority.right_press.is_none()
            && !authority.right_dragged
            && authority.marquee.is_none()
            && authority.gumball.is_none()
            && authority.blocked.is_none()
            && authority.queue.terminal_is_empty()
    })
}

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
    pub(in crate::world) fn step(&mut self, state: &mut World3dState, generation: u64, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, context: &mut semio_framework_job::StepContext<'_>) -> WorldInteractionAuthorityStep {
        if self.closing || context.should_yield() {
            return WorldInteractionAuthorityStep::Pending;
        }
        if let Some(active) = self.active.take() {
            return self.step_active(active, state, generation, input, context);
        }
        if self.faulted {
            return WorldInteractionAuthorityStep::Fault;
        }
        if self.marquee.as_ref().is_some_and(|marquee| marquee.revision != state.interaction_revision || marquee.retiring) {
            let complete = self.marquee.as_mut().expect("stale marquee owner").close_step();
            if complete {
                self.marquee = None;
            }
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Stale;
        }
        if self.gumball.as_ref().is_some_and(|gumball| gumball.revision != state.interaction_revision) {
            let complete = self.gumball.as_mut().expect("stale gumball owner").close_step();
            if complete {
                self.gumball = None;
            }
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Stale;
        }
        if state.interaction_objects.revision != state.interaction_revision || self.registry.is_some() {
            let mut registry = self.registry.take().unwrap_or_else(|| WorldInteractionRegistryBuildCursor::new(state.interaction_revision));
            match registry.step(state, context) {
                WorldInteractionStep::Pending => {
                    self.registry = Some(registry);
                    return WorldInteractionAuthorityStep::Pending;
                }
                WorldInteractionStep::Complete => {
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Pending;
                }
                WorldInteractionStep::Stale => {
                    self.registry = Some(WorldInteractionRegistryBuildCursor::new(state.interaction_revision));
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
            if let Some(plan) = plan_world3d_camera_settle(state, generation) {
                self.active = Some(WorldInteractionActive::Plan { plan, retirement: None });
                context.consume_fuel(1);
                return WorldInteractionAuthorityStep::Pending;
            }
            if let (true, Some([x, y])) = (state.camera_sync.hover_owed, state.camera_sync.at) {
                state.camera_sync.hover_owed = false;
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
            state.camera_sync.at = Some([intent.x, intent.y]);
            if intent.phase == WorldInteractionPhase::PointerButton && !intent.down {
                state.camera_sync.owed = true;
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
            if let Some(plan) = plan_world3d_paint_abort(state, generation, reason) {
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
            if let Some(active) = plan_world3d_non_primary_press_pick(state, generation, &intent) {
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
                if let Some(active) = plan_world3d_non_primary_press_pick(state, generation, &intent) {
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
            if let Some(plan) = plan_world3d_drag(state, generation, intent.dx, intent.dy, intent.button, &modifiers) {
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
        if intent.phase == WorldInteractionPhase::PointerButton && intent.button == 0 && intent.down && state.active_utility == "select" && self.gumball.is_none() && world3d_offers_transform_gumball(state) {
            self.active = Some(WorldInteractionActive::GumballPick { cursor: WorldGumballPickCursor::new(state, generation, intent.x, intent.y), retirement: None });
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
            match gumball.update_step(state) {
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
        if intent.phase == WorldInteractionPhase::PointerButton && intent.button == 0 && !intent.down && state.active_utility == "brush" {
            match WorldBrushCommitJob::new(state, intent.generation) {
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
        let select_marquee_route = state.active_utility == "select";
        if intent.phase == WorldInteractionPhase::PointerButton && intent.button == 0 && intent.down && select_marquee_route {
            if self.marquee.is_some() {
                self.faulted = true;
                return WorldInteractionAuthorityStep::Fault;
            }
            self.marquee = Some(WorldMarqueeGesture::new(state.interaction_revision, intent.generation, [intent.x, intent.y]));
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
            let Some(cursor) = WorldMarqueePickCursor::new(state, intent.generation, marquee) else {
                self.faulted = true;
                return WorldInteractionAuthorityStep::Fault;
            };
            self.active = Some(WorldInteractionActive::MarqueePick { cursor, retirement: None });
            context.consume_fuel(1);
            return WorldInteractionAuthorityStep::Pending;
        }
        let active = match intent.phase {
            WorldInteractionPhase::Wheel => plan_world3d_wheel(state, generation, intent.delta).map(|plan| WorldInteractionActive::Plan { plan, retirement: None }),
            WorldInteractionPhase::PointerLeave => plan_world3d_pointer_leave(state, generation).map(|plan| WorldInteractionActive::Plan { plan, retirement: None }),
            WorldInteractionPhase::PointerDrag => {
                if world3d_relocate_active(state) {
                    world3d_update_relocate_drag(state, intent.x, intent.y);
                    self.queue.retire_front(intent.generation);
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Complete;
                }
                let modifiers = PointerModifiers { shift: intent.shift, ctrl: intent.ctrl, alt: intent.alt, meta: intent.meta };
                let Some(plan) = plan_world3d_drag(state, generation, intent.dx, intent.dy, intent.button, &modifiers) else {
                    self.queue.retire_front(intent.generation);
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Complete;
                };
                Some(WorldInteractionActive::Plan { plan, retirement: None })
            }
            WorldInteractionPhase::PointerMove => {
                if world3d_relocate_active(state) {
                    world3d_update_relocate_drag(state, intent.x, intent.y);
                    self.queue.retire_front(intent.generation);
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Complete;
                }
                let modifiers = PointerModifiers { shift: intent.shift, ctrl: intent.ctrl, alt: intent.alt, meta: intent.meta };
                if intent.down {
                    if let Some(plan) = plan_world3d_drag(state, generation, intent.dx, intent.dy, intent.button, &modifiers) {
                        Some(WorldInteractionActive::Plan { plan, retirement: None })
                    } else if intent.button == 0 && state.interaction_mode == "paint" && state.paint_stroke_active {
                        WorldRayPickCursor::new(state, generation, WorldRayPickPurpose::Paint, intent.x, intent.y).map(|cursor| WorldInteractionActive::Pick { cursor, retirement: None })
                    } else {
                        None
                    }
                } else {
                    if state.active_utility == "brush" || (state.active_utility == "select" && state.granularity == "vertex") {
                        WorldObjectPickCursor::new(state, generation, WorldObjectPickPurpose::VortexHover, intent.x, intent.y).map(|cursor| WorldInteractionActive::ObjectPick { cursor, retirement: None })
                    } else if component_mode_active(state) {
                        WorldComponentPickCursor::new(state, generation, WorldComponentPickPurpose::Hover, intent.x, intent.y).map(|cursor| WorldInteractionActive::ComponentPick { cursor, retirement: None })
                    } else {
                        WorldRayPickCursor::new(state, generation, WorldRayPickPurpose::Hover, intent.x, intent.y).map(|cursor| WorldInteractionActive::Pick { cursor, retirement: None })
                    }
                }
            }
            WorldInteractionPhase::PointerButton => {
                if intent.button == 0 && (world3d_relocate_active(state) || world3d_relocate_mode(state)) {
                    if intent.down {
                        if world3d_begin_relocate_drag(state, intent.x, intent.y) {
                            self.queue.retire_front(intent.generation);
                            context.consume_fuel(1);
                            return WorldInteractionAuthorityStep::Complete;
                        }
                    } else if world3d_relocate_active(state) {
                        let commit = world3d_end_relocate_drag(state, Some((intent.x, intent.y)));
                        match commit.and_then(|(object_id, position)| plan_world3d_relocate(state, generation, &object_id, position)) {
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
                if intent.down && intent.button == 0 && state.interaction_mode == "paint" && !state.paint_stroke_active {
                    // 🖌️ A press opens the paint gesture and dabs where it lands (a `paintAt` stream tick when the ray hits
                    // a mesh); the moves stream the drag's dabs and the release commits them as ONE edit.
                    state.paint_stroke_active = true;
                    state.interaction_revision = state.interaction_revision.wrapping_add(1);
                    WorldRayPickCursor::new(state, generation, WorldRayPickPurpose::Paint, intent.x, intent.y).map(|cursor| WorldInteractionActive::Pick { cursor, retirement: None })
                } else if let Some(plan) = (!intent.down).then(|| plan_world3d_paint_release(state, generation, intent.button)).flatten() {
                    Some(WorldInteractionActive::Plan { plan, retirement: None })
                } else if !intent.down && intent.button == 0 && state.active_utility == "surfaceBrush" {
                    WorldRayPickCursor::new(state, generation, WorldRayPickPurpose::Surface, intent.x, intent.y).map(|cursor| WorldInteractionActive::Pick { cursor, retirement: None })
                } else if intent.down && intent.button == 0 && (state.active_utility == "brush" || (state.active_utility == "select" && state.granularity == "vertex")) {
                    WorldObjectPickCursor::new(state, generation, WorldObjectPickPurpose::VortexSelect, intent.x, intent.y).map(|mut cursor| {
                        cursor.merge = world_merge_code(intent.shift, intent.ctrl, intent.meta);
                        WorldInteractionActive::ObjectPick { cursor, retirement: None }
                    })
                } else if !intent.down && intent.button == 0 && state.active_utility == "select" && component_mode_active(state) {
                    WorldComponentPickCursor::new(state, generation, WorldComponentPickPurpose::Select, intent.x, intent.y).map(|mut cursor| {
                        cursor.merge = world_merge_code(intent.shift, intent.ctrl, intent.meta);
                        WorldInteractionActive::ComponentPick { cursor, retirement: None }
                    })
                } else if intent.down && intent.button == 0 && state.active_utility == "select" && component_mode_active(state) {
                    self.queue.retire_front(intent.generation);
                    context.consume_fuel(1);
                    return WorldInteractionAuthorityStep::Complete;
                } else if intent.button == 0 && state.active_utility == "select" && !component_mode_active(state) {
                    if intent.down {
                        self.queue.retire_front(intent.generation);
                        context.consume_fuel(1);
                        return WorldInteractionAuthorityStep::Complete;
                    }
                    WorldRayPickCursor::new(state, generation, WorldRayPickPurpose::Instance, intent.x, intent.y).map(|mut cursor| {
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
            if !world_pick_rect(state).contains(intent.x, intent.y) {
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
    fn step_active(&mut self, active: WorldInteractionActive, state: &mut World3dState, generation: u64, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, context: &mut semio_framework_job::StepContext<'_>) -> WorldInteractionAuthorityStep {
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
                match publish_world3d_plan_step(state, &mut plan, generation, input, context) {
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
                match cursor.step(state, generation, context) {
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
                    WorldInteractionStep::Complete => match cursor.finish_plan(state, generation) {
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
                match cursor.step(state, generation, context) {
                    WorldInteractionStep::Pending => {
                        self.active = Some(WorldInteractionActive::ObjectPick { cursor, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Complete => match cursor.finish_plan(state, generation) {
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
                match cursor.step(state, generation, context) {
                    WorldInteractionStep::Pending => {
                        self.active = Some(WorldInteractionActive::ComponentPick { cursor, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Complete => match cursor.finish_plan(state, generation) {
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
                match cursor.step(state, generation, context) {
                    WorldInteractionStep::Pending => {
                        self.active = Some(WorldInteractionActive::MarqueePick { cursor, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Complete => {
                        let cursor_generation = cursor.generation;
                        let component_kind = cursor.component_kind;
                        match cursor.finish(state, generation) {
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
                match job.step(state, generation, input, context) {
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
                match job.step(state, generation, input, context) {
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
                match cursor.step(state, generation, context) {
                    WorldInteractionStep::Pending => {
                        self.active = Some(WorldInteractionActive::GumballPick { cursor, retirement: None });
                        WorldInteractionAuthorityStep::Pending
                    }
                    WorldInteractionStep::Complete => {
                        let cursor_generation = cursor.generation;
                        let cursor_revision = cursor.revision;
                        let point = [cursor.x, cursor.y];
                        match cursor.finish(state, generation) {
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
                match job.step(state, generation, input, context) {
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
                match job.step(state, generation, input, context) {
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

/** 🧭️ The plan a settled gesture owes: the classified `noteWorldNavigation` when a navigation
 * actually started AND moved the camera, then the one `setCamera` every pointer-up owes — in that
 * order, because React's `onNavigationGestures` is immediate and its `reportCamera` is debounced.
 * See [`WorldCameraSync`] for the reference's own two conditions; `None` when nothing is owed. */
/** 👻️ Whether this surface has a hover PUBLISHED to the guest — the only thing a pointer leave has
 * to answer. The renderer asks before it enqueues one, so a pointer travelling over chrome costs the
 * bounded intent queue nothing once the hover is clear (React's canvas receives no move at all while
 * the pointer is over a DOM layer above it). */
pub fn world3d_hover_is_published(state: &World3dState) -> bool {
    state.local_hover_id.is_some() || state.hovered_vortex_id.is_some() || state.hovered_component_id.is_some() || state.hovered_component_object_id.is_some() || state.hovered_component_mode.is_some()
}

/** 🚪️ Whether this surface OWES a hover clear that nothing has been enqueued for yet — the exact
 * predicate a per-FRAME caller must ask, as opposed to the per-EVENT
 * [`world3d_hover_is_published`] the renderer's move path asks.
 *
 * 🩸️ Without the queued term, a frame-driven clear re-enqueues one leave per frame: the published
 * hover only goes clear when the leave is ANSWERED, and while the leave sits behind the rest of the
 * queue the predicate stays true. Measured live at `context-menu` — `queued=64 blocked=true` on
 * `puzzle3d-main-perspective`, the bounded intent queue full of identical leaves, then
 * `bounded frame input action authority faulted` and a dead page
 * (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `🗑️generated/w13c-parity-run-20`). */
pub fn world3d_hover_clear_is_owed(state: &World3dState) -> bool {
    world3d_hover_is_published(state) && state.interaction_authority.as_ref().is_some_and(|authority| !authority.queue.holds_pointer_leave() && !authority.blocked.is_some_and(|intent| intent.phase == WorldInteractionPhase::PointerLeave))
}

/** 🚪️ The clear a [`WorldInteractionPhase::PointerLeave`] owes, in whichever hover lane this
 * surface currently publishes: `interactionHover` with empty targets in component mode, `worldVortexHover` with a
 * null `fullId` under the brush/vertex utilities, and `interactionHover` with empty `targets`
 * otherwise — the same three clears a hover pick that MISSES produces, which is what React gets from
 * r3f's `onPointerOut` (`🌍️world/🎨️r3f/🟦️.tsx`'s `dispatchInstanceHover`).
 *
 * ⚖️ `None` when nothing is published, so a leave over an already-clear surface is retired without a
 * journal entry — React's canvas does not re-clear a hover it never had. */
/// 🧲️ A leave clears whichever hover channel is live — the instance one OR the sub-object pick
/// target one; the `Hover` publish arm with no target clears both.
fn plan_world3d_pointer_leave(state: &World3dState, generation: u64) -> Option<WorldInteractionPlan> {
    let mut plan = WorldInteractionPlan::new(state.interaction_revision, generation);
    let controller = plan.push_string(&state.controller_id)?;
    let surface = plan.push_string(&state.surface_id)?;
    if component_mode_active(state) {
        let mode = state.hovered_component_mode.as_deref()?;
        let mode = plan.push_string(mode)?;
        let domain = plan.push_string(resolved_domain_id(state))?;
        let action = WorldFlatAction { kind: WorldFlatActionKind::Hover, strings: [Some(controller), Some(surface), None, Some(domain), Some(mode), None, None, None], numbers: [0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0] };
        return plan.push_action(action).then_some(plan);
    }
    if state.active_utility == "brush" || (state.active_utility == "select" && state.granularity == "vertex") {
        state.hovered_vortex_id.as_ref()?;
        let action = WorldFlatAction { kind: WorldFlatActionKind::VortexHover, strings: [Some(controller), Some(surface), None, None, None, None, None, None], numbers: [0.0; 10] };
        return plan.push_action(action).then_some(plan);
    }
    if state.local_hover_id.is_none() && state.pick_hover_key.is_none() {
        return None;
    }
    let domain = plan.push_string(resolved_domain_id(state))?;
    let action = WorldFlatAction { kind: WorldFlatActionKind::Hover, strings: [Some(controller), None, None, Some(domain), None, None, None, None], numbers: [0.0; 10] };
    plan.push_action(action).then_some(plan)
}

pub(in crate::world) fn plan_world3d_camera_settle(state: &World3dState, generation: u64) -> Option<WorldInteractionPlan> {
    if !state.camera_sync.owed {
        return None;
    }
    let camera = state.orbit.to_camera();
    let mut plan = WorldInteractionPlan::new(state.interaction_revision, generation);
    let controller = plan.push_string(&state.controller_id)?;
    let surface = plan.push_string(&state.surface_id)?;
    let gestures = state.camera_sync.start.map(|start| classify_world_navigation_gestures(start, world_navigation_snapshot(state))).unwrap_or([false; 3]);
    if gestures.iter().any(|moved| *moved) {
        let mut numbers = [0.0; 10];
        for (index, moved) in gestures.iter().enumerate() {
            numbers[index] = if *moved { 1.0 } else { 0.0 };
        }
        let navigation = WorldFlatAction { kind: WorldFlatActionKind::Navigation, strings: [Some(controller), Some(surface), None, None, None, None, None, None], numbers };
        if !plan.push_action(navigation) {
            return None;
        }
    }
    let sync = WorldFlatAction {
        kind: WorldFlatActionKind::Camera,
        strings: [Some(controller), Some(surface), None, None, None, None, None, None],
        numbers: [camera.position.x as f64, camera.position.y as f64, camera.position.z as f64, camera.target.x as f64, camera.target.y as f64, camera.target.z as f64, world3d_camera_zoom(&camera), 0.0, 0.0, 0.0],
    };
    plan.push_action(sync).then_some(plan)
}

pub fn plan_world3d_wheel(state: &World3dState, generation: u64, delta: f32) -> Option<WorldInteractionPlan> {
    let mut next = state.orbit.clone();
    next.zoom(delta);
    let camera = next.to_camera();
    let mut plan = WorldInteractionPlan::new(state.interaction_revision, generation);
    let controller = plan.push_string(&state.controller_id)?;
    let surface = plan.push_string(&state.surface_id)?;
    let action = WorldFlatAction {
        kind: WorldFlatActionKind::CameraLocal,
        strings: [Some(controller), Some(surface), None, None, None, None, None, None],
        numbers: [camera.position.x as f64, camera.position.y as f64, camera.position.z as f64, camera.target.x as f64, camera.target.y as f64, camera.target.z as f64, world3d_camera_zoom(&camera), delta as f64, 0.0, 0.0],
    };
    plan.push_action(action).then_some(plan)
}

/** 🖱️🎯️ The selection a NON-primary press publishes — the twin of React's own "a press selects,
 * whatever the button".
 *
 * React reaches it through R3F, not through a handler of its own: `🌐️World3dHost/🟦️.tsx`'s
 * `handlePointerDown` returns immediately for `event.button !== 0`, but the `<Canvas>`'s
 * `onPointerMissed={handleEmptyClick}` is raised by R3F on a `pointerdown` that resolved no
 * event-handling object — for ANY button — and `handleEmptyClick` publishes `interactionSelect` over
 * the target the pane is hovering (or an empty selection when it hovers nothing). A primary press
 * arms the marquee instead, and its own drag flag (`wasMarqueeDragRef`) then swallows the first of
 * the two calls; a middle or right press arms nothing, so both land. That is exactly the measured
 * shape: `orbit-drag` journals ONE `interactionSelect` on React, `pan-drag` and `context-menu` TWO,
 * and the wgpu pane journalled NONE for the latter two
 * (`🗑️generated/w12c-parity-run-19/steps.json`, ticket 26/09/17 packet W13c §1).
 *
 * 🧭️ The pick is the SAME one a primary click runs — [`WorldRayPickPurpose::Instance`], whose miss
 * arm publishes the empty selection React's own miss publishes — so the two buttons answer with one
 * verb in one lane rather than with a second, hand-rolled selection route. `None` outside that lane
 * (component mode and the brush/vertex utilities own their own press verbs) and for a point this
 * surface no longer covers, which retires the press as before. */
fn plan_world3d_non_primary_press_pick(state: &World3dState, generation: u64, intent: &WorldInteractionIntent) -> Option<WorldInteractionActive> {
    if state.active_utility != "select" || component_mode_active(state) || state.granularity == "vertex" {
        return None;
    }
    WorldRayPickCursor::new(state, generation, WorldRayPickPurpose::Instance, intent.x, intent.y).map(|mut cursor| {
        cursor.merge = world_merge_code(intent.shift, intent.ctrl, intent.meta);
        WorldInteractionActive::Pick { cursor, retirement: None }
    })
}

pub fn plan_world3d_drag(state: &World3dState, generation: u64, dx: f32, dy: f32, button: i16, modifiers: &PointerModifiers) -> Option<WorldInteractionPlan> {
    let operation = if button == 1 || (button == 2 && modifiers.shift) {
        1.0
    } else if button == 2 && (modifiers.alt || modifiers.meta) {
        2.0
    } else {
        return None;
    };
    let mut next = state.orbit.clone();
    if operation == 1.0 {
        next.pan(-dx, -dy);
    } else {
        next.orbit(dx, dy);
    }
    let camera = next.to_camera();
    let mut plan = WorldInteractionPlan::new(state.interaction_revision, generation);
    let controller = plan.push_string(&state.controller_id)?;
    let surface = plan.push_string(&state.surface_id)?;
    let action = WorldFlatAction {
        kind: WorldFlatActionKind::CameraLocal,
        strings: [Some(controller), Some(surface), None, None, None, None, None, None],
        numbers: [camera.position.x as f64, camera.position.y as f64, camera.position.z as f64, camera.target.x as f64, camera.target.y as f64, camera.target.z as f64, world3d_camera_zoom(&camera), dx as f64, operation, dy as f64],
    };
    plan.push_action(action).then_some(plan)
}

/// 🖌️ The release of a paint gesture: ONE `paintAt` with `phase: "commit"` that commits the app's open paint transaction,
/// clearing `paint_stroke_active` only once the action is reserved. `None` outside an open left-button paint gesture.
pub fn plan_world3d_paint_release(state: &World3dState, generation: u64, button: i16) -> Option<WorldInteractionPlan> {
    if state.interaction_mode != "paint" || button != 0 || !state.paint_stroke_active {
        return None;
    }
    let mut plan = WorldInteractionPlan::new(state.interaction_revision, generation);
    let controller = plan.push_string(&state.controller_id)?;
    let surface = plan.push_string(&state.surface_id)?;
    let action = WorldFlatAction { kind: WorldFlatActionKind::PaintCommit, strings: [Some(controller), Some(surface), None, None, None, None, None, None], numbers: [0.0; 10] };
    plan.push_action(action).then_some(plan)
}

/// 🧯️ Whether a host cancel of this surface would abort anything — a live gumball gesture that streamed into the guest's
/// open tool transaction, or an open paint stroke — so a closing shell defers retirement only when a cancel is owed.
pub fn world3d_cancel_owed(state: &World3dState) -> bool {
    let gumball = state.interaction_authority.as_ref().and_then(|authority| authority.gumball.as_ref()).is_some_and(|gesture| gesture.live && gesture.streamed);
    gumball || (state.interaction_mode == "paint" && state.paint_stroke_active)
}

/// 🧯️ A host cancel of an open paint stroke: ONE `paintAt` with `phase: "abort"` and the `reason`, clearing
/// `paint_stroke_active` only once the action is reserved. `None` outside an open paint stroke.
pub(in crate::world) fn plan_world3d_paint_abort(state: &World3dState, generation: u64, reason: WorldCancelReason) -> Option<WorldInteractionPlan> {
    if state.interaction_mode != "paint" || !state.paint_stroke_active {
        return None;
    }
    let mut plan = WorldInteractionPlan::new(state.interaction_revision, generation);
    let controller = plan.push_string(&state.controller_id)?;
    let surface = plan.push_string(&state.surface_id)?;
    let action = WorldFlatAction { kind: WorldFlatActionKind::PaintAbort, strings: [Some(controller), Some(surface), None, None, None, None, None, None], numbers: [f64::from(reason as i16), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0] };
    plan.push_action(action).then_some(plan)
}

/// 🎯️ `interactionSelect`/`interactionHover` carry `targets` as a JSON-encoded STRING, never as a
/// structured array.
///
/// ⚖️ The framework's own decoder is `parse_interaction_targets`
/// (`💻️os/🔨️modules/🔌️plugin/🦀️.rs`): it reads the arg with `DslValue::as_str` and hands the text to
/// `serde_json`, and the manifest declares the arg as `ActionArgDef::text("targets")`. An ARRAY makes
/// `as_str` answer `None`, so the guest faults the job with `missing required arg targets` — and
/// because a reserved tool job's fault never reaches the dispatch promise, the shell saw a clean
/// dispatch and the selection simply never moved. Measured on 6118 across hover, click, shift-click,
/// empty-click and marquee (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️wgpu-world3d-interaction-2026-09-13.md`). React encodes the same way —
/// `world3dSelectionActionArgs`/`world3dHoverActionArgs` both `JSON.stringify(targets)`.
///
/// The three spans are written through `string_joined`, so the JSON text costs no allocation and
/// stays inside the action's own byte credits.
pub(super) const INTERACTION_TARGETS_ARRAY_OPEN: &str = "[";
pub(super) const INTERACTION_TARGETS_ARRAY_CLOSE: &str = "]";
pub(super) const INTERACTION_TARGETS_OPEN: &str = "{\"granularity\":\"";
pub(super) const INTERACTION_TARGETS_MIDDLE: &str = "\",\"id\":\"";
pub(super) const INTERACTION_TARGETS_TAIL: &str = "\"}";
pub(in crate::world) const INTERACTION_TARGETS_EMPTY: &str = "[]";

/// 🎯️ Writes the `targets` arg of one `interactionSelect`/`interactionHover` — see
/// [`INTERACTION_TARGETS_OPEN`]. `bare_id` is true when a real app domain is bound, in which case the
/// target id is the topology id alone; the shared `world` domain is `PathDelimited` and prefixes the
/// surface. A miss on any of the three spans is an EMPTY target list — the clear, which the fixture's
/// `background-click-clears` case requires to be a dispatch rather than a silence.
pub(super) struct WorldInteractionText {
    pub(in crate::world) bytes: [u8; ui_wgpu::wgpu::ACTION_STRING_BYTE_CAPACITY],
    pub(in crate::world) len: usize,
}

impl Default for WorldInteractionText {
    fn default() -> Self { Self { bytes: [0; ui_wgpu::wgpu::ACTION_STRING_BYTE_CAPACITY], len: 0 } }
}

impl std::fmt::Write for WorldInteractionText {
    fn write_str(&mut self, text: &str) -> std::fmt::Result {
        let end = self.len.checked_add(text.len()).filter(|end| *end <= self.bytes.len()).ok_or(std::fmt::Error)?;
        self.bytes[self.len..end].copy_from_slice(text.as_bytes()); self.len = end; Ok(())
    }
}

impl WorldInteractionText {
    pub(in crate::world) fn as_str(&self) -> &str { std::str::from_utf8(&self.bytes[..self.len]).expect("interaction text retains valid UTF-8") }
}

fn world_write_json_string(output: &mut impl std::fmt::Write, parts: &[&str]) -> std::fmt::Result {
    output.write_char('"')?;
    for character in parts.iter().flat_map(|part| part.chars()) {
        match character {
            '"' => output.write_str("\\\"")?,
            '\\' => output.write_str("\\\\")?,
            '\n' => output.write_str("\\n")?,
            '\r' => output.write_str("\\r")?,
            '\t' => output.write_str("\\t")?,
            character if character < ' ' => write!(output, "\\u{:04x}", character as u32)?,
            character => output.write_char(character)?,
        }
    }
    output.write_char('"')
}

pub(super) fn world_write_interaction_target(output: &mut impl std::fmt::Write, granularity: &str, id: &[&str]) -> std::fmt::Result {
    output.write_str("{\"granularity\":")?;
    world_write_json_string(output, &[granularity])?;
    output.write_str(",\"id\":")?;
    world_write_json_string(output, id)?;
    output.write_char('}')
}

pub(in crate::world) fn world_interaction_targets(surface: Option<&str>, object: Option<&str>, granularity: Option<&str>, bare_id: bool) -> Result<WorldInteractionText, ui_wgpu::wgpu::BoundedActionFault> {
    use std::fmt::Write;
    let mut text = WorldInteractionText::default();
    text.write_char('[').map_err(|_| ui_wgpu::wgpu::BoundedActionFault::StringCredits)?;
    if let (Some(surface), Some(object), Some(granularity)) = (surface, object, granularity) {
        let id = if bare_id { ["", "", object] } else { [surface, WORLD_ITEM_PATH_DELIMITER, object] };
        world_write_interaction_target(&mut text, granularity, &id).map_err(|_| ui_wgpu::wgpu::BoundedActionFault::StringCredits)?;
    }
    text.write_char(']').map_err(|_| ui_wgpu::wgpu::BoundedActionFault::StringCredits)?;
    Ok(text)
}

/// 🪟️ `windowId`, not `surfaceId`. `setCamera` is a WINDOW-OWNED action: the app declares
/// it on its preview window kinds only, and the shell resolves
/// `ActionAddress::window_instance_id` from this very argument
/// (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`'s `dispatch_action`). Without it the address fell back
/// to the focused window and the guest refused, measured verbatim on 6118 as
/// `handle_action promise failed: window kind procedural-main does not own action
/// setCamera` — the same defect class as the retained row's `addGeneration`
/// (`📓️wgpu-input-hit-runtime-2026-09-13.md` §10.3). A World3d surface IS keyed by its
/// window instance id (`ShellState::world3d_states`), so its own id is that address, and
/// React's `worldCameraSetCameraDispatchArgs` sends exactly `{windowId, camera}`.
///
/// 📷️ `zoom`/`up`, never `fov`: `buildWorldCameraDispatchArgs` sends exactly
/// `{position, target, zoom, up?}`, and the guest camera value has no `fov` member — a
/// `fov` key makes the whole `camera` object fail deserialization and silently drops the
/// dispatch, the same trap React's own docstring records for a bare `projection` string.
///
/// 🚚️ React sends `{objectId, position}` and nothing else (`dispatch("worldRelocate", args)`
/// with `world3dRelocateDispatchArgsV1`'s own return); `surfaceId` rides along the way every
/// other verb on this surface carries it.
///
/// 🧲️ `numbers[1]` marks a SUB-OBJECT hover (`WorldRayPickCursor::pick_target_plan`): the
/// published target is a pick target's `kind:id`, so the dedupe key it updates is
/// `pick_hover_key`, not the instance-level `local_hover_id`. A CLEAR (no object) clears
/// both, because a move off the model leaves neither channel hovered.
pub fn publish_world3d_plan_step(
    state: &mut World3dState,
    plan: &mut WorldInteractionPlan,
    generation: u64,
    input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>,
    context: &mut semio_framework_job::StepContext<'_>,
) -> Result<WorldInteractionStep, ui_wgpu::wgpu::BoundedActionFault> {
    if context.should_yield() {
        return Ok(WorldInteractionStep::Pending);
    }
    if plan.faulted {
        return Ok(WorldInteractionStep::Fault);
    }
    if plan.generation != generation || plan.revision != state.interaction_revision {
        return Ok(WorldInteractionStep::Stale);
    }
    if plan.cursor == plan.action_len {
        plan.byte_len = 0;
        return Ok(WorldInteractionStep::Complete);
    }
    let action = plan.actions[usize::from(plan.cursor)].expect("world interaction action cursor remains admitted");
    match action.kind {
        WorldFlatActionKind::Camera => {
            let controller = plan.string(action.strings[0].expect("camera controller span"));
            let surface = plan.string(action.strings[1].expect("camera surface span"));
            let action_id = "setCamera";
            let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "windowId", surface, "camera", "position", "target", "zoom", "up"])?;
            let mut reservation = input.reserve_action(controller, action_id, bytes)?;
            let builder = reservation.builder();
            builder.begin_object(None)?;
            builder.string(Some("windowId"), surface)?;
            builder.begin_object(Some("camera"))?;
            builder.begin_array(Some("position"))?;
            for value in &action.numbers[..3] {
                builder.number(None, *value)?;
            }
            builder.end_container()?;
            builder.begin_array(Some("target"))?;
            for value in &action.numbers[3..6] {
                builder.number(None, *value)?;
            }
            builder.end_container()?;
            builder.number(Some("zoom"), action.numbers[6])?;
            builder.begin_array(Some("up"))?;
            for value in [state.orbit.up.x, state.orbit.up.y, state.orbit.up.z] {
                builder.number(None, f64::from(value))?;
            }
            builder.end_container()?;
            builder.end_container()?;
            builder.end_container()?;
            reservation.publish_with(|| {
                state.camera_sync = WorldCameraSync { hover_owed: true, at: state.camera_sync.at, ..WorldCameraSync::default() };
                advance_world3d_view_revision(state);
            })?;
        }
        WorldFlatActionKind::CameraLocal => {
            if !state.camera_sync.navigating {
                state.camera_sync.start = Some(world_navigation_snapshot(state));
                state.camera_sync.navigating = true;
            }
            let first = action.numbers[7] as f32;
            let operation = action.numbers[8] as u8;
            let second = action.numbers[9] as f32;
            match operation {
                0 => state.orbit.zoom(first),
                1 => state.orbit.pan(-first, -second),
                2 => state.orbit.orbit(first, second),
                _ => unreachable!("world camera plan operation is schema-bounded"),
            }
            state.camera_user_moved = true;
            state.camera_sync.owed = true;
            advance_world3d_view_revision(state);
        }
        WorldFlatActionKind::Navigation => {
            let controller = plan.string(action.strings[0].expect("navigation controller span"));
            let surface = plan.string(action.strings[1].expect("navigation surface span"));
            let action_id = "noteWorldNavigation";
            let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "windowId", surface, "gestures", WORLD_NAVIGATION_GESTURE_IDS[0], WORLD_NAVIGATION_GESTURE_IDS[1], WORLD_NAVIGATION_GESTURE_IDS[2]])?;
            let mut reservation = input.reserve_action(controller, action_id, bytes)?;
            let builder = reservation.builder();
            builder.begin_object(None)?;
            builder.string(Some("windowId"), surface)?;
            builder.begin_array(Some("gestures"))?;
            for (index, name) in WORLD_NAVIGATION_GESTURE_IDS.iter().enumerate() {
                if action.numbers[index] != 0.0 {
                    builder.string(None, name)?;
                }
            }
            builder.end_container()?;
            builder.end_container()?;
            reservation.publish()?;
        }
        WorldFlatActionKind::PaintCommit => {
            let controller = plan.string(action.strings[0].expect("paint controller span"));
            let surface = plan.string(action.strings[1].expect("paint surface span"));
            let action_id = "paintAt";
            let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "surfaceId", surface, "phase", "commit"])?;
            let mut reservation = input.reserve_action(controller, action_id, bytes)?;
            let builder = reservation.builder();
            builder.begin_object(None)?;
            builder.string(Some("surfaceId"), surface)?;
            builder.string(Some("phase"), "commit")?;
            builder.end_container()?;
            reservation.publish_with(|| {
                state.paint_stroke_active = false;
                state.interaction_revision = state.interaction_revision.wrapping_add(1);
            })?;
        }
        WorldFlatActionKind::PaintAbort => {
            let controller = plan.string(action.strings[0].expect("paint controller span"));
            let surface = plan.string(action.strings[1].expect("paint surface span"));
            let reason = WorldCancelReason::from_button(action.numbers[0] as i16).code();
            let action_id = "paintAt";
            let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "surfaceId", surface, "phase", "abort", "reason", reason])?;
            let mut reservation = input.reserve_action(controller, action_id, bytes)?;
            let builder = reservation.builder();
            builder.begin_object(None)?;
            builder.string(Some("surfaceId"), surface)?;
            builder.string(Some("phase"), "abort")?;
            builder.string(Some("reason"), reason)?;
            builder.end_container()?;
            reservation.publish_with(|| {
                state.paint_stroke_active = false;
                state.interaction_revision = state.interaction_revision.wrapping_add(1);
            })?;
        }
        WorldFlatActionKind::PaintAt => {
            let controller = plan.string(action.strings[0].expect("paint controller span"));
            let surface = plan.string(action.strings[1].expect("paint surface span"));
            let object = plan.string(action.strings[2].expect("paint object span"));
            let action_id = "paintAt";
            let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "surfaceId", surface, "objectId", object, "u", "v", "phase", "stream"])?;
            let mut reservation = input.reserve_action(controller, action_id, bytes)?;
            let builder = reservation.builder();
            builder.begin_object(None)?;
            builder.string(Some("surfaceId"), surface)?;
            builder.string(Some("objectId"), object)?;
            builder.number(Some("u"), action.numbers[0])?;
            builder.number(Some("v"), action.numbers[1])?;
            builder.string(Some("phase"), "stream")?;
            builder.end_container()?;
            reservation.publish()?;
        }
        WorldFlatActionKind::SurfacePlace => {
            let controller = plan.string(action.strings[0].expect("surface controller span"));
            let surface = plan.string(action.strings[1].expect("surface span"));
            let object = plan.string(action.strings[2].expect("surface object span"));
            let action_id = "worldSurfacePlace";
            let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "surfaceId", surface, "pane", surface, "objectId", object, "position", "normal"])?;
            let mut reservation = input.reserve_action(controller, action_id, bytes)?;
            let builder = reservation.builder();
            builder.begin_object(None)?;
            builder.string(Some("surfaceId"), surface)?;
            builder.string(Some("pane"), surface)?;
            builder.string(Some("objectId"), object)?;
            builder.begin_array(Some("position"))?;
            for value in &action.numbers[..3] {
                builder.number(None, *value)?;
            }
            builder.end_container()?;
            builder.begin_array(Some("normal"))?;
            for value in &action.numbers[3..6] {
                builder.number(None, *value)?;
            }
            builder.end_container()?;
            builder.end_container()?;
            reservation.publish()?;
        }
        WorldFlatActionKind::Relocate => {
            let controller = plan.string(action.strings[0].expect("relocate controller span"));
            let surface = plan.string(action.strings[1].expect("relocate surface span"));
            let object = plan.string(action.strings[2].expect("relocate object span"));
            let action_id = "worldRelocate";
            let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "surfaceId", surface, "objectId", object, "position"])?;
            let mut reservation = input.reserve_action(controller, action_id, bytes)?;
            let builder = reservation.builder();
            builder.begin_object(None)?;
            builder.string(Some("surfaceId"), surface)?;
            builder.string(Some("objectId"), object)?;
            builder.begin_array(Some("position"))?;
            for value in &action.numbers[..3] {
                builder.number(None, *value)?;
            }
            builder.end_container()?;
            builder.end_container()?;
            reservation.publish()?;
        }
        WorldFlatActionKind::Select => {
            let controller = plan.string(action.strings[0].expect("selection controller span"));
            let surface = action.strings[1].map(|span| plan.string(span));
            let object = action.strings[2].map(|span| plan.string(span));
            let domain = plan.string(action.strings[3].expect("selection domain span"));
            let granularity = action.strings[4].map(|span| plan.string(span));
            let merge = plan.string(action.strings[5].expect("selection merge span"));
            let method = plan.string(action.strings[6].expect("selection method span"));
            let action_id = "interactionSelect";
            let targets = world_interaction_targets(surface, object, granularity, action.numbers[0] != 0.0)?;
            let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "domainId", domain, "targets", targets.as_str(), "merge", merge, "method", method])?;
            let mut reservation = input.reserve_action(controller, action_id, bytes)?;
            let builder = reservation.builder();
            builder.begin_object(None)?;
            builder.string(Some("domainId"), domain)?;
            builder.string(Some("targets"), targets.as_str())?;
            builder.string(Some("merge"), merge)?;
            builder.string(Some("method"), method)?;
            builder.end_container()?;
            reservation.publish()?;
        }
        WorldFlatActionKind::Hover => {
            let controller = plan.string(action.strings[0].expect("hover controller span"));
            let domain = plan.string(action.strings[3].expect("hover domain span"));
            let object = action.strings[2].map(|span| plan.string(span));
            let surface = action.strings[1].map(|span| plan.string(span));
            let granularity = action.strings[4].map(|span| plan.string(span));
            let action_id = "interactionHover";
            let targets = world_interaction_targets(surface, object, granularity, action.numbers[0] != 0.0)?;
            let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "domainId", domain, "channel", "pointer", "targets", targets.as_str()])?;
            let mut reservation = input.reserve_action(controller, action_id, bytes)?;
            let builder = reservation.builder();
            builder.begin_object(None)?;
            builder.string(Some("domainId"), domain)?;
            builder.string(Some("channel"), "pointer")?;
            builder.string(Some("targets"), targets.as_str())?;
            builder.end_container()?;
            let sub_object = action.numbers[1] != 0.0;
            let pick_key = match (sub_object, granularity, object) {
                (true, Some(granularity), Some(object)) => Some(format!("{granularity}:{object}")),
                _ => None,
            };
            let component = action.numbers[2] != 0.0;
            let component_object = action.strings[7].map(|span| plan.string(span));
            reservation.publish_with(|| {
                if component {
                    state.hovered_component_id = component_object.map(|_| (action.numbers[3] as u32).to_string());
                    state.hovered_component_object_id = component_object.map(str::to_owned);
                    state.hovered_component_mode = component_object.and(granularity).map(str::to_owned);
                    state.pick_hover_key = pick_key;
                    if component_object.is_none() { state.local_hover_id = None; state.local_hover_granularity_id = None; }
                } else { match (sub_object, object) {
                    (true, Some(_)) => state.pick_hover_key = pick_key,
                    (true, None) => {
                        state.pick_hover_key = None;
                        state.local_hover_id = None;
                        state.local_hover_granularity_id = None;
                    }
                    (false, _) => {
                        state.local_hover_id = object.map(str::to_owned);
                        state.local_hover_granularity_id = granularity.map(str::to_owned);
                        if object.is_none() {
                            state.pick_hover_key = None;
                        }
                    }
                } }
                state.interaction_revision = state.interaction_revision.wrapping_add(1);
            })?;
        }
        WorldFlatActionKind::VortexHover => {
            let controller = plan.string(action.strings[0].expect("vortex hover controller span"));
            let surface = plan.string(action.strings[1].expect("vortex hover surface span"));
            let hit = action.strings[2].map(|span| plan.string(span));
            let action_id = "worldVortexHover";
            let bytes = match hit {
                Some(hit) => ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "surfaceId", surface, "fullId", hit])?,
                None => ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "surfaceId", surface, "fullId"])?,
            };
            let mut reservation = input.reserve_action(controller, action_id, bytes)?;
            let builder = reservation.builder();
            builder.begin_object(None)?;
            builder.string(Some("surfaceId"), surface)?;
            match hit {
                Some(hit) => builder.string(Some("fullId"), hit)?,
                None => builder.null(Some("fullId"))?,
            }
            builder.end_container()?;
            reservation.publish_with(|| {
                state.hovered_vortex_id = hit.map(str::to_owned);
                state.interaction_revision = state.interaction_revision.wrapping_add(1);
            })?;
        }
        WorldFlatActionKind::VortexSelect => {
            let controller = plan.string(action.strings[0].expect("vortex selection controller span"));
            let surface = plan.string(action.strings[1].expect("vortex selection surface span"));
            let hit = plan.string(action.strings[2].expect("vortex selection id span"));
            let merge = plan.string(action.strings[3].expect("vortex selection merge span"));
            let action_id = "worldVortexSelect";
            let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller, action_id, "surfaceId", surface, "fullId", hit, "merge", merge])?;
            let mut reservation = input.reserve_action(controller, action_id, bytes)?;
            let builder = reservation.builder();
            builder.begin_object(None)?;
            builder.string(Some("surfaceId"), surface)?;
            builder.string(Some("fullId"), hit)?;
            builder.string(Some("merge"), merge)?;
            builder.end_container()?;
            reservation.publish()?;
        }

    }
    plan.actions[usize::from(plan.cursor)] = None;
    plan.cursor += 1;
    plan.revision = state.interaction_revision;
    context.consume_fuel(1);
    Ok(WorldInteractionStep::Pending)
}
//#endregion 🧵️WorldInteractionTransaction
