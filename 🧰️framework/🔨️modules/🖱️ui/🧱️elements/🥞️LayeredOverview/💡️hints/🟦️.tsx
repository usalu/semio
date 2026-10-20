/** 💡️ The swipe hints of LayeredOverview: the neighbouring page and its direction at each edge of the view. */
import type { LayeredDirection, LayeredLanding } from "../../../🔨️modules/🥞️layered-overview-geometry/🟦️.ts";
import { type IconName, Icon } from "../../🔣️Icons/🟦️.tsx";
import * as React from "react";
import { cn } from "../../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import type { LayeredInsets, LayeredLabels, LayeredPane } from "../📋️types/🟦️.ts";

const HINT_ICONS: { readonly [D in LayeredDirection]: IconName } = { up: "arrow-up", left: "arrow-left", right: "arrow-right", down: "arrow-down" };
const HINT_PLACES: { readonly [D in LayeredDirection]: string } = {
  up: "left-1/2 -translate-x-1/2 max-w-[calc(100%-6rem)] min-h-[28px] flex-row px-single",
  down: "left-1/2 -translate-x-1/2 max-w-[calc(100%-6rem)] min-h-[28px] flex-row px-single",
  left: "top-1/2 -translate-y-1/2 max-h-[45%] w-[28px] flex-col py-single",
  right: "top-1/2 -translate-y-1/2 max-h-[45%] w-[28px] flex-col py-single",
};
const HINT_TEXT: { readonly [D in LayeredDirection]: string } = {
  up: "min-w-0 truncate",
  down: "min-w-0 truncate",
  left: "min-h-0 overflow-hidden text-ellipsis whitespace-nowrap [writing-mode:vertical-rl] rotate-180",
  right: "min-h-0 overflow-hidden text-ellipsis whitespace-nowrap [writing-mode:vertical-rl]",
};

/** 📏️ Where the hint of `direction` stands: inside the app's chrome at the top and the bottom, in the room the cells keep free at the sides. */
function hintPlace(direction: LayeredDirection, insets: Required<LayeredInsets>): React.CSSProperties {
  switch (direction) {
    case "up":
      return { top: `calc(${insets.top} + var(--spacing-single))` };
    case "down":
      return { bottom: `calc(${insets.bottom} + var(--spacing-single))` };
    case "left":
      return { left: "4px" };
    case "right":
      return { right: "4px" };
  }
}

/** 🧭️ One page one swipe away from the resting page and the direction it lies in. */
export interface LayeredNeighbour extends LayeredLanding {
  readonly direction: LayeredDirection;
  readonly pane: LayeredPane;
}

/** 🧩️ The hints of the swiped overview: at each edge of the view behind which a page lies, that page's label with an arrow — the page a swipe
 * the other way brings in — at the top and bottom edges across, at the sides along them. Each hint is a button that goes there. While the strip
 * moves the hints fade out (and leave the focus order), so they always name the neighbours of the page at rest.
 * @see https://www.w3.org/WAI/WCAG22/Understanding/pointer-gestures.html — every swipe also has a single-pointer way */
export const LayeredNeighbourHints = /* @__PURE__ */ React.memo(function LayeredNeighbourHints(props: {
  readonly neighbours: readonly LayeredNeighbour[];
  readonly moving: boolean;
  readonly insets: Required<LayeredInsets>;
  readonly label: LayeredLabels["neighbour"];
  readonly onGo: (neighbour: LayeredNeighbour) => void;
}): React.ReactElement {
  return (
    <>
      {props.neighbours.map((neighbour) => (
        <button
          key={neighbour.direction}
          type="button"
          data-layered-neighbour={neighbour.direction}
          data-pane={neighbour.pane.id}
          data-level="dialog"
          aria-label={props.label(neighbour.pane, neighbour.direction)}
          aria-hidden={props.moving || undefined}
          tabIndex={props.moving ? -1 : undefined}
          onClick={() => props.onGo(neighbour)}
          className={cn(
            "ui-glass absolute z-[32] inline-flex items-center justify-center gap-[4px] border border-border-normal text-xs font-medium text-foreground outline-none transition-opacity duration-150 hover:border-border-emphasized focus-visible:ring-2 focus-visible:ring-ring",
            HINT_PLACES[neighbour.direction],
            props.moving && "pointer-events-none opacity-0",
          )}
          style={hintPlace(neighbour.direction, props.insets)}
        >
          <Icon icon={HINT_ICONS[neighbour.direction]} size="small" className="shrink-0" />
          <span className={HINT_TEXT[neighbour.direction]}>{neighbour.pane.shortLabel ?? neighbour.pane.label}</span>
        </button>
      ))}
    </>
  );
});
