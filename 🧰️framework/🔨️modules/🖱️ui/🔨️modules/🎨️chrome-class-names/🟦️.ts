/** 🎨 Owns the chrome class-name constants that panels, tab bars, windows and drop zones compose. */
import { cn } from "../🏷️class-name-composition/🟦️.ts";
import { hoverExcludingHandleTextEmphasizedClass, hoverExcludingHandleBgFillClass, interactiveActiveFillClass } from "../🖱️interaction-presentation/🟦️.ts";
import { borderNormalBottomClass, borderNormalClass } from "../📏️border-presentation/🟦️.ts";
import { surfaceClass } from "../🌈️surface-presentation/🟦️.ts";
import { menuItemClassName } from "../../🧱️elements/📋️MenuItem/🟦️.tsx";

/** 🌳️ Typography for measure tree leaf labels. */
export const windowMeasureTreeLeafLabelClass = "text-tiny font-normal text-element group-hover:text-emphasized transition-colors";

/** 🔖️ Typography for measure tree group headers. */
export const windowMeasureTreeGroupLabelClass = "text-tiny font-semibold uppercase tracking-wide text-element group-hover:text-emphasized";

/** 📑️ Panel tab label beside the icon. */
export const panelTabLabelClass = "min-w-0 truncate text-xs leading-none";

/** 🖼️ Panel tab icon slot — defers dimensions to the tab icon (12px). */
export const panelTabIconSlotClass = "inline-flex shrink-0 items-center justify-center overflow-visible leading-none [&_svg]:block";

/** 🎯️ Label/icon emphasis paired with {@link dropZoneReadyFillClass} so text stays legible on the fill. */
export const dropZoneReadyTextClass = "text-emphasized";

/** 💧️ Passive drop-zone fill — secondary accent, kept visually distinct from the stronger primary-accent indicator on the actively hovered target. */
export const dropZoneReadyFillClass = "bg-[var(--accent-secondary)]";

/** ➖️ Combined passive drop-zone treatment (fill + emphasized text/icons). */
export const dropZoneReadyClass = cn(dropZoneReadyFillClass, dropZoneReadyTextClass);

/** 🥂️ The glass overlay box recipe shared by the introduction info box and modal dialogs (see
 * `UIDialog`) — the two mechanisms are styled identically by construction, not by convention.
 * Border color is CSS-owned (`[data-slot="introduction-info-box"|"dialog-box"]`): normal at rest,
 * emphasized on `:hover` only — never `:focus-within`, so clicking a button does not leave the
 * border stuck after the pointer leaves. */
export const GLASS_OVERLAY_BOX_CLASS = cn("text-foreground pointer-events-auto fixed z-tutorial max-w-sm rounded-lg p-double shadow-lg");

/** 📏️ Normal bottom edge utility for in-chrome dividers (not shell navbar — navbar uses a CSS `::after` stroke). */

/** 🔝️ Normal top edge utility for in-chrome dividers (not shell footer — footer uses a CSS `::before` stroke). */
export const borderNormalTopClass = `border-t ${borderNormalClass}`;

/** 🔲️ Parent stroke on top of fill and content (transparent center; ghost-dimmed with the open panel/pane). Border color is CSS-only so parent `:hover` can emphasize it like the window silhouette. */
export const shellChromeFrameLayerClass = "pointer-events-none absolute inset-0 z-30 box-border bg-transparent";

/** 🎬️ Action row inside {@link floatingMenuSurfaceClass}. */
export const floatingMenuItemClass = menuItemClassName;

/** 📜️ Panel tab strip scroll row — `ui-scrollbar-hidden` preserves fixed control height when overflowing tabs scroll horizontally. */
const panelTabBarScrollClass = "ui-scrollbar-hidden relative z-40 flex min-w-0 items-stretch shrink-0 overflow-x-auto overscroll-x-contain scroll-px-single";

/** 🍫️ Panel tab strip base — `w-full` spans chrome/mobile dividers across the row; floating panel caps omit it so the U-gap opens after the last tab. */
const panelTabBarBaseClass = cn(panelTabBarScrollClass, "w-full");

/** 🧱️ Panel tab strip with its divider on the content-facing side. */
export const panelTabBarClass = cn(panelTabBarBaseClass, borderNormalBottomClass);

/** 🔘️ Panel tab icon slot — defers dimensions to the tab icon (12px). */

/** 🏷️ Panel tab label beside the icon. */

/** ⏹️ Normal logical-end divider between sibling panel-tab toggles; the last toggle defers its outer edge to the hosting chrome silhouette. */
/** 🔚️ Normal logical-end divider between sibling panel-tab toggles; the last toggle defers its outer edge to the hosting chrome silhouette. */
export const panelTabButtonDividerClass = "border-e border-solid !border-normal last:border-e-0";

/** ❗️ Panel tab button with icon, mandatory name, and a normal divider between sibling toggles. */
/** 🔤️ Panel tab button with icon, mandatory name, and a normal divider between sibling toggles. */
export const panelTabButtonClass = cn(
  "inline-flex min-h-0 shrink-0 items-center gap-tiny bg-transparent p-0",
  panelTabButtonDividerClass,
  "cursor-pointer whitespace-nowrap text-xs leading-none text-element transition-colors",
  "outline-none focus-visible:ring-1 focus-visible:ring-inset focus-visible:ring-active-base",
  "[&[data-active=true]:not(:first-child)]:border-s [&[data-active=true]:not(:first-child)]:-ms-px",
  hoverExcludingHandleBgFillClass,
  hoverExcludingHandleTextEmphasizedClass,
);

/** ⚓️ Floating panel tab strip inside {@link WindowChrome} — collapsed tabs defer every outer edge to the silhouette; expanded tabs restore the normal content-facing edge that separates their toggles from the panel body. */
export function panelAnchorTabBarClass(direction: "up" | "down", expanded = false): string {
  return cn(panelTabBarScrollClass, "h-[var(--panel-header-height)]", expanded && (direction === "up" ? borderNormalTopClass : borderNormalBottomClass));
}

/** 📌️ Panel tab button padding. */
/** 🧷️ Panel tab button padding. */
export const panelAnchorTabButtonClass = cn(panelTabButtonClass, "px-tiny");

/** 📱️ Mobile panel tab strip height. */
export const mobilePanelTabBarClass = cn(panelTabBarClass, "h-large");

/** 📲️ Mobile panel tab button padding. */
export const mobilePanelTabButtonClass = cn(panelTabButtonClass, "px-single");

/** ↔ Accent stroke on the panel resize edge while hovered or dragging. */
export function panelResizeEdgeAccentClass(resizeSide: "left" | "right", active: boolean): string | undefined {
  if (!active) return undefined;
  switch (resizeSide) {
    case "left":
      return "border-l-accent";
    case "right":
      return "border-r-accent";
  }
}

/** 🎞️ Body fill only — outer stroke is the stack silhouette SVG (tabs + cutout + controls + body); base level (mode body / floor windows float on) — host element must also carry `data-level="base"`. */
export const windowBodyFrameClass = cn("relative border-0", surfaceClass);

/** 🌑️ Inactive sibling tab — normal pill resting on the U-frame baseline; transparent so it
 * shows the chip-cell glass beneath it rather than a second opaque fill (matches {@link panelWindowInactiveTabClass}'s rule for the panel variant). */
export const modeDockInactiveTabClass = cn(`relative z-30 box-border min-h-medium shrink-0`, "bg-transparent");

/** 🕳️ Inactive tab before gap — inner divider only; outer stroke owned by the silhouette SVG. */
export const modeDockInactiveTabBeforeGapClass = cn(`relative z-30 box-border min-h-medium shrink-0`, "bg-transparent");

/** 💤️ Inactive panel tab inside {@link WindowChrome} — no opaque fill; the chip-cap glass is the panel boundary and must meet the body without a seam. */
export const panelWindowInactiveTabClass = "relative z-30 box-border min-h-medium h-full shrink-0 bg-transparent";

/** 🪟️ Icon + title cluster inside a mode-dock tab — standard gap between glyph and label. */
export const modeDockTabLabelClassName = "flex min-w-0 flex-1 items-center gap-single overflow-visible";

/** ⭐️ Globally active dock tab — primary fill + emphasized label. */
export const modeDockActiveTabFillClass = interactiveActiveFillClass;

/** 🥞️ Stack-active tab fill — outline owned by the stack silhouette SVG; `border-0` must win over {@link interactiveActiveFillClass}'s border color utility. */
export const modeDockActiveTabClass = cn("relative z-20 box-border min-h-medium shrink-0 border-0", modeDockActiveTabFillClass);

/** 📐️ Merged top-left Actions body beside the engagement chrome toggle: the active engagement's status/control (when present) stacked above the categorized ad-hoc actions tree; scrolls once content exceeds the window body. */
export const windowEngagementBodyClass = "flex min-h-medium min-w-0 max-h-full flex-auto flex-col gap-half overflow-y-auto px-single";

/** 🛠️ Utility row beside the utility bar chrome toggle — a single utility keeps the chrome's height, but the active utility's options tree (stacked above it) can grow taller; its inline `maxHeight` (see {@link useWindowUtilityBarMaxHeightPx}) caps it just below the top-anchored chrome and this scrolls the overflow instead of painting past that line. */
export const utilityBarBodyClass = "flex min-h-medium min-w-0 flex-auto items-center gap-single overflow-x-auto overflow-y-auto px-single";

/** 🌲️ Measure tree body: grows with content, scrolls once the stack hits the window bottom. */
export const windowMeasuresBodyClass = "flex min-h-0 min-w-0 flex-auto flex-col overflow-y-auto overscroll-contain p-tiny";
