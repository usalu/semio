/** 🟠️ The pane styles phase of LayeredOverview. */
import * as React from "react";
import { cellOffset, clampOffset, inWindow, layeredZoomArm, layeredZoomMove, neighbourWarmBoot, pointerOffset, swipeNeighbours, type LayeredZoomHold } from "../../../🔨️modules/🥞️layered-overview-geometry/🟦️.ts";
import type { LayeredOverviewProps, LayeredPane } from "../📋️types/🟦️.ts";
import { FOCUSABLE, FOLLOW, requireCell, syncHash } from "../🧰️pane-helpers/🟦️.ts";
import { LayeredCardHost, LayeredPaneView } from "../🧵️pane/🟦️.tsx";
import { type LayeredNeighbour } from "../💡️hints/🟦️.tsx";
import type { usePanesState } from "../🔶️panes/🟦️.tsx";
import type { useMoveState } from "../🔸️move/🟦️.tsx";

/** 🖱️ The last mouse point, so a level that becomes active can tell a hover it inherited from the click. */
let lastMouse: { readonly x: number; readonly y: number } | undefined;

if (typeof window !== "undefined") {
  window.addEventListener("pointermove", (event) => {
    if (event.pointerType !== "mouse") return;
    lastMouse = { x: event.clientX, y: event.clientY };
  }, { passive: true });
}

/** 🟡 Builds paneStyles, insetTop, insetBottom, insets of LayeredOverview. */
export function usePaneStylesState({ previousOpened, opened, openedRef, interactive, routing, deepLink, latest, revealedRef, setRevealedId, sourceRef, setRevealSource, finishGhost, focusView, container, rootRef, cardElements, props, revealedId, touch, repaint, mode, pan, reduced, stop, targetRef, paint, follow, cellList, ids, aim, driveRef, cells, requestOpen, panning, swiping, resting, live, lifecycle, warm, panes, paneAt, moveTo, grid, windowBox, booted, posters, labels, registerContainer, markDirty, revealSource, reveal, conceal, cardCallback, renderCard }: Pick<ReturnType<typeof usePanesState>, "previousOpened" | "opened" | "openedRef" | "interactive" | "routing" | "deepLink" | "latest" | "revealedRef" | "setRevealedId" | "sourceRef" | "setRevealSource" | "finishGhost" | "container" | "rootRef" | "cardElements" | "revealedId" | "touch" | "repaint" | "mode" | "pan" | "reduced" | "stop" | "targetRef" | "paint" | "cellList" | "ids" | "aim" | "driveRef" | "cells" | "panning" | "swiping" | "resting" | "live" | "lifecycle" | "warm" | "panes" | "paneAt" | "grid" | "windowBox" | "booted" | "posters" | "labels" | "registerContainer" | "markDirty" | "revealSource" | "renderCard"> & Pick<ReturnType<typeof useMoveState>, "focusView" | "follow" | "requestOpen" | "moveTo" | "reveal" | "conceal" | "cardCallback"> & { readonly props: LayeredOverviewProps }) {
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
  const zoomHold = React.useRef<LayeredZoomHold | null>(null);
  const wasInteractive = React.useRef<boolean | null>(null);
  React.useLayoutEffect(() => {
    const before = wasInteractive.current;
    wasInteractive.current = interactive;
    if (!interactive) {
      zoomHold.current = null;
      return;
    }
    if (before === false && lastMouse !== undefined) zoomHold.current = layeredZoomArm(lastMouse.x, lastMouse.y);
  }, [interactive]);
  React.useEffect(() => {
    if (!interactive) return;
    const onMove = (event: PointerEvent) => {
      if (event.pointerType !== "mouse") return;
      const hold = zoomHold.current;
      if (hold === null) return;
      const hit = event.target instanceof Element ? event.target : event.target instanceof Text ? event.target.parentElement : null;
      const trigger = hit?.closest("[data-layered-zoom]") ?? null;
      const over = trigger !== null && (rootRef.current?.contains(trigger) ?? false);
      const step = layeredZoomMove(hold, event.clientX, event.clientY, over);
      zoomHold.current = step.hold;
      if (!step.reveal || trigger === null) return;
      const id = trigger.closest("[data-layered-card]")?.getAttribute("data-layered-card");
      if (id !== null && id !== undefined) reveal(id, "pointer");
    };
    window.addEventListener("pointermove", onMove);
    return () => window.removeEventListener("pointermove", onMove);
  }, [interactive, reveal]);
  const cardHost = (pane: LayeredPane) => (
    <LayeredCardHost key={pane.id} pane={pane} mode={mode} revealed={revealedId === pane.id} source={revealedId === pane.id ? revealSource : null} zoomHold={zoomHold} onOpen={requestOpen} onReveal={reveal} onConceal={conceal} onElement={cardCallback} renderCard={renderCard} />
  );
  const chrome = props.renderChrome?.({ revealed: revealedId !== null, opened: opened !== null });
  return { restedOn, stripStyle, paneView, cellStyles, cardHost, neighbours, insets, goTo, chrome };
}
