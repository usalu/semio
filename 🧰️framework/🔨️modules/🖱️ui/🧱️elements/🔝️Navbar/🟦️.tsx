// #region 🧲️Header
// 💻️ framework/ui/elements/🔝️Navbar/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { shellFloorPaints, shellFloorFillClass } from "../../🔨️modules/🏠️shell-floor-presentation/🟦️.ts";
import { useSurface, SurfaceScope, getLevelZClass } from "../🌈️Surface/🟦️.tsx";
import { NavbarTrailingChromeSlot } from "../../🎯️targets/⚛️react/🟦️";
// #endregion 🔌️Adapters

// #region 🩺️Navbar
// Top navigation bar with icon items.
// Consumers MUST provide NavbarItem entries.

/**
 * Configuration interface for a single navbar item.
 **/
export interface NavbarItem {
  content: React.ReactNode;
  className?: string;
  key?: React.Key;
  /** @emoji 🎯️ When true, positions the item absolutely so it is centered relative to the full navbar width, independent of sibling item widths — but never across the flow row's own chrome; see {@link navbarCenteredLeftV1}. */
  centered?: boolean;
}

/** @emoji ↔️ One horizontal interval of a chrome band, in whole pixels measured from the band's own left edge. */
export type NavbarSpanV1 = { readonly left: number; readonly right: number };

/** @emoji 🧱️ Whether one flow-row child actually OCCUPIES its box. A filler ({@link navbarFillItem}) renders
 * nothing, so its wide `flex-1` box is exactly the room a centered item may use; every other child paints
 * chrome a centered item must stay clear of. */
export function navbarFlowChildOccupiesV1(child: { readonly childElementCount: number; readonly textContent: string | null }): boolean {
  return child.childElementCount > 0 || (child.textContent ?? "").trim().length > 0;
}

/** @emoji 🛟️ The widest horizontal band of a `width`-wide chrome bar that no occupied span covers.
 *
 * A centered navbar item is absolutely positioned, so it is invisible to the flow row's own layout and
 * will happily paint (and swallow clicks for) the chrome parked at the bar's edges. Measured on
 * 2026-09-14 at 1280 px: the playground's centered cluster ran 264→1017 while the `top-right`
 * `PanelChromeTabBar` started at 947 and paints at `z-40` inside the dock's stacking context, so
 * `document.elementFromPoint` at the navbar `Viewer` button's own centre resolved to
 * `framework.panel.inspection` and the role switch was unreachable by pointer
 * (`📓️role-switch-regression-2026-09-14.md`). A bar whose every pixel is occupied answers its whole
 * width rather than a zero-width band: displacing an item into nothing is not an improvement. */
export function navbarFreeBandV1(width: number, occupied: readonly NavbarSpanV1[]): NavbarSpanV1 {
  const clamped = occupied
    .map((span) => ({ left: Math.max(0, Math.min(width, span.left)), right: Math.max(0, Math.min(width, span.right)) }))
    .filter((span) => span.right > span.left)
    .sort((left, right) => left.left - right.left);
  const merged: NavbarSpanV1[] = [];
  for (const span of clamped) {
    const last = merged[merged.length - 1];
    if (last !== undefined && span.left <= last.right) merged[merged.length - 1] = { left: last.left, right: Math.max(last.right, span.right) };
    else merged.push(span);
  }
  let band: NavbarSpanV1 = { left: 0, right: 0 };
  let cursor = 0;
  for (const span of [...merged, { left: width, right: width }]) {
    if (span.left - cursor > band.right - band.left) band = { left: cursor, right: span.left };
    cursor = Math.max(cursor, span.right);
  }
  return band.right > band.left ? band : { left: 0, right: width };
}

/** @emoji 🎯️ Left offset of a centered item: the bar's own centre whenever the item fits there without
 * crossing {@link navbarFreeBandV1}'s edges, and otherwise the nearest position inside the band. An item
 * wider than the band starts at the band's left edge — its own `max-width` is what makes it fit, so this
 * is a fixed point rather than a step that re-measures into a different answer. */
export function navbarCenteredLeftV1(width: number, band: NavbarSpanV1, contentWidth: number): number {
  const latest = band.right - contentWidth;
  if (latest <= band.left) return band.left;
  return Math.round(Math.min(Math.max((width - contentWidth) / 2, band.left), latest));
}

/**
 * Props interface for the Navbar component.
 **/
export interface NavbarProps {
  items: NavbarItem[];
  className?: string;
  showFullscreenToggle?: boolean;
  onFullscreenToggle?: () => void;
  /** @emoji 💬 Chrome parked immediately left of the fullscreen control (e.g. chat panel toggle). */
  trailingBeforeFullscreen?: React.ReactNode;
}

/**
 * Navbar holds the data fields for a Navbar record.
 *
 * 🪜️ The band stacks at the BASE level, never above it. A chrome-hosted {@link Panel} unfolds INTO this
 * band (see `chromeHostedOpenPanelPositionStyle`: its cap lands exactly on the centered `h-medium`
 * control row, in the slot `PanelChromeTabBar` empties while the panel is open), so a navbar painting
 * above `z-panel` would cover that cap and every row under it. `🎨️ui.css` has always forced
 * `z-index: var(--z-base) !important` here; this class said `z-navbar` and was simply never the truth —
 * a contradiction that cost ticket 26/09/02 three waves of "the navbar covers the catalogue".
 **/
/** @emoji 🛟️ The measured placement of every centered item of one chrome band, in band-relative pixels.
 * `null` until the first layout pass — a band that has not been measured centers by transform, so the
 * first painted frame is already centered and nothing flashes in from the left edge. */
type NavbarCenteredPlacementV1 = { readonly left: number; readonly maxWidth: number };

/** @emoji 🪜️ The shared body of {@link Navbar} and {@link Footer}: one flow row of ordinary items plus one
 * absolutely positioned layer per centered item, each placed by {@link navbarCenteredLeftV1} against the
 * free band the flow row leaves. Measured live, because which chrome the row carries — panel tab bars
 * above all — is a runtime user choice, and a band computed once boots stale the first time a panel
 * opens. */
function NavbarBandBody({ normalItems, centeredItems, trailing }: { normalItems: NavbarItem[]; centeredItems: NavbarItem[]; trailing?: React.ReactNode }) {
  const rowRef = React.useRef<HTMLDivElement | null>(null);
  const centeredRefs = React.useRef<(HTMLDivElement | null)[]>([]);
  const [placements, setPlacements] = React.useState<readonly (NavbarCenteredPlacementV1 | null)[]>([]);
  React.useLayoutEffect(() => {
    const row = rowRef.current;
    if (row === null || typeof ResizeObserver !== "function") return;
    const measure = () => {
      const bandRect = row.getBoundingClientRect();
      const width = Math.round(bandRect.width);
      if (width === 0) return;
      const occupied = [...row.children]
        .filter((child): child is HTMLElement => child instanceof HTMLElement && navbarFlowChildOccupiesV1(child))
        .map((child) => {
          const rect = child.getBoundingClientRect();
          return { left: Math.round(rect.left - bandRect.left), right: Math.round(rect.right - bandRect.left) };
        });
      const free = navbarFreeBandV1(width, occupied);
      const next = centeredItems.map((_, index) => {
        const element = centeredRefs.current[index];
        if (!element) return null;
        return { left: navbarCenteredLeftV1(width, free, Math.round(element.getBoundingClientRect().width)), maxWidth: free.right - free.left };
      });
      setPlacements((current) => (current.length === next.length && current.every((entry, index) => entry?.left === next[index]?.left && entry?.maxWidth === next[index]?.maxWidth) ? current : next));
    };
    const observer = new ResizeObserver(measure);
    observer.observe(row);
    const observeRowChildren = () => {
      for (const child of row.children) if (child instanceof HTMLElement) observer.observe(child);
    };
    observeRowChildren();
    for (const element of centeredRefs.current) if (element) observer.observe(element);
    const mutations = new MutationObserver(() => {
      observeRowChildren();
      measure();
    });
    mutations.observe(row, { childList: true, subtree: true, characterData: true });
    measure();
    return () => {
      observer.disconnect();
      mutations.disconnect();
    };
  }, [centeredItems.length, normalItems.length]);
  return (
    <>
      <div ref={rowRef} className="p-single flex gap-single items-center min-w-0 h-full">
        {normalItems.map((item, index) => (
          <div key={item.key ?? index} className={cn("h-medium flex shrink-0 items-center min-w-0", item.className)}>
            {item.content}
          </div>
        ))}
        {trailing}
      </div>
      {centeredItems.map((item, index) => {
        const placement = placements[index] ?? null;
        return (
          <div key={item.key ?? index} className="pointer-events-none absolute inset-0 flex items-center">
            <div
              ref={(element) => {
                centeredRefs.current[index] = element;
              }}
              data-slot="navbar-centered"
              className={cn("pointer-events-auto absolute h-medium flex items-center", item.className)}
              style={placement === null ? { left: "50%", transform: "translateX(-50%)" } : { left: `${placement.left}px`, maxWidth: `${placement.maxWidth}px` }}
            >
              {item.content}
            </div>
          </div>
        );
      })}
    </>
  );
}

function Navbar({ items, className, showFullscreenToggle = true, onFullscreenToggle, trailingBeforeFullscreen }: NavbarProps) {
  const parent = useSurface();
  const paints = shellFloorPaints(parent);
  const bgClass = shellFloorFillClass(parent);
  const normalItems = items.filter((item) => !item.centered);
  const centeredItems = items.filter((item) => item.centered);
  const body = (
    <NavbarBandBody
      normalItems={normalItems}
      centeredItems={centeredItems}
      trailing={
        showFullscreenToggle || trailingBeforeFullscreen ? (
          <NavbarTrailingChromeSlot beforeFullscreen={trailingBeforeFullscreen} showFullscreenToggle={showFullscreenToggle} onFullscreenToggle={onFullscreenToggle} />
        ) : null
      }
    />
  );
  return (
    <nav id="ui.navbar" data-slot="navbar" data-level="base" data-ui-reveal-region="navbar" data-elevation-root="" className={cn("relative h-large", getLevelZClass("base"), bgClass, className)}>
      {paints ? <SurfaceScope level="base" fill="surface">{body}</SurfaceScope> : body}
    </nav>
  );
}

export { Navbar, NavbarBandBody };

//#region 🩺️SemioLogo
/** @emoji 🎨️ Round dark semio emblem for navbar and chrome. */
export function SemioLogo({ className, style }: { className?: string; style?: React.CSSProperties }) {
  return (
    <svg viewBox="0 0 350 350" className={className} style={style} xmlns="http://www.w3.org/2000/svg">
      <path d="M270.589 28.413a175 175 0 0151.24 241.804A175 175 0 0180.155 322.07 175 175 0 0127.691 80.528a175 175 0 01241.408-53.076" fill="#001117" />
      <path d="M76.25 271.933l35-35.808V118.75h-35z" fill="#fa9500" stroke="#f7f3e3" strokeWidth="2.5" strokeMiterlimit="5" />
      <g fill="#ff344f" stroke="#f7f3e3" strokeWidth="2.5" strokeMiterlimit="5">
        <path d="M76.25 113.75h155.563l37.66-37.5H76.25zM236.263 273.75l-.013-155.606 37.5-37.62V273.75z" />
      </g>
      <g fill="#34d1bf" stroke="#f7f3e3" strokeWidth="2.5" strokeMiterlimit="5">
        <path d="M160.467 273.75h70.783v-37.5h-34.169zM160.468 193.75h70.782v-37.5h-34.169z" />
      </g>
    </svg>
  );
}
//#endregion 🩺️SemioLogo

//#region 🏷️ShellBrandLogo
/** @emoji 🏷️ Renders a shell brand's raw inline-SVG mark in navbar chrome (first-party repo content authored in `framework/os/dev/brand`, injected as markup). */
export function ShellBrandLogo({ svg, className, style }: { svg: string; className?: string; style?: React.CSSProperties }) {
  return <span className={cn("inline-flex items-center [&>svg]:h-full [&>svg]:w-auto", className)} style={style} dangerouslySetInnerHTML={{ __html: svg }} />;
}
//#endregion 🏷️ShellBrandLogo

/** @emoji ↔ Flex grow class that pushes trailing navbar chrome to the right edge. */
const navbarFillClassName = "flex-1 min-w-0";

/** @emoji ↔ Invisible navbar filler; use before trailing toggles when no center slot consumes the flex region. */
export function navbarFillItem(key = "navbarFill"): NavbarItem {
  return { key, className: navbarFillClassName, content: null };
}

// #endregion 🩺️Navbar
