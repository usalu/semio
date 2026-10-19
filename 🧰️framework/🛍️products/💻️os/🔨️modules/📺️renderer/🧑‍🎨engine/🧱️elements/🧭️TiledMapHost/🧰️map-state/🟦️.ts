/** 🧰️ TiledMapHost state: the scene, shell scope and window instance, the container, canvas, renderer and popup refs, the marquee and context menu state, the labels, the parsed overlay index and hovered feature, the dispatchers and the camera, hit-query and container-size helpers. */
import { useCallback, useContext, useMemo, useRef, useState } from "react";
import { useLabel, useShellScopeOptional, screenRectFromPoints, type SelectionMarqueeCoverage, type SelectionMarqueePoint, type SelectionMarqueeMethod } from "@semio-tech/ui-react";
import { type ComponentSceneHostProps } from "@semio-tech/framework";
// 🐢️ Direct element-to-element imports — `World3dHost`/`🟦️Interpreter` already landed in a prior batch.
import { useMapContextMenuSpecs, useShellContextMenuFallback, type SurfaceContextMenuResult } from "../../🗣️Interpreter/🖱️surface-menu/🟦️.ts";
import { WindowInstanceIdContext } from "../../🗣️Interpreter/🌉️scene-host-contract/🟦️.ts";
import { EMPTY_MAP_OVERLAY_INDEX, getTiledMapCameraLimits, type MapCamera, type MapHoveredFeature, type MapLayerIds, parseMapHoveredFeature, parseMapLayerIds, parseMapOverlayIndex } from "../📐️map-parsing/🟦️.ts";
import { MapRenderer } from "../🖼️map-renderer/🟦️.ts";

// #region TiledMapHost
/** 🧰️ Holds the refs, state, labels, parsed lanes and helper callbacks of the map window. */
export function useMapState({ node, onAction, requestContextMenu }: ComponentSceneHostProps) {
  const scene = node.tiledMap;
  const shellScope = useShellScopeOptional();
  const windowInstanceId = useContext(WindowInstanceIdContext);
  const containerRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const rendererRef = useRef<MapRenderer | null>(null);
  const panningRef = useRef(false);
  const userAdjustedCameraRef = useRef(false);
  const popupRef = useRef<HTMLDivElement>(null);
  const [marqueeOverlay, setMarqueeOverlay] = useState<
    { coverage: SelectionMarqueeCoverage; shape: "rect"; rect: { x: number; y: number; width: number; height: number } } | { coverage: SelectionMarqueeCoverage; shape: "polygon"; points: readonly SelectionMarqueePoint[] } | null
  >(null);
  const [contextMenu, setContextMenu] = useState<SurfaceContextMenuResult & {
    open: boolean;
    position: { x: number; y: number } | null;
  }>({
    open: false,
    position: null,
    items: [],
    titleKey: "ui.surfaceContextMenu.map",
  });
  const contextMenuTitleLabel = useLabel(contextMenu.titleKey);
  const emptySceneLabel = useLabel("ui.host.emptyScene");
  const sourceAvailableLabel = useLabel("ui.host.sourceAvailable");

  const overlayIndex = useMemo(() => (scene ? parseMapOverlayIndex(scene.mapDescriptorJson) : EMPTY_MAP_OVERLAY_INDEX), [scene?.mapDescriptorJson]);
  const hoveredFeature = useMemo(() => (scene ? parseMapHoveredFeature(scene.hoverJson) : null), [scene?.hoverJson]);
  const popupFeature = useMemo(() => (hoveredFeature && overlayIndex.layers.some((layer) => layer.granularity === hoveredFeature.kind && layer.geometry === "point") ? hoveredFeature : null), [hoveredFeature, overlayIndex]);
  const selectionMethod = (scene?.selectionMethod ?? "rectangle") as SelectionMarqueeMethod;

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({ controllerId: node.controllerId, action, args: { surfaceId: node.surfaceId, ...args } });
    },
    [node.controllerId, node.surfaceId, onAction],
  );

  const mapTiledContextMenu = useMapContextMenuSpecs(dispatch);
  const shellContextMenuFallback = useShellContextMenuFallback();

  const dispatchCamera = useCallback(
    (camera: MapCamera) => {
      dispatch("setCamera", { camera });
    },
    [dispatch],
  );

  const clampMapZoom = useCallback((zoom: number): number => {
    const { min, max } = getTiledMapCameraLimits(rendererRef.current?.session);
    return Math.min(max, Math.max(min, zoom));
  }, []);

  const clampCamera = useCallback((next: MapCamera): MapCamera => ({ x: next.x, y: next.y, zoom: clampMapZoom(next.zoom) }), [clampMapZoom]);

  const mirrorSessionCameraToReact = useCallback(() => {
    const parsed = rendererRef.current?.readCameraFromSession();
    if (!parsed) return;
    rendererRef.current!.camera = parsed;
    dispatchCamera(parsed);
  }, [dispatchCamera]);

  const clientToLocal = useCallback((clientX: number, clientY: number): SelectionMarqueePoint => {
    const rect = canvasRef.current?.getBoundingClientRect();
    if (!rect) return { x: 0, y: 0 };
    return { x: clientX - rect.left, y: clientY - rect.top };
  }, []);

  const queryFeatureHits = useCallback(
    (points: readonly SelectionMarqueePoint[], crossing: boolean): MapLayerIds => {
      const session = rendererRef.current?.session;
      if (!session) return {};
      if (selectionMethod === "lasso" && points.length >= 3) {
        return parseMapLayerIds(session.featuresInPolygonJson(JSON.stringify(points.map((point) => [point.x, point.y])), crossing));
      }
      const rect = screenRectFromPoints(points);
      if (!rect) return {};
      return parseMapLayerIds(session.featuresInRectJson(rect.x, rect.y, rect.x + rect.width, rect.y + rect.height, crossing));
    },
    [selectionMethod],
  );

  const queryHitFeature = useCallback((point: SelectionMarqueePoint): MapHoveredFeature | null => {
    const session = rendererRef.current?.session;
    if (!session) return null;
    return parseMapHoveredFeature(session.hitTestFeatureJson(point.x, point.y));
  }, []);

  const resolveMapPaneElement = useCallback((container: HTMLElement): HTMLElement => {
    let el: HTMLElement | null = container;
    while (el) {
      const slot = el.dataset.slot;
      if (slot === "window" || slot === "mode-dock-stack-body") return el;
      el = el.parentElement;
    }
    return container;
  }, []);

  const readContainerSize = useCallback((): { w: number; h: number } => {
    const container = containerRef.current;
    if (!container) return { w: 1, h: 1 };
    const pane = resolveMapPaneElement(container);
    const rect = pane.getBoundingClientRect();
    const style = globalThis.getComputedStyle(pane);
    const padX = Number.parseFloat(style.paddingLeft) + Number.parseFloat(style.paddingRight);
    const padY = Number.parseFloat(style.paddingTop) + Number.parseFloat(style.paddingBottom);
    const innerW = rect.width - (Number.isFinite(padX) ? padX : 0);
    const innerH = rect.height - (Number.isFinite(padY) ? padY : 0);
    return {
      w: Math.max(1, Math.round(innerW || pane.clientWidth || container.clientWidth)),
      h: Math.max(1, Math.round(innerH || pane.clientHeight || container.clientHeight)),
    };
  }, [resolveMapPaneElement]);

  const mirrorSessionCameraToReactRef = useRef(mirrorSessionCameraToReact);
  mirrorSessionCameraToReactRef.current = mirrorSessionCameraToReact;
  const dispatchCameraRef = useRef(dispatchCamera);
  dispatchCameraRef.current = dispatchCamera;
  return { canvasRef, clampCamera, clientToLocal, containerRef, contextMenu, contextMenuTitleLabel, dispatch, dispatchCameraRef, emptySceneLabel, mapTiledContextMenu, marqueeOverlay, mirrorSessionCameraToReact, mirrorSessionCameraToReactRef, node, overlayIndex, panningRef, popupFeature, popupRef, queryFeatureHits, queryHitFeature, readContainerSize, rendererRef, requestContextMenu, resolveMapPaneElement, scene, selectionMethod, setContextMenu, setMarqueeOverlay, shellContextMenuFallback, shellScope, sourceAvailableLabel, userAdjustedCameraRef };
}

export type MapCore1 = ReturnType<typeof useMapState>;
// #endregion TiledMapHost
