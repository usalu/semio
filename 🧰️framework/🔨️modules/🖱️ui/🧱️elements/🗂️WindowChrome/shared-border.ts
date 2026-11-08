import * as React from "react";
import { flushSync } from "react-dom";
import { resolveBorderSegments, type BorderState } from "../../🔨️modules/📏️border-presentation/🟦️.ts";
import { sharedBorderAxisScale, sharedOutlineSegments, clipSharedOutlineSegments, visibleSharedOutlineSegments, createSharedBorderJunctionResolver, type SharedOutlineSegment } from "../🔲️WindowSilhouette/shared-border.ts";
import { windowSilhouetteOutline, type WindowSilhouetteGeometry } from "../🔲️WindowSilhouette/🟦️.tsx";
import { useShellScopeOptional } from "../🐚️ShellScope/🟦️.tsx";

export interface SharedBorderPaintSegment {
  readonly d: string;
  readonly state: BorderState;
}
interface Registration {
  element: HTMLElement;
  geometry: WindowSilhouetteGeometry;
  state: BorderState;
  update: (segments: readonly SharedBorderPaintSegment[]) => void;
}
interface Frame {
  id: string;
  svg: SVGSVGElement;
  attached: string | null;
  position?: { value: string; priority: string };
}
const selector = "[data-shared-border-frame], [data-shared-border-edges]";
const namespace = "http://www.w3.org/2000/svg";
const states: readonly BorderState[] = ["normal", "focus", "active", "waiting", "loading", "introduced", "celebrated", "hover"];
const roots = new WeakMap<HTMLElement, { compositor: BorderCompositor; participants: number }>();
let sequence = 0;

/** 🧱️ Realm-independent element test: observer callbacks can outlive the window global that defined `Element`. */
const isElement = (node: unknown): node is Element => typeof node === "object" && node !== null && (node as Node).nodeType === 1;

/** 🛡️ Inline important geometry prevents ancestor icon selectors from shrinking frame paint. */
export function sizeSharedBorderOverlay(svg: SVGSVGElement | null, width: number, height: number): void {
  if (!svg) return;
  for (const [property, value] of Object.entries({ width: `${width}px`, height: `${height}px`, "min-width": "0", "min-height": "0", "max-width": "none", "max-height": "none", "flex-shrink": "0" })) svg.style.setProperty(property, value, "important");
}

/** 🎨️ Shared state paint used by automatic frame overlays as well as window segments. */
export function sharedBorderStroke(state: BorderState): string {
  if (state === "hover") return "var(--hover-base)";
  if (state === "focus") return "var(--active-base)";
  if (state === "active") return "var(--active-base)";
  if (state === "introduced") return "var(--introduced-border-color, var(--color-secondary))";
  if (state === "loading") return "var(--loading-border-color, var(--border-normal-color))";
  if (state === "waiting") return "var(--waiting-border-color, var(--border-normal-color))";
  return "var(--border-normal-color)";
}

/** 🪡️ Root-local resolver; original owners paint only winning portions of their unchanged outline. */
export class BorderCompositor {
  readonly windows = new Map<string, Registration>();
  private frames = new Map<HTMLElement, Frame>();
  private frame = 0;
  private disposed = false;
  private observer: MutationObserver | null = null;
  private resize: ResizeObserver | null = null;
  private hovered: Element | null = null;
  private motion = new Map<Element, Set<string>>();
  constructor(readonly root: HTMLElement) {
    root.setAttribute("data-shared-border-root", "");
    const view = root.ownerDocument.defaultView;
    this.observer = typeof MutationObserver === "undefined" ? null : new MutationObserver(records => {
      if (records.some(record => !(isElement(record.target) && record.target.closest("[data-shared-border-overlay], [data-window-silhouette-border]")) && (record.type !== "childList" || [...record.addedNodes, ...record.removedNodes].some(node => !(isElement(node) && node.hasAttribute("data-shared-border-overlay")))))) this.schedule();
    });
    this.observer?.observe(root, { subtree: true, childList: true, attributes: true, attributeFilter: ["class", "style", "data-active", "data-state", "aria-selected", "aria-pressed", "data-shared-border-state", "data-shared-border-frame", "data-shared-border-edges", "data-introduced", "data-celebrated", "disabled", "aria-disabled"] });
    this.resize = typeof ResizeObserver === "undefined" ? null : new ResizeObserver(this.schedule);
    this.resize?.observe(root);
    root.addEventListener("pointerover", this.pointer);
    root.addEventListener("pointerout", this.pointer);
    root.addEventListener("focusin", this.schedule);
    root.addEventListener("focusout", this.schedule);
    root.addEventListener("transitionrun", this.motionStart);
    root.addEventListener("transitionend", this.motionEnd);
    root.addEventListener("transitioncancel", this.motionEnd);
    view?.addEventListener("resize", this.schedule);
    view?.addEventListener("scroll", this.schedule, true);
  }
  private pointer = (event: Event) => {
    const pointer = event as PointerEvent;
    this.hovered = (event.type === "pointerout" ? pointer.relatedTarget : pointer.target) as Element | null;
    this.schedule();
  };
  private motionStart = (event: Event) => {
    const property = (event as TransitionEvent).propertyName;
    if (!(isElement(event.target)) || !/^(transform|translate|scale|width|height|top|left|right|bottom|flex-basis|grid-template-)/.test(property)) return;
    const properties = this.motion.get(event.target) ?? new Set<string>();
    properties.add(property); this.motion.set(event.target, properties); this.schedule();
  };
  private motionEnd = (event: Event) => {
    if (!(isElement(event.target))) return;
    const properties = this.motion.get(event.target);
    properties?.delete((event as TransitionEvent).propertyName);
    if (!properties?.size) this.motion.delete(event.target);
    this.schedule();
  };
  schedule = () => {
    if (this.disposed) return;
    this.root.setAttribute("data-shared-border-pending", "");
    if (this.frame) return;
    const view = this.root.ownerDocument.defaultView;
    if (view?.requestAnimationFrame) this.frame = view.requestAnimationFrame(this.commit);
    else this.commit();
  };
  private state(element: HTMLElement, fallback: BorderState): BorderState {
    if (this.hovered && element.contains(this.hovered) && !element.matches(":disabled") && !element.closest('[aria-disabled="true"]')) return "hover";
    const explicit = element.getAttribute("data-shared-border-state") as BorderState | null;
    const candidates: BorderState[] = [fallback];
    if (explicit && states.includes(explicit)) candidates.push(explicit);
    if (element.hasAttribute("data-celebrated") && element.getAttribute("data-celebrated") !== "false") candidates.push("celebrated");
    if (element.hasAttribute("data-introduced") && element.getAttribute("data-introduced") !== "false") candidates.push("introduced");
    if (element.classList.contains("border-loading")) candidates.push("loading");
    if (element.classList.contains("border-waiting")) candidates.push("waiting");
    if (element.getAttribute("data-active") === "true" || ["on", "active"].includes(element.getAttribute("data-state") ?? "") || element.getAttribute("aria-selected") === "true" || element.getAttribute("aria-pressed") === "true") candidates.push("active");
    if (element.contains(element.ownerDocument.activeElement)) candidates.push("focus");
    return candidates.reduce((winner, candidate) => states.indexOf(candidate) > states.indexOf(winner) ? candidate : winner);
  }
  private visibleSources(element: HTMLElement, sources: SharedOutlineSegment[]): SharedOutlineSegment[] {
    const clip = { left: -Infinity, top: -Infinity, right: Infinity, bottom: Infinity };
    for (let ancestor = element.parentElement; ancestor; ancestor = ancestor.parentElement) {
      const style = ancestor.ownerDocument.defaultView?.getComputedStyle(ancestor);
      const clipsX = /^(hidden|clip|scroll|auto)$/.test(style?.overflowX || style?.overflow || "");
      const clipsY = /^(hidden|clip|scroll|auto)$/.test(style?.overflowY || style?.overflow || "");
      if (!clipsX && !clipsY) continue;
      this.resize?.observe(ancestor);
      const rect = ancestor.getBoundingClientRect();
      const scaleX = ancestor.offsetWidth ? rect.width / ancestor.offsetWidth : 1;
      const scaleY = ancestor.offsetHeight ? rect.height / ancestor.offsetHeight : 1;
      const left = rect.left + ancestor.clientLeft * scaleX, top = rect.top + ancestor.clientTop * scaleY;
      if (clipsX) { clip.left = Math.max(clip.left, left); clip.right = Math.min(clip.right, left + (ancestor.offsetWidth ? ancestor.clientWidth * scaleX : rect.width)); }
      if (clipsY) { clip.top = Math.max(clip.top, top); clip.bottom = Math.min(clip.bottom, top + (ancestor.offsetHeight ? ancestor.clientHeight * scaleY : rect.height)); }
    }
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
    this.frames.delete(element);
  }
  private commit = () => {
    this.frame = 0;
    let sources: SharedOutlineSegment[] = [];
    const updates = new Map<Registration, readonly SharedBorderPaintSegment[]>();
    const frameSources = new Map<string, { element: HTMLElement; frame: Frame; width: number; height: number; sides: string }>();
    for (const [element, frame] of this.frames) if (!this.root.contains(element) || !element.matches(selector)) this.restore(element, frame);
    for (const [id, entry] of this.windows) {
      const rect = entry.element.getBoundingClientRect();
      if (rect.width <= 0 || rect.height <= 0 || entry.geometry.state !== "ready") { updates.set(entry, []); continue; }
      const metrics = entry.geometry.metrics;
      sources.push(...this.visibleSources(entry.element, sharedOutlineSegments(id, entry.geometry.outline, windowSilhouetteOutline(metrics), this.state(entry.element, entry.state), { left: rect.left, top: rect.top, scaleX: sharedBorderAxisScale(rect.width, entry.element.offsetWidth), scaleY: sharedBorderAxisScale(rect.height, entry.element.offsetHeight) })));
    }
    for (const element of this.root.querySelectorAll<HTMLElement>(selector)) {
      if (element.hasAttribute("data-window-silhouette") || element.closest(".semio-scope") !== this.root.closest(".semio-scope")) continue;
      const rect = element.getBoundingClientRect();
      const scaleX = sharedBorderAxisScale(rect.width, element.offsetWidth), scaleY = sharedBorderAxisScale(rect.height, element.offsetHeight);
      const width = rect.width / scaleX, height = rect.height / scaleY;
      if (rect.width <= 0 || rect.height <= 0 || width <= 2 || height <= 2) { const old = this.frames.get(element); if (old) this.restore(element, old); continue; }
      const sides = element.getAttribute("data-shared-border-edges") ?? "top right bottom left";
      const logical = [{ x: 0, y: 0 }, { x: width, y: 0 }, { x: width, y: height }, { x: 0, y: height }];
      const paint = [{ x: 1, y: 1 }, { x: width - 1, y: 1 }, { x: width - 1, y: height - 1 }, { x: 1, y: height - 1 }];
      let frame = this.frames.get(element);
      if (!frame) {
        const svg = element.ownerDocument.createElementNS(namespace, "svg");
        svg.setAttribute("data-shared-border-overlay", "");
        svg.setAttribute("aria-hidden", "true");
        svg.style.cssText = "position:absolute;inset:0;overflow:visible;pointer-events:none;z-index:40";
        frame = { id: `frame-${++sequence}`, svg, attached: element.getAttribute("data-shared-border-attached") };
        const position = element.ownerDocument.defaultView?.getComputedStyle(element).position;
        if (!position || position === "static") {
          frame.position = { value: element.style.getPropertyValue("position"), priority: element.style.getPropertyPriority("position") };
          element.style.position = "relative";
        }
        this.frames.set(element, frame);
        element.append(svg);
        this.resize?.observe(element);
      }
      const edges = sharedOutlineSegments(frame.id, logical, paint, this.state(element, "normal"), { left: rect.left, top: rect.top, scaleX, scaleY });
      sources.push(...this.visibleSources(element, edges.filter((_, index) => sides.split(/\s+/).includes(["top", "right", "bottom", "left"][index]!))));
      frameSources.set(frame.id, { element, frame, width, height, sides });
    }
    sources = visibleSharedOutlineSegments(sources);
    const byId = new Map(sources.map(source => [source.id, source]));
    const output = new Map<string, SharedBorderPaintSegment[]>();
    const resolved = resolveBorderSegments(sources);
    const junction = createSharedBorderJunctionResolver(sources);
    const interacting = new Set<string>();
    for (const source of sources) {
      const start = junction(source.logicalFrom), end = junction(source.logicalTo), t = source.transform;
      if (start.x !== t.left + source.paintFrom.x * t.scaleX || start.y !== t.top + source.paintFrom.y * t.scaleY || end.x !== t.left + source.paintTo.x * t.scaleX || end.y !== t.top + source.paintTo.y * t.scaleY) interacting.add(source.owner);
    }
    for (const segment of resolved) {
      const source = byId.get(segment.id)!;
      const local = (paint: { x: number; y: number }) => {
        const t = source.transform;
        return { x: (paint.x - t.left) / t.scaleX, y: (paint.y - t.top) / t.scaleY };
      };
      const horizontal = segment.from.y === segment.to.y, normal = horizontal ? "y" : "x";
      const values = segment.contributors.map(id => {
        const edge = byId.get(id)!, t = edge.transform;
        return (horizontal ? t.top : t.left) + edge.paintFrom[normal] * (horizontal ? t.scaleY : t.scaleX);
      });
      const centerline = (Math.min(...values) + Math.max(...values)) / 2;
      const from = junction(segment.from), to = junction(segment.to);
      const originalNormal = (horizontal ? source.transform.top : source.transform.left) + source.paintFrom[normal] * (horizontal ? source.transform.scaleY : source.transform.scaleX);
      if (centerline !== originalNormal || from[normal] !== centerline || to[normal] !== centerline) interacting.add(segment.owner);
      const points = [from, { ...from, [normal]: centerline }, { ...to, [normal]: centerline }, to].filter((point, index, all) => index === 0 || point.x !== all[index - 1]!.x || point.y !== all[index - 1]!.y).map(local);
      const list = output.get(segment.owner) ?? [];
      list.push({ d: points.map((point, index) => `${index ? "L" : "M"} ${point.x} ${point.y}`).join(" "), state: segment.state });
      output.set(segment.owner, list);
    }
    for (const [id, entry] of this.windows) {
      const owned = sources.filter(source => source.owner === id);
      const won = resolved.filter(segment => segment.owner === id);
      const length = (segment: { from: { x: number; y: number }; to: { x: number; y: number } }) => Math.abs(segment.to.x - segment.from.x) + Math.abs(segment.to.y - segment.from.y);
      const complete = owned.length > 0 && Math.abs(owned.reduce((sum, edge) => sum + length(edge), 0) - won.reduce((sum, edge) => sum + length(edge), 0)) < 0.001;
      const sameState = won.length > 0 && won.every(segment => segment.state === won[0]!.state);
      const unclipped = owned.every(source => (source.from.x === source.logicalFrom.x && source.from.y === source.logicalFrom.y && source.to.x === source.logicalTo.x && source.to.y === source.logicalTo.y) || (source.to.x === source.logicalFrom.x && source.to.y === source.logicalFrom.y && source.from.x === source.logicalTo.x && source.from.y === source.logicalTo.y));
      const degenerate = entry.geometry.outline.length !== windowSilhouetteOutline(entry.geometry.metrics).length;
      updates.set(entry, degenerate ? [{ d: entry.geometry.borderPath, state: this.state(entry.element, entry.state) }] : complete && sameState && unclipped && !interacting.has(id) ? [{ d: entry.geometry.borderPath, state: won[0]!.state }] : output.get(id) ?? []);
    }
    for (const [id, { element, frame, width, height, sides }] of frameSources) {
      const computed = element.ownerDocument.defaultView?.getComputedStyle(element);
      frame.svg.style.left = `${-parseFloat(computed?.borderLeftWidth || "0") || 0}px`;
      frame.svg.style.top = `${-parseFloat(computed?.borderTopWidth || "0") || 0}px`;
      sizeSharedBorderOverlay(frame.svg, width, height);
      frame.svg.setAttribute("width", String(width)); frame.svg.setAttribute("height", String(height));
      frame.svg.setAttribute("viewBox", `0 0 ${width} ${height}`);
      const segments = output.get(id) ?? [];
      const signature = JSON.stringify(segments);
      if (frame.svg.getAttribute("data-paint") !== signature) {
        frame.svg.replaceChildren(...segments.map((segment, index) => {
          const path = element.ownerDocument.createElementNS(namespace, "path");
          path.setAttribute("d", segment.d); path.setAttribute("fill", "none");
          path.setAttribute("stroke", sharedBorderStroke(segment.state));
          path.setAttribute("vector-effect", "non-scaling-stroke");
          path.setAttribute("stroke-linecap", "square"); path.setAttribute("stroke-linejoin", "miter");
          path.setAttribute("class", `window-silhouette-border window-silhouette-border-${segment.state}`);
          path.setAttribute("data-shared-border-segment", ""); path.setAttribute("data-shared-border-state", segment.state);
          if (segment.state !== "celebrated") return path;
          path.classList.add("window-silhouette-border-celebrated-mask");
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
          content.className = "window-silhouette-border-celebrated-fill"; content.style.cssText = "width:100%;height:100%";
          fill.append(content); group.append(defs, fill); return group;
        }));
        frame.svg.setAttribute("data-paint", signature);
      }
      if (element.getAttribute("data-shared-border-attached") !== sides) element.setAttribute("data-shared-border-attached", sides);
    }
    if (updates.size) flushSync(() => { for (const [entry, segments] of updates) entry.update(segments); });
    if (!this.frame) this.root.removeAttribute("data-shared-border-pending");
    for (const element of this.motion.keys()) if (!this.root.contains(element)) this.motion.delete(element);
    if (this.motion.size) this.schedule();
  };
  dispose() {
    if (this.disposed) return;
    this.disposed = true;
    this.root.removeAttribute("data-shared-border-root");
    this.root.removeAttribute("data-shared-border-pending");
    this.observer?.disconnect(); this.resize?.disconnect();
    const view = this.root.ownerDocument.defaultView;
    if (this.frame) view?.cancelAnimationFrame(this.frame);
    this.frame = 0;
    for (const [element, frame] of this.frames) this.restore(element, frame);
    this.root.removeEventListener("pointerover", this.pointer); this.root.removeEventListener("pointerout", this.pointer);
    this.root.removeEventListener("focusin", this.schedule); this.root.removeEventListener("focusout", this.schedule);
    this.root.removeEventListener("transitionrun", this.motionStart); this.root.removeEventListener("transitionend", this.motionEnd); this.root.removeEventListener("transitioncancel", this.motionEnd);
    view?.removeEventListener("resize", this.schedule); view?.removeEventListener("scroll", this.schedule, true);
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

/** 🪟️ Connects a silhouette to its shell-local compositor, including portaled descendants. */
export function useSharedWindowBorder(element: HTMLElement | null, geometry: WindowSilhouetteGeometry, state: BorderState): readonly SharedBorderPaintSegment[] | null {
  const scope = useShellScopeOptional();
  const id = React.useId();
  const [segments, setSegments] = React.useState<readonly SharedBorderPaintSegment[] | null>(null);
  const current = React.useRef<BorderCompositor | null>(null);
  const latest = React.useRef({ geometry, state });
  latest.current = { geometry, state };
  React.useLayoutEffect(() => {
    if (!element) return;
    const root = scope?.rootRef?.current ?? element.closest<HTMLElement>(".semio-scope") ?? element.ownerDocument.body;
    const { compositor, release } = retainSharedBorderRoot(root);
    current.current = compositor;
    compositor.windows.set(id, { element, ...latest.current, update: next => setSegments(previous => JSON.stringify(previous) === JSON.stringify(next) ? previous : next) });
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
    current.current?.schedule();
  }, [geometry, id, state]);
  return segments;
}
