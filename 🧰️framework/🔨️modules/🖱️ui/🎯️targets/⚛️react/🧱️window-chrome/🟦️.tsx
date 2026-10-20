import { ContextMenuChrome } from "@semio-tech/ui/elements/ContextMenu/chrome";
import { cn } from "@semio-tech/ui/modules/class-name-composition";
import { interactiveHoverClass } from "@semio-tech/ui/modules/interaction-presentation";
import { domSizePx } from "@semio-tech/ui-styling";
import { reactHostPort } from "@semio-tech/ui/elements/Ports/react";
import * as React from "react";
import { useWindowContentDeadLineInset, windowContentDeadLineScrollClass } from "@semio-tech/ui/elements/WindowContentDeadLine";
import { borderElementClass } from "@semio-tech/ui/modules/border-presentation";

/** 🪧️ Uppercase shell section title — element gray at rest. */
export const shellChromeSectionTitleClassName = "text-2xs font-semibold uppercase tracking-wide text-element";

/** 📏️ Multi-tab controls cap — chip glass only; U-gap stays a clear punch-through. */
export const windowControlsCapActiveSplitClass = "relative flex shrink-0 items-stretch border-0 bg-transparent text-element";

ContextMenuChrome.displayName = "ContextMenuChrome";

/** 🪟️ Window chrome icon button — element gray by default, emphasize on hover. */
export const windowChromeControlButtonClass = cn("flex size-medium items-center justify-center border-0 bg-transparent transition-colors", interactiveHoverClass);

/** 📐️ Max width cap for window engagement (token-derived). */
export const windowEngagementMaxWidthPx = domSizePx("layoutEngagementMaxUiSpacing");

/** 🚧️ Scroll surface that keeps window content below floating chrome and preserves explicit edgeless layouts. */
export const ChromeAwareWindowScrollSurface = reactHostPort.forwardRef<HTMLDivElement, React.ComponentPropsWithoutRef<"div">>(({ className, children, ...props }, ref) => {
  const scrollerRef = reactHostPort.useRef<HTMLDivElement | null>(null);
  const setScrollerRef = reactHostPort.useCallback(
    (node: HTMLDivElement | null) => {
      scrollerRef.current = node;
      if (typeof ref === "function") ref(node);
      else if (ref) ref.current = node;
    },
    [ref],
  );
  useWindowContentDeadLineInset(scrollerRef);
  return (
    <div ref={setScrollerRef} data-slot="window-dead-line-scroll" className={cn("min-h-0 min-w-0 overflow-auto", windowContentDeadLineScrollClass, className)} {...props}>
      {children}
    </div>
  );
});
ChromeAwareWindowScrollSurface.displayName = "ChromeAwareWindowScrollSurface";

/** 📐️ Labelled icon action in window rail chrome bars (options + action). */
export const windowRailChromeLabelActionClass = cn("flex h-medium w-auto items-center justify-center border-0 bg-transparent text-element px-single gap-single", interactiveHoverClass);

/** 📐️ Vertical rhythm between top-level measure groups in the rail. */
export const windowMeasuresStackInnerClass = "flex w-full min-w-0 flex-col gap-tiny";

/** 📐️ Single measure tile in the window rail — transparent so the {@link WindowChrome} body glass shows through, never a second glass layer of its own. */
export const windowMeasureTileClass = cn("pointer-events-auto select-none bg-transparent w-full min-w-0 shrink-0 rounded-sm border", `${borderElementClass}/40`, "px-tiny py-tiny");

/** 📐️ Optional measure caption above a control. */
export const windowMeasureLabelClass = "text-muted-foreground mb-tiny block min-w-0 truncate text-2xs font-medium leading-none";

/** 📐️ Measure section title without a heavy chrome box. */
export const windowMeasureSectionClass = "text-muted-foreground w-full truncate px-single py-tiny text-center text-2xs font-medium uppercase tracking-wide";

/** 📐️ Constrains measure controls to the rail width. */
export const windowMeasureControlClass = "w-full min-w-0 max-w-full";

/** 🌳️ Compact disclosure header for a nested measure group. */
export const windowMeasureGroupHeaderClass = "pointer-events-auto flex h-small w-full min-w-0 shrink-0 cursor-pointer select-none items-center gap-tiny rounded-sm px-tiny py-0 text-element hover:bg-hover-interactive-fill hover:text-emphasized";

/** 🌳️ Indented children under a measure group (minimal chrome). */
export const windowMeasureGroupChildrenClass = "pointer-events-none flex w-full min-w-0 flex-col gap-0 border-s ps-tiny ms-tiny pb-0 pt-0";

/** 🌳️ Nested measure leaf without an outer tile border (indent only). */
export const windowMeasureTileNestedClass = "pointer-events-auto select-none w-full min-w-0 shrink-0 px-0 py-0";

/** 📐️ Toggle sized to fill the measure tree row (active fill spans full width). */
export const windowMeasureToggleClass =
  "!w-full min-w-0 max-w-full [&_[data-slot=toggle-group-item]]:!flex-1 [&_[data-slot=toggle-group-item]]:min-w-0 [&_[data-slot=toggle-group-item]]:max-w-full [&_[data-slot=toggle-group-item]]:!aspect-auto [&_[data-slot=toggle-group-item]]:!shrink [&_[data-slot=inline-label]]:min-w-0 [&_[data-slot=inline-label]]:truncate";

/** 📐️ Dense toggle row for nested measure groups (shorter control chrome). */
export const windowMeasureToggleCompactClass =
  "[&_[data-slot=toggle-group]]:h-small [&_[data-slot=toggle-group-item]]:min-h-0 [&_[data-slot=toggle-group-item]]:py-tiny [&_[data-slot=toggle-group-item]]:px-single [&_[data-slot=inline-label]]:!text-tiny";
