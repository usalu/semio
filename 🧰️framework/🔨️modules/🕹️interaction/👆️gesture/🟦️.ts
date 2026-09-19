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

/** 📐️ Applies one whole {@link PinchStep} to a 2D camera: zoom about the centroid first, then translate
 * by the centroid's own travel converted to world units. The order matters — translating first would
 * pan by a distance measured at the OLD zoom. */
export const applyPinchToCamera = (
  camera: { readonly x: number; readonly y: number; readonly zoom: number },
  step: PinchStep,
  viewport: { readonly w: number; readonly h: number },
  bounds: ZoomBounds,
): { readonly x: number; readonly y: number; readonly zoom: number } => {
  const zoomed = zoomAboutPoint(camera, { x: step.centroidX, y: step.centroidY }, viewport, clampZoom(camera.zoom * step.scale, bounds));
  return { x: zoomed.x - step.panX / zoomed.zoom, y: zoomed.y - step.panY / zoomed.zoom, zoom: zoomed.zoom };
};
// #endregion 🔖️Zoom
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
      clampZoom,
      gestureIsMultiTouch,
      gesturePointerDown,
      gesturePointerMove,
      gesturePointerUp,
      pinchFrame,
      pinchStep,
      pinchWheelDelta,
      shortestAngleDelta,
      zoomAboutPoint,
    },
    { directory: (await import("node:path")).dirname((await import("node:url")).fileURLToPath(import.meta.url)), url: import.meta.url },
  );
}
//#endregion 🧪️GestureTests
