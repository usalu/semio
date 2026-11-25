/** 🎨 Owns the chrome class-name constants that panels, tab bars, windows and drop zones compose. */
import { cn } from "../🏷️class-name-composition/🟦️.ts";
import { hoverExcludingHandleTextEmphasizedClass, hoverExcludingHandleBgFillClass, interactiveActiveFillClass } from "../🖱️interaction-presentation/🟦️.ts";
import { borderNormalClass } from "../📏️border-presentation/🟦️.ts";
import { menuItemClassName } from "../../🧱️elements/📋️MenuItem/🟦️.tsx";

/** 🌳️ Typography for measure tree leaf labels. */
export const windowMeasureTreeLeafLabelClass = "text-tiny font-normal text-element group-hover:text-emphasized transition-colors";

/** 🔖️ Typography for measure tree group headers. */
export const windowMeasureTreeGroupLabelClass = "text-tiny font-semibold uppercase tracking-wide text-element group-hover:text-emphasized";

/** 📑️ Panel tab label beside the icon. */
export const panelTabLabelClass = "min-w-0 truncate text-xs leading-none";

/** 🖼️ Panel tab icon slot — defers dimensions to the tab icon (12px). */
export const panelTabIconSlotClass = "inline-flex shrink-0 items-center justify-center overflow-visible leading-none [&_svg]:block";

/** 🥂️ The glass overlay box recipe shared by the introduction info box and modal dialogs (see
 * `UIDialog`) — the two mechanisms are styled identically by construction, not by convention.
 * Border color is CSS-owned (`[data-slot="introduction-info-box"|"dialog-box"]`): normal at rest,
 * emphasized on `:hover` only — never `:focus-within`, so clicking a button does not leave the
 * border stuck after the pointer leaves. */
export const GLASS_OVERLAY_BOX_CLASS = cn("text-foreground pointer-events-auto fixed z-tutorial max-w-sm p-double");

/** 📏️ Normal bottom edge utility for in-chrome dividers (not shell navbar — navbar uses a CSS `::after` stroke). */

/** 🔝️ Normal top edge utility for in-chrome dividers (not shell footer — footer uses a CSS `::before` stroke). */
export const borderNormalTopClass = `border-t ${borderNormalClass}`;

/** 🎬️ Action row inside {@link floatingMenuSurfaceClass}. */
export const floatingMenuItemClass = menuItemClassName;

/** 📜️ Panel tab strip scroll row inside {@link panelTabBarClass} — `ui-scrollbar-hidden` preserves fixed control height when overflowing tabs scroll horizontally; it paints no edge, so scrolling never moves a seam. */
export const panelTabBarScrollRowClass = "ui-scrollbar-hidden relative z-40 flex min-w-0 flex-auto items-stretch overflow-x-auto overscroll-x-contain scroll-px-single";

/** 🪟️ Non-scrolling host of a panel tab row — the shared-border frame whose single content-facing edge is the one seam between tab strip and panel body, so the compositor overlay never rides the scroller. */
const panelTabFrameClass = "relative flex min-w-0 shrink-0 items-stretch";

/** 🧱️ Panel tab strip frame spanning its host (`w-full` spans chrome/mobile dividers across the row; floating panel caps use {@link panelAnchorTabBarClass} so free row space opens after the last tab); its content-facing divider is a compositor edge declared by {@link panelTabBarEdge}, not a native border. */
export const panelTabBarClass = cn(panelTabFrameClass, "w-full");

/** 🔘️ Panel tab icon slot — defers dimensions to the tab icon (12px). */

/** 🏷️ Panel tab label beside the icon. */

/** ⏹️ Normal logical-end divider between sibling panel-tab toggles; the last toggle defers its outer edge to the hosting chrome frame. */
/** 🔚️ Normal logical-end divider between sibling panel-tab toggles; the last toggle defers its outer edge to the hosting chrome frame. */
export const panelTabButtonDividerClass = "shared-border-cell";

/** ❗️ Panel tab button with icon, mandatory name, and a normal divider between sibling toggles. */
/** 🔤️ Panel tab button with icon, mandatory name, and a normal divider between sibling toggles. */
export const panelTabButtonClass = cn(
  "inline-flex min-h-0 shrink-0 items-center gap-tiny bg-transparent p-0",
  panelTabButtonDividerClass,
  "cursor-pointer whitespace-nowrap text-xs leading-none text-element transition-colors",
  "outline-none",
  hoverExcludingHandleBgFillClass,
  hoverExcludingHandleTextEmphasizedClass,
);

/** ⚓️ Floating panel tab strip frame inside {@link WindowChrome} — sizes the strip; the edge toward the panel body is declared separately by {@link panelTabBarEdge}. */
export const panelAnchorTabBarClass = cn(panelTabFrameClass, "h-[var(--panel-header-height)]");

/** 🧭️ The one physical edge a tab strip shares with the panel body: collapsed strips defer every edge to the hosting chrome frame, expanded ones own only the content-facing side. */
export function panelTabBarEdge(variant: "panel" | "chrome" | "mobile", direction: "up" | "down", expanded: boolean): "top" | "bottom" | undefined {
  if (variant === "mobile") return "bottom";
  if (!expanded) return undefined;
  return direction === "up" ? "top" : "bottom";
}

/** 📌️ Panel tab button padding. */
/** 🧷️ Panel tab button padding. */
export const panelAnchorTabButtonClass = cn(panelTabButtonClass, "px-tiny");

/** 📱️ Mobile panel tab strip height. */
export const mobilePanelTabBarClass = cn(panelTabBarClass, "h-large");

/** 📲️ Mobile panel tab button padding. */
export const mobilePanelTabButtonClass = cn(panelTabButtonClass, "px-single");

/** 🌑️ Inactive sibling tab — normal pill resting on the U-frame baseline; transparent so it
 * shows the chip-cell glass beneath it rather than a second opaque fill (matches {@link panelWindowInactiveTabClass}'s rule for the panel variant). */
export const modeDockInactiveTabClass = cn(`relative z-30 box-border min-h-medium shrink-0`, "bg-transparent");

/** 🕳️ Inactive tab before the free row space — inner divider only; outer stroke owned by the frame border SVG. */
export const modeDockInactiveTabBeforeGapClass = cn(`relative z-30 box-border min-h-medium shrink-0`, "bg-transparent");

/** 💤️ Inactive panel tab inside {@link WindowChrome} — no opaque fill; the chip-cap glass is the panel boundary and must meet the body without a seam. */
export const panelWindowInactiveTabClass = "relative z-30 box-border min-h-medium h-full shrink-0 bg-transparent";

/** 🪟️ Icon + title cluster inside a mode-dock tab — standard gap between glyph and label. */
export const modeDockTabLabelClassName = "flex min-w-0 flex-1 items-center gap-single overflow-visible";

/** ⭐️ Globally active dock tab — primary fill + emphasized label. */
export const modeDockActiveTabFillClass = interactiveActiveFillClass;

/** 🥞️ Stack-active tab fill — outline owned by the stack frame border SVG; `border-0` must win over {@link interactiveActiveFillClass}'s border color utility. */
export const modeDockActiveTabClass = cn("relative z-20 box-border min-h-medium shrink-0 border-0", modeDockActiveTabFillClass);

/** 📐️ Merged top-left Actions body beside the engagement chrome toggle: the active engagement's status/control (when present) stacked above the categorized ad-hoc actions tree; scrolls once content exceeds the window body. */
export const windowEngagementBodyClass = "ui-scrollbar-hidden flex min-h-medium min-w-0 max-h-full flex-auto flex-col overflow-y-auto";

/** 🛠️ Utility row beside the utility bar chrome toggle — a single utility keeps the chrome's height, but the active utility's options tree (stacked above it) can grow taller; its inline `maxHeight` (see {@link useWindowUtilityBarMaxHeightPx}) caps it just below the top-anchored chrome and this scrolls the overflow instead of painting past that line. */
export const utilityBarBodyClass = "ui-scrollbar-hidden flex min-h-medium min-w-0 flex-auto items-center gap-single overflow-x-auto overflow-y-auto px-single";

/** 🌲️ Measure tree body: grows with content, scrolls once the stack hits the window bottom. */
export const windowMeasuresBodyClass = "ui-scrollbar-hidden flex min-h-0 min-w-0 flex-auto flex-col overflow-y-auto overscroll-contain p-tiny";
