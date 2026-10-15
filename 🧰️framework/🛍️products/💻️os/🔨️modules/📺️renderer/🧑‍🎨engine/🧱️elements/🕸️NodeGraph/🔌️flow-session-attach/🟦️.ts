/** 🔌️ The keyboard port, the session creation, the one-time GPU canvas attach, scene application and the scene sync pass.
 *
 * 🧱️ Child hook module of {@link ../🟦️.tsx | FlowGraphCanvasHost}: `useFlowSessionAttach` runs the original statements in order and returns the bindings later hooks and the render read; the component merges them into the shared core object.
 */

import { useCallback, useEffect } from "react";
import { EASED_SURFACE_TRAILING_WINDOW_MS, createDemandFrameScheduler } from "@semio-tech/canvas-react-renderer";
import { nodeGraphActions } from "@semio-tech/framework";
import { createFlowSession } from "../../🪪️WasmSessionLoader/🟦️.tsx";
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
      // 🪪️ A retained surface host unmounts exactly once, when its window closes. A second `host mount`
      // after this line means the React subtree was re-keyed — see `uiSiblingReactKeys`.
      // 🪶️ REDUCE-DEMONSTRATOR-IDLE-MEMORY-FOOTPRINT: was never freed on unmount — the wasm-side
      // session (and everything it retains) leaked for the rest of the document's lifetime.
      if (sessionRef.current) {
        cancelFlowTasks(sessionRef.current);
        void sessionRef.current.free();
      }
      sessionRef.current = null;
    };
  }, [surfaceId]);

  // Attaches the GPU canvas exactly once per session (NOT per document edit — `scene` must stay out
  // of this effect's deps). It used to depend on `scene`, so it re-ran `attachCanvas` on every single
  // commit (including every slider tick): the wasm session rejects a second attach ("canvas surface
  // already attached"), and because the cleanup below was returned from inside the `.then()` instead
  // of from the effect itself, React never saw it — every re-run leaked its ResizeObserver/rAF loop
  // and could disrupt the live GPU surface, which is what read as the whole view "resetting".
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
        // 🪪️ Kept so a canvas that turns out to be unpresentable can cancel exactly this surface.
        const handle = attached as { readonly surface?: number; readonly surfaceGeneration?: number; readonly presentsOnGpu?: boolean } | undefined;
        attachedSurfaceRef.current = typeof handle?.surface === "number" && typeof handle.surfaceGeneration === "number" ? { surface: handle.surface, surfaceGeneration: handle.surfaceGeneration, presentsOnGpu: handle.presentsOnGpu === true } : null;
        surfaceReadyRef.current = true;
        syncFlowSessionFromScene(session, sceneRef.current, appCatalogueRef.current);
        // 🖼️ The opening camera is decided HERE, once per surface, against the pane it actually got:
        // a stored camera that does not frame this graph loses to the fit, and the fit is persisted
        // as a viewport gesture so the next open honours it.
        // 🎥️ An attach that completes while the user is already driving the camera leaves the live
        // camera alone: a surface can re-attach long after boot (a re-keyed subtree, a canvas that
        // lost its device), and a framing decision landing mid-scroll both snaps the view away and
        // publishes a viewport inside a gesture that owes the plugin exactly one, at its settle.
        const opening = isGestureActive() ? { camera: { x: 0, y: 0, zoom: 1 }, fitted: false } : applyFlowStartupCamera(session, sceneRef.current, Math.round(rect.width), Math.round(rect.height));
        framedGraphSignatureRef.current = nodeGraphContentSignature(sceneRef.current.nodes);
        if (opening.fitted) {
          dispatchRef.current(nodeGraphActions.viewport, nodeGraphViewportActionArgs(opening.camera));
        }
        syncFlowCanvasTheme(session);
        // 📏️ The container observer above already owns both backing stores; this only hands the freshly
        // attached surface its first logical size and paints it.
        syncSurfaceSize();
        renderFlow();
        paintOverlays();
        // 🪶️ REDUCE-DEMONSTRATOR-IDLE-MEMORY-FOOTPRINT: was an unconditional 60fps `requestAnimationFrame`
        // loop for the surface's entire lifetime — see `createDemandFrameScheduler`'s docstring.
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
        // 🚨️ Was an anonymous swallow. A rejected attach leaves `surfaceReadyRef` false forever, so every
        // later `renderFlow()` is a silent no-op and the window stays blank with nothing in the console.
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
    // 🧾️ The marks this scene carries are the plugin's OWN, so they set the publication baseline
    // without owing a hop: a selection the plugin made itself (a keyboard verb, an outline pick) must
    // never be re-published back at it, and must never be shadowed by a stale note of what we sent.
    if (scene.selection) interactionLedger.adoptSelection({ nodeIds: scene.selection });
    if (scene.hover !== undefined) interactionLedger.adoptHover({ hoveredId: scene.hover?.nodeId ?? undefined, portId: scene.hover?.portId ?? undefined });
    // 🔀️ An example switch replaces the whole graph under a live camera. Only when the new graph left
    // the view entirely is the camera re-framed — an ordinary edit never moves it.
    const signature = nodeGraphContentSignature(scene.nodes);
    const rect = containerRef.current?.getBoundingClientRect();
    if (rect && surfaceReadyRef.current && framedGraphSignatureRef.current !== null && framedGraphSignatureRef.current !== signature) {
      framedGraphSignatureRef.current = signature;
      readObservedFlowTask(session, "viewport:refit", session.viewport())
        .then((value) => {
          const live = sessionRef.current;
          // 🎥️ A framing decision taken before the user grabbed the camera loses to the gesture: the
          // read is asynchronous, so a refit armed by a graph change can land mid-scroll and both snap
          // the view away under the user's hand and publish a viewport during a gesture that owes the
          // plugin exactly one, at its settle.
          if (!live || isGestureActive()) return;
          const fitted = refitFlowCameraIfContentLeftView(live, sceneRef.current, parseNodeGraphSessionViewport(value), Math.round(rect.width), Math.round(rect.height));
          if (!fitted) return;
          dispatchRef.current(nodeGraphActions.viewport, nodeGraphViewportActionArgs(fitted));
          renderFlow();
          paintOverlays();
        })
        .catch(() => {
          /* a cancelled read is a closed session, not a framing decision */
        });
    }
    renderFlow();
    paintOverlays();
    schedulerRef.current?.invalidate();
  }, [interactionLedger, isGestureActive, paintOverlays, renderFlow, surfaceId]);
  applyDeferredSceneRef.current = applyScene;

  useEffect(() => {
    const session = sessionRef.current;
    if (!session || !sessionReady) return;
    // 🎚️ While a slider (or other continuous) gesture is active, the wasm session already holds the live
    // fixture edits via `setSliderValue`; applying `scene.evalJson` here would install a stale baseline
    // (new slider seeds + old channel outputs) and wipe computing chrome mid-drag. The scene is not
    // dropped: a camera gesture applies it when it ends, and a content gesture's own commit brings the
    // next one.
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
