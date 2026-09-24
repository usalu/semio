// #region 🧲️Header
/** @emoji 👆️ Pure multi-touch gesture math for every viewport surface — `pointerId`-keyed pointer
 * tracking plus the pinch (scale ⊗ translation ⊗ rotation) a two-finger gesture carries.
 *
 * Domain-neutral and renderer-neutral on purpose: `🖥️Board2dHost` drives a WASM board session,
 * `🌐️World3dHost` drives a Three orbit rig, and the wgpu target drives its own arena — all three ask
 * the SAME functions what two fingers just did, so a pinch can never mean one thing on one surface and
 * something else on another.
 *
 * Deliberately NOT in `🧬️schema/🔣️.json`: an active-pointer set is **ephemeral local** state (the same
 * axis as `DomainHover`, see the schema's own description) that never crosses a wire, is never
 * persisted, and is never broadcast to a peer. The schema owns what CROSSES a boundary; this owns what
 * a surface computes between two pointer events and throws away.
 *
 * The per-frame results belong in window TRANSIENT state (a silent local camera write), never in a
 * coalesced config amend — see `📓️project-per-frame-state-belongs-in-window-transient.md`. A gesture
 * commits exactly ONE authoritative pose when it ends.
 *
 * @see {@link https://www.w3.org/TR/pointerevents3/ | W3C Pointer Events Level 3}
 * @see `🧰️framework/🔨️modules/🕹️interaction/🟦️.ts` — the selection/hover machine this sits beside. */
// #endregion 🧲️Header

// #region 👆️Gesture
// #region 🔖️Pointers
/** 👆️ One live pointer contact, in the surface's own local pixel space (never client space, so a
 * scrolled or transformed host cannot skew the math). */
export type GesturePointer = {
  readonly pointerId: number;
  readonly x: number;
  readonly y: number;
};

/** 👆️ Every pointer currently down on one surface, in the order they went down: `[0]` is the primary
 * contact for as long as it lives, so a pinch never flips its own centroid when a third finger lands. */
export type GesturePointers = {
  readonly pointers: readonly GesturePointer[];
};

/** 👆️ No contact at all — the TS twin of an empty arena slot. */
export const EMPTY_GESTURE_POINTERS: GesturePointers = { pointers: [] };

/** 🔢️ How many simultaneous contacts make a gesture a MULTI-touch gesture rather than a drag. */
export const GESTURE_MULTI_TOUCH_POINTERS = 2;

/** 👆️ Adds (or re-seats, for a duplicate `pointerdown` the browser may replay) one contact, preserving
 * down-order for every pointer already tracked. */
export const gesturePointerDown = (state: GesturePointers, pointer: GesturePointer): GesturePointers => {
  const index = state.pointers.findIndex((candidate) => candidate.pointerId === pointer.pointerId);
  if (index === -1) return { pointers: [...state.pointers, pointer] };
  const pointers = [...state.pointers];
  pointers[index] = pointer;
  return { pointers };
};

/** 👆️ Moves one tracked contact. A move for an untracked `pointerId` is ignored rather than adopted —
 * a hover move on a touch-capable surface must never fabricate a contact. */
export const gesturePointerMove = (state: GesturePointers, pointer: GesturePointer): GesturePointers => {
  const index = state.pointers.findIndex((candidate) => candidate.pointerId === pointer.pointerId);
  if (index === -1) return state;
  const pointers = [...state.pointers];
  pointers[index] = pointer;
  return { pointers };
};

/** 👆️ Drops one contact (`pointerup`, `pointercancel`, `lostpointercapture`). Unknown ids are a no-op. */
export const gesturePointerUp = (state: GesturePointers, pointerId: number): GesturePointers => {
  const pointers = state.pointers.filter((candidate) => candidate.pointerId !== pointerId);
  return pointers.length === state.pointers.length ? state : { pointers };
};

/** 👆️ Whether at least {@link GESTURE_MULTI_TOUCH_POINTERS} contacts are down — the ONE predicate every
 * host uses to suppress its single-pointer lane (marquee, pick, brush) while a pinch owns the surface. */
export const gestureIsMultiTouch = (state: GesturePointers): boolean => state.pointers.length >= GESTURE_MULTI_TOUCH_POINTERS;
// #endregion 🔖️Pointers

// #region 🔖️Pinch
/** 🤏️ The measurable frame of a two-finger gesture: the midpoint the zoom anchors on, the separation
 * that carries the scale, and the angle that carries the rotation. */
export type PinchFrame = {
  readonly centroidX: number;
  readonly centroidY: number;
  readonly distance: number;
  readonly angle: number;
};

/** 🤏️ What one pinch STEP did, relative to the previous frame: a multiplicative `scale`, a translation
 * of the centroid in surface pixels, and a signed rotation in radians. */
export type PinchStep = {
  readonly scale: number;
  readonly panX: number;
  readonly panY: number;
  readonly rotation: number;
  readonly centroidX: number;
  readonly centroidY: number;
};

/** 🤏️ A pinch step that changed nothing — the identity every degenerate case collapses to. */
export const IDENTITY_PINCH_STEP: PinchStep = { scale: 1, panX: 0, panY: 0, rotation: 0, centroidX: 0, centroidY: 0 };

/** 🤏️ Separations below this many pixels are treated as coincident contacts: dividing by them turns
 * finger jitter into an unbounded zoom jump. */
export const PINCH_MIN_DISTANCE_PX = 1e-3;

/** 🤏️ The two-finger frame of the FIRST two contacts, or `null` when fewer than two are down. Extra
 * fingers are deliberately ignored rather than averaged in, so putting a third finger down mid-pinch
 * does not teleport the anchor. */
export const pinchFrame = (state: GesturePointers): PinchFrame | null => {
  const first = state.pointers[0];
  const second = state.pointers[1];
  if (!first || !second) return null;
  const dx = second.x - first.x;
  const dy = second.y - first.y;
  return {
    centroidX: (first.x + second.x) / 2,
    centroidY: (first.y + second.y) / 2,
    distance: Math.hypot(dx, dy),
    angle: Math.atan2(dy, dx),
  };
};

/** 🧮️ The shortest signed angle from `from` to `to`, in `(-π, π]` — a pinch that crosses the ±π seam
 * must report a small rotation, never a full turn. */
export const shortestAngleDelta = (from: number, to: number): number => {
  const twoPi = Math.PI * 2;
  const delta = (((to - from) % twoPi) + twoPi * 1.5) % twoPi - Math.PI;
  return delta === -Math.PI ? Math.PI : delta;
};

/**
 * 🤏️ Diffs two pinch frames into one step.
 *
 * `scale` is the separation ratio (>1 = fingers spreading = zoom in), `panX`/`panY` the centroid's
 * travel, `rotation` the shortest signed twist. A frame whose separation collapses below
 * {@link PINCH_MIN_DISTANCE_PX} contributes no scale and no rotation — only its translation — so two
 * fingers landing on the same pixel can never produce an infinite zoom.
 */
export const pinchStep = (previous: PinchFrame, next: PinchFrame): PinchStep => {
  const degenerate = previous.distance < PINCH_MIN_DISTANCE_PX || next.distance < PINCH_MIN_DISTANCE_PX;
  return {
    scale: degenerate ? 1 : next.distance / previous.distance,
    panX: next.centroidX - previous.centroidX,
    panY: next.centroidY - previous.centroidY,
    rotation: degenerate ? 0 : shortestAngleDelta(previous.angle, next.angle),
    centroidX: next.centroidX,
    centroidY: next.centroidY,
  };
};
// #endregion 🔖️Pinch

// #region 🔖️Recognizer
/** 🧭️ What ONE pointer event means for a viewport host, as decided by the shared recognizer:
 * - `single` — no multi-touch gesture owns the surface; the host runs its own single-pointer lane.
 * - `pinchBegin` — a second contact just landed; the host abandons its single-pointer gesture (marquee,
 *   drag, capture) exactly once.
 * - `pinch` — the two-finger frame moved; the host applies `step` to its camera.
 * - `held` — a multi-touch gesture owns the surface but this event carries no step; the host ignores it.
 * - `pinchEnd` — the LAST contact of a multi-touch gesture lifted; the host commits its pose once. */
export type GestureVerdict =
  | { readonly kind: "single" }
  | { readonly kind: "pinchBegin"; readonly frame: PinchFrame }
  | { readonly kind: "pinch"; readonly step: PinchStep }
  | { readonly kind: "held" }
  | { readonly kind: "pinchEnd" };

/** 🧭️ The recognizer's whole memory: the live contacts, the last two-finger frame, and the LATCH that
 * keeps a multi-touch gesture owning the surface until every finger has lifted — so the finger left
 * down after the other lifts never resumes the single-pointer lane (no stray marquee, drag or click). */
export type GestureRecognizerState = {
  readonly pointers: GesturePointers;
  readonly frame: PinchFrame | null;
  readonly latched: boolean;
};

/** 🧭️ No contact, no gesture. */
export const IDLE_GESTURE_RECOGNIZER: GestureRecognizerState = { pointers: EMPTY_GESTURE_POINTERS, frame: null, latched: false };

/** 🧭️ One recognizer step: the next state plus what the host must do with the event. */
export type GestureTransition = {
  readonly state: GestureRecognizerState;
  readonly verdict: GestureVerdict;
};

const SINGLE_VERDICT: GestureVerdict = { kind: "single" };
const HELD_VERDICT: GestureVerdict = { kind: "held" };
const PINCH_END_VERDICT: GestureVerdict = { kind: "pinchEnd" };

/** 🧭️ `pointerdown`: the second contact latches the surface and seeds the pinch frame; a contact landing
 * while latched re-seeds the frame from the first two contacts so the next step never jumps. */
export const recognizeGesturePointerDown = (state: GestureRecognizerState, pointer: GesturePointer): GestureTransition => {
  const pointers = gesturePointerDown(state.pointers, pointer);
  if (!gestureIsMultiTouch(pointers)) return { state: { ...state, pointers }, verdict: state.latched ? HELD_VERDICT : SINGLE_VERDICT };
  const frame = pinchFrame(pointers)!;
  if (state.latched) return { state: { pointers, frame, latched: true }, verdict: HELD_VERDICT };
  return { state: { pointers, frame, latched: true }, verdict: { kind: "pinchBegin", frame } };
};

/** 🧭️ `pointermove`: while two or more contacts are down the first two drive one {@link PinchStep}; a
 * move of an untracked pointer (mouse hover) passes through as `single` unless a gesture is latched. */
export const recognizeGesturePointerMove = (state: GestureRecognizerState, pointer: GesturePointer): GestureTransition => {
  const pointers = gesturePointerMove(state.pointers, pointer);
  if (!state.latched) return { state: pointers === state.pointers ? state : { ...state, pointers }, verdict: SINGLE_VERDICT };
  const frame = pinchFrame(pointers);
  if (!frame || !state.frame) return { state: { ...state, pointers, frame }, verdict: HELD_VERDICT };
  return { state: { pointers, frame, latched: true }, verdict: { kind: "pinch", step: pinchStep(state.frame, frame) } };
};

/** 🧭️ `pointerup` / `pointercancel` / `lostpointercapture`: re-seeds the frame from the contacts that
 * REMAIN (a pinch ending one finger at a time must not snap), and releases the latch with `pinchEnd`
 * only when the last contact lifts. */
export const recognizeGesturePointerUp = (state: GestureRecognizerState, pointerId: number): GestureTransition => {
  const pointers = gesturePointerUp(state.pointers, pointerId);
  if (!state.latched) return { state: pointers === state.pointers ? state : { ...state, pointers }, verdict: SINGLE_VERDICT };
  if (pointers.pointers.length === 0) return { state: IDLE_GESTURE_RECOGNIZER, verdict: pointers === state.pointers ? HELD_VERDICT : PINCH_END_VERDICT };
  return { state: { pointers, frame: pinchFrame(pointers), latched: true }, verdict: HELD_VERDICT };
};

/**
 * 🧭️ The ONE gesture recognizer every viewport host holds (`🖥️Board2dHost`, `🌐️World3dHost`, the dag
 * `🕸️NodeGraph` surface, …): a mutable cell around the pure `recognizeGesture*` transitions so a host
 * keeps one field instead of re-assembling pointer set, frame and latch by hand — the hand-assembled
 * copies were exactly where the "remaining finger resumes the single lane" defect lived.
 */
export class GestureRecognizer {
  #state: GestureRecognizerState = IDLE_GESTURE_RECOGNIZER;

  /** 🧭️ The current recognizer state (read-only snapshot). */
  get state(): GestureRecognizerState {
    return this.#state;
  }

  /** 🧭️ Every contact currently down, in down order. */
  get pointers(): readonly GesturePointer[] {
    return this.#state.pointers.pointers;
  }

  /** 🧭️ Whether a multi-touch gesture currently owns the surface. */
  get latched(): boolean {
    return this.#state.latched;
  }

  /** 🧭️ Feeds a `pointerdown`. */
  down(pointer: GesturePointer): GestureVerdict {
    return this.#apply(recognizeGesturePointerDown(this.#state, pointer));
  }

  /** 🧭️ Feeds a `pointermove`. */
  move(pointer: GesturePointer): GestureVerdict {
    return this.#apply(recognizeGesturePointerMove(this.#state, pointer));
  }

  /** 🧭️ Feeds a `pointerup`, `pointercancel` or `lostpointercapture`. */
  up(pointerId: number): GestureVerdict {
    return this.#apply(recognizeGesturePointerUp(this.#state, pointerId));
  }

  #apply(transition: GestureTransition): GestureVerdict {
    this.#state = transition.state;
    return transition.verdict;
  }
}
// #endregion 🔖️Recognizer

// #region 🔖️Zoom
/** 🔍️ Lower/upper bounds one surface clamps its zoom factor to. */
export type ZoomBounds = {
  readonly min: number;
  readonly max: number;
};

/** 🔍️ Clamps `zoom` into `bounds`; a non-finite or non-positive input falls back to `bounds.min`. */
export const clampZoom = (zoom: number, bounds: ZoomBounds): number => {
  if (!Number.isFinite(zoom) || zoom <= 0) return bounds.min;
  return Math.min(bounds.max, Math.max(bounds.min, zoom));
};

/** 🔍️ How many wheel pixels one doubling of the zoom factor is worth — the conversion constant that
 * lets a pinch reuse a surface's already-tested wheel zoom path (anchored at a point) instead of
 * growing a second, differently-behaved zoom implementation beside it. */
export const PINCH_WHEEL_PIXELS_PER_DOUBLING = 200;

/**
 * 🔍️ The wheel `deltaY` (in `DOM_DELTA_PIXEL` units) equivalent to a pinch `scale`.
 *
 * Wheel zoom is exponential in `-deltaY`, so the inverse is a base-2 log: `scale = 2` (fingers doubled
 * their separation) answers `-PINCH_WHEEL_PIXELS_PER_DOUBLING`, i.e. exactly one doubling of zoom IN,
 * and `scale = 0.5` answers the same magnitude OUT. A non-finite or non-positive scale answers `0`.
 */
export const pinchWheelDelta = (scale: number, pixelsPerDoubling: number = PINCH_WHEEL_PIXELS_PER_DOUBLING): number => {
  if (!Number.isFinite(scale) || scale <= 0) return 0;
  return -Math.log2(scale) * pixelsPerDoubling;
};

/** 📐️ Re-anchors a 2D camera so the world point under `anchor` stays under `anchor` across a zoom
 * change — the standard zoom-at-cursor transform, shared by pinch (anchor = centroid) and wheel
 * (anchor = cursor). `viewport` is the surface's size in the same pixel space as `anchor`.
 *
 * Uses the canonical board transform `screenX = (worldX - camera.x) * zoom + width / 2`
 * (`🖥️Board2dHost/🟦️.tsx`'s `puzzle2dWorldToScreen`), so a host can hand this its own camera record. */
export const zoomAboutPoint = (
  camera: { readonly x: number; readonly y: number; readonly zoom: number },
  anchor: { readonly x: number; readonly y: number },
  viewport: { readonly w: number; readonly h: number },
  nextZoom: number,
): { readonly x: number; readonly y: number; readonly zoom: number } => {
  const zoom = camera.zoom > 0 ? camera.zoom : 1;
  if (!Number.isFinite(nextZoom) || nextZoom <= 0) return { x: camera.x, y: camera.y, zoom };
  const worldX = camera.x + (anchor.x - viewport.w / 2) / zoom;
  const worldY = camera.y + (anchor.y - viewport.h / 2) / zoom;
  return {
    x: worldX - (anchor.x - viewport.w / 2) / nextZoom,
    y: worldY - (anchor.y - viewport.h / 2) / nextZoom,
    zoom: nextZoom,
  };
};

/** 📐️ Applies one whole {@link PinchStep} to a 2D camera with the canonical centred transform
 * `screen = (world - camera) * zoom + viewport / 2`: the world point that was under the PREVIOUS centroid
 * (`centroid - pan`) lands under the NEW centroid at the new zoom — the fingers hold the content, so zoom
 * and pan are one exact move, not a zoom followed by an approximate pan. */
export const applyPinchToCamera = (
  camera: { readonly x: number; readonly y: number; readonly zoom: number },
  step: PinchStep,
  viewport: { readonly w: number; readonly h: number },
  bounds: ZoomBounds,
): { readonly x: number; readonly y: number; readonly zoom: number } => {
  const zoom = camera.zoom > 0 ? camera.zoom : 1;
  const nextZoom = clampZoom(zoom * step.scale, bounds);
  const worldX = camera.x + (step.centroidX - step.panX - viewport.w / 2) / zoom;
  const worldY = camera.y + (step.centroidY - step.panY - viewport.h / 2) / zoom;
  return { x: worldX - (step.centroidX - viewport.w / 2) / nextZoom, y: worldY - (step.centroidY - viewport.h / 2) / nextZoom, zoom: nextZoom };
};

/** 📐️ {@link applyPinchToCamera} for a surface whose camera is a SCREEN offset,
 * `screen = world * zoom + camera` (the ink canvas): the same finger-anchored law in that transform. */
export const applyPinchToOffsetCamera = (
  camera: { readonly x: number; readonly y: number; readonly zoom: number },
  step: PinchStep,
  bounds: ZoomBounds,
): { readonly x: number; readonly y: number; readonly zoom: number } => {
  const zoom = camera.zoom > 0 ? camera.zoom : 1;
  const nextZoom = clampZoom(zoom * step.scale, bounds);
  const worldX = (step.centroidX - step.panX - camera.x) / zoom;
  const worldY = (step.centroidY - step.panY - camera.y) / zoom;
  return { x: step.centroidX - worldX * nextZoom, y: step.centroidY - worldY * nextZoom, zoom: nextZoom };
};

/** 🔍️ The multiplicative zoom of ONE wheel notch on a surface whose engine zooms in fixed steps
 * (`in` > 1 per notch toward the viewer, `out` < 1 per notch away). */
export type ZoomNotchFactors = {
  readonly in: number;
  readonly out: number;
};

/** 🔍️ Log-space slack for a notch boundary: `ln(0.81)` is two `ln(0.9)` notches, but floating point puts it a
 * hair above the second boundary, which would silently drop a whole notch of an exact pinch. */
export const PINCH_NOTCH_LOG_EPSILON = 1e-12;

/**
 * 🔍️ Converts a pinch `scale` into whole wheel notches for a step-quantized zoom engine, carrying the
 * sub-notch remainder in log space so a slow pinch still zooms (the remainder accumulates) and a fast
 * one never overshoots. Positive `notches` zoom in, negative zoom out; `pendingLogScale` is the
 * remainder the host keeps for the next step and resets when the gesture ends.
 */
export const pinchZoomNotches = (pendingLogScale: number, scale: number, factors: ZoomNotchFactors): { readonly notches: number; readonly pendingLogScale: number } => {
  const stepIn = Math.log(factors.in);
  const stepOut = Math.log(factors.out);
  let remaining = pendingLogScale + (Number.isFinite(scale) && scale > 0 ? Math.log(scale) : 0);
  let notches = 0;
  if (!(stepIn > 0) || !(stepOut < 0)) return { notches, pendingLogScale: remaining };
  while (remaining >= stepIn - PINCH_NOTCH_LOG_EPSILON) {
    remaining -= stepIn;
    notches += 1;
  }
  while (remaining <= stepOut + PINCH_NOTCH_LOG_EPSILON) {
    remaining -= stepOut;
    notches -= 1;
  }
  return { notches, pendingLogScale: remaining };
};
// #endregion 🔖️Zoom

// #region 🔖️Orbit
/** 🛰️ A 3D vector as a plain tuple, so the orbit law stays renderer-neutral (no Three types). */
export type OrbitVector = readonly [number, number, number];

/** 🛰️ An orbit camera pose: eye, look-at target, world up, and the orthographic zoom factor (`1` for a
 * perspective camera, which zooms by distance instead). */
export type OrbitPose = {
  readonly position: OrbitVector;
  readonly target: OrbitVector;
  readonly up: OrbitVector;
  readonly zoom: number;
};

/** 🛰️ What one surface pixel is worth at the target: a perspective camera needs its vertical field of
 * view, an orthographic one its unzoomed frustum height (`top - bottom`). */
export type OrbitProjection = { readonly kind: "perspective"; readonly fovYRadians: number } | { readonly kind: "orthographic"; readonly frustumHeight: number };

const orbitSub = (a: OrbitVector, b: OrbitVector): OrbitVector => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
const orbitAdd = (a: OrbitVector, b: OrbitVector): OrbitVector => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
const orbitScale = (a: OrbitVector, k: number): OrbitVector => [a[0] * k, a[1] * k, a[2] * k];
const orbitCross = (a: OrbitVector, b: OrbitVector): OrbitVector => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
const orbitLength = (a: OrbitVector): number => Math.hypot(a[0], a[1], a[2]);

/**
 * 🛰️ Applies one {@link PinchStep} to an orbit camera: spreading the fingers dollies toward the target
 * (perspective) or raises the zoom factor (orthographic), and the centroid's travel pans target and eye
 * together in the camera plane so the scene under the fingers follows them. The dolly is target-centred,
 * exactly like the two-finger `DOLLY_PAN` it replaces, so a touch user sees the same camera as before.
 * A pose whose eye sits on its target, or whose forward is parallel to `up`, is returned unchanged.
 */
export const applyPinchToOrbit = (pose: OrbitPose, step: PinchStep, projection: OrbitProjection, viewportHeight: number, bounds: { readonly distance: ZoomBounds; readonly zoom: ZoomBounds }): OrbitPose => {
  const offset = orbitSub(pose.position, pose.target);
  const distance = orbitLength(offset);
  if (!(distance > 0) || !(viewportHeight > 0)) return pose;
  const forward = orbitScale(offset, -1 / distance);
  const rightRaw = orbitCross(forward, pose.up);
  const rightLength = orbitLength(rightRaw);
  if (!(rightLength > 0)) return pose;
  const right = orbitScale(rightRaw, 1 / rightLength);
  const cameraUp = orbitCross(right, forward);
  const perspective = projection.kind === "perspective";
  const nextDistance = perspective ? clampZoom(distance / (step.scale > 0 ? step.scale : 1), bounds.distance) : distance;
  const nextZoom = perspective ? pose.zoom : clampZoom(pose.zoom * step.scale, bounds.zoom);
  const worldPerPixel = perspective ? (2 * nextDistance * Math.tan(projection.fovYRadians / 2)) / viewportHeight : projection.frustumHeight / (nextZoom * viewportHeight);
  const shift = orbitAdd(orbitScale(right, -step.panX * worldPerPixel), orbitScale(cameraUp, step.panY * worldPerPixel));
  const target = orbitAdd(pose.target, shift);
  return { position: orbitSub(target, orbitScale(forward, nextDistance)), target, up: pose.up, zoom: nextZoom };
};
// #endregion 🔖️Orbit
// #endregion 👆️Gesture

//#region 🧪️GestureTests
if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🔬️unit/🟦️.ts");
  await registerTests1(
    import.meta.vitest,
    {
      EMPTY_GESTURE_POINTERS,
      GESTURE_MULTI_TOUCH_POINTERS,
      IDENTITY_PINCH_STEP,
      PINCH_WHEEL_PIXELS_PER_DOUBLING,
      applyPinchToCamera,
      applyPinchToOffsetCamera,
      GestureRecognizer,
      IDLE_GESTURE_RECOGNIZER,
      applyPinchToOrbit,
      clampZoom,
      gestureIsMultiTouch,
      gesturePointerDown,
      gesturePointerMove,
      gesturePointerUp,
      pinchFrame,
      pinchStep,
      pinchWheelDelta,
      pinchZoomNotches,
      recognizeGesturePointerDown,
      recognizeGesturePointerMove,
      recognizeGesturePointerUp,
      shortestAngleDelta,
      zoomAboutPoint,
    },
    { directory: (await import("node:path")).dirname((await import("node:url")).fileURLToPath(import.meta.url)), url: import.meta.url },
  );
}
//#endregion 🧪️GestureTests
