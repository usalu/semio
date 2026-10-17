/** 👆️ Canvas2dHost pointer lane: the pointer sample and extras, the dispatch contract and the gesture sample lane that coalesces canvas pointer moves. */
import { type CanvasInputModifiers } from "@semio-tech/framework/canvas/react-renderer";
import { createGestureSampleLaneV1, type GestureSampleLaneV1 } from "../../🏛️ShellHost/🎯️input-ledger/🟦️.ts";

//#region 🔖️Canvas2dHost
//#region JsonLayersCanvasSession
//#region CanvasPointerGestureLane
/** 🖱️ One pointer sample: its logical CSS-pixel position, the keys held, and the WORLD position it maps to under the
 * camera at the moment it was sampled — a later camera move never re-maps a sample already taken. */
export type CanvasPointerSample = readonly [number, number, CanvasInputModifiers, number, number];

/** ⌨️ Copies keyboard state and the sample's world position (`world`) with the pointer position before dispatch is queued. */
export function pointerSample(x: number,y: number,modifiers: CanvasInputModifiers|undefined,world: { readonly x: number; readonly y: number }): CanvasPointerSample {
  return [x,y,{shift:modifiers?.shift??false,ctrl:modifiers?.ctrl??false,meta:modifiers?.meta??false,alt:modifiers?.alt??false},world.x,world.y];
}

/** ⌨️ What a discrete gesture phase (`begin`/`end`) carries besides its sample: the button that pressed,
 * the modifiers held, and the viewport the sample is measured in — every field the pre-lane
 * `canvasPointerDown`/`canvasPointerUp` commands already carried. */
export type CanvasPointerExtra = {
  readonly button?: number;
  readonly shift: boolean;
  readonly ctrl: boolean;
  readonly meta: boolean;
  readonly alt: boolean;
  readonly width: number;
  readonly height: number;
};

/** 📤️ The host's action dispatcher as the lane needs it: RETURNS `onAction`'s promise, because the lane's
 * "at most one send in flight" gate is exactly that promise — a `void` dispatcher clears the gate on the
 * next microtask and gates nothing (the World3dHost wave-B33 lesson, same shape as `dispatchSettled`). */
export type CanvasPointerDispatch = (action: string, args?: Record<string, unknown>) => void | Promise<unknown>;

export type CanvasPointerGestureLane = GestureSampleLaneV1<CanvasPointerSample, CanvasPointerExtra>;

/** 🖱️ The `Canvas2dHost` gesture lane (design §2 D): every DOM pointer event used to be ONE guest round
 * trip (`canvasPointerDown` / `canvasPointerMove` / `canvasPointerUp`), so a one-second marquee at 60 Hz
 * queued ~60 commands and drained for 1–3 s after mouseup. Through `createGestureSampleLaneV1` the same
 * three commands are still the only wire (additive fields only), but:
 *   - every `pointermove` offered while a send is out is APPENDED to one owed `canvasPointerMove` whose
 *     `samples` (`[x, y][]`, oldest first) carry the whole batch and whose `x`/`y` stay the LAST sample,
 *     so a guest that ignores `samples` keeps today's semantics with fewer calls;
 *   - `begin` / `end` are discrete and ordered behind every owed sample — an `end` never overtakes a move;
 *   - a cancel (pointer left the canvas, capture lost) is `canvasPointerUp { cancelled: true }`, never a
 *     forged release;
 *   - every command carries `worldX`/`worldY` (its sample's world position) and a move its `worldSamples`
 *     (`[x, y][]` beside `samples`), exactly as the wgpu host sends them, so a guest maps a sample to its own
 *     content without knowing the host's camera.
 * `width`/`height` are read at SEND time (`size()`), as the pre-lane commands read `logicalWidth/Height`. */
export function createCanvasPointerGestureLane(dispatch: CanvasPointerDispatch, size: () => { readonly width: number; readonly height: number }): CanvasPointerGestureLane {
  return createGestureSampleLaneV1<CanvasPointerSample, CanvasPointerExtra>({
    send(item) {
      switch (item.phase) {
        case "begin": {
          const { sample, extra } = item;
          return dispatch("canvasPointerDown", { x: sample[0], y: sample[1], worldX: sample[3], worldY: sample[4], button: extra.button, shift: extra.shift, ctrl: extra.ctrl, meta: extra.meta, alt: extra.alt, width: extra.width, height: extra.height });
        }
        case "live": {
          const last = item.samples[item.samples.length - 1];
          if (last === undefined) return undefined;
          const viewport = size();
          return dispatch("canvasPointerMove", { x: last[0], y: last[1], worldX: last[3], worldY: last[4], ...last[2], width: viewport.width, height: viewport.height, samples: item.samples.map(([x, y]) => [x, y]), worldSamples: item.samples.map((sample) => [sample[3], sample[4]]) });
        }
        case "end": {
          const { sample, extra } = item;
          return dispatch("canvasPointerUp", { x: sample[0], y: sample[1], worldX: sample[3], worldY: sample[4], shift: extra.shift, ctrl: extra.ctrl, meta: extra.meta, alt: extra.alt, width: extra.width, height: extra.height, cancelled: false });
        }
        case "cancel": {
          const viewport = size();
          return dispatch("canvasPointerUp", { x: item.sample?.[0] ?? 0, y: item.sample?.[1] ?? 0, worldX: item.sample?.[3] ?? 0, worldY: item.sample?.[4] ?? 0, shift: false, ctrl: false, meta: false, alt: false, width: viewport.width, height: viewport.height, cancelled: true });
        }
      }
    },
    onFault: (error) => console.warn(`canvas2d gesture lane: ${error instanceof Error ? error.message : String(error)}`),
  });
}
//#endregion CanvasPointerGestureLane
//#endregion JsonLayersCanvasSession
//#endregion 🔖️Canvas2dHost
