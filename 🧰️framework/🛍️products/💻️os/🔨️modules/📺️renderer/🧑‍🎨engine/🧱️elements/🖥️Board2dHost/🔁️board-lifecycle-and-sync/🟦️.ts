/** 🔁️ Board2dHost lifecycle and sync: the layout effect that creates the board session and the scene sync effects that feed the session and the canvas appearance. */
import { useEffect, useLayoutEffect } from "react";
import { useCanvasAppearanceSync } from "@semio-tech/ui-react";
import { syncSessionCanvasTheme } from "@semio-tech/ui-styling";
import { type DemandFrameSchedulerV1, createDemandFrameScheduler, frameDemandingSessionV1, type Board2dWasmSession } from "@semio-tech/framework/canvas/react-renderer";
import { type Board2dPeer } from "../../🪪️WasmSessionLoader/🟦️.tsx";
import { parseBoard2dAreaBrushSize, parseBoard2dTransformFlags, parseBoardCamera } from "../🔬️board-parsing/🟦️.ts";
import { applyBoard2dHighlightedIds, applySnapshotToSession, applyToSession, boardPeerOwnsGesture, registerBoard2dPeer, unregisterBoard2dPeer } from "../🔄️session-sync/🟦️.ts";
import type { BoardCore2 } from "../💡️board-suggestions-and-events/🟦️.ts";

// #region Board2dHost
/** 🔁️ Creates the board session and keeps it in step with the scene lanes. */
export function useBoardLifecycleAndSync(core: BoardCore2) {
  const { board2dHostShellScope, bootSyncedRef, cameraInteractionActiveRef, canvasRef, containerRef, factory, hoverActiveRef, node, onPeerGestureEndedRef, peerRef, peerScope, pendingSelectionJsonRef, pendingSnapshotSceneRef, publishBoardVitals, publishBoardVitalsRef, publishHoverPaint, rawSessionRef, readContainerSize, recordSnapshotVerdict, scene, schedulerRef, sessionEpoch, sessionRef, setLocalSelectionJson, setSessionEpoch, setSessionError, vitalsPendingRef } = core;

  //#region SessionLifecycle
  useLayoutEffect(() => {
    const canvas = canvasRef.current;
    const container = containerRef.current;
    if (!canvas || !container) return undefined;
    if (!factory) throw new Error("The current app has no registered board session factory.");
    let disposed = false;
    let resizeObserver: ResizeObserver | null = null;
    let owner: Board2dWasmSession | null = null;
    let ownerHandle: Board2dWasmSession | null = null;
    let ownerScheduler: DemandFrameSchedulerV1 | null = null;
    let peer: Board2dPeer | null = null;
    let booting = false;
    const release = (): void => {
      const session = owner;
      owner = null;
      ownerScheduler?.dispose();
      if (schedulerRef.current === ownerScheduler) schedulerRef.current = null;
      if (sessionRef.current === ownerHandle) sessionRef.current = null;
      if (rawSessionRef.current === session) rawSessionRef.current = null;
      session?.free();
    };
    const fail = (error: unknown): void => {
      resizeObserver?.disconnect();
      unregisterBoard2dPeer(peerScope, node.controllerId, node.surfaceId, peer);
      release();
      if (!disposed) setSessionError(error instanceof Error ? error : new Error(String(error)));
    };

    void factory.create().then((session) => {
      if (disposed) {
        session.free();
        return;
      }
      owner = session;
      const scheduler = createDemandFrameScheduler(() => {
        try {
          session.renderFrame();
        } catch {
          return;
        }
        if (!vitalsPendingRef.current) return;
        vitalsPendingRef.current = false;
        publishBoardVitalsRef.current();
      });
      const handle = frameDemandingSessionV1(session, scheduler);
      ownerScheduler = scheduler;
      ownerHandle = handle;
      schedulerRef.current = scheduler;
      rawSessionRef.current = session;
      sessionRef.current = handle;
      peer = { session: handle, onPeerGestureEnded: (flushed) => onPeerGestureEndedRef.current(flushed) };
      peerRef.current = peer;
      registerBoard2dPeer(peerScope, node.controllerId, node.surfaceId, peer);

      const applySize = (): void => {
        const nextDpr = globalThis.devicePixelRatio || 1;
        const { w, h } = readContainerSize();
        session.setSize(w, h, nextDpr);
        scheduler.invalidate();
      };

      const boot = async (): Promise<void> => {
        let { w, h } = readContainerSize();
        for (let attempt = 0; attempt < 240 && (w < 64 || h < 64); attempt += 1) {
          await new Promise<void>((resolve) => {
            if (typeof globalThis.requestAnimationFrame === "function") globalThis.requestAnimationFrame(() => resolve());
            else queueMicrotask(resolve);
          });
          if (disposed) return;
          ({ w, h } = readContainerSize());
        }
        const dpr = globalThis.devicePixelRatio || 1;
        await session.attach_canvas(canvas, w, h, dpr);
        if (disposed) return;
        applySize();
        syncSessionCanvasTheme(session);
        scheduler.paintNow();
        setSessionEpoch((epoch) => epoch + 1);
      };

      resizeObserver =
        typeof ResizeObserver === "undefined"
          ? null
          : new ResizeObserver(() => {
              applySize();
            });
      resizeObserver?.observe(container);
      booting = true;
      void boot().finally(() => {
        booting = false;
        if (disposed) release();
      }).catch(fail);
    }).catch(fail);

    return () => {
      disposed = true;
      resizeObserver?.disconnect();
      ownerScheduler?.dispose();
      unregisterBoard2dPeer(peerScope, node.controllerId, node.surfaceId, peer);
      if (peerRef.current === peer) peerRef.current = null;
      if (sessionRef.current === ownerHandle) sessionRef.current = null;
      if (!booting) release();
    };
  }, [node.controllerId, node.surfaceId, readContainerSize, factory?.create, factory?.pluginId, factory?.appId, factory?.instanceId, peerScope]);
  //#endregion SessionLifecycle

  //#region SceneSync
  useEffect(() => {
    if (!scene) return;
    const session = sessionRef.current;
    if (!session) return;
    if (session.defersDescriptorSyncFromJs?.() || cameraInteractionActiveRef.current || boardPeerOwnsGesture(peerScope, node.controllerId, node.surfaceId)) {
      pendingSnapshotSceneRef.current = scene;
      return;
    }
    applyToSession(session, (s) => recordSnapshotVerdict(scene, applySnapshotToSession(s, scene)));
    if (!bootSyncedRef.current) {
      bootSyncedRef.current = true;
      try {
        session.drainEventsJson();
      } catch {
        /* session not ready */
      }
    }
  }, [peerScope, recordSnapshotVerdict, sessionEpoch, scene?.snapshotJson, node.controllerId, node.surfaceId]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => session.setKindCatalogsJson(scene.glyphCatalogsJson));
  }, [sessionEpoch, scene?.glyphCatalogsJson]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => session.setHandleLinkCompatJson?.(scene.placementCompatibilityJson));
  }, [sessionEpoch, scene?.placementCompatibilityJson]);

  useEffect(() => {
    if (!scene) return;
    const session = sessionRef.current;
    if (!session) return;
    if (boardPeerOwnsGesture(peerScope, node.controllerId, node.surfaceId)) {
      pendingSelectionJsonRef.current = scene.selectionJson;
      return;
    }
    applyToSession(session, (s) => {
      if (s.setSelectionIdsJsonSilent) s.setSelectionIdsJsonSilent(scene.selectionJson);
      else s.setSelectionIdsJson(scene.selectionJson);
    });
    setLocalSelectionJson(null);
  }, [peerScope, sessionEpoch, scene?.selectionJson, node.controllerId, node.surfaceId]);

  useEffect(() => {
    if (!scene) return;
    const session = sessionRef.current;
    if (!session || session.defersDescriptorSyncFromJs?.() || cameraInteractionActiveRef.current) return;
    applyToSession(session, (s) => {
      const camera = parseBoardCamera(scene.cameraJson);
      if (!camera) return;
      if (s.setCameraSilent) s.setCameraSilent(camera.x, camera.y, camera.zoom);
      else s.setCamera(camera.x, camera.y, camera.zoom);
    });
  }, [sessionEpoch, scene?.cameraJson]);

  // 🐁️ The guest's hover echo paints the panes the pointer is NOT over — an outliner or catalogue row
  // hovered in a panel highlights the node on every canvas this way. It is deliberately NOT applied to
  // the pane under the pointer: that pane's engine already painted its own raycast this frame, and an
  // echo lagging a round trip behind would blink the live hover off and back on.
  useEffect(() => {
    if (!scene || hoverActiveRef.current) return;
    publishHoverPaint(scene.hoveredId ?? null);
    applyToSession(sessionRef.current, (session) => session.setHoveredIdSilent?.(scene.hoveredId ?? null));
  }, [publishHoverPaint, sessionEpoch, scene?.hoveredId]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => session.setActiveUtility?.(scene.activeUtility ?? "select"));
  }, [sessionEpoch, scene?.activeUtility]);

  useEffect(() => {
    if (!scene) return;
    applyBoard2dHighlightedIds(sessionRef.current, scene);
  }, [sessionEpoch, scene?.highlightedIdsJson]);

  useEffect(() => {
    if (!scene || !board2dHostShellScope) return;
    const updateOptions = () => {
      const mode = board2dHostShellScope.selection.get();
      // 🎯️ The owning app's selectable-kind filter; an older scene omits the flags and reads as on.
      // Engine argument order is (nodes, edges, handles).
      applyToSession(sessionRef.current, (session) =>
        session.setSelectionOptions?.(scene.selectionMethod, mode, scene.selectableNodes !== false, scene.selectableEdges !== false, scene.selectableHandles !== false),
      );
    };
    updateOptions();
    return board2dHostShellScope.selection.subscribe(updateOptions);
  }, [sessionEpoch, scene?.selectionMethod, scene?.selectableNodes, scene?.selectableEdges, scene?.selectableHandles, board2dHostShellScope]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => session.setGridVisible?.(scene.gridVisible !== false));
  }, [sessionEpoch, scene?.gridVisible]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => session.setGridSnapEnabled?.(scene.gridSnapEnabled));
  }, [sessionEpoch, scene?.gridSnapEnabled]);

  // 🕹️ `setTransformGumballFlag` composes which gumball handles the select utility offers; a scene
  // that declares none leaves the engine's own default (move + rotate, never scale).
  useEffect(() => {
    if (!scene) return;
    const flags = parseBoard2dTransformFlags(scene.transformFlags);
    applyToSession(sessionRef.current, (session) => session.setTransformFlags?.(flags.move, flags.rotate));
    publishBoardVitals();
  }, [publishBoardVitals, sessionEpoch, scene?.transformFlags]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => session.setGridFactor?.(scene.gridFactor));
  }, [sessionEpoch, scene?.gridFactor]);

  // 🖍️ What ONE area-brush click paints; a click-drag states its own rectangle and ignores this.
  useEffect(() => {
    if (!scene) return;
    const size = parseBoard2dAreaBrushSize(scene.areaBrushSize);
    applyToSession(sessionRef.current, (session) => session.setAreaBrushExtent?.(size.width, size.height));
  }, [sessionEpoch, scene?.areaBrushSize]);

  // 🏷️ The owning domain's brush placement naming, so this lane mints the same ids as the wgpu lane and the domain's own boards.
  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => session.setPlacementNamingJson?.(scene.placementNaming ?? null));
  }, [sessionEpoch, scene?.placementNaming]);

  useEffect(() => {
    if (!scene || scene.suggestionOffset <= 0) return;
    applyToSession(sessionRef.current, (session) => session.setSuggestionOffset?.(scene.suggestionOffset));
  }, [sessionEpoch, scene?.suggestionOffset]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => session.setBrushKindWeights?.(scene.brushWeightsJson));
  }, [sessionEpoch, scene?.brushWeightsJson]);

  useEffect(() => {
    if (!scene) return;
    applyToSession(sessionRef.current, (session) => {
      if (scene.lodMode === "automatic") {
        session.setAutomaticLod?.(true);
      } else {
        session.setAutomaticLod?.(false);
        session.setForcedDrawLodLabel?.(scene.lodMode);
      }
    });
  }, [sessionEpoch, scene?.lodMode]);
  //#endregion SceneSync

  // 🩺️ `data-board-interaction-json`/`data-board-transform-json` are written ONLY here and from
  // `scheduleRender`, never from JSX — a React re-render must not stamp a stale frame over the live one.
  useEffect(() => {
    publishBoardVitals();
  });

  useCanvasAppearanceSync(
    () => {
      syncSessionCanvasTheme(sessionRef.current);
      try {
        sessionRef.current?.renderFrame();
      } catch {
        /* gpu not ready */
      }
    },
    true,
    board2dHostShellScope?.rootRef.current ?? undefined,
  );
}

export type BoardCore3 = BoardCore2 & ReturnType<typeof useBoardLifecycleAndSync>;
// #endregion Board2dHost
