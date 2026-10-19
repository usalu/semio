/** 🛟 Owns the chrome panel safe-area bookkeeping and the shell navbar trailing reserve that panels yield to. */
import { uiSpacingPx } from "@semio-tech/ui-styling";
import { ephemeralMap } from "../../../🫧️transient/🟦️.ts";
import * as React from "react";
import { reactHostPort } from "../../🧱️elements/🔌️Ports/⚛️react/🟦️.ts";
import { useShellScopeOptional } from "../../🧱️elements/🐚️ShellScope/🟦️.tsx";
import { resolveElementsSurfaceChromeRoot } from "@semio-tech/ui-react/appearance";
import { type Anchor, anchorHorizontal, anchorVertical } from "../⚓️panel-anchor/🟦️.ts";

// 🐚️ Keyed by shell root (falling back to `document.documentElement` outside any shell) so two navbars
// of different widths — one per shell — never overwrite each other's measured reserve.
export const shellNavbarTrailingEndWidthByRoot = ephemeralMap<HTMLElement, number>("framework.modules.ui.packages.typescript.targets.react.index.tsx.shellNavbarTrailingEndWidthByRoot");

export const shellNavbarTrailingEndWidthListenersByRoot = ephemeralMap<HTMLElement, Set<() => void>>("framework.modules.ui.packages.typescript.targets.react.index.tsx.shellNavbarTrailingEndWidthListenersByRoot");

/** ↔ Measured trailing navbar chrome width (fullscreen toggle footprint) for this shell. */
export function useShellNavbarTrailingEndWidthPx(root?: HTMLElement): number {
  const key = resolveElementsSurfaceChromeRoot(root);
  return reactHostPort.useSyncExternalStore(
    (onStoreChange) => {
      if (!key) return () => {};
      let listeners = shellNavbarTrailingEndWidthListenersByRoot.get(key);
      if (!listeners) {
        listeners = new Set();
        shellNavbarTrailingEndWidthListenersByRoot.set(key, listeners);
      }
      listeners.add(onStoreChange);
      return () => listeners.delete(onStoreChange);
    },
    () => (key ? (shellNavbarTrailingEndWidthByRoot.get(key) ?? 0) : 0),
    () => 0,
  );
}

/** ↔ Inline cap-row reserve that clears trailing navbar controls for chrome-hosted right panels. */
export function shellNavbarTrailingEndReserveStyle(widthPx: number): React.CSSProperties | undefined {
  if (widthPx <= 0) return undefined;
  return { paddingInlineStart: `${widthPx + uiSpacingPx(1)}px` };
}

/** 🧢 One OPEN chrome-hosted dock's cap row: it sizes to its CONTENT and never below the body it
 * caps, so the fold control is laid out BESIDE the tab strip instead of underneath it.
 *
 * The cap row is a flex row of `[tab strip | gap | controls]` inside a panel root pinned to the body
 * width, and the strip is `flex: 0 1 auto` over a container with visible overflow. Pinned to the body,
 * a strip whose tabs need more room than is left after the fold control and the navbar reserve keeps
 * its own content width and simply PAINTS outside the box flex gave it — over the controls, which carry
 * `z-[2]` and therefore win the hit test. Measured on generation3d 6018 at 1600×1000
 * (`🐍️panel-tab-occlusion-recon.mjs`, `🗑️generated/react-reds/recon/recon.json`): cap `[1297,3,300,22]`,
 * chip box 144 px wide for a 238 px strip, fold `[1297,3,64,22]` — so the Inspection tab
 * `[1267,2,92,22]` had the fold button at its own centre and a user clicking the middle of that tab
 * COLLAPSED the dock instead of opening Inspection. A chrome-hosted cap lives in the shell chrome band
 * with free space beside it, so growing along the inline axis is what the band is for; `minWidth: 100%`
 * keeps the row flush with its body whenever the strip does fit. */
export function chromeHostedPanelCapRowStyle(anchor: Anchor, trailingEndWidthPx: number): React.CSSProperties {
  const reserve = anchor === "top-right" ? shellNavbarTrailingEndReserveStyle(trailingEndWidthPx) : undefined;
  return { ...reserve, width: "max-content", minWidth: "100%" };
}

/** 🛟️ An axis-aligned viewport box in CSS pixels — the one geometry currency of the safe area,
 * so a caller can state a box it has not laid out yet ({@link Window}'s right-edge chrome column) as
 * readily as one it measured ({@link safeAreaBoxFromRect}). */
export interface SafeAreaBox {
  readonly left: number;
  readonly top: number;
  readonly right: number;
  readonly bottom: number;
}

/** 🛟️ One open anchored chrome {@link Panel}'s occupied viewport box, with the anchor it grew from. */
export interface ChromePanelOccupancy {
  readonly key: string;
  readonly anchor: Anchor;
  readonly box: SafeAreaBox;
}

/** 🛟️ Which axis an in-window affordance is free to yield on. Floating panes and content overlays take
 * `"either"`, i.e. whichever single axis clears the chrome panel with the smaller displacement. */
export type SafeAreaYield = "inline" | "block" | "either";

/** 🛟️ How far an affordance must move off its anchor's own two edges to clear the chrome panels
 * over it — at most one axis is ever non-zero. */
export interface ChromePanelSafeArea {
  readonly inlinePx: number;
  readonly blockPx: number;
}

const CHROME_PANEL_SAFE_AREA_CLEAR: ChromePanelSafeArea = Object.freeze({ inlinePx: 0, blockPx: 0 });

const NO_CHROME_PANEL_OCCUPANCIES: readonly ChromePanelOccupancy[] = Object.freeze([]);

/** 🛟️ A measured DOM rect as a {@link SafeAreaBox}, rounded to whole pixels so a sub-pixel
 * reflow of an unchanged panel never re-publishes and never re-renders its readers. */
export function safeAreaBoxFromRect(rect: DOMRect): SafeAreaBox {
  return { left: Math.round(rect.left), top: Math.round(rect.top), right: Math.round(rect.right), bottom: Math.round(rect.bottom) };
}

function safeAreaBoxesEqual(a: SafeAreaBox, b: SafeAreaBox): boolean {
  return a.left === b.left && a.top === b.top && a.right === b.right && a.bottom === b.bottom;
}

function safeAreaBoxesOverlap(a: SafeAreaBox, b: SafeAreaBox): boolean {
  return a.left < b.right && a.right > b.left && a.top < b.bottom && a.bottom > b.top;
}

// 🐚️ Keyed by shell root (falling back to `document.documentElement` outside any shell), one entry per
// open anchored chrome panel — all eight anchors can be open at once, and two shells on one page must
// never overwrite each other's measured boxes.
const shellChromePanelBoxesByRoot = ephemeralMap<HTMLElement, Map<string, ChromePanelOccupancy>>("framework.modules.ui.packages.typescript.targets.react.index.tsx.shellChromePanelBoxesByRoot");

const shellChromePanelSnapshotByRoot = ephemeralMap<HTMLElement, readonly ChromePanelOccupancy[]>("framework.modules.ui.packages.typescript.targets.react.index.tsx.shellChromePanelSnapshotByRoot");

const shellChromePanelListenersByRoot = ephemeralMap<HTMLElement, Set<() => void>>("framework.modules.ui.packages.typescript.targets.react.index.tsx.shellChromePanelListenersByRoot");

/** 🛟️ Publishes (or, with `box: null`, retracts) one open anchored chrome panel's occupied
 * viewport box. An anchored panel floats OVER the canvas region in the app root's stacking context (see
 * {@link anchorPositionStyle} and `📐️Layout`), so it is the only party that knows the box it occupies —
 * window content painted under it cannot derive that from its own layout, and cannot win the corner back
 * on z-index either, since the two live in different stacking contexts. This registry is the single
 * publication of that geometry; {@link chromePanelSafeArea} is the single rule read off it. */
export function publishShellChromePanelBox(root: HTMLElement | undefined, key: string, anchor: Anchor, box: SafeAreaBox | null): void {
  const rootKey = resolveElementsSurfaceChromeRoot(root);
  if (!rootKey) return;
  let entries = shellChromePanelBoxesByRoot.get(rootKey);
  if (!entries) {
    if (box === null) return;
    entries = new Map();
    shellChromePanelBoxesByRoot.set(rootKey, entries);
  }
  if (box === null) {
    if (!entries.delete(key)) return;
  } else {
    const published = entries.get(key);
    if (published && published.anchor === anchor && safeAreaBoxesEqual(published.box, box)) return;
    entries.set(key, { key, anchor, box });
  }
  shellChromePanelSnapshotByRoot.set(rootKey, entries.size === 0 ? NO_CHROME_PANEL_OCCUPANCIES : Object.freeze([...entries.values()]));
  for (const listener of shellChromePanelListenersByRoot.get(rootKey) ?? []) listener();
}

/** 🛟️ Every open anchored chrome panel of this shell, as the boxes they occupy — the single
 * observable any in-window affordance reserves itself against. */
export function useShellChromePanelBoxes(root?: HTMLElement): readonly ChromePanelOccupancy[] {
  const rootKey = resolveElementsSurfaceChromeRoot(root);
  return reactHostPort.useSyncExternalStore(
    (onStoreChange) => {
      if (!rootKey) return () => {};
      let listeners = shellChromePanelListenersByRoot.get(rootKey);
      if (!listeners) {
        listeners = new Set();
        shellChromePanelListenersByRoot.set(rootKey, listeners);
      }
      listeners.add(onStoreChange);
      return () => listeners.delete(onStoreChange);
    },
    () => (rootKey ? (shellChromePanelSnapshotByRoot.get(rootKey) ?? NO_CHROME_PANEL_OCCUPANCIES) : NO_CHROME_PANEL_OCCUPANCIES),
    () => NO_CHROME_PANEL_OCCUPANCIES,
  );
}

/** 🧭️ Sweep sorted forbidden displacement intervals, preserving the requested axis gap and host bounds. */
function chromePanelAxisReserve(affordance: SafeAreaBox, host: SafeAreaBox, panels: readonly SafeAreaBox[], inline: boolean, direction: number, gap: number): number {
  if (!direction) return 0;
  const a = inline ? [affordance.left, affordance.right, affordance.top, affordance.bottom] : [affordance.top, affordance.bottom, affordance.left, affordance.right];
  const h = inline ? [host.left, host.right, host.top, host.bottom] : [host.top, host.bottom, host.left, host.right];
  if (a[2]! < h[2]! || a[3]! > h[3]!) return 0;
  const intervals = panels.flatMap((panel) => {
    const p = inline ? [panel.left, panel.right, panel.top, panel.bottom] : [panel.top, panel.bottom, panel.left, panel.right];
    if (a[2]! >= p[3]! || a[3]! <= p[2]!) return [];
    return [direction > 0 ? [p[0]! - a[1]! - gap, p[1]! - a[0]! + gap] : [a[0]! - p[1]! - gap, a[1]! - p[0]! + gap]];
  }).sort((a,b) => a[0]! - b[0]!);
  let offset = 0;
  for (const interval of intervals) {
    if (interval[0]! >= offset) break;
    if (interval[1]! > offset) offset = Math.ceil(interval[1]!);
  }
  const start = a[0]! + direction * offset;
  const end = a[1]! + direction * offset;
  return offset > 0 && start >= h[0]! && end <= h[1]! ? offset : 0;
}

/** 🛟️ Clear every overlapping shell panel along one authored anchor axis, including panels reached after
 * moving. Preserve the axis gap, round outward, and stay entirely inside the actual host. The smallest
 * viable displacement wins; block wins ties. No overlap or no fit leaves placement unchanged.
 * {@link chromePanelSafeAreaStyle} applies this reserve; the native twin consumes the same Shell corpus. */
export function chromePanelSafeArea(affordance: SafeAreaBox, host: SafeAreaBox, anchor: Anchor, panels: readonly SafeAreaBox[], yieldAxis: SafeAreaYield, gapPx: number): ChromePanelSafeArea {
  if (!panels.some((panel) => safeAreaBoxesOverlap(panel, affordance))) return CHROME_PANEL_SAFE_AREA_CLEAR;
  const vertical = anchorVertical(anchor);
  const horizontal = anchorHorizontal(anchor);
  const block = yieldAxis === "inline" ? 0 : chromePanelAxisReserve(affordance, host, panels, false, vertical === "top" ? 1 : vertical === "bottom" ? -1 : 0, gapPx);
  const inline = yieldAxis === "block" ? 0 : chromePanelAxisReserve(affordance, host, panels, true, horizontal === "left" ? 1 : horizontal === "right" ? -1 : 0, gapPx);
  if (block > 0 && (inline === 0 || block <= inline)) return { inlinePx: 0, blockPx: block };
  if (inline > 0) return { inlinePx: inline, blockPx: 0 };
  return CHROME_PANEL_SAFE_AREA_CLEAR;
}

/** 🛟️ {@link anchorPositionStyle}'s two edge insets, moved inward by a {@link chromePanelSafeArea}
 * — the flush `var(--spacing-single)` inset (or the caller's own `base`, e.g. a window content rail's
 * {@link windowChromeClearedTopOffset}) when the safe area is clear, so a shell with no open panel lays
 * out byte-for-byte as authored. A middle anchor has no edge to reserve on that axis. */
export function chromePanelSafeAreaStyle(anchor: Anchor, safeArea: ChromePanelSafeArea, base?: { readonly block?: string; readonly inline?: string }): React.CSSProperties {
  const vertical = anchorVertical(anchor);
  const horizontal = anchorHorizontal(anchor);
  const blockBase = base?.block ?? "var(--spacing-single)";
  const inlineBase = base?.inline ?? "var(--spacing-single)";
  const style: React.CSSProperties = {};
  if (vertical !== "middle") style[vertical] = safeArea.blockPx > 0 ? `calc(${blockBase} + ${safeArea.blockPx}px)` : blockBase;
  if (horizontal !== "middle") style[horizontal] = safeArea.inlinePx > 0 ? `calc(${inlineBase} + ${safeArea.inlinePx}px)` : inlineBase;
  return style;
}

/** 🛟️ Live {@link chromePanelSafeArea} for one mounted affordance, measured against every open
 * anchored chrome panel of the enclosing shell.
 *
 * The affordance is graded on its UNRESERVED box — its measured rect with the offset this hook currently
 * applies taken back off — so a rail that has already stepped clear of a panel does not read as clear,
 * drop its offset and step back under it. */
export function useChromePanelSafeArea({
  hostRef,
  affordanceRef,
  anchor,
  yieldAxis,
  gapPx = 0,
  enabled = true,
}: {
  readonly hostRef: React.RefObject<HTMLElement | null>;
  readonly affordanceRef: React.RefObject<HTMLElement | null>;
  readonly anchor: Anchor;
  readonly yieldAxis: SafeAreaYield;
  readonly gapPx?: number;
  readonly enabled?: boolean;
}): ChromePanelSafeArea {
  const shellScope = useShellScopeOptional();
  const panels = useShellChromePanelBoxes(shellScope?.rootRef.current ?? undefined);
  const [safeArea, setSafeArea] = reactHostPort.useState<ChromePanelSafeArea>(CHROME_PANEL_SAFE_AREA_CLEAR);
  const appliedRef = reactHostPort.useRef<ChromePanelSafeArea>(CHROME_PANEL_SAFE_AREA_CLEAR);
  reactHostPort.useLayoutEffect(() => {
    const host = hostRef.current;
    const affordance = affordanceRef.current;
    const commit = (next: ChromePanelSafeArea) => {
      if (next.inlinePx === appliedRef.current.inlinePx && next.blockPx === appliedRef.current.blockPx) return;
      appliedRef.current = next;
      setSafeArea(next);
    };
    if (!enabled || !host || !affordance || panels.length === 0) {
      commit(CHROME_PANEL_SAFE_AREA_CLEAR);
      return;
    }
    const boxes = panels.map((panel) => panel.box);
    const vertical = anchorVertical(anchor);
    const horizontal = anchorHorizontal(anchor);
    const sync = () => {
      const applied = appliedRef.current;
      const rect = affordance.getBoundingClientRect();
      const blockShift = vertical === "top" ? -applied.blockPx : vertical === "bottom" ? applied.blockPx : 0;
      const inlineShift = horizontal === "right" ? applied.inlinePx : horizontal === "left" ? -applied.inlinePx : 0;
      const unreserved: SafeAreaBox = { left: rect.left + inlineShift, top: rect.top + blockShift, right: rect.right + inlineShift, bottom: rect.bottom + blockShift };
      commit(chromePanelSafeArea(unreserved, safeAreaBoxFromRect(host.getBoundingClientRect()), anchor, boxes, yieldAxis, gapPx));
    };
    sync();
    const resizeObserver = typeof ResizeObserver !== "undefined" ? new ResizeObserver(sync) : null;
    resizeObserver?.observe(host);
    resizeObserver?.observe(affordance);
    window.addEventListener("resize", sync);
    return () => {
      resizeObserver?.disconnect();
      window.removeEventListener("resize", sync);
    };
  }, [affordanceRef, anchor, enabled, gapPx, hostRef, panels, yieldAxis]);
  return safeArea;
}
