/** 🔸️ The move phase of LayeredOverview. */
import * as React from "react";
import { cellOffset, clampOffset, layeredCardConcealed, layeredCardTinted, swipeWrapTarget, type LayeredRevealSource, type LayeredOffset } from "../../../🔨️modules/🥞️layered-overview-geometry/🟦️.ts";
import type { LayeredOverviewProps } from "../📋️types/🟦️.ts";
import { FOLLOW, requireCell } from "../🧰️pane-helpers/🟦️.ts";
import { type LayeredSwipeDrive, useSwipe } from "../👆️swipe/🟦️.ts";
import type { usePanesState } from "../🔶️panes/🟦️.tsx";

/** 🔹 Builds moveTo, glideTo, follow, focusView of LayeredOverview. */
export function useMoveState({ layoutRef, logicalRestingRef, targetRef, offsetRef, latest, stop, driveRef, paint, settleGhost, run, runningRef, aim, finishGhost, rootRef, interactive, swiping, opened, stripRef, repaint, veilRef, cardStripRef, cardElements, openedRef, revealedRef, pressedRef, setPressed, pressed, cellList, sourceRef, setRevealSource, setRevealedId, touch, props, revealedId, setOwnOpened, deepLink }: Pick<ReturnType<typeof usePanesState>, "layoutRef" | "logicalRestingRef" | "targetRef" | "offsetRef" | "latest" | "stop" | "driveRef" | "paint" | "settleGhost" | "run" | "runningRef" | "aim" | "finishGhost" | "rootRef" | "interactive" | "swiping" | "opened" | "stripRef" | "repaint" | "veilRef" | "cardStripRef" | "cardElements" | "openedRef" | "revealedRef" | "pressedRef" | "setPressed" | "pressed" | "cellList" | "sourceRef" | "setRevealSource" | "setRevealedId" | "touch" | "revealedId" | "setOwnOpened" | "deepLink"> & { readonly props: LayeredOverviewProps }) {
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
  const mode = latest.current.mode, ids = latest.current.ids;
  React.useLayoutEffect(() => {
    if (firstCells.current) {
      firstCells.current = false;
      return;
    }
    const current = latest.current, active = document.activeElement;
    const host = current.props.interactive !== false && active instanceof HTMLElement && active.closest("[inert],[hidden],[aria-hidden=true]") === null && active.closest("[data-layered-overview]") === rootRef.current ? active.closest<HTMLElement>("[data-layered-card]") : null;
    const focused = host !== null && cardElements.current.get(host.dataset.layeredCard ?? "") === host ? host.dataset.layeredCard : null;
    const anchor = [opened, current.props.revealedId, mode === "swipe" ? current.props.swipeId : null, focused, revealedRef.current, logicalRestingRef.current].find((id): id is string => typeof id === "string" && current.ids.includes(id));
    const next = anchor === undefined ? clampOffset(offsetRef.current, cellList) : cellOffset(requireCell(current.cells, anchor));
    stop();
    aim(null);
    driveRef.current = FOLLOW;
    layoutRef.current = { mode, cellList, ids };
    targetRef.current = next;
    paint(next);
  }, [aim, cellList, ids, mode, paint, stop]);

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
      pressedRef.current = false;
      setPressed(false);
      setRevealedId(id);
      touch(id);
      finishGhost();
      focusView(id);
      repaint();
    },
    [finishGhost, focusView, repaint, touch, pressedRef, setPressed],
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
      pressedRef.current = false;
      setPressed(false);
      setRevealedId(null);
      setRevealSource(null);
      latest.current.props.onRevealedIdChange?.(null);
      repaint();
    },
    [repaint, pressedRef, setPressed],
  );

  const press = React.useCallback(
    (id: string, source: LayeredRevealSource) => {
      if (latest.current.props.interactive === false || openedRef.current !== null) return;
      if (latest.current.props.revealedId === undefined && revealedRef.current !== id) reveal(id, source);
      if (latest.current.props.pressed !== undefined) {
        latest.current.props.onPressedChange?.(true);
        return;
      }
      if (pressedRef.current) return;
      pressedRef.current = true;
      setPressed(true);
      repaint();
    },
    [reveal, repaint, pressedRef, setPressed],
  );

  const release = React.useCallback(() => {
    if (latest.current.props.pressed !== undefined) {
      if (latest.current.props.interactive !== false) latest.current.props.onPressedChange?.(false);
      return;
    }
    if (!pressedRef.current) return;
    pressedRef.current = false;
    setPressed(false);
    repaint();
  }, [repaint, pressedRef, setPressed]);

  React.useLayoutEffect(() => {
    pressedRef.current = pressed;
    repaint();
  }, [pressed, pressedRef, repaint]);

  React.useLayoutEffect(() => {
    if (!interactive && props.revealedId === undefined && revealedRef.current !== null) conceal(revealedRef.current);
  }, [interactive, props.revealedId, conceal]);

  React.useLayoutEffect(() => {
    for (const [id, element] of cardElements.current) {
      const source = props.revealedId === undefined ? sourceRef.current : props.revealedBy ?? "pointer";
      const hidden = layeredCardConcealed(id, revealedId, source, pressed);
      const tinted = layeredCardTinted(id, revealedId, source, pressed);
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
  return { focusView, follow, requestOpen, moveTo, reveal, conceal, press, release, cardCallback, close, stripCallback, veilCallback, cardStripCallback };
}
