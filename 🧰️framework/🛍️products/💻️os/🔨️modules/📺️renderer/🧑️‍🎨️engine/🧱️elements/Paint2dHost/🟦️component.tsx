// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/Paint2dHost/component.tsx
/** @emoji 🖌️ `Paint2dHost` — raster-2d `ComponentSceneHost`: drives a `paint2d`-shaped scene through
 * the raster wasm session (brush/selection utilities, marquee selection, navigator viewport), reusing
 * `Canvas2dHost`'s pan/zoom camera math and `Interpreter`'s surface context-menu plumbing. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { useCallback, useContext, useEffect, useRef, useState, type MouseEvent, type PointerEvent as ReactPointerEvent, type WheelEvent as ReactWheelEvent } from "react";
import {
  ContextMenuController,
  registerIntroductionSurfaceResolver,
  windowElementId,
  useLabel,
  useShellScopeOptional,
  useCanvasAppearanceSync,
  useCanvasPickInteraction,
  marqueeModeFromModifiers,
  selectionMergeIds,
  type SelectionMergeMode,
  marqueeCoverageFromGesture,
  screenRectFromPoints,
  type SelectionMarqueeCoverage,
  type SelectionMarqueeMethod,
  SelectionMarquee,
  CanvasPickMenu,
  type ContextMenuItem,
} from "@semio-tech/ui-react";
import { syncSessionCanvasTheme } from "@semio-tech/ui-styling";
import { type ComponentSceneHostProps, type Paint2dScene, type ActionDescriptor, type UiComponentSceneNode, type PluginContextMenuRequest, type ContextMenuItemSpec } from "@semio-tech/framework";
import { type RasterWasmSession, createRasterSession } from "../WasmSessionLoader/🟦️component.tsx";
import { useMapContextMenuSpecs } from "../ShellHost/🟦️component.tsx";
// 🐢️ Direct element-to-element imports — `Canvas2dHost`/`Interpreter` already landed in a prior batch.
import { type CanvasCamera, worldToScreenLogical, wheelCameraAtScreen } from "../Canvas2dHost/🟦️component.tsx";
import { WindowInstanceIdContext } from "../World3dHost/🟦️component.tsx";
import { useShellContextMenuFallback, openSurfaceContextMenu, type SurfaceContextMenuResult } from "../Interpreter/🟦️component.tsx";
// #endregion 🔌️Adapters

//#region 🔖️Paint2dHost
//#region Paint2dParsing
const PAINT_2D_MARQUEE_THRESHOLD_PX = 4;

type Paint2dViewportSize = { readonly width: number; readonly height: number };
type Paint2dScreenRect = { readonly x: number; readonly y: number; readonly width: number; readonly height: number };
type Paint2dPickTarget = { readonly domain: string; readonly id: string; readonly generality: number };

function parsePaint2dCameraJson(json: string | undefined): CanvasCamera {
  try {
    const parsed = JSON.parse(json ?? "{}") as Partial<CanvasCamera>;
    return { x: Number(parsed.x ?? 0), y: Number(parsed.y ?? 0), zoom: Number(parsed.zoom ?? 1) };
  } catch {
    return { x: 0, y: 0, zoom: 1 };
  }
}

function paint2dCameraEqual(a: CanvasCamera, b: CanvasCamera): boolean {
  return a.x === b.x && a.y === b.y && a.zoom === b.zoom;
}

function parsePaint2dViewport(json: string | undefined): Paint2dViewportSize | null {
  if (!json) return null;
  try {
    const parsed = JSON.parse(json) as Partial<Paint2dViewportSize>;
    if (parsed.width == null || parsed.height == null) return null;
    return { width: Number(parsed.width), height: Number(parsed.height) };
  } catch {
    return null;
  }
}

function parsePaint2dSelection(json: string | undefined): string[] {
  try {
    const parsed = JSON.parse(json ?? "[]") as unknown;
    return Array.isArray(parsed) ? parsed.filter((value): value is string => typeof value === "string") : [];
  } catch {
    return [];
  }
}

type Paint2dAssetRecord = { readonly mime?: string; readonly data: string };

function parsePaint2dAssets(json: string | undefined): Record<string, Paint2dAssetRecord> {
  try {
    return JSON.parse(json ?? "{}") as Record<string, Paint2dAssetRecord>;
  } catch {
    return {};
  }
}

export function base64ToBytes(base64: string): Uint8Array {
  const binary = atob(base64);
  const bytes = new Uint8Array(binary.length);
  for (let index = 0; index < binary.length; index += 1) bytes[index] = binary.charCodeAt(index);
  return bytes;
}

function paint2dSelectionMethod(activeUtility: string): SelectionMarqueeMethod | null {
  if (activeUtility === "selectMarquee") return "rectangle";
  if (activeUtility === "selectLasso") return "lasso";
  return null;
}

function isPaint2dSelectionUtility(activeUtility: string): boolean {
  return activeUtility === "selectMarquee" || activeUtility === "selectLasso" || activeUtility === "selectWand";
}
//#endregion Paint2dParsing

//#region Paint2dNoopSession
function noopPaint2dSession(): RasterWasmSession {
  return {
    gpuReady: () => false,
    attachCanvas: async () => undefined,
    setSize: () => {},
    renderFrame: () => {},
    setCamera: () => {},
    wheelScreen: () => {},
    pointerDownScreen: () => {},
    pointerMoveScreen: () => {},
    pointerUpScreen: () => {},
    syncDocumentJson: () => {},
    uploadLayerImage: () => {},
    uploadRasterImageKey: () => {},
    setActiveUtility: () => {},
    setBrushSize: () => {},
    setBrushOpacity: () => {},
    setHoveredIdSilent: () => {},
    setSelectionIdsJson: () => {},
    setCanvasThemeJson: () => {},
    cameraJson: () => '{"x":0,"y":0,"zoom":1}',
    setViewMode: () => {},
    pickTargetsAtScreenJson: () => "[]",
    marqueeHitsJson: () => "[]",
    navigatorFitCameraJson: () => '{"x":0,"y":0,"zoom":1}',
    navigatorViewportOverlayJson: () => '{"x":0,"y":0,"width":0,"height":0}',
    free: () => {},
  };
}
//#endregion Paint2dNoopSession

//#region Paint2dMarqueeOverlay
type Paint2dMarqueeOverlay =
  | { readonly coverage: SelectionMarqueeCoverage; readonly shape: "rect"; readonly rect: Paint2dScreenRect }
  | { readonly coverage: SelectionMarqueeCoverage; readonly shape: "polygon"; readonly points: readonly { readonly x: number; readonly y: number }[] };
//#endregion Paint2dMarqueeOverlay

//#region Paint2dCanvasSurface
function Paint2dCanvasSurface({
  node,
  scene,
  onAction,
  requestContextMenu,
}: {
  readonly node: UiComponentSceneNode;
  readonly scene: Paint2dScene;
  readonly onAction: (action: ActionDescriptor) => void;
  readonly requestContextMenu?: (request: PluginContextMenuRequest) => Promise<readonly ContextMenuItemSpec[]>;
}) {
  const windowInstanceId = useContext(WindowInstanceIdContext);
  const isNavigator = scene.viewMode === "navigator";
  const containerRef = useRef<HTMLDivElement>(null);
  const sessionRef = useRef<RasterWasmSession | null>(null);
  const cameraRef = useRef<CanvasCamera>({ x: 0, y: 0, zoom: 1 });
  const documentSyncRef = useRef<string | null>(null);
  const assetsRef = useRef<string | null>(null);
  const marqueeRef = useRef<{ tracking: boolean; active: boolean; start: { x: number; y: number }; points: { x: number; y: number }[] }>({
    tracking: false,
    active: false,
    start: { x: 0, y: 0 },
    points: [],
  });
  const panRef = useRef<{ last: { x: number; y: number } } | null>(null);

  const [wasmSession, setWasmSession] = useState<RasterWasmSession | null>(null);
  const [attachError, setAttachError] = useState<string | null>(null);
  const [marqueeOverlay, setMarqueeOverlay] = useState<Paint2dMarqueeOverlay | null>(null);
  const [overlayRect, setOverlayRect] = useState<Paint2dScreenRect | null>(null);
  const [contextMenu, setContextMenu] = useState<(SurfaceContextMenuResult & { readonly x: number; readonly y: number }) | null>(null);
  const contextMenuTitleLabel = useLabel(contextMenu?.titleKey ?? "ui.surfaceContextMenu.paint");
  const canvasUnavailableLabel = useLabel("ui.host.canvasUnavailable");

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({ controllerId: node.controllerId, action, args: { surfaceId: node.surfaceId, ...args } });
    },
    [node.controllerId, node.surfaceId, onAction],
  );
  const mapContextMenu = useMapContextMenuSpecs(dispatch);
  const shellContextMenuFallback = useShellContextMenuFallback();

  // 🎯️ Raster layers have no per-entity world position — only `canvasPoint` (camera-space world
  // coordinates) is targetable here, not `entity`/`curve`.
  useEffect(() => {
    if (!windowInstanceId) return;
    return registerIntroductionSurfaceResolver(windowElementId(windowInstanceId), {
      canvasPoint: (x, y) => {
        const container = containerRef.current;
        if (!container) return null;
        const rect = container.getBoundingClientRect();
        const screen = worldToScreenLogical(x, y, cameraRef.current, rect.width, rect.height);
        return { x: rect.left + screen.x, y: rect.top + screen.y, visible: true };
      },
    });
  }, [windowInstanceId]);

  //#region Session lifecycle
  useEffect(() => {
    let cancelled = false;
    let createdSession: RasterWasmSession | null = null;
    void createRasterSession().then((session) => {
      if (cancelled) {
        session.free();
        return;
      }
      createdSession = session;
      setWasmSession(session);
    });
    return () => {
      cancelled = true;
      // 🪶️ REDUCE-DEMONSTRATOR-IDLE-MEMORY-FOOTPRINT: was never freed on unmount — the wasm-side
      // session (and everything it retains) leaked for the rest of the document's lifetime.
      createdSession?.free();
    };
  }, []);

  useEffect(() => {
    if (!wasmSession) return;
    let cancelled = false;
    const timer = setTimeout(() => {
      if (!cancelled && !wasmSession.gpuReady()) setAttachError("WebGPU did not initialize");
    }, 4000);
    return () => {
      cancelled = true;
      clearTimeout(timer);
    };
  }, [wasmSession]);

  const syncAll = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    if (documentSyncRef.current !== scene.documentSyncJson) {
      try {
        session.syncDocumentJson(scene.documentSyncJson);
        documentSyncRef.current = scene.documentSyncJson;
      } catch (error) {
        console.error("[DEBUG] paint2d syncDocumentJson failed", error);
        return;
      }
    }
    if (assetsRef.current !== scene.assetsJson) {
      const assets = parsePaint2dAssets(scene.assetsJson);
      for (const [key, asset] of Object.entries(assets)) {
        try {
          session.uploadRasterImageKey(key, base64ToBytes(asset.data));
        } catch (error) {
          console.error("[DEBUG] paint2d uploadRasterImageKey failed", key, error);
        }
      }
      assetsRef.current = scene.assetsJson;
    }
    session.setActiveUtility(scene.activeUtility);
    session.setBrushSize(scene.brushSize);
    session.setBrushOpacity(scene.brushOpacity);
    session.setSelectionIdsJson(scene.selectionJson);
    session.setHoveredIdSilent(scene.hoveredId ?? null);
    session.setViewMode(scene.viewMode);
    if (isNavigator) {
      const rect = containerRef.current?.getBoundingClientRect();
      const width = rect?.width || 1;
      const height = rect?.height || 1;
      const fit = parsePaint2dCameraJson(session.navigatorFitCameraJson(width, height));
      session.setCamera(fit.x, fit.y, fit.zoom);
      cameraRef.current = fit;
      if (scene.compositeViewportJson) {
        try {
          setOverlayRect(JSON.parse(session.navigatorViewportOverlayJson(scene.cameraJson, scene.compositeViewportJson)) as Paint2dScreenRect);
        } catch {
          setOverlayRect(null);
        }
      } else {
        setOverlayRect(null);
      }
    } else {
      const sceneCamera = parsePaint2dCameraJson(scene.cameraJson);
      if (!paint2dCameraEqual(sceneCamera, cameraRef.current)) {
        session.setCamera(sceneCamera.x, sceneCamera.y, sceneCamera.zoom);
        cameraRef.current = sceneCamera;
      }
    }
    session.renderFrame();
  }, [isNavigator, scene.documentSyncJson, scene.assetsJson, scene.cameraJson, scene.selectionJson, scene.hoveredId, scene.activeUtility, scene.brushSize, scene.brushOpacity, scene.viewMode, scene.compositeViewportJson]);

  useEffect(() => {
    syncAll();
  }, [syncAll]);

  const paint2dCanvasSurfaceShellScope = useShellScopeOptional();
  useCanvasAppearanceSync(
    () => {
      if (!sessionRef.current) return;
      syncSessionCanvasTheme(sessionRef.current);
      sessionRef.current.renderFrame();
    },
    true,
    paint2dCanvasSurfaceShellScope?.rootRef.current ?? undefined,
  );

  const onSessionReady = useCallback(
    (session: RasterWasmSession) => {
      sessionRef.current = session;
      syncAll();
    },
    [syncAll],
  );

  const sessionFactory = useCallback((): RasterWasmSession => wasmSession ?? noopPaint2dSession(), [wasmSession]);
  //#endregion Session lifecycle

  //#region CompositeViewportReporting
  useEffect(() => {
    if (isNavigator) return;
    const container = containerRef.current;
    if (!container) return;
    let last: Paint2dViewportSize = { width: 0, height: 0 };
    const report = () => {
      const rect = container.getBoundingClientRect();
      const width = Math.round(rect.width);
      const height = Math.round(rect.height);
      if (width === last.width && height === last.height) return;
      last = { width, height };
      dispatch("setCompositeViewport", { width, height });
    };
    report();
    const observer = new ResizeObserver(report);
    observer.observe(container);
    return () => observer.disconnect();
  }, [dispatch, isNavigator]);
  //#endregion CompositeViewportReporting

  //#region PickInteraction
  const clientPoint = useCallback((event: { readonly clientX: number; readonly clientY: number }): { readonly x: number; readonly y: number } => {
    const rect = containerRef.current?.getBoundingClientRect();
    return { x: event.clientX - (rect?.left ?? 0), y: event.clientY - (rect?.top ?? 0) };
  }, []);

  const pickInteraction = useCanvasPickInteraction({
    resolveTargetsAtClient: (client) => {
      const session = sessionRef.current;
      const container = containerRef.current;
      if (!session || !container) return [];
      const rect = container.getBoundingClientRect();
      const point = { x: client.x - rect.left, y: client.y - rect.top };
      try {
        const targets = JSON.parse(session.pickTargetsAtScreenJson(point.x, point.y)) as Paint2dPickTarget[];
        return targets.map((target) => ({ ...target, label: target.id }));
      } catch {
        return [];
      }
    },
    onHoverFocus: (focus) => {
      const session = sessionRef.current;
      if (!session) return;
      const id = focus.target?.id ?? null;
      session.setHoveredIdSilent(id);
      session.renderFrame();
      dispatch("setHover", { id });
    },
    onSelectTarget: (target, request) => {
      const mergeMode = marqueeModeFromModifiers(
        {
          shiftKey: request.modifiers?.shift === true,
          ctrlKey: request.modifiers?.ctrl === true,
          metaKey: request.modifiers?.meta === true,
        },
        shellScope?.selection.get(),
      );
      dispatch("setSelection", { ids: selectionMergeIds(mergeMode, parsePaint2dSelection(scene.selectionJson), [target.id]) });
    },
  });
  //#endregion PickInteraction

  //#region Marquee
  const selectionMethod = paint2dSelectionMethod(scene.activeUtility);

  const updateMarqueeOverlay = useCallback(
    (point: { readonly x: number; readonly y: number }) => {
      if (!selectionMethod) return;
      const marquee = marqueeRef.current;
      const points = selectionMethod === "lasso" ? marquee.points : [marquee.start, point];
      const coverage = marqueeCoverageFromGesture({ method: selectionMethod, startX: marquee.start.x, endX: point.x, path: points });
      if (selectionMethod === "lasso") {
        setMarqueeOverlay({ coverage, shape: "polygon", points });
        return;
      }
      const rect = screenRectFromPoints(points);
      if (!rect) return;
      setMarqueeOverlay({ coverage, shape: "rect", rect });
    },
    [selectionMethod],
  );

  const commitMarqueeSelection = useCallback(
    (point: { readonly x: number; readonly y: number }, mergeMode: SelectionMergeMode) => {
      const session = sessionRef.current;
      if (!session) return;
      const marquee = marqueeRef.current;
      const points = selectionMethod === "lasso" ? [...marquee.points, point] : [marquee.start, point];
      const coverage = marqueeCoverageFromGesture({ method: selectionMethod ?? "rectangle", startX: marquee.start.x, endX: point.x, path: points });
      try {
        const hits = JSON.parse(session.marqueeHitsJson(JSON.stringify({ points, crossing: coverage === "partial" }))) as string[];
        dispatch("setSelection", { ids: selectionMergeIds(mergeMode, parsePaint2dSelection(scene.selectionJson), hits) });
      } catch {
        /* marquee hit test failed */
      }
    },
    [dispatch, scene.selectionJson, selectionMethod],
  );
  //#endregion Marquee

  //#region Pointer
  const onPointerDown = useCallback(
    (event: ReactPointerEvent<HTMLDivElement>) => {
      const point = clientPoint(event);
      const session = sessionRef.current;
      if (event.button === 1) {
        if (isNavigator) panRef.current = { last: point };
        else session?.pointerDownScreen(point.x, point.y, event.button);
        event.currentTarget.setPointerCapture(event.pointerId);
        return;
      }
      if (isNavigator || !session) return;
      if (isPaint2dSelectionUtility(scene.activeUtility)) {
        pickInteraction.onCanvasPointerDown({ x: event.clientX, y: event.clientY });
        if (selectionMethod) marqueeRef.current = { tracking: true, active: false, start: point, points: [point] };
        return;
      }
      session.pointerDownScreen(point.x, point.y, event.button);
      session.renderFrame();
    },
    [clientPoint, isNavigator, pickInteraction, scene.activeUtility, selectionMethod],
  );

  const onPointerMove = useCallback(
    (event: ReactPointerEvent<HTMLDivElement>) => {
      const point = clientPoint(event);
      const session = sessionRef.current;
      const pan = panRef.current;
      if (pan) {
        if (isNavigator) {
          const contentCamera = parsePaint2dCameraJson(scene.cameraJson);
          const next = {
            x: contentCamera.x - (point.x - pan.last.x) / contentCamera.zoom,
            y: contentCamera.y - (point.y - pan.last.y) / contentCamera.zoom,
            zoom: contentCamera.zoom,
          };
          panRef.current = { last: point };
          dispatch("setCamera", { camera: next });
        }
        return;
      }
      if (isNavigator || !session) return;
      const marquee = marqueeRef.current;
      if (marquee.tracking) {
        const distance = Math.hypot(point.x - marquee.start.x, point.y - marquee.start.y);
        if (!marquee.active && distance >= PAINT_2D_MARQUEE_THRESHOLD_PX) marquee.active = true;
        if (marquee.active) {
          if (selectionMethod === "lasso") marquee.points = [...marquee.points, point];
          updateMarqueeOverlay(point);
        }
      }
      if (isPaint2dSelectionUtility(scene.activeUtility) && !pickInteraction.pickMenuOpen) {
        pickInteraction.onCanvasPointerMove({ x: event.clientX, y: event.clientY });
        return;
      }
      session.pointerMoveScreen(point.x, point.y);
      const nextCamera = parsePaint2dCameraJson(session.cameraJson());
      if (!paint2dCameraEqual(nextCamera, cameraRef.current)) {
        cameraRef.current = nextCamera;
        dispatch("setCamera", { camera: nextCamera });
      }
      session.renderFrame();
    },
    [clientPoint, dispatch, isNavigator, pickInteraction, scene.activeUtility, scene.cameraJson, selectionMethod, updateMarqueeOverlay],
  );

  const onPointerUp = useCallback(
    (event: ReactPointerEvent<HTMLDivElement>) => {
      const point = clientPoint(event);
      const session = sessionRef.current;
      if (event.currentTarget.hasPointerCapture(event.pointerId)) event.currentTarget.releasePointerCapture(event.pointerId);
      if (panRef.current) {
        panRef.current = null;
        return;
      }
      if (isNavigator || !session) return;
      const marquee = marqueeRef.current;
      if (marquee.tracking) {
        if (marquee.active) {
          const mergeMode = marqueeModeFromModifiers({ shiftKey: event.shiftKey, ctrlKey: event.ctrlKey, metaKey: event.metaKey }, shellScope?.selection.get());
          commitMarqueeSelection(point, mergeMode);
        }
        marqueeRef.current = { tracking: false, active: false, start: point, points: [] };
        setMarqueeOverlay(null);
      }
      if (isPaint2dSelectionUtility(scene.activeUtility)) {
        pickInteraction.onCanvasPointerUp({ x: event.clientX, y: event.clientY }, { shift: event.shiftKey, ctrl: event.ctrlKey, meta: event.metaKey, alt: event.altKey });
        return;
      }
      session.pointerUpScreen(point.x, point.y);
      session.renderFrame();
    },
    [clientPoint, commitMarqueeSelection, isNavigator, pickInteraction, scene.activeUtility],
  );

  const onWheel = useCallback(
    (event: ReactWheelEvent<HTMLDivElement>) => {
      event.preventDefault();
      const point = clientPoint(event);
      const session = sessionRef.current;
      if (isNavigator) {
        const contentCamera = parsePaint2dCameraJson(scene.cameraJson);
        const contentViewport = parsePaint2dViewport(scene.compositeViewportJson) ?? { width: 800, height: 600 };
        const next = wheelCameraAtScreen(contentCamera, point.x, point.y, event.deltaY, contentViewport.width, contentViewport.height);
        dispatch("setCamera", { camera: next });
        return;
      }
      if (!session) return;
      session.wheelScreen(point.x, point.y, event.deltaY);
      const nextCamera = parsePaint2dCameraJson(session.cameraJson());
      cameraRef.current = nextCamera;
      session.renderFrame();
      dispatch("setCamera", { camera: nextCamera });
    },
    [clientPoint, dispatch, isNavigator, scene.cameraJson, scene.compositeViewportJson],
  );

  /** @emoji 🖱️ Pick targets fresh at the click point via the raster wasm session (mirrors `pickInteraction.resolveTargetsAtClient`) — raster layers have no live pick/hover state cached in React, so hits are recomputed here rather than reused. */
  const onContextMenu = useCallback(
    (event: MouseEvent<HTMLDivElement>): void => {
      event.preventDefault();
      if (isNavigator || !requestContextMenu) return;
      event.stopPropagation();
      const session = sessionRef.current;
      const point = clientPoint(event);
      let targets: Paint2dPickTarget[] = [];
      if (session) {
        try {
          targets = JSON.parse(session.pickTargetsAtScreenJson(point.x, point.y)) as Paint2dPickTarget[];
        } catch {
          targets = [];
        }
      }
      const hits = targets.map((target) => ({ domain: target.domain, id: target.id }));
      const selectionIds = parsePaint2dSelection(scene.selectionJson);
      void (async () => {
        const menu = await openSurfaceContextMenu(
          requestContextMenu,
          {
            menu: { id: "paint2d" },
            surface: {
              surfaceId: node.surfaceId,
              kind: "paint2d",
              hits,
              selection: selectionIds.length > 0 ? [{ domain: "layer", ids: selectionIds }] : [],
            },
            windowInstanceId: windowInstanceId ?? undefined,
            point: { x: event.clientX, y: event.clientY },
          },
          mapContextMenu,
          shellContextMenuFallback,
        );
        setContextMenu({ x: event.clientX, y: event.clientY, ...menu });
      })();
    },
    [clientPoint, isNavigator, mapContextMenu, node.surfaceId, requestContextMenu, scene.selectionJson, shellContextMenuFallback, windowInstanceId],
  );
  //#endregion Pointer

  return (
    <div ref={containerRef} className="semio-paint-2d-canvas-surface relative h-full min-h-[24rem] w-full ui-surface" data-level="base" data-controller-id={node.controllerId} data-surface-id={node.surfaceId} data-view-mode={scene.viewMode}>
      <Paint2dWasmCanvas sessionFactory={sessionFactory} onSessionReady={onSessionReady} />
      {attachError ? (
        <div className="absolute inset-0 flex items-center justify-center text-xs text-muted-foreground">
          {canvasUnavailableLabel}: {attachError}
        </div>
      ) : null}
      {marqueeOverlay?.shape === "rect" ? <SelectionMarquee coverage={marqueeOverlay.coverage} shape="rect" rect={marqueeOverlay.rect} /> : null}
      {marqueeOverlay?.shape === "polygon" ? <SelectionMarquee coverage={marqueeOverlay.coverage} shape="polygon" points={marqueeOverlay.points} /> : null}
      {isNavigator && overlayRect ? <div className="pointer-events-none absolute z-20 border-2 border-accent" style={{ left: overlayRect.x, top: overlayRect.y, width: overlayRect.width, height: overlayRect.height }} /> : null}
      <div
        className="absolute inset-0 z-30"
        onPointerDown={onPointerDown}
        onPointerMove={onPointerMove}
        onPointerUp={onPointerUp}
        onPointerLeave={() => pickInteraction.onCanvasPointerLeave()}
        onWheel={onWheel}
        onContextMenu={onContextMenu}
      />
      {!isNavigator ? (
        <CanvasPickMenu request={pickInteraction.pickMenu} hoveredKey={pickInteraction.menuHoveredKey} onHoverKey={pickInteraction.onMenuHoverKey} onPick={pickInteraction.onMenuPick} onDismiss={pickInteraction.dismissPickMenu} />
      ) : null}
      <ContextMenuController
        title={contextMenuTitleLabel}
        open={contextMenu != null}
        position={contextMenu ?? { x: 0, y: 0 }}
        items={contextMenu?.items ?? []}
        onOpenChange={(open) => {
          if (!open) setContextMenu(null);
        }}
      />
    </div>
  );
}
//#endregion Paint2dCanvasSurface

//#region Paint2dWasmCanvas
/** 🖼️ Minimal canvas-attach wrapper (no pointer forwarding — {@link Paint2dCanvasSurface} owns pointer/wheel routing). */
function Paint2dWasmCanvas({ sessionFactory, onSessionReady }: { readonly sessionFactory: () => RasterWasmSession; readonly onSessionReady: (session: RasterWasmSession) => void }) {
  const containerRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const rafRef = useRef<number | null>(null);
  const observerRef = useRef<ResizeObserver | null>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    const container = containerRef.current;
    if (!canvas || !container) return;
    const session = sessionFactory();
    onSessionReady(session);
    const rect = container.getBoundingClientRect();
    const dpr = globalThis.devicePixelRatio || 1;
    const initW = Math.max(1, Math.round(rect.width));
    const initH = Math.max(1, Math.round(rect.height));
    canvas.width = Math.round(initW * dpr);
    canvas.height = Math.round(initH * dpr);
    canvas.style.width = `${initW}px`;
    canvas.style.height = `${initH}px`;
    let disposed = false;
    void session
      .attachCanvas(canvas, initW, initH, dpr)
      .then(() => {
        if (disposed) return;
        const resize = () => {
          const nextRect = container.getBoundingClientRect();
          const nextDpr = globalThis.devicePixelRatio || 1;
          const w = Math.max(1, Math.round(nextRect.width));
          const h = Math.max(1, Math.round(nextRect.height));
          canvas.width = Math.round(w * nextDpr);
          canvas.height = Math.round(h * nextDpr);
          canvas.style.width = `${w}px`;
          canvas.style.height = `${h}px`;
          session.setSize(w, h, nextDpr);
          session.renderFrame();
        };
        resize();
        const observer = new ResizeObserver(resize);
        observer.observe(container);
        observerRef.current = observer;
        const tick = () => {
          session.renderFrame();
          rafRef.current = requestAnimationFrame(tick);
        };
        rafRef.current = requestAnimationFrame(tick);
      })
      .catch(() => {
        /* attach failed — surfaced via session.gpuReady() polling in Paint2dCanvasSurface */
      });
    return () => {
      disposed = true;
      if (rafRef.current != null) cancelAnimationFrame(rafRef.current);
      observerRef.current?.disconnect();
      observerRef.current = null;
    };
  }, [onSessionReady, sessionFactory]);

  return (
    <div ref={containerRef} className="absolute inset-0">
      <canvas ref={canvasRef} className="block h-full w-full touch-none" />
    </div>
  );
}
//#endregion Paint2dWasmCanvas

//#region Paint2dHost
export function Paint2dHost({ node, onAction, requestContextMenu }: ComponentSceneHostProps) {
  const scene = node.paint2d;
  const emptySceneLabel = useLabel("ui.host.emptyScene");
  if (!scene) return <div className="semio-paint-2d-empty">{emptySceneLabel}</div>;
  return <Paint2dCanvasSurface node={node} scene={scene} onAction={onAction} requestContextMenu={requestContextMenu} />;
}
//#endregion Paint2dHost
//#endregion 🔖️Paint2dHost
