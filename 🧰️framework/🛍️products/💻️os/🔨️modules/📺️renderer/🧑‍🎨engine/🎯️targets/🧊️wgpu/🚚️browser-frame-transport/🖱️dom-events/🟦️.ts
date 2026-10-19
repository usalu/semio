import type { BrowserFrameLosslessEvent, BrowserFrameModifiers, BrowserFramePointer, BrowserFrameReplaceableEvent } from "../📋️contract/🟦️.ts";

/** 🖱️ One DOM input as the UI isolate observes it on `#semio-wgpu-canvas`, reduced to the fields
 * the wire carries. Deliberately NOT `PointerEvent`/`WheelEvent`/`KeyboardEvent`: this shape is what the
 * language-neutral oracle `🧫️fixtures/🎮️wgpu-browser-input-wire/🔣️.json` names, so the projection below
 * can be exercised without a DOM and the Rust law can read the same rows. */
type BrowserFramePointerDomFields = BrowserFrameModifiers & { readonly pointerId: number; readonly pointerType: string; readonly offsetX: number; readonly offsetY: number; readonly pressure?: number; readonly tiltX?: number; readonly tiltY?: number; readonly button?: number };

type BrowserFrameKeyDomFields = { readonly key: string; readonly shift: boolean; readonly ctrl: boolean; readonly alt: boolean; readonly meta: boolean };

export type BrowserFrameDomEvent =
  | ({ readonly type: "pointercancel" } & BrowserFramePointerDomFields)
  | ({ readonly type: "pointermove" } & BrowserFramePointerDomFields)
  | ({ readonly type: "pointerdown" } & BrowserFramePointerDomFields)
  | ({ readonly type: "pointerup" } & BrowserFramePointerDomFields)
  | ({ readonly type: "wheel"; readonly offsetX: number; readonly offsetY: number; readonly deltaX: number; readonly deltaY: number } & BrowserFrameModifiers)
  | ({ readonly type: "keydown" } & BrowserFrameKeyDomFields)
  | ({ readonly type: "keyup" } & BrowserFrameKeyDomFields)
  | { readonly type: "resize"; readonly clientWidth: number; readonly clientHeight: number };

/** 📏️ The ONE place a CSS pixel becomes a physical pixel — and it is used for the SURFACE
 * EXTENT ONLY. The GPU surface is configured in device pixels; the renderer's layout, hit registry,
 * dock plan and every chrome constant are CSS/logical pixels, exactly like the React host's DOM, so
 * pointer coordinates cross this wire unscaled. Scroll deltas are device-independent either way.
 * Ticket 26/09/17/WGPU-RENDERER-REACT-PARITY packet W1g. */
function physical(css: number, devicePixelRatio: number): number {
  return css * devicePixelRatio;
}

/** 🖱️ Names a DOM `button` integer. The wire carries a name so neither side has to agree on
 * the DOM's numbering, and an unknown button is the primary one rather than a dropped event. */
function pointerButtonName(button: number | undefined): "primary" | "secondary" | "middle" {
  return button === 2 ? "secondary" : button === 1 ? "middle" : "primary";
}

/** 🖱️ `offsetX`/`offsetY` are CSS pixels relative to the canvas and stay that way: the
 * renderer hit-tests in logical pixels. */
function pointerFields(event: Extract<BrowserFrameDomEvent, { type: "pointermove" | "pointerdown" | "pointerup" | "pointercancel" }>): BrowserFramePointer {
  return {
    pointerId: event.pointerId,
    pointerKind: event.pointerType === "touch" || event.pointerType === "pen" || event.pointerType === "eraser" ? event.pointerType : "mouse",
    x: event.offsetX,
    y: event.offsetY,
    ...(event.pressure ? { pressure: event.pressure } : {}),
    ...(event.tiltX ? { tiltX: event.tiltX } : {}),
    ...(event.tiltY ? { tiltY: event.tiltY } : {}),
    shift: event.shift,
    ctrl: event.ctrl,
    alt: event.alt,
    meta: event.meta,
  };
}

/** ⌨️ Reads the event-time modifier snapshot carried by every DOM pointer and wheel event. */
export function browserFrameModifiersFromDom(event: Pick<MouseEvent, "shiftKey" | "ctrlKey" | "altKey" | "metaKey">): BrowserFrameModifiers {
  return { shift: event.shiftKey, ctrl: event.ctrlKey, alt: event.altKey, meta: event.metaKey };
}

/** 🖱️ Reduces one real DOM pointer event into the owned input vocabulary. */
export function browserFramePointerDomEvent(event: PointerEvent, type: "pointermove" | "pointerdown" | "pointerup" | "pointercancel"): BrowserFrameDomEvent {
  return {
    type,
    pointerId: event.pointerId,
    pointerType: event.pointerType,
    offsetX: event.offsetX,
    offsetY: event.offsetY,
    pressure: event.pressure,
    tiltX: event.tiltX,
    tiltY: event.tiltY,
    button: event.button,
    ...browserFrameModifiersFromDom(event),
  };
}

/** 🎡️ Reduces one real DOM wheel event into the owned input vocabulary. */
export function browserFrameWheelDomEvent(event: WheelEvent): BrowserFrameDomEvent {
  return { type: "wheel", offsetX: event.offsetX, offsetY: event.offsetY, deltaX: event.deltaX, deltaY: event.deltaY, ...browserFrameModifiersFromDom(event) };
}

/** 🎮️ Projects one observed DOM input onto the single wire event the frame Worker decodes.
 *
 * This is the UI isolate's whole share of input semantics — the listeners in `🚀️browser-boot/🟦️.ts`
 * add nothing but focus/capture and the lossless-versus-replaceable lane choice. Keeping the projection
 * HERE, beside the wire types it produces and inside the module the transport's own vitest suite already
 * imports, is what lets `🧫️fixtures/🎮️wgpu-browser-input-wire/🔣️.json` be answered without a browser and
 * by the Rust law on the other side of the same fixture.
 *
 * `devicePixelRatio` reaches exactly one event kind: `resize`, whose width/height ARE the physical
 * surface extent. Pointer and wheel positions stay CSS pixels. */
export function browserFrameEventFromDom(event: BrowserFrameDomEvent, devicePixelRatio: number): BrowserFrameReplaceableEvent | BrowserFrameLosslessEvent {
  if (event.type === "pointercancel") return { kind: "pointer-cancel", ...pointerFields(event) };
  if (event.type === "pointermove") return { kind: "pointer-move", ...pointerFields(event) };
  if (event.type === "pointerdown" || event.type === "pointerup") return { kind: event.type === "pointerdown" ? "pointer-down" : "pointer-up", ...pointerFields(event), button: pointerButtonName(event.button) };
  if (event.type === "wheel") return { kind: "wheel", x: event.offsetX, y: event.offsetY, deltaX: event.deltaX, deltaY: event.deltaY, shift: event.shift, ctrl: event.ctrl, alt: event.alt, meta: event.meta };
  if (event.type === "resize") return { kind: "resize", width: Math.max(1, Math.round(physical(event.clientWidth, devicePixelRatio))), height: Math.max(1, Math.round(physical(event.clientHeight, devicePixelRatio))), dpr: devicePixelRatio };
  return { kind: event.type === "keydown" ? "key-down" : "key-up", key: event.key, shift: event.shift, ctrl: event.ctrl, alt: event.alt, meta: event.meta };
}

/** 🫧 Whether a projected wire event belongs to the coalescing lane (`enqueueReplaceable`) rather
 * than the credit-bearing lossless lane. A pointer MOVE, a wheel and a resize are each "the newest one
 * wins"; a button transition and a key transition are not, because dropping one latches shell state. */
export function browserFrameEventIsReplaceable(event: BrowserFrameReplaceableEvent | BrowserFrameLosslessEvent): event is BrowserFrameReplaceableEvent {
  return event.kind === "pointer-move" || event.kind === "wheel" || event.kind === "resize";
}
