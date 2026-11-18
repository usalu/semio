import { borderNormalClass } from "@semio-tech/ui/modules/border-presentation";
import { cn } from "@semio-tech/ui/modules/class-name-composition";
import { glassClass, surfaceClass } from "@semio-tech/ui/modules/surface-presentation";
import type { FlowInline } from "@semio-tech/ui/modules/flow-direction-context";
import type { IconName } from "@semio-tech/assets";

/** 📏️ Emphasized shell stroke for active/selected chrome accents. */
export const borderEmphasizedClass = "!border-emphasized";

/** 🖼️ Emphasized chrome frame (`border` + {@link borderEmphasizedClass}). */
export const borderEmphasizedFrameClass = `box-border border border-solid ${borderEmphasizedClass}`;

/** ⬇️ Emphasized navbar bottom edge. */
export const borderEmphasizedBottomClass = `border-b ${borderEmphasizedClass}`;

/** 🔝️ Emphasized footer top edge. */
export const borderEmphasizedTopClass = `border-t ${borderEmphasizedClass}`;

/** 🔲️ Subtle normal stroke for controls, windows, dividers, and in-chrome separators. */

/** 🔳️ Normal chrome frame (`border` + {@link borderNormalClass}); panel/pane hosts use a compositor `data-shared-border-frame`. */
export const borderNormalFrameClass = `box-border border border-solid ${borderNormalClass}`;

/** 🥞️ Active window chrome line when that stack is globally active. */
export const activeLineClass = "border-active-base";

/** 🐚️ Shell outline — normal gray frame; emphasized while the pointer is inside the parent `[data-slot="panel"]` / `[data-slot="pane"]`. */
export const shellChromeBorderClass = borderNormalFrameClass;

/** 📋️ Frosted floating menu/popover surface for technology renderer overlays — menu tier; host element must also carry `data-level="menu"` (Radix-portal-style consumers stamp their own content root). */
export const floatingMenuSurfaceClass = cn(glassClass, "overflow-hidden rounded-md border shadow-sm text-element", borderNormalClass);

/** 🪟️ Frosted editor aside chrome for technology renderers — pane-level chrome; host element must also carry `data-level="pane"`. Carries no border, gap, padding or scroll of its own: render it as a vertical `SharedBorderRow` whose cells hold the padding and whose scroller sits inside. */
export const floatingPaneAsideClass = cn("relative flex shrink-0 flex-col text-element z-[2]", glassClass);

/** 🎀️ Frosted compact ribbon chrome (projection switch, align controls) — window-level chrome; host element must also carry `data-level="window"`. */
export const floatingRibbonSurfaceClass = cn("overflow-hidden rounded-md border shadow-sm text-element", borderNormalClass, glassClass);

/** 🏠️ Golden-window host root for technology canvases inside {@link ProductShell} — base level (the floor windows float on); host element must also carry `data-level="base"`. */
export const canvasHostRootClass = cn("relative flex h-full min-h-0 w-full min-w-0 flex-col text-element font-sans", surfaceClass);

/** 📝️ Full-viewport standalone editor shell (outside golden windows) — base level. */
export const editorShellRootClass = cn("text-element flex h-screen min-h-0 w-full flex-row font-sans", surfaceClass);

/** 🎨️ Canvas viewport surface inside a host root. */
export const canvasViewportClass = cn("relative h-full min-h-0 w-full min-w-0 outline-none", surfaceClass);

/** 🧭️ Chevron {@link IconName} that points toward where a fold's content is — the opposite state's chevron, mirrored for {@link FlowInline} `"rtl"`. `pointsOut` is `true` when the fold is collapsed (chevron points toward the hidden content) and `false` when expanded (chevron points back at the fold). */
export function flowChevronIconName(inline: FlowInline, pointsOut: boolean): IconName {
  const right = inline === "rtl" ? pointsOut : !pointsOut;
  return right ? "chevron-right" : "chevron-left";
}
