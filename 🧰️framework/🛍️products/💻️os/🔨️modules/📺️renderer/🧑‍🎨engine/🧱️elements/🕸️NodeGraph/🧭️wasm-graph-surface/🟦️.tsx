/** 🧭️ The wasm dag-engine canvas surface: its session wiring, pointer, wheel and keyboard gestures, overlays and context menu.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | NodeGraph}, which re-exports every public name below so import specifiers stay unchanged.
 */

import React, {
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useRef,
  useState
} from "react";
import { type WasmCanvasSession, WasmCanvas } from "@semio-tech/framework/canvas/react-renderer";
import {
  CanvasPickMenu,
  cn,
  ContextMenuController,
  registerIntroductionSurfaceResolver,
  SelectionMarquee,
  surfaceClass,
  useCanvasAppearanceSync,
  useCanvasPickInteraction,
  useLabel,
  useShellScopeOptional,
  type CanvasPickTarget
} from "@semio-tech/ui-react";
import {
  GestureRecognizer,
  nodeGraphActions,
  windowElementId,
  type ActionDescriptor,
  type ContextMenuItemSpec,
  type NodeGraphScene,
  type PluginContextMenuRequest,
  type PluginContextMenuSurfaceTarget
} from "@semio-tech/framework";
import { openSurfaceContextMenu, useMapContextMenuSpecs, useShellContextMenuFallback, type SurfaceContextMenuResult } from "@semio-tech/framework-os/modules/renderer/engine/elements/Interpreter/surface-menu";
import { parseSelectionDomainsFromSession, selectionGroupsFromDomains, WindowInstanceIdContext } from "@semio-tech/framework-os/modules/renderer/engine/elements/Interpreter/scene-host-contract";
import { createGraphSession } from "@semio-tech/framework-os/modules/renderer/engine/elements/WasmSessionLoader";
import {
  DEFAULT_NODE_GRAPH_VIEWPORT,
  type FrameworkGraphSession,
  type GraphKeyboardPort,
  graphPinchWheelPlan,
  nodeGraphEditRows,
  nodeGraphPickChannel,
  publishNodeGraphHover,
  publishNodeGraphSelection,
  syncOptionalGraphCanvasTheme
} from "../🧰️graph-core/🟦️.ts";
import {
  alignModeToDag,
  computeDagMarqueeOverlay,
  dagIntroductionResolver,
  GraphSliderOverlays,
  paintDagLabelOverlays,
  parseDagNodeIdArray,
  parseDagSelectionUnionBoundsScreen,
  sceneToSyncPack,
  SelectionAlignChrome,
  useGraphSliderLanes
} from "../🖼️dag-overlays/🟦️.tsx";

//#region 🔖️NodeGraphHost
//#region WasmGraphSurface
export function WasmGraphSurface({
  scene,
  surfaceId,
  controllerId,
  editable,
  requestContextMenu,
  onAction,
  keyboardPort,
}: {
  readonly scene: NodeGraphScene;
  readonly surfaceId: string;
  readonly controllerId: string;
  readonly editable: boolean;
  readonly requestContextMenu?: (request: PluginContextMenuRequest) => Promise<readonly ContextMenuItemSpec[]>;
  readonly onAction: (action: ActionDescriptor) => void;
  readonly keyboardPort: React.MutableRefObject<GraphKeyboardPort | null>;
}) {
  const windowInstanceId = useContext(WindowInstanceIdContext);
  const sessionRef = useRef<FrameworkGraphSession | null>(null);
  const labelCanvasRef = useRef<HTMLCanvasElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const [contextMenu, setContextMenu] = useState<(SurfaceContextMenuResult & { readonly x: number; readonly y: number }) | null>(null);
  const contextMenuTitleLabel = useLabel(contextMenu?.titleKey ?? "ui.surfaceContextMenu.node");
  const [selectionBounds, setSelectionBounds] = useState<ReturnType<typeof parseDagSelectionUnionBoundsScreen>>(null);
  const [marquee, setMarquee] = useState<ReturnType<typeof computeDagMarqueeOverlay>>(null);
  const [overlaySize, setOverlaySize] = useState({ w: 0, h: 0 });
  const [sliderStateJson, setSliderStateJson] = useState("{}");
  const scenePack = useMemo(() => sceneToSyncPack(scene), [scene]);
  const sceneRef = useRef(scene);
  sceneRef.current = scene;

  const dispatch = useCallback((action: string, args?: Record<string, unknown>) => onAction({ controllerId, action, args: { surfaceId, ...args } }), [controllerId, onAction, surfaceId]);
  const dispatchRef = useRef(dispatch);
  dispatchRef.current = dispatch;
  const { sliderLane } = useGraphSliderLanes(surfaceId, dispatchRef);
  const [gestureRecognizer] = useState(() => new GestureRecognizer());
  const pinchLogScaleRef = useRef(0);

  const mapContextMenu = useMapContextMenuSpecs(dispatch);
  const shellContextMenuFallback = useShellContextMenuFallback();

  const paintOverlays = useCallback(() => {
    const session = sessionRef.current;
    const labelCanvas = labelCanvasRef.current;
    const container = containerRef.current;
    if (!session || !labelCanvas || !container) return;
    const rect = container.getBoundingClientRect();
    const dpr = globalThis.devicePixelRatio || 1;
    try {
      paintDagLabelOverlays(session.labelOverlayPaintStateJson(), labelCanvas, rect.width, rect.height, dpr, {
        hoveredId: session.hoveredNodeId() ?? null,
        selectedIds: parseDagNodeIdArray(session.selectedNodeIdsJson()),
        preselect: { ids: [], removedIds: [] },
        dimmedIds: [],
        highlightIds: sceneRef.current.highlighted ?? [],
      });
    } catch {}
    setSelectionBounds(parseDagSelectionUnionBoundsScreen(session.selectionUnionBoundsScreenJson()));
    setMarquee(computeDagMarqueeOverlay(session.selectionPreviewPointsJson(), session.selectionPreviewCrossing(), session.selectionPreviewMethod?.() ?? "rectangle"));
    try {
      const nextSliderJson = session.sliderOverlayStateJson();
      setSliderStateJson((prev) => (prev === nextSliderJson ? prev : nextSliderJson));
    } catch {}
    setOverlaySize((prev) => (prev.w === rect.width && prev.h === rect.height ? prev : { w: rect.width, h: rect.height }));
    try {
      container.setAttribute("data-viewport-camera-json", JSON.stringify(session.viewport()));
      container.setAttribute("data-session-selection-json", session.selectedNodeIdsJson());
    } catch {}
  }, []);

  useEffect(() => {
    try {
      sessionRef.current?.syncFromScenePack?.(scenePack);
      paintOverlays();
    } catch (error) {
    }
  }, [scenePack, paintOverlays]);

  const onSessionReady = useCallback(
    (session: WasmCanvasSession) => {
      sessionRef.current = session as FrameworkGraphSession;
      syncOptionalGraphCanvasTheme(sessionRef.current);
      try {
        sessionRef.current.syncFromScenePack?.(scenePack);
        paintOverlays();
      } catch (error) {
      }
    },
    [scenePack, paintOverlays],
  );

  const wasmGraphSurfaceShellScope = useShellScopeOptional();
  useCanvasAppearanceSync(
    () => {
      syncOptionalGraphCanvasTheme(sessionRef.current);
      try {
        sessionRef.current?.renderFrame();
      } catch {}
      paintOverlays();
    },
    true,
    wasmGraphSurfaceShellScope?.rootRef.current ?? undefined,
  );

  const [wasmSession, setWasmSession] = useState<FrameworkGraphSession | null>(null);

  useEffect(() => {
    let cancelled = false;
    void createGraphSession().then((session) => {
      if (!cancelled) setWasmSession(session as FrameworkGraphSession);
    });
    return () => {
      cancelled = true;
    };
  }, []);

  useEffect(() => {
    if (!windowInstanceId) return;
    return registerIntroductionSurfaceResolver(windowElementId(windowInstanceId), dagIntroductionResolver(sessionRef, containerRef));
  }, [windowInstanceId]);

  const sessionFactory = useCallback(() => {
    if (wasmSession) return wasmSession;
    return {
      attachCanvas: async () => undefined,
      setSize: () => {},
      renderFrame: () => {},
      syncFromSceneJson: () => {},
      syncFromScenePack: () => {},
      setCanvasThemeJson: () => {},
      pointerDownScreen: () => {},
      pointerMoveScreen: () => {},
      pointerUpScreen: () => {},
      pointerCancelScreen: () => {},
      wheelScreen: () => {},
      labelOverlayPaintStateJson: () => '{"labels":[]}',
      sliderOverlayStateJson: () => "{}",
      selectionUnionBoundsScreenJson: () => "{}",
      selectionPreviewPointsJson: () => "[]",
      selectionPreviewCrossing: () => false,
      selectionPreviewMethod: () => "rectangle",
      selectedNodeIdsJson: () => "[]",
      hoveredNodeId: () => null,
      hoveredChannelJson: () => '{"channel":null,"refusal":null}',
      syncInteraction: () => {},
      viewport: () => scene.viewport ?? DEFAULT_NODE_GRAPH_VIEWPORT,
      pickTargetsAtScreenJson: () => "[]",
      setHover: () => {},
      setHoverChannel: () => {},
      alignSelection: () => {},
      takeGraphEditsJson: () => '{"operations":[]}',
      takePendingOpenInstanceId: () => null,
    } satisfies FrameworkGraphSession;
  }, [scene.viewport, wasmSession]);

  const emitInteractionState = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    try {
      const nodeIds = JSON.parse(session.selectedNodeIdsJson()) as string[];
      publishNodeGraphSelection(dispatch, sceneRef.current.interactionDomain, { nodeIds });
      const hovered = session.hoveredNodeId();
      publishNodeGraphHover(dispatch, sceneRef.current.interactionDomain, hovered);
      const openId = session.takePendingOpenInstanceId?.();
      if (openId) dispatch("openInstance", { instanceId: openId });
    } catch {}
    paintOverlays();
  }, [dispatch, paintOverlays]);

  useEffect(() => {
    keyboardPort.current = {
      selectedIds: () => {
        const session = sessionRef.current;
        return session ? parseDagNodeIdArray(session.selectedNodeIdsJson()) : [];
      },
      focus: (nodeId) => {
        const session = sessionRef.current;
        if (!session?.syncInteraction) return;
        session.syncInteraction(session.selectedNodeIdsJson(), nodeId);
        session.renderFrame();
        paintOverlays();
      },
      select: (nodeIds, focusedId) => {
        const session = sessionRef.current;
        if (!session?.syncInteraction) return;
        session.syncInteraction(JSON.stringify(nodeIds), focusedId);
        session.renderFrame();
        emitInteractionState();
      },
    };
    return () => {
      keyboardPort.current = null;
    };
  }, [emitInteractionState, keyboardPort, paintOverlays]);

  /** 🔗️ Dispatches what the last gesture (or align) did to the graph — the rows the shared DAG journal narrates
   * (`connect`, `disconnect`, `move`, `setSlider`, `insertPort`; design §13.3), the very rows the wgpu renderer writes
   * from the same journal — and nothing when it did nothing. The whole fixture is never published. */
  const dispatchGraphEdits = useCallback(() => {
    const session = sessionRef.current;
    if (!session?.takeGraphEditsJson) return;
    const operations = nodeGraphEditRows(session.takeGraphEditsJson());
    if (operations.length > 0) dispatch(nodeGraphActions.edit, { operations });
  }, [dispatch]);

  const pickInteraction = useCanvasPickInteraction({
    resolveTargetsAtClient: (client) => {
      const session = sessionRef.current;
      const container = containerRef.current;
      if (!session?.pickTargetsAtScreenJson || !container) return [];
      const rect = container.getBoundingClientRect();
      const sx = client.x - rect.left;
      const sy = client.y - rect.top;
      try {
        return JSON.parse(session.pickTargetsAtScreenJson(sx, sy)) as CanvasPickTarget[];
      } catch {
        return [];
      }
    },
    onHoverFocus: (focus) => {
      const session = sessionRef.current;
      if (!session) return;
      const target = focus.target;
      const channel = nodeGraphPickChannel(target);
      if (!target) {
        session.setHover?.(null);
      } else if (channel) {
        session.setHoverChannel?.(channel.nodeId, channel.portId);
      } else {
        session.setHover?.(target.id);
      }
      try {
        const hovered = session.hoveredNodeId();
        publishNodeGraphHover(dispatch, sceneRef.current.interactionDomain, hovered, channel?.portId);
      } catch {}
      session.renderFrame();
      paintOverlays();
    },
    onSelectTarget: () => {
      emitInteractionState();
    },
  });

  return (
    <div
      ref={containerRef}
      className={cn("relative h-full w-full", surfaceClass)}
      data-level="base"
      onContextMenu={(event) => {
        if (!editable || !requestContextMenu) return;
        event.preventDefault();
        event.stopPropagation();
        void (async () => {
          const session = sessionRef.current;
          const container = containerRef.current;
          let hits: NonNullable<PluginContextMenuSurfaceTarget["hits"]> = [];
          let domains = { nodes: [] as string[], edges: [] as string[], handles: [] as string[] };
          if (session && container) {
            const rect = container.getBoundingClientRect();
            const sx = event.clientX - rect.left;
            const sy = event.clientY - rect.top;
            try {
              const targets = JSON.parse(session.pickTargetsAtScreenJson(sx, sy)) as CanvasPickTarget[];
              hits = targets.map((target) => ({ domain: target.domain, id: target.id, label: target.label }));
            } catch {
              hits = [];
            }
            try {
              domains = parseSelectionDomainsFromSession(session.selectionDomainsJson?.() ?? session.selectedNodeIdsJson());
            } catch {
              domains = { nodes: [], edges: [], handles: [] };
            }
          }
          const menu = await openSurfaceContextMenu(
            requestContextMenu,
            {
              menu: { id: "nodeGraph", args: null },
              surface: { surfaceId, kind: "nodeGraph", hits, selection: selectionGroupsFromDomains(domains) },
              windowInstanceId: windowInstanceId ?? undefined,
              point: { x: event.clientX, y: event.clientY },
            },
            mapContextMenu,
            shellContextMenuFallback,
          );
          setContextMenu({ x: event.clientX, y: event.clientY, ...menu });
        })();
      }}
      onPointerUp={emitInteractionState}
    >
      <WasmCanvas className="absolute inset-0" sessionFactory={sessionFactory} onSessionReady={onSessionReady} enablePointer={false} />
      <canvas ref={labelCanvasRef} className="pointer-events-none absolute inset-0 z-40" />
      {selectionBounds ? <div className="pointer-events-none absolute z-20 border-2 border-accent" style={{ left: selectionBounds.x, top: selectionBounds.y, width: selectionBounds.width, height: selectionBounds.height }} /> : null}
      {marquee ? (
        marquee.kind === "lasso" ? (
          <SelectionMarquee className="z-50" coverage={marquee.coverage ?? "full"} shape="polygon" points={marquee.points ?? []} />
        ) : (
          <SelectionMarquee className="z-50" coverage={marquee.coverage ?? "full"} shape="rect" rect={{ x: marquee.x ?? 0, y: marquee.y ?? 0, width: marquee.width ?? 0, height: marquee.height ?? 0 }} />
        )
      ) : null}
      <div
        className="absolute inset-0 z-30 touch-none"
        data-gesture-surface="dag"
        onPointerDown={(event) => {
          const rect = event.currentTarget.getBoundingClientRect();
          const verdict = gestureRecognizer.down({ pointerId: event.pointerId, x: event.clientX - rect.left, y: event.clientY - rect.top });
          if (verdict.kind === "pinchBegin") {
            const session = sessionRef.current;
            pinchLogScaleRef.current = 0;
            pickInteraction.onCanvasPointerLeave();
            for (const tracked of gestureRecognizer.pointers) event.currentTarget.setPointerCapture?.(tracked.pointerId);
            session?.pointerCancelScreen?.();
            session?.renderFrame();
            paintOverlays();
            return;
          }
          if (verdict.kind !== "single") return;
          if (!editable) return;
          if (event.button === 2) return;
          const session = sessionRef.current;
          if (!session?.pointerDownScreen) return;
          const client = { x: event.clientX, y: event.clientY };
          pickInteraction.onCanvasPointerDown(client);
          session.pointerDownScreen(event.clientX - rect.left, event.clientY - rect.top, event.button, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey);
          session.renderFrame();
          paintOverlays();
        }}
        onPointerMove={(event) => {
          const session = sessionRef.current;
          const rect = event.currentTarget.getBoundingClientRect();
          const verdict = gestureRecognizer.move({ pointerId: event.pointerId, x: event.clientX - rect.left, y: event.clientY - rect.top });
          if (verdict.kind === "pinch") {
            if (!session?.wheelScreen) return;
            const plan = graphPinchWheelPlan(verdict.step, pinchLogScaleRef.current);
            pinchLogScaleRef.current = plan.pendingLogScale;
            if (plan.calls.length === 0) return;
            for (const call of plan.calls) session.wheelScreen(call.sx, call.sy, call.deltaX, call.deltaY, call.zoomGesture);
            session.renderFrame();
            paintOverlays();
            return;
          }
          if (verdict.kind !== "single") return;
          if (!session?.pointerMoveScreen) return;
          const client = { x: event.clientX, y: event.clientY };
          pickInteraction.onCanvasPointerMove(client);
          session.pointerMoveScreen(event.clientX - rect.left, event.clientY - rect.top, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey);
          session.renderFrame();
          paintOverlays();
        }}
        onPointerUp={(event) => {
          const session = sessionRef.current;
          const verdict = gestureRecognizer.up(event.pointerId);
          if (verdict.kind === "pinchEnd") {
            pinchLogScaleRef.current = 0;
            emitInteractionState();
          }
          if (verdict.kind !== "single") return;
          if (!session?.pointerUpScreen) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const client = { x: event.clientX, y: event.clientY };
          pickInteraction.onCanvasPointerUp(client, { shift: event.shiftKey, ctrlOrMeta: event.metaKey || event.ctrlKey, alt: event.altKey });
          session.pointerUpScreen(event.clientX - rect.left, event.clientY - rect.top, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey);
          session.renderFrame();
          emitInteractionState();
          dispatchGraphEdits();
        }}
        onPointerCancel={(event) => {
          const session = sessionRef.current;
          const verdict = gestureRecognizer.up(event.pointerId);
          if (verdict.kind === "pinchEnd") {
            pinchLogScaleRef.current = 0;
            emitInteractionState();
          }
          if (verdict.kind !== "single") return;
          pickInteraction.onCanvasPointerLeave();
          if (!session?.pointerCancelScreen) return;
          session.pointerCancelScreen();
          session.renderFrame();
          paintOverlays();
        }}
        onPointerLeave={() => pickInteraction.onCanvasPointerLeave()}
        onWheel={(event) => {
          event.preventDefault();
          const session = sessionRef.current;
          if (!session?.wheelScreen) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const delta = event.deltaMode === 1 ? event.deltaY * 16 : event.deltaMode === 2 ? event.deltaY * 400 : event.deltaY;
          session.wheelScreen(event.clientX - rect.left, event.clientY - rect.top, 0, delta, true);
          session.renderFrame();
          emitInteractionState();
        }}
      />
      {selectionBounds && editable ? (
        <SelectionAlignChrome
          bounds={selectionBounds}
          onAlign={(mode) => {
            const session = sessionRef.current;
            if (!session?.alignSelection) return;
            session.alignSelection(alignModeToDag(mode));
            dispatchGraphEdits();
            session.renderFrame();
            emitInteractionState();
          }}
        />
      ) : null}
      <GraphSliderOverlays
        scopeId={JSON.stringify([windowInstanceId, controllerId, surfaceId])}
        stateJson={sliderStateJson}
        logicalW={overlaySize.w}
        logicalH={overlaySize.h}
        editable={editable}
        onSliderChange={(widgetId, value) => sliderLane(widgetId).offer(value)}
        onSliderCommit={(widgetId, value) => sliderLane(widgetId).commit(value)}
        onSliderAbort={(widgetId, reason) => sliderLane(widgetId).abort(reason)}
      />
      <CanvasPickMenu request={pickInteraction.pickMenu} hoveredKey={pickInteraction.menuHoveredKey} onHoverKey={pickInteraction.onMenuHoverKey} onPick={pickInteraction.onMenuPick} onDismiss={pickInteraction.dismissPickMenu} />
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
//#endregion WasmGraphSurface
//#endregion 🔖️NodeGraphHost
