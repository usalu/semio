/** 👆️ TiledMapHost pointer: the pointer state, the marquee reset, the feature selection emitter and the canvas pointer, context menu and gesture effect. */
import { useCallback, useEffect, useRef, useState } from "../../⚛️react-port/🟦️.ts";
import { marqueeCoverageFromGesture, marqueeModeFromModifiers, screenRectFromPoints, type SelectionMarqueePoint, type SelectionMarqueeMethod } from "@semio-tech/ui-react";
import { GestureRecognizer, applyPinchToCamera, type MergeMode } from "@semio-tech/framework";
import { openSurfaceContextMenu } from "../../🗣️Interpreter/🖱️surface-menu/🟦️.ts";
import { getTiledMapCameraLimits, MAP_MARQUEE_THRESHOLD_PX, mapFeatureHoverActionArgs, mapFeatureSelectionActionArgs, mapLayerIdsFlat, parseMapHoveredFeature, parseMapLayerIds } from "../📐️map-parsing/🟦️.ts";
import type { MapCore2 } from "../🎞️map-render-effects/🟦️.ts";

const TOUCH_PAN_THRESHOLD_PX = 6;

type PanContact = { position: SelectionMarqueePoint };
type MiddlePan = PanContact & { pointerId: number };
type TouchContact = PanContact & { start: SelectionMarqueePoint; panning: boolean };

// #region TiledMapHost
/** 👆️ Wires the canvas pointer gestures, marquee selection, hover and context menu. */
export function useMapPointer(core: MapCore2) {
  const { canvasRef, clientToLocal, dispatch, mapTiledContextMenu, mirrorSessionCameraToReact, node, overlayIndex, panningRef, queryFeatureHits, queryHitFeature, renderAttempt, rendererRef, rendererUnavailable, requestContextMenu, scene, selectionMethod, setContextMenu, setMarqueeOverlay, shellContextMenuFallback, shellScope, userAdjustedCameraRef } = core;
  const pointer = useRef({
    leftDown: false,
    middlePan: null as MiddlePan | null,
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
  const touchContact = useRef<TouchContact | null>(null);

  useEffect(() => {
    if (rendererUnavailable) {
      for (const contact of gestureRecognizer.pointers) gestureRecognizer.up(contact.pointerId);
      touchContact.current = null;
      pointer.current.leftDown = false;
      pointer.current.middlePan = null;
      pointer.current.marqueeTracking = false;
      pointer.current.marqueeActive = false;
      return undefined;
    }
    const canvas = canvasRef.current;
    if (!canvas || !scene) return undefined;
    const beginPan = (start: SelectionMarqueePoint): void => {
      panningRef.current = true;
      userAdjustedCameraRef.current = true;
      rendererRef.current?.session.pointerDownScreen(start.x, start.y, 1);
    };
    const endPan = (contact: PanContact): void => {
      rendererRef.current?.session.pointerUpScreen(contact.position.x, contact.position.y);
    };
    const movePan = (contact: PanContact, point: SelectionMarqueePoint): void => {
      if (point.x === contact.position.x && point.y === contact.position.y) return;
      contact.position = point;
      const renderer = rendererRef.current;
      if (!renderer) return;
      renderer.session.pointerMoveScreen(point.x, point.y);
      const camera = renderer.readCameraFromSession();
      if (camera) renderer.camera = camera;
      renderer.invalidate();
      renderer.scheduleRefreshTiles();
    };
    const moveTouch = (contact: TouchContact, point: SelectionMarqueePoint): void => {
      if (!contact.panning) {
        if (Math.hypot(point.x - contact.start.x, point.y - contact.start.y) < TOUCH_PAN_THRESHOLD_PX) return;
        contact.panning = true;
        beginPan(contact.start);
      }
      movePan(contact, point);
    };
    const publishFinishedCamera = (): void => {
      panningRef.current = false;
      mirrorSessionCameraToReact();
      rendererRef.current?.scheduleRefreshTiles();
    };
    const finishMiddlePan = (event: PointerEvent, pan: MiddlePan): void => {
      pointer.current.middlePan = null;
      if (event.type === "pointerup") movePan(pan, clientToLocal(event.clientX, event.clientY));
      endPan(pan);
      if (canvas.hasPointerCapture?.(event.pointerId)) canvas.releasePointerCapture(event.pointerId);
      publishFinishedCamera();
    };
    const finishTouch = (event: PointerEvent): void => {
      if (!gestureRecognizer.pointers.some(({ pointerId }) => pointerId === event.pointerId)) return;
      event.stopPropagation();
      const verdict = gestureRecognizer.up(event.pointerId);
      if (canvas.hasPointerCapture?.(event.pointerId)) canvas.releasePointerCapture(event.pointerId);
      if (verdict.kind === "pinchEnd") {
        publishFinishedCamera();
        return;
      }
      const contact = touchContact.current;
      if (!contact) return;
      touchContact.current = null;
      const point = event.type === "pointerup" ? clientToLocal(event.clientX, event.clientY) : null;
      if (point) moveTouch(contact, point);
      if (contact.panning) {
        endPan(contact);
        publishFinishedCamera();
      } else if (point) {
        const hit = queryHitFeature(point);
        emitFeatureSelection(hit ? [hit.id] : [], marqueeModeFromModifiers(event, shellScope?.selection.get()), "pick");
      }
    };
    const onPointerDown = (event: PointerEvent): void => {
      event.stopPropagation();
      const point = clientToLocal(event.clientX, event.clientY);
      if (event.pointerType === "touch") {
        if (pointer.current.leftDown || pointer.current.middlePan) return;
        event.preventDefault();
        const verdict = gestureRecognizer.down({ pointerId: event.pointerId, ...point });
        canvas.setPointerCapture?.(event.pointerId);
        if (verdict.kind === "single") {
          touchContact.current = { start: point, position: point, panning: false };
        } else if (verdict.kind === "pinchBegin") {
          const contact = touchContact.current;
          if (contact?.panning) endPan(contact);
          touchContact.current = null;
          panningRef.current = true;
          userAdjustedCameraRef.current = true;
        }
        return;
      }
      if (gestureRecognizer.pointers.length > 0) return;
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
        pointer.current.middlePan = { pointerId: event.pointerId, position: point };
        canvas.setPointerCapture?.(event.pointerId);
        beginPan(point);
      }
    };
    const onPointerMove = (event: PointerEvent): void => {
      const point = clientToLocal(event.clientX, event.clientY);
      if (event.pointerType === "touch") {
        if (!gestureRecognizer.pointers.some(({ pointerId }) => pointerId === event.pointerId)) return;
        event.stopPropagation();
        const verdict = gestureRecognizer.move({ pointerId: event.pointerId, ...point });
        if (verdict.kind === "pinch") {
          const renderer = rendererRef.current;
          const camera = renderer?.readCameraFromSession();
          if (!renderer || !camera) return;
          const rect = canvas.getBoundingClientRect();
          renderer.applyCameraToSession(applyPinchToCamera(camera, verdict.step, { w: rect.width, h: rect.height }, getTiledMapCameraLimits(renderer.session)));
          renderer.scheduleRefreshTiles();
        } else if (verdict.kind === "single" && touchContact.current) {
          moveTouch(touchContact.current, point);
        }
        return;
      }
      if (gestureRecognizer.pointers.length > 0) return;
      const pan = pointer.current.middlePan;
      if (pan) {
        if (pan.pointerId !== event.pointerId) return;
        event.stopPropagation();
        movePan(pan, point);
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
    const onPointerUp = (event: PointerEvent): void => {
      if (event.pointerType === "touch") return finishTouch(event);
      if (gestureRecognizer.pointers.length > 0) return;
      event.stopPropagation();
      const pan = pointer.current.middlePan;
      if (event.button === 1 && pan?.pointerId === event.pointerId) {
        finishMiddlePan(event, pan);
        return;
      }
      if (event.button !== 0 || !pointer.current.leftDown) return;
      const point = clientToLocal(event.clientX, event.clientY);
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
      if (event.pointerType === "touch") return finishTouch(event);
      if (gestureRecognizer.pointers.length > 0) return;
      const pan = pointer.current.middlePan;
      if (pan && pan.pointerId !== event.pointerId) return;
      if (!pan && !pointer.current.leftDown) return;
      pointer.current.leftDown = false;
      resetMarquee();
      if (pan) finishMiddlePan(event, pan);
      else if (canvas.hasPointerCapture?.(event.pointerId)) canvas.releasePointerCapture(event.pointerId);
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
    canvas.addEventListener("lostpointercapture", onPointerCancel);
    canvas.addEventListener("contextmenu", onContextMenu);
    return () => {
      canvas.removeEventListener("pointerdown", onPointerDown);
      window.removeEventListener("pointermove", onPointerMove);
      window.removeEventListener("pointerup", onPointerUp);
      window.removeEventListener("pointercancel", onPointerCancel);
      canvas.removeEventListener("lostpointercapture", onPointerCancel);
      canvas.removeEventListener("contextmenu", onContextMenu);
    };
  }, [clientToLocal, dispatch, emitFeatureSelection, gestureRecognizer, mapTiledContextMenu, mirrorSessionCameraToReact, overlayIndex, queryFeatureHits, queryHitFeature, renderAttempt, rendererUnavailable, requestContextMenu, resetMarquee, scene, selectionMethod, shellContextMenuFallback]);
}

export type MapCore3 = MapCore2 & ReturnType<typeof useMapPointer>;
// #endregion TiledMapHost
