/** 👆️ TiledMapHost pointer: the pointer state, the marquee reset, the feature selection emitter and the canvas pointer, context menu and gesture effect. */
import { useCallback, useEffect, useRef, useState } from "react";
import { marqueeCoverageFromGesture, marqueeModeFromModifiers, screenRectFromPoints, type SelectionMarqueePoint, type SelectionMarqueeMethod } from "@semio-tech/ui-react";
import { GestureRecognizer, applyPinchToCamera, type MergeMode } from "@semio-tech/framework";
import { openSurfaceContextMenu } from "../../🗣️Interpreter/🖱️surface-menu/🟦️.ts";
import { getTiledMapCameraLimits, MAP_MARQUEE_THRESHOLD_PX, mapFeatureHoverActionArgs, mapFeatureSelectionActionArgs, mapLayerIdsFlat, parseMapHoveredFeature, parseMapLayerIds } from "../📐️map-parsing/🟦️.ts";
import type { MapCore2 } from "../🎞️map-render-effects/🟦️.ts";

// #region TiledMapHost
/** 👆️ Wires the canvas pointer gestures, marquee selection, hover and context menu. */
export function useMapPointer(core: MapCore2) {
  const { canvasRef, clientToLocal, dispatch, mapTiledContextMenu, mirrorSessionCameraToReact, node, overlayIndex, panningRef, queryFeatureHits, queryHitFeature, rendererRef, requestContextMenu, scene, selectionMethod, setContextMenu, setMarqueeOverlay, shellContextMenuFallback, shellScope, userAdjustedCameraRef } = core;
  const pointer = useRef({
    leftDown: false,
    middleDown: false,
    marqueeTracking: false,
    marqueeActive: false,
    start: { x: 0, y: 0 } as SelectionMarqueePoint,
    points: [] as SelectionMarqueePoint[],
  });

  const resetMarquee = useCallback(() => {
    pointer.current.marqueeTracking = false;
    pointer.current.marqueeActive = false;
    pointer.current.points = [];
    setMarqueeOverlay(null);
  }, []);

  /** 🕹️ `interactionSelect`/`clearSelection` against the framework-owned `"features"` domain's
   * `"feature"` granularity — the generic replacement for the deleted bespoke `setFeatureSelection`
   * action (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM), mirroring
   * `select_feature_action_args`/`layout_select_effect`'s hit/miss split: a miss redispatches
   * `clearSelection` rather than an empty-`targets` `interactionSelect` (`next_selection` treats an
   * empty `targets` as a no-op, not a clear). `crossing` is already baked into `hits` by the caller's
   * coverage-aware hit query, so it carries no separate wire argument. */
  const emitFeatureSelection = useCallback(
    (ids: readonly string[], mode: MergeMode, method: "pick" | SelectionMarqueeMethod) => {
      if (ids.length === 0) {
        dispatch("clearSelection");
        return;
      }
      dispatch("interactionSelect", mapFeatureSelectionActionArgs(ids, mode, method));
    },
    [dispatch],
  );

  const [gestureRecognizer] = useState(() => new GestureRecognizer());

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas || !scene) return undefined;
    /** 🤏️ The second contact hands the map to the shared recognizer's pinch: the first finger's
     * marquee/pan is dropped, every capture released, and tiles are polled continuously until the last
     * finger lifts. */
    const yieldToPinch = (): void => {
      resetMarquee();
      if (pointer.current.middleDown) rendererRef.current?.endContinuousInteraction("pan");
      pointer.current.leftDown = false;
      pointer.current.middleDown = false;
      panningRef.current = false;
      for (const tracked of gestureRecognizer.pointers) if (canvas.hasPointerCapture?.(tracked.pointerId)) canvas.releasePointerCapture(tracked.pointerId);
      userAdjustedCameraRef.current = true;
      rendererRef.current?.beginContinuousInteraction("pinch");
    };
    const onPointerDown = (event: PointerEvent): void => {
      event.stopPropagation();
      const point = clientToLocal(event.clientX, event.clientY);
      const verdict = gestureRecognizer.down({ pointerId: event.pointerId, x: point.x, y: point.y });
      if (verdict.kind === "pinchBegin") yieldToPinch();
      if (verdict.kind !== "single") return;
      if (event.button === 0) {
        pointer.current.leftDown = true;
        pointer.current.marqueeTracking = true;
        pointer.current.marqueeActive = false;
        pointer.current.start = point;
        pointer.current.points = [point];
        canvas.setPointerCapture?.(event.pointerId);
        return;
      }
      if (event.button === 1) {
        event.preventDefault();
        pointer.current.middleDown = true;
        panningRef.current = true;
        userAdjustedCameraRef.current = true;
        canvas.setPointerCapture?.(event.pointerId);
        rendererRef.current?.session.pointerDownScreen(point.x, point.y, 1);
        rendererRef.current?.beginContinuousInteraction("pan");
      }
    };
    const onPointerMove = (event: PointerEvent): void => {
      const point = clientToLocal(event.clientX, event.clientY);
      const verdict = gestureRecognizer.move({ pointerId: event.pointerId, x: point.x, y: point.y });
      if (verdict.kind === "pinch") {
        event.stopPropagation();
        const renderer = rendererRef.current;
        const camera = renderer?.readCameraFromSession();
        if (!renderer || !camera) return;
        const rect = canvas.getBoundingClientRect();
        renderer.applyCameraToSession(applyPinchToCamera(camera, verdict.step, { w: rect.width, h: rect.height }, getTiledMapCameraLimits(renderer.session)));
        renderer.scheduleRefreshTiles();
        return;
      }
      if (verdict.kind !== "single") return;
      if (pointer.current.middleDown) {
        event.stopPropagation();
        rendererRef.current?.session.pointerMoveScreen(point.x, point.y);
        mirrorSessionCameraToReact();
        rendererRef.current?.scheduleRefreshTiles();
        return;
      }
      if (!pointer.current.marqueeTracking) {
        const hit = queryHitFeature(point);
        const nextHover = hit ? { kind: hit.kind, id: hit.id } : null;
        const currentHover = parseMapHoveredFeature(scene.hoverJson);
        if ((currentHover?.id ?? null) !== (nextHover?.id ?? null) || (currentHover?.kind ?? null) !== (nextHover?.kind ?? null)) {
          dispatch("interactionHover", mapFeatureHoverActionArgs(nextHover?.id));
        }
        return;
      }
      event.stopPropagation();
      const distance = Math.hypot(point.x - pointer.current.start.x, point.y - pointer.current.start.y);
      if (!pointer.current.marqueeActive && distance >= MAP_MARQUEE_THRESHOLD_PX) {
        pointer.current.marqueeActive = true;
      }
      if (!pointer.current.marqueeActive) return;
      const method = selectionMethod;
      const points = method === "lasso" ? [...pointer.current.points, point] : [pointer.current.start, point];
      pointer.current.points = points;
      const coverage = marqueeCoverageFromGesture({
        method,
        startX: pointer.current.start.x,
        endX: point.x,
        path: points,
      });
      const rect = screenRectFromPoints(points);
      setMarqueeOverlay(method === "lasso" ? { coverage, shape: "polygon", points } : { coverage, shape: "rect", rect: rect ?? { x: 0, y: 0, width: 0, height: 0 } });
    };
    const endPinch = (): void => {
      rendererRef.current?.endContinuousInteraction("pinch");
      mirrorSessionCameraToReact();
      rendererRef.current?.scheduleRefreshTiles();
    };
    const onPointerUp = (event: PointerEvent): void => {
      event.stopPropagation();
      const verdict = gestureRecognizer.up(event.pointerId);
      if (verdict.kind === "pinchEnd") endPinch();
      if (verdict.kind !== "single") return;
      const point = clientToLocal(event.clientX, event.clientY);
      if (event.button === 1 && pointer.current.middleDown) {
        pointer.current.middleDown = false;
        panningRef.current = false;
        rendererRef.current?.session.pointerUpScreen(point.x, point.y);
        rendererRef.current?.endContinuousInteraction("pan");
        if (canvas.hasPointerCapture?.(event.pointerId)) canvas.releasePointerCapture(event.pointerId);
        mirrorSessionCameraToReact();
        rendererRef.current?.scheduleRefreshTiles();
        return;
      }
      if (event.button !== 0 || !pointer.current.leftDown) return;
      pointer.current.leftDown = false;
      if (canvas.hasPointerCapture?.(event.pointerId)) canvas.releasePointerCapture(event.pointerId);
      const distance = Math.hypot(point.x - pointer.current.start.x, point.y - pointer.current.start.y);
      const mode = marqueeModeFromModifiers(event, shellScope?.selection.get());
      const method = selectionMethod;
      if (pointer.current.marqueeActive && distance >= MAP_MARQUEE_THRESHOLD_PX) {
        const points = method === "lasso" ? [...pointer.current.points, point] : [pointer.current.start, point];
        const coverage = marqueeCoverageFromGesture({
          method,
          startX: pointer.current.start.x,
          endX: point.x,
          path: points,
        });
        emitFeatureSelection(mapLayerIdsFlat(queryFeatureHits(points, coverage === "partial")), mode, method);
      } else if (distance < MAP_MARQUEE_THRESHOLD_PX) {
        const hit = queryHitFeature(point);
        emitFeatureSelection(hit ? [hit.id] : [], mode, "pick");
      }
      resetMarquee();
    };
    const onPointerCancel = (event: PointerEvent): void => {
      const verdict = gestureRecognizer.up(event.pointerId);
      if (verdict.kind === "pinchEnd") endPinch();
      if (verdict.kind !== "single") return;
      const point = clientToLocal(event.clientX, event.clientY);
      rendererRef.current?.session.pointerUpScreen(point.x, point.y);
      pointer.current.leftDown = false;
      pointer.current.middleDown = false;
      panningRef.current = false;
      rendererRef.current?.endContinuousInteraction("pan");
      resetMarquee();
      if (canvas.hasPointerCapture?.(event.pointerId)) canvas.releasePointerCapture(event.pointerId);
      mirrorSessionCameraToReact();
    };
    const onContextMenu = (event: globalThis.MouseEvent): void => {
      event.preventDefault();
      event.stopPropagation();
      if (!requestContextMenu || !scene) return;
      void (async () => {
        const point = clientToLocal(event.clientX, event.clientY);
        const feature = queryHitFeature(point);
        const hits = feature ? [{ domain: feature.kind, id: feature.id }] : [];
        const selection = parseMapLayerIds(scene.selectionJson);
        const selectionGroups=overlayIndex.layers.flatMap(layer=>{const ids=selection[layer.id];return layer.granularity&&ids&&ids.length>0?[{domain:layer.granularity,ids:[...ids]}]:[];});
        const menu = await openSurfaceContextMenu(
          requestContextMenu,
          {
            menu: { id: "tiledMap", args: null },
            surface: { surfaceId: node.surfaceId, kind: "tiledMap", hits, selection: selectionGroups },
            point: { x: event.clientX, y: event.clientY },
          },
          mapTiledContextMenu,
          shellContextMenuFallback,
        );
        setContextMenu({ open: menu.items.length > 0, position: { x: event.clientX, y: event.clientY }, ...menu });
      })();
    };
    canvas.addEventListener("pointerdown", onPointerDown);
    window.addEventListener("pointermove", onPointerMove);
    window.addEventListener("pointerup", onPointerUp);
    window.addEventListener("pointercancel", onPointerCancel);
    canvas.addEventListener("contextmenu", onContextMenu);
    return () => {
      canvas.removeEventListener("pointerdown", onPointerDown);
      window.removeEventListener("pointermove", onPointerMove);
      window.removeEventListener("pointerup", onPointerUp);
      window.removeEventListener("pointercancel", onPointerCancel);
      canvas.removeEventListener("contextmenu", onContextMenu);
    };
  }, [clientToLocal, dispatch, emitFeatureSelection, gestureRecognizer, mapTiledContextMenu, mirrorSessionCameraToReact, overlayIndex, queryFeatureHits, queryHitFeature, requestContextMenu, resetMarquee, scene, selectionMethod, shellContextMenuFallback]);
}

export type MapCore3 = MapCore2 & ReturnType<typeof useMapPointer>;
// #endregion TiledMapHost
