import * as React from "react";
import { borderStatePriority, boxDrawnSides, resolveBorderSegments, resolveBoxStates, withoutLoneEmphasis, type BorderPoint, type BorderSide, type BorderState, type StateBox } from "../../🔨️modules/📏️border-presentation/🟦️.ts";
import { type SharedBorderClip, type SharedBorderTransform, createSharedBorderSnap, sharedBorderAxisScale, sharedOutlineSegments, clipSharedOutlineSegments, visibleSharedOutlineSegments, createSharedBorderJunctionResolver, leadingSharedOutlineSegments, sharedBorderPaintPoints, type SharedOutlineSegment } from "../🔲️WindowFrame/shared-border.ts";
import { WINDOW_FRAME_PATH_INSET, type WindowFrameChipRegion, type WindowFrameGeometry } from "../🔲️WindowFrame/🟦️.tsx";
import { useShellScopeOptional } from "../🐚️ShellScope/🟦️.tsx";

export interface SharedBorderPaintSegment {
  readonly d: string;
  readonly state: BorderState;
  readonly disabled?: true;
}
interface Registration {
  element: HTMLElement;
  geometry: WindowFrameGeometry;
  state: BorderState;
  update: (segments: readonly SharedBorderPaintSegment[]) => void;
  painted?: string;
}
interface Frame {
  id: string;
  svg: SVGSVGElement;
  attached: string | null;
  position?: { value: string; priority: string };
  sized?: string;
}
interface Measure {
  readonly raw: DOMRect;
  readonly snapped: DOMRect;
  readonly offsetWidth: number;
  readonly offsetHeight: number;
}
const MARKER_ATTRIBUTES = new Set(["role", "tabindex", "data-shared-border-frame", "data-shared-border-edges", "data-shared-border-cells", "data-shared-border-cell", "data-shared-border-group", "data-shared-border-interactive", "data-shared-border-delegate"]);
const GEOMETRIC_ATTRIBUTES = new Set(["class", "style", ...MARKER_ATTRIBUTES]);
const implicitBorderless = '[data-slot="mode-dock-tab"]';
const selector = `[data-shared-border-frame], [data-shared-border-edges], ${implicitBorderless}`;
const groupSelector = "[data-shared-border-cells], [data-shared-border-group]";
const ownerSelector = `${selector}, ${groupSelector}`;
const cellSelector = ".shared-border-cell, [data-shared-border-cell]";
const bareFieldSelector = '[data-shared-border-cells] > :is(input:not([type="hidden"]), select, textarea)';
const interactiveSelector = 'button, a[href], input:not([type="hidden"]), select, textarea, summary, [contenteditable]:not([contenteditable="false"]), [role="button"], [role="link"], [role="tab"], [role="option"], [role="menuitem"], [role="menuitemcheckbox"], [role="menuitemradio"], [role="treeitem"], [role="switch"], [role="checkbox"], [role="radio"], [role="textbox"], [role="combobox"], [role="searchbox"], [role="slider"], [role="spinbutton"], [tabindex]:not([tabindex="-1"]), [data-shared-border-interactive]';
const activeSelector = '[data-active="true"], [data-selected="true"], [data-state="on"], [data-state="active"], [aria-selected="true"], [aria-pressed="true"]';
const invalidSelector = '[aria-invalid="true"], [data-invalid]:not([data-invalid="false"])';
const boxRelevantSelector = `${ownerSelector}, ${cellSelector}, ${bareFieldSelector}, [data-window-frame-chip], ${interactiveSelector}`;
const allSides: readonly BorderSide[] = ["top", "right", "bottom", "left"];
const namespace = "http://www.w3.org/2000/svg";
const xhtml = "http://www.w3.org/1999/xhtml";
const states: readonly BorderState[] = ["normal", "focus", "active", "invalid", "waiting", "loading", "introduced", "celebrated", "hover"];
const roots = new WeakMap<HTMLElement, { compositor: BorderCompositor; participants: number }>();
let sequence = 0;

/** 🧱️ Realm-independent element test: observer callbacks can outlive the window global that defined `Element`. */
const isElement = (node: unknown): node is Element => typeof node === "object" && node !== null && (node as Node).nodeType === 1;

/** 🕰️ Remove transition tokens whose corresponding live CSS transition has ended or disappeared. */
export function pruneSharedBorderMotion(motion: Map<Element, Set<string>>): void {
  for (const [element, properties] of motion) {
    if (typeof element.getAnimations !== "function") continue;
    const transitions = element.getAnimations().filter((animation): animation is CSSTransition =>
      "transitionProperty" in animation && animation.playState !== "finished" && animation.playState !== "idle");
    for (const property of properties) if (!transitions.some(animation => animation.transitionProperty === property)) properties.delete(property);
    if (!properties.size) motion.delete(element);
  }
}

/** 🛡️ Inline important geometry prevents ancestor icon selectors from shrinking frame paint. The overlay's origin is moved onto the
 * parent's device-snapped corner (the compositor resolves every line in snapped space), so two overlays whose parents lie at
 * different fractions of a pixel (cells of a fractional grid track) paint a shared seam at one coordinate. */
const sizedOverlays = new WeakMap<SVGSVGElement, string>();
export function sizeSharedBorderOverlay(svg: SVGSVGElement | null, width: number, height: number, measured?: DOMRect): void {
  if (!svg) return;
  const size = `${width}x${height}`;
  if (sizedOverlays.get(svg) !== size || svg.style.getPropertyValue("width") === "") {
    for (const [property, value] of Object.entries({ width: `${width}px`, height: `${height}px`, "min-width": "0", "min-height": "0", "max-width": "none", "max-height": "none", "flex-shrink": "0" })) svg.style.setProperty(property, value, "important");
    sizedOverlays.set(svg, size);
  }
  const parent = svg.parentElement;
  if (!parent) return;
  const rect = measured ?? parent.getBoundingClientRect(), snapped = snapRect(rect);
  const x = (snapped.left - rect.left) / sharedBorderAxisScale(rect.width, parent.offsetWidth), y = (snapped.top - rect.top) / sharedBorderAxisScale(rect.height, parent.offsetHeight);
  if (x || y) {
    const translate = `${Math.round(x * 1e3) / 1e3}px ${Math.round(y * 1e3) / 1e3}px`;
    if (svg.style.getPropertyValue("translate") !== translate) svg.style.setProperty("translate", translate, "important");
  } else if (svg.style.getPropertyValue("translate") !== "") svg.style.removeProperty("translate");
}

/** 🎨️ Shared state paint used by automatic frame overlays as well as window segments. */
export function sharedBorderStroke(state: BorderState): string {
  if (state === "hover") return "var(--hover-base)";
  if (state === "focus") return "var(--active-base)";
  if (state === "active") return "var(--active-base)";
  if (state === "invalid") return "var(--destructive-border)";
  if (state === "introduced") return "var(--introduced-border-color, var(--color-secondary))";
  if (state === "loading") return "var(--loading-border-color, var(--border-normal-color))";
  if (state === "waiting") return "var(--waiting-border-color, var(--border-normal-color))";
  return "var(--border-normal-color)";
}

/** 📌️ A client rect with every edge rounded to a whole device pixel, so frame strokes and the bars they meet land on the same physical pixel at fractional heights and zoom levels. */
export function deviceSnappedRect(element: Element): DOMRect {
  return snapRect(element.getBoundingClientRect());
}

function snapRect(rect: DOMRect): DOMRect {
  const ratio = (typeof window !== "undefined" && window.devicePixelRatio) || 1;
  const snap = (value: number) => Math.round(value * ratio) / ratio;
  const left = snap(rect.left), top = snap(rect.top), right = snap(rect.left + rect.width), bottom = snap(rect.top + rect.height);
  return { left, top, right, bottom, x: left, y: top, width: right - left, height: bottom - top, toJSON: () => ({}) } as DOMRect;
}

/** 🫥️ Props of a borderless box (a tab, tree row, menu item, icon button): no line in the normal state; on hover it emphasises its foreground and the lines of other boxes that lie on its perimeter and adds none; while a state that closes it (focus, active, invalid, …) holds it paints the full closed outline. Its overlay exists only while the box is stated. */
export const sharedBorderlessBoxProps = { "data-shared-border-frame": "", "data-shared-border-edges": "" } as const;

/** 📜️ A group that scrolls its content cannot hold an overlay that stays on the strip: its cells are painted by the next owner out. */
const scrollsContent = (element: HTMLElement): boolean => {
  const style = element.ownerDocument.defaultView?.getComputedStyle(element);
  return /(auto|scroll)/.test(`${style?.overflowX} ${style?.overflowY}`) || element.scrollWidth > element.clientWidth + 1 || element.scrollHeight > element.clientHeight + 1;
};

/** 🧩️ The owner whose overlay paints `element` as a cell: the framed parent of a `data-shared-border-cells` owner (a bare `input`, `select` or `textarea` child of it is a cell too: its row is the owner of its edges), or for an element carrying `.shared-border-cell` / `data-shared-border-cell` the nearest ancestor owner that is not itself a cell (frames that are cells are walked past, and so are plain wrappers). Cells paint their four edges in that overlay and register no frame of their own. */
export function sharedBorderCellOwner(element: Element): HTMLElement | null {
  const parent = element.parentElement;
  if (!parent) return null;
  if (parent.hasAttribute("data-shared-border-cells") && (element.matches(selector) || element.matches(bareFieldSelector))) return parent;
  if (!element.matches(cellSelector)) return null;
  let scroller: HTMLElement | null = null;
  for (let ancestor: HTMLElement | null = parent; ancestor && !ancestor.hasAttribute("data-window-frame"); ancestor = ancestor.parentElement) {
    if (!ancestor.matches(ownerSelector) || !(ancestor.matches(groupSelector) || sharedBorderCellOwner(ancestor) === null)) continue;
    if (ancestor.matches(selector) || !scrollsContent(ancestor)) return ancestor;
    scroller ??= ancestor;
  }
  return scroller;
}

/** 🧩️ A cell is an element with a {@link sharedBorderCellOwner}. */
export const isSharedBorderCell = (element: Element): boolean => sharedBorderCellOwner(element) !== null;

/** 🕳️ Void and replaced elements cannot hold the overlay `svg`: a box of this kind is painted by the nearest ancestor that can. */
export const holdsNoOverlay = (element: Element): boolean => /^(input|select|textarea|img|canvas|video|audio|iframe|object|embed|br|hr|svg)$/i.test(element.localName) || element.namespaceURI !== "http://www.w3.org/1999/xhtml";

/** 🏠️ A host paints an overlay: every frame, every cells owner and group, except a plain cell. */
export function isSharedBorderHost(element: Element): boolean {
  return element.matches(ownerSelector) && !holdsNoOverlay(element) && (!isSharedBorderCell(element) || element.matches(groupSelector));
}

/** ⌨️ Realm-safe `:focus-visible` test; engines without the pseudo-class never paint focus. */
export function matchesFocusVisible(element: Element): boolean {
  try {
    return element.matches(":focus-visible");
  } catch {
    return false;
  }
}

/** 🧲️ A chip edge within 2 px of a line of the window's own outline snaps onto it, so a stated chip never sits beside the seam it recolours. */
const snapToLine = (value: number, lines: readonly number[]): number => {
  let best = value, reach = 2 + 1e-6;
  for (const line of lines) if (Math.abs(line - value) < reach) { best = line; reach = Math.abs(line - value); }
  return best;
};

/** 🔲️ The logical corners of a frame rectangle and the corners of its inset paint path, clockwise from the top left.
 *
 * @see ../🔲️WindowFrame/frame.cases.json */
export function windowFrameOutline(metrics: { readonly width: number; readonly height: number }): { readonly logical: readonly BorderPoint[]; readonly paint: readonly BorderPoint[] } {
  const { width, height } = metrics, inset = Math.min(WINDOW_FRAME_PATH_INSET, width / 2, height / 2);
  return {
    logical: [{ x: 0, y: 0 }, { x: width, y: 0 }, { x: width, y: height }, { x: 0, y: height }],
    paint: [{ x: inset, y: inset }, { x: width - inset, y: inset }, { x: width - inset, y: height - inset }, { x: inset, y: height - inset }],
  };
}

/** 📏️ The vertical and horizontal paint lines a stated chip snaps onto: the four frame edges and the inner edges of each chip region. */
export function windowFrameLines(paint: readonly BorderPoint[], regions: readonly WindowFrameChipRegion[]): { readonly vertical: readonly { x: number; top: number; bottom: number }[]; readonly ys: readonly number[] } {
  const [topLeft, , bottomRight] = paint as readonly [BorderPoint, BorderPoint, BorderPoint, BorderPoint];
  const vertical = [{ x: topLeft.x, top: topLeft.y, bottom: bottomRight.y }, { x: bottomRight.x, top: topLeft.y, bottom: bottomRight.y }];
  const ys = [topLeft.y, bottomRight.y];
  for (const region of regions) {
    const inset = Math.min(WINDOW_FRAME_PATH_INSET, region.width / 2, region.height / 2);
    const top = region.y + inset, bottom = region.y + region.height - inset;
    vertical.push({ x: region.x + inset, top, bottom }, { x: region.x + region.width - inset, top, bottom });
    ys.push(region.dock === "top" ? bottom : top);
  }
  return { vertical, ys };
}

/** 🪟️ A stated box stays fully visible: a side that overshoots a clipping ancestor by more than a pixel moves onto the clip edge, while a clip coinciding with the box edge within a pixel does not clip it at all. Returns null when nothing of the box is visible. */
function clampToClip(rect: { left: number; top: number; right: number; bottom: number }, widened: SharedBorderClip): { left: number; top: number; right: number; bottom: number } | null {
  const clip = { left: widened.left + 0.5, top: widened.top + 0.5, right: widened.right - 0.5, bottom: widened.bottom - 0.5 };
  const outside = (low: number, high: number, from: number, to: number) => high > low ? high <= from || low >= to : low < from || low > to;
  if (outside(rect.left, rect.right, clip.left, clip.right) || outside(rect.top, rect.bottom, clip.top, clip.bottom)) return null;
  return {
    left: clip.left - rect.left > 1 ? clip.left : rect.left, top: clip.top - rect.top > 1 ? clip.top : rect.top,
    right: rect.right - clip.right > 1 ? clip.right : rect.right, bottom: rect.bottom - clip.bottom > 1 ? clip.bottom : rect.bottom,
  };
}

interface Box {
  readonly id: string;
  readonly element: HTMLElement;
  readonly kind: "window" | "chip" | "frame" | "group" | "cell";
  readonly paint: HTMLElement | null;
  readonly entry?: Registration;
  parent: Box | null;
}
const edgesOf = (element: Element): string => element.getAttribute("data-shared-border-edges") ?? (element.matches(implicitBorderless) ? "" : "top right bottom left");
const boxIds = new WeakMap<Element, string>();
const boxId = (element: Element): string => {
  let id = boxIds.get(element);
  if (!id) { id = `box-${++sequence}`; boxIds.set(element, id); }
  return id;
};

/** 🏷️ State markers carried by the box element or by its single interactive child. */
function marksOf(holders: readonly (Element | null)[]): BorderState[] {
  const marks: BorderState[] = [];
  for (const holder of holders) {
    if (!holder) continue;
    const explicit = holder.getAttribute("data-shared-border-state") as BorderState | null;
    if (explicit && explicit !== "normal" && states.includes(explicit)) marks.push(explicit);
    for (const [attribute, state] of [["data-celebrated", "celebrated"], ["data-introduced", "introduced"]] as const) if (holder.hasAttribute(attribute) && holder.getAttribute(attribute) !== "false") marks.push(state);
    if (holder.classList.contains("border-loading")) marks.push("loading");
    if (holder.classList.contains("border-waiting")) marks.push("waiting");
    if (holder.matches(activeSelector)) marks.push("active");
    if (holder.matches(invalidSelector)) marks.push("invalid");
  }
  return marks;
}

/** 🪡️ Root-local resolver; original owners paint only winning portions of their unchanged outline. A line shared with an owner inside an elevation root (`data-elevation-root`: a band that lies above the content) is always painted by that owner, in whatever state won it: painted from beneath, half of the line would lie under the band, so it would look thinner and change with every hover. */
export class BorderCompositor {
  readonly windows = new Map<string, Registration>();
  private frames = new Map<HTMLElement, Frame>();
  private frame = 0;
  private disposed = false;
  private observer: MutationObserver | null = null;
  private resize: ResizeObserver | null = null;
  private hovered: Element | null = null;
  private hoveredBox: Element | null = null;
  private boxElements = new Set<Element>();
  private tracked: Set<Element> | null = null;
  private watched = new Set<Element>();
  private sizes = new WeakMap<Element, string>();
  private candidates: { readonly owners: HTMLElement[]; readonly frames: HTMLElement[]; readonly chips: HTMLElement[]; readonly cells: HTMLElement[] } | null = null;
  /** 🔢️ Number of commits so far; a commit re-measures every box. */
  commits = 0;
  private delegates = new Map<Element, HTMLElement | null>();
  private motion = new Map<Element, Set<string>>();
  constructor(readonly root: HTMLElement) {
    root.setAttribute("data-shared-border-root", "");
    const view = root.ownerDocument.defaultView;
    this.observer = typeof MutationObserver === "undefined" ? null : new MutationObserver(records => {
      const tracked = this.tracked;
      let changed = false, geometric = false, structural = false;
      for (const record of records) {
        const target = record.target;
        if (!isElement(target)) continue;
        if (record.type === "childList") {
          let touched = false, any = false;
          for (const list of [record.addedNodes, record.removedNodes]) for (const node of list) {
            if (isElement(node) ? node.hasAttribute("data-shared-border-overlay") : false) continue;
            any = true;
            if (isElement(node) && (node.matches(boxRelevantSelector) || (node.namespaceURI === xhtml && node.querySelector(boxRelevantSelector) !== null))) touched = true;
          }
          if (!any || (!touched && tracked !== null && !tracked.has(target)) || target.closest("[data-shared-border-overlay], [data-window-frame-border]")) continue;
          changed = geometric = true;
          if (touched) structural = true;
        } else {
          const name = record.attributeName!;
          if (tracked !== null && !tracked.has(target) && !MARKER_ATTRIBUTES.has(name) && !(name === "class" && target.matches(cellSelector))) continue;
          if (target.closest("[data-shared-border-overlay], [data-window-frame-border]")) continue;
          changed = true;
          if (GEOMETRIC_ATTRIBUTES.has(name)) { geometric = true; if (name !== "style") structural = true; }
        }
      }
      if (structural) this.schedule();
      else if (geometric) this.scheduleGeometry();
      else if (changed) this.scheduleState();
    });
    this.observer?.observe(root, { subtree: true, childList: true, attributes: true, attributeFilter: [...GEOMETRIC_ATTRIBUTES, "data-active", "data-selected", "data-state", "aria-selected", "aria-pressed", "aria-invalid", "data-invalid", "data-shared-border-state", "data-shared-border-hover", "data-introduced", "data-celebrated", "disabled", "aria-disabled"] });
    this.resize = typeof ResizeObserver === "undefined" ? null : new ResizeObserver(entries => {
      let moved = false;
      for (const entry of entries) {
        const size = `${Math.round(entry.contentRect.width * 100)}x${Math.round(entry.contentRect.height * 100)}`;
        if (this.sizes.get(entry.target) !== size) { moved = this.sizes.has(entry.target) || moved; this.sizes.set(entry.target, size); }
      }
      if (moved) this.scheduleGeometry();
    });
    this.resize?.observe(root);
    root.addEventListener("pointerover", this.pointer);
    root.addEventListener("pointerout", this.pointer);
    root.addEventListener("focusin", this.scheduleState);
    root.addEventListener("focusout", this.scheduleState);
    root.addEventListener("keydown", this.scheduleState, true);
    root.addEventListener("pointerdown", this.scheduleState, true);
    root.addEventListener("transitionrun", this.motionStart);
    root.addEventListener("transitionend", this.motionEnd);
    root.addEventListener("transitioncancel", this.motionEnd);
    view?.addEventListener("resize", this.scheduleGeometry);
    view?.addEventListener("scroll", this.scheduleGeometry, true);
  }
  /** 🪆️ A nested `.semio-scope` belongs to its own compositor: neither pointer nor focus of its content states a box of this one. */
  private nested = (element: Element): boolean => element !== this.root && element.classList.contains("semio-scope");
  private pointer = (event: Event) => {
    const pointer = event as PointerEvent;
    const next = (event.type === "pointerout" ? pointer.relatedTarget : pointer.target) as Element | null;
    const target = isElement(next) ? next : null;
    let box: Element | null = null;
    for (let element = target; element && !box && !this.nested(element); element = element.parentElement) if (this.boxElements.has(element)) box = element;
    const changed = box !== this.hoveredBox;
    this.hovered = target;
    this.hoveredBox = box;
    if (changed) this.scheduleState();
  };
  private motionStart = (event: Event) => {
    const property = (event as TransitionEvent).propertyName;
    if (!(isElement(event.target)) || !/^(transform|translate|scale|width|height|top|left|right|bottom|flex-basis|grid-template-)/.test(property)) return;
    const properties = this.motion.get(event.target) ?? new Set<string>();
    properties.add(property); this.motion.set(event.target, properties); this.scheduleGeometry();
  };
  private motionEnd = (event: Event) => {
    if (!(isElement(event.target))) return;
    const properties = this.motion.get(event.target);
    if (!properties?.delete((event as TransitionEvent).propertyName)) return;
    if (!properties.size) this.motion.delete(event.target);
    this.scheduleGeometry();
  };
  /** 🔭️ Geometry may have changed: the next frame measures again. */
  schedule = () => {
    this.delegates.clear();
    this.candidates = null;
    this.watched.clear();
    this.scheduleGeometry();
  };
  /** 🧭️ Frames moved or resized but none was added, removed or reclassified (a pan, a scroll, a resize): measure again, keep what is known of the structure. */
  scheduleGeometry = () => {
    this.geometryDirty = true;
    this.queue();
  };
  /** 🎨️ Only a state changed (hover, focus, an active or pressed marker): the next frame repaints from the geometry measured last. */
  scheduleState = () => {
    this.geometryDirty = true;
    this.queue();
  };
  private geometryDirty = true;
  private measured = new Map<Element, Measure>();
  private measure(element: Element): Measure {
    const known = this.measured.get(element);
    if (known) return known;
    const html = element as HTMLElement;
    const raw = element.getBoundingClientRect();
    const measure = { raw, snapped: snapRect(raw), offsetWidth: html.offsetWidth, offsetHeight: html.offsetHeight };
    this.measured.set(element, measure);
    this.resize?.observe(element);
    return measure;
  }
  private queue() {
    if (this.disposed || this.frame) return;
    if (!this.root.hasAttribute("data-shared-border-pending")) this.root.setAttribute("data-shared-border-pending", "");
    const view = this.root.ownerDocument.defaultView;
    if (view?.requestAnimationFrame) this.frame = view.requestAnimationFrame(this.commit);
    else this.commit();
  };
  /** 🎯️ The element whose interactivity and markers a box carries: itself when interactive, else its only interactive descendant when the box holds no other box; a window inside an interactive overview card carries the card. */
  private delegate(box: Box, childless: boolean): HTMLElement | null {
    const known = this.delegates.get(box.element);
    if (known !== undefined) return known;
    let found: HTMLElement | null = null;
    if (box.element.matches(interactiveSelector)) found = box.element;
    else if (box.element.matches(implicitBorderless) && box.element.querySelector('[role="tab"]')) found = box.element;
    else if (box.kind === "window") { const card = box.element.closest<HTMLElement>("[data-overview-card]"); found = card?.matches(interactiveSelector) ? card : null; }
    else if (childless && box.element.getAttribute("data-shared-border-delegate") !== "self") { const candidates = box.element.querySelectorAll<HTMLElement>(interactiveSelector); found = candidates.length === 1 ? candidates[0]! : null; }
    this.delegates.set(box.element, found);
    return found;
  }
  private clips = new Map<Element, { readonly clipsX: boolean; readonly clipsY: boolean; readonly left: number; readonly top: number; readonly right: number; readonly bottom: number } | null>();
  private clipOf(ancestor: Element) {
    if (this.clips.has(ancestor)) return this.clips.get(ancestor)!;
    const html = ancestor as HTMLElement;
    const style = ancestor.ownerDocument.defaultView?.getComputedStyle(ancestor);
    const clipsX = /^(hidden|clip|scroll|auto)$/.test(style?.overflowX || style?.overflow || "");
    const clipsY = /^(hidden|clip|scroll|auto)$/.test(style?.overflowY || style?.overflow || "");
    let clip = null;
    if (clipsX || clipsY) {
      this.resize?.observe(ancestor);
      const rect = ancestor.getBoundingClientRect();
      const scaleX = html.offsetWidth ? rect.width / html.offsetWidth : 1;
      const scaleY = html.offsetHeight ? rect.height / html.offsetHeight : 1;
      const left = rect.left + ancestor.clientLeft * scaleX, top = rect.top + ancestor.clientTop * scaleY;
      clip = { clipsX, clipsY, left, top, right: left + (html.offsetWidth ? ancestor.clientWidth * scaleX : rect.width), bottom: top + (html.offsetHeight ? ancestor.clientHeight * scaleY : rect.height) };
    }
    this.clips.set(ancestor, clip);
    return clip;
  }
  /** ✂️ The visible window of `element`: every clipping ancestor except `skip`, widened by half a pixel so a clip that lands on the box edge by rounding never cuts the line lying on it. */
  private clipRectOf(element: HTMLElement, skip?: Element): SharedBorderClip {
    const clip = { left: -Infinity, top: -Infinity, right: Infinity, bottom: Infinity };
    for (let ancestor = element.parentElement; ancestor; ancestor = ancestor.parentElement) {
      const bounds = ancestor === skip ? null : this.clipOf(ancestor);
      if (bounds === null) continue;
      if (bounds.clipsX) { clip.left = Math.max(clip.left, bounds.left); clip.right = Math.min(clip.right, bounds.right); }
      if (bounds.clipsY) { clip.top = Math.max(clip.top, bounds.top); clip.bottom = Math.min(clip.bottom, bounds.bottom); }
    }
    return { left: clip.left - 0.5, top: clip.top - 0.5, right: clip.right + 0.5, bottom: clip.bottom + 0.5 };
  }
  private visibleSources(element: HTMLElement, sources: SharedOutlineSegment[], skip?: Element): SharedOutlineSegment[] {
    const clip = this.clipRectOf(element, skip);
    return clipSharedOutlineSegments(sources, clip).map(source => ({ ...source, clip }));
  }
  private restore(element: HTMLElement, frame: Frame) {
    frame.svg.remove();
    if (frame.position && element.style.position === "relative") {
      if (frame.position.value) element.style.setProperty("position", frame.position.value, frame.position.priority);
      else element.style.removeProperty("position");
    }
    if (frame.attached === null) element.removeAttribute("data-shared-border-attached");
    else element.setAttribute("data-shared-border-attached", frame.attached);
    this.resize?.unobserve(element);
    this.watched.delete(element);
    this.frames.delete(element);
  }
  /** 📐️ The range a line of `owner` can take and still be seen: an owner that clips its own overflow cuts the half of a seam centred on its edge, so such a line is held one frame inset inside the clip. An owner that does not clip keeps the seam centred. */
  private insideOwner(owner: string, windows: ReadonlyMap<string, Registration>, frames: ReadonlyMap<string, { element: HTMLElement }>, horizontal: boolean): { readonly low: number; readonly high: number } | null {
    const element = windows.get(owner)?.element ?? frames.get(owner)?.element;
    const clip = element ? this.clipOf(element) : null;
    if (!clip || !(horizontal ? clip.clipsY : clip.clipsX)) return null;
    const [low, high] = horizontal ? [clip.top, clip.bottom] : [clip.left, clip.right];
    return high - low > 2 * WINDOW_FRAME_PATH_INSET ? { low: low + WINDOW_FRAME_PATH_INSET, high: high - WINDOW_FRAME_PATH_INSET } : null;
  }
  private commit = () => {
    this.frame = 0;
    if (this.disposed) return;
    this.commits++;
    const fresh = this.geometryDirty;
    this.geometryDirty = false;
    if (fresh) { this.measured.clear(); this.clips.clear(); }
    let sources: SharedOutlineSegment[] = [];
    const updates = new Map<Registration, readonly SharedBorderPaintSegment[]>();
    const frameSources = new Map<string, { element: HTMLElement; frame: Frame; width: number; height: number; sides: string | null }>();
    const scope = this.root.closest(".semio-scope");
    const found = this.candidates ??= { owners: [...this.root.querySelectorAll<HTMLElement>(ownerSelector)], frames: [...this.root.querySelectorAll<HTMLElement>(selector)], chips: [...this.root.querySelectorAll<HTMLElement>("[data-window-frame-chip]")], cells: [...this.root.querySelectorAll<HTMLElement>(`${cellSelector}, ${selector}, ${bareFieldSelector}`)] };
    const hosts = found.owners.filter(element => !element.hasAttribute("data-window-frame") && element.closest(".semio-scope") === scope && isSharedBorderHost(element));
    const hostSet = new Set<Element>(hosts);
    const replaced = new Map<HTMLElement, HTMLElement>();
    for (const element of found.frames) {
      if (!holdsNoOverlay(element) || element.closest(".semio-scope") !== scope || sharedBorderCellOwner(element)) continue;
      let owner = element.parentElement;
      while (owner && holdsNoOverlay(owner)) owner = owner.parentElement;
      if (!owner) continue;
      replaced.set(element, owner);
      if (!hostSet.has(owner)) { hostSet.add(owner); hosts.push(owner); }
    }
    for (const [element, frame] of this.frames) if (!this.root.contains(element) || !hostSet.has(element)) this.restore(element, frame);
    const boxes: Box[] = [];
    const byElement = new Map<Element, Box>();
    const put = (box: Box) => { if (!byElement.has(box.element)) { boxes.push(box); byElement.set(box.element, box); } };
    for (const [id, entry] of this.windows) put({ id, element: entry.element, kind: "window", paint: null, entry, parent: null });
    const windowElements = new Set<Element>([...this.windows.values()].map(entry => entry.element));
    for (const chip of found.chips) {
      let window: Element | null = chip.parentElement;
      while (window && !windowElements.has(window)) window = window.parentElement;
      if (window) put({ id: boxId(chip), element: chip, kind: "chip", paint: window as HTMLElement, parent: null });
    }
    for (const host of hosts) {
      const cell = sharedBorderCellOwner(host);
      put({ id: boxId(host), element: host, kind: cell ? "cell" : host.matches(selector) ? "frame" : "group", paint: cell ? (hostSet.has(cell) ? cell : null) : host, parent: null });
    }
    for (const [element, owner] of replaced) put({ id: boxId(element), element, kind: "frame", paint: owner, parent: null });
    for (const element of found.cells) {
      const owner = sharedBorderCellOwner(element);
      if (owner && hostSet.has(owner) && !element.hasAttribute("data-window-frame") && element.closest(".semio-scope") === scope) put({ id: boxId(element), element, kind: "cell", paint: owner, parent: null });
    }
    const parents = new Set<Box | null>();
    for (const box of boxes) {
      for (let element = box.element.parentElement; element && !box.parent; element = element.parentElement) box.parent = byElement.get(element) ?? null;
      parents.add(box.parent);
    }
    this.boxElements = new Set(byElement.keys());
    const disabledBoxes = new Set<string>();
    const stateBoxes: StateBox[] = boxes.map(box => {
      const holder = this.delegate(box, !parents.has(box)), target = holder ?? box.element;
      const marks = marksOf([box.element, holder]);
      if (box.entry && box.entry.state !== "normal") marks.push(box.entry.state);
      const disabled = target.matches(":disabled") || target.closest('[aria-disabled="true"]') !== null;
      if (disabled) disabledBoxes.add(box.id);
      return { id: box.id, parent: box.parent?.id ?? null, interactive: holder !== null || (box.kind === "chip" && box.element.querySelector(interactiveSelector) !== null), hoverWithin: box.element.hasAttribute("data-shared-border-hover"), disabled, marks };
    });
    const tracked = new Set<Element>();
    for (const box of boxes) for (let element: Element | null = box.element; element && !tracked.has(element); element = element.parentElement) tracked.add(element);
    for (const holder of this.delegates.values()) if (holder) tracked.add(holder);
    this.tracked = tracked;
    for (const box of boxes) {
      const parent = box.element.parentElement;
      if (!parent || this.watched.has(parent)) continue;
      this.watched.add(parent);
      this.resize?.observe(parent);
    }
    const boxOf = (start: Element | null): string | null => {
      for (let element = start; element && !this.nested(element); element = element.parentElement) { const box = byElement.get(element); if (box) return box.id; }
      return null;
    };
    const active = this.root.ownerDocument.activeElement;
    const states = resolveBoxStates(stateBoxes, boxOf(this.hovered), active && this.root.contains(active) && matchesFocusVisible(active) ? boxOf(active) : null);
    const xs: number[] = [], ys: number[] = [];
    for (const box of boxes) {
      const rect = this.measure(box.element).snapped;
      if (rect.width > 0 || rect.height > 0) { xs.push(rect.left, rect.right); ys.push(rect.top, rect.bottom); }
    }
    const snap = createSharedBorderSnap(xs, ys);
    const windowTransforms = new Map<Element, { id: string; vertical: readonly { x: number; top: number; bottom: number }[]; ys: readonly number[] } & SharedBorderTransform>();
    for (const [id, entry] of this.windows) {
      const { snapped: rect, offsetWidth, offsetHeight } = this.measure(entry.element);
      if (rect.width <= 0 || rect.height <= 0 || entry.geometry.state !== "ready") { updates.set(entry, []); continue; }
      const metrics = entry.geometry.metrics;
      const { logical, paint } = windowFrameOutline(metrics), regions = entry.geometry.chipRegions;
      const lines = windowFrameLines(paint, regions);
      windowTransforms.set(entry.element, { id, vertical: lines.vertical, ys: lines.ys, left: rect.left, top: rect.top, scaleX: sharedBorderAxisScale(rect.width, offsetWidth), scaleY: sharedBorderAxisScale(rect.height, offsetHeight) });
      sources.push(...this.visibleSources(entry.element, sharedOutlineSegments(id, logical, paint, states[id]!, { left: rect.left, top: rect.top, scaleX: sharedBorderAxisScale(rect.width, offsetWidth), scaleY: sharedBorderAxisScale(rect.height, offsetHeight) }, snap, id, regions)));
    }
    const declared = (box: Box): readonly BorderSide[] => box.kind === "group" ? [] : box.kind === "frame" ? edgesOf(box.element).split(/\s+/).filter((side): side is BorderSide => allSides.includes(side as BorderSide)) : allSides;
    const cellHosts = new Set<Element>();
    for (const box of boxes) if (box.paint && (box.kind === "cell" || (box.element !== box.paint && (declared(box).length > 0 || states[box.id] !== "normal")))) cellHosts.add(box.paint);
    const geometry = new Map<HTMLElement, { rect: DOMRect; scaleX: number; scaleY: number; width: number; height: number }>();
    for (const element of hosts) {
      const own = byElement.get(element);
      if (own?.kind === "window") continue;
      if (!cellHosts.has(element) && (!own || own.kind === "cell" || (declared(own).length === 0 && states[own.id] === "normal"))) { const idle = this.frames.get(element); if (idle) this.restore(element, idle); continue; }
      const { snapped: rect, offsetWidth, offsetHeight } = this.measure(element);
      const scaleX = sharedBorderAxisScale(rect.width, offsetWidth), scaleY = sharedBorderAxisScale(rect.height, offsetHeight);
      const width = rect.width / scaleX, height = rect.height / scaleY;
      if ((rect.width <= 0 && rect.height <= 0) || ((width <= 2 || height <= 2) && !(own && states[own.id] !== "normal"))) { const old = this.frames.get(element); if (old) this.restore(element, old); continue; }
      let frame = this.frames.get(element);
      if (!frame) {
        const svg = element.ownerDocument.createElementNS(namespace, "svg");
        svg.setAttribute("data-shared-border-overlay", "");
        svg.setAttribute("aria-hidden", "true");
        svg.style.cssText = "position:absolute;inset:0;overflow:visible;pointer-events:none;z-index:40";
        frame = { id: boxId(element), svg, attached: element.getAttribute("data-shared-border-attached") };
        const position = element.ownerDocument.defaultView?.getComputedStyle(element).position;
        if (!position || position === "static") {
          frame.position = { value: element.style.getPropertyValue("position"), priority: element.style.getPropertyPriority("position") };
          element.style.position = "relative";
        }
        this.frames.set(element, frame);
        this.resize?.observe(element);
      }
      if (frame.svg.parentElement !== element) element.append(frame.svg);
      geometry.set(element, { rect, scaleX, scaleY, width, height });
      frameSources.set(frame.id, { element, frame, width, height, sides: own?.kind === "frame" && !element.matches(implicitBorderless) ? edgesOf(element) : null });
    }
    const flushParent = (box: Box): Box | null => {
      if (box.kind === "cell") return box.paint ? byElement.get(box.paint) ?? null : null;
      let parent = box.parent;
      while (parent && parent.kind !== "frame" && parent.kind !== "window") parent = parent.parent;
      return parent;
    };
    for (const box of boxes) {
      if (box.kind !== "chip") continue;
      const window = box.paint && windowTransforms.get(box.paint), state = states[box.id]!, rect = this.measure(box.element).snapped;
      if (!box.paint || !window || state === "normal" || rect.width <= 2 || rect.height <= 2) continue;
      const visible = clampToClip(rect, this.clipRectOf(box.element, box.paint));
      if (!visible) continue;
      const left = (visible.left - window.left) / window.scaleX, top = (visible.top - window.top) / window.scaleY, right = (visible.right - window.left) / window.scaleX, bottom = (visible.bottom - window.top) / window.scaleY;
      const logical = [{ x: left, y: top }, { x: right, y: top }, { x: right, y: bottom }, { x: left, y: bottom }];
      const xs = window.vertical.filter(line => line.top < bottom && line.bottom > top).map(line => line.x);
      const [l, r] = [snapToLine(left + 1, xs), snapToLine(right - 1, xs)], [t, b] = [snapToLine(top + 1, window.ys), snapToLine(bottom - 1, window.ys)];
      const paint = [{ x: l, y: t }, { x: r, y: t }, { x: r, y: b }, { x: l, y: b }];
      sources.push(...sharedOutlineSegments(window.id, logical, paint, state, window, snap, box.id).map(edge => ({ ...edge, follows: true as const })));
    }
    const emphasis = new Set<string>();
    const borderOf = (element: Element, side: BorderSide): number => parseFloat(element.ownerDocument.defaultView?.getComputedStyle(element).getPropertyValue(`border-${side}-width`) ?? "") || 0;
    const flushRect = (box: Box, rect: { left: number; top: number; right: number; bottom: number }) => {
      const out = { left: rect.left, top: rect.top, right: rect.right, bottom: rect.bottom };
      for (const side of allSides) for (let parent = box.parent; parent; parent = parent.parent) {
        const outer = this.measure(parent.element).snapped;
        if (Math.abs(outer[side] - out[side]) > borderOf(parent.element, side) + 0.75) break;
        out[side] = outer[side];
      }
      return out;
    };
    const windowOf = (box: Box) => {
      for (let parent = box.parent; parent; parent = parent.parent) if (parent.kind === "chip" && parent.paint) return windowTransforms.get(parent.paint) ?? null;
      return null;
    };
    const restDrawn = new Map<Box, readonly BorderSide[]>();
    const restSides = (box: Box): readonly BorderSide[] => {
      const known = restDrawn.get(box);
      if (known) return known;
      const outer = flushParent(box);
      const own = { id: box.id, parent: outer?.id ?? null, owner: box.id, rect: flushRect(box, this.measure(box.element).snapped), edges: declared(box), state: "normal" } as const;
      const parent = outer ? { id: outer.id, parent: null, owner: outer.id, rect: this.measure(outer.element).snapped, edges: restSides(outer), state: "normal" } as const : undefined;
      const sides = boxDrawnSides(own, parent);
      restDrawn.set(box, sides);
      return sides;
    };
    for (const box of boxes) {
      const geo = box.paint && geometry.get(box.paint), frame = box.paint && this.frames.get(box.paint);
      if (!box.paint || !geo || !frame) continue;
      const state = states[box.id]!, measured = flushRect(box, this.measure(box.element).snapped);
      const rect = state === "normal" ? measured : clampToClip(measured, this.clipRectOf(box.kind === "cell" ? box.element : box.paint, box.paint));
      if (!rect) continue;
      const left = (rect.left - geo.rect.left) / geo.scaleX, top = (rect.top - geo.rect.top) / geo.scaleY;
      const width = (rect.right - rect.left) / geo.scaleX, height = (rect.bottom - rect.top) / geo.scaleY;
      if ((width <= 0 && height <= 0) || ((width <= 2 || height <= 2) && state === "normal")) continue;
      const inset = { x: Math.min(1, width / 2), y: Math.min(1, height / 2) };
      const right = left + width, bottom = top + height;
      const logical = [{ x: left, y: top }, { x: right, y: top }, { x: right, y: bottom }, { x: left, y: bottom }];
      const paint = [{ x: left + inset.x, y: top + inset.y }, { x: right - inset.x, y: top + inset.y }, { x: right - inset.x, y: bottom - inset.y }, { x: left + inset.x, y: bottom - inset.y }];
      const outer = flushParent(box);
      const normalBox = { id: box.id, parent: outer?.id ?? null, owner: frame.id, rect, edges: declared(box), state: "normal" } as const;
      const parentBox = outer ? { id: outer.id, parent: null, owner: outer.id, rect: this.measure(outer.element).snapped, edges: declared(outer), state: "normal" } as const : undefined;
      const normalSides = boxDrawnSides(normalBox, parentBox);
      const drawn = state === "normal" ? normalSides : allSides;
      const chipWindow = windowOf(box);
      if (chipWindow) {
        const top = (rect.top - chipWindow.top) / chipWindow.scaleY;
        const bottom = (rect.bottom - chipWindow.top) / chipWindow.scaleY;
        const xs = chipWindow.vertical.filter(line => line.top < bottom && line.bottom > top).map(line => line.x);
        for (const point of paint) {
          point.x = (chipWindow.left + snapToLine((geo.rect.left + point.x * geo.scaleX - chipWindow.left) / chipWindow.scaleX, xs) * chipWindow.scaleX - geo.rect.left) / geo.scaleX;
          point.y = (chipWindow.top + snapToLine((geo.rect.top + point.y * geo.scaleY - chipWindow.top) / chipWindow.scaleY, chipWindow.ys) * chipWindow.scaleY - geo.rect.top) / geo.scaleY;
        }
      }
      const edges = sharedOutlineSegments(frame.id, logical, paint, state, { left: geo.rect.left, top: geo.rect.top, scaleX: geo.scaleX, scaleY: geo.scaleY }, snap, box.id);
      const kept = edges.flatMap(edge => {
        const side = allSides[Number(edge.id.slice(edge.id.lastIndexOf(":") + 1))]!;
        if (!drawn.includes(side)) return [];
        if (state === "hover" && !restSides(box).includes(side)) emphasis.add(edge.id);
        return [chipWindow || !normalSides.includes(side) ? { ...edge, follows: true as const } : edge];
      });
      sources.push(...(state === "normal" ? this.visibleSources(box.kind === "cell" ? box.element : box.paint, kept, box.paint) : kept));
    }
    sources = visibleSharedOutlineSegments(sources);
    const byId = new Map(sources.map(source => [source.id, source]));
    const output = new Map<string, SharedBorderPaintSegment[]>();
    const raised = new Map<string, boolean>();
    for (const [id, entry] of this.windows) raised.set(id, entry.element.closest("[data-elevation-root]") !== null);
    for (const [element, frame] of this.frames) raised.set(frame.id, element.closest("[data-elevation-root]") !== null);
    const windowsById = this.windows;
    const resolved = withoutLoneEmphasis(resolveBorderSegments(sources), id => emphasis.has(id.split(":", 2).join(":"))).map(segment => {
      if (raised.get(segment.owner)) return segment;
      const upper = segment.contributors.map(id => byId.get(id)!).find(source => raised.get(source.owner));
      return upper ? { ...segment, id: upper.id, owner: upper.owner } : segment;
    });
    const junction = createSharedBorderJunctionResolver(sources);
    const interacting = new Set<string>();
    for (const source of sources) {
      const start = junction(source.logicalFrom), end = junction(source.logicalTo), t = source.transform;
      if (start.x !== t.left + source.paintFrom.x * t.scaleX || start.y !== t.top + source.paintFrom.y * t.scaleY || end.x !== t.left + source.paintTo.x * t.scaleX || end.y !== t.top + source.paintTo.y * t.scaleY) interacting.add(source.owner);
    }
    const lineOf = new Map<string, number>();
    const leadingOf = (ids: readonly string[]): readonly string[] => leadingSharedOutlineSegments(ids.map(id => byId.get(id)!)).map(edge => edge.id);
    const normalValue = (id: string, horizontal: boolean): number => { const edge = byId.get(id)!, t = edge.transform; return (horizontal ? t.top : t.left) + edge.paintFrom[horizontal ? "y" : "x"] * (horizontal ? t.scaleY : t.scaleX); };
    for (const segment of resolved) {
      const lead = leadingOf(segment.contributors);
      if (lead.length < 2) continue;
      const horizontal = segment.from.y === segment.to.y, values = lead.map(id => normalValue(id, horizontal));
      const line = (Math.min(...values) + Math.max(...values)) / 2;
      if (Math.abs(line - (horizontal ? segment.from.y : segment.from.x)) > 0.01) continue;
      for (const id of lead) if (!lineOf.has(`${id}:${horizontal}`)) lineOf.set(`${id}:${horizontal}`, line);
    }
    for (const segment of resolved) {
      const source = byId.get(segment.id)!;
      const local = (paint: { x: number; y: number }) => {
        const t = source.transform;
        return { x: (paint.x - t.left) / t.scaleX, y: (paint.y - t.top) / t.scaleY };
      };
      const horizontal = segment.from.y === segment.to.y, normal = horizontal ? "y" : "x";
      const lead = leadingOf(segment.contributors);
      const values = lead.map(id => {
        const edge = byId.get(id)!, t = edge.transform;
        return (horizontal ? t.top : t.left) + edge.paintFrom[normal] * (horizontal ? t.scaleY : t.scaleX);
      });
      const centerline = lead.length === 1 ? lineOf.get(`${lead[0]}:${horizontal}`) ?? (Math.min(...values) + Math.max(...values)) / 2 : (Math.min(...values) + Math.max(...values)) / 2;
      const room = this.insideOwner(segment.owner, windowsById, frameSources, horizontal);
      const line = room ? Math.min(Math.max(centerline, room.low), room.high) : centerline;
      const joined = junction(segment.from), joinedTo = junction(segment.to);
      const from = line === centerline ? joined : { ...joined, [normal]: line }, to = line === centerline ? joinedTo : { ...joinedTo, [normal]: line };
      const originalNormal = (horizontal ? source.transform.top : source.transform.left) + source.paintFrom[normal] * (horizontal ? source.transform.scaleY : source.transform.scaleX);
      if (line !== originalNormal || from[normal] !== line || to[normal] !== line) interacting.add(segment.owner);
      const points = sharedBorderPaintPoints(from, to, normal, line);
      if (points.length === 0) { interacting.add(segment.owner); continue; }
      const list = output.get(segment.owner) ?? [];
      const paintSegment: SharedBorderPaintSegment = { d: points.map(local).map((point, index) => `${index ? "L" : "M"} ${point.x} ${point.y}`).join(" "), state: segment.state, ...disabledBoxes.has(segment.id.split(":")[0]!) ? { disabled: true as const } : {} };
      list.push(paintSegment);
      output.set(segment.owner, list);
    }
    for (const [id, entry] of this.windows) {
      const owned = sources.filter(source => source.owner === id);
      const won = resolved.filter(segment => segment.owner === id);
      const length = (segment: { from: { x: number; y: number }; to: { x: number; y: number } }) => Math.abs(segment.to.x - segment.from.x) + Math.abs(segment.to.y - segment.from.y);
      const complete = owned.length > 0 && Math.abs(owned.reduce((sum, edge) => sum + length(edge), 0) - won.reduce((sum, edge) => sum + length(edge), 0)) < 0.001;
      const sameState = won.length > 0 && won.every(segment => segment.state === won[0]!.state);
      const unclipped = owned.every(source => (source.from.x === source.logicalFrom.x && source.from.y === source.logicalFrom.y && source.to.x === source.logicalTo.x && source.to.y === source.logicalTo.y) || (source.to.x === source.logicalFrom.x && source.to.y === source.logicalFrom.y && source.from.x === source.logicalTo.x && source.from.y === source.logicalTo.y));
      updates.set(entry, entry.geometry.chipRegions.length === 0 && complete && sameState && unclipped && !interacting.has(id) ? [{ d: entry.geometry.borderPath, state: won[0]!.state }] : output.get(id) ?? []);
    }
    for (const [id, { element, frame, width, height, sides }] of frameSources) {
      const size = `${width}x${height}`;
      if (fresh || frame.sized !== size) {
        const computed = element.ownerDocument.defaultView?.getComputedStyle(element);
        frame.svg.style.left = `${-parseFloat(computed?.borderLeftWidth || "0") || 0}px`;
        frame.svg.style.top = `${-parseFloat(computed?.borderTopWidth || "0") || 0}px`;
        sizeSharedBorderOverlay(frame.svg, width, height, this.measure(element).raw);
      }
      if (frame.sized !== size) {
        frame.svg.setAttribute("width", String(width)); frame.svg.setAttribute("height", String(height));
        if (width > 0 && height > 0) frame.svg.setAttribute("viewBox", `0 0 ${width} ${height}`); else frame.svg.removeAttribute("viewBox");
        frame.sized = size;
      }
      const segments = [...output.get(id) ?? []].sort((a, b) => borderStatePriority[a.state] - borderStatePriority[b.state]);
      const signature = JSON.stringify(segments);
      const stated = segments.some(segment => segment.state !== "normal") ? "41" : "40";
      if (frame.svg.style.zIndex !== stated) frame.svg.style.zIndex = stated;
      if (frame.svg.getAttribute("data-paint") !== signature) {
        frame.svg.replaceChildren(...segments.map((segment, index) => {
          const path = element.ownerDocument.createElementNS(namespace, "path");
          path.setAttribute("d", segment.d); path.setAttribute("fill", "none");
          path.setAttribute("stroke", sharedBorderStroke(segment.state));
          path.setAttribute("vector-effect", "non-scaling-stroke");
          path.setAttribute("stroke-linecap", "square"); path.setAttribute("stroke-linejoin", "miter");
          path.setAttribute("class", `window-frame-border window-frame-border-${segment.state}`);
          path.setAttribute("data-shared-border-segment", ""); path.setAttribute("data-shared-border-state", segment.state);
          if (segment.disabled) path.setAttribute("data-shared-border-disabled", "");
          if (segment.state !== "celebrated") return path;
          path.classList.add("window-frame-border-celebrated-mask");
          const group = element.ownerDocument.createElementNS(namespace, "g");
          const defs = element.ownerDocument.createElementNS(namespace, "defs");
          const mask = element.ownerDocument.createElementNS(namespace, "mask");
          const maskId = `shared-border-celebrate-${frame.id}-${index}`;
          mask.setAttribute("id", maskId); mask.setAttribute("maskUnits", "userSpaceOnUse");
          mask.setAttribute("x", "0"); mask.setAttribute("y", "0"); mask.setAttribute("width", String(width)); mask.setAttribute("height", String(height));
          path.setAttribute("stroke", "white"); mask.append(path); defs.append(mask);
          const fill = element.ownerDocument.createElementNS(namespace, "foreignObject");
          fill.setAttribute("x", "0"); fill.setAttribute("y", "0"); fill.setAttribute("width", String(width)); fill.setAttribute("height", String(height)); fill.setAttribute("mask", `url(#${maskId})`);
          const content = element.ownerDocument.createElement("div");
          content.className = "window-frame-border-celebrated-fill"; content.style.cssText = "width:100%;height:100%";
          fill.append(content); group.append(defs, fill); return group;
        }));
        frame.svg.setAttribute("data-paint", signature);
      }
      if (sides !== null && element.getAttribute("data-shared-border-attached") !== sides) element.setAttribute("data-shared-border-attached", sides);
    }
    const changed: [Registration, readonly SharedBorderPaintSegment[], string][] = [];
    for (const [entry, segments] of updates) {
      const signature = JSON.stringify(segments);
      if (entry.painted !== signature) changed.push([entry, segments, signature]);
    }
    for (const [entry, segments, signature] of changed) { entry.painted = signature; entry.update(segments); }
    for (const element of this.motion.keys()) if (!this.root.contains(element)) this.motion.delete(element);
    pruneSharedBorderMotion(this.motion);
    this.observer?.takeRecords();
    if (!this.frame) this.root.removeAttribute("data-shared-border-pending");
    if (this.motion.size) this.scheduleGeometry();
  };
  dispose() {
    if (this.disposed) return;
    this.disposed = true;
    this.root.removeAttribute("data-shared-border-root");
    this.root.removeAttribute("data-shared-border-pending");
    this.observer?.disconnect(); this.resize?.disconnect();
    const view = this.root.ownerDocument.defaultView;
    if (this.frame) view?.cancelAnimationFrame?.(this.frame);
    this.frame = 0;
    for (const [element, frame] of this.frames) this.restore(element, frame);
    this.root.removeEventListener("pointerover", this.pointer); this.root.removeEventListener("pointerout", this.pointer);
    this.root.removeEventListener("focusin", this.scheduleState); this.root.removeEventListener("focusout", this.scheduleState);
    this.root.removeEventListener("keydown", this.scheduleState, true); this.root.removeEventListener("pointerdown", this.scheduleState, true);
    this.root.removeEventListener("transitionrun", this.motionStart); this.root.removeEventListener("transitionend", this.motionEnd); this.root.removeEventListener("transitioncancel", this.motionEnd);
    view?.removeEventListener?.("resize", this.scheduleGeometry); view?.removeEventListener?.("scroll", this.scheduleGeometry, true);
    if (roots.get(this.root)?.compositor === this) roots.delete(this.root);
  }
}

/** 🧷️ Retains one root-local compositor until its last window or frame participant leaves. */
export function retainSharedBorderRoot(root: HTMLElement): { compositor: BorderCompositor; release: () => void } {
  let entry = roots.get(root);
  if (!entry) {
    entry = { compositor: new BorderCompositor(root), participants: 0 };
    roots.set(root, entry);
  }
  entry.participants++;
  entry.compositor.schedule();
  let released = false;
  return { compositor: entry.compositor, release: () => {
    if (released) return;
    released = true;
    if (--entry.participants === 0) entry.compositor.dispose();
    else entry.compositor.schedule();
  } };
}

/** 🖇️ Activates marker frames without a window and preserves the host's forwarded ref. */
export function useSharedBorderFrameRef<T extends HTMLElement>(forwardedRef?: React.Ref<T>): React.RefCallback<T> {
  const scope = useShellScopeOptional();
  const [element, setElement] = React.useState<T | null>(null);
  React.useLayoutEffect(() => {
    if (!element) return;
    const root = scope?.rootRef?.current ?? element.closest<HTMLElement>(".semio-scope") ?? element.ownerDocument.body;
    return retainSharedBorderRoot(root).release;
  }, [element, scope]);
  return React.useCallback((node: T | null) => {
    setElement(node);
    if (typeof forwardedRef === "function") {
      const cleanup = forwardedRef(node);
      if (typeof cleanup === "function") return () => { setElement(null); cleanup(); };
    } else if (forwardedRef) forwardedRef.current = node;
  }, [forwardedRef]);
}

/** 🖌️ Paints a window's resolved segments into its frame `svg` without React: the path marked `data-shared-border-main` takes the outline of the segments in the window's own state (`fallback` before any are resolved), and the layer marked `data-shared-border-extra` holds one path per segment in another state. Writes only what changed. */
export function paintSharedWindowBorder(svg: SVGSVGElement, segments: readonly SharedBorderPaintSegment[] | null, fallback: string, kind: BorderState): void {
  const main = svg.querySelector("[data-shared-border-main]");
  const outline = segments === null ? fallback : segments.filter(segment => segment.state === kind).map(segment => segment.d).join(" ");
  if (main && main.getAttribute("d") !== outline) main.setAttribute("d", outline);
  const layer = svg.querySelector("[data-shared-border-extra]");
  if (!layer) return;
  const extra = segments === null ? [] : segments.filter(segment => segment.state !== kind);
  while (layer.children.length > extra.length) layer.lastElementChild!.remove();
  extra.forEach((segment, index) => {
    let path = layer.children[index] as SVGPathElement | undefined;
    if (!path) {
      path = svg.ownerDocument.createElementNS(namespace, "path");
      path.setAttribute("fill", "none"); path.setAttribute("stroke-linejoin", "miter"); path.setAttribute("stroke-linecap", "square");
      path.setAttribute("vector-effect", "non-scaling-stroke"); path.setAttribute("data-shared-border-segment", "");
      layer.append(path);
    }
    if (path.getAttribute("d") !== segment.d) path.setAttribute("d", segment.d);
    const stroke = sharedBorderStroke(segment.state), className = `window-frame-border window-frame-border-${segment.state}`;
    if (path.getAttribute("stroke") !== stroke) path.setAttribute("stroke", stroke);
    if (path.getAttribute("class") !== className) path.setAttribute("class", className);
    path.setAttribute("data-shared-border-state", segment.state);
    if (kind === "active" && segment.state === "focus") path.setAttribute("data-focus-on-accent", ""); else path.removeAttribute("data-focus-on-accent");
  });
}

/** 🪟️ Connects a frame to its shell-local compositor, including portaled descendants. The compositor hands every changed outline to `paint`, outside React: a pan frame renders no component. */
export function useSharedWindowBorder(element: HTMLElement | null, geometry: WindowFrameGeometry, state: BorderState, paint?: (segments: readonly SharedBorderPaintSegment[]) => void): void {
  const scope = useShellScopeOptional();
  const id = React.useId();
  const current = React.useRef<BorderCompositor | null>(null);
  const latest = React.useRef({ geometry, state, paint });
  latest.current = { geometry, state, paint };
  React.useLayoutEffect(() => {
    if (!element) return;
    const root = scope?.rootRef?.current ?? element.closest<HTMLElement>(".semio-scope") ?? element.ownerDocument.body;
    const { compositor, release } = retainSharedBorderRoot(root);
    current.current = compositor;
    compositor.windows.set(id, { element, geometry: latest.current.geometry, state: latest.current.state, update: next => latest.current.paint?.(next) });
    compositor.schedule();
    return () => {
      current.current = null;
      compositor.windows.delete(id);
      release();
    };
  }, [element, id, scope]);
  React.useLayoutEffect(() => {
    const entry = current.current?.windows.get(id);
    if (!entry) return;
    entry.geometry = geometry; entry.state = state;
    current.current?.scheduleGeometry();
  }, [geometry, id, state]);
}

/** 🧰️ Props and root-retaining ref of a borderless box: spread the result on the element. The compositor is also attached by {@link attachSharedBorderRoots}, so a box without this hook (a mapped row, a constant attribute set) is still painted. */
export function useSharedBorderBox<T extends HTMLElement>(forwardedRef?: React.Ref<T>) {
  const ref = useSharedBorderFrameRef<T>(forwardedRef);
  return { ref, ...sharedBorderlessBoxProps } as const;
}

const attachedRoots = new Map<HTMLElement, () => void>();
const attachSelector = `${ownerSelector}, [data-window-frame-chip]`;

/** 🔗️ Retains a compositor for the root of every box that appears in `document`, until that root leaves the document or holds no box any more: a box can never exist without a compositor, however its attributes were set. Called once at module load; returns the stop function. */
export function attachSharedBorderRoots(document: Document): () => void {
  if (typeof MutationObserver === "undefined") return () => undefined;
  const attach = (box: Element) => {
    const root = box.closest<HTMLElement>(".semio-scope") ?? box.ownerDocument.body;
    if (root && !attachedRoots.has(root)) attachedRoots.set(root, retainSharedBorderRoot(root).release);
  };
  const sweep = () => { for (const [root, release] of attachedRoots) if (!root.isConnected || !root.querySelector(attachSelector)) { release(); attachedRoots.delete(root); } };
  const observer = new MutationObserver(records => {
    for (const record of records) {
      if (record.type === "attributes") { if (isElement(record.target)) attach(record.target); continue; }
      for (const node of record.addedNodes) if (isElement(node)) { if (node.matches(attachSelector)) attach(node); else if (node.namespaceURI === xhtml) { const inner = node.querySelector(attachSelector); if (inner) attach(inner); } }
    }
    if (records.some(record => [...record.removedNodes].some(node => isElement(node) && (node.matches(attachSelector) || (node.namespaceURI === xhtml && node.querySelector(attachSelector) !== null))))) sweep();
  });
  observer.observe(document.documentElement, { subtree: true, childList: true, attributes: true, attributeFilter: ["data-shared-border-frame", "data-shared-border-edges", "data-shared-border-cells", "data-shared-border-group"] });
  for (const box of document.querySelectorAll(attachSelector)) attach(box);
  return () => { observer.disconnect(); for (const release of attachedRoots.values()) release(); attachedRoots.clear(); };
}

if (typeof document !== "undefined") attachSharedBorderRoots(document);
