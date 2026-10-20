import * as React from "react";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { type LayeredDirection } from "../../🔨️modules/🥞️layered-overview-geometry/🟦️.ts";
import type { LayeredOverviewProps } from "./📋️types/🟦️.ts";
import { LayeredNeighbourHints } from "./💡️hints/🟦️.tsx";
import { usePanesState } from "./🔶️panes/🟦️.tsx";
import { useMoveState } from "./🔸️move/🟦️.tsx";
import { usePaneStylesState } from "./🟠️pane-styles/🟦️.tsx";
import { buildOverviewButton } from "./🟢️overview-button/🟦️.tsx";

export type { LayeredDirection };
// #endregion 👆️LayeredSwipe
/** 🥞️ A layered landing: every page on a strip of container-sized cells, ONE `ui-veil` glass above it whose `clip-path` punches a hole over the
 * revealed page, and the cards above the glass — in `strip` mode the app's card grid, fixed; in `swipe` mode a card layer of the strip's own
 * grid that moves with the strip, each card centred on its page and scrolling inside its cell when taller. It fills its positioned parent.
 * Hovering (mouse) or keyboard-focusing a card glides the strip to that page (500 ms, cubic) and shows it clear; leaving or blurring the card
 * restores the glass and the pointer pan (12 % of the gap per 60 Hz frame, by elapsed time). A pointer hover also fades every other card out
 * (its slot and hit target kept, so the pointer can move onto another card) and tints the hovered card with the veil, so the page behind it is the only crisp surface, until the pointer leaves; keyboard focus leaves every card fully visible. In `swipe` mode a touch carries the strip along
 * the axis it takes first and settles it on the neighbouring cell (320 ms, cubic ease-out); past an edge the index wraps while the motion goes
 * on — the far page is shifted into the slot beside the view, and once the strip rests there the shift and the offset are undone in the same
 * frame — and the panes one swipe away are warmed. Opening a
 * card (or `#id`) shows its page full size as a focused region; Escape or the Overview button — the element's own, or the app's chrome closing the
 * controlled page — return focus to the card unless it lies in that chrome.
 * Transforms and veil are written imperatively in percent (no render per frame, no viewport reads, a rotated phone stays on its cell), and only
 * pages near the view keep placeholders. The pan, the glide and the settle are how the landing is read, so they run whatever the device says
 * about motion (`reducedMotion` `"never"`, the default): a Remote Desktop session reports reduced motion for everyone in it. An app that wants
 * the device's request honoured passes `"auto"`, one that wants stillness `"always"` — then nothing pans and a page is simply there.
 *
 * @see ../../🔨️modules/🥞️layered-overview-geometry/🟦️.ts — the pure geometry and lifecycle policy
 * @see ../🃏️OverviewCard/🟦️.tsx — the card the apps render
 * @see https://www.w3.org/WAI/ARIA/apg/practices/landmark-regions/ — the opened page is a labelled region */
export function LayeredOverview(props: LayeredOverviewProps): React.ReactElement {
  const { targetRef, offsetRef, latest, stop, driveRef, paint, settleGhost, run, runningRef, aim, finishGhost, rootRef, interactive, swiping, opened, stripRef, repaint, veilRef, cardStripRef, cardElements, openedRef, revealedRef, cellList, sourceRef, setRevealSource, setRevealedId, touch, revealedId, setOwnOpened, deepLink, previousOpened, routing, container, mode, pan, reduced, ids, cells, panning, resting, live, lifecycle, warm, panes, paneAt, grid, windowBox, booted, posters, labels, registerContainer, markDirty, revealSource, renderCard, cellRefs } = usePanesState({ props });
  const { focusView, follow, requestOpen, moveTo, reveal, conceal, cardCallback, close, stripCallback, veilCallback, cardStripCallback } = useMoveState({ targetRef, offsetRef, latest, stop, driveRef, paint, settleGhost, run, runningRef, aim, finishGhost, rootRef, interactive, swiping, opened, stripRef, repaint, veilRef, cardStripRef, cardElements, openedRef, revealedRef, cellList, sourceRef, setRevealSource, setRevealedId, touch, props, revealedId, setOwnOpened, deepLink });
  const { restedOn, stripStyle, paneView, cellStyles, cardHost, neighbours, insets, goTo, chrome } = usePaneStylesState({ previousOpened, opened, openedRef, interactive, routing, deepLink, latest, revealedRef, setRevealedId, sourceRef, setRevealSource, finishGhost, focusView, container, rootRef, cardElements, props, revealedId, touch, repaint, mode, pan, reduced, stop, targetRef, paint, follow, cellList, ids, aim, driveRef, cells, requestOpen, panning, swiping, resting, live, lifecycle, warm, panes, paneAt, moveTo, grid, windowBox, booted, posters, labels, registerContainer, markDirty, revealSource, reveal, conceal, cardCallback, renderCard });
  const { overviewButton } = buildOverviewButton({ interactive, opened, labels, close });
  return (
    <div
      ref={rootRef}
      data-layered-overview=""
      data-interactive={interactive ? "" : undefined}
      data-mode={mode}
      data-layered-resting={restedOn.current ?? undefined}
      data-pan={panning ? "pointer" : "none"}
      className="relative h-full w-full overflow-clip bg-background text-foreground"
      style={swiping && opened === null ? { touchAction: "pinch-zoom" } : undefined}
    >
      {overviewButton}
      <div ref={stripCallback} data-layered-strip="" className="grid will-change-transform" style={stripStyle}>
        {panes.map(paneView)}
      </div>
      {opened === null ? (
        <>
          <div ref={veilCallback} data-layered-veil="" data-level="dialog" className="ui-veil pointer-events-none absolute inset-0 z-30" />
          {swiping ? (
            <>
              <div ref={cardStripCallback} data-layered-overlay="" role="group" aria-label={labels.grid} tabIndex={interactive ? -1 : undefined} inert={!interactive} className="pointer-events-none absolute left-0 top-0 z-[31] grid outline-none will-change-transform" style={stripStyle}>
                {panes.map((pane) => (
                  <div key={pane.id} ref={cellRefs.get(pane.id)} data-layered-cell={pane.id} className="pointer-events-none flex min-h-0 min-w-0 items-center-safe justify-center overflow-y-auto px-[36px] [:where(&)_*]:touch-pinch-zoom" style={cellStyles.get(pane.id)}>
                    {cardHost(pane)}
                  </div>
                ))}
              </div>
              <LayeredNeighbourHints neighbours={neighbours} moving={resting === null} insets={insets} label={labels.neighbour} onGo={goTo} />
            </>
          ) : (
            <div data-layered-overlay="" role="group" aria-label={labels.grid} tabIndex={interactive ? -1 : undefined} inert={!interactive} className={cn("pointer-events-none absolute inset-0 z-[31] outline-none", props.overlayClassName)} style={props.overlayStyle}>
              {panes.map(cardHost)}
            </div>
          )}
        </>
      ) : null}
      {chrome}
    </div>
  );
}
