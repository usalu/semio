/** 💡️ Board2dHost suggestions and events: the suggestion menu and its rows, the hover paint and dispatch and the buffered board event drain, flush, snapshot and selection application with the peer gesture end and camera interaction settle. */
import { useCallback, useEffect, useMemo, useRef } from "react";
import { type Board2dScene } from "@semio-tech/framework";
import { createCoalescingActionDispatcher } from "../../🛠️ShellHelpers/🛑️extension-io/🟦️.ts";
import { suggestionPopupOwnsWindow, suggestionRowsWithFocus, useSuggestionSubmenuSearch } from "../../🎣️suggestion-submenu/🟦️.ts";
import { board2dGranularityById, board2dHoverActionArgs, board2dKindDomainById, board2dKindHoverFromElementId, board2dSuggestionMenuItems, board2dSuggestionMenuOwnsWindow, type BoardEventRow, latestBoard2dHoverId, parseBoard2dSuggestionMenu } from "../🔬️board-parsing/🟦️.ts";
import { coalesceBoard2dEvents, collectBoardLiveMirrorMutations } from "../📡️board-events/🟦️.ts";
import { applySnapshotToSession, applyToSession, boardPeerOwnsGesture, pushBoardLiveMirrorMutations } from "../🔄️session-sync/🟦️.ts";
import type { BoardCore1 } from "../🧰️board-session-state/🟦️.ts";
import { type Board2dWasmSession } from "@semio-tech/framework/canvas/react-renderer";

// #region Board2dHost
/** 💡️ Wires the suggestion menu, hover and the buffered board event flush. */
export function useBoardSuggestionsAndEvents(core: BoardCore1) {
  const { boardStatusRef, cameraInteractionActiveRef, cameraSettleTimeoutRef, containerRef, contextMenu, dispatch, dispatchSettled, dispatchSuggestion, mapSuggestionMenu, node, onPeerGestureEndedRef, peerScope, pendingCameraDispatchRef, pendingEventRowsRef, pendingSelectionJsonRef, pendingSnapshotSceneRef, publishBoardVitals, scene, sessionEpoch, sessionRef, setLocalSelectionJson, setSuggestionFocus, suggestionCheckingPlacementLabel, suggestionNoPlacementLabel, windowInstanceId } = core;
  //#region SuggestionMenu
  const suggestionMenu = useMemo(() => parseBoard2dSuggestionMenu(scene?.suggestionMenuJson), [scene?.suggestionMenuJson]);
  const suggestionMenuOwnsThisWindow = board2dSuggestionMenuOwnsWindow(suggestionMenu, windowInstanceId ?? undefined);
  const suggestionPopupOwnsThisWindow = suggestionPopupOwnsWindow(suggestionMenu, windowInstanceId ?? undefined);
  const suggestionMenuOwnsThisWindowRef = useRef(false);
  suggestionMenuOwnsThisWindowRef.current = suggestionMenuOwnsThisWindow;
  useSuggestionSubmenuSearch(contextMenu?.suggestTarget ?? null, { x: contextMenu?.x ?? 0, y: contextMenu?.y ?? 0 }, windowInstanceId ?? undefined, dispatch, () => setSuggestionFocus(null));
  useEffect(() => {
    if (!suggestionMenuOwnsThisWindow) setSuggestionFocus(null);
  }, [suggestionMenuOwnsThisWindow]);
  /** 📂️ The candidate rows one suggestion menu lists — the context menu's "suggest" submenu and the floating
   * popup share them. A submenu whose search has not echoed back yet lists the pending row. */
  const suggestionRows = useMemo(() => {
    const listed = suggestionMenuOwnsThisWindow && suggestionMenu ? suggestionMenu : { open: true, x: 0, y: 0, hoveredIndex: 0, submenu: true, pending: true, candidates: [] };
    const rows = mapSuggestionMenu(board2dSuggestionMenuItems(listed, { checkingPlacement: suggestionCheckingPlacementLabel, noPlacement: suggestionNoPlacementLabel }));
    return suggestionRowsWithFocus(rows, listed, setSuggestionFocus);
  }, [mapSuggestionMenu, suggestionCheckingPlacementLabel, suggestionMenu, suggestionMenuOwnsThisWindow, suggestionNoPlacementLabel]);
  const closeSuggestionMenu = useCallback(() => dispatchSuggestion("closeHandleSuggestions"), [dispatchSuggestion]);

  // 💡️ The provisional paint. The popup lists what the GUEST resolved, but the ghost on the canvas is
  // drawn by THIS pane's engine, so the open handle and the previewed index are mirrored into the local
  // slot — hovering a row moves the ghost this frame instead of a round trip later. The commit is never
  // mirrored: `acceptSuggestion` places through the guest so the placement is exactly one document edit.
  useEffect(() => {
    const session = sessionRef.current;
    if (!session) return;
    if (!suggestionPopupOwnsThisWindow || !suggestionMenu?.handleId) {
      applyToSession(session, (s) => s.brushCancelSlot?.());
      return;
    }
    applyToSession(session, (s) => {
      s.brushOpenSlot?.(suggestionMenu.handleId!);
      s.brushSetCandidateIndex?.(suggestionMenu.hoveredIndex);
    });
  }, [sessionEpoch, suggestionMenu?.handleId, suggestionMenu?.hoveredIndex, suggestionPopupOwnsThisWindow]);

  // 🪟️ A pane that does NOT render the popup still has to be able to dismiss it, otherwise an open menu
  // owned by a sibling gates this pane's ordinary context menu with no way out — the same hole
  // `World3dHost` closes with its own escape/outside-pointer path.
  useEffect(() => {
    if (!suggestionMenu?.open || suggestionMenuOwnsThisWindow) return undefined;
    const onKeyDown = (event: globalThis.KeyboardEvent): void => {
      if (event.key === "Escape") closeSuggestionMenu();
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [closeSuggestionMenu, suggestionMenu?.open, suggestionMenuOwnsThisWindow]);
  //#endregion SuggestionMenu

  //#region Hover
  const granularityById = useMemo(() => board2dGranularityById(scene?.snapshotJson ?? ""), [scene?.snapshotJson]);
  const granularityByIdRef = useRef(granularityById);
  granularityByIdRef.current = granularityById;
  const interactionDomainId = scene?.domainId;
  /** 🩺️ Imperative mirror of the painted hover — no React state, so a pointermove never re-renders the host. */
  const publishHoverPaint = useCallback((id: string | null) => {
    const container = containerRef.current;
    if (!container) return;
    if (id === null) container.removeAttribute("data-board-hover-paint-id");
    else container.setAttribute("data-board-hover-paint-id", id);
  }, []);

  /** 🖱️ Transitive KIND hover: pointing at a catalogue kind row anywhere in the chrome paints every
   * node/handle/edge of that kind on EVERY board pane, the 2d twin of puzzle 3d's `hoveredKindId`. The
   * engine already owns the whole mechanism (`set_hovered_kind_silent` → `ids_matching_kind_hover` →
   * `hovered_style_kind`); nothing called it, which is why 2B had to leave the catalogue rows unbound.
   *
   * 🧲️ It listens on the document rather than taking a per-row action, for two reasons: a hover is not a
   * document verb (it must never cost a guest round trip or a repaint), and a per-row `UiValue` arg map
   * is exactly the tree-row cost that starves sibling panels. The row is recognised by ITS OWN kind id
   * matched against this board's catalogs, so no panel id scheme and no plugin name is hard-coded here. */
  const kindDomainById = useMemo(() => board2dKindDomainById(scene?.glyphCatalogsJson ?? ""), [scene?.glyphCatalogsJson]);
  const kindDomainByIdRef = useRef(kindDomainById);
  kindDomainByIdRef.current = kindDomainById;
  useEffect(() => {
    const container = containerRef.current;
    let painted: string | null = null;
    const paint = (hover: { readonly domain: string; readonly kindId: string } | null): void => {
      const next = hover ? `${hover.domain}:${hover.kindId}` : null;
      if (next === painted) return;
      painted = next;
      applyToSession(sessionRef.current, (session) => session.setHoveredKindSilent?.(hover?.domain ?? null, hover?.kindId ?? null));
      if (!container) return;
      if (next === null) container.removeAttribute("data-board-hovered-kind");
      else container.setAttribute("data-board-hovered-kind", next);
    };
    const onPointerOver = (event: globalThis.PointerEvent): void => {
      const target = event.target instanceof globalThis.Element ? event.target.closest<globalThis.HTMLElement>("[id]") : null;
      paint(board2dKindHoverFromElementId(target?.id, kindDomainByIdRef.current));
    };
    globalThis.document.addEventListener("pointerover", onPointerOver, true);
    return () => {
      globalThis.document.removeEventListener("pointerover", onPointerOver, true);
      paint(null);
    };
  }, [sessionEpoch]);
  /** 🐁️ At most one `interactionHover` round trip outstanding, the rest coalesced onto the latest
   * target — the board twin of `World3dHost`'s `dispatchInstanceHover`. An app declaring no interaction
   * domain publishes nothing rather than dispatching a verb no window kind owns. */
  const dispatchBoardHover = useMemo(
    () =>
      createCoalescingActionDispatcher<string | null>((id) => {
        if (!interactionDomainId) return undefined;
        return dispatchSettled("interactionHover", board2dHoverActionArgs(interactionDomainId, (id && granularityByIdRef.current.get(id)) || "node", id));
      }),
    [dispatchSettled, interactionDomainId],
  );
  //#endregion Hover

  //#region BoardEventFlush
  const drainIntoBuffer = useCallback((): void => {
    const session = sessionRef.current;
    if (!session) return;
    try {
      const json = session.drainEventsJson();
      if (!json || json === "[]") return;
      const rows = JSON.parse(json) as BoardEventRow[];
      pendingEventRowsRef.current.push(...rows);
      // 🐁️ Hover leaves the board-events batch here: the engine already painted it locally this frame,
      // and the framework copy travels on its own coalesced `interactionHover` lane so the outliner rows
      // and the sibling panes follow without a whole-surface republish per pointermove.
      const hovered = latestBoard2dHoverId(rows);
      if (hovered !== undefined) {
        publishHoverPaint(hovered);
        dispatchBoardHover(hovered);
      }
      const mutations = collectBoardLiveMirrorMutations(rows);
      pushBoardLiveMirrorMutations(peerScope, node.controllerId, node.surfaceId, mutations);
      // 🕹️ The engine's own `select` row is this pane's OPTIMISTIC selection; `scene.selectionJson`
      // stays the guest-confirmed one, so a probe can tell the two apart the way 3d's does.
      if (mutations.selectionIds) setLocalSelectionJson(JSON.stringify(mutations.selectionIds));
      boardStatusRef.current.pendingEvents = pendingEventRowsRef.current.length;
      publishBoardVitals();
    } catch {
      /* session not ready */
    }
  }, [dispatchBoardHover, peerScope, node.controllerId, node.surfaceId, publishBoardVitals, publishHoverPaint]);

  const dispatchBufferedEvents = useCallback((): void => {
    if (pendingEventRowsRef.current.length === 0) return;
    const { eventsJson, camera } = coalesceBoard2dEvents(pendingEventRowsRef.current);
    pendingEventRowsRef.current = [];
    boardStatusRef.current.pendingEvents = 0;
    publishBoardVitals();
    if (camera) dispatch("setCamera", { camera });
    if (eventsJson !== "[]") dispatch("applyBoardEvents", { eventsJson });
  }, [dispatch, publishBoardVitals]);

  const drainAndMaybeFlush = useCallback((): void => {
    drainIntoBuffer();
    if (pendingEventRowsRef.current.length === 0) return;
    const { flushNow } = coalesceBoard2dEvents(pendingEventRowsRef.current);
    if (flushNow) dispatchBufferedEvents();
  }, [drainIntoBuffer, dispatchBufferedEvents]);

  const flushBoardEvents = useCallback((): void => {
    drainIntoBuffer();
    dispatchBufferedEvents();
  }, [drainIntoBuffer, dispatchBufferedEvents]);

  /** 🩺️ Records one snapshot-apply verdict into the status vitals and republishes them, so a
   * refused snapshot names itself in the DOM instead of only in the console. */
  const recordSnapshotVerdict = useCallback(
    (applied: Board2dScene, parsed: boolean): void => {
      boardStatusRef.current.snapshotParsed = parsed;
      boardStatusRef.current.snapshotChars = applied.snapshotJson.length;
      boardStatusRef.current.refusalReason = parsed ? "" : "engine refused the snapshot";
      boardStatusRef.current.guestRevision += 1;
      containerRef.current?.setAttribute("data-board-snapshot-parsed", String(parsed));
      publishBoardVitals();
    },
    [publishBoardVitals],
  );

  const applyPendingSnapshotIfReady = useCallback(
    (session: Board2dWasmSession): void => {
      const pendingScene = pendingSnapshotSceneRef.current;
      if (!pendingScene) return;
      if (session.defersDescriptorSyncFromJs?.() || cameraInteractionActiveRef.current || boardPeerOwnsGesture(peerScope, node.controllerId, node.surfaceId)) return;
      pendingSnapshotSceneRef.current = null;
      applyToSession(session, (s) => recordSnapshotVerdict(pendingScene, applySnapshotToSession(s, pendingScene)));
    },
    [peerScope, node.controllerId, node.surfaceId, recordSnapshotVerdict],
  );

  /** 🐢️ Mirror of `applyPendingSnapshotIfReady` for the selection-only echo — a peer-owned gesture defers the plugin's `selectionJson` so it doesn't clobber a mirrored preselect highlight mid-marquee. */
  const applyPendingSelectionIfReady = useCallback(
    (session: Board2dWasmSession): void => {
      const pendingSelectionJson = pendingSelectionJsonRef.current;
      if (pendingSelectionJson === null) return;
      if (boardPeerOwnsGesture(peerScope, node.controllerId, node.surfaceId)) return;
      pendingSelectionJsonRef.current = null;
      applyToSession(session, (s) => {
        if (s.setSelectionIdsJsonSilent) s.setSelectionIdsJsonSilent(pendingSelectionJson);
        else s.setSelectionIdsJson(pendingSelectionJson);
      });
    },
    [peerScope, node.controllerId, node.surfaceId],
  );

  onPeerGestureEndedRef.current = (flushed: boolean): void => {
    const session = sessionRef.current;
    if (!session) return;
    if (flushed) {
      pendingSnapshotSceneRef.current = null;
      pendingSelectionJsonRef.current = null;
      return;
    }
    applyPendingSnapshotIfReady(session);
    applyPendingSelectionIfReady(session);
  };

  /**
   * 🫧️ Call when a gesture on this pane ends, right before flushing. Drains first so we know
   * whether a commit is about to go out; if so, drops any pending snapshot/selection stashed mid-gesture
   * instead of applying it — that stashed snapshot is stale (a guest echo that landed while the gesture was
   * live) and the flush response due back in a moment will supersede it anyway, so applying it here would
   * flicker: correct live state -> stale snapshot -> correct committed state. Returns whether a flush is
   * pending, so the caller can pass it on to peers.
   */
  const settleGestureEnd = useCallback(
    (session: Board2dWasmSession): boolean => {
      drainIntoBuffer();
      const flushed = pendingEventRowsRef.current.length > 0;
      if (flushed) {
        pendingSnapshotSceneRef.current = null;
        pendingSelectionJsonRef.current = null;
      } else {
        applyPendingSnapshotIfReady(session);
        applyPendingSelectionIfReady(session);
      }
      return flushed;
    },
    [applyPendingSnapshotIfReady, applyPendingSelectionIfReady, drainIntoBuffer],
  );

  /** 🐁️ Marks a wheel-zoom gesture in flight so scene-driven camera echoes (which lag several ticks behind during a fast scroll) don't fight the live local zoom — mirrors `defersDescriptorSyncFromJs` for pan/drag, which the engine doesn't track for wheel. */
  const beginCameraInteraction = useCallback((): void => {
    cameraInteractionActiveRef.current = true;
    if (cameraSettleTimeoutRef.current) clearTimeout(cameraSettleTimeoutRef.current);
    cameraSettleTimeoutRef.current = setTimeout(() => {
      cameraInteractionActiveRef.current = false;
      cameraSettleTimeoutRef.current = null;
      const session = sessionRef.current;
      if (session) applyPendingSnapshotIfReady(session);
      const pendingCamera = pendingCameraDispatchRef.current;
      if (pendingCamera) {
        pendingCameraDispatchRef.current = null;
        dispatch("setCamera", pendingCamera);
      }
    }, 350);
  }, [applyPendingSnapshotIfReady, dispatch]);

  useEffect(
    () => () => {
      if (cameraSettleTimeoutRef.current) clearTimeout(cameraSettleTimeoutRef.current);
    },
    [],
  );
  //#endregion BoardEventFlush
  return { applyPendingSelectionIfReady, applyPendingSnapshotIfReady, beginCameraInteraction, closeSuggestionMenu, dispatchBufferedEvents, drainAndMaybeFlush, drainIntoBuffer, flushBoardEvents, interactionDomainId, publishHoverPaint, recordSnapshotVerdict, settleGestureEnd, suggestionMenu, suggestionMenuOwnsThisWindowRef, suggestionPopupOwnsThisWindow, suggestionRows };
}

export type BoardCore2 = BoardCore1 & ReturnType<typeof useBoardSuggestionsAndEvents>;
// #endregion Board2dHost
