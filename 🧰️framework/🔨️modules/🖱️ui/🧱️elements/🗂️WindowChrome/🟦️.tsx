// #region 🧲️Header
// 💻️ framework/ui/elements/🗂️WindowChrome/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { deviceSnappedRect, paintSharedWindowBorder, useSharedWindowBorder, sizeSharedBorderOverlay, type SharedBorderPaintSegment } from "./shared-border.ts";
import { reactHostPort } from "../🔌️Ports/⚛️react/🟦️.ts";
import { surfaceLevelAbove as windowFrameChipLevel, SurfaceScope, surfaceFillClass, useLevel, useSurface, type Level, type SurfaceActiveBindProps, type SurfaceFill } from "../🌈️Surface/🟦️.tsx";
import {
  WINDOW_FRAME_CHIP_EPSILON,
  createWindowFrameGeometry,
  normalizeWindowFrameChips,
  type WindowFrameChip,
  type WindowFrameEdge,
  type WindowFrameGeometry,
  type WindowFrameMetrics,
} from "../🔲️WindowFrame/🟦️.tsx";
import { sharedBorderAxisScale as windowFrameScale } from "../🔲️WindowFrame/shared-border.ts";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { modeDockTabClassName } from "../../🔨️modules/🎛️chrome-control-presentation/🟦️.ts";
import { interactiveHoverClass } from "../../🔨️modules/🖱️interaction-presentation/🟦️.ts";
import { glassClass } from "../../🔨️modules/🌈️surface-presentation/🟦️.ts";
// #endregion 🔌️Adapters

export { useSharedBorderFrameRef, useSharedBorderBox, sharedBorderlessBoxProps } from "./shared-border.ts";

//#region 🗂️WindowFrameBorder
/** 🖌️ All border effects the frame border SVG can paint. */
export const WINDOW_FRAME_BORDER_KINDS = ["celebrated", "introduced", "loading", "waiting", "active", "normal"] as const;

/** 🎨️ Which border effect the frame border overlay should paint. */
export type WindowFrameBorderKind = (typeof WINDOW_FRAME_BORDER_KINDS)[number];

/** 🎯️ Whether an introduced stamp is the window chrome body itself (kind/instance scroll surface or
 * `[data-slot="window"]`), not a nested utility/action/tree row inside the pane. Window frame pulse
 * and the stack SVG border must follow only these stamps — introducing `transform` must pulse the utility
 * toggle, not the enclosing Top/Perspective frame. */
export function isWindowChromeIntroducedTarget(el: Element): boolean {
  if (el.getAttribute("data-slot") === "window") return true;
  const ids = [el.getAttribute("id") ?? "", ...(el.getAttribute("data-element-alias") ?? "").split(/\s+/)].filter(Boolean);
  for (const id of ids) {
    if (!id.startsWith("framework.window.")) continue;
    const rest = id.slice("framework.window.".length);
    if (!rest.includes(".")) return true;
  }
  return false;
}

/** 🏁️ Resolves frame border kind from the active window + stack active flag.
 * Introduction stamps `data-introduced` on the window kind id target — often the inner scroll surface
 * (`framework.window.{kind}`), not `[data-slot="window"]` itself — so window-chrome descendants count.
 * Nested introduce targets (utilities, actions) must not promote the window frame. `celebrated`
 * (from `celebrateElements()`) is checked FIRST: it follows an introduced stamp being cleared on the
 * same target, and completion feedback must win during any overlap. */
export function resolveWindowFrameBorderKind(windowEl: Element | null, stackActive = false): WindowFrameBorderKind {
  if (windowEl?.getAttribute("data-celebrated") === "true") return "celebrated";
  if (windowEl) {
    for (const el of windowEl.querySelectorAll('[data-celebrated="true"]')) {
      if (isWindowChromeIntroducedTarget(el)) return "celebrated";
    }
  }
  if (windowEl?.getAttribute("data-introduced") === "true") return "introduced";
  if (windowEl) {
    for (const el of windowEl.querySelectorAll('[data-introduced="true"]')) {
      if (isWindowChromeIntroducedTarget(el)) return "introduced";
    }
  }
  const className = windowEl && typeof windowEl.className === "string" ? windowEl.className : "";
  if (/(?:^|\s)border-loading(?:-active|-element)?(?:\s|$)/.test(className)) return "loading";
  if (/(?:^|\s)border-waiting(?:-active|-element)?(?:\s|$)/.test(className)) return "waiting";
  return stackActive ? "active" : "normal";
}

/** 🗺️ Maps a frame border kind to stroke classes and color tokens. */
export function windowFrameBorderPaint(kind: WindowFrameBorderKind): { readonly className: string; readonly stroke: string } {
  switch (kind) {
    case "celebrated":
      return { className: "window-frame-border window-frame-border-celebrated-mask", stroke: "white" };
    case "introduced":
      return { className: "window-frame-border window-frame-border-introduced", stroke: "var(--introduced-border-color, var(--color-secondary))" };
    case "loading":
      return { className: "window-frame-border window-frame-border-loading", stroke: "var(--loading-border-color, var(--border-normal-color))" };
    case "waiting":
      return { className: "window-frame-border window-frame-border-waiting", stroke: "var(--waiting-border-color, var(--border-normal-color))" };
    case "active":
      return { className: "window-frame-border window-frame-border-active", stroke: "var(--active-base)" };
    case "normal":
      return { className: "window-frame-border window-frame-border-normal", stroke: "var(--border-normal-color)" };
  }
}

const WINDOW_CHROME_CAP_SELECTOR = '[data-slot="window-chrome-cap"], [data-slot="window-chrome-footer"], [data-slot="mode-dock-tabbar"]';

/** 🪟️ Whether a stack-local rect sits on the given frame dock edge. */
function windowFrameRectOnDock(stackRect: DOMRect, rect: DOMRect, dock: "top" | "bottom"): boolean {
  return dock === "top" ? rect.top - stackRect.top <= WINDOW_FRAME_CHIP_EPSILON : stackRect.bottom - rect.bottom <= WINDOW_FRAME_CHIP_EPSILON;
}

/** 🥞️ Whether `element` belongs to `stack`'s own chrome — nested pane/panel `[data-window-frame]` hosts (e.g. projection) keep their chips out of the enclosing frame's clearances. */
function windowFrameOwnsElement(stack: HTMLElement, element: Element): boolean {
  const owner = element.closest("[data-window-frame]");
  return owner === null || owner === stack;
}

/** 📊️ Reads live frame metrics from painted chips grouped by `data-dock` (works for RTL caps and bottom-docked panels), in the stack's own pixels even while a host scales it on screen — see {@link windowFrameScale}. Nested frame chips are ignored — see {@link windowFrameOwnsElement}.
 *
 * @see ../🔲️WindowFrame/frame.cases.json */
export function measureWindowFrameMetrics(stack: HTMLElement): WindowFrameMetrics | null {
  const stackRect = deviceSnappedRect(stack);
  const scale = { x: windowFrameScale(stackRect.width, stack.offsetWidth), y: windowFrameScale(stackRect.height, stack.offsetHeight) };
  const width = stackRect.width / scale.x;
  const height = stackRect.height / scale.y;
  if (width <= 0 || height <= 0) return null;
  const measureEdge = (dock: "top" | "bottom"): WindowFrameEdge => {
    const chips: WindowFrameChip[] = [];
    let depth = 0;
    for (const chip of stack.querySelectorAll<HTMLElement>(`[data-window-frame-chip][data-dock="${dock}"]`)) {
      if (!windowFrameOwnsElement(stack, chip)) continue;
      const rect = deviceSnappedRect(chip);
      if (rect.width <= WINDOW_FRAME_CHIP_EPSILON || rect.height <= WINDOW_FRAME_CHIP_EPSILON) continue;
      chips.push({ left: (rect.left - stackRect.left) / scale.x, right: (rect.right - stackRect.left) / scale.x });
      depth = Math.max(depth, rect.height / scale.y);
    }
    for (const cap of stack.querySelectorAll<HTMLElement>(WINDOW_CHROME_CAP_SELECTOR)) {
      if (!windowFrameOwnsElement(stack, cap)) continue;
      const capRect = deviceSnappedRect(cap);
      if (capRect.height > WINDOW_FRAME_CHIP_EPSILON && windowFrameRectOnDock(stackRect, capRect, dock)) depth = Math.max(depth, capRect.height / scale.y);
    }
    return { depth, chips: normalizeWindowFrameChips(chips, 0, width) };
  };
  return { width, height, top: measureEdge("top"), bottom: measureEdge("bottom") };
}

/** 📐️ Measures the current chip layout before paint, then coalesces later geometry changes for the clearances, the chip regions and the borders. */
export function useWindowFrameGeometry(stack: HTMLElement | null, enabled = true, chipOnly = false): WindowFrameGeometry {
  const [geometry, setGeometry] = reactHostPort.useState<WindowFrameGeometry>(() => createWindowFrameGeometry(null));
  reactHostPort.useLayoutEffect(() => {
    if (!stack || !enabled) return;
    let frame = 0;
    const commit = () => {
      frame = 0;
      const next = createWindowFrameGeometry(measureWindowFrameMetrics(stack));
      setGeometry((previous) => (previous.state === next.state && JSON.stringify(previous.metrics) === JSON.stringify(next.metrics) ? previous : next));
    };
    const schedule = () => {
      if (frame) return;
      if (typeof requestAnimationFrame === "function") frame = requestAnimationFrame(commit);
      else commit();
    };
    const targetSelector = `[data-window-frame-chip], ${WINDOW_CHROME_CAP_SELECTOR}`;
    const resizeObserver = typeof ResizeObserver === "undefined" ? null : new ResizeObserver(schedule);
    const refreshResizeTargets = () => {
      resizeObserver?.disconnect();
      resizeObserver?.observe(stack);
      for (const element of stack.querySelectorAll<HTMLElement>(targetSelector)) {
        if (windowFrameOwnsElement(stack, element)) resizeObserver?.observe(element);
      }
    };
    const containsGeometryTarget = (node: globalThis.Node): boolean => node instanceof Element && (node.matches(targetSelector) || node.querySelector(targetSelector) !== null);
    const mutationObserver =
      typeof MutationObserver === "undefined"
        ? null
        : new MutationObserver((records) => {
            const changed = records.some((record) =>
              record.type === "attributes"
                ? (record.target === stack && record.attributeName === "data-frame-remeasure") || (record.target instanceof Element && (record.target.matches(targetSelector) || record.target.closest(targetSelector) !== null))
                : [...record.addedNodes, ...record.removedNodes].some(containsGeometryTarget),
            );
            if (!changed) return;
            refreshResizeTargets();
            schedule();
          });
    commit();
    refreshResizeTargets();
    stack.addEventListener("scroll", schedule, true);
    mutationObserver?.observe(stack, { attributes: true, attributeFilter: ["data-dock", "data-frame-remeasure", "data-slot", "data-window-frame-chip"], childList: true, subtree: true });
    return () => {
      if (frame && typeof cancelAnimationFrame === "function") cancelAnimationFrame(frame);
      resizeObserver?.disconnect();
      mutationObserver?.disconnect();
      stack.removeEventListener("scroll", schedule, true);
    };
  }, [enabled, stack, chipOnly]);
  return geometry;
}

/** 🧊 Chip surface: a pointer-active glass cell inside the frame. The level is stamped on the chip by {@link windowFrameChipLevel}, one above the frame's content surface. */
export const windowCapFrameClass = "pointer-events-auto relative border-0 bg-transparent";

/** 🔲️ Maximize/controls glass cell — host stamps {@link glassClass}. */
export const windowControlsCapClass = "pointer-events-auto relative flex shrink-0 items-stretch border-0 bg-transparent text-element";

export { windowFrameChipLevel };

/** 🗂️ `WindowChrome`'s own internal stacking, as inline z-indexes.
 *
 * Tailwind ARBITRARY z utilities (`z-[2]`/`z-[1]`) are not emitted by this design system's stylesheet, so the order is stated inline: the chip rows float above the content plane (the content scrolls beneath them) and the border overlay sits above both. */
export const WINDOW_CHROME_CHIP_ROW_STYLE: React.CSSProperties = { zIndex: 2 };
/** 🪜️ The content plane, one level below every chip row — see {@link WINDOW_CHROME_CHIP_ROW_STYLE}. */
export const WINDOW_CHROME_BODY_PLANE_STYLE: React.CSSProperties = { zIndex: 1 };
//#endregion 🗂️WindowFrameBorder

//#region 🗂️WindowChrome
/** 🪟️ True beneath a {@link WindowChrome} body: the enclosing frame already draws the outline of any window it hosts, so that window draws none of its own. */
export const WindowFrameHostContext = reactHostPort.createContext(false);

/** 🎛️ Optional right-cap control on {@link WindowChrome} (enlarge / close). */
export interface WindowChromeControlAction {
  readonly id: string;
  readonly slot: string;
  readonly icon: React.ReactNode;
  readonly label: string;
  readonly onClick: () => void;
}

/** 🔤️ Title chip in the window-chrome chip row (name + optional drag) — transparent and
 * borderless so the painted chip glass shows through and the frame border remains the sole outline. */
export const windowChromeTitleChipClass = cn(modeDockTabClassName, "relative z-30 box-border min-h-chip-row shrink-0 border-0 bg-transparent");

export interface WindowChromeProps {
  readonly active?: boolean;
  readonly chipOnly?: boolean;
  readonly className?: string;
  readonly stackClassName?: string;
  readonly bodyClassName?: string;
  /** 🎨️ Fill painted by the frame beneath its content. */
  readonly bodySurfaceFill?: SurfaceFill | "inherit";
  readonly bodySurfaceLevel?: Level;
  /** 🎈️ Sets the frame level and supplies the content level when no body override is given. */
  readonly level?: Level;
  readonly style?: React.CSSProperties;
  readonly stackRef?: React.Ref<HTMLDivElement>;
  readonly capRef?: React.Ref<HTMLDivElement>;
  readonly bodyRef?: React.Ref<HTMLDivElement>;
  readonly stackSlot?: string;
  readonly bodySlot?: string;
  readonly bodyStyle?: React.CSSProperties;
  readonly titleChips?: React.ReactNode;
  /** 🧭️ Optional top-right chip content rendered ahead of enlarge/close in the controls cell. */
  readonly capRightChips?: React.ReactNode;
  readonly body?: React.ReactNode;
  readonly enlarge?: WindowChromeControlAction;
  readonly close?: WindowChromeControlAction;
  readonly footerLeftChips?: React.ReactNode;
  readonly footerCenterChips?: React.ReactNode;
  readonly footerRightChips?: React.ReactNode;
  readonly footerRef?: React.Ref<HTMLDivElement>;
  readonly introduceTarget?: Element | null;
  /** 🎓️ Force frame border kind (e.g. introduction steps pulse like `data-introduced` until activated). */
  readonly borderKind?: WindowFrameBorderKind;
  readonly stackBindProps?: SurfaceActiveBindProps;
  readonly stackDataAttrs?: Record<string, string | undefined>;
  /** 💸️ Chip row shrink-wraps to its chips (context menus). */
  readonly capFitContent?: boolean;
  /** 🔗️ Which frame edge the chip row docks to — `"bottom"` for panels that grow upward from a bottom anchor. */
  readonly capDock?: "top" | "bottom";
  /** ↔ Inline layout overrides for the chip row (e.g. chrome-hosted trailing navbar reserve). */
  readonly capRowStyle?: React.CSSProperties;
  readonly capSlot?: string;
  readonly chipSlot?: string;
  readonly controlsSlot?: string;
  readonly frameBorderSlot?: string;
}

/** 🪵️ SVG overlay that paints the frame rectangle and the inner chip edges of any window-chrome stack. */
export const WindowChromeFrameBorder: React.FC<{
  readonly stack: HTMLElement | null;
  readonly geometry?: WindowFrameGeometry;
  readonly active?: boolean;
  readonly introduceTarget?: Element | null;
  readonly borderKind?: WindowFrameBorderKind;
  readonly frameBorderSlot?: string;
}> = ({ stack, geometry, active = false, introduceTarget, borderKind, frameBorderSlot = "window-chrome-frame-border" }) => {
  const [epoch, setEpoch] = reactHostPort.useState(0);
  const celebrateMaskId = `window-frame-celebrate-${reactHostPort.useId().replace(/:/g, "")}`;
  const observedGeometry = useWindowFrameGeometry(stack, geometry === undefined);
  const resolvedGeometry = geometry ?? observedGeometry;

  reactHostPort.useLayoutEffect(() => {
    if (!stack) return;
    const bump = () => setEpoch((value) => value + 1);
    bump();
    const mutationObserver = new MutationObserver(bump);
    mutationObserver.observe(stack, { attributes: true, attributeFilter: ["class", "data-celebrated", "data-introduced"], subtree: true });
    return () => {
      mutationObserver.disconnect();
    };
  }, [stack]);
  const windowEl = stack?.querySelector('[data-slot="window"]') ?? introduceTarget ?? stack;
  const kind = resolveWindowFrameBorderKind(windowEl, active);
  const resolvedKind =
    stack && [...stack.querySelectorAll('[data-celebrated="true"]')].some(isWindowChromeIntroducedTarget)
      ? "celebrated"
      : borderKind
        ? borderKind
        : stack && [...stack.querySelectorAll('[data-introduced="true"]')].some(isWindowChromeIntroducedTarget)
          ? "introduced"
          : kind;
  const metrics = resolvedGeometry.metrics;
  const overlayWidth = metrics.width, overlayHeight = metrics.height;
  const overlay = reactHostPort.useRef<SVGSVGElement | null>(null);
  const resolved = reactHostPort.useRef<readonly SharedBorderPaintSegment[] | null>(null);
  const live = reactHostPort.useRef({ fallback: resolvedGeometry.borderPath, kind: resolvedKind });
  live.current = { fallback: resolvedGeometry.borderPath, kind: resolvedKind };
  const sizeOverlay = React.useCallback((svg: SVGSVGElement | null) => {
    overlay.current = svg;
    sizeSharedBorderOverlay(svg, overlayWidth, overlayHeight);
  }, [overlayWidth, overlayHeight]);
  const paint = React.useCallback((segments: readonly SharedBorderPaintSegment[]) => {
    resolved.current = segments;
    if (overlay.current) paintSharedWindowBorder(overlay.current, segments, live.current.fallback, live.current.kind);
  }, []);
  useSharedWindowBorder(stack, resolvedGeometry, resolvedKind, paint);
  reactHostPort.useLayoutEffect(() => {
    if (overlay.current) paintSharedWindowBorder(overlay.current, resolved.current, resolvedGeometry.borderPath, resolvedKind);
  });
  void epoch;

  if (resolvedGeometry.state === "pending") {
    return <div data-slot={frameBorderSlot} data-window-frame-border data-kind={resolvedKind} data-pending="" data-dim="" aria-hidden className="pointer-events-none absolute inset-0 z-[40] overflow-visible" />;
  }
  const kindPaint = windowFrameBorderPaint(resolvedKind);
  if (resolvedKind === "celebrated") {
    return (
      <svg
        ref={sizeOverlay}
        data-slot={frameBorderSlot}
        data-window-frame-border
        data-kind={resolvedKind}
        data-dim=""
        className="pointer-events-none absolute inset-0 z-[40] overflow-visible"
        width={metrics.width}
        height={metrics.height}
        viewBox={`0 0 ${metrics.width} ${metrics.height}`}
        aria-hidden
      >
        <defs>
          <mask id={celebrateMaskId} maskUnits="userSpaceOnUse" x={0} y={0} width={metrics.width} height={metrics.height}>
            <rect x={0} y={0} width={metrics.width} height={metrics.height} fill="black" />
            <path data-shared-border-main="" fill="none" stroke={kindPaint.stroke} strokeLinejoin="miter" vectorEffect="non-scaling-stroke" className={kindPaint.className} />
          </mask>
        </defs>
        <foreignObject x={0} y={0} width={metrics.width} height={metrics.height} mask={`url(#${celebrateMaskId})`}>
          <div className="window-frame-border-celebrated-fill" style={{ width: "100%", height: "100%" }} />
        </foreignObject>
        <g data-shared-border-extra="" />
      </svg>
    );
  }
  return (
    <svg
      ref={sizeOverlay}
      data-slot={frameBorderSlot}
      data-window-frame-border
      data-kind={resolvedKind}
      data-dim=""
      className="pointer-events-none absolute inset-0 z-[40] overflow-visible"
      width={metrics.width}
      height={metrics.height}
      viewBox={`0 0 ${metrics.width} ${metrics.height}`}
      aria-hidden
    >
      <path data-shared-border-main="" fill="none" stroke={kindPaint.stroke} strokeLinejoin="miter" strokeLinecap="square" vectorEffect="non-scaling-stroke" className={kindPaint.className} data-shared-border-segment="" data-shared-border-state={resolvedKind} />
      <g data-shared-border-extra="" />
    </svg>
  );
};

/** 🟢️ Window chrome: one rectangular frame whose content plane fills it edge to edge. The chip row (title chip, optional controls) and the footer row are absolutely positioned overlays inside the frame, above the content; each chip carries its own glass one level above the content surface, and the space of a row no chip covers belongs to the content. */
export const WindowChrome = reactHostPort.forwardRef<HTMLDivElement, WindowChromeProps>(
  (
    {
      active = false,
      chipOnly = false,
      className,
      stackClassName,
      bodyClassName,
      bodySurfaceFill: requestedFill,
      bodySurfaceLevel,
      level,
      style,
      stackRef,
      capRef,
      bodyRef,
      stackSlot = "window-chrome-stack",
      bodySlot = "window-chrome-body",
      bodyStyle,
      titleChips,
      capRightChips,
      body,
      enlarge,
      close,
      footerLeftChips,
      footerCenterChips,
      footerRightChips,
      footerRef,
      introduceTarget,
      borderKind,
      stackBindProps,
      stackDataAttrs,
      capFitContent = false,
      capDock = "top",
      capRowStyle,
      capSlot = "window-chrome-cap",
      chipSlot = "window-chrome-chip-cap",
      controlsSlot = "window-chrome-controls",
      frameBorderSlot = "window-chrome-frame-border",
    },
    ref,
  ) => {
    const [stackEl, setStackEl] = reactHostPort.useState<HTMLDivElement | null>(null);
    const setStackRef = reactHostPort.useCallback(
      (element: HTMLDivElement | null) => {
        setStackEl(element);
        if (typeof ref === "function") ref(element);
        else if (ref) (ref as React.MutableRefObject<HTMLDivElement | null>).current = element;
        if (typeof stackRef === "function") stackRef(element);
        else if (stackRef) (stackRef as React.MutableRefObject<HTMLDivElement | null>).current = element;
      },
      [ref, stackRef],
    );

    const inheritedLevel = useLevel();
    const owner = useSurface();
    const contentLevel = requestedFill === "inherit" ? inheritedLevel : bodySurfaceLevel ?? level ?? inheritedLevel;
    const bodySurfaceFill = requestedFill ?? (contentLevel === "window" || contentLevel === "base" ? "surface" : "glass");
    const chipLevel = windowFrameChipLevel(contentLevel);
    const chipSurfaceClass = cn(windowCapFrameClass, glassClass);
    const bodyFill = bodySurfaceFill === "inherit" ? owner?.fill ?? "none" : bodySurfaceFill;
    const bodySurfaceClass = cn(bodySurfaceFill === "inherit" ? undefined : surfaceFillClass(bodySurfaceFill), "pointer-events-none absolute inset-0 z-0 border-0");
    const controlsSurfaceClass = cn(windowControlsCapClass, glassClass);
    const setCapRef = reactHostPort.useCallback(
      (element: HTMLDivElement | null) => {
        if (typeof capRef === "function") capRef(element);
        else if (capRef) (capRef as React.MutableRefObject<HTMLDivElement | null>).current = element;
      },
      [capRef],
    );
    const setFooterRef = reactHostPort.useCallback(
      (element: HTMLDivElement | null) => {
        if (typeof footerRef === "function") footerRef(element);
        else if (footerRef) (footerRef as React.MutableRefObject<HTMLDivElement | null>).current = element;
      },
      [footerRef],
    );
    const geometry = useWindowFrameGeometry(stackEl, true, chipOnly);
    const frameVars = {
      "--window-frame-top-clearance": `${geometry.safeClearances.top}px`,
      "--window-frame-bottom-clearance": `${geometry.safeClearances.bottom}px`,
    } as React.CSSProperties;
    const chipContent = (node: React.ReactNode) => <SurfaceScope level={chipLevel} fill="glass">{node}</SurfaceScope>;

    if (chipOnly) {
      return (
        <div ref={setStackRef} data-slot={stackSlot} data-window-frame data-frame-state={geometry.state} data-level={level} className={cn("relative inline-flex min-w-0 bg-transparent", className, stackClassName)} style={style} {...stackDataAttrs}>
          <WindowChromeFrameBorder stack={stackEl} geometry={geometry} active={active} borderKind={borderKind} frameBorderSlot={frameBorderSlot} />
          {titleChips ? (
            <div data-slot={chipSlot} data-window-frame-chip data-dock={capDock} data-level={chipLevel} data-ui-reveal-region="window-cap" data-dim className={cn("relative flex min-h-chip-row min-w-0 shrink items-stretch", chipSurfaceClass)}>
              {chipContent(titleChips)}
            </div>
          ) : null}
        </div>
      );
    }

    const hasFooter = Boolean(footerLeftChips || footerCenterChips || footerRightChips);
    const footerChipClass = cn("relative flex min-h-chip-row min-w-0 shrink items-stretch", chipSurfaceClass);
    const capRow = (
        <div
          ref={setCapRef}
          data-slot={capSlot}
          data-dock={capDock}
          data-ui-reveal-region="window-cap"
          data-dim
          className={cn("pointer-events-none absolute inset-x-0 flex min-w-0 items-stretch justify-between bg-transparent", capDock === "bottom" ? "bottom-0" : "top-0", capFitContent ? "w-fit max-w-full" : "w-full")}
          style={{ ...capRowStyle, ...WINDOW_CHROME_CHIP_ROW_STYLE }}
        >
          {titleChips ? (
            <div data-slot={chipSlot} data-window-frame-chip data-chip-reach="next" data-dock={capDock} data-level={chipLevel} className={cn("relative flex min-h-chip-row min-w-0 shrink items-stretch", chipSurfaceClass)}>
              {chipContent(titleChips)}
            </div>
          ) : null}
          {capRightChips || enlarge || close ? (
            <div data-slot={controlsSlot} data-window-frame-chip data-dock={capDock} data-level={chipLevel} className={cn("ms-auto flex shrink-0 items-stretch", controlsSurfaceClass)}>
              <SurfaceScope level={chipLevel} fill="glass">
                {capRightChips}
                {enlarge ? (
                  <button
                    type="button"
                    id={enlarge.id}
                    data-slot={enlarge.slot}
                    className={cn("flex min-h-chip-row w-auto items-center justify-center self-stretch border-0 bg-transparent transition-colors px-single gap-single text-element", interactiveHoverClass)}
                    onClick={enlarge.onClick}
                  >
                    {enlarge.icon}
                    <span className="text-tiny whitespace-nowrap">{enlarge.label}</span>
                  </button>
                ) : null}
                {close ? (
                  <button
                    type="button"
                    id={close.id}
                    data-slot={close.slot}
                    className={cn("flex min-h-chip-row w-auto items-center justify-center self-stretch border-0 bg-transparent transition-colors px-single gap-single text-element", interactiveHoverClass)}
                    onClick={close.onClick}
                  >
                    {close.icon}
                    <span className="text-tiny whitespace-nowrap">{close.label}</span>
                  </button>
                ) : null}
              </SurfaceScope>
            </div>
          ) : null}
          </div>
    );
    return (
      <div
        ref={setStackRef}
        data-slot={stackSlot}
        data-window-frame
        data-frame-state={geometry.state}
        data-level={level}
        data-active={active ? "true" : undefined}
        className={cn("relative flex min-h-0 min-w-0 flex-col overflow-clip bg-transparent text-foreground", stackClassName, className)}
        style={{ ...style, ...frameVars }}
        {...stackBindProps}
        {...stackDataAttrs}
      >
        <WindowChromeFrameBorder stack={stackEl} geometry={geometry} active={active} introduceTarget={introduceTarget} borderKind={borderKind} frameBorderSlot={frameBorderSlot} />
        {bodySurfaceFill !== "inherit" && <div data-slot="window-chrome-body-surface" data-level={contentLevel} data-dim aria-hidden className={bodySurfaceClass} />}
        {capDock === "bottom" ? null : capRow}
        <div
          ref={bodyRef}
          data-slot={bodySlot}
          data-level={contentLevel}
          data-window-frame-content
          data-frame-state={geometry.state}
          data-dim
          className={cn("window-frame-content-plane relative min-h-0 flex-1 border-0", bodyClassName)}
          style={{ ...bodyStyle, ...WINDOW_CHROME_BODY_PLANE_STYLE }}
        >
          <SurfaceScope level={contentLevel} fill={bodyFill}>
            <WindowFrameHostContext.Provider value={true}>{body}</WindowFrameHostContext.Provider>
          </SurfaceScope>
        </div>
        {capDock === "bottom" ? capRow : null}
        {hasFooter ? (
          footerCenterChips ? (
            <div ref={setFooterRef} data-slot="window-chrome-footer" data-dock="bottom" data-dim className="pointer-events-none absolute inset-x-0 bottom-0 grid w-full min-w-0 grid-cols-[minmax(0,1fr)_auto_minmax(0,1fr)] items-stretch bg-transparent" style={WINDOW_CHROME_CHIP_ROW_STYLE}>
              {footerLeftChips ? (
                <div data-slot="window-chrome-footer-left" data-window-frame-chip data-chip-reach="next" data-dock="bottom" data-level={chipLevel} className={cn("col-start-1 justify-self-start", footerChipClass)}>
                  {chipContent(footerLeftChips)}
                </div>
              ) : null}
              <div data-slot="window-chrome-footer-center" data-window-frame-chip data-dock="bottom" data-level={chipLevel} className={cn("col-start-2 justify-self-center", footerChipClass)}>
                {chipContent(footerCenterChips)}
              </div>
              {footerRightChips ? (
                <div data-slot="window-chrome-footer-right" data-window-frame-chip data-chip-reach="prev" data-dock="bottom" data-level={chipLevel} className={cn("col-start-3 justify-self-end", footerChipClass)}>
                  {chipContent(footerRightChips)}
                </div>
              ) : null}
            </div>
          ) : (
            <div
              ref={setFooterRef}
              data-slot="window-chrome-footer"
              data-dock="bottom"
              data-dim
              className="pointer-events-none absolute inset-x-0 bottom-0 flex w-full min-w-0 items-stretch justify-between bg-transparent"
              style={WINDOW_CHROME_CHIP_ROW_STYLE}
            >
              {footerLeftChips ? (
                <div data-slot="window-chrome-footer-left" data-window-frame-chip data-chip-reach="next" data-dock="bottom" data-level={chipLevel} className={footerChipClass}>
                  {chipContent(footerLeftChips)}
                </div>
              ) : null}
              {footerRightChips ? (
                <div data-slot="window-chrome-footer-right" data-window-frame-chip data-chip-reach="prev" data-dock="bottom" data-level={chipLevel} className={cn("ms-auto", footerChipClass)}>
                  {chipContent(footerRightChips)}
                </div>
              ) : null}
            </div>
          )
        ) : null}
      </div>
    );
  },
);
WindowChrome.displayName = "WindowChrome";
//#endregion 🗂️WindowChrome
