// #region 🧲️Header
// 💻️ framework/ui/elements/🥞️LayeredOverview/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { cellOffset, clampOffset, followStep, glideOffset, inWindow,
  layeredCardConcealed,
  layeredCardTinted, neighbourWarmBoot, pointerOffset, resolveLifecycle, restingCell, settleOffset, stripGrid, stripTransform, swipeNeighbours, swipeWrapTarget, veilClip, windowAround, type LayeredDirection, type LayeredGrid, type LayeredLanding, type LayeredRevealSource,
  type LayeredOffset, type LayeredWindow } from "../../🔨️modules/🥞️layered-overview-geometry/🟦️.ts";
import { CatalogIcon as Icon } from "../🔣️Icons/🟦️.tsx";
import type { LayeredOverviewProps, LayeredPane } from "./📋️types/🟦️.ts";
import { FOCUSABLE, FOLLOW, type LayeredDrive, applyVeil, hashPaneId, requireCell, sameWindow, syncHash, useReducedMotion, useStableByKey } from "./🧰️pane-helpers/🟦️.ts";
import { LayeredCardHost, LayeredPaneView, usePaneLifecycle } from "./🧵️pane/🟦️.tsx";
import { type LayeredNeighbour, LayeredNeighbourHints } from "./💡️hints/🟦️.tsx";
import { type LayeredGhost, type LayeredSwipeDrive, useSwipe } from "./👆️swipe/🟦️.ts";

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
  const latest = React.useRef({ props, cells, cellList, grid, reduced, radius, mode, controlled, ids, paneAt });
  latest.current = { props, cells, cellList, grid, reduced, radius, mode, controlled, ids, paneAt };

  React.useEffect(() => {
    if (mode === "swipe" && interactive && opened === null && resting !== null) props.onSwipeIdChange?.(resting);
  }, [mode, interactive, opened, resting, props.onSwipeIdChange]);

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

  const moveTo = React.useCallback(
    (to: LayeredOffset, kind: "glide" | "settle") => {
      targetRef.current = to;
      const from = offsetRef.current;
      if (latest.current.reduced || (from.x === to.x && from.y === to.y)) {
        stop();
        driveRef.current = FOLLOW;
        paint(to);
        settleGhost();
        return;
      }
      driveRef.current = { kind, from, to, startedAt: performance.now() };
      run();
    },
    [paint, run, settleGhost, stop],
  );
  const glideTo = React.useCallback((to: LayeredOffset) => moveTo(to, "glide"), [moveTo]);

  const follow = React.useCallback(
    (to: LayeredOffset) => {
      targetRef.current = to;
      if (driveRef.current.kind === "follow" && runningRef.current) return;
      driveRef.current = FOLLOW;
      run();
    },
    [run],
  );

  const focusView = React.useCallback((id: string) => glideTo(cellOffset(requireCell(latest.current.cells, id))), [glideTo]);

  const swipeDrive = React.useRef<LayeredSwipeDrive>({
    offset: () => offsetRef.current,
    cells: () => latest.current.cellList,
    drag: (offset) => {
      stop();
      driveRef.current = FOLLOW;
      targetRef.current = offset;
      paint(offset);
    },
    aim: (from, axis, step) => {
      const landing = swipeWrapTarget(from, axis, step, latest.current.cellList);
      aim(landing);
      return landing;
    },
    settle: (to) => moveTo(to, "settle"),
    finish: finishGhost,
    stop,
  });
  useSwipe(rootRef, interactive && swiping && opened === null, swipeDrive);

  React.useEffect(() => stop, [stop]);

  const stripCallback = React.useCallback(
    (element: HTMLDivElement | null) => {
      stripRef.current = element;
      if (element) repaint();
    },
    [repaint],
  );
  const veilCallback = React.useCallback(
    (element: HTMLDivElement | null) => {
      veilRef.current = element;
      if (element) repaint();
    },
    [repaint],
  );
  const cardStripCallback = React.useCallback(
    (element: HTMLDivElement | null) => {
      cardStripRef.current = element;
      if (element) repaint();
    },
    [repaint],
  );
  const cardCallback = React.useCallback((id: string, element: HTMLElement | null) => {
    if (element) cardElements.current.set(id, element);
    else cardElements.current.delete(id);
  }, []);

  const firstCells = React.useRef(true);
  React.useLayoutEffect(() => {
    if (firstCells.current) {
      firstCells.current = false;
      return;
    }
    const focus = openedRef.current ?? revealedRef.current;
    stop();
    aim(null);
    driveRef.current = FOLLOW;
    const next = focus === null ? clampOffset(offsetRef.current, cellList) : cellOffset(requireCell(latest.current.cells, focus));
    targetRef.current = next;
    paint(next);
  }, [aim, cellList, paint, stop]);

  const reveal = React.useCallback(
    (id: string, source: LayeredRevealSource) => {
      if (latest.current.props.interactive === false || openedRef.current !== null) return;
      if (latest.current.props.revealedId !== undefined) {
        latest.current.props.onRevealedIdChange?.(id, source);
        return;
      }
      if (revealedRef.current === id && (sourceRef.current === source || source === "focus")) return;
      const same = revealedRef.current === id;
      revealedRef.current = id;
      sourceRef.current = source;
      setRevealSource(source);
      latest.current.props.onRevealedIdChange?.(id, source);
      if (same) return;
      setRevealedId(id);
      touch(id);
      finishGhost();
      focusView(id);
      repaint();
    },
    [finishGhost, focusView, repaint, touch],
  );

  const conceal = React.useCallback(
    (id: string) => {
      if (latest.current.props.revealedId !== undefined) {
        if (latest.current.props.interactive !== false) latest.current.props.onRevealedIdChange?.(null);
        return;
      }
      if (revealedRef.current !== id) return;
      revealedRef.current = null;
      sourceRef.current = null;
      setRevealedId(null);
      setRevealSource(null);
      latest.current.props.onRevealedIdChange?.(null);
      repaint();
    },
    [repaint],
  );

  React.useLayoutEffect(() => {
    if (!interactive && props.revealedId === undefined && revealedRef.current !== null) conceal(revealedRef.current);
  }, [interactive, props.revealedId, conceal]);

  React.useLayoutEffect(() => {
    for (const [id, element] of cardElements.current) {
      const source = props.revealedId === undefined ? sourceRef.current : props.revealedBy ?? "pointer";
      const hidden = layeredCardConcealed(id, revealedId, source);
      const tinted = layeredCardTinted(id, revealedId, source);
      if (hidden) element.dataset.concealed = "";
      else delete element.dataset.concealed;
      if (tinted) element.dataset.tinted = "";
      else delete element.dataset.tinted;
      for (const child of element.children) if (child instanceof HTMLElement) child.style.opacity = hidden ? "0" : "";
    }
  });

  const requestOpen = React.useCallback((id: string | null) => {
    if (latest.current.props.interactive === false || id === openedRef.current) return;
    if (!latest.current.controlled) setOwnOpened(id);
    latest.current.props.onOpenedIdChange?.(id);
  }, []);
  const close = React.useCallback(() => requestOpen(null), [requestOpen]);

  React.useEffect(() => {
    if (interactive && deepLink !== null && (!latest.current.controlled || deepLink !== openedRef.current)) latest.current.props.onOpenedIdChange?.(deepLink);
  }, [deepLink, interactive]);

  React.useLayoutEffect(() => {
    const previous = previousOpened.current;
    previousOpened.current = opened;
    openedRef.current = opened;
    if (interactive && routing === "hash" && (previous !== opened || deepLink === null)) syncHash(opened, latest.current.ids);
    if (previous === opened) return;
    if (revealedRef.current !== null) {
      revealedRef.current = null;
      setRevealedId(null);
      if (latest.current.props.revealedId === undefined) latest.current.props.onRevealedIdChange?.(null);
    }
    if (sourceRef.current !== null) {
      sourceRef.current = null;
      setRevealSource(null);
    }
    if (opened !== null) {
      finishGhost();
      focusView(opened);
      const region = container(opened);
      if (interactive && region && !region.contains(document.activeElement)) region.focus({ preventScroll: true });
      return;
    }
    if (!interactive || previous === null) return;
    const active = document.activeElement;
    if (active && active !== document.body && rootRef.current && !rootRef.current.contains(active)) return;
    cardElements.current.get(previous)?.querySelector<HTMLElement>(FOCUSABLE)?.focus({ preventScroll: true });
  }, [opened, interactive, routing, deepLink, container, finishGhost, focusView]);

  React.useLayoutEffect(() => {
    if (props.revealedId === undefined) return;
    const source = revealedId === null ? null : props.revealedBy ?? "pointer";
    revealedRef.current = revealedId;
    sourceRef.current = source;
    setRevealSource(source);
    if (revealedId !== null) {
      touch(revealedId);
      finishGhost();
      focusView(revealedId);
    }
    repaint();
  }, [revealedId, props.revealedId, props.revealedBy, opened, finishGhost, focusView, repaint, touch]);

  React.useLayoutEffect(() => {
    if (props.camera === undefined || mode !== "strip" || pan !== "pointer" || opened !== null || revealedId !== null) return;
    const { grid: strip, cellList: placed } = latest.current;
    const target = clampOffset(pointerOffset(props.camera.x, props.camera.y, strip), placed);
    if (reduced) {
      stop();
      targetRef.current = target;
      paint(target);
    } else follow(target);
  }, [props.camera?.x, props.camera?.y, mode, pan, opened, revealedId, reduced, cellList, follow, paint, stop]);

  React.useLayoutEffect(() => {
    if (props.swipeId === undefined || mode !== "swipe" || opened !== null || revealedId !== null || !ids.includes(props.swipeId)) return;
    stop();
    aim(null);
    driveRef.current = FOLLOW;
    targetRef.current = cellOffset(requireCell(cells, props.swipeId));
    paint(targetRef.current);
  }, [props.swipeId, mode, opened, revealedId, cells, ids, aim, paint, stop]);

  React.useEffect(() => {
    if (!interactive || routing !== "hash") return;
    const onHash = () => {
      const raw = window.location.hash.replace(/^#/, "");
      if (latest.current.ids.includes(raw)) requestOpen(raw);
      else if (raw === "") requestOpen(null);
    };
    window.addEventListener("hashchange", onHash);
    return () => window.removeEventListener("hashchange", onHash);
  }, [interactive, routing, requestOpen]);

  React.useEffect(() => {
    if (!interactive || opened === null) return;
    const onKey = (event: KeyboardEvent) => {
      if (event.key === "Escape" && !event.defaultPrevented) {
        event.preventDefault();
        requestOpen(null);
      }
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [interactive, opened, requestOpen]);

  React.useEffect(() => {
    if (!panning) return;
    const onMove = (event: PointerEvent) => {
      if (event.pointerType !== "mouse" || openedRef.current !== null) return;
      const box = rootRef.current?.getBoundingClientRect();
      if (!box || box.width <= 0 || box.height <= 0) return;
      const unit = (value: number): number => Math.min(1, Math.max(0, value));
      const camera = { x: unit((event.clientX - box.left) / box.width), y: unit((event.clientY - box.top) / box.height) };
      latest.current.props.onCameraChange?.(camera);
      if (revealedRef.current !== null || latest.current.props.camera !== undefined) return;
      const { grid: strip, cellList: placed } = latest.current;
      follow(clampOffset(pointerOffset(camera.x, camera.y, strip), placed));
    };
    window.addEventListener("pointermove", onMove, { passive: true });
    return () => window.removeEventListener("pointermove", onMove);
  }, [panning, follow]);

  React.useEffect(() => {
    if (swiping && opened === null && resting !== null) touch(resting);
  }, [swiping, opened, resting, touch]);

  React.useEffect(() => {
    if (!swiping || resting === null) return;
    const step = neighbourWarmBoot(ids, cellList, requireCell(latest.current.cells, resting), live, opened, lifecycle.budget);
    if (step.kind === "boot") warm(step.id);
  }, [swiping, resting, ids, cellList, live, opened, lifecycle.budget, warm]);

  const paneStyles = React.useMemo(() => new Map(ids.map((id, index) => [id, { gridColumn: cellList[index]!.column + 1, gridRow: cellList[index]!.row + 1 }])), [ids, cellList]);
  const insetTop = props.insets?.top ?? "0px";
  const insetBottom = props.insets?.bottom ?? "0px";
  const insets = React.useMemo(() => ({ top: insetTop, bottom: insetBottom }), [insetTop, insetBottom]);
  const cellStyles = React.useMemo(() => new Map([...paneStyles].map(([id, style]) => [id, { ...style, touchAction: "pinch-zoom", paddingTop: `calc(${insets.top} + 44px)`, paddingBottom: `calc(${insets.bottom} + 44px)` }])), [paneStyles, insets]);
  const restedOn = React.useRef(resting);
  if (resting !== null) restedOn.current = resting;
  const hintFrom = restedOn.current;
  const neighbours = React.useMemo<readonly LayeredNeighbour[]>(
    () => (hintFrom === null ? [] : swipeNeighbours(requireCell(cells, hintFrom), cellList).map(({ direction, cell, slot }) => ({ direction, cell, slot, pane: panes[ids.indexOf(paneAt.get(`${cell.column}:${cell.row}`)!)]! }))),
    [hintFrom, cells, cellList, panes, ids, paneAt],
  );
  const goTo = React.useCallback(
    (neighbour: LayeredNeighbour) => {
      if (latest.current.props.interactive === false) return;
      finishGhost();
      aim(neighbour);
      moveTo(neighbour.slot, "settle");
    },
    [aim, finishGhost, moveTo],
  );
  const nearby = React.useMemo(() => new Set(neighbours.map(({ pane }) => pane.id)), [neighbours]);
  const stripStyle: React.CSSProperties = { width: `${grid.columns * 100}%`, height: `${grid.rows * 100}%`, gridTemplateColumns: `repeat(${grid.columns}, minmax(0, 1fr))`, gridTemplateRows: `repeat(${grid.rows}, minmax(0, 1fr))`, contain: swiping ? "layout" : "layout paint" };
  const paneView = (pane: LayeredPane, index: number) => (
    <LayeredPaneView
      key={pane.id}
      pane={pane}
      style={paneStyles.get(pane.id)}
      opened={opened === pane.id}
      revealed={revealedId === pane.id}
      live={live.has(pane.id)}
      shown={inWindow(cellList[index]!, windowBox) || (swiping && nearby.has(pane.id))}
      busy={booted.has(pane.id)}
      poster={posters.get(pane.id) ?? pane.poster ?? null}
      waiting={labels.waiting(pane)}
      failed={labels.failed(pane)}
      onContainer={registerContainer}
      onDirty={markDirty}
    />
  );
  const cardHost = (pane: LayeredPane) => (
    <LayeredCardHost key={pane.id} pane={pane} mode={mode} revealed={revealedId === pane.id} source={revealedId === pane.id ? revealSource : null} onOpen={requestOpen} onReveal={reveal} onConceal={conceal} onElement={cardCallback} renderCard={renderCard} />
  );
  const chrome = props.renderChrome?.({ revealed: revealedId !== null, opened: opened !== null });
  const overviewButton =
    !interactive || opened === null || labels.overview === undefined ? null : (
      <button
        type="button"
        data-layered-overview-button=""
        data-level="dialog"
        aria-keyshortcuts="Escape"
        onClick={close}
        className="ui-glass absolute right-double top-double z-40 inline-flex items-center gap-single border border-border-normal px-single py-half text-sm font-medium text-foreground outline-none transition-colors hover:border-border-emphasized focus-visible:ring-2 focus-visible:ring-ring motion-reduce:transition-none"
      >
        <Icon icon="layout-grid" size="small" />
        {labels.overview}
      </button>
    );

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