import * as React from "react";
import { cn } from "../../../🔨️modules/🏷️class-name-composition/🟦️.ts";

/** 🦶️ Chip content for the bottom edge of a canvas frame: each entry brings its own glass surface (a chip-only window chrome) and is overlaid inside the canvas rectangle. */
export interface CanvasBottomChips {
  readonly start?: React.ReactNode;
  readonly center?: React.ReactNode;
  readonly end?: React.ReactNode;
}

const PLACEMENTS = ["start", "center", "end"] as const;

const PLACEMENT_CLASS = { start: "justify-self-start", center: "justify-self-center", end: "justify-self-end" } as const;

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
        <div
          key={placement}
          data-slot={`canvas-footer-${placement}`}
          data-canvas-chip=""
          data-dock="bottom"
          className={cn("pointer-events-auto flex min-w-0 items-stretch", PLACEMENT_CLASS[placement], placement === "start" ? "col-start-1" : placement === "center" ? "col-start-2" : "col-start-3")}
        >
          {chips![placement]}
        </div>
      ))}
    </div>
  );
};
