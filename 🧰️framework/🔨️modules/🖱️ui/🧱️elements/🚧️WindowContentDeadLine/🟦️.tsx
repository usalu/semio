// #region Header
// framework/ui/elements/🚧️WindowContentDeadLine/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// Licensed under LGPL-3.0-or-later.
// #endregion Header

// #region 🔌️Adapters
import * as React from "react";
import { readSizeVarPx, STYLING_COMPACT_ROOT_PX, uiSpacingPx } from "@semio-tech/ui-styling";
import { reactHostPort } from "../🔌️Ports/🟦️.tsx";
// #endregion 🔌️Adapters

// #region 🚧️WindowContentDeadLine
/** @emoji 📐️ CSS variable for invisible top clearance below floating window chrome. */
export const windowChromeScrollClearanceVar = "--window-chrome-scroll-clearance";

/** @emoji 🏝️ CSS variable for the default cleared line under floating window chrome (dead-island). */
export const windowContentDeadLineVar = "--window-content-dead-line";

/** @emoji 🏝️ Chrome-aware scroll hosts stay edgeless but reserve scroll-padding for the dead line. */
export const windowContentDeadLineScrollClass = "overscroll-contain [scroll-padding-top:var(--window-content-dead-line)]";

/** @emoji 📐️ Resolves {@link windowChromeScrollClearanceVar} to px for layout math. */
export function readWindowChromeScrollClearancePx(element?: Element | null, rootPx = STYLING_COMPACT_ROOT_PX): number {
  const measured = measureWindowChromeScrollClearancePx(element);
  if (measured > 0) return measured;
  const fromCss = readSizeVarPx(windowChromeScrollClearanceVar, element);
  if (fromCss > 0) return fromCss;
  return uiSpacingPx(9, rootPx);
}

/** @emoji 📐️ Measures live floating engagement/measures chrome height inside the nearest window body. */
export function measureWindowChromeScrollClearancePx(element?: Element | null): number {
  const windowBody = element?.closest('[data-slot="window-body"]');
  if (!windowBody) return 0;
  const overlays = windowBody.querySelectorAll('[data-slot="window-engagement-overlay"], [data-slot="window-search-overlay"], [data-slot="window-measures-overlay"]');
  const bodyTop = windowBody.getBoundingClientRect().top;
  let maxBottom = bodyTop;
  for (const overlay of overlays) {
    if (!(overlay instanceof HTMLElement)) continue;
    const bottom = overlay.getBoundingClientRect().bottom;
    if (bottom > maxBottom) maxBottom = bottom;
  }
  return Math.max(0, Math.ceil(maxBottom - bodyTop));
}

/** @emoji 🏝️ True when an element scrolls inside a window with floating chrome overlays (not edgeless canvas bodies). */
export function isWindowContentDeadLineHost(element: Element | null): boolean {
  if (!element) return false;
  if (element.closest("[data-window-content-layout=edgeless]")) return false;
  const windowBody = element.closest('[data-slot="window-body"]');
  if (!windowBody) return false;
  return windowBody.querySelector('[data-slot="window-engagement-overlay"], [data-slot="window-search-overlay"], [data-slot="window-measures-overlay"]') != null;
}

/** @emoji 🏝️ Resolves the default dead-line scroll offset for chrome-aware window bodies. */
export function readWindowContentDeadLinePx(element?: Element | null, rootPx = STYLING_COMPACT_ROOT_PX): number {
  if (!element || !isWindowContentDeadLineHost(element)) return 0;
  const windowBody = element.closest('[data-slot="window-body"]');
  const fromBodyVar = readSizeVarPx(windowContentDeadLineVar, windowBody ?? element);
  if (fromBodyVar > 0) return fromBodyVar;
  const measured = measureWindowChromeScrollClearancePx(element);
  if (measured > 0) return measured;
  return readWindowChromeScrollClearancePx(element, rootPx);
}

/** @emoji 🏝️ True when a scroll host's content exceeds its viewport. */
export function readScrollerContentOverflows(scroller: HTMLElement): boolean {
  if (scroller.clientHeight <= 0) return true;
  if (scroller.scrollHeight > scroller.clientHeight + 1) return true;
  const viewport = scroller.querySelector('[data-slot="scroll-area-viewport"]');
  if (viewport instanceof HTMLElement) return viewport.scrollHeight > scroller.clientHeight + 1;
  return false;
}

/** @emoji 🏝️ Clears the first line under floating chrome by default; scrolling up reveals it edgelessly. */
export function useWindowContentDeadLineScroll(scrollerRef: React.RefObject<HTMLElement | null>): void {
  const edgelessScrollRef = reactHostPort.useRef(false);
  reactHostPort.useLayoutEffect(() => {
    const el = scrollerRef.current;
    if (!el) return;
    const applyDefault = () => {
      if (!isWindowContentDeadLineHost(el)) return;
      const overflows = readScrollerContentOverflows(el);
      if (!overflows) {
        edgelessScrollRef.current = false;
        if (el.scrollTop !== 0) el.scrollTop = 0;
        return;
      }
      const deadLine = readWindowContentDeadLinePx(el);
      if (deadLine <= 0 || edgelessScrollRef.current) return;
      if (el.scrollTop < deadLine) el.scrollTop = deadLine;
    };
    const onScroll = () => {
      if (!isWindowContentDeadLineHost(el)) return;
      if (!readScrollerContentOverflows(el)) {
        edgelessScrollRef.current = false;
        return;
      }
      const deadLine = readWindowContentDeadLinePx(el);
      if (deadLine <= 0) return;
      if (el.scrollTop < deadLine - 1) edgelessScrollRef.current = true;
      else if (el.scrollTop >= deadLine) edgelessScrollRef.current = false;
    };
    applyDefault();
    el.addEventListener("scroll", onScroll, { passive: true });
    const body = el.closest('[data-slot="window-body"]');
    if (!body) return () => el.removeEventListener("scroll", onScroll);
    const ro = new ResizeObserver(applyDefault);
    ro.observe(body);
    for (const slot of ["window-engagement-overlay", "window-search-overlay", "window-measures-overlay"] as const) {
      const overlay = body.querySelector(`[data-slot="${slot}"]`);
      if (overlay) ro.observe(overlay);
    }
    return () => {
      el.removeEventListener("scroll", onScroll);
      ro.disconnect();
    };
  }, []);
}
// #endregion 🚧️WindowContentDeadLine
