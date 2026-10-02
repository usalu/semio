/** 📡️ The senses of the pet layer: measures a document into the `surveyed` event of the stage (the top edges pets may stand on, the floor, everything they must not cover), tells when that measurement went stale and turns the pointer into stage events.
 *
 * Every DOM read of the pets target happens here, in one batch per call and without a single write. The document and
 * the window are passed in, so the module runs against a real page, jsdom or a fake alike. Nothing here ever calls
 * `preventDefault` or `stopPropagation`; every listener is passive.
 *
 * @see ../🫧️layer/🟦️.tsx — the only caller in production
 * @see https://www.w3.org/WAI/WCAG22/Understanding/focus-not-obscured-minimum.html — why the focused element is a keep-out
 * @see https://developer.mozilla.org/en-US/docs/Web/API/Element/getBoundingClientRect
 */

//#region 🔌️Adapters
import type { Point, Pointed, Poked, Rect, Surface, Surveyed, Unpointed } from "@semio-tech/pets";
//#endregion 🔌️Adapters

//#region 🔖️Selectors
/** 🪵️ The elements whose top edge carries pets unless the host names others. */
export const PET_SURFACES = "[data-pet-surface]";

/** 🎛️ Everything a person can operate; pets neither stand in front of it nor take a poke that was meant for it. */
export const PET_CONTROLS =
  'a[href], button, input, select, textarea, summary, [role="button"], [role="link"], [role="checkbox"], [role="radio"], [role="tab"], [role="menuitem"], [role="option"], [role="slider"], [role="switch"], [tabindex]:not([tabindex="-1"]), [contenteditable]:not([contenteditable="false"])';

/** 📄️ What a person reads: blocks of text, code, the read-outs of a form (`output`, `progress`, `meter`) and the regions that announce something (`alert`, `status`). A selector cannot name text that lies bare in a `div` or a `span`: a host marks such an element `data-pet-keepout`. */
export const PET_TEXTS = 'p, li, h1, h2, h3, h4, h5, h6, label, figcaption, td, th, legend, dt, dd, blockquote, pre, code, output, progress, meter, [role="alert"], [role="status"]';

/** 🚧️ What pets must not cover unless the host names something else: controls, what a person reads and whatever is marked `data-pet-keepout`. */
export const PET_KEEPOUTS = `${PET_CONTROLS}, ${PET_TEXTS}, [data-pet-keepout]`;

/** 🧱️ The id of the surface every stage has: the bottom edge of the stage. */
export const FLOOR = "floor";

/** 📏️ The narrowest element that counts as a surface, in pixels. */
export const SURFACE_WIDTH = 48;

/** 🫧️ How far a keep-out reaches beyond the box of its element, in pixels. */
export const KEEPOUT_MARGIN = 4;

/** 🎯️ How far the keep-out of the focused element reaches beyond its box, in pixels (room for the focus indicator). */
export const FOCUS_MARGIN = 8;

const UNSEEN = "[inert], [hidden]";
const EDGE_SLACK = 0.5;
const NO_EDGES: readonly Surface[] = [];
const SHOW_ELEMENT = 1;
const FILTER_ACCEPT = 1;
const FILTER_REJECT = 2;
//#endregion 🔖️Selectors

//#region 🔖️Measurement
/** 🎚️ What a survey looks for and relative to what it measures: the selector of the elements whose top edge carries pets
 * ({@link PET_SURFACES} when absent), the selector of what pets must not cover ({@link PET_KEEPOUTS} when absent) and the
 * element whose box is the stage (the pet layer; the viewport when absent or without an area). */
export interface SurveyOptions {
  readonly surfaces?: string;
  readonly keepouts?: string;
  readonly frame?: Element | null;
}

type Edges = { readonly left: number; readonly top: number; readonly right: number; readonly bottom: number };

const SURFACE_IDS = new WeakMap<Element, string>();
let issued = 0;

/** 🏷️ The id of the surface an element carries: issued once per element and kept for as long as the element lives, so the stage recognises a surface that moved. */
export function surfaceId(element: Element): string {
  const known = SURFACE_IDS.get(element);
  if (known !== undefined) return known;
  issued += 1;
  const id = `s${issued}`;
  SURFACE_IDS.set(element, id);
  return id;
}

/** 📃️ The document a root is, or belongs to. */
function documentOf(root: Document | Element): Document {
  return "documentElement" in root ? root : root.ownerDocument;
}

/** 🖼️ The stage in viewport pixels: the box of `frame` when it has an area, otherwise the viewport without its scrollbars. */
export function stageBox(root: Document | Element, frame?: Element | null): Rect {
  const box = frame?.getBoundingClientRect();
  if (box !== undefined && box.width > 0 && box.height > 0) return { x: box.left, y: box.top, width: box.width, height: box.height };
  const page = documentOf(root);
  const view = page.defaultView;
  return { x: 0, y: 0, width: page.documentElement.clientWidth || (view?.innerWidth ?? 0), height: page.documentElement.clientHeight || (view?.innerHeight ?? 0) };
}

/** ✂️ The region the ancestors of `element` let through: the intersection of the boxes of every ancestor that clips its overflow (scrolling panes, clipped cards), `null` when nothing clips. The root and the body never count: their overflow belongs to the viewport, which the stage already is. */
function clipOf(element: Element, view: Window | null, known: Map<Element, Edges | null>): Edges | null {
  const parent = element.parentElement;
  if (parent === null || view === null || parent === parent.ownerDocument.body || parent === parent.ownerDocument.documentElement) return null;
  const cached = known.get(parent);
  if (cached !== undefined) return cached;
  const above = clipOf(parent, view, known);
  const style = view.getComputedStyle(parent);
  let clip = above;
  if ((style.overflowX || "visible") !== "visible" || (style.overflowY || "visible") !== "visible") {
    const box = parent.getBoundingClientRect();
    clip = above === null ? { left: box.left, top: box.top, right: box.right, bottom: box.bottom } : { left: Math.max(above.left, box.left), top: Math.max(above.top, box.top), right: Math.min(above.right, box.right), bottom: Math.min(above.bottom, box.bottom) };
  }
  known.set(parent, clip);
  return clip;
}

/** 👁️ The elements under `root` that match `surfaces` and those that match `keepouts`, each in document order, without whatever is inert or hidden or inside something that is.
 *
 * Where the page hides nothing, the browser's own queries answer. Otherwise one walk over the tree passes over every
 * inert or hidden subtree at its root, so a page that keeps most of itself inert behind the scenes costs what it shows,
 * not what it holds. */
function seen(root: Document | Element, surfaces: string, keepouts: string): { readonly carrying: Element[]; readonly kept: Element[] } {
  if (!("documentElement" in root) && root.closest(UNSEEN) !== null) return { carrying: [], kept: [] };
  if (root.querySelector(UNSEEN) === null) return { carrying: [...root.querySelectorAll(surfaces)], kept: [...root.querySelectorAll(keepouts)] };
  const carrying: Element[] = [];
  const kept: Element[] = [];
  const walker = documentOf(root).createTreeWalker(root, SHOW_ELEMENT, (node) => ((node as Element).hasAttribute("inert") || (node as Element).hasAttribute("hidden") ? FILTER_REJECT : FILTER_ACCEPT));
  for (let node = walker.nextNode() as Element | null; node !== null; node = walker.nextNode() as Element | null) {
    if (node.matches(surfaces)) carrying.push(node);
    if (node.matches(keepouts)) kept.push(node);
  }
  return { carrying, kept };
}

/** 👀️ What of a box one can see: the box cut down to the region its clipping ancestors let through (`clip`); `null` when it has no area or all of it is scrolled or clipped away. */
function shown(box: DOMRect, clip: Edges | null): Edges | null {
  if (box.width <= 0 || box.height <= 0) return null;
  if (clip === null) return { left: box.left, top: box.top, right: box.right, bottom: box.bottom };
  const left = Math.max(box.left, clip.left);
  const top = Math.max(box.top, clip.top);
  const right = Math.min(box.right, clip.right);
  const bottom = Math.min(box.bottom, clip.bottom);
  return right > left && bottom > top ? { left, top, right, bottom } : null;
}

/** ⬛️ A region of the viewport as a box in stage pixels, `null` when it lies outside the stage. */
function staged(region: Edges, stage: Rect): Rect | null {
  const x = region.left - stage.x;
  const y = region.top - stage.y;
  const width = region.right - region.left;
  const height = region.bottom - region.top;
  return x < stage.width && x + width > 0 && y < stage.height && y + height > 0 ? { x, y, width, height } : null;
}

/** 🛑️ The keep-out of what one can see of an element: that region grown by `margin` in stage pixels. The margin never reaches over an edge the element lies under — what begins at or below a surface that spans it is part of that surface's body, and a pet standing on the edge covers nothing of it. `null` outside the stage. */
function keepout(region: Edges, margin: number, edges: readonly Surface[], stage: Rect): Rect | null {
  let top = region.top - margin;
  for (const edge of edges) {
    const y = edge.y + stage.y;
    if (region.top >= y - EDGE_SLACK && top < y && region.left < edge.x1 + stage.x && region.right > edge.x0 + stage.x) top = y;
  }
  return staged({ left: region.left - margin, top, right: region.right + margin, bottom: region.bottom + margin }, stage);
}

/** 🗺️ Measures `root` into the `surveyed` event of a stage, reads only.
 *
 * Surfaces are the top edges of the elements matching `surfaces` that are neither inert nor hidden, cross the stage,
 * are not clipped away by a scrolling ancestor and stay at least {@link SURFACE_WIDTH} wide, in document order, each
 * under the id of its element ({@link surfaceId}); the last surface is the {@link FLOOR}. A host that wants pets to
 * follow a silhouette — a card with a title tab standing up on its body — names every part: each part carries pets on
 * its own top edge, and since every part is solid (below), an edge is free only where no other part stands on it.
 *
 * Keep-outs, each cut down to what its scrolling and clipping ancestors let one see (what is scrolled out of a pane
 * blocks nothing):
 * 1. the elements matching `keepouts`, grown by {@link KEEPOUT_MARGIN} — except the surfaces themselves and whatever
 *    contains one — where the margin never reaches over an edge the element lies under;
 * 2. the surfaces' own elements, grown by {@link KEEPOUT_MARGIN} to the left and to the right only (never upwards: a
 *    part that stands on an edge leaves the rest of that edge free): pets stand on a thing, never in front of another
 *    one, and not shoulder to shoulder with its side;
 * 3. the focused element grown by {@link FOCUS_MARGIN} unless it is the page itself or a region that contains a surface
 *    (a focused surface keeps its whole margin, so nobody stands in its focus indicator).
 *
 * All numbers are pixels relative to the stage ({@link stageBox}).
 */
export function survey(root: Document | Element, options: SurveyOptions = {}): Surveyed {
  const page = documentOf(root);
  const view = page.defaultView;
  const stage = stageBox(root, options.frame);
  const { carrying, kept } = seen(root, options.surfaces ?? PET_SURFACES, options.keepouts ?? PET_KEEPOUTS);
  const grounds = new Set<Element>(carrying);
  const carriers = new Set<Element>();
  const clips = new Map<Element, Edges | null>();
  const surfaces: Surface[] = [];
  const solids: Rect[] = [];
  for (const element of carrying) {
    for (let node = element.parentElement; node !== null && !carriers.has(node); node = node.parentElement) carriers.add(node);
    const box = element.getBoundingClientRect();
    const region = shown(box, clipOf(element, view, clips));
    if (region === null || staged(region, stage) === null) continue;
    solids.push({ x: region.left - stage.x - KEEPOUT_MARGIN, y: region.top - stage.y, width: region.right - region.left + 2 * KEEPOUT_MARGIN, height: region.bottom - region.top });
    if (region.top > box.top || region.right - region.left < SURFACE_WIDTH) continue;
    surfaces.push({ id: surfaceId(element), x0: region.left - stage.x, x1: region.right - stage.x, y: region.top - stage.y });
  }
  surfaces.push({ id: FLOOR, x0: 0, x1: stage.width, y: stage.height });
  const keepouts: Rect[] = [];
  for (const element of kept) {
    if (grounds.has(element) || carriers.has(element)) continue;
    const region = shown(element.getBoundingClientRect(), clipOf(element, view, clips));
    const box = region === null ? null : keepout(region, KEEPOUT_MARGIN, surfaces, stage);
    if (box !== null) keepouts.push(box);
  }
  keepouts.push(...solids);
  const focused = page.activeElement;
  if (focused !== null && focused !== page.body && focused !== page.documentElement && !carriers.has(focused) && root.contains(focused) && focused.closest(UNSEEN) === null) {
    const region = shown(focused.getBoundingClientRect(), clipOf(focused, view, clips));
    const box = region === null ? null : keepout(region, FOCUS_MARGIN, grounds.has(focused) ? NO_EDGES : surfaces, stage);
    if (box !== null) keepouts.push(box);
  }
  return { kind: "surveyed", width: stage.width, height: stage.height, surfaces, keepouts };
}
//#endregion 🔖️Measurement

//#region 🔖️Staleness
/** 🪟️ What the watchers need of a window; the layer passes `window`, a test may pass a fake. */
export type SurveyHost = Pick<Window, "document" | "addEventListener" | "removeEventListener"> & {
  readonly ResizeObserver?: typeof ResizeObserver;
  readonly MutationObserver?: typeof MutationObserver;
};

/** 🙈️ What a survey watch leaves out: an element whose children never make a survey stale (the pet layer, which changes with every frame) — neither their mutations nor the end of a transition inside it, which a host's stylesheet may give them with every write of a frame; its own watched attributes still count, since they say whether the layer itself went inert or hidden. */
export interface SurveyWatchOptions {
  readonly ignore?: Element | null;
}

/** 🧬️ The attributes whose change can move, hide or reveal a surface or a keep-out. */
export const SURVEY_ATTRIBUTES = ["class", "hidden", "inert", "open"] as const;

/** 👂️ Calls `stale` whenever the last survey of `root` may no longer hold, and returns the function that ends the watch.
 *
 * `stale(true)` means the page moves under the pets right now (scrolling, resizing) and wants a new survey with the
 * next frame; `stale(false)` means something changed once (the tree, a class, the focus, the end of a transition) and
 * a survey within a few ticks is soon enough. The watch observes the size of the root (the first notification of an
 * observer about a root that already has a size is its greeting, not a change, and is passed over), mutations of its
 * subtree ({@link SURVEY_ATTRIBUTES} and the child lists), and listens passively, in the capture phase, to `scroll`,
 * `resize`, `focusin`, `focusout` and `transitionend` (a transition that ends inside the ignored element is passed
 * over).
 */
export function watchSurvey(view: SurveyHost, root: Document | Element, stale: (urgent: boolean) => void, options: SurveyWatchOptions = {}): () => void {
  const page = view.document;
  const subject = "documentElement" in root ? root.documentElement : root;
  const ignore = options.ignore ?? null;
  const moved = (): void => stale(true);
  const changed = (): void => stale(false);
  const settled = (event: Event): void => {
    const target = event.target;
    if (ignore === null || target === null || !("nodeType" in target) || !ignore.contains(target as Node)) stale(false);
  };
  let greeted = subject.clientWidth === 0 && subject.clientHeight === 0;
  const resizes =
    view.ResizeObserver === undefined
      ? null
      : new view.ResizeObserver(() => {
          if (greeted) stale(true);
          greeted = true;
        });
  resizes?.observe(subject);
  const mutations = view.MutationObserver === undefined ? null : new view.MutationObserver((records) => {
    if (ignore === null || records.some((record) => (record.target === ignore ? record.type === "attributes" : !ignore.contains(record.target)))) stale(false);
  });
  mutations?.observe(subject, { subtree: true, childList: true, attributes: true, attributeFilter: [...SURVEY_ATTRIBUTES] });
  const passive = { capture: true, passive: true } as const;
  page.addEventListener("scroll", moved, passive);
  view.addEventListener("resize", moved, passive);
  page.addEventListener("focusin", changed, passive);
  page.addEventListener("focusout", changed, passive);
  page.addEventListener("transitionend", settled, passive);
  return () => {
    resizes?.disconnect();
    mutations?.disconnect();
    page.removeEventListener("scroll", moved, true);
    view.removeEventListener("resize", moved, true);
    page.removeEventListener("focusin", changed, true);
    page.removeEventListener("focusout", changed, true);
    page.removeEventListener("transitionend", settled, true);
  };
}
//#endregion 🔖️Staleness

//#region 🔖️Pointer
/** 🧭️ How pointer positions become stage positions and which of them touch a pet: the origin of the stage in viewport
 * pixels as of the last survey (the viewport's own origin when absent) and whether a stage position lies within the box
 * of an actor (nothing is ever poked when absent). */
export interface PointerOptions {
  readonly origin?: () => Point;
  readonly hit?: (x: number, y: number) => boolean;
}

/** 🖱️ Turns the pointer into stage events and returns the function that ends the feed.
 *
 * Every `pointermove` and `pointerdown` on the window becomes `pointed`; a mouse or pen leaving the page becomes
 * `unpointed`, and so does a finger that is lifted or whose touch is cancelled (a finger has no hover: it points only
 * while it touches, so a pet it tapped does not stay see-through). A primary `pointerdown` becomes `poked` as well when
 * what it hit has no interactive ancestor ({@link PET_CONTROLS}) and the position lies within an actor's box. The pet
 * layer takes no pointer events, so the target is always what lies beneath the pets; the listeners are passive and
 * leave the event untouched.
 */
export function watchPointer(view: SurveyHost, emit: (event: Pointed | Unpointed | Poked) => void, options: PointerOptions = {}): () => void {
  const page = view.document.documentElement;
  const at = (event: PointerEvent): Point => {
    const origin = options.origin?.();
    return { x: event.clientX - (origin?.x ?? 0), y: event.clientY - (origin?.y ?? 0) };
  };
  const moved = (event: PointerEvent): void => {
    emit({ kind: "pointed", ...at(event) });
  };
  const pressed = (event: PointerEvent): void => {
    const point = at(event);
    emit({ kind: "pointed", ...point });
    if (event.button !== 0 || event.isPrimary === false) return;
    const target = event.target as Partial<Element> | null;
    if (typeof target?.closest === "function" && target.closest(PET_CONTROLS) !== null) return;
    if (options.hit?.(point.x, point.y) === true) emit({ kind: "poked", ...point });
  };
  const left = (event: PointerEvent): void => {
    if (event.pointerType !== "touch") emit({ kind: "unpointed" });
  };
  const lifted = (event: PointerEvent): void => {
    if (event.pointerType === "touch") emit({ kind: "unpointed" });
  };
  const passive = { passive: true } as const;
  view.addEventListener("pointermove", moved, passive);
  view.addEventListener("pointerdown", pressed, passive);
  view.addEventListener("pointerup", lifted, passive);
  view.addEventListener("pointercancel", lifted, passive);
  page.addEventListener("pointerleave", left, passive);
  return () => {
    view.removeEventListener("pointermove", moved);
    view.removeEventListener("pointerdown", pressed);
    view.removeEventListener("pointerup", lifted);
    view.removeEventListener("pointercancel", lifted);
    page.removeEventListener("pointerleave", left);
  };
}
//#endregion 🔖️Pointer
