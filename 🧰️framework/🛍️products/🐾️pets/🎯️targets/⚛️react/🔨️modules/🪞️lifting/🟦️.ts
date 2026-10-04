/** 🪞️ Mischief without touching the page: {@link liftFixtures} realises the lifts of a frame — a pet pushes an element of the host out of its stack — by showing a copy of the element in the pet layer while the element itself stays where it is, only transparent, and gives it back the moment the learner reaches for it.
 *
 * The element (the original) keeps its place, its size, its content, its attributes, its focusability and its
 * listeners. Only two properties of its inline style change while it is lifted, and only through the CSSOM:
 * `opacity` (0 while the copy shows the whole lift; the author's own declaration is kept and put back) and
 * `transition-property` (`none`, so that neither the lift nor its undo is smoothed by a transition of the host — the
 * undo is instant, and a host that shortens every transition for reduced motion gets no transition to end).
 *
 * The copy ({@link copyOf}) is a deep clone without anything another script could find or reach it by: no `id`,
 * `name`, `for`, `form`, `data-*`, event handler attributes, inline `style` attributes, `autofocus`, tab stops, roles,
 * titles or links; it is `inert` and `aria-hidden`, takes no pointer events, runs no animation and wears the computed
 * look of every node of the original, copied property by property ({@link LIFT_LOOK}) with every box held to the size
 * the original has. It lives in the pet layer — fixed, clipped, beyond every scroll container — at the original's box
 * plus the lift's offset, turned by its tilt about its middle.
 *
 * Taking an element back: one listener set on the document, in the capture phase and passive, hears every
 * `pointerover`, `pointerdown`, `focusin`, `keydown`, `input`, `change`, `dragstart`, `selectstart` and `click`
 * ({@link RECLAIMS}); whatever lands on the original or inside it ({@link LIFT_REACH} ancestors up) restores it in the
 * same task — before the page's own handlers see the event — and tells the stage (`reclaimed`). While the stage is
 * quiet, any of them anywhere restores every lift. The document going hidden or away restores every lift too; the
 * layer adds a scene change, `still`, a box that moved under a survey and its own end. The listeners exist only while
 * something is lifted.
 *
 * Safe under a Content-Security-Policy without inline styles: `cloneNode` copies `style` attributes, so the copy loses
 * them and gets its look by CSSOM writes; no `innerHTML`, no `<style>`, no `setAttribute("style")`.
 *
 * @see ../📡️survey/🟦️.ts — which elements are fixtures ({@link copyable}, {@link fixtureId})
 * @see ../🫧️layer/🟦️.tsx — the caller
 * @see https://html.spec.whatwg.org/multipage/interaction.html#inert-subtrees — what `inert` takes away
 * @see https://drafts.csswg.org/css-transitions/#starting — why a transition cannot start while `transition-property` is `none`
 * @see https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Security-Policy/style-src-attr — what a policy blocks
 */

import type { LiftFrame, Point } from "@semio-tech/pets";
import { rounded } from "../🖌️depiction/🟦️.ts";
import { PET_PROPS, copyable, fixtureId } from "../📡️survey/🟦️.ts";

//#region 🔖️Rules
/** 🪜️ How many ancestors of an event's target are looked at for a lifted original: a press on a word deep inside a row still takes the row back. */
export const LIFT_REACH = 12;

/** 👂️ The events that take a lifted element back when they land on it or inside it: a pointer that comes over it or presses, focus, keys, a changed value (also one set by a script that only dispatches `input` and `change`), the start of a drag or of a selection, and an activation without a press. */
export const RECLAIMS = ["pointerover", "pointerdown", "focusin", "keydown", "input", "change", "dragstart", "selectstart", "click"] as const;

/** 🎨️ The computed properties a copy takes over from every node of its original, longhands only (a computed shorthand reads empty in some engines): box and flow, spacing, borders and their corners, backgrounds and shadows, colour and opacity, type and text, lists, overflow, replaced content and the paints of inline SVG. Sizes are not among them: every box of a copy is held to the size its original has on screen. */
export const LIFT_LOOK = [
  "display",
  "position",
  "top",
  "right",
  "bottom",
  "left",
  "flex-direction",
  "flex-wrap",
  "order",
  "grid-template-columns",
  "grid-template-rows",
  "grid-auto-flow",
  "grid-auto-columns",
  "grid-auto-rows",
  "grid-column-start",
  "grid-column-end",
  "grid-row-start",
  "grid-row-end",
  "row-gap",
  "column-gap",
  "align-items",
  "align-self",
  "align-content",
  "justify-content",
  "justify-items",
  "justify-self",
  "margin-top",
  "margin-right",
  "margin-bottom",
  "margin-left",
  "padding-top",
  "padding-right",
  "padding-bottom",
  "padding-left",
  "border-top-width",
  "border-right-width",
  "border-bottom-width",
  "border-left-width",
  "border-top-style",
  "border-right-style",
  "border-bottom-style",
  "border-left-style",
  "border-top-color",
  "border-right-color",
  "border-bottom-color",
  "border-left-color",
  "border-top-left-radius",
  "border-top-right-radius",
  "border-bottom-right-radius",
  "border-bottom-left-radius",
  "background-color",
  "background-image",
  "background-position",
  "background-size",
  "background-repeat",
  "background-origin",
  "background-clip",
  "box-shadow",
  "color",
  "opacity",
  "visibility",
  "font-family",
  "font-size",
  "font-weight",
  "font-style",
  "font-stretch",
  "font-variant-caps",
  "font-variant-numeric",
  "font-feature-settings",
  "line-height",
  "letter-spacing",
  "word-spacing",
  "text-align",
  "text-indent",
  "text-transform",
  "text-overflow",
  "white-space",
  "word-break",
  "overflow-wrap",
  "vertical-align",
  "direction",
  "text-decoration-line",
  "text-decoration-color",
  "text-decoration-style",
  "text-decoration-thickness",
  "text-underline-offset",
  "list-style-type",
  "list-style-position",
  "overflow-x",
  "overflow-y",
  "object-fit",
  "object-position",
  "fill",
  "stroke",
  "stroke-width",
  "stroke-linecap",
  "stroke-linejoin",
] as const;

const DROPPED = new Set(["id", "name", "for", "form", "style", "autofocus", "tabindex", "accesskey", "contenteditable", "draggable", "role", "title", "popover", "popovertarget", "popovertargetaction", "headers", "list", "slot", "nonce"]);
const LINKS = "a, area";
const XHTML = "http://www.w3.org/1999/xhtml";
const UNSEEN = "[inert], [hidden]";
const SHAPELESS = new Set(["inline", "contents", "none"]);
const PLACE = 100;
const MOVED = 0.5;
const SLOTS = 5;

/** 🎚️ What lifting needs of its host: the selector of the elements that may be lifted ({@link PET_PROPS} when absent or empty), whether the stage is quiet right now (then any learner input takes every lift back) and who hears that a fixture was taken back. */
export interface LiftingOptions {
  readonly props?: () => string | undefined;
  readonly quiet?: () => boolean;
  readonly reclaimed: (fixture: string) => void;
}

/** 🎭️ The lifts of one layer: {@link Lifting.realise} applies the lifts of a frame, {@link Lifting.measured} gives back whatever moved under a survey, {@link Lifting.release} gives back everything and tells the stage, {@link Lifting.end} gives back everything without a word and leaves nothing behind; `copies` are the copies in the layer right now (they lie behind everything else in it). */
export type Lifting = {
  readonly copies: ReadonlySet<Element>;
  readonly realise: (lifts: readonly LiftFrame[], origin: Point, size: number) => void;
  readonly measured: () => void;
  readonly release: () => void;
  readonly end: () => void;
};

/** 🏷️ An inline declaration as the author left it. */
type Declared = { readonly value: string; readonly priority: string };

/** 📦️ An element while it is lifted: the original, its copy, its box on screen when the lift began, its own opacity, the declarations lifting replaced and whether it had a `style` attribute at all; `painted` holds the copy's place (x, y), turn (degrees), opacity and whether the original is transparent — not a number before the first frame. */
type Held = {
  readonly fixture: string;
  readonly original: Element & ElementCSSInlineStyle;
  readonly copy: Element & ElementCSSInlineStyle;
  readonly box: DOMRect;
  readonly author: number;
  readonly opacity: Declared;
  readonly transition: Declared;
  readonly styled: boolean;
  readonly painted: Float64Array;
};
//#endregion 🔖️Rules

//#region 🔖️Copy
/** ✂️ Takes from a node of a copy every attribute by which a script could find it or a person reach it. */
function stripped(made: Element): void {
  for (const name of made.getAttributeNames()) {
    if (DROPPED.has(name) || name.startsWith("data-") || name.startsWith("on") || name.startsWith("aria-") || ((name === "href" || name === "xlink:href") && made.matches(LINKS))) made.removeAttribute(name);
  }
}

/** 👗️ Gives a node of a copy the computed look of its source node, the size its source has on screen and nothing that moves or takes a pointer. */
function dressed(source: Element, made: Element & ElementCSSInlineStyle, view: Window): void {
  const look = view.getComputedStyle(source);
  const style = made.style;
  for (const name of LIFT_LOOK) {
    const value = look.getPropertyValue(name);
    if (value !== "") style.setProperty(name, value);
  }
  const position = look.getPropertyValue("position");
  if (position === "fixed") style.setProperty("position", "absolute");
  else if (position === "sticky") style.setProperty("position", "relative");
  if ((source.namespaceURI === XHTML || source.localName === "svg") && !SHAPELESS.has(look.getPropertyValue("display"))) {
    const box = source.getBoundingClientRect();
    style.setProperty("box-sizing", "border-box");
    style.setProperty("width", `${box.width}px`);
    style.setProperty("height", `${box.height}px`);
    style.setProperty("min-width", "0px");
    style.setProperty("min-height", "0px");
    style.setProperty("max-width", "none");
    style.setProperty("max-height", "none");
    style.setProperty("flex-grow", "0");
    style.setProperty("flex-shrink", "0");
    style.setProperty("flex-basis", "auto");
  }
  style.setProperty("pointer-events", "none");
  style.setProperty("animation-name", "none");
}

/** 🎛️ Carries what a person entered into a form control of the original over to its copy, where cloning does not: the chosen option of a list, and to be sure the value and the checkedness of a field. Property writes only: no event fires. */
function filled(source: Element, made: Element): void {
  if (source.namespaceURI !== XHTML) return;
  if (source.localName === "select") (made as HTMLSelectElement).selectedIndex = (source as HTMLSelectElement).selectedIndex;
  else if (source.localName === "textarea") (made as HTMLTextAreaElement).value = (source as HTMLTextAreaElement).value;
  else if (source.localName === "input") {
    (made as HTMLInputElement).value = (source as HTMLInputElement).value;
    (made as HTMLInputElement).checked = (source as HTMLInputElement).checked;
  }
}

/** 🖨️ A copy of `original` for the pet layer, not yet in any document: a deep clone whose every node is {@link stripped}, dressed in the computed look of its source and filled with what a person entered; the copy itself is `inert`, `aria-hidden`, without margins, placed at the layer's corner (its frame moves it) and not selectable. Reads the computed style and the box of every node of the original, writes nothing to it. */
export function copyOf(original: Element, view: Window): Element & ElementCSSInlineStyle {
  const copy = original.cloneNode(true) as Element & ElementCSSInlineStyle;
  const sources = [original, ...original.querySelectorAll("*")];
  const copies = [copy, ...copy.querySelectorAll("*")] as (Element & ElementCSSInlineStyle)[];
  for (const [index, source] of sources.entries()) {
    const made = copies[index];
    if (made === undefined) break;
    stripped(made);
    dressed(source, made, view);
    filled(source, made);
  }
  const style = copy.style;
  style.setProperty("position", "absolute");
  style.setProperty("left", "0px");
  style.setProperty("top", "0px");
  style.setProperty("right", "auto");
  style.setProperty("bottom", "auto");
  style.setProperty("margin", "0px");
  style.setProperty("transform-origin", "50% 50%");
  style.setProperty("user-select", "none");
  copy.setAttribute("inert", "");
  copy.setAttribute("aria-hidden", "true");
  return copy;
}
//#endregion 🔖️Copy

//#region 🔖️Lifting
/** 📝️ The inline declaration of `name` on `style` as the author left it. */
function declared(style: CSSStyleDeclaration, name: string): Declared {
  return { value: style.getPropertyValue(name), priority: style.getPropertyPriority(name) };
}

/** ↩️ Puts an inline declaration back as the author left it: removed when there was none. */
function restored(style: CSSStyleDeclaration, name: string, kept: Declared): void {
  if (kept.value === "") style.removeProperty(name);
  else style.setProperty(name, kept.value, kept.priority);
}

/** 🖐️ Whether the pointer rests on `element` right now, as the engine's own `:hover` says; an engine that cannot tell says no. */
function hovered(element: Element): boolean {
  try {
    return element.matches(":hover");
  } catch {
    return false;
  }
}

/** 🎪️ Starts lifting in the pet layer `host`: copies are put first in it, behind everything the layer draws.
 *
 * {@link Lifting.realise} takes the lifts of every frame. A lift the layer sees for the first time finds its element
 * among the host's fixtures by its id ({@link fixtureId}); an element that is gone, no longer {@link copyable}, inert or
 * hidden, holds the focus or lies under the pointer is not lifted, and the stage hears that it was taken back. The copy
 * is drawn from the first frame on; the original turns transparent only while the lift is wholly opaque (the copy fades
 * in and out lying exactly on it), so the page never shows a hole or a double. A lift the frame no longer holds is
 * over: the original comes back, the copy goes. A fixture that was taken back is not lifted again before a frame
 * arrives that no longer holds it — the stage has to hear first. Writes only what changed: the copy's `transform` and
 * `opacity` (place to hundredths of a pixel, turn to hundredths of a degree, opacity to thousandths) and the original's
 * `opacity`; nothing at all while nothing is lifted.
 */
export function liftFixtures(host: Element, options: LiftingOptions): Lifting {
  const page = host.ownerDocument;
  const view = page.defaultView;
  const lifted = new Map<string, Held>();
  const originals = new Map<Node, Held>();
  const copies = new Set<Element>();
  const refused = new Set<string>();
  let listening = false;

  const give = (held: Held, tell: boolean): void => {
    lifted.delete(held.fixture);
    originals.delete(held.original);
    copies.delete(held.copy);
    held.copy.remove();
    const style = held.original.style;
    restored(style, "opacity", held.opacity);
    void view?.getComputedStyle(held.original).opacity;
    restored(style, "transition-property", held.transition);
    if (!held.styled && held.original.getAttribute("style") === "") held.original.removeAttribute("style");
    if (tell) {
      refused.add(held.fixture);
      options.reclaimed(held.fixture);
    }
    if (lifted.size === 0) deafen();
  };

  const release = (): void => {
    for (const held of [...lifted.values()]) give(held, true);
  };

  const heard = (event: Event): void => {
    if (event.type === "visibilitychange" && page.visibilityState !== "hidden") return;
    if (event.type === "visibilitychange" || event.type === "pagehide" || options.quiet?.() === true) {
      release();
      return;
    }
    let node = event.target as Node | null;
    for (let step = 0; node !== null && step <= LIFT_REACH; step++, node = node.parentNode) {
      const held = originals.get(node);
      if (held === undefined) continue;
      give(held, true);
      return;
    }
  };

  const passive = { capture: true, passive: true } as const;

  const listen = (): void => {
    if (listening || view === null) return;
    listening = true;
    for (const type of RECLAIMS) page.addEventListener(type, heard, passive);
    page.addEventListener("visibilitychange", heard, passive);
    view.addEventListener("pagehide", heard, passive);
  };

  function deafen(): void {
    if (!listening || view === null) return;
    listening = false;
    for (const type of RECLAIMS) page.removeEventListener(type, heard, true);
    page.removeEventListener("visibilitychange", heard, true);
    view.removeEventListener("pagehide", heard, true);
  }

  const refuse = (fixture: string): void => {
    refused.add(fixture);
    options.reclaimed(fixture);
  };

  const found = (fixture: string): Element | null => {
    for (const element of page.querySelectorAll(options.props?.() || PET_PROPS)) if (!host.contains(element) && fixtureId(element) === fixture) return element;
    return null;
  };

  const begin = (fixture: string): Held | null => {
    const element = view === null ? null : found(fixture);
    if (view === null || element === null || !("style" in element) || element.closest(UNSEEN) !== null || !copyable(element) || hovered(element)) return null;
    const focused = page.activeElement;
    if (focused !== null && focused !== page.body && element.contains(focused)) return null;
    const box = element.getBoundingClientRect();
    if (box.width <= 0 || box.height <= 0) return null;
    const original = element as Element & ElementCSSInlineStyle;
    const author = Number.parseFloat(view.getComputedStyle(original).opacity);
    const held: Held = {
      fixture,
      original,
      copy: copyOf(original, view),
      box,
      author: Number.isFinite(author) ? author : 1,
      opacity: declared(original.style, "opacity"),
      transition: declared(original.style, "transition-property"),
      styled: original.hasAttribute("style"),
      painted: new Float64Array(SLOTS).fill(Number.NaN),
    };
    original.style.setProperty("transition-property", "none", "important");
    held.painted[4] = 0;
    host.prepend(held.copy);
    lifted.set(fixture, held);
    originals.set(original, held);
    copies.add(held.copy);
    listen();
    return held;
  };

  const paint = (held: Held, lift: LiftFrame, origin: Point, size: number): void => {
    const { painted, copy, original } = held;
    const x = rounded(held.box.left - origin.x + lift.dx * size, PLACE);
    const y = rounded(held.box.top - origin.y + lift.dy * size, PLACE);
    const degrees = rounded(lift.tilt * 360, PLACE);
    if (x !== painted[0] || y !== painted[1] || degrees !== painted[2]) {
      painted[0] = x;
      painted[1] = y;
      painted[2] = degrees;
      copy.style.setProperty("transform", `translate(${x}px, ${y}px) rotate(${degrees}deg)`);
    }
    const opacity = rounded(held.author * lift.opacity);
    if (opacity !== painted[3]) {
      painted[3] = opacity;
      copy.style.setProperty("opacity", String(opacity));
    }
    const transparent = rounded(lift.opacity) >= 1 ? 1 : 0;
    if (transparent !== painted[4]) {
      painted[4] = transparent;
      if (transparent === 1) original.style.setProperty("opacity", "0", "important");
      else restored(original.style, "opacity", held.opacity);
    }
  };

  return {
    copies,
    realise: (lifts, origin, size) => {
      if (lifts.length === 0 && lifted.size === 0 && refused.size === 0) return;
      const wanted = new Set(lifts.map((lift) => lift.fixture));
      for (const fixture of refused) if (!wanted.has(fixture)) refused.delete(fixture);
      for (const held of [...lifted.values()]) if (!wanted.has(held.fixture)) give(held, false);
      for (const lift of lifts) {
        if (refused.has(lift.fixture)) continue;
        const held = lifted.get(lift.fixture) ?? begin(lift.fixture);
        if (held === null) refuse(lift.fixture);
        else paint(held, lift, origin, size);
      }
    },
    measured: () => {
      for (const held of [...lifted.values()]) {
        const { original, box } = held;
        const now = original.isConnected && original.closest(UNSEEN) === null ? original.getBoundingClientRect() : null;
        if (now === null || Math.abs(now.left - box.left) > MOVED || Math.abs(now.top - box.top) > MOVED || Math.abs(now.width - box.width) > MOVED || Math.abs(now.height - box.height) > MOVED) give(held, true);
      }
    },
    release,
    end: () => {
      for (const held of [...lifted.values()]) give(held, false);
      refused.clear();
      deafen();
    },
  };
}
//#endregion 🔖️Lifting
