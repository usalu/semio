/** 🤏️ The learner's hand on the pets: turns presses on a pet, the drag that follows, its release or its cancellation into stage events, shows the hand cursor, and tells the stage when the learner does anything at all or the page scrolls.
 *
 * The pet layer never takes the pointer (design-v2 §14.1): every listener sits on the window or the document, in the
 * capture phase, ahead of everything the page listens to. A primary press counts for the pets only when the host lets
 * them be played with, nothing interactive lies under the pointer (a control always wins), no text is selected, no
 * modifier key asks for something else, and the point lies in a visible pet's solid body of the last frame — the host
 * answers the first and the last with `takes`. That press is the only event the hand keeps from the page: it is
 * cancelled (no text selection starts, no focus moves, no compatible `mousedown` follows) and stopped (nothing beneath
 * it acts, no listener of the host hears it), the pointer is captured on the document element so that the drag and the
 * release arrive wherever the pointer goes, and the click the press leads to is swallowed. Every other event passes
 * untouched.
 *
 * Positions are pixels of the layer (viewport pixels minus `origin`), like those of the survey's pointer feed.
 *
 * @see ../📡️survey/🟦️.ts — the pointer feed (`pointed` with what lies under the pointer)
 * @see ../🫧️layer/🟦️.tsx — the only caller in production
 * @see https://www.w3.org/TR/pointerevents3/#the-pointerdown-event — what cancelling a `pointerdown` prevents
 * @see https://www.w3.org/WAI/WCAG22/Understanding/pointer-cancellation.html — a press only arms; Escape and a cancelled pointer abort
 * @see https://developer.mozilla.org/en-US/docs/Web/API/Element/setPointerCapture
 */

//#region 🔌️Adapters
import type { Cancelled, Dragged, Point, Pointer, Pressed, Released, Scrolled, Stirred } from "@semio-tech/pets";
import { PET_PRESS_CONTROLS } from "../📡️survey/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🔖️Constants
/** ⏱️ The shortest time in milliseconds between two `stirred` events: the stage learns that the learner is busy, not every key they type. */
export const STIR_MILLISECONDS = 1000;

/** 🖐️ The attribute of the document element that says which hand shows: `grab` while the pointer rests on a pet that can be picked up, `grabbing` while one is held. A watcher of class changes, such as the survey, does not hear it; no rule of the stylesheet names it.
 *
 * `grab` comes and goes whenever the pointer passes a pet, so the hand shows it on the one element under the pointer
 * (an inline `cursor: grab !important`, given back exactly as it was once the pointer leaves that element or the pet):
 * a browser restyles every element a rule could reach whenever an attribute that rule names changes, whatever its
 * value, so a rule that let every element follow this mark would restyle the whole page with every pass. */
export const PET_CURSOR = "data-pet-cursor";

/** ✊️ The attribute the document element carries while the learner holds a pet: the stylesheet's mark for the grabbing hand on every element, since the pointer crosses anything while it drags. It changes only when a pet is picked up and let go. */
export const PET_HELD = "data-pet-held";

const ESCAPE = "Escape";
//#endregion 🔖️Constants

//#region 🔖️Events
/** 📨️ What the hand tells the stage, in pixels of the layer. */
export type Grasped = Pressed | Dragged | Released | Cancelled | Stirred | Scrolled;
//#endregion 🔖️Events

//#region 🔖️Hand
/** 🪟️ What the hand needs of a window; the layer passes `window`, a test may pass a fake. */
export type GraspHost = Pick<Window, "document" | "addEventListener" | "removeEventListener"> & { readonly getSelection?: () => Selection | null };

/** 🎚️ How the hand reads the page: the origin of the stage in viewport pixels (the viewport's corner when absent); the selector of what a press never takes away from, read at every press ({@link PET_PRESS_CONTROLS} when absent); whether a press at a position of the layer may be taken — the host lets the pets be played with and a visible pet's body lies there (nothing is ever taken when absent); and the clock in milliseconds that spaces `stirred` (`Date.now` when absent). */
export interface GraspOptions {
  readonly origin?: () => Point;
  readonly controls?: () => string;
  readonly takes?: (x: number, y: number) => boolean;
  readonly now?: () => number;
}

/** ✋️ A running hand: `frame` tells it whether the frame just painted holds a pet — it shows `grabbing` then, ends its press quietly when the stage let go of the pet by itself, and judges the cursor anew for a pointer that rests while pets move; `cancel` aborts a press the host no longer allows (play switched off, a still stage, a hidden document, an inert layer); `stop` releases everything and leaves nothing behind. */
export interface Grasp {
  readonly frame: (held: boolean) => void;
  readonly cancel: () => void;
  readonly stop: () => void;
}

type Press = { readonly id: number; readonly pointer: Pointer };
type Rest = { readonly x: number; readonly y: number; readonly target: EventTarget | null };
type Styled = Element & ElementCSSInlineStyle;
type Marked = { readonly element: Styled; readonly value: string; readonly priority: string; readonly attributed: boolean };

/** 🎨️ Whether an event target is an element whose inline style can carry a cursor. */
function styled(target: EventTarget | null): target is Styled {
  return typeof (target as Partial<ElementCSSInlineStyle> | null)?.style?.setProperty === "function" && typeof (target as Partial<Element>).getAttribute === "function";
}

/** 🧭️ The kind of pointer an event comes from; a pointer without a kind (a synthetic event) counts as a mouse. */
function pointerOf(event: PointerEvent): Pointer {
  return event.pointerType === "pen" || event.pointerType === "touch" ? event.pointerType : "mouse";
}

/** ✍️ Whether the document holds a selection of text, which a press would extend, change or clear. */
function selecting(view: GraspHost): boolean {
  const selection = view.getSelection?.() ?? null;
  return selection !== null && selection.rangeCount > 0 && !selection.isCollapsed;
}

/** 📌️ Captures the pointer `id` on `root` or releases it; a pointer the browser does not know (a synthetic press) is passed over, and so is a browser without capture. */
function capture(root: Element, id: number, on: boolean): void {
  if (typeof root.setPointerCapture !== "function") return;
  try {
    if (on) root.setPointerCapture(id);
    else if (root.hasPointerCapture(id)) root.releasePointerCapture(id);
  } catch {
    return;
  }
}

/** 🫴️ Lends the pets the learner's hand and returns it; see the module for the rules.
 *
 * A taken press becomes `pressed`, every move of that pointer `dragged` (the layer keeps the latest per frame), its
 * release `released` and its loss `cancelled`: Escape (which is kept from the page then), `pointercancel` (the browser
 * took the pointer over, a pan for instance), `lostpointercapture`, a window that loses focus, and the host's
 * {@link Grasp.cancel}. Any `pointerdown`, `keydown`, `wheel` and `input` anywhere becomes `stirred`, at most once per
 * {@link STIR_MILLISECONDS}; a finger is a pointer, so touches count. Any scroll of the page or of a pane becomes
 * `scrolled`. While a press is taken, the start of a text selection, a native drag and a context menu are cancelled.
 * The click a taken press leads to is swallowed, also after the press was called off while its button was still down;
 * a key typed once that button is up, a cancelled pointer, a window that loses focus or the next press forget it, so
 * a click from the keyboard always passes.
 */
export function watchGrasp(view: GraspHost, emit: (event: Grasped) => void, options: GraspOptions = {}): Grasp {
  const root = view.document.documentElement;
  const now = options.now ?? (() => Date.now());
  let press: Press | null = null;
  let pinned: number | null = null;
  let holding = false;
  let swallow = false;
  let rest: Rest | null = null;
  let stirredAt = -Infinity;
  let shown: string | null = null;
  let marked: Marked | null = null;
  let ended = false;

  const at = (event: MouseEvent): Point => {
    const origin = options.origin?.();
    return { x: event.clientX - (origin?.x ?? 0), y: event.clientY - (origin?.y ?? 0) };
  };
  const control = (target: EventTarget | null): boolean => {
    const element = target as Partial<Element> | null;
    return typeof element?.closest === "function" && element.closest(options.controls?.() ?? PET_PRESS_CONTROLS) !== null;
  };
  const mark = (target: EventTarget | null): void => {
    const element = styled(target) ? target : null;
    if (marked?.element === element) return;
    if (marked !== null) {
      const { element: before, value, priority, attributed } = marked;
      if (value === "") before.style.removeProperty("cursor");
      else before.style.setProperty("cursor", value, priority);
      if (!attributed && before.getAttribute("style") === "") before.removeAttribute("style");
      marked = null;
    }
    if (element === null) return;
    marked = { element, value: element.style.getPropertyValue("cursor"), priority: element.style.getPropertyPriority("cursor"), attributed: element.hasAttribute("style") };
    element.style.setProperty("cursor", "grab", "important");
  };
  const show = (): void => {
    const cursor = press !== null && holding ? "grabbing" : rest !== null && options.takes?.(rest.x, rest.y) === true && !control(rest.target) ? "grab" : null;
    mark(cursor === "grab" ? (rest?.target ?? null) : null);
    if (cursor === shown) return;
    if (cursor === "grabbing") root.setAttribute(PET_HELD, "");
    else if (shown === "grabbing") root.removeAttribute(PET_HELD);
    shown = cursor;
    if (cursor === null) root.removeAttribute(PET_CURSOR);
    else root.setAttribute(PET_CURSOR, cursor);
  };
  const stir = (): void => {
    const time = now();
    if (time - stirredAt < STIR_MILLISECONDS) return;
    stirredAt = time;
    emit({ kind: "stirred" });
  };
  const end = (): void => {
    if (press !== null) capture(root, press.id, false);
    press = null;
    holding = false;
    show();
  };
  const abort = (): void => {
    if (press === null) return;
    end();
    emit({ kind: "cancelled" });
  };

  const down = (event: PointerEvent): void => {
    swallow = false;
    stir();
    if (event.isPrimary !== false) pinned = null;
    if (press !== null && event.pointerId !== press.id && event.isPrimary !== false) abort();
    if (press !== null || event.button !== 0 || event.isPrimary === false || event.defaultPrevented || event.shiftKey || event.ctrlKey || event.metaKey || event.altKey) return;
    const point = at(event);
    if (control(event.target) || selecting(view) || options.takes?.(point.x, point.y) !== true) return;
    event.preventDefault();
    event.stopPropagation();
    press = { id: event.pointerId, pointer: pointerOf(event) };
    pinned = event.pointerId;
    swallow = true;
    rest = press.pointer === "touch" ? null : { ...point, target: event.target };
    capture(root, event.pointerId, true);
    emit({ kind: "pressed", ...point, pointer: press.pointer });
    show();
  };
  const move = (event: PointerEvent): void => {
    if (press !== null && event.pointerId === press.id) {
      emit({ kind: "dragged", ...at(event) });
      return;
    }
    rest = event.pointerType === "touch" || event.buttons !== 0 ? null : { ...at(event), target: event.target };
    show();
  };
  const up = (event: PointerEvent): void => {
    if (event.pointerId === pinned) pinned = null;
    if (press === null || event.pointerId !== press.id) return;
    const point = at(event);
    const pointer = press.pointer;
    end();
    emit({ kind: "released", ...point });
    rest = pointer === "touch" ? null : { ...point, target: event.target };
    show();
  };
  const lost = (event: PointerEvent): void => {
    if (press !== null && event.pointerId === press.id) abort();
  };
  const dropped = (event: PointerEvent): void => {
    if (event.pointerId === pinned) {
      pinned = null;
      swallow = false;
    }
    lost(event);
  };
  const typed = (event: KeyboardEvent): void => {
    if (pinned === null) swallow = false;
    stir();
    if (press === null || event.key !== ESCAPE) return;
    event.preventDefault();
    event.stopPropagation();
    abort();
  };
  const blurred = (): void => {
    pinned = null;
    swallow = false;
    abort();
  };
  const clicked = (event: MouseEvent): void => {
    if (!swallow) return;
    swallow = false;
    event.preventDefault();
    event.stopPropagation();
  };
  const refused = (event: Event): void => {
    if (press !== null) event.preventDefault();
  };
  const scrolled = (): void => emit({ kind: "scrolled" });

  const active = { capture: true, passive: false } as const;
  const passive = { capture: true, passive: true } as const;
  const listening: readonly (readonly [Pick<EventTarget, "addEventListener" | "removeEventListener">, string, EventListener, AddEventListenerOptions])[] = [
    [view, "pointerdown", down as EventListener, active],
    [view, "pointermove", move as EventListener, passive],
    [view, "pointerup", up as EventListener, passive],
    [view, "pointercancel", dropped as EventListener, passive],
    [view, "lostpointercapture", lost as EventListener, passive],
    [view, "keydown", typed as EventListener, active],
    [view, "wheel", stir, passive],
    [view, "input", stir, passive],
    [view, "blur", blurred, { capture: false, passive: true }],
    [view, "click", clicked as EventListener, active],
    [view, "selectstart", refused, active],
    [view, "dragstart", refused, active],
    [view, "contextmenu", refused, active],
    [view.document, "scroll", scrolled, passive],
  ];
  for (const [target, type, listener, how] of listening) target.addEventListener(type, listener, how);

  return {
    frame: (held) => {
      if (ended) return;
      if (press !== null && holding && !held) end();
      else {
        holding = press !== null && held;
        show();
      }
    },
    cancel: () => {
      if (!ended) abort();
    },
    stop: () => {
      if (ended) return;
      ended = true;
      for (const [target, type, listener, how] of listening) target.removeEventListener(type, listener, how.capture);
      rest = null;
      pinned = null;
      swallow = false;
      end();
    },
  };
}
//#endregion 🔖️Hand
