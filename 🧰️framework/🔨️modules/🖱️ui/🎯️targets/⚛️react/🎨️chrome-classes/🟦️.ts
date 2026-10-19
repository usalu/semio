import { borderNormalClass } from "../../../🔨️modules/📏️border-presentation/🟦️.ts";
import { cn } from "../../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { glassClass, surfaceClass } from "../../../🔨️modules/🌈️surface-presentation/🟦️.ts";
import type { FlowInline } from "../../../🔨️modules/🧭️flow-direction-context/🟦️.tsx";
import type { IconName } from "@semio-tech/assets";

/** 📏️ Emphasized shell stroke for active/selected chrome accents. */
export const borderEmphasizedClass = "!border-emphasized";

/** 📏️ Emphasized chrome frame (`border` + {@link borderEmphasizedClass}). */
export const borderEmphasizedFrameClass = `box-border border border-solid ${borderEmphasizedClass}`;

/** 📏️ Emphasized navbar bottom edge. */
export const borderEmphasizedBottomClass = `border-b ${borderEmphasizedClass}`;

/** 📏️ Emphasized footer top edge. */
export const borderEmphasizedTopClass = `border-t ${borderEmphasizedClass}`;

/** 📏️ Subtle normal stroke for controls, windows, dividers, and in-chrome separators. */

/** 📏️ Normal chrome frame (`border` + {@link borderNormalClass}); panel/pane hosts prefer {@link shellChromeFrameLayerClass} + CSS parent-hover. */
export const borderNormalFrameClass = `box-border border border-solid ${borderNormalClass}`;

/** 📏️ Active window chrome line when that stack is globally active. */
export const activeLineClass = "border-active-base";

/** 🪟️ Shell outline — normal gray frame; emphasized while the pointer is inside the parent `[data-slot="panel"]` / `[data-slot="pane"]`. */
export const shellChromeBorderClass = borderNormalFrameClass;

/** 🪟️ Frosted floating menu/popover surface for technology renderer overlays — menu tier; host element must also carry `data-level="menu"` (Radix-portal-style consumers stamp their own content root). */
export const floatingMenuSurfaceClass = cn(glassClass, "overflow-hidden rounded-md border shadow-sm text-element", borderNormalClass);

/** 🪟️ Frosted editor aside chrome for technology renderers — pane-level chrome; host element must also carry `data-level="pane"`. */
export const floatingPaneAsideClass = cn("relative flex shrink-0 flex-col gap-single overflow-auto p-double text-element z-[2]", shellChromeBorderClass, glassClass);

/** 🪟️ Frosted compact ribbon chrome (projection switch, align controls) — window-level chrome; host element must also carry `data-level="window"`. */
export const floatingRibbonSurfaceClass = cn("overflow-hidden rounded-md border shadow-sm text-element", borderNormalClass, glassClass);

/** 🪟️ Frosted inline field/action shell inside editor asides — menu tier (fallback); host element must also carry `data-level="menu"`. */
export const floatingFieldSurfaceClass = cn(glassClass, "relative overflow-visible rounded-md border", borderNormalClass);

/** 🪟️ Golden-window host root for technology canvases inside {@link ProductShell} — base level (the floor windows float on); host element must also carry `data-level="base"`. */
export const canvasHostRootClass = cn("relative flex h-full min-h-0 w-full min-w-0 flex-col text-element font-sans", surfaceClass);

/** 🪟️ Full-viewport standalone editor shell (outside golden windows) — base level. */
export const editorShellRootClass = cn("text-element flex h-screen min-h-0 w-full flex-row font-sans", surfaceClass);

/** 🏷️ Toggle chip for layer/filter controls in technology renderers. */
export const floatingTagClass = cn("inline-flex items-center gap-half rounded-full border px-half py-0.5 text-xs text-element", borderNormalClass);

export const floatingTagOnClass = "bg-accent text-accent-foreground";
export const floatingTagOffClass = "bg-transparent text-muted-foreground";

/** 🪟️ Canvas viewport surface inside a host root. */
export const canvasViewportClass = cn("relative h-full min-h-0 w-full min-w-0 outline-none", surfaceClass);

/** 🧭️ Chevron {@link IconName} that points toward where a fold's content is — the opposite state's chevron, mirrored for {@link FlowInline} `"rtl"`. `pointsOut` is `true` when the fold is collapsed (chevron points toward the hidden content) and `false` when expanded (chevron points back at the fold). */
export function flowChevronIconName(inline: FlowInline, pointsOut: boolean): IconName {
  const right = inline === "rtl" ? pointsOut : !pointsOut;
  return right ? "chevron-right" : "chevron-left";
}
