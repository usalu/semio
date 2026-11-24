import * as React from "react";
import { cn } from "../../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { type Anchor } from "../../../🔨️modules/⚓️panel-anchor/🟦️.ts";
import { publishShellChromePanelBox, safeAreaBoxFromRect } from "../../../🔨️modules/🛟️chrome-safe-area/🟦️.tsx";
import { useShellScopeOptional } from "../../🐚️ShellScope/🟦️.tsx";

/** 🦶️ Chip content for the bottom edge of a canvas frame: each entry brings its own glass surface (a chip-only window chrome) and is overlaid inside the canvas rectangle. */
export interface CanvasBottomChips {
  readonly start?: React.ReactNode;
  readonly center?: React.ReactNode;
  readonly end?: React.ReactNode;
}

const PLACEMENTS = ["start", "center", "end"] as const;

const PLACEMENT_ANCHOR = { start: "bottom-left", center: "bottom-middle", end: "bottom-right" } as const satisfies Record<(typeof PLACEMENTS)[number], Anchor>;

const PLACEMENT_CLASS = { start: "justify-self-start", center: "justify-self-center", end: "justify-self-end" } as const;

/**
 * 🛟️ One chip slot of the bottom row, published as an occupied chrome box so bottom-anchored window chrome steps clear of it
 * instead of sitting underneath, whatever the chip widths or window layout.
 * @see {@link publishShellChromePanelBox}
 */
const CanvasBottomChipSlot: React.FC<{ placement: (typeof PLACEMENTS)[number]; children: React.ReactNode }> = ({ placement, children }) => {
  const shellRoot = useShellScopeOptional()?.rootRef.current ?? undefined;
  const slotRef = React.useRef<HTMLDivElement>(null);
  const key = `canvas-chip:${React.useId()}`;
  React.useLayoutEffect(() => {
    const slot = slotRef.current;
    if (!slot) return;
    const sync = () => publishShellChromePanelBox(shellRoot, key, PLACEMENT_ANCHOR[placement], safeAreaBoxFromRect(slot.getBoundingClientRect()));
    sync();
    const observer = typeof ResizeObserver !== "undefined" ? new ResizeObserver(sync) : null;
    observer?.observe(slot);
    window.addEventListener("resize", sync);
    return () => {
      observer?.disconnect();
      window.removeEventListener("resize", sync);
      publishShellChromePanelBox(shellRoot, key, PLACEMENT_ANCHOR[placement], null);
    };
  }, [key, placement, shellRoot]);
  return (
    <div
      ref={slotRef}
      data-slot={`canvas-footer-${placement}`}
      data-canvas-chip=""
      data-dock="bottom"
      className={cn("pointer-events-auto flex min-w-0 items-stretch", PLACEMENT_CLASS[placement], placement === "start" ? "col-start-1" : placement === "center" ? "col-start-2" : "col-start-3")}
    >
      {children}
    </div>
  );
};

/**
 * 🦶️ Bottom-edge chip row of a canvas frame. An absolute, pointer-transparent overlay inside the canvas rectangle: the canvas
 * content stays full-bleed beneath it, each chip slot re-enables pointer input and the uncovered row is canvas.
 * @see https://developer.mozilla.org/docs/Web/CSS/pointer-events
 */
export const CanvasBottomChipRow: React.FC<{ chips?: CanvasBottomChips }> = ({ chips }) => {
  const present = PLACEMENTS.filter((placement) => chips?.[placement] !== undefined && chips[placement] !== null && chips[placement] !== false);
  if (present.length === 0) return null;
  return (
    <div data-slot="canvas-footer" data-dock="bottom" className="pointer-events-none absolute inset-x-0 bottom-0 z-10 grid grid-cols-[minmax(0,1fr)_auto_minmax(0,1fr)] items-end">
      {present.map((placement) => (
        <CanvasBottomChipSlot key={placement} placement={placement}>
          {chips![placement]}
        </CanvasBottomChipSlot>
      ))}
    </div>
  );
};
