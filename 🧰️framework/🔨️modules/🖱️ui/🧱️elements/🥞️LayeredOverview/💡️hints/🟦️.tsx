/** 💡️ The swipe hints of LayeredOverview: the neighbouring page and its direction at each edge of the view. */
import type { LayeredDirection, LayeredLanding } from "../../../🔨️modules/🥞️layered-overview-geometry/🟦️.ts";
import { type IconName, Icon } from "../../🔣️Icons/🟦️.tsx";
import * as React from "react";
import { cn } from "../../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { useSharedBorderFrameRef } from "../../🗂️WindowChrome/shared-border.ts";
import type { LayeredInsets, LayeredLabels, LayeredPane } from "../📋️types/🟦️.ts";

const HINT_ICONS: { readonly [D in LayeredDirection]: IconName } = { up: "arrow-up", left: "arrow-left", right: "arrow-right", down: "arrow-down" };
const HINT_PLACES: { readonly [D in LayeredDirection]: string } = {
  up: "inset-x-0 h-[28px] flex-row px-single",
  down: "inset-x-0 h-[28px] flex-row px-single",
  left: "w-[28px] flex-col py-single",
  right: "w-[28px] flex-col py-single",
};
const HINT_TEXT: { readonly [D in LayeredDirection]: string } = {
  up: "min-w-0 truncate",
  down: "min-w-0 truncate",
  left: "min-h-0 overflow-hidden text-ellipsis whitespace-nowrap [writing-mode:vertical-rl] rotate-180",
  right: "min-h-0 overflow-hidden text-ellipsis whitespace-nowrap [writing-mode:vertical-rl]",
};

/** 📏️ Where the hint of `direction` stands: a bar filling its whole 28 px band, flush with the app's chrome at the top and the bottom and with the view's edge at the sides, so the band holds no gap between the chrome (or the edge) and the card. The top and bottom bars span the full width, the side bars the height between them. */
function hintPlace(direction: LayeredDirection, insets: Required<LayeredInsets>, directions: ReadonlySet<LayeredDirection>): React.CSSProperties {
  switch (direction) {
    case "up":
      return { top: insets.top };
    case "down":
      return { bottom: insets.bottom };
    case "left":
    case "right":
      return {
        [direction]: 0,
        top: directions.has("up") ? `calc(${insets.top} + 28px)` : insets.top,
        bottom: directions.has("down") ? `calc(${insets.bottom} + 28px)` : insets.bottom,
      };
  }
}

/** 🧭️ One page one swipe away from the resting page and the direction it lies in. */
export interface LayeredNeighbour extends LayeredLanding {
  readonly direction: LayeredDirection;
  readonly pane: LayeredPane;
}

const LayeredNeighbourHint = /* @__PURE__ */ React.memo(function LayeredNeighbourHint(props: {
  readonly neighbour: LayeredNeighbour;
  readonly moving: boolean;
  readonly insets: Required<LayeredInsets>;
  readonly directions: ReadonlySet<LayeredDirection>;
  readonly label: LayeredLabels["neighbour"];
  readonly onGo: (neighbour: LayeredNeighbour) => void;
}): React.ReactElement {
  const { neighbour, moving } = props;
  const frame = useSharedBorderFrameRef<HTMLButtonElement>();
  return (
    <button
      ref={frame}
      type="button"
      data-shared-border-frame=""
      data-layered-neighbour={neighbour.direction}
      data-pane={neighbour.pane.id}
      data-level="dialog"
      aria-label={props.label(neighbour.pane, neighbour.direction)}
      aria-hidden={moving || undefined}
      tabIndex={moving ? -1 : undefined}
      onClick={() => props.onGo(neighbour)}
      className={cn(
        "ui-glass absolute z-[32] inline-flex items-center justify-center gap-[4px] border-0 text-xs font-medium text-foreground outline-none transition-opacity duration-150",
        HINT_PLACES[neighbour.direction],
        moving && "pointer-events-none opacity-0",
      )}
      style={hintPlace(neighbour.direction, props.insets, props.directions)}
    >
      <Icon icon={HINT_ICONS[neighbour.direction]} size="small" className="shrink-0" />
      <span className={HINT_TEXT[neighbour.direction]}>{neighbour.pane.shortLabel ?? neighbour.pane.label}</span>
    </button>
  );
});

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
  const directions = React.useMemo(() => new Set(props.neighbours.map(({ direction }) => direction)), [props.neighbours]);
  return (
    <>
      {props.neighbours.map((neighbour) => (
        <LayeredNeighbourHint key={neighbour.direction} neighbour={neighbour} moving={props.moving} insets={props.insets} directions={directions} label={props.label} onGo={props.onGo} />
      ))}
    </>
  );
});
