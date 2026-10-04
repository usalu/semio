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
/** 📐️ CSS variable for invisible top clearance below floating window chrome. */
export const windowChromeScrollClearanceVar = "--window-chrome-scroll-clearance";

/** 🏝️ CSS variable for the default cleared line under floating window chrome (dead-island). */
export const windowContentDeadLineVar = "--window-content-dead-line";

/** 🏝️ Chrome-aware scroll hosts stay edgeless but reserve scroll-padding for the dead line. */
export const windowContentDeadLineScrollClass = "overscroll-contain [scroll-padding-top:var(--window-content-dead-line)]";

/** 🪟️ Floating top chrome belongs only to its nearest window body. */
function windowTopOverlays(body: Element): Element[] {
  return Array.from(body.querySelectorAll('[data-slot="window-engagement-overlay"], [data-slot="window-search-overlay"], [data-slot="window-measures-overlay"]')).filter((overlay) => overlay.closest('[data-slot="window-body"]') === body);
}

/** 📐️ Resolves {@link windowChromeScrollClearanceVar} to px for layout math. */
export function readWindowChromeScrollClearancePx(element?: Element | null, rootPx = STYLING_COMPACT_ROOT_PX): number {
  const measured = measureWindowChromeScrollClearancePx(element);
  if (measured > 0) return measured;
  const fromCss = readSizeVarPx(windowChromeScrollClearanceVar, element);
  if (fromCss > 0) return fromCss;
  return uiSpacingPx(9, rootPx);
}

/** 📐️ Measures live floating engagement/measures chrome height inside the nearest window body. */
export function measureWindowChromeScrollClearancePx(element?: Element | null): number {
  const windowBody = element?.closest('[data-slot="window-body"]');
  if (!windowBody) return 0;
  const overlays = windowTopOverlays(windowBody);
  const bodyTop = windowBody.getBoundingClientRect().top;
  let maxBottom = bodyTop;
  for (const overlay of overlays) {
    if (!(overlay instanceof HTMLElement)) continue;
    const bottom = overlay.getBoundingClientRect().bottom;
    if (bottom > maxBottom) maxBottom = bottom;
  }
  return Math.max(0, Math.ceil(maxBottom - bodyTop));
}

/** 🏝️ True when an element scrolls inside a window with floating chrome overlays (not edgeless canvas bodies). */
export function isWindowContentDeadLineHost(element: Element | null): boolean {
  if (!element) return false;
  if (element.closest("[data-window-content-layout=edgeless]")) return false;
  const windowBody = element.closest('[data-slot="window-body"]');
  if (!windowBody) return false;
  return windowTopOverlays(windowBody).length > 0;
}

/** 🏝️ Resolves the default dead-line scroll offset for chrome-aware window bodies. */
export function readWindowContentDeadLinePx(element?: Element | null, rootPx = STYLING_COMPACT_ROOT_PX): number {
  if (!element || !isWindowContentDeadLineHost(element)) return 0;
  const windowBody = element.closest('[data-slot="window-body"]');
  const fromBodyVar = readSizeVarPx(windowContentDeadLineVar, windowBody ?? element);
  if (fromBodyVar > 0) return fromBodyVar;
  const measured = measureWindowChromeScrollClearancePx(element);
  if (measured > 0) return measured;
  return readWindowChromeScrollClearancePx(element, rootPx);
}

/** 🏝️ True when a scroll host's content exceeds its viewport. */
export function readScrollerContentOverflows(scroller: HTMLElement): boolean {
  if (scroller.clientHeight <= 0) return true;
  if (scroller.scrollHeight > scroller.clientHeight + 1) return true;
  const viewport = scroller.querySelector('[data-slot="scroll-area-viewport"]');
  if (viewport instanceof HTMLElement) return viewport.scrollHeight > scroller.clientHeight + 1;
  return false;
}

/** 🚧️ Reserves the floating chrome's measured space without discarding content or changing scroll position. */
export function useWindowContentDeadLineInset(scrollerRef: React.RefObject<HTMLElement | null>): void {
  reactHostPort.useLayoutEffect(() => {
    const el = scrollerRef.current;
    const body = el?.closest('[data-slot="window-body"]');
    if (!el || !body) return;
    const previous = el.style.paddingBlockStart;
    const apply = () => {
      const edgeless = el.closest('[data-window-content-layout="edgeless"]') || el.querySelector(':scope > [data-window-content-layout="edgeless"]:only-child');
      const bottom = windowTopOverlays(body).reduce((value, overlay) => Math.max(value, overlay.getBoundingClientRect().bottom), -Infinity);
      el.style.paddingBlockStart = `${edgeless ? 0 : Math.max(0, Math.ceil(bottom - el.getBoundingClientRect().top))}px`;
    };
    const resize = new ResizeObserver(apply);
    const observe = () => {
      resize.disconnect();
      resize.observe(body);
      resize.observe(el);
      for (const overlay of windowTopOverlays(body)) resize.observe(overlay);
      apply();
    };
    const mutations = new MutationObserver(observe);
    mutations.observe(body, { childList: true, subtree: true, attributes: true, attributeFilter: ["data-window-content-layout"] });
    observe();
    return () => { resize.disconnect(); mutations.disconnect(); el.style.paddingBlockStart = previous; };
  }, []);
}

// #endregion 🚧️WindowContentDeadLine
