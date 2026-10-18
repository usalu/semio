import { STYLING_METRICS, STYLING_SEMIO_THEME } from "../../🤖️generated/🔤️tokens/🟦️.ts";
import { resolveThemeGeometry, resolveThemeSpacingPx } from "../🏛️model/🟦️.ts";
import { activeUiTheme } from "../🎛️active-theme/🟦️.ts";

/** 🔑️ Canonical DOM size CSS variable names. */
export const STYLING_SIZE_VAR = {
  uiSpacing: "--ui-spacing",
  spacingSingle: "--spacing-single",
  spacingDouble: "--spacing-double",
  sizeTiny: "--size-tiny",
  sizeSmall: "--size-small",
  sizeWorkbench: "--size-workbench",
  sizeMedium: "--size-medium",
  sizeLarge: "--size-large",
  sizeHuge: "--size-huge",
  sizeMega: "--size-mega",
  sizeGiga: "--size-giga",
  sizeXl: "--size-xl",
  sizeTera: "--size-tera",
  sizePeta: "--size-peta",
  layoutLabel: "--layout-label",
  layoutPopoverMin: "--layout-popover-min",
  layoutPanelMin: "--layout-panel-min",
  layoutPanelRail: "--layout-panel-rail",
  layoutPanelMax: "--layout-panel-max",
  layoutCommandMax: "--layout-command-max",
  layoutEngagementMax: "--layout-engagement-max",
  layoutDeckWidth: "--layout-deck-width",
  layoutDeckHeight: "--layout-deck-height",
  strokeHairline: "--stroke-hairline",
} as const;

/** 🔑️ Returns a `var(--…)` reference for a DOM size token. */
export function sizeVar(key: keyof typeof STYLING_SIZE_VAR): string {
  return `var(${STYLING_SIZE_VAR[key]})`;
}

/** 📐️ DOM layout metrics and root-rem authority from the canonical styling contract. */
export const STYLING_DOM = STYLING_METRICS.dom;

/** 🔑️ Compact-mode reference root (px) for headless layout math. */
export const STYLING_COMPACT_ROOT_PX = STYLING_DOM.rootRemPx;

/** 📏️ A ui-spacing multiplier as a CSS length that follows the live `--ui-spacing` (compact or touch). */
export function uiSpacingLen(multiplier: number): string {
  return `calc(${multiplier} * var(--ui-spacing))`;
}

/** 📐️ Converts a ui-spacing multiplier to px at the compact reference root. */
export function uiSpacingPx(multiplier: number, rootPx?: number): number {
  const geometry = resolveThemeGeometry(activeUiTheme());
  return multiplier * (rootPx === undefined ? geometry.spacingPx : resolveThemeSpacingPx(activeUiTheme().spacing.compact ?? STYLING_SEMIO_THEME.spacing.compact, rootPx));
}

/** 📐️ Resolves a DOM metric key to px at the compact reference root. */
export function domSizePx(key: Exclude<keyof typeof STYLING_DOM, "rootRemPx">, rootPx?: number): number {
  const value = activeUiTheme().metrics.dom?.[key] ?? STYLING_DOM[key];
  if (typeof value !== "number") throw new Error(`theme DOM metric ${key} must be scalar`);
  return uiSpacingPx(value, rootPx);
}

/** 📐️ Reads a resolved CSS size variable from the document (browser only). */
export function readSizeVarPx(varName: string, element?: Element | null): number {
  if (typeof document === "undefined") {
    return 0;
  }
  const host = element ?? document.documentElement;
  const raw = getComputedStyle(host).getPropertyValue(varName.startsWith("--") ? varName : `--${varName}`);
  return Number.parseFloat(raw) || 0;
}
