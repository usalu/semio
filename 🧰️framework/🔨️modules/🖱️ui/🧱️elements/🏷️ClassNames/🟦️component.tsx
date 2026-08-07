// #region 🧲️Header
// 💻️ framework/ui/elements/🫀️core/🏷️ClassNames/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { ClassValue, clsx } from "clsx";
import { extendTailwindMerge } from "tailwind-merge";
import * as React from "react";
import type { SurfaceScopeValue } from "../🌈️Surface/🟦️component.tsx";
import type { UiStatus } from "@semio-tech/ui-styling";
// #endregion 🔌️Adapters

//#region 🎨️ClassNames
/**
 * 🆔️ `cn`, split out of the ui-react barrel into its own `🧱️elements/` file (ticket
 * 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE) — NOT deferred to a later "core extraction" pass like the
 * rest of `🎼️Utilities`, because `ActionGroup`/`Toggle` call `cn(...)` at MODULE TOP LEVEL (inside a
 * top-level `cva(cn(...))` call), not inside a component body. A module-top-level read of a barrel-defined
 * `const` (here `twMergeUi`) re-exported by a barrel that in turn imports these same elements is a genuine
 * ES-module circular-import initialization-order bug: whichever module the loader reaches first in the
 * cycle sees the other's `const` still in its temporal dead zone (see `🧱️elements/🔌️Ports/🟦️component.tsx`'s
 * header comment for the sibling `reactHostPort` case). Elements that only call `cn(...)` inside function
 * bodies (the overwhelming majority) are unaffected — evaluation happens at render time, long after both
 * modules have finished loading — so only this symbol needed to move early.
 *
 * @emoji 🎨️ `ui-surface`/`ui-glass`/`ui-veil` are the only per-level fills — extending Tailwind's built-in
 * `bg-color` group makes them mutually exclusive with each other AND with every `bg-*` utility (same group
 * ⇒ last-in-`cn()` wins, both directions), so a fill composed after `bg-transparent` genuinely paints
 * instead of losing silently to CSS declaration-count ordering.
 */
const twMergeUi = extendTailwindMerge({
  extend: {
    classGroups: {
      "bg-color": ["ui-surface", "ui-glass", "ui-veil"],
    },
  },
});

/**
 * Merges CSS class names using Tailwind merge.
 **/
export function cn(...inputs: ClassValue[]) {
  return twMergeUi(clsx(inputs));
}
//#endregion 🎨️ClassNames

//#region 🎨️StyleClasses
/** @emoji 🌀️ Dashed, slow-spinning + gently pulsing waiting ring in the element's normal border color. */
export const waitingBorderClass = "border-waiting";

/** @emoji 🌀️ Waiting ring recolored to the active stroke; pair with selected/active elements. */
export const waitingBorderActiveClass = cn(waitingBorderClass, "border-waiting-active");

/** @emoji 🌀️ Clockwise spinning + pulsing loading ring in the element's normal border color. */
export const loadingBorderClass = "border-loading";

/** @emoji 🌀️ Loading ring recolored to the active stroke; pair with selected/active elements. */
export const loadingBorderActiveClass = cn(loadingBorderClass, "border-loading-active");

/** @emoji 🎨️ Shared transition for interactive chrome (hover, focus, active backgrounds). */
export const interactiveControlTransitionClass = "transition-[color,border-color,background-color]";

/** @emoji 🎯️ Focus/open on form controls: accent border color only, never extra ring width. */
export const formControlFocusBorderClass = cn("outline-none", interactiveControlTransitionClass, "focus-visible:border-accent data-[state=open]:border-accent aria-invalid:border-destructive focus-visible:ring-0 shadow-none");

/**
 * @emoji 🫳️ Hover-reactive utilities suppressed while a nested DragHandle is hovered — hovering the grip
 * then only highlights the grip, not the whole element. Pair with `{HANDLE_HOVER_SCOPE_ATTR}` on the same element
 * (the handle toggles `data-handle-hovered` on its nearest `data-hover-scope` ancestor via plain DOM writes, no
 * re-render). Deliberately avoids `:has()` — it isn't reliably supported across every environment this ships to
 * (older embedded webviews), and `:has()`-based ancestor exclusion also matches ANY ancestor with a matching
 * class, not necessarily the nearest one, which is wrong once tree rows nest.
 *
 * These MUST be written as complete literal strings, not built via `${}` interpolation in a helper function —
 * Tailwind's build only discovers classes by scanning source files for literal text, it never executes JS, so a
 * class name assembled from a template placeholder at runtime is invisible to it and silently generates no CSS
 * at all (this broke hover entirely here once already).
 */
const hoverExcludingHandleBgFillClass = "hover:not-data-[handle-hovered=true]:bg-hover-interactive-fill";
const hoverExcludingHandleActiveBgClass = "hover:not-data-[handle-hovered=true]:bg-active-base/90";
const hoverExcludingHandleActiveBorderClass = "hover:not-data-[handle-hovered=true]:border-active-base";

export const groupHoverExcludingHandleBgFillClass = "group-hover/tree-row:not-group-data-[handle-hovered=true]/tree-row:bg-hover-interactive-fill";

export const hoverExcludingHandleTextEmphasizedClass = "hover:not-data-[handle-hovered=true]:text-emphasized";

/** @emoji 🎨️ Normal-border gray fill for interactive hover states. */
export const interactiveHoverFillClass = "hover:bg-hover-interactive-fill";

/** @emoji 🎨️ Interactive hover: normal-border fill + emphasized content. */
export const interactiveHoverClass = cn(interactiveHoverFillClass, "hover:text-emphasized");

/** @emoji 📏️ Active stroke paired with {@link interactiveActiveFillClass}. */
export const interactiveActiveBorderClass = "border-active-base";

/** @emoji 🎨️ Shared active fill for pressed tabs, toggles, and nav selection. */
export const interactiveActiveFillClass = cn("bg-active-base", interactiveActiveBorderClass, "text-emphasized", hoverExcludingHandleActiveBgClass, hoverExcludingHandleActiveBorderClass, hoverExcludingHandleTextEmphasizedClass);

/** @emoji 🎨️ Active/on: primary fill + active border + emphasized content (never the transient hover fill). */
export const interactiveOnClass = cn(
  "data-[state=on]:bg-active-base",
  "data-[state=on]:border-active-base",
  "data-[state=on]:text-emphasized",
  "data-[state=on]:hover:bg-active-base/90",
  "data-[state=on]:hover:border-active-base",
  "data-[state=on]:hover:text-emphasized",
);

/** @emoji 🎨️ Active tab: primary fill + active border + emphasized content. */
export const interactiveTabActiveClass = cn(
  "data-[state=active]:bg-active-base",
  "data-[state=active]:border-active-base",
  "data-[state=active]:text-emphasized",
  "data-[state=active]:hover:bg-active-base/90",
  "data-[state=active]:hover:border-active-base",
  "data-[state=active]:hover:text-emphasized",
);

/** @emoji 🚫️ React props that disable native browser affordances on editable UI controls. */
export const uiFormControlBrowserDefaultProps = {
  autoComplete: "off",
  autoCorrect: "off",
  autoCapitalize: "off",
  spellCheck: false,
  "data-1p-ignore": true,
  "data-lpignore": "true",
} as const satisfies Pick<React.InputHTMLAttributes<HTMLInputElement>, "autoComplete" | "autoCorrect" | "autoCapitalize" | "spellCheck"> & { readonly "data-1p-ignore": boolean; readonly "data-lpignore": string };

/** @emoji 📏️ Subtle normal stroke for controls, windows, dividers, and in-chrome separators. */
export const borderNormalClass = "!border-normal";

/** @emoji 📏️ Normal bottom edge utility for in-chrome dividers (not shell navbar — navbar uses a CSS `::after` stroke). */
export const borderNormalBottomClass = `border-b ${borderNormalClass}`;

/** @emoji 📏️ Implicit element border color (controls, dropdowns, dividers). */
export const borderElementClass = "border-element";

/** @emoji 🎨️ Opaque per-level fill — background-color only, no blur (see `[data-level]` cascade in 🎨️ui.css). */
export const surfaceClass = "ui-surface";

export const glassClass = "ui-glass";

/** @emoji 🎨️ Fullscreen scrim; host element must carry `data-level="dialog"` for correct tint. */
export const veilClass = "ui-veil";

/** @emoji 📋️ Hover row styling for menus, selects, comboboxes, and context menus. */
export const menuListItemClassName = cn(
  "text-element",
  interactiveHoverClass,
  "focus:bg-hover-interactive-fill focus:text-emphasized",
  "data-[active=true]:bg-hover-interactive-fill data-[active=true]:text-emphasized",
  "data-[selected=true]:bg-active-base data-[selected=true]:border-active-base data-[selected=true]:text-emphasized",
);

/** @emoji 🎨️ Whether a base-floor chrome row (navbar/footer/canvas/mode-body) must paint its own
 * {@link surfaceClass}, or stay transparent so Layout's one continuous base surface shows through.
 * Nested same-level paints are the "navbar ≠ canvas ≠ footer" bug class — one base floor, one fill. */
export function shellFloorPaints(parent: SurfaceScopeValue | null): boolean {
  return !(parent?.level === "base" && parent.fill !== "none");
}

/** @emoji 🎨️ Fill class for base-floor chrome — {@link surfaceClass} when standalone, transparent on Layout's painted base. */
export function shellFloorFillClass(parent: SurfaceScopeValue | null): string {
  return shellFloorPaints(parent) ? surfaceClass : "bg-transparent";
}

/** @emoji Re-export private handle-hover fill for chrome tab cells still composing in the barrel. */
export { hoverExcludingHandleBgFillClass };

//#region 🎨️ChromeControlClasses
/** @emoji 🌀️ Waiting ring matching the element current state color; empty when not waiting. */
export function waitingBorderStateClass(waiting: boolean, active = false): string {
  return waiting ? (active ? waitingBorderActiveClass : waitingBorderClass) : "";
}

/** @emoji 🌀️ Loading ring matching the element current state color; empty when not loading. */
export function loadingBorderStateClass(loading: boolean, active = false): string {
  return loading ? (active ? loadingBorderActiveClass : loadingBorderClass) : "";
}

/** @emoji 🌀️ Maps shell chrome UiStatus to the shared border ring utilities. */
export function chromeStatusBorderClass(status: UiStatus | undefined, active = false): string {
  if (status === "loading") return loadingBorderStateClass(true, active);
  if (status === "waiting") return waitingBorderStateClass(true, active);
  return "";
}

/** @emoji 🎛️ Shared control cell base — transparent on the group glass. */
export const chromeControlItemBaseClass = cn(
  "text-element inline-flex items-center justify-center gap-single text-xs font-medium bg-transparent",
  "cursor-selectable disabled:pointer-events-none disabled:opacity-50 disabled:cursor-not-allowed",
  "[&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-small [&_svg]:shrink-0",
  formControlFocusBorderClass,
  "whitespace-nowrap h-medium p-single overflow-hidden leading-none",
);

/** @emoji 🎛️ Navbar/button/toggle cell hover. */
export const chromeControlItemClass = cn(chromeControlItemBaseClass, interactiveHoverClass);

/** @emoji 🎛️ Tab/chip cell hover — preserves drag-handle exclusion beside labels. */
export const chromeControlTabItemClass = cn(chromeControlItemBaseClass, hoverExcludingHandleBgFillClass, hoverExcludingHandleTextEmphasizedClass);


/** @emoji 📑️ Default mode-dock tab label. */
export const modeDockTabClassName = cn(chromeControlTabItemClass, "group max-w-[12rem] shrink-0 cursor-pointer items-center px-single select-none transition-colors");

/** @emoji 📑️ Pane chrome toggle class. */
export const windowPaneChromeToggleClass = cn(
  modeDockTabClassName,
  "relative z-30 box-border min-h-medium shrink-0 border-0 bg-transparent",
  "outline-none focus-visible:ring-1 focus-visible:ring-inset focus-visible:ring-active-base",
  "disabled:pointer-events-none disabled:opacity-50",
);

/** @emoji 🌀️ Loading ring in the level-aware element border color. */
export const loadingBorderElementClass = cn(loadingBorderClass, "border-loading-element");

/** @emoji 🌀️ Waiting ring in the level-aware element border color. */
export const waitingBorderElementClass = cn(waitingBorderClass, "border-waiting-element");

/** @emoji 🎛️ Shared outer chrome shell for chips, buttons, and toggles — glass group with hairline dividers. */
export const chromeControlGroupShellClass = cn("flex items-center border divide-x overflow-hidden w-fit shrink-0", borderNormalClass, "divide-normal", glassClass);

/** @emoji 🎛️ Standard {@link chromeControlGroupShellClass} height for chips, buttons, and toggles. */
export const chromeControlGroupClass = cn(chromeControlGroupShellClass, "h-medium");

/** @emoji 🎛️ Pressed/on via `data-state="on"` — toggles and toggle-group items. */
export const chromeControlItemOnClass = interactiveOnClass;

/** @emoji 🎛️ Pressed/on via `data-active="true"` — panel/window tab cells. */
export const chromeControlTabActiveClass = cn(
  "data-[active=true]:bg-active-base",
  "data-[active=true]:border-active-base",
  "data-[active=true]:text-emphasized",
  "data-[active=true]:hover:bg-active-base/90",
  "data-[active=true]:hover:border-active-base",
  "data-[active=true]:hover:text-emphasized",
);

/** @emoji 🎚️ Slider filled range — element gray at rest; foreground emphasis on hover; active fill while dragging. */
export const sliderRangeClassName = cn("bg-element absolute transition-[background-color] data-[orientation=horizontal]:h-full data-[orientation=vertical]:w-full", "group-hover:bg-emphasized", "data-[dragging=true]:bg-active-base");

/** @emoji 🎚️ Slider ready extent — secondary highlight from the knob to the preloaded/ready value on a fixed range. */
export const sliderReadyClassName = cn("bg-[var(--accent-secondary)] pointer-events-none absolute data-[orientation=horizontal]:h-full data-[orientation=vertical]:w-full");

/** @emoji 🎚️ Slider thumb — element border at rest; hover fill; primary fill when dragging/focused. */
export const sliderThumbClassName = cn(
  "block size-small shrink-0 rounded-[9999px] bg-element transition-[background-color] outline-hidden",
  "hover:bg-emphasized group-hover:bg-emphasized",
  "focus-visible:bg-active-base focus-visible:ring-0",
  "data-[dragging=true]:bg-active-base",
  "disabled:pointer-events-none disabled:opacity-50",
);

/** @emoji 🎚️ Slider numeric readout — element gray at rest. */
export const sliderValueClassName = cn("text-element w-large text-end text-xs leading-none select-none transition-colors", "hover:text-emphasized group-hover:text-emphasized");

/** @emoji 📊 Interactive table row — element text + hover fill. */
export const tableRowInteractiveClass = cn("text-element", interactiveControlTransitionClass, interactiveHoverClass);

/** @emoji 📊 Selected table row fill. */
export const tableRowSelectedClass = interactiveActiveFillClass;
//#endregion 🎨️ChromeControlClasses

//#endregion 🎨️StyleClasses


