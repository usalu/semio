import { cn } from "@semio-tech/ui/modules/class-name-composition";
import { glassClass, surfaceClass } from "@semio-tech/ui/modules/surface-presentation";
import type { FlowInline } from "@semio-tech/ui/modules/flow-direction-context";
import type { IconName } from "@semio-tech/assets";

/** 📏️ Emphasized shell stroke for active/selected chrome accents. */
export const borderEmphasizedClass = "!border-emphasized";

/** 🥞️ Active window chrome line when that stack is globally active. */
export const activeLineClass = "border-active-base";

/** 📋️ Frosted floating menu/popover surface for technology renderer overlays — menu tier; host element must also carry `data-level="menu"` and its outline is a compositor frame (`data-shared-border-frame`), never a drawn border. */
export const floatingMenuSurfaceClass = cn(glassClass, "overflow-hidden text-element");

/** 🪟️ Frosted editor aside chrome for technology renderers — pane-level chrome; host element must also carry `data-level="pane"`. Carries no border, gap, padding or scroll of its own: render it as a vertical `SharedBorderRow` whose cells hold the padding and whose scroller sits inside. */
export const floatingPaneAsideClass = cn("relative flex shrink-0 flex-col text-element z-[2]", glassClass);

/** 🎀️ Frosted compact ribbon chrome (projection switch, align controls) — a glass chip inside its frame; host element must also carry `data-level="window"` and `data-glass-chip`, and draws no border of its own. */
export const floatingRibbonSurfaceClass = cn("overflow-hidden text-element", glassClass);

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
