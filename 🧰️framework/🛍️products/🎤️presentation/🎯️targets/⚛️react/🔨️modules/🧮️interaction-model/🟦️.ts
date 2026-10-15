/** 🧮️ Interactive layout model: transforms, placements, marquee and resize rectangle math, section-relative measurement, flow offsets and the interaction state context.
 *
 * 🧱️ Child module of {@link presentation-react | the `@semio-tech/presentation-react` barrel}, which re-exports every public name below so import specifiers stay unchanged.
 */

import type { DispositionPosition } from "@semio-tech/presentation";
import { unionDispositionPositions } from "@semio-tech/presentation";
import { createContext, useContext, type CSSProperties } from "@semio-tech/ui-react";
import { type RevealResolvedDisposition } from "../🔀️reveal-morph/🟦️.ts";
import { parsePresentationSlideCssSize } from "../🏷️morph-context/🟦️.ts";

//#region 🔖️Interaction
const DISPOSITION_MIN_FRACTION = 0.02;
export const POINTER_DRAG_THRESHOLD_PX = 3;

/** 🗺️ Whether reveal.js is showing the slide grid (Escape overview). */
export function revealDeckInOverview(element: Element | null): boolean {
  return element?.closest(".reveal")?.classList.contains("overview") ?? false;
}

/** 🗺️ Swallow the post-gesture click reveal.js uses to leave overview (capture on `.reveal`). */
export function suppressRevealOverviewSlideNavigation(event: Event): void {
  const origin = event.target;
  if (!(origin instanceof Element)) {
    return;
  }
  const reveal = origin.closest(".reveal");
  if (!reveal) {
    return;
  }
  const swallowClick = (clickEvent: Event): void => {
    if (!(clickEvent.target instanceof Element) || !clickEvent.target.closest("section")) {
      return;
    }
    clickEvent.preventDefault();
    clickEvent.stopImmediatePropagation();
    reveal.removeEventListener("click", swallowClick, true);
  };
  reveal.addEventListener("click", swallowClick, true);
}

/** 🖱️ Disposition ids to move/resize for this gesture (before async `selectIds` commits). */
export function resolveDispositionDragGroupIds(id: string, wasSelected: boolean, additive: boolean, selectedIds: ReadonlySet<string>): readonly string[] {
  if (!wasSelected) {
    if (additive) {
      const group = new Set(selectedIds);
      group.add(id);
      return group.size > 1 ? [...group] : [id];
    }
    return [id];
  }
  if (additive) {
    return [id];
  }
  return selectedIds.size > 1 ? [...selectedIds] : [id];
}

/** 📐️ Ephemeral slide-space rectangle for interactive dispositions (normalized 0..1). */
export type DispositionTransform = DispositionPosition;

/** ↔ Marquee selection mode: crossing (L→R partial overlap) vs window (R→L full containment). */
export type MarqueeSelectionRule = "crossing" | "window";

/** ⊡ Eight resize handles on a disposition frame. */
export type DispositionResizeHandle = "nw" | "n" | "ne" | "e" | "se" | "s" | "sw" | "w";

export const DISPOSITION_RESIZE_HANDLES: readonly DispositionResizeHandle[] = ["nw", "n", "ne", "e", "se", "s", "sw", "w"];

/** 🔑️ Stable id for one resolved disposition on a slide. */
export function dispositionInteractionId(renderSlideId: string, disposition: RevealResolvedDisposition, index: number): string {
  return `${renderSlideId}--${disposition.morphId}--${disposition.embodimentId ?? index}`;
}

/** 🔑️ Stable id for one split tile on an interactive slide. */
export function tileDispositionInteractionId(renderSlideId: string, disposition: RevealResolvedDisposition, dispositionIndex: number, tileKey: string): string {
  return `${dispositionInteractionId(renderSlideId, disposition, dispositionIndex)}--tile--${tileKey}`;
}

/** 🔑️ Stable id for a visual row band grouping split tiles on one disposition. */
export function rowBandInteractionId(renderSlideId: string, disposition: RevealResolvedDisposition, dispositionIndex: number, rowIndex: number): string {
  return `${dispositionInteractionId(renderSlideId, disposition, dispositionIndex)}--row--${rowIndex}`;
}

/** 🖱️ One interactive placement (whole disposition or a single split tile). */
export interface InteractiveDispositionPlacement {
  readonly id: string;
  readonly disposition: RevealResolvedDisposition;
  /** 📐️ Wrapper frame (row-local for tiles inside a visual row). */
  readonly declaredRect: DispositionPosition | undefined;
  /** 📐️ Slide-space frame for marquee, drag, and resize. */
  readonly sectionRect: DispositionPosition | undefined;
  /** 🔗️ reveal.js `data-id` on the canvas wrapper when the tile frame must not own it. */
  readonly revealMorphId?: string;
  readonly rowBandId?: string;
}

/** 📏️ Row-level hit target spanning all tiles in one visual row of a split disposition. */
export interface InteractiveRowBandPlacement {
  readonly id: string;
  readonly frame: DispositionPosition;
  readonly tileIds: readonly string[];
}

/** 🖱️ Interactive placements and row bands for one slide arrangement. */
export interface InteractiveSlideLayout {
  readonly placements: readonly InteractiveDispositionPlacement[];
  readonly rowBands: readonly InteractiveRowBandPlacement[];
}

/** 🧩️ Builds one interactive placement per resolved disposition. */
export function buildInteractiveSlideLayout(renderSlideId: string, resolved: readonly RevealResolvedDisposition[], morph = false): InteractiveSlideLayout {
  const placements: InteractiveDispositionPlacement[] = [];
  resolved.forEach((disposition, dispositionIndex) => {
    const declaredRect = declaredDispositionRect(disposition);
    placements.push({
      id: dispositionInteractionId(renderSlideId, disposition, dispositionIndex),
      disposition,
      declaredRect,
      sectionRect: declaredRect,
      revealMorphId: morph && declaredRect !== undefined ? (disposition.revealMorphCompanion !== undefined ? disposition.morphId : disposition.morphFrom?.length ? undefined : disposition.morphId) : undefined,
    });
  });
  return { placements, rowBands: [] };
}

/** 📐️ True when two normalized rectangles overlap with positive area. */
export function rectsIntersect(a: DispositionPosition, b: DispositionPosition): boolean {
  return a.x < b.x + b.width && a.x + a.width > b.x && a.y < b.y + b.height && a.y + a.height > b.y;
}

/** 📐️ True when outer fully contains inner. */
export function rectContains(outer: DispositionPosition, inner: DispositionPosition): boolean {
  return inner.x >= outer.x && inner.y >= outer.y && inner.x + inner.width <= outer.x + outer.width && inner.y + inner.height <= outer.y + outer.height;
}

/** ⊞ Normalized marquee rectangle from two pointer fractions. */
export function normalizeMarquee(start: { readonly x: number; readonly y: number }, end: { readonly x: number; readonly y: number }): DispositionPosition {
  const x = Math.min(start.x, end.x);
  const y = Math.min(start.y, end.y);
  return {
    x,
    y,
    width: Math.abs(end.x - start.x),
    height: Math.abs(end.y - start.y),
  };
}

/** ↔ Crossing when dragged right-to-left (end.x < start.x), else window. */
export function marqueeSelectionRule(start: { readonly x: number; readonly y: number }, end: { readonly x: number; readonly y: number }): MarqueeSelectionRule {
  return end.x < start.x ? "crossing" : "window";
}

/** 🎯️ Whether a marquee selects a target rect under crossing or window rules. */
export function marqueeSelects(marquee: DispositionPosition, target: DispositionPosition, rule: MarqueeSelectionRule): boolean {
  if (marquee.width <= 0 || marquee.height <= 0) {
    return false;
  }
  return rule === "crossing" ? rectsIntersect(marquee, target) : rectContains(marquee, target);
}

function clampFraction(value: number): number {
  return Math.max(0, Math.min(1, value));
}

/** ↔ Moves a normalized rect by fractional deltas (unbounded; follows pointer across the slide). */
export function translateDispositionRect(rect: DispositionPosition, dx: number, dy: number): DispositionPosition {
  return {
    x: rect.x + dx,
    y: rect.y + dy,
    width: rect.width,
    height: rect.height,
  };
}

/** ⊡ Resizes a normalized rect from one handle by fractional deltas. */
export function resizeDispositionRect(rect: DispositionPosition, handle: DispositionResizeHandle, dx: number, dy: number, minSize: number = DISPOSITION_MIN_FRACTION): DispositionPosition {
  let { x, y, width, height } = rect;
  if (handle.includes("e")) {
    width += dx;
  }
  if (handle.includes("w")) {
    x += dx;
    width -= dx;
  }
  if (handle.includes("s")) {
    height += dy;
  }
  if (handle.includes("n")) {
    y += dy;
    height -= dy;
  }
  width = Math.max(minSize, width);
  height = Math.max(minSize, height);
  x = clampFraction(x);
  y = clampFraction(y);
  if (x + width > 1) {
    width = 1 - x;
  }
  if (y + height > 1) {
    height = 1 - y;
  }
  if (width < minSize) {
    width = minSize;
  }
  if (height < minSize) {
    height = minSize;
  }
  return { x, y, width, height };
}

/** ⊞ Union bounding box of normalized rectangles. */
export function groupBoundingRect(rects: readonly DispositionPosition[]): DispositionPosition | null {
  if (rects.length === 0) {
    return null;
  }
  return unionDispositionPositions(rects);
}

/** ⊞ Scales one member rect when a group bounding box is resized. */
export function scaleRectWithinGroup(rect: DispositionPosition, oldGroup: DispositionPosition, newGroup: DispositionPosition): DispositionPosition {
  if (oldGroup.width <= 0 || oldGroup.height <= 0) {
    return rect;
  }
  const relX = (rect.x - oldGroup.x) / oldGroup.width;
  const relY = (rect.y - oldGroup.y) / oldGroup.height;
  const relW = rect.width / oldGroup.width;
  const relH = rect.height / oldGroup.height;
  return {
    x: newGroup.x + relX * newGroup.width,
    y: newGroup.y + relY * newGroup.height,
    width: relW * newGroup.width,
    height: relH * newGroup.height,
  };
}

/** ⛶️ Centered near-slide frame for interactive enlarge (uniform across figure, video, pdf, tiles). */
export const SLIDE_INTERACTIVE_ENLARGE_FRAME: DispositionPosition = {
  x: 0.05,
  y: 0.075,
  width: 0.9,
  height: 0.85,
};

/** ⛶️ Toggles uniform enlarged slide frame vs stashed pre-enlarge rect. */
export function toggleEnlargeRect(current: DispositionPosition, stash: DispositionPosition | undefined): { readonly rect: DispositionPosition; readonly stash: DispositionPosition | undefined } {
  if (stash !== undefined) {
    return { rect: stash, stash: undefined };
  }
  return { rect: SLIDE_INTERACTIVE_ENLARGE_FRAME, stash: current };
}

/** 📐️ reveal.js nested slide section with usable layout height for pointer math. */
export function slideCoordinateRoot(sectionEl: HTMLElement): HTMLElement {
  const arrangement = sectionEl.closest("section.presentation-arrangement--interactive");
  if (arrangement instanceof HTMLElement) {
    return arrangement;
  }
  let current: HTMLElement | null = sectionEl;
  while (current && !current.classList.contains("slides")) {
    if (current.offsetWidth > 0 && current.offsetHeight > 0) {
      return current;
    }
    current = current.parentElement;
  }
  const stack = sectionEl.closest(".reveal .slides > section.present");
  if (stack instanceof HTMLElement && stack.offsetWidth > 0 && stack.offsetHeight > 0) {
    return stack;
  }
  const parent = sectionEl.parentElement;
  if (parent instanceof HTMLElement && parent.offsetWidth > 0 && parent.offsetHeight > 0) {
    return parent;
  }
  const reveal = sectionEl.closest(".reveal");
  return reveal instanceof HTMLElement ? reveal : sectionEl;
}

/** 📐️ Client/layout bounds for slide-space fractions when inner reveal sections report zero height. */
export function slideLayoutBounds(sectionEl: HTMLElement): DOMRect {
  const root = slideCoordinateRoot(sectionEl);
  const rect = root.getBoundingClientRect();
  if (rect.width > 0 && rect.height > 0) {
    return rect;
  }
  const reveal = sectionEl.closest(".reveal");
  const { width, height } = parsePresentationSlideCssSize(reveal instanceof HTMLElement ? reveal : null);
  const anchor = sectionEl.getBoundingClientRect();
  return new DOMRect(anchor.left, anchor.top, width, height);
}

/** 🖼️ Arrangement canvas when dispositions use declared slide-space frames; otherwise the slide coordinate root. */
export function dispositionPlacementContainer(sectionEl: HTMLElement, canvasPlacement: boolean): HTMLElement {
  if (!canvasPlacement) {
    return slideCoordinateRoot(sectionEl);
  }
  const canvas = sectionEl.querySelector(".presentation-arrangement-canvas");
  return canvas instanceof HTMLElement ? canvas : sectionEl;
}

/** 🖼️ Ink-bearing node used to measure a positioned disposition on the arrangement canvas. */
export function dispositionFrameElement(root: HTMLElement): HTMLElement {
  return (root.querySelector(".presentation-disposition-frame, .presentation-morph-slot--figure, .presentation-morph-anchor") as HTMLElement | null) ?? root;
}

/** ⊡ Selection chrome style: flow slides use measured ink frame; canvas-framed slides use wrapper inset via CSS. */
export function interactiveDispositionChromeStyle(options: { readonly selected: boolean; readonly effectiveRect: DispositionPosition | undefined; readonly canvasFramed: boolean; readonly enlarged: boolean }): CSSProperties | undefined {
  const { selected, effectiveRect, canvasFramed, enlarged } = options;
  if (!selected || !effectiveRect || enlarged || canvasFramed) {
    return undefined;
  }
  return transformFrameStyle(effectiveRect);
}

/** 📍️ Maps client coordinates to normalized fractions inside a section element. */
export function clientToSectionFraction(sectionEl: HTMLElement, clientX: number, clientY: number, options?: { readonly clamp?: boolean }): { readonly x: number; readonly y: number } {
  const bounds = slideLayoutBounds(sectionEl);
  if (bounds.width <= 0 || bounds.height <= 0) {
    return { x: 0, y: 0 };
  }
  const x = (clientX - bounds.left) / bounds.width;
  const y = (clientY - bounds.top) / bounds.height;
  if (options?.clamp === false) {
    return { x, y };
  }
  return {
    x: clampFraction(x),
    y: clampFraction(y),
  };
}

/** 📍️ Maps an element's client rect to normalized fractions inside a section. */
export function measureElementRectInSection(element: HTMLElement, sectionEl: HTMLElement): DispositionPosition | null {
  const sectionBounds = slideLayoutBounds(sectionEl);
  if (sectionBounds.width <= 0 || sectionBounds.height <= 0) {
    return null;
  }
  const rect = element.getBoundingClientRect();
  if (rect.width <= 0 || rect.height <= 0) {
    return null;
  }
  return clientRectToSectionFraction(rect, sectionBounds);
}

const DISPOSITION_BOUNDS_SELECTORS =
  "[data-id], .presentation-disposition-frame, .presentation-morph-anchor, .presentation-intro-line, .presentation-morph-text, h1, h2, h3, h4, p, li, img, video, iframe, .presentation-media-figure, .presentation-media-iframe, .presentation-figure-crop-fill, .presentation-morph-slot--figure";

function unionDomRects(a: DOMRect, b: DOMRect): DOMRect {
  const left = Math.min(a.left, b.left);
  const top = Math.min(a.top, b.top);
  const right = Math.max(a.right, b.right);
  const bottom = Math.max(a.bottom, b.bottom);
  return new DOMRect(left, top, right - left, bottom - top);
}

function tightRangeBoundsRect(element: HTMLElement): DOMRect | null {
  if (typeof document.createRange !== "function") {
    return null;
  }
  try {
    const range = document.createRange();
    range.selectNodeContents(element);
    if (typeof range.getClientRects === "function") {
      const clientRects = range.getClientRects();
      let union: DOMRect | null = null;
      for (const rect of clientRects) {
        if (rect.width <= 0 || rect.height <= 0) {
          continue;
        }
        union = union === null ? rect : unionDomRects(union, rect);
      }
      if (union !== null) {
        return union;
      }
    }
    if (typeof range.getBoundingClientRect === "function") {
      const rangeBox = range.getBoundingClientRect();
      if (rangeBox.width > 0 && rangeBox.height > 0) {
        return rangeBox;
      }
    }
  } catch {
    return null;
  }
  return null;
}

function alignTightBoxWithin(element: HTMLElement, container: DOMRect, tight: DOMRect): DOMRect {
  const align = typeof getComputedStyle === "function" ? getComputedStyle(element).textAlign : "center";
  let left = tight.left;
  if (align === "center") {
    left = container.left + (container.width - tight.width) / 2;
  } else if (align === "right") {
    left = container.right - tight.width;
  }
  return new DOMRect(left, tight.top, tight.width, tight.height);
}

function tightProbeBoundsRect(element: HTMLElement, container: DOMRect): DOMRect | null {
  const probe = document.createElement("div");
  probe.style.cssText = "position:fixed;left:0;top:0;visibility:hidden;pointer-events:none;width:max-content;max-width:none;";
  const clone = element.cloneNode(true) as HTMLElement;
  clone.style.display = "inline-block";
  clone.style.width = "auto";
  clone.style.maxWidth = "none";
  probe.appendChild(clone);
  document.body.appendChild(probe);
  const probeBox = clone.getBoundingClientRect();
  document.body.removeChild(probe);
  if (probeBox.width <= 0 || probeBox.height <= 0) {
    return null;
  }
  return alignTightBoxWithin(element, container, probeBox);
}

/** 📍️ Ink bounds for text nodes (block headings otherwise span the full slide width). */
export function tightElementBoundsRect(element: HTMLElement): DOMRect | null {
  const box = element.getBoundingClientRect();
  if (box.width <= 0 || box.height <= 0) {
    return null;
  }
  const rangeBox = tightRangeBoundsRect(element);
  if (rangeBox !== null && rangeBox.width < box.width * 0.95) {
    return alignTightBoxWithin(element, box, rangeBox);
  }
  const probeBox = tightProbeBoundsRect(element, box);
  if (probeBox !== null && probeBox.width < box.width * 0.95) {
    return probeBox;
  }
  return box;
}

function dispositionNodeBoundsRect(node: HTMLElement, sectionWidth: number): DOMRect | null {
  const box = node.getBoundingClientRect();
  if (box.width <= 0 || box.height <= 0) {
    return null;
  }
  const useTight = node.matches("[data-id], h1, h2, h3, h4, p, .presentation-morph-text, li, .presentation-intro-line") && box.width >= sectionWidth * 0.85;
  return useTight ? (tightElementBoundsRect(node) ?? box) : box;
}

function clientRectToSectionFraction(rect: DOMRect, sectionBounds: DOMRect): DispositionPosition {
  return {
    x: (rect.left - sectionBounds.left) / sectionBounds.width,
    y: (rect.top - sectionBounds.top) / sectionBounds.height,
    width: rect.width / sectionBounds.width,
    height: rect.height / sectionBounds.height,
  };
}

/** 📍️ Union of morph content bounds in section space (avoids full-width flow wrappers). */
export function measureDispositionBoundsInSection(root: HTMLElement, sectionEl: HTMLElement): DispositionPosition | null {
  root.classList.add("presentation-interactive-disposition--measuring");
  try {
    const sectionBounds = slideLayoutBounds(sectionEl);
    if (sectionBounds.width <= 0 || sectionBounds.height <= 0) {
      return null;
    }
    let union: DOMRect | null = null;
    for (const node of root.querySelectorAll(DISPOSITION_BOUNDS_SELECTORS)) {
      if (!(node instanceof HTMLElement)) {
        continue;
      }
      const rect = dispositionNodeBoundsRect(node, sectionBounds.width);
      if (!rect) {
        continue;
      }
      union = union === null ? rect : unionDomRects(union, rect);
    }
    if (union === null) {
      const fallback = root.getBoundingClientRect();
      if (fallback.width <= 0 || fallback.height <= 0) {
        return null;
      }
      return clientRectToSectionFraction(fallback, sectionBounds);
    }
    return clientRectToSectionFraction(union, sectionBounds);
  } finally {
    root.classList.remove("presentation-interactive-disposition--measuring");
  }
}

/** 📍️ Ink bounds as fractions inside a container (selection chrome and fill use this space). */
export function measureDispositionBoundsInContainer(root: HTMLElement, containerEl: HTMLElement): DispositionPosition | null {
  root.classList.add("presentation-interactive-disposition--measuring");
  try {
    const containerBounds = containerEl.getBoundingClientRect();
    if (containerBounds.width <= 0 || containerBounds.height <= 0) {
      return null;
    }
    let union: DOMRect | null = null;
    for (const node of root.querySelectorAll(DISPOSITION_BOUNDS_SELECTORS)) {
      if (!(node instanceof HTMLElement)) {
        continue;
      }
      const rect = dispositionNodeBoundsRect(node, containerBounds.width);
      if (!rect) {
        continue;
      }
      union = union === null ? rect : unionDomRects(union, rect);
    }
    if (union === null) {
      const fallback = root.getBoundingClientRect();
      if (fallback.width <= 0 || fallback.height <= 0) {
        return null;
      }
      return clientRectToSectionFraction(fallback, containerBounds);
    }
    return clientRectToSectionFraction(union, containerBounds);
  } finally {
    root.classList.remove("presentation-interactive-disposition--measuring");
  }
}

/** 📏️ True when measured fractions are large enough to drag or resize reliably. */
export function isUsableMeasuredRect(rect: DispositionPosition): boolean {
  return rect.width >= DISPOSITION_MIN_FRACTION && rect.height >= DISPOSITION_MIN_FRACTION;
}

/** 📐️ Declared placement for one resolved disposition (includes dormant morph anchors at opacity 0). */
export function declaredDispositionRect(disposition: RevealResolvedDisposition): DispositionPosition | undefined {
  return disposition.position;
}

export function transformFrameStyle(transform: DispositionPosition): CSSProperties {
  return {
    position: "absolute",
    left: `${transform.x * 100}%`,
    top: `${transform.y * 100}%`,
    width: `${transform.width * 100}%`,
    height: `${transform.height * 100}%`,
    boxSizing: "border-box",
  };
}

/** 📐️ Cumulative visual scale for one element (client rect vs layout box, includes ancestor transforms). */
export function elementVisualScale(element: HTMLElement): number {
  const rect = element.getBoundingClientRect();
  const layoutW = element.offsetWidth;
  const layoutH = element.offsetHeight;
  if (layoutW <= 0 || layoutH <= 0 || rect.width <= 0 || rect.height <= 0) {
    return 1;
  }
  const scaleX = rect.width / layoutW;
  const scaleY = rect.height / layoutH;
  const scale = Math.min(scaleX, scaleY);
  if (!Number.isFinite(scale) || scale <= 0) {
    return 1;
  }
  return Math.min(4, Math.max(0.05, scale));
}

/** 📐️ reveal.js scales slides visually; map screen-pointer deltas to local translate pixels. */
export function sectionVisualScale(sectionEl: HTMLElement): number {
  return elementVisualScale(slideCoordinateRoot(sectionEl));
}

/** ↔ Node that receives flow `translate3d` during drag (offset wrapper, else content, else disposition root). */
export function flowDragTransformElement(sectionEl: HTMLElement, root: HTMLElement | null, content: HTMLElement | null): HTMLElement {
  if (root?.classList.contains("presentation-interactive-disposition--offset")) {
    return root;
  }
  if (content && content.offsetWidth > 0 && content.offsetHeight > 0) {
    return content;
  }
  if (root && root.offsetWidth > 0 && root.offsetHeight > 0) {
    return root;
  }
  return slideCoordinateRoot(sectionEl);
}

/** ↔ Pointer travel in screen px → local px for CSS translate on the flow drag target. */
export function flowPointerDeltaToLocal(transformEl: HTMLElement, startClientX: number, startClientY: number, currentClientX: number, currentClientY: number): { readonly dx: number; readonly dy: number } {
  const scale = elementVisualScale(transformEl);
  const safe = Number.isFinite(scale) && scale > 0 ? Math.min(4, Math.max(0.05, scale)) : 1;
  return {
    dx: (currentClientX - startClientX) / safe,
    dy: (currentClientY - startClientY) / safe,
  };
}

/** ↔ Flow-layout drag: local-pixel translate (x/y are px, not normalized). */
export function flowDispositionOffsetStyle(transform: DispositionPosition): CSSProperties {
  return {
    transform: `translate3d(${transform.x}px, ${transform.y}px, 0)`,
  };
}

/** 📐️ True when two normalized disposition frames differ. */
export function dispositionPositionChanged(a: DispositionPosition, b: DispositionPosition): boolean {
  return Math.abs(a.x - b.x) > 1e-6 || Math.abs(a.y - b.y) > 1e-6 || Math.abs(a.width - b.width) > 1e-6 || Math.abs(a.height - b.height) > 1e-6;
}

const SLIDE_INTERACTION_RESET_PROXIMITY_PX = 72;

/** 📐️ True when any disposition on the slide has ephemeral layout (drag, resize, enlarge, or pdf page). */
export function slideHasEphemeralLayout(
  transforms: ReadonlyMap<string, DispositionTransform>,
  enlargedIds: ReadonlySet<string>,
  declaredRects: ReadonlyMap<string, DispositionPosition | undefined>,
  pdfPageById: ReadonlyMap<string, number> = new Map(),
): boolean {
  if (enlargedIds.size > 0 || pdfPageById.size > 0) {
    return true;
  }
  for (const [id, transform] of transforms) {
    const declared = declaredRects.get(id);
    if (declared === undefined) {
      return true;
    }
    if (dispositionPositionChanged(declared, transform)) {
      return true;
    }
  }
  return false;
}

/** 📐️ True when a disposition transform differs from its declared anchor. */
export function dispositionHasEphemeralLayout(transform: DispositionTransform | undefined, anchorRect: DispositionPosition | undefined, measuredNatural: DispositionPosition | undefined, flowLayout: boolean): boolean {
  if (!transform) {
    return false;
  }
  if (!flowLayout) {
    return anchorRect !== undefined && dispositionPositionChanged(anchorRect, transform);
  }
  if (measuredNatural) {
    return dispositionPositionChanged(flowDispositionManipulationRect(measuredNatural, undefined), transform);
  }
  return true;
}

/** 🎯️ Pointer is within the top-right hotspot where the slide reset control lives. */
export function pointerNearSlideResetHotspot(section: HTMLElement, clientX: number, clientY: number, proximityPx = SLIDE_INTERACTION_RESET_PROXIMITY_PX): boolean {
  const bounds = slideLayoutBounds(section);
  const reachX = Math.max(proximityPx, bounds.width * 0.12);
  const reachY = Math.max(proximityPx, bounds.height * 0.12);
  return clientX >= bounds.right - reachX && clientY >= bounds.top && clientY <= bounds.top + reachY;
}

/** 📐️ Flow drag: preserve the wrapper footprint while ink is absolutely positioned inside. */
export function flowDispositionReserveStyle(reservePx: { readonly width: number; readonly height: number }): CSSProperties {
  return {
    minWidth: reservePx.width,
    minHeight: reservePx.height,
  };
}

/** 📐️ Flow transforms store pointer deltas; positioned transforms store slide-space frames. */
export function flowDispositionManipulationRect(measured: DispositionPosition, existing: DispositionPosition | undefined): DispositionPosition {
  if (existing) {
    return existing;
  }
  return { x: 0, y: 0, width: measured.width, height: measured.height };
}

/** 📐️ True when x/y/width/height are a normalized slide-space frame (not pixel drag storage). */
export function isNormalizedSlideFrame(transform: DispositionPosition): boolean {
  return (
    transform.x >= 0 &&
    transform.x <= 1 &&
    transform.y >= 0 &&
    transform.y <= 1 &&
    transform.width > 0 &&
    transform.width <= 1 &&
    transform.height > 0 &&
    transform.height <= 1 &&
    transform.x + transform.width <= 1 + 1e-6 &&
    transform.y + transform.height <= 1 + 1e-6
  );
}

/** ↔ True when a flow transform only stores pixel translate with unchanged measured size. */
export function isFlowPixelOffsetTransform(transform: DispositionPosition, measured: DispositionPosition | undefined): boolean {
  const sizeMatches = measured === undefined ? transform.width > 0 && transform.width <= 1 && transform.height > 0 && transform.height <= 1 : transform.width === measured.width && transform.height === measured.height;
  if (!sizeMatches) {
    return false;
  }
  if (measured !== undefined) {
    if (transform.x === 0 && transform.y === 0) {
      return true;
    }
    return !isNormalizedSlideFrame(transform);
  }
  if (Math.abs(transform.x) > 1 || Math.abs(transform.y) > 1) {
    return true;
  }
  if (transform.x === 0 && transform.y === 0) {
    return true;
  }
  return !isNormalizedSlideFrame(transform);
}

/** 🔍️ Uniform scale for pinned resize so ink zooms inside the frame without reflow overflow. */
export function interactiveDispositionContentScale(transform: DispositionPosition, baseline: DispositionPosition | undefined): number | null {
  if (!baseline || baseline.width <= 0 || baseline.height <= 0) {
    return null;
  }
  if (transform.width <= 0 || transform.height <= 0) {
    return null;
  }
  const scaleX = transform.width / baseline.width;
  const scaleY = transform.height / baseline.height;
  const widthChanged = Math.abs(scaleX - 1) >= 1e-4;
  const heightChanged = Math.abs(scaleY - 1) >= 1e-4;
  if (!widthChanged && !heightChanged) {
    return null;
  }
  if (!widthChanged) {
    return Number.isFinite(scaleY) ? scaleY : null;
  }
  if (!heightChanged) {
    return Number.isFinite(scaleX) ? scaleX : null;
  }
  const uniform = Math.min(scaleX, scaleY);
  return Number.isFinite(uniform) ? uniform : null;
}

/** 🔍️ CSS transform that scales disposition content from its center during interactive resize. */
export function interactiveDispositionContentScaleStyle(scale: number): CSSProperties {
  return {
    transform: `scale(${scale})`,
    transformOrigin: "center center",
    willChange: "transform",
  };
}

/** 📍️ Converts a flow pixel-offset transform into a slide-space frame using measured ink bounds. */
export function flowPixelOffsetToSectionRect(measured: DispositionPosition, transform: DispositionPosition, sectionEl: HTMLElement, transformEl?: HTMLElement): DispositionPosition {
  const el = transformEl ?? flowDragTransformElement(sectionEl, null, null);
  const layout = slideLayoutBounds(sectionEl);
  const scale = elementVisualScale(el);
  const w = layout.width / scale;
  const h = layout.height / scale;
  return {
    x: measured.x + transform.x / w,
    y: measured.y + transform.y / h,
    width: transform.width,
    height: transform.height,
  };
}

export interface PresentationInteractionState {
  readonly selectedIds: ReadonlySet<string>;
  readonly transforms: ReadonlyMap<string, DispositionTransform>;
  readonly enlargedIds: ReadonlySet<string>;
  readonly enlargeStashById: ReadonlyMap<string, DispositionPosition>;
  readonly pdfPageById: ReadonlyMap<string, number>;
  readonly isSelected: (id: string) => boolean;
  readonly isEnlarged: (id: string) => boolean;
  readonly hasPdfPageOverride: (id: string) => boolean;
  readonly getTransform: (id: string) => DispositionTransform | undefined;
  readonly getEnlargeStash: (id: string) => DispositionPosition | undefined;
  readonly getPdfPage: (id: string, defaultPage: number) => number;
  readonly setTransform: (id: string, rect: DispositionTransform) => void;
  readonly setTransforms: (updates: ReadonlyMap<string, DispositionTransform>) => void;
  readonly setPdfPage: (id: string, page: number, defaultPage: number) => void;
  readonly stashEnlargeRect: (id: string, rect: DispositionPosition) => void;
  readonly clearEnlargeStash: (id: string) => void;
  readonly selectIds: (ids: readonly string[], additive: boolean) => void;
  readonly clearSelection: () => void;
  readonly toggleEnlarge: (id: string) => void;
  readonly clearTransform: (id: string) => void;
  readonly resetSlide: (dispositionIds: readonly string[]) => void;
  readonly clearEphemeralLayout: () => void;
}

export const PresentationInteractionContext = createContext<PresentationInteractionState | null>(null);

export function usePresentationInteractionState(): PresentationInteractionState {
  const value = useContext(PresentationInteractionContext);
  if (!value) {
    throw new Error("Presentation interaction requires PresentationInteractionContext.");
  }
  return value;
}
//#endregion 🔖️Interaction
