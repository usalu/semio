/** 🔶️ The panes phase of LayeredOverview. */
import * as React from "react";
import { cellOffset, followStep, glideOffset, resolveLifecycle, restingCell, settleOffset, stripGrid, stripTransform, veilClip, windowAround, type LayeredGrid, type LayeredLanding, type LayeredRevealSource, type LayeredOffset, type LayeredWindow } from "../../../🔨️modules/🥞️layered-overview-geometry/🟦️.ts";
import type { LayeredOverviewProps } from "../📋️types/🟦️.ts";
import { FOLLOW, type LayeredDrive, applyVeil, hashPaneId, requireCell, sameWindow, useReducedMotion, useStableByKey } from "../🧰️pane-helpers/🟦️.ts";
import { usePaneLifecycle } from "../🧵️pane/🟦️.tsx";
import { type LayeredGhost } from "../👆️swipe/🟦️.ts";

/** 🔷 Builds panes, cells, renderCard, labels of LayeredOverview. */
export function usePanesState({ props }: { readonly props: LayeredOverviewProps }) {
  const { panes, cells, renderCard, labels, mode = "strip", pan = "pointer", routing = "hash", reducedMotion = "never", interactive = true } = props;
  const radius = props.windowing?.radius ?? 1;
  const lifecycle = resolveLifecycle(props.lifecycle);
  const freshIds = panes.map((pane) => pane.id);
  const ids = useStableByKey(freshIds, freshIds.join("\n"));
  const freshCells = ids.map((id) => requireCell(cells, id));
  const cellList = useStableByKey(freshCells, freshCells.map((cell) => `${cell.column}:${cell.row}`).join(" "));
  const grid = React.useMemo<LayeredGrid>(() => stripGrid(cellList), [cellList]);
  const reduced = useReducedMotion(reducedMotion);
  const panning = interactive && mode === "strip" && pan === "pointer" && !reduced;
  const controlled = props.openedId !== undefined;
  const [deepLink] = React.useState<string | null>(() => (routing === "hash" ? hashPaneId(ids) : null));
  const [ownOpened, setOwnOpened] = React.useState<string | null>(controlled ? null : deepLink);
  const wanted = controlled ? (props.openedId ?? null) : ownOpened;
  const opened = wanted !== null && ids.includes(wanted) ? wanted : null;
  const [ownRevealed, setRevealedId] = React.useState<string | null>(null);
  const wantedReveal = props.revealedId === undefined ? ownRevealed : props.revealedId;
  const revealedId = opened === null && wantedReveal !== null && ids.includes(wantedReveal) ? wantedReveal : null;
  const [revealSource, setRevealSource] = React.useState<LayeredRevealSource | null>(null);
  const { booted, live, posters, touch, warm, markDirty, registerContainer, container } = usePaneLifecycle(panes, ids, opened, revealedId, lifecycle, mode === "strip");

  const swiping = mode === "swipe";
  const initial = opened ? cellOffset(requireCell(cells, opened)) : cellList[0] ? cellOffset(cellList[0]) : { x: 0, y: 0 };
  const paneAt = React.useMemo(() => new Map(cellList.map((cell, index) => [`${cell.column}:${cell.row}`, ids[index]!])), [cellList, ids]);
  const rootRef = React.useRef<HTMLDivElement | null>(null);
  const stripRef = React.useRef<HTMLDivElement | null>(null);
  const veilRef = React.useRef<HTMLDivElement | null>(null);
  const cardStripRef = React.useRef<HTMLDivElement | null>(null);
  const cardElements = React.useRef(new Map<string, HTMLElement>());
  const offsetRef = React.useRef<LayeredOffset>(initial);
  const targetRef = React.useRef<LayeredOffset>(initial);
  const driveRef = React.useRef<LayeredDrive>(FOLLOW);
  const frameRef = React.useRef(0);
  const epochRef = React.useRef(0);
  const runningRef = React.useRef(false);
  const revealedRef = React.useRef<string | null>(null);
  const sourceRef = React.useRef<LayeredRevealSource | null>(null);
  const openedRef = React.useRef(opened);
  const previousOpened = React.useRef(opened);
  const [windowBox, setWindowBox] = React.useState<LayeredWindow>(() => windowAround(initial, radius));
  const windowRef = React.useRef(windowBox);
  const [resting, setResting] = React.useState<string | null>(() => paneAt.get(`${initial.x}:${initial.y}`) ?? null);
  const restingRef = React.useRef(resting);
  const logicalRestingRef = React.useRef(resting);
  const layoutRef = React.useRef({ mode, cellList, ids });
  const latest = React.useRef({ props, cells, cellList, grid, reduced, radius, mode, controlled, ids, paneAt });
  latest.current = { props, cells, cellList, grid, reduced, radius, mode, controlled, ids, paneAt };

  React.useEffect(() => {
    if (mode === "swipe" && interactive && opened === null && resting !== null && ids.includes(resting) && resting === logicalRestingRef.current) props.onSwipeIdChange?.(resting);
  }, [mode, interactive, opened, resting, ids, props.onSwipeIdChange]);

  const paint = React.useCallback((offset: LayeredOffset) => {
    offsetRef.current = offset;
    const { grid: strip, cells: placed, radius: reach, paneAt: at } = latest.current;
    const transform = stripTransform(offset, strip);
    if (stripRef.current) stripRef.current.style.transform = transform;
    if (cardStripRef.current) cardStripRef.current.style.transform = transform;
    if (veilRef.current) applyVeil(veilRef.current, veilClip(revealedRef.current === null ? null : (placed[revealedRef.current] ?? null), offset));
    const next = windowAround(offset, reach);
    if (!sameWindow(next, windowRef.current)) {
      windowRef.current = next;
      setWindowBox(next);
    }
    const cell = restingCell(offset);
    const still = cell === null ? null : (at.get(`${cell.column}:${cell.row}`) ?? null);
    if (layoutRef.current.mode !== latest.current.mode || layoutRef.current.cellList !== latest.current.cellList || layoutRef.current.ids !== latest.current.ids) return;
    if (still !== null) logicalRestingRef.current = still;
    if (still !== restingRef.current) {
      restingRef.current = still;
      setResting(still);
    }
  }, []);

  const repaint = React.useCallback(() => paint(offsetRef.current), [paint]);

  const stop = React.useCallback(() => {
    epochRef.current += 1;
    cancelAnimationFrame(frameRef.current);
    runningRef.current = false;
  }, []);

  const ghostRef = React.useRef<LayeredGhost | null>(null);
  const cellElements = React.useRef(new Map<string, HTMLElement>());
  const cellRefs = React.useMemo(
    () =>
      new Map(
        ids.map((id) => [
          id,
          (element: HTMLDivElement | null) => {
            if (element) cellElements.current.set(id, element);
            else cellElements.current.delete(id);
          },
        ]),
      ),
    [ids],
  );

  const aim = React.useCallback(
    (landing: LayeredLanding | null) => {
      const ghost = landing === null || (landing.slot.x === landing.cell.column && landing.slot.y === landing.cell.row) ? null : { id: latest.current.paneAt.get(`${landing.cell.column}:${landing.cell.row}`)!, ...landing };
      const previous = ghostRef.current;
      if (previous !== null && previous.id !== ghost?.id) for (const element of [container(previous.id), cellElements.current.get(previous.id)]) if (element) element.style.transform = "";
      ghostRef.current = ghost;
      if (ghost === null) return;
      const transform = `translate(${(ghost.slot.x - ghost.cell.column) * 100}%, ${(ghost.slot.y - ghost.cell.row) * 100}%)`;
      for (const element of [container(ghost.id), cellElements.current.get(ghost.id)]) if (element) element.style.transform = transform;
    },
    [container],
  );

  const settleGhost = React.useCallback(() => {
    const ghost = ghostRef.current;
    if (ghost === null) return;
    const at = offsetRef.current;
    aim(null);
    if (at.x !== ghost.slot.x || at.y !== ghost.slot.y) return;
    targetRef.current = cellOffset(ghost.cell);
    paint(targetRef.current);
  }, [aim, paint]);

  const finishGhost = React.useCallback(() => {
    if (ghostRef.current === null) return;
    if (runningRef.current) {
      stop();
      driveRef.current = FOLLOW;
      paint(targetRef.current);
    }
    settleGhost();
  }, [paint, settleGhost, stop]);

  const run = React.useCallback(() => {
    stop();
    runningRef.current = true;
    const epoch = epochRef.current;
    let painted = performance.now();
    const advance = (drive: LayeredDrive, now: number): boolean => {
      const elapsed = now - painted;
      painted = now;
      if (drive.kind !== "follow") {
        const step = (drive.kind === "glide" ? glideOffset : settleOffset)(drive.from, drive.to, now - drive.startedAt);
        paint(step.offset);
        return step.done;
      }
      const step = followStep(offsetRef.current, targetRef.current, elapsed);
      paint(step.offset);
      return step.settled;
    };
    const tick = () => {
      if (epoch !== epochRef.current) return;
      if (advance(driveRef.current, performance.now())) {
        driveRef.current = FOLLOW;
        runningRef.current = false;
        settleGhost();
        return;
      }
      frameRef.current = requestAnimationFrame(tick);
    };
    frameRef.current = requestAnimationFrame(tick);
  }, [paint, settleGhost, stop]);
  return { layoutRef, logicalRestingRef, targetRef, offsetRef, latest, stop, driveRef, paint, settleGhost, run, runningRef, aim, finishGhost, rootRef, interactive, swiping, opened, stripRef, repaint, veilRef, cardStripRef, cardElements, openedRef, revealedRef, cellList, sourceRef, setRevealSource, setRevealedId, touch, revealedId, setOwnOpened, deepLink, previousOpened, routing, container, mode, pan, reduced, ids, cells, panning, resting, live, lifecycle, warm, panes, paneAt, grid, windowBox, booted, posters, labels, registerContainer, markDirty, revealSource, renderCard, cellRefs };
}
