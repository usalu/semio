/** 🧲️ World3dHost handle drag: the drag parameter along a handle axis, the pick of a handle marker and the gesture state machine whose output is one `delta`, `commit` or `abort` binding input.
 *
 * @see ../🧪️tests/🧫️fixtures/🧲️handle-drag.json — the language-neutral vectors and event sequences every renderer reproduces.
 */

export type WorldVec3 = readonly [number, number, number];
export type WorldHandleModifiers = { readonly shift: boolean; readonly alt: boolean; readonly ctrl: boolean };
export type WorldHandleTrigger = "delta" | "commit" | "abort";
export type WorldHandleDragInput = { readonly handle: string; readonly s: number; readonly origin: number; readonly modifiers: WorldHandleModifiers; readonly at: WorldVec3 };
export type WorldHandleDragState = { readonly handle: string; readonly origin: number; readonly s: number; readonly moved: boolean } | null;
export type WorldHandleDragEvent =
  | { readonly kind: "press"; readonly handle: string; readonly s: number }
  | { readonly kind: "move" | "release"; readonly s: number; readonly modifiers: WorldHandleModifiers; readonly at: WorldVec3 }
  | { readonly kind: "cancel"; readonly modifiers: WorldHandleModifiers; readonly at: WorldVec3 };
export type WorldHandleDragStep = { readonly state: WorldHandleDragState; readonly dispatch: { readonly trigger: WorldHandleTrigger; readonly input: WorldHandleDragInput } | null };

const dot = (a: WorldVec3, b: WorldVec3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
const sub = (a: WorldVec3, b: WorldVec3): WorldVec3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
const unit = (a: WorldVec3): WorldVec3 | null => {
  const length = Math.sqrt(dot(a, a));
  return length > 0 ? [a[0] / length, a[1] / length, a[2] / length] : null;
};

/** 📐️ The signed distance `s` from `axisOrigin` along `axis` of the point of the axis line closest to the pointer ray; `null` when the ray is parallel to the axis or either direction is degenerate. */
export function world3dHandleDragParameterV1(rayOrigin: WorldVec3, rayDirection: WorldVec3, axisOrigin: WorldVec3, axis: WorldVec3): number | null {
  const u = unit(rayDirection);
  const d = unit(axis);
  if (!u || !d) return null;
  const w = sub(axisOrigin, rayOrigin);
  const b = dot(d, u);
  const denominator = 1 - b * b;
  if (denominator < 1e-9) return null;
  return (b * dot(u, w) - dot(d, w)) / denominator;
}

/** 🎯️ The id of the nearest visible handle marker within `radiusPx` of the pointer, ties to the first in scene order; `null` when none is under it. */
export function world3dHandlePickV1(handles: readonly { readonly id: string; readonly x: number; readonly y: number }[], x: number, y: number, radiusPx: number): string | null {
  let best: { id: string; distance: number } | null = null;
  for (const handle of handles) {
    const distance = Math.hypot(handle.x - x, handle.y - y);
    if (distance <= radiusPx && (!best || distance < best.distance)) best = { id: handle.id, distance };
  }
  return best ? best.id : null;
}

/** 🧾️ The gesture of one handle drag: a press arms it silently, a changed parameter streams `delta`, a release after movement commits once, a cancel after movement aborts, and a click without travel dispatches nothing. */
export function world3dHandleDragStepV1(state: WorldHandleDragState, event: WorldHandleDragEvent): WorldHandleDragStep {
  if (event.kind === "press") return { state: { handle: event.handle, origin: event.s, s: event.s, moved: false }, dispatch: null };
  if (!state) return { state: null, dispatch: null };
  const input = (s: number, modifiers: WorldHandleModifiers, at: WorldVec3): WorldHandleDragInput => ({ handle: state.handle, s, origin: state.origin, modifiers: { shift: modifiers.shift, alt: modifiers.alt, ctrl: modifiers.ctrl }, at: [at[0], at[1], at[2]] });
  if (event.kind === "move") {
    if (event.s === state.s) return { state, dispatch: null };
    return { state: { ...state, s: event.s, moved: true }, dispatch: { trigger: "delta", input: input(event.s, event.modifiers, event.at) } };
  }
  if (event.kind === "release") return { state: null, dispatch: state.moved ? { trigger: "commit", input: input(event.s, event.modifiers, event.at) } : null };
  return { state: null, dispatch: state.moved ? { trigger: "abort", input: input(state.s, event.modifiers, event.at) } : null };
}
