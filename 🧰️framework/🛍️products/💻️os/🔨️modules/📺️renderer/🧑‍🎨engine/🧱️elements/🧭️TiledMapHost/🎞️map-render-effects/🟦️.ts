/** 🎞️ TiledMapHost render effects: the layout effect that creates the renderer and session, the scene-sync effects that feed the renderer, the popup placement effect and the wheel zoom handler. */
import { useCallback, useEffect, useLayoutEffect } from "react";
import { useCanvasAppearanceSync } from "@semio-tech/ui-react";
import { createMapSession } from "../../🪪️WasmSessionLoader/🟦️.tsx";
import { DEFAULT_CAMERA_JSON, type MapRenderMode, parseCameraJson, parseMapScreenPoint } from "../📐️map-parsing/🟦️.ts";
import { MapRenderer } from "../🖼️map-renderer/🟦️.ts";
import type { MapCore1 } from "../🧰️map-state/🟦️.ts";

// #region TiledMapHost
/** 🎞️ Creates the renderer, keeps it in step with the scene lanes and installs the wheel zoom. */
export function useMapRenderEffects(core: MapCore1) {
  const { canvasRef, clampCamera, containerRef, dispatchCameraRef, mirrorSessionCameraToReact, mirrorSessionCameraToReactRef, panningRef, popupFeature, popupRef, readContainerSize, rendererRef, resolveMapPaneElement, scene, shellScope, userAdjustedCameraRef } = core;
  useLayoutEffect(() => {
    const canvas = canvasRef.current;
    const container = containerRef.current;
    if (!canvas || !container || !scene) return;
    let disposed = false;
    let resizeRafId: number | null = null;
    let resizeObserver: ResizeObserver | null = null;
    const dpr = globalThis.devicePixelRatio || 1;

    void createMapSession().then((session) => {
      if (disposed) {
        session.free();
        return;
      }
      const renderer = new MapRenderer(scene.tileUrlTemplate, scene.vectorTileUrlTemplate, session);
      renderer.setRenderMode(scene.renderMode as MapRenderMode);
      renderer.setVectorStyle(scene.vectorStyle);
      renderer.setLodMode(scene.lodMode);
      renderer.setLayerVisibilityJson(scene.layerVisibilityJson);
      renderer.setLayerStrokeScaleJson(scene.layerStrokeScaleJson);
      rendererRef.current = renderer;

      const applySize = (): void => {
        const nextDpr = globalThis.devicePixelRatio || 1;
        const { w, h } = readContainerSize();
        if (!renderer.setSize(w, h, nextDpr)) return;
        mirrorSessionCameraToReactRef.current();
        renderer.scheduleRefreshTiles();
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
        await renderer.attach(canvas, w, h, dpr);
        if (disposed) {
          renderer.dispose();
          return;
        }
        applySize();
        if (!userAdjustedCameraRef.current) {
          if (scene.cameraJson === DEFAULT_CAMERA_JSON) {
            renderer.session.fitWorldCamera();
          } else {
            const bootCamera = parseCameraJson(scene.cameraJson);
            if (bootCamera) renderer.applyCameraToSession(clampCamera(bootCamera));
          }
          const bootCamera = renderer.readCameraFromSession();
          if (bootCamera) {
            renderer.applyCameraToSession(bootCamera);
            // 🧭️ Skip the dispatch when this boot-time read is just mirroring back the camera the
            // scene/document already provided — otherwise opening the app logs a phantom "Set Camera" row
            // in the command-history panel before the user has done anything.
            const providedCamera = parseCameraJson(scene.cameraJson);
            const isPhantomBootCamera = providedCamera != null && Math.abs(bootCamera.x - providedCamera.x) < 1e-6 && Math.abs(bootCamera.y - providedCamera.y) < 1e-6 && Math.abs(bootCamera.zoom - providedCamera.zoom) < 1e-6;
            if (!isPhantomBootCamera) dispatchCameraRef.current(bootCamera);
          }
        }
        renderer.syncDescriptor(scene.mapDescriptorJson);
        renderer.syncInteraction(scene.selectionJson, scene.hoverJson);
        await renderer.refreshTiles();
        renderer.startLoop();
      };

      void boot();
    });

    return () => {
      disposed = true;
      resizeObserver?.disconnect();
      if (resizeRafId !== null && typeof globalThis.cancelAnimationFrame === "function") {
        globalThis.cancelAnimationFrame(resizeRafId);
      }
      rendererRef.current?.dispose();
      rendererRef.current = null;
    };
  }, [clampCamera, readContainerSize, resolveMapPaneElement, scene?.tileUrlTemplate, scene?.vectorTileUrlTemplate]);

  // 🪶️ REDUCE-DEMONSTRATOR-IDLE-MEMORY-FOOTPRINT: theme sync used to run unconditionally every
  // frame inside `MapRenderer`'s render loop (forcing a style recalc every frame via
  // `serializeMapCanvasThemeJson`); now it only runs when the theme actually changes.
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
    if (!popupFeature) return undefined;
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
  }, [popupFeature]);

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
}

export type MapCore2 = MapCore1 & ReturnType<typeof useMapRenderEffects>;
// #endregion TiledMapHost
