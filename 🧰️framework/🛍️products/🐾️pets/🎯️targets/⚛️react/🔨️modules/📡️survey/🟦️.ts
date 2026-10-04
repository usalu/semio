/** 📡️ The senses of the pet layer: measures a document into the `surveyed` event of the stage (the top edges pets may stand on, the floor, everything they must not cover, the sides they may climb), tells when that measurement went stale and turns the pointer into stage events.
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
import type { Fixture, Point, Pointed, Rect, Surface, Surveyed, Unpointed, Wall } from "@semio-tech/pets";
//#endregion 🔌️Adapters

//#region 🔖️Selectors
/** 🪵️ The elements whose top edge carries pets unless the host names others. */
export const PET_SURFACES = "[data-pet-surface]";

/** 🎛️ Everything a person can operate; pets neither stand in front of it nor take a press that was meant for it. */
export const PET_CONTROLS =
  'a[href], button, input, select, textarea, summary, [role="button"], [role="link"], [role="checkbox"], [role="radio"], [role="tab"], [role="menuitem"], [role="option"], [role="slider"], [role="switch"], [tabindex]:not([tabindex="-1"]), [contenteditable]:not([contenteditable="false"])';

/** 🕹️ What a press never takes away from a person's hand unless the host names more: every control, and labels, which hand their press on to their control. A pointer over one of them is over a control. */
export const PET_PRESS_CONTROLS = `${PET_CONTROLS}, label`;

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

/** 🧸️ The elements a pet may play with unless the host names others: their copies are what a pet pushes out of their stack ({@link PROP_KEY} names what each of them is about). */
export const PET_PROPS = "[data-pet-prop]";

/** 🔑️ The attribute whose value is the key of a fixture: a ground in the vocabulary of the menagerie (`<quiz>`, `<quiz>/<task>`, `<quiz>/<task>/<item>`). An element without a key is no fixture, whatever selector found it. */
export const PROP_KEY = "data-pet-prop";

/** 🧮️ The most elements a fixture may consist of, itself included: a copy of more would cost more than a frame can spare. */
export const FIXTURE_ELEMENTS = 80;

/** 🎞️ What a copy cannot show as it is — drawings made by a script, films, sounds, documents of their own: an element that is or holds one is no fixture. */
export const UNCOPYABLE = "canvas, video, audio, iframe, object, embed";

const TABLE_PARTS = "tr, td, th, thead, tbody, tfoot, caption, col, colgroup";
const XHTML = "http://www.w3.org/1999/xhtml";
const UNSEEN = "[inert], [hidden]";
const EDGE_SLACK = 0.5;
const NO_EDGES: readonly Surface[] = [];
const SHOW_ELEMENT = 1;
const FILTER_ACCEPT = 1;
const FILTER_REJECT = 2;
//#endregion 🔖️Selectors

//#region 🔖️Measurement
/** 🎚️ What a survey looks for and relative to what it measures: the selector of the elements whose top edge carries pets
 * ({@link PET_SURFACES} when absent), the selector of what pets must not cover ({@link PET_KEEPOUTS} when absent), the
 * selector of more elements whose sides pets may climb besides those of the surfaces (none when absent), the selector
 * of the elements a pet may play with ({@link PET_PROPS} when absent), where the pointer is in viewport pixels (`null`
 * or absent when it is not on the page) and the element whose box is the stage (the pet layer; the viewport when
 * absent or without an area), into which the survey never looks. */
export interface SurveyOptions {
  readonly surfaces?: string;
  readonly keepouts?: string;
  readonly walls?: string;
  readonly props?: string;
  readonly pointer?: Point | null;
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

/** 🧗️ The id of a wall: the side (`-1` left, `1` right) of the element with that surface id, kept for as long as the element lives. */
export function wallId(element: Element, side: 1 | -1): string {
  return `${surfaceId(element)}:${side < 0 ? "left" : "right"}`;
}

const FIXTURE_IDS = new WeakMap<Element, string>();
let fixtures = 0;

/** 🪪️ The id of the fixture an element is: issued once per element and kept for as long as the element lives, so a lift names its element across surveys. */
export function fixtureId(element: Element): string {
  const known = FIXTURE_IDS.get(element);
  if (known !== undefined) return known;
  fixtures += 1;
  const id = `f${fixtures}`;
  FIXTURE_IDS.set(element, id);
  return id;
}

/** 🧩️ Whether an element of a fixture would keep a copy from standing in for it: it is {@link UNCOPYABLE}, or a custom element (cloning one runs its author's code). */
function opaque(element: Element): boolean {
  return element.localName.includes("-") || element.matches(UNCOPYABLE);
}

/** 📋️ Whether a copy of `element` can stand in for it in the pet layer: it is an element of HTML or an `<svg>`, no part of a table (a row or a cell means nothing outside its table), neither it nor anything in it is {@link UNCOPYABLE} or a custom element, and it consists of at most {@link FIXTURE_ELEMENTS} elements. Reads the tree only, and stops at the first element too many. */
export function copyable(element: Element): boolean {
  if ((element.namespaceURI !== XHTML && element.localName !== "svg") || element.matches(TABLE_PARTS) || opaque(element)) return false;
  const walker = documentOf(element).createTreeWalker(element, SHOW_ELEMENT);
  let count = 1;
  for (let node = walker.nextNode() as Element | null; node !== null; node = walker.nextNode() as Element | null) {
    count += 1;
    if (count > FIXTURE_ELEMENTS || opaque(node)) return false;
  }
  return true;
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

/** 👁️ For every selector, the elements under `root` that match it, in document order, without whatever is inert or hidden or inside something that is, and without the element `apart` and everything in it (the stage's own element, whose drawings are no part of the page); an absent selector finds nothing.
 *
 * Where the page hides nothing, the browser's own queries answer. Otherwise one walk over the tree passes over every
 * inert or hidden subtree and over `apart` at their roots, so a page that keeps most of itself inert behind the scenes
 * costs what it shows, not what it holds, and a stage full of pets costs nothing; an element the walk reaches is asked
 * once whether it matches any of the selectors, and only then which. */
function seen(root: Document | Element, selectors: readonly (string | undefined)[], apart: Element | null): Element[][] {
  if ((!("documentElement" in root) && root.closest(UNSEEN) !== null) || apart?.contains(root) === true) return selectors.map(() => []);
  if (root.querySelector(UNSEEN) === null) return selectors.map((selector) => (selector === undefined ? [] : [...root.querySelectorAll(selector)].filter((element) => apart?.contains(element) !== true)));
  const found = selectors.map((): Element[] => []);
  const named = selectors.filter((selector) => selector !== undefined);
  const any = named.length > 1 ? named.join(", ") : null;
  const walker = documentOf(root).createTreeWalker(root, SHOW_ELEMENT, (node) => (node === apart || (node as Element).hasAttribute("inert") || (node as Element).hasAttribute("hidden") ? FILTER_REJECT : FILTER_ACCEPT));
  for (let node = walker.nextNode() as Element | null; node !== null; node = walker.nextNode() as Element | null) {
    if (any !== null && !node.matches(any)) continue;
    for (const [index, selector] of selectors.entries()) if (selector !== undefined && node.matches(selector)) found[index]!.push(node);
  }
  return found;
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
 * Walls are the sides of the surfaces' elements, in their order, then the sides of the other elements matching `walls`
 * (which are walls and nothing else: no surface, no solid), in document order; each element's left side (`side` −1,
 * its air on the left) before its right side (1). A side counts only where it can be seen — a side a scrolling or
 * clipping ancestor cuts away is no wall — and runs from the top of what one can see of its element down to the
 * bottom of it, under the id {@link wallId} and the surface id of its element (whether or not that element carries a
 * surface right now: a part narrower than {@link SURFACE_WIDTH} or one whose top is scrolled away still has sides).
 * Which stretches of a wall are free to climb is the core's to cut.
 *
 * Fixtures are the elements matching `props` that carry a key ({@link PROP_KEY}), in document order, each under the id
 * of its element ({@link fixtureId}) with its whole box: only an element that is neither inert nor hidden, that shows
 * whole (no scrolling or clipping ancestor cuts any of it, and it lies wholly on the stage — a copy of it is drawn
 * beyond every clip), that neither holds the focus nor lies under the pointer (its edges included) and that a copy
 * can stand in for ({@link copyable}). Nothing about an element but its key decides whether it is a fixture: no value,
 * no state of the host.
 *
 * Nothing inside the stage's own element (`frame`, the pet layer, with its drawings, pads and copies) is a surface, a
 * keep-out, a wall or a fixture: the survey never looks into it.
 *
 * All numbers are pixels relative to the stage ({@link stageBox}).
 */
export function survey(root: Document | Element, options: SurveyOptions = {}): Surveyed {
  const page = documentOf(root);
  const view = page.defaultView;
  const stage = stageBox(root, options.frame);
  const [carrying = [], kept = [], walled = [], props = []] = seen(root, [options.surfaces ?? PET_SURFACES, options.keepouts ?? PET_KEEPOUTS, options.walls, options.props ?? PET_PROPS], options.frame ?? null);
  const grounds = new Set<Element>(carrying);
  const carriers = new Set<Element>();
  const clips = new Map<Element, Edges | null>();
  const surfaces: Surface[] = [];
  const solids: Rect[] = [];
  const walls: Wall[] = [];
  const sides = (element: Element, box: DOMRect, region: Edges): void => {
    const y0 = region.top - stage.y;
    const y1 = region.bottom - stage.y;
    if (region.left === box.left) walls.push({ id: wallId(element, -1), surface: surfaceId(element), side: -1, x: box.left - stage.x, y0, y1 });
    if (region.right === box.right) walls.push({ id: wallId(element, 1), surface: surfaceId(element), side: 1, x: box.right - stage.x, y0, y1 });
  };
  for (const element of carrying) {
    for (let node = element.parentElement; node !== null && !carriers.has(node); node = node.parentElement) carriers.add(node);
    const box = element.getBoundingClientRect();
    const region = shown(box, clipOf(element, view, clips));
    if (region === null || staged(region, stage) === null) continue;
    solids.push({ x: region.left - stage.x - KEEPOUT_MARGIN, y: region.top - stage.y, width: region.right - region.left + 2 * KEEPOUT_MARGIN, height: region.bottom - region.top });
    sides(element, box, region);
    if (region.top > box.top || region.right - region.left < SURFACE_WIDTH) continue;
    surfaces.push({ id: surfaceId(element), x0: region.left - stage.x, x1: region.right - stage.x, y: region.top - stage.y });
  }
  for (const element of walled) {
    if (grounds.has(element)) continue;
    const box = element.getBoundingClientRect();
    const region = shown(box, clipOf(element, view, clips));
    if (region !== null && staged(region, stage) !== null) sides(element, box, region);
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
  const fixtures: Fixture[] = [];
  const pointer = options.pointer ?? null;
  for (const element of props) {
    const key = element.getAttribute(PROP_KEY);
    if (key === null || key === "") continue;
    const box = element.getBoundingClientRect();
    const region = shown(box, clipOf(element, view, clips));
    if (region === null || region.left !== box.left || region.top !== box.top || region.right !== box.right || region.bottom !== box.bottom) continue;
    const x = box.left - stage.x;
    const y = box.top - stage.y;
    if (x < 0 || y < 0 || x + box.width > stage.width || y + box.height > stage.height) continue;
    if (focused !== null && focused !== page.body && focused !== page.documentElement && element.contains(focused)) continue;
    if (pointer !== null && pointer.x >= box.left && pointer.x <= box.right && pointer.y >= box.top && pointer.y <= box.bottom) continue;
    if (copyable(element)) fixtures.push({ id: fixtureId(element), key, x, y, width: box.width, height: box.height });
  }
  return { kind: "surveyed", width: stage.width, height: stage.height, surfaces, keepouts, walls, fixtures };
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
/** 🧭️ How pointer positions become stage positions and what lies under them: the origin of the stage in viewport pixels
 * as of the last survey (the viewport's own origin when absent) and the selector of the controls, read at every event
 * ({@link PET_PRESS_CONTROLS} when absent). */
export interface PointerOptions {
  readonly origin?: () => Point;
  readonly controls?: () => string;
}

/** 🖱️ Turns the pointer into stage events and returns the function that ends the feed.
 *
 * Every `pointermove` and `pointerdown` on the window becomes `pointed`: `over` a control while what lies under the
 * pointer has an interactive ancestor (`controls`) or while the learner is busy with a press the pets did not take —
 * selecting text, dragging something of the host; from the event after its press on — and over free space otherwise. A mouse or pen leaving the page
 * becomes `unpointed`, and so does a finger that is lifted or whose touch is cancelled (a finger has no hover: it points
 * only while it touches, so the pets do not keep looking up to where a finger tapped). A press the pets take (the hand,
 * `../🤏️grasp/🟦️.ts`) is stopped before it reaches the window's bubbling listeners, so every press this feed hears is
 * somebody else's; it is busy until that press ends or a move arrives without a button. The pet layer takes no
 * pointer events, so the target is always what lies beneath the pets; the listeners are passive and leave the event
 * untouched.
 */
export function watchPointer(view: SurveyHost, emit: (event: Pointed | Unpointed) => void, options: PointerOptions = {}): () => void {
  const page = view.document.documentElement;
  let busy = false;
  const at = (event: PointerEvent): Pointed => {
    const origin = options.origin?.();
    const target = event.target as Partial<Element> | null;
    const over = busy || (typeof target?.closest === "function" && target.closest(options.controls?.() ?? PET_PRESS_CONTROLS) !== null) ? "control" : "free";
    return { kind: "pointed", x: event.clientX - (origin?.x ?? 0), y: event.clientY - (origin?.y ?? 0), over };
  };
  const moved = (event: PointerEvent): void => {
    busy = busy && event.buttons !== 0;
    emit(at(event));
  };
  const pressed = (event: PointerEvent): void => {
    emit(at(event));
    if (event.button === 0 && event.isPrimary !== false) busy = true;
  };
  const left = (event: PointerEvent): void => {
    if (event.pointerType !== "touch") emit({ kind: "unpointed" });
  };
  const lifted = (event: PointerEvent): void => {
    if (event.isPrimary !== false) busy = false;
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
