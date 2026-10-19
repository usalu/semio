/** 🔌️ The keyboard port, the session creation, the one-time GPU canvas attach, scene application and the scene sync pass.
 *
 * 🧱️ Child hook module of {@link ../🟦️.tsx | FlowGraphCanvasHost}: `useFlowSessionAttach` runs the original statements in order and returns the bindings later hooks and the render read; the component merges them into the shared core object.
 */

import { useCallback, useEffect } from "react";
import { EASED_SURFACE_TRAILING_WINDOW_MS, createDemandFrameScheduler } from "@semio-tech/framework/canvas/react-renderer";
import { nodeGraphActions } from "@semio-tech/framework";
import { createFlowSession } from "@semio-tech/framework-os/modules/renderer/engine/elements/WasmSessionLoader";
import { nodeGraphViewportActionArgs, parseNodeGraphSessionViewport } from "../🧰️graph-core/🟦️.ts";
import { nodeGraphContentSignature } from "../🖼️dag-overlays/🟦️.tsx";
import {
  applyFlowStartupCamera,
  cancelFlowTasks,
  observeFlowTask,
  readObservedFlowTask,
  refitFlowCameraIfContentLeftView,
  syncFlowCanvasTheme,
  syncFlowSessionFromScene
} from "../🔄️flow-session-sync/🟦️.ts";
import type { FlowCoreAfterFlowInteractionPublication } from "../🧾️flow-interaction-lane/🟦️.ts";
//#region 🔖️NodeGraphHost
//#region 🔖️flow-graph-canvas-host
//#region FlowGraphCanvasHost

export type FlowCoreAfterFlowSessionAttach = FlowCoreAfterFlowInteractionPublication & ReturnType<typeof useFlowSessionAttach>;

export function useFlowSessionAttach(core: FlowCoreAfterFlowInteractionPublication) {
  const {
    scene, surfaceId, keyboardPort, sessionRef, schedulerRef, surfaceReadyRef, gpuCanvasRef, containerRef,
    canvasGeneration, framedGraphSignatureRef, attachedSurfaceRef, sessionReady, setSessionReady, appCatalogueRef,
    sceneSignature, sceneRef, dispatchRef, isGestureActive, deferredSceneRef, applyDeferredSceneRef, paintOverlays,
    renderFlow, syncSurfaceSize, interactionLedger, emitInteractionState
  } = core;
  useEffect(() => {
    keyboardPort.current = {
      selectedIds: () => sceneRef.current.selection ?? [],
      focus: (nodeId) => {
        const session = sessionRef.current;
        if (!session) return;
        observeFlowTask(session, "setHover", session.setHover(nodeId));
        emitInteractionState();
      },
      select: (nodeIds) => {
        const session = sessionRef.current;
        if (!session) return;
        observeFlowTask(session, "setSelection", session.setSelection(JSON.stringify({nodes:nodeIds,edges:[],handles:[]})));
        emitInteractionState();
      },
    };
    return () => {
      keyboardPort.current = null;
    };
  }, [emitInteractionState, keyboardPort]);

  useEffect(() => {
    let cancelled = false;
    void createFlowSession().then((session) => {
      if (cancelled) {
        void session.free();
        return;
      }
      sessionRef.current = session;
      setSessionReady(true);
    }, (error: unknown) => {
    });
    return () => {
      cancelled = true;
      if (sessionRef.current) {
        cancelFlowTasks(sessionRef.current);
        void sessionRef.current.free();
      }
      sessionRef.current = null;
    };
  }, [surfaceId]);

  useEffect(() => {
    const session = sessionRef.current;
    const canvas = gpuCanvasRef.current;
    const container = containerRef.current;
    if (!session || !canvas || !container || !sessionReady) return;
    const rect = container.getBoundingClientRect();
    const dpr = globalThis.devicePixelRatio || 1;
    let cancelled = false;
    let cleanupAttached: (() => void) | undefined;
    const attachment = session.attachCanvas(canvas, Math.round(rect.width), Math.round(rect.height), dpr);
    const unsubscribeAttachment = attachment.subscribe(() => schedulerRef.current?.invalidate());
    void attachment.result
      .then((attached) => {
        if (cancelled) return;
        const handle = attached as { readonly surface?: number; readonly surfaceGeneration?: number; readonly presentsOnGpu?: boolean } | undefined;
        attachedSurfaceRef.current = typeof handle?.surface === "number" && typeof handle.surfaceGeneration === "number" ? { surface: handle.surface, surfaceGeneration: handle.surfaceGeneration, presentsOnGpu: handle.presentsOnGpu === true } : null;
        surfaceReadyRef.current = true;
        syncFlowSessionFromScene(session, sceneRef.current, appCatalogueRef.current);
        const opening = isGestureActive() ? { camera: { x: 0, y: 0, zoom: 1 }, fitted: false } : applyFlowStartupCamera(session, sceneRef.current, Math.round(rect.width), Math.round(rect.height));
        framedGraphSignatureRef.current = nodeGraphContentSignature(sceneRef.current.nodes);
        if (opening.fitted) {
          dispatchRef.current(nodeGraphActions.viewport, nodeGraphViewportActionArgs(opening.camera));
        }
        syncFlowCanvasTheme(session);
        syncSurfaceSize();
        renderFlow();
        paintOverlays();
        const scheduler = createDemandFrameScheduler(
          () => {
            renderFlow();
            paintOverlays();
          },
          { trailingWindowMs: EASED_SURFACE_TRAILING_WINDOW_MS },
        );
        schedulerRef.current = scheduler;
        scheduler.invalidate();
        cleanupAttached = () => {
          scheduler.dispose();
          schedulerRef.current = null;
        };
      })
      .catch((error: unknown) => {
        if (cancelled) return;
      });
    return () => {
      cancelled = true;
      surfaceReadyRef.current = false;
      attachedSurfaceRef.current = null;
      unsubscribeAttachment();
      attachment.cancel();
      cleanupAttached?.();
    };
  }, [sessionReady, isGestureActive, paintOverlays, renderFlow, surfaceId, syncSurfaceSize, canvasGeneration]);

  /** 🎬️ Hands the session the current scene and re-frames only when the graph left the view. Held as a
   * callback rather than inlined in the effect so a gesture that deferred a scene can run the very
   * same body when it ends, instead of waiting for a next scene that may never come. */
  const applyScene = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    const scene = sceneRef.current;
    syncFlowSessionFromScene(session, scene, appCatalogueRef.current);
    if (scene.selection) interactionLedger.adoptSelection({ nodeIds: scene.selection });
    if (scene.hover !== undefined) interactionLedger.adoptHover({ hoveredId: scene.hover?.nodeId ?? undefined, portId: scene.hover?.portId ?? undefined });
    const signature = nodeGraphContentSignature(scene.nodes);
    const rect = containerRef.current?.getBoundingClientRect();
    if (rect && surfaceReadyRef.current && framedGraphSignatureRef.current !== null && framedGraphSignatureRef.current !== signature) {
      framedGraphSignatureRef.current = signature;
      readObservedFlowTask(session, "viewport:refit", session.viewport())
        .then((value) => {
          const live = sessionRef.current;
          if (!live || isGestureActive()) return;
          const fitted = refitFlowCameraIfContentLeftView(live, sceneRef.current, parseNodeGraphSessionViewport(value), Math.round(rect.width), Math.round(rect.height));
          if (!fitted) return;
          dispatchRef.current(nodeGraphActions.viewport, nodeGraphViewportActionArgs(fitted));
          renderFlow();
          paintOverlays();
        })
        .catch(() => {});
    }
    renderFlow();
    paintOverlays();
    schedulerRef.current?.invalidate();
  }, [interactionLedger, isGestureActive, paintOverlays, renderFlow, surfaceId]);
  applyDeferredSceneRef.current = applyScene;

  useEffect(() => {
    const session = sessionRef.current;
    if (!session || !sessionReady) return;
    if (isGestureActive()) {
      deferredSceneRef.current = true;
      renderFlow();
      paintOverlays();
      schedulerRef.current?.invalidate();
      return;
    }
    applyScene();
  }, [sceneSignature, applyScene, isGestureActive, paintOverlays, renderFlow, scene, sessionReady]);

  return {};
}
//#endregion FlowGraphCanvasHost
//#endregion 🔖️flow-graph-canvas-host
//#endregion 🔖️NodeGraphHost
