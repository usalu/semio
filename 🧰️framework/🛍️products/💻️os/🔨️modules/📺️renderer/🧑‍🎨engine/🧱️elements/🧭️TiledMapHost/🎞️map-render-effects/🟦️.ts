/** 🎞️ TiledMapHost render effects: the layout effect that creates the renderer and session, the scene-sync effects that feed the renderer, the popup placement effect and the wheel zoom handler. */
import { useCallback, useEffect, useLayoutEffect, useRef, useState } from "../../⚛️react-port/🟦️.ts";
import { useCanvasAppearanceSync } from "@semio-tech/ui-react";
import { createMapSession } from "../../🪪️WasmSessionLoader/🟦️.tsx";
import { DEFAULT_CAMERA_JSON, type MapCamera, type MapRenderMode, parseCameraJson, parseMapScreenPoint } from "../📐️map-parsing/🟦️.ts";
import { MapRenderer } from "../🖼️map-renderer/🟦️.ts";
import type { MapCore1 } from "../🧰️map-state/🟦️.ts";

const MAP_STARTUP_TIMEOUT_MS = 15_000;
const MAP_GPU_ATTACH_TIMEOUT_MS = 4_000;
const KEYBOARD_PAN = new Map<string, readonly [number, number]>([
  ["ArrowRight", [-64, 0]], ["ArrowLeft", [64, 0]], ["ArrowDown", [0, -64]], ["ArrowUp", [0, 64]],
]);
const KEYBOARD_ZOOM = new Map([["+", -120], ["=", -120], ["-", 120]]);

// #region TiledMapHost
/** 🎞️ Creates the renderer, keeps it in step with the scene lanes and installs the wheel zoom. */
export function useMapRenderEffects(core: MapCore1) {
  const { canvasRef, clampCamera, containerRef, dispatchCameraRef, mirrorSessionCameraToReact, mirrorSessionCameraToReactRef, panningRef, popupFeature, popupRef, readContainerSize, rendererRef, resolveMapPaneElement, scene, setContextMenu, setMarqueeOverlay, shellScope, userAdjustedCameraRef } = core;
  const [renderAttempt, setRenderAttempt] = useState(0);
  const [canvasRevision, setCanvasRevision] = useState(0);
  const [rendererUnavailable, setRendererUnavailable] = useState(false);
  const recoveryCamera = useRef<MapCamera | null>(null);
  const pendingCanvas = useRef<((canvas: HTMLCanvasElement | null) => void) | null>(null);
  const latestScene = useRef(scene);
  latestScene.current = scene;
  const retryRenderer = useCallback(() => {
    setRendererUnavailable(false);
    setCanvasRevision((revision) => revision + 1);
    setRenderAttempt((attempt) => attempt + 1);
  }, []);

  useLayoutEffect(() => {
    pendingCanvas.current?.(canvasRef.current);
    pendingCanvas.current = null;
  }, [canvasRef, canvasRevision]);

  useLayoutEffect(() => {
    const initialCanvas = canvasRef.current;
    const container = containerRef.current;
    if (!initialCanvas || !container || !scene) return;
    let canvas = initialCanvas;
    setRendererUnavailable(false);
    let disposed = false;
    let renderer: MapRenderer | null = null;
    let resizeRafId: number | null = null;
    let resizeObserver: ResizeObserver | null = null;
    let attachTimer: ReturnType<typeof setTimeout> | undefined;
    let usingCanvas2d = false;
    let stopWatchingContext = () => {};
    const dpr = globalThis.devicePixelRatio || 1;
    const release = (): void => {
      if (disposed) return;
      disposed = true;
      clearTimeout(startupTimer);
      clearTimeout(attachTimer);
      pendingCanvas.current?.(null);
      pendingCanvas.current = null;
      stopWatchingContext();
      resizeObserver?.disconnect();
      if (resizeRafId !== null && typeof globalThis.cancelAnimationFrame === "function") globalThis.cancelAnimationFrame(resizeRafId);
      if (rendererRef.current === renderer) rendererRef.current = null;
      renderer?.dispose();
    };
    const fail = (): void => {
      if (disposed) return;
      if (renderer && rendererRef.current === renderer) {
        recoveryCamera.current = renderer.camera;
        try {
          recoveryCamera.current = renderer.readCameraFromSession() ?? recoveryCamera.current;
        } catch {}
      }
      release();
      panningRef.current = false;
      setMarqueeOverlay(null);
      setContextMenu((previous) => ({ ...previous, open: false }));
      setRendererUnavailable(true);
    };
    const startupTimer = setTimeout(fail, MAP_STARTUP_TIMEOUT_MS);
    const attachCanvas2d = async (next: MapRenderer): Promise<void> => {
      if (disposed) return;
      stopWatchingContext();
      const wasFocused = canvas.ownerDocument.activeElement === canvas;
      const replacement = await new Promise<HTMLCanvasElement | null>((resolve) => {
        pendingCanvas.current = resolve;
        setCanvasRevision((revision) => revision + 1);
      });
      if (!replacement || disposed) return;
      canvas = replacement;
      const { w, h } = readContainerSize();
      await next.attach(replacement, w, h, globalThis.devicePixelRatio || 1, true);
      usingCanvas2d = true;
      if (wasFocused && replacement.ownerDocument.activeElement === replacement.ownerDocument.body) {
        replacement.focus({ preventScroll: true });
      }
    };
    const recover = (): void => {
      const next = renderer;
      if (disposed || !next) return;
      if (usingCanvas2d) {
        fail();
        return;
      }
      next.stopLoop();
      void attachCanvas2d(next).then(() => {
        if (disposed) return;
        watchContext();
        next.startLoop(recover);
        next.scheduleRefreshTiles();
      }).catch(fail);
    };
    const watchContext = (): void => {
      const watched = canvas;
      watched.addEventListener("contextlost", recover);
      stopWatchingContext = () => watched.removeEventListener("contextlost", recover);
    };

    const boot = async (): Promise<void> => {
      const session = await createMapSession();
      if (disposed) {
        session.free();
        return;
      }
      const next = new MapRenderer(scene.tileUrlTemplate, scene.vectorTileUrlTemplate, session);
      renderer = next;
      let { w, h } = readContainerSize();
      for (let attempt = 0; attempt < 240 && (w < 64 || h < 64); attempt += 1) {
        await new Promise<void>((resolve) => {
          if (typeof globalThis.requestAnimationFrame === "function") globalThis.requestAnimationFrame(() => resolve());
          else queueMicrotask(resolve);
        });
        if (disposed) return;
        ({ w, h } = readContainerSize());
      }
      try {
        await Promise.race([
          next.attach(initialCanvas, w, h, dpr),
          new Promise<never>((_, reject) => {
            attachTimer = setTimeout(() => reject(new Error("WebGPU canvas attachment timed out")), MAP_GPU_ATTACH_TIMEOUT_MS);
          }),
        ]);
        usingCanvas2d = canvas.dataset.mapRenderer === "canvas2d";
      } catch {
        await attachCanvas2d(next);
      } finally {
        clearTimeout(attachTimer);
      }
      if (disposed) return;
      watchContext();
      const current = latestScene.current ?? scene;
      next.setRenderMode(current.renderMode as MapRenderMode);
      next.setVectorStyle(current.vectorStyle);
      next.setLayerVisibilityJson(current.layerVisibilityJson);
      next.setLayerStrokeScaleJson(current.layerStrokeScaleJson);
      next.syncDescriptor(current.mapDescriptorJson);
      next.syncInteraction(current.selectionJson, current.hoverJson);
      next.syncTheme();
      rendererRef.current = next;

      const providedCamera = parseCameraJson(current.cameraJson);
      if (recoveryCamera.current) {
        next.applyCameraToSession(clampCamera(recoveryCamera.current));
        recoveryCamera.current = null;
      } else if (!userAdjustedCameraRef.current) {
        if (current.cameraJson === DEFAULT_CAMERA_JSON) session.fitWorldCamera();
        else if (providedCamera) next.applyCameraToSession(clampCamera(providedCamera));
        const bootCamera = next.readCameraFromSession();
        if (bootCamera) {
          next.applyCameraToSession(bootCamera);
          const sameCamera = providedCamera != null && Math.abs(bootCamera.x - providedCamera.x) < 1e-6 && Math.abs(bootCamera.y - providedCamera.y) < 1e-6 && Math.abs(bootCamera.zoom - providedCamera.zoom) < 1e-6;
          if (!sameCamera) dispatchCameraRef.current(bootCamera);
        }
      }

      const applySize = (): void => {
        const nextDpr = globalThis.devicePixelRatio || 1;
        const { w, h } = readContainerSize();
        if (!next.setSize(w, h, nextDpr)) return;
        mirrorSessionCameraToReactRef.current();
        next.scheduleRefreshTiles();
      };

      resizeObserver =
        typeof ResizeObserver === "undefined"
          ? null
          : new ResizeObserver(() => {
              if (resizeRafId !== null) return;
              const schedule =
                typeof globalThis.requestAnimationFrame === "function"
                  ? (fn: () => void) => {
                      resizeRafId = globalThis.requestAnimationFrame(() => {
                        resizeRafId = null;
                        fn();
                      });
                    }
                  : (fn: () => void) => {
                      queueMicrotask(fn);
                    };
              schedule(() => {
                if (disposed) return;
                applySize();
              });
            });
      const pane = resolveMapPaneElement(container);
      resizeObserver?.observe(pane);
      if (pane !== container) resizeObserver?.observe(container);

      applySize();
      next.startLoop(recover);
      next.setLodMode(current.lodMode);
      void next.refreshTiles().catch(fail);
      clearTimeout(startupTimer);
    };
    void boot().catch(fail);
    return release;
  }, [clampCamera, readContainerSize, renderAttempt, resolveMapPaneElement, scene?.tileUrlTemplate, scene?.vectorTileUrlTemplate]);

  useCanvasAppearanceSync(() => rendererRef.current?.syncTheme(), true, shellScope?.rootRef.current ?? undefined);

  useEffect(() => {
    if (!scene) return;
    const renderer = rendererRef.current;
    if (!renderer) return;
    renderer.setRenderMode(scene.renderMode as MapRenderMode);
    renderer.scheduleRefreshTiles();
  }, [scene?.renderMode]);

  useEffect(() => {
    if (!scene) return;
    rendererRef.current?.setVectorStyle(scene.vectorStyle);
  }, [scene?.vectorStyle]);

  useEffect(() => {
    if (!scene) return;
    rendererRef.current?.setLodMode(scene.lodMode);
  }, [scene?.lodMode]);

  useEffect(() => {
    if (!scene) return;
    rendererRef.current?.setLayerVisibilityJson(scene.layerVisibilityJson);
  }, [scene?.layerVisibilityJson]);

  useEffect(() => {
    if (!scene) return;
    rendererRef.current?.setLayerStrokeScaleJson(scene.layerStrokeScaleJson);
  }, [scene?.layerStrokeScaleJson]);

  useEffect(() => {
    if (!scene) return;
    rendererRef.current?.syncDescriptor(scene.mapDescriptorJson);
    rendererRef.current?.scheduleRefreshTiles();
  }, [scene?.mapDescriptorJson]);

  useEffect(() => {
    if (!scene || !rendererRef.current) return;
    rendererRef.current.syncInteraction(scene.selectionJson, scene.hoverJson);
  }, [scene?.selectionJson, scene?.hoverJson]);

  useEffect(() => {
    if (!scene || panningRef.current) return;
    const camera = parseCameraJson(scene.cameraJson);
    if (!camera) return;
    rendererRef.current?.applyCameraToSession(clampCamera(camera));
    rendererRef.current?.scheduleRefreshTiles();
  }, [clampCamera, scene?.cameraJson]);

  useEffect(() => {
    if (!popupFeature || rendererUnavailable) return undefined;
    let raf = 0;
    const tick = () => {
      const screen = rendererRef.current?.session.featureScreenJson(popupFeature.kind, popupFeature.id);
      const parsed = parseMapScreenPoint(screen ?? "null");
      const popup = popupRef.current;
      if (parsed && popup) {
        popup.style.left = `${parsed.x}px`;
        popup.style.top = `${parsed.y}px`;
      }
      raf = requestAnimationFrame(tick);
    };
    raf = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(raf);
  }, [popupFeature, rendererUnavailable]);

  const applyWheelZoom = useCallback(
    (event: WheelEvent): void => {
      event.preventDefault();
      event.stopPropagation();
      const r = rendererRef.current;
      const canvas = canvasRef.current;
      if (!r || !canvas) return;
      const rect = canvas.getBoundingClientRect();
      let deltaY = event.deltaY * (event.deltaMode === WheelEvent.DOM_DELTA_LINE ? 16 : event.deltaMode === WheelEvent.DOM_DELTA_PAGE ? 400 : 1);
      if (event.ctrlKey) deltaY *= 2.5;
      userAdjustedCameraRef.current = true;
      r.session.wheelScreen(event.clientX - rect.left, event.clientY - rect.top, deltaY);
      mirrorSessionCameraToReact();
      r.scheduleRefreshTiles();
    },
    [mirrorSessionCameraToReact],
  );

  useEffect(() => {
    const element = containerRef.current;
    if (!element) return undefined;
    element.addEventListener("wheel", applyWheelZoom, { passive: false });
    return () => element.removeEventListener("wheel", applyWheelZoom);
  }, [applyWheelZoom]);

  const onMapKeyDown = useCallback((event: Pick<KeyboardEvent, "key" | "altKey" | "ctrlKey" | "metaKey" | "preventDefault" | "stopPropagation">): void => {
    if (event.altKey || event.ctrlKey || event.metaKey || panningRef.current) return;
    const renderer = rendererRef.current;
    const canvas = canvasRef.current;
    if (!renderer?.session.rendererReady() || !canvas) return;
    const offset = KEYBOARD_PAN.get(event.key);
    const zoomDelta = KEYBOARD_ZOOM.get(event.key);
    if (!offset && zoomDelta === undefined) return;
    event.preventDefault();
    event.stopPropagation();
    const rect = canvas.getBoundingClientRect();
    const x = rect.width / 2;
    const y = rect.height / 2;
    userAdjustedCameraRef.current = true;
    if (offset) {
      renderer.session.pointerDownScreen(x, y, 1);
      renderer.session.pointerMoveScreen(x + offset[0], y + offset[1]);
      renderer.session.pointerUpScreen(x + offset[0], y + offset[1]);
    } else if (zoomDelta !== undefined) {
      renderer.session.wheelScreen(x, y, zoomDelta);
    }
    mirrorSessionCameraToReact();
    renderer.scheduleRefreshTiles();
  }, [canvasRef, mirrorSessionCameraToReact, panningRef, rendererRef, userAdjustedCameraRef]);
  return { onMapKeyDown, renderAttempt: canvasRevision, rendererUnavailable, retryRenderer };
}

export type MapCore2 = MapCore1 & ReturnType<typeof useMapRenderEffects>;
// #endregion TiledMapHost
