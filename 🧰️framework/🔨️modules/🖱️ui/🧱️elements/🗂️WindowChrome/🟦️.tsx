// #region 🧲️Header
// 💻️ framework/ui/elements/🗂️WindowChrome/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { reactHostPort } from "../🔌️Ports/🟦️.tsx";
import { SurfaceScope, type Level, type SurfaceActiveBindProps } from "../🌈️Surface/🟦️.tsx";
import {
  WINDOW_SILHOUETTE_CHIP_EPSILON,
  createWindowSilhouetteGeometry,
  normalizeWindowSilhouetteChips,
  type WindowSilhouetteChip,
  type WindowSilhouetteEdge,
  type WindowSilhouetteGeometry,
  type WindowSilhouetteMetrics,
} from "../🔲️WindowSilhouette/🟦️.tsx";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { modeDockTabClassName } from "../../🔨️modules/🎛️chrome-control-presentation/🟦️.ts";
import { interactiveHoverClass } from "../../🔨️modules/🖱️interaction-presentation/🟦️.ts";
import { glassClass } from "../../🔨️modules/🌈️surface-presentation/🟦️.ts";
// #endregion 🔌️Adapters

//#region 🗂️WindowSilhouetteBorder
/** @emoji 🪟️ All border effects the silhouette SVG can paint. */
export const WINDOW_SILHOUETTE_BORDER_KINDS = ["celebrated", "introduced", "loading", "waiting", "active", "normal"] as const;

/** @emoji 🪟️ Which border effect the dock-stack silhouette overlay should paint. */
export type WindowSilhouetteBorderKind = (typeof WINDOW_SILHOUETTE_BORDER_KINDS)[number];

/** @emoji 🪟️ Whether an introduced stamp is the window chrome body itself (kind/instance scroll surface or
 * `[data-slot="window"]`), not a nested utility/action/tree row inside the pane. Window silhouette pulse
 * and the stack SVG border must follow only these stamps — introducing `transform` must pulse the utility
 * toggle, not the enclosing Top/Perspective silhouette. */
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

/** @emoji 🪟️ Resolves silhouette border kind from the active window + stack active flag.
 * Introduction stamps `data-introduced` on the window kind id target — often the inner scroll surface
 * (`framework.window.{kind}`), not `[data-slot="window"]` itself — so window-chrome descendants count.
 * Nested introduce targets (utilities, actions) must not promote the window silhouette. `celebrated`
 * (from `celebrateElements()`) is checked FIRST: it follows an introduced stamp being cleared on the
 * same target, and completion feedback must win during any overlap. */
export function resolveWindowSilhouetteBorderKind(windowEl: Element | null, stackActive = false): WindowSilhouetteBorderKind {
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

/** @emoji 🪟️ Maps a silhouette border kind to stroke classes and color tokens. */
export function windowSilhouetteBorderPaint(kind: WindowSilhouetteBorderKind): { readonly className: string; readonly stroke: string } {
  switch (kind) {
    case "celebrated":
      return { className: "window-silhouette-border window-silhouette-border-celebrated-mask", stroke: "white" };
    case "introduced":
      return { className: "window-silhouette-border window-silhouette-border-introduced", stroke: "var(--introduced-border-color, var(--color-secondary))" };
    case "loading":
      return { className: "window-silhouette-border window-silhouette-border-loading", stroke: "var(--loading-border-color, var(--border-normal-color))" };
    case "waiting":
      return { className: "window-silhouette-border window-silhouette-border-waiting", stroke: "var(--waiting-border-color, var(--border-normal-color))" };
    case "active":
      return { className: "window-silhouette-border window-silhouette-border-active", stroke: "var(--active-base)" };
    case "normal":
      return { className: "window-silhouette-border window-silhouette-border-normal", stroke: "var(--border-normal-color)" };
  }
}

const WINDOW_CHROME_GAP_SELECTOR = '[data-slot="window-chrome-gap"], [data-slot="mode-dock-tab-gap"]';
const WINDOW_CHROME_CAP_SELECTOR = '[data-slot="window-chrome-cap"], [data-slot="mode-dock-tabbar"]';

/** @emoji 🪟️ Whether a stack-local rect sits on the given silhouette dock edge. */
function windowSilhouetteRectOnDock(stackRect: DOMRect, rect: DOMRect, dock: "top" | "bottom"): boolean {
  return dock === "top" ? rect.top - stackRect.top <= WINDOW_SILHOUETTE_CHIP_EPSILON : stackRect.bottom - rect.bottom <= WINDOW_SILHOUETTE_CHIP_EPSILON;
}

/** @emoji 🪟️ Whether `element` belongs to `stack`'s own chrome — nested pane/panel `[data-window-silhouette]` hosts (e.g. projection) keep their chips out of the enclosing window outline so the window bottom stays rectangular while those panes overlay like window options. */
function windowSilhouetteOwnsElement(stack: HTMLElement, element: Element): boolean {
  const owner = element.closest("[data-window-silhouette]");
  return owner === null || owner === stack;
}

/** @emoji 🔍️ How much an element is scaled on screen along one axis: its painted size over its layout size — 1 while they agree to a pixel (layout sizes are whole pixels) or it has no layout size (jsdom). A stack inside a scaled host (a page on the grid of a layered overview) is measured in its own pixels, so its outline stays put when the host scales. */
function windowSilhouetteScale(painted: number, layout: number): number {
  return layout > 0 && Math.abs(painted - layout) > 1 ? painted / layout : 1;
}

/** @emoji 🖱️ Reads fused submenu wing rects for a context menu stack, in the stack's own pixels (`scale` from {@link windowSilhouetteScale}). */
function measureContextMenuFusion(stack: HTMLElement, stackRect: DOMRect, base: WindowSilhouetteMetrics, scale: { readonly x: number; readonly y: number }): WindowSilhouetteMetrics {
  const fusionBody = stack.querySelector<HTMLElement>('[data-slot="context-menu-fusion-body"]');
  if (!fusionBody) return base;
  const primary = fusionBody.querySelector<HTMLElement>('[data-slot="context-menu-fusion-primary"]');
  if (!primary) return base;
  const wings = [...fusionBody.querySelectorAll<HTMLElement>('[data-slot="context-menu-submenu"]')];
  if (wings.length === 0) return base;
  const toLocal = (rect: DOMRect) => ({
    left: (rect.left - stackRect.left) / scale.x,
    top: (rect.top - stackRect.top) / scale.y,
    right: (rect.right - stackRect.left) / scale.x,
    bottom: (rect.bottom - stackRect.top) / scale.y,
  });
  const fusion = { primary: toLocal(primary.getBoundingClientRect()), wings: wings.map((wing) => toLocal(wing.getBoundingClientRect())) };
  let width = base.width;
  let height = base.height;
  for (const wing of fusion.wings) {
    width = Math.max(width, wing.right);
    height = Math.max(height, wing.bottom);
  }
  return { ...base, width, height, contextMenuFusion: fusion };
}

/** @emoji 🪟️ Reads live silhouette metrics from painted chip spans grouped by `data-dock` (works for RTL caps and bottom-docked panels), in the stack's own pixels even while a host scales it on screen — see {@link windowSilhouetteScale}. Nested silhouette chips are ignored — see {@link windowSilhouetteOwnsElement}. */
export function measureWindowSilhouetteMetrics(stack: HTMLElement): WindowSilhouetteMetrics | null {
  const stackRect = stack.getBoundingClientRect();
  const scale = { x: windowSilhouetteScale(stackRect.width, stack.offsetWidth), y: windowSilhouetteScale(stackRect.height, stack.offsetHeight) };
  const width = stackRect.width / scale.x;
  const height = stackRect.height / scale.y;
  if (width <= 0 || height <= 0) return null;
  const measureEdge = (dock: "top" | "bottom"): WindowSilhouetteEdge => {
    const chips: WindowSilhouetteChip[] = [];
    let depth = 0;
    for (const chip of stack.querySelectorAll<HTMLElement>(`[data-window-silhouette-chip][data-dock="${dock}"]`)) {
      if (!windowSilhouetteOwnsElement(stack, chip)) continue;
      const rect = chip.getBoundingClientRect();
      if (rect.width <= WINDOW_SILHOUETTE_CHIP_EPSILON || rect.height <= WINDOW_SILHOUETTE_CHIP_EPSILON) continue;
      chips.push({ left: (rect.left - stackRect.left) / scale.x, right: (rect.right - stackRect.left) / scale.x });
      depth = Math.max(depth, rect.height / scale.y);
    }
    for (const gap of stack.querySelectorAll<HTMLElement>(WINDOW_CHROME_GAP_SELECTOR)) {
      if (!windowSilhouetteOwnsElement(stack, gap)) continue;
      const gapRect = gap.getBoundingClientRect();
      if (gapRect.height > WINDOW_SILHOUETTE_CHIP_EPSILON && windowSilhouetteRectOnDock(stackRect, gapRect, dock)) depth = Math.max(depth, gapRect.height / scale.y);
    }
    for (const cap of stack.querySelectorAll<HTMLElement>(WINDOW_CHROME_CAP_SELECTOR)) {
      if (!windowSilhouetteOwnsElement(stack, cap)) continue;
      const capRect = cap.getBoundingClientRect();
      if (capRect.height > WINDOW_SILHOUETTE_CHIP_EPSILON && windowSilhouetteRectOnDock(stackRect, capRect, dock)) depth = Math.max(depth, capRect.height / scale.y);
    }
    return { depth, chips: normalizeWindowSilhouetteChips(chips, 0, width) };
  };
  const base = { width, height, top: measureEdge("top"), bottom: measureEdge("bottom") };
  return measureContextMenuFusion(stack, stackRect, base, scale);
}

/** @emoji 📐️ Coalesced owned-chip measurement shared by silhouette content, glass, border, and hit clipping. */
export function useWindowSilhouetteGeometry(stack: HTMLElement | null, enabled = true): WindowSilhouetteGeometry {
  const [geometry, setGeometry] = reactHostPort.useState<WindowSilhouetteGeometry>(() => createWindowSilhouetteGeometry(null));
  reactHostPort.useLayoutEffect(() => {
    if (!stack || !enabled) return;
    let frame = 0;
    const commit = () => {
      frame = 0;
      const next = createWindowSilhouetteGeometry(measureWindowSilhouetteMetrics(stack));
      setGeometry((previous) =>
        previous.state === next.state && previous.contentClipPath === next.contentClipPath && previous.borderPath === next.borderPath && previous.metrics.width === next.metrics.width && previous.metrics.height === next.metrics.height
          ? previous
          : next,
      );
    };
    const schedule = () => {
      if (frame) return;
      if (typeof requestAnimationFrame === "function") frame = requestAnimationFrame(commit);
      else commit();
    };
    const targetSelector = '[data-window-silhouette-chip], [data-slot="window-chrome-cap"], [data-slot="mode-dock-tabbar"], [data-slot="context-menu-fusion-body"], [data-slot="context-menu-submenu"]';
    const resizeObserver = typeof ResizeObserver === "undefined" ? null : new ResizeObserver(schedule);
    const refreshResizeTargets = () => {
      resizeObserver?.disconnect();
      resizeObserver?.observe(stack);
      for (const element of stack.querySelectorAll<HTMLElement>(targetSelector)) {
        if (windowSilhouetteOwnsElement(stack, element)) resizeObserver?.observe(element);
      }
    };
    const containsGeometryTarget = (node: globalThis.Node): boolean => node instanceof Element && (node.matches(targetSelector) || node.querySelector(targetSelector) !== null);
    const mutationObserver =
      typeof MutationObserver === "undefined"
        ? null
        : new MutationObserver((records) => {
            const changed = records.some((record) =>
              record.type === "attributes"
                ? (record.target === stack && record.attributeName === "data-silhouette-remeasure") || (record.target instanceof Element && (record.target.matches(targetSelector) || record.target.closest(targetSelector) !== null))
                : [...record.addedNodes, ...record.removedNodes].some(containsGeometryTarget),
            );
            if (!changed) return;
            refreshResizeTargets();
            schedule();
          });
    commit();
    refreshResizeTargets();
    mutationObserver?.observe(stack, { attributes: true, attributeFilter: ["data-dock", "data-silhouette-remeasure", "data-slot", "data-window-silhouette-chip"], childList: true, subtree: true });
    return () => {
      if (frame && typeof cancelAnimationFrame === "function") cancelAnimationFrame(frame);
      resizeObserver?.disconnect();
      mutationObserver?.disconnect();
    };
  }, [enabled, stack]);
  return geometry;
}

/** @emoji 📏️ Tab/gap/controls cells stay transparent; glass lives on chip (+ controls) cells only so the U-gap punches through to the base floor. Borders owned by {@link ModeDockStackSilhouetteBorder}. */
export const windowCapFrameClass = "relative z-[2] border-0 bg-transparent";

/** @emoji 🪟️ Gap cutout stays clear — never glass — so the base/canvas floor shows through the U-notch. */
export const windowGapFrameClass = "border-0 bg-transparent";

/** @emoji 📏️ Maximize/controls glass cell — host stamps {@link glassClass}; fill must not span the U-gap. */
export const windowControlsCapClass = "pointer-events-auto relative z-[2] flex shrink-0 items-stretch border-0 bg-transparent text-element";

/** @emoji 🪜️ `WindowChrome`'s own internal stacking, as inline z-indexes.
 *
 * The chrome's chip rows carried Tailwind ARBITRARY z utilities (`z-[2]`/`z-[1]`), and this design
 * system's stylesheet does not emit those — measured on a served boot, cap row, body plane and footer all
 * computed `z-index: auto`, so DOM order decided instead. The body plane negative-margins UP over the cap
 * row by design (`window-silhouette-content-plane`, so the silhouette outline can wrap the chips), which
 * made it the topmost element across the whole cap row: every cap-row control became unreachable by any
 * hit-tested press. The tour's own `ui.introduction.skip` was one of them, while its veil held
 * `pointer-events: auto` over the entire application (26/09/02/PUZZLE-3D-END-TO-END wave B49 §1).
 * Stated inline, the order cannot silently vanish with a utility that was never generated. */
export const WINDOW_CHROME_CHIP_ROW_STYLE: React.CSSProperties = { zIndex: 2 };
/** @emoji 🪜️ The body plane, one level below every chip row — see {@link WINDOW_CHROME_CHIP_ROW_STYLE}. */
export const WINDOW_CHROME_BODY_PLANE_STYLE: React.CSSProperties = { zIndex: 1 };
//#endregion 🗂️WindowSilhouetteBorder

//#region 🗂️WindowChrome
/** @emoji 🪟️ Optional right-cap control on {@link WindowChrome} (enlarge / close). */
export interface WindowChromeControlAction {
  readonly id: string;
  readonly slot: string;
  readonly icon: React.ReactNode;
  readonly label: string;
  readonly onClick: () => void;
}

/** @emoji 🪟️ Title chip in the window-chrome cap row (name + optional drag) — transparent and
 * borderless so the painted chip-cap cell shows through and the silhouette remains the sole outline. */
export const windowChromeTitleChipClass = cn(modeDockTabClassName, "relative z-30 box-border min-h-medium shrink-0 border-0 bg-transparent");

export interface WindowChromeProps {
  readonly active?: boolean;
  readonly chipOnly?: boolean;
  readonly className?: string;
  readonly stackClassName?: string;
  readonly bodyClassName?: string;
  readonly bodySurfaceClassName?: string;
  readonly bodySurfaceLevel?: Level;
  /** 🎈️ Stamps `data-level={level}` on the chrome stack and wraps its content in a {@link LevelProvider}; cap/controls/body all render {@link glassClass} so one level is one appearance. */
  readonly level?: Level;
  readonly style?: React.CSSProperties;
  readonly stackRef?: React.Ref<HTMLDivElement>;
  readonly capRef?: React.Ref<HTMLDivElement>;
  readonly bodyRef?: React.Ref<HTMLDivElement>;
  readonly stackSlot?: string;
  readonly bodySlot?: string;
  readonly bodyStyle?: React.CSSProperties;
  readonly titleChips?: React.ReactNode;
  /** @emoji 🧭️ Optional top-right chip content rendered ahead of enlarge/close in the controls cell. */
  readonly capRightChips?: React.ReactNode;
  readonly body?: React.ReactNode;
  readonly enlarge?: WindowChromeControlAction;
  readonly close?: WindowChromeControlAction;
  readonly gapProps?: React.HTMLAttributes<HTMLDivElement>;
  readonly footerLeftChips?: React.ReactNode;
  readonly footerCenterChips?: React.ReactNode;
  readonly footerRightChips?: React.ReactNode;
  readonly footerGapProps?: React.HTMLAttributes<HTMLDivElement>;
  readonly footerRef?: React.Ref<HTMLDivElement>;
  readonly introduceTarget?: Element | null;
  /** 🎓️ Force silhouette border kind (e.g. introduction steps pulse like `data-introduced` until activated). */
  readonly borderKind?: WindowSilhouetteBorderKind;
  readonly stackBindProps?: SurfaceActiveBindProps;
  readonly stackDataAttrs?: Record<string, string | undefined>;
  /** @emoji 📐️ Cap row shrink-wraps to title chip + gap (context menus with fused wings). */
  readonly capFitContent?: boolean;
  /** @emoji 🧭️ Which silhouette edge the cap row docks to — `"bottom"` for panels that grow upward from a bottom anchor. */
  readonly capDock?: "top" | "bottom";
  /** @emoji ↔ Inline layout overrides for the cap row (e.g. chrome-hosted trailing navbar reserve). */
  readonly capRowStyle?: React.CSSProperties;
  readonly capSlot?: string;
  readonly chipSlot?: string;
  readonly controlsSlot?: string;
  readonly silhouetteSlot?: string;
}

/** @emoji 🪟️ SVG overlay that paints the U-cutout silhouette for any window-chrome stack. */
export const WindowChromeSilhouetteBorder: React.FC<{
  readonly stack: HTMLElement | null;
  readonly geometry?: WindowSilhouetteGeometry;
  readonly active?: boolean;
  readonly introduceTarget?: Element | null;
  readonly borderKind?: WindowSilhouetteBorderKind;
  readonly silhouetteSlot?: string;
}> = ({ stack, geometry, active = false, introduceTarget, borderKind, silhouetteSlot = "window-chrome-silhouette-border" }) => {
  const [epoch, setEpoch] = reactHostPort.useState(0);
  const celebrateMaskId = `window-silhouette-celebrate-${reactHostPort.useId().replace(/:/g, "")}`;
  const observedGeometry = useWindowSilhouetteGeometry(stack, geometry === undefined);
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
  const kind = resolveWindowSilhouetteBorderKind(windowEl, active);
  const resolvedKind =
    stack && [...stack.querySelectorAll('[data-celebrated="true"]')].some(isWindowChromeIntroducedTarget)
      ? "celebrated"
      : borderKind
        ? borderKind
        : stack && [...stack.querySelectorAll('[data-introduced="true"]')].some(isWindowChromeIntroducedTarget)
          ? "introduced"
          : kind;
  const metrics = resolvedGeometry.metrics;
  void epoch;

  if (resolvedGeometry.state === "pending") {
    return <div data-slot={silhouetteSlot} data-window-silhouette-border data-kind={resolvedKind} data-pending="" data-dim="" aria-hidden className="pointer-events-none absolute inset-0 z-[40] overflow-visible" />;
  }
  const path = resolvedGeometry.borderPath;
  const paint = windowSilhouetteBorderPaint(resolvedKind);
  if (resolvedKind === "celebrated") {
    return (
      <svg
        data-slot={silhouetteSlot}
        data-window-silhouette-border
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
            <path d={path} fill="none" stroke={paint.stroke} strokeLinejoin="miter" vectorEffect="non-scaling-stroke" className={paint.className} />
          </mask>
        </defs>
        <foreignObject x={0} y={0} width={metrics.width} height={metrics.height} mask={`url(#${celebrateMaskId})`}>
          <div className="window-silhouette-border-celebrated-fill" style={{ width: "100%", height: "100%" }} />
        </foreignObject>
      </svg>
    );
  }
  return (
    <svg
      data-slot={silhouetteSlot}
      data-window-silhouette-border
      data-kind={resolvedKind}
      data-dim=""
      className="pointer-events-none absolute inset-0 z-[40] overflow-visible"
      width={metrics.width}
      height={metrics.height}
      viewBox={`0 0 ${metrics.width} ${metrics.height}`}
      aria-hidden
    >
      <path d={path} fill="none" stroke={paint.stroke} strokeLinejoin="miter" vectorEffect="non-scaling-stroke" className={paint.className} />
    </svg>
  );
};

/** @emoji 🪟️ Shared U-cutout window chrome: left title chip(s), open gap, optional enlarge/close, continuous body border.
 * Cap glass lives only on the chip (+ controls) cells — never the full cap row — so the U-gap stays transparent
 * and shows whatever sits behind the stack (veil, canvas, page). Do not paint an absolute inset fill. */
export const WindowChrome = reactHostPort.forwardRef<HTMLDivElement, WindowChromeProps>(
  (
    {
      active = false,
      chipOnly = false,
      className,
      stackClassName,
      bodyClassName,
      bodySurfaceClassName,
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
      gapProps,
      footerLeftChips,
      footerCenterChips,
      footerRightChips,
      footerGapProps,
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
      silhouetteSlot = "window-chrome-silhouette-border",
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

    const chipSurfaceClass = cn(windowCapFrameClass, glassClass);
    const bodySurfaceClass = cn("pointer-events-none absolute inset-x-0 z-0 border-0", bodySurfaceClassName ?? glassClass);
    const bodyContentClass = "window-silhouette-content-plane relative border-0";
    const controlsSurfaceClass = cn(windowControlsCapClass, glassClass);
    const geometry = useWindowSilhouetteGeometry(stackEl);
    const silhouetteVars = {
      "--window-silhouette-top-clearance": `${geometry.safeClearances.top}px`,
      "--window-silhouette-bottom-clearance": `${geometry.safeClearances.bottom}px`,
    } as React.CSSProperties;
    const contentStyle = {
      ...bodyStyle,
      clipPath: geometry.contentClipPath,
      WebkitClipPath: geometry.contentClipPath,
    } as React.CSSProperties;
    // 🪟️ The stack element itself stamps `data-level` (below), so this only needs to open the
    // SurfaceScope (fill="glass" — every cell above already renders it) for descendants to see via useSurface().
    const wrapLevel = (node: React.ReactNode): React.ReactNode =>
      level ? (
        <SurfaceScope level={level} fill="glass">
          {node}
        </SurfaceScope>
      ) : (
        node
      );

    if (chipOnly) {
      return wrapLevel(
        <div ref={setStackRef} data-slot={stackSlot} data-window-silhouette data-level={level} className={cn("relative inline-flex min-w-0 bg-transparent", className, stackClassName)} style={style} {...stackDataAttrs}>
          <WindowChromeSilhouetteBorder stack={stackEl} geometry={geometry} active={active} borderKind={borderKind} silhouetteSlot={silhouetteSlot} />
          {titleChips ? (
            <div data-slot={chipSlot} data-window-silhouette-chip data-dock={capDock} data-ui-reveal-region="window-cap" data-dim className={cn("relative flex min-h-medium min-w-0 shrink items-stretch", chipSurfaceClass)}>
              {titleChips}
            </div>
          ) : null}
        </div>,
      );
    }

    const { className: gapClassName, ...gapRest } = gapProps ?? {};
    const { className: footerGapClassName, ...footerGapRest } = footerGapProps ?? {};
    const hasFooter = Boolean(footerLeftChips || footerCenterChips || footerRightChips);
    const hasFooterGap = Boolean(footerLeftChips && footerRightChips && !footerCenterChips);
    const footerGapClass = cn("pointer-events-none relative min-h-0 min-w-0 bg-transparent", windowGapFrameClass, footerGapClassName);
    const footerChipClass = cn("relative flex min-h-medium min-w-0 shrink-0 items-stretch", chipSurfaceClass);
    return wrapLevel(
      <div
        ref={setStackRef}
        data-slot={stackSlot}
        data-window-silhouette
        data-level={level}
        data-active={active ? "true" : undefined}
        className={cn("relative flex min-h-0 min-w-0 flex-col overflow-visible bg-transparent text-foreground", capDock === "bottom" && "flex-col-reverse", stackClassName, className)}
        style={{ ...style, ...silhouetteVars }}
        {...stackBindProps}
        {...stackDataAttrs}
      >
        <WindowChromeSilhouetteBorder stack={stackEl} geometry={geometry} active={active} introduceTarget={introduceTarget} borderKind={borderKind} silhouetteSlot={silhouetteSlot} />
        <div
          ref={capRef}
          data-slot={capSlot}
          data-ui-reveal-region="window-cap"
          data-dim
          className={cn("relative flex min-w-0 shrink-0 items-stretch bg-transparent", capFitContent ? "w-fit max-w-full" : "w-full")}
          style={{ ...capRowStyle, ...WINDOW_CHROME_CHIP_ROW_STYLE }}
        >
          {titleChips ? (
            <div data-slot={chipSlot} data-window-silhouette-chip data-dock={capDock} className={cn("relative flex min-h-medium min-w-0 shrink items-stretch", chipSurfaceClass)}>
              {titleChips}
            </div>
          ) : null}
          <div data-slot="window-chrome-gap" data-window-silhouette-gap aria-hidden {...gapRest} className={cn("pointer-events-none relative min-h-medium min-w-0 flex-1 bg-transparent", windowGapFrameClass, gapClassName)} />
          {capRightChips || enlarge || close ? (
            <div data-slot={controlsSlot} data-window-silhouette-chip data-dock={capDock} className={cn("relative z-[2] flex shrink-0 items-stretch", controlsSurfaceClass)}>
              {capRightChips}
              {enlarge ? (
                <button
                  type="button"
                  id={enlarge.id}
                  data-slot={enlarge.slot}
                  className={cn("flex h-medium w-auto items-center justify-center border-0 bg-transparent transition-colors px-single gap-single text-element", interactiveHoverClass)}
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
                  className={cn("flex h-medium w-auto items-center justify-center border-0 bg-transparent transition-colors px-single gap-single text-element", interactiveHoverClass)}
                  onClick={close.onClick}
                >
                  {close.icon}
                  <span className="text-tiny whitespace-nowrap">{close.label}</span>
                </button>
              ) : null}
            </div>
          ) : null}
        </div>
        {geometry.bodyRegion ? (
          <div
            data-slot="window-chrome-body-surface"
            data-level={bodySurfaceLevel ?? level}
            data-dim
            aria-hidden
            className={bodySurfaceClass}
            style={{ top: geometry.bodyRegion.y, bottom: geometry.metrics.height - geometry.bodyRegion.y - geometry.bodyRegion.height }}
          />
        ) : null}
        <div
          ref={bodyRef}
          data-slot={bodySlot}
          data-level={bodySurfaceLevel ?? level}
          data-window-silhouette-content
          data-silhouette-state={geometry.state}
          data-dim
          className={cn("min-h-0 flex-1", bodyContentClass, bodyClassName)}
          style={{ ...contentStyle, ...WINDOW_CHROME_BODY_PLANE_STYLE }}
        >
          {body}
        </div>
        {hasFooter ? (
          footerCenterChips ? (
            <div ref={footerRef} data-slot="window-chrome-footer" data-dim className="relative grid w-full min-w-0 shrink-0 grid-cols-[minmax(0,1fr)_auto_minmax(0,1fr)] items-end bg-transparent" style={WINDOW_CHROME_CHIP_ROW_STYLE}>
              <div data-slot="window-chrome-footer-gap-left" data-window-silhouette-gap {...footerGapRest} aria-hidden={footerLeftChips ? undefined : true} className={cn(footerGapClass, "justify-self-start")}>
                {footerLeftChips ? (
                  <div data-slot="window-chrome-footer-left" data-window-silhouette-chip data-dock="bottom" className={cn("pointer-events-auto", footerChipClass)}>
                    {footerLeftChips}
                  </div>
                ) : null}
              </div>
              <div data-slot="window-chrome-footer-center" className={cn(footerGapClass, "flex justify-center justify-self-center")}>
                <div data-slot="window-chrome-footer-center-chip" data-window-silhouette-chip data-dock="bottom" className={cn("pointer-events-auto", footerChipClass)}>
                  {footerCenterChips}
                </div>
              </div>
              <div data-slot="window-chrome-footer-gap-right" data-window-silhouette-gap aria-hidden={footerRightChips ? undefined : true} className={cn(footerGapClass, "justify-self-end")}>
                {footerRightChips ? (
                  <div data-slot="window-chrome-footer-right" data-window-silhouette-chip data-dock="bottom" className={cn("pointer-events-auto", footerChipClass)}>
                    {footerRightChips}
                  </div>
                ) : null}
              </div>
            </div>
          ) : (
            <div
              ref={footerRef}
              data-slot="window-chrome-footer"
              data-dim
              className={cn("relative flex w-full min-w-0 shrink-0 items-stretch bg-transparent", footerLeftChips && !footerRightChips && "justify-start", footerRightChips && !footerLeftChips && "justify-end")}
              style={WINDOW_CHROME_CHIP_ROW_STYLE}
            >
              {footerLeftChips ? (
                <div data-slot="window-chrome-footer-left" data-window-silhouette-chip data-dock="bottom" className={cn("relative flex min-h-medium min-w-0 shrink items-stretch", chipSurfaceClass)}>
                  {footerLeftChips}
                </div>
              ) : null}
              {hasFooterGap ? <div data-slot="window-chrome-footer-gap" data-window-silhouette-gap aria-hidden {...footerGapRest} className={cn(footerGapClass, "flex-1")} /> : null}
              {footerRightChips ? (
                <div data-slot="window-chrome-footer-right" data-window-silhouette-chip data-dock="bottom" className={cn("relative flex min-h-medium min-w-0 shrink items-stretch", chipSurfaceClass)}>
                  {footerRightChips}
                </div>
              ) : null}
            </div>
          )
        ) : null}
      </div>,
    );
  },
);
WindowChrome.displayName = "WindowChrome";
//#endregion 🗂️WindowChrome
