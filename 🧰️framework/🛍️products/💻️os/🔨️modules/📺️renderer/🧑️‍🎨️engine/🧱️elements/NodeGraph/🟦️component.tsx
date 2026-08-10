// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/NodeGraph/component.tsx
/** @emoji 🕸️ `NodeGraph` — the node-graph/flow program scene host: wasm dag-engine canvas surface,
 * the flow-engine (React Flow) canvas host, the catalogue double-click spotlight, label/slider/marquee
 * canvas overlays shared by both engines, and the SSR-safe `Diagram`-based fallback. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import React, { useCallback, useContext, useEffect, useMemo, useRef, useState, type DragEvent, type KeyboardEvent, type MouseEvent } from "react";
import { type GraphWasmSession, GraphWasmCanvas } from "@semio-tech/infinite-canvas-react-renderer";
import { currentStylingAppearanceName, resolveColorHex, syncSessionCanvasTheme } from "@semio-tech/ui-styling";
import {
  borderNormalBottomClass,
  CanvasPickMenu,
  CATALOGUE_DRAG_MIME,
  cn,
  ContextMenuController,
  Diagram,
  floatingMenuItemClass,
  floatingMenuSurfaceClass,
  getActiveCatalogueDragPayload,
  glassClass,
  Handle,
  Input,
  pickMostSpecificCanvasTarget,
  Position,
  registerIntroductionSurfaceResolver,
  SelectionMarquee,
  Slider,
  surfaceClass,
  useCanvasAppearanceSync,
  useCanvasPickInteraction,
  useLabel,
  useShellScopeOptional,
  type CanvasPickTarget,
  type ContextMenuItem,
  type Edge,
  type IntroductionResolvedGeometry,
  type IntroductionSurfaceResolver,
  type Node,
  type NodeProps,
  type NodeTypes,
} from "@semio-tech/ui-react";
import {
  nodeGraphActions,
  windowElementId,
  type ActionDescriptor,
  type ComponentSceneHostProps,
  type ContextMenuItemSpec,
  type NodeGraphEdgeRecord,
  type NodeGraphFindItem,
  type NodeGraphHover,
  type NodeGraphNodeRecord,
  type NodeGraphPortRecord,
  type NodeGraphScene,
  type NodeGraphViewport,
  type PluginContextMenuRequest,
  type PluginContextMenuSurfaceTarget,
  type PresencePeer,
  type UiComponentSceneNode,
} from "@semio-tech/framework";
import { encodePackValue } from "@semio-tech/framework-os";
import { openSurfaceContextMenu, parseSceneJsonField, useShellContextMenuFallback, type SurfaceContextMenuResult } from "../Interpreter/🟦️component.tsx";
import { mapContextMenuSpecs, parseJsonArray, parseSelectionDomainsFromSession, selectionGroupsFromDomains, WindowInstanceIdContext } from "../World3dHost/🟦️component.tsx";
import { createDemandFrameScheduler, createFlowSession, createGraphSession, isFlowGraphScene, type FlowWasmSession } from "../WasmSessionLoader/🟦️component.tsx";
import { useAppKeybindingsByActionId, useMapContextMenuSpecs } from "../ShellHost/🟦️component.tsx";
import { useUIFindSafe } from "../ShellSearch/🟦️component.tsx";
// #endregion 🔌️Adapters

//#region 🔖️NodeGraphHost
//#region Types
type WorkflowNodeData = {
  readonly label: string;
  readonly inputs: readonly NodeGraphPortRecord[];
  readonly outputs: readonly NodeGraphPortRecord[];
  readonly width: number;
  readonly height: number;
};

type GraphContextMenuItem = ContextMenuItemSpec;

type FrameworkGraphSession = GraphWasmSession & {
  syncFromSceneJson?(json: string): void;
  syncFromScenePack?(bytes: Uint8Array): void;
  pointerDownScreen(sx: number, sy: number, button: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean): void;
  pointerMoveScreen(sx: number, sy: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean): void;
  pointerUpScreen(sx: number, sy: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean): void;
  wheelScreen(sx: number, sy: number, deltaX: number, deltaY: number, zoomGesture: boolean): void;
  labelOverlayPaintStateJson(): string;
  sliderOverlayStateJson(): string;
  selectionUnionBoundsScreenJson(): string;
  selectionPreviewPointsJson(): string;
  selectionPreviewCrossing(): boolean;
  selectionPreviewMethod?(): string;
  selectedNodeIdsJson(): string;
  hoveredNodeId(): string | null | undefined;
  hoveredChannelJson(): string;
  cameraJson(): string;
  takePendingOpenInstanceId(): string | null | undefined;
  pickTargetsAtScreenJson(sx: number, sy: number): string;
  /** 🎯️ Screen-space geometry for a live entity (`domain`/`id` in the pick-target grammar) — powers
   * introduction-demonstration semantic targeting. */
  entityScreenJson?(domain: string, id: string): string;
  setHover?(widgetId: string | null): void;
  setHoverChannel?(widgetId: string | null, port?: string | null): void;
  alignSelection?(mode: string): void;
  fixtureJson?(): string;
  setCanvasThemeJson?(json: string): void;
};
//#endregion Types

//#region Viewport
export function nodeGraphViewportActionArgs(cameraJson: string): { readonly viewportJson: string } {
  return { viewportJson: cameraJson };
}
//#endregion Viewport

//#region Parsing
const DEFAULT_NODE_GRAPH_VIEWPORT: NodeGraphViewport = { x: 0, y: 0, zoom: 1 };

/** @emoji 🔎️ Resolves a flow fixture widget id to the workflow instance id it previews, used to open an app instance without depending on plugin-side selection state. */
export function resolveFixtureWidgetInstanceId(fixtureJson: string | undefined, widgetId: string | undefined | null): string | undefined {
  if (!fixtureJson || !widgetId) return undefined;
  try {
    const fixture = JSON.parse(fixtureJson) as {
      readonly widgets?: readonly { readonly id?: string; readonly params?: { readonly instanceId?: string } }[];
    };
    return fixture.widgets?.find((widget) => widget.id === widgetId)?.params?.instanceId;
  } catch {
    return undefined;
  }
}

export interface CatalogueAppDragPayload {
  readonly pluginId: string;
  readonly appId: string;
  readonly label?: string;
}

/** @emoji 🎯️ Parses a catalogue drag payload; returns null for non-catalogue-app payloads (garbage/legacy descriptors). */
export function parseCatalogueAppDragPayload(raw: string): CatalogueAppDragPayload | null {
  try {
    const parsed = JSON.parse(raw) as { readonly pluginId?: string; readonly appId?: string; readonly label?: string };
    if (!parsed.pluginId || !parsed.appId) return null;
    return { pluginId: parsed.pluginId, appId: parsed.appId, label: parsed.label };
  } catch {
    return null;
  }
}

/** @emoji 👻️ Builds the ghost widget descriptor shown while a catalogue app is dragged over the workflow. */
export function catalogueGhostDescriptorJson(payload: CatalogueAppDragPayload): string {
  return JSON.stringify({ kind: "neuron", neuronKind: payload.label ?? payload.appId });
}

//#region FlowCatalogueSpotlight
export type FlowCatalogueItem = {
  readonly kind: string;
  readonly neuronKind?: string;
  readonly action?: string;
  readonly format?: string;
  readonly name: string;
  readonly abbreviation: string;
  readonly icon: string;
  readonly summary: string;
};

export type FlowCatalogueGroup = {
  readonly id: string;
  readonly title: string;
  readonly items?: readonly FlowCatalogueItem[];
  readonly groups?: readonly FlowCatalogueGroup[];
};

export type FlowCatalogueSection = {
  readonly id: string;
  readonly title: string;
  readonly items?: readonly FlowCatalogueItem[];
  readonly groups?: readonly FlowCatalogueGroup[];
};

/** @emoji 🧩️ Builds an addWidget/setGhostWidget descriptor JSON from a catalogue row. */
export function flowCatalogueItemDescriptor(item: FlowCatalogueItem): string {
  const descriptor: Record<string, string> = { kind: item.kind };
  if (item.neuronKind) descriptor.neuronKind = item.neuronKind;
  if (item.action) descriptor.action = item.action;
  if (item.format) descriptor.format = item.format;
  return JSON.stringify(descriptor);
}

function flattenFlowCatalogueItems(sections: readonly FlowCatalogueSection[]): FlowCatalogueItem[] {
  const out: FlowCatalogueItem[] = [];
  const walkGroup = (group: FlowCatalogueGroup) => {
    for (const item of group.items ?? []) out.push(item);
    for (const child of group.groups ?? []) walkGroup(child);
  };
  for (const section of sections) {
    for (const item of section.items ?? []) out.push(item);
    for (const group of section.groups ?? []) walkGroup(group);
  }
  return out;
}

function scoreFlowCatalogueItem(item: FlowCatalogueItem, query: string, sectionTitle?: string): number | null {
  if (!query) return item.kind === "neuron" ? 1 : 2;
  const q = query.toLowerCase();
  const name = item.name.toLowerCase();
  const neuron = (item.neuronKind ?? "").toLowerCase();
  const abbr = item.abbreviation.toLowerCase();
  const summary = item.summary.toLowerCase();
  const section = (sectionTitle ?? "").toLowerCase();
  if (name === q || neuron === q || abbr === q) return 0;
  if (name.startsWith(q) || neuron.startsWith(q) || abbr.startsWith(q) || section.startsWith(q)) return 1;
  if (name.includes(q) || neuron.includes(q) || abbr.includes(q) || summary.includes(q) || section.includes(q)) return 2;
  return null;
}

/** @emoji 🔎️ Ranks catalogue items for the double-click spotlight (exact/prefix/substring; neurons first). */
export function flowRankCatalogueSuggestions(sections: readonly FlowCatalogueSection[], query: string): FlowCatalogueItem[] {
  const scored: { item: FlowCatalogueItem; score: number }[] = [];
  const walkGroup = (group: FlowCatalogueGroup, sectionTitle: string) => {
    for (const item of group.items ?? []) {
      const score = scoreFlowCatalogueItem(item, query.trim(), sectionTitle);
      if (score != null) scored.push({ item, score });
    }
    for (const child of group.groups ?? []) walkGroup(child, sectionTitle);
  };
  for (const section of sections) {
    for (const item of section.items ?? []) {
      const score = scoreFlowCatalogueItem(item, query.trim(), section.title);
      if (score != null) scored.push({ item, score });
    }
    for (const group of section.groups ?? []) walkGroup(group, section.title);
  }
  scored.sort((a, b) => {
      if (a.score !== b.score) return a.score - b.score;
      const aNeuron = a.item.kind === "neuron" ? 0 : 1;
      const bNeuron = b.item.kind === "neuron" ? 0 : 1;
      if (aNeuron !== bNeuron) return aNeuron - bNeuron;
      return a.item.name.localeCompare(b.item.name);
    });
  return scored.map((row) => row.item);
}

/** @emoji 🔍️ Scroll container classes for expanded flow spotlight suggestions. */
export function flowSpotlightSuggestionListScrollClass(expanded: boolean): string {
  return cn("min-h-0 overscroll-contain", expanded ? "overflow-y-auto max-h-[min(24rem,70vh)]" : "overflow-hidden");
}

function parseFlowCatalogueSections(json: string | undefined | null): FlowCatalogueSection[] {
  if (!json) return [];
  try {
    const parsed = JSON.parse(json) as unknown;
    return Array.isArray(parsed) ? (parsed as FlowCatalogueSection[]) : [];
  } catch {
    return [];
  }
}

type FlowSpotlightState = {
  readonly screen: { readonly x: number; readonly y: number };
  readonly world: { readonly x: number; readonly y: number };
};

/** @emoji 🔦️ Inline catalogue search opened by double-clicking empty flow canvas; hover/top match drives highlighted ghost preview. */
function FlowSpotlight({
  state,
  sections,
  onPreview,
  onCommit,
  onClose,
}: {
  readonly state: FlowSpotlightState;
  readonly sections: readonly FlowCatalogueSection[];
  readonly onPreview: (item: FlowCatalogueItem | null) => void;
  readonly onCommit: (item: FlowCatalogueItem) => void;
  readonly onClose: () => void;
}) {
  const rootRef = useRef<HTMLDivElement>(null);
  const activeItemRef = useRef<HTMLButtonElement | null>(null);
  const [query, setQuery] = useState("");
  const [expanded, setExpanded] = useState(false);
  const [activeIndex, setActiveIndex] = useState(0);
  const [previewArmed, setPreviewArmed] = useState(false);
  const suggestions = useMemo(() => flowRankCatalogueSuggestions(sections, query), [query, sections]);
  const visible = expanded ? suggestions : suggestions.slice(0, 1);
  const hasMore = suggestions.length > 1;
  const activeItem = suggestions[activeIndex] ?? null;
  const shouldPreview = query.trim().length > 0 || previewArmed;
  const typeToAddLabel = useLabel("ui.flowSpotlight.typeToAdd");
  const collapseSuggestionsLabel = useLabel("ui.flowSpotlight.collapseSuggestions");
  const showAllSuggestionsLabel = useLabel("ui.flowSpotlight.showAllSuggestions");
  const noMatchesLabel = useLabel("ui.windowSearch.noMatches");

  useEffect(() => {
    setActiveIndex(0);
    setExpanded(false);
  }, [query]);

  useEffect(() => {
    onPreview(shouldPreview ? activeItem : null);
  }, [activeItem, onPreview, shouldPreview]);

  useEffect(() => {
    if (!expanded) return;
    activeItemRef.current?.scrollIntoView({ block: "nearest" });
  }, [activeIndex, expanded]);

  useEffect(() => {
    const onPointerDown = (event: PointerEvent) => {
      const root = rootRef.current;
      if (root?.contains(event.target as globalThis.Node)) return;
      onClose();
    };
    window.addEventListener("pointerdown", onPointerDown, true);
    return () => window.removeEventListener("pointerdown", onPointerDown, true);
  }, [onClose]);

  return (
    <div
      ref={rootRef}
      className={cn("pointer-events-auto absolute z-60 flex min-h-0 w-layout-floating-menu-sm flex-col overflow-hidden", floatingMenuSurfaceClass)}
      data-level="menu"
      style={{ left: state.screen.x, top: state.screen.y }}
      onPointerDown={(event) => event.stopPropagation()}
      onDoubleClick={(event) => event.stopPropagation()}
      onWheel={(event) => event.stopPropagation()}
    >
      <div className={cn("flex shrink-0 items-center gap-single px-single py-half", borderNormalBottomClass)}>
        <Input
          autoFocus
          value={query}
          placeholder={typeToAddLabel}
          className="min-w-0 flex-1 border-0 bg-transparent p-0 text-xs shadow-none focus-visible:ring-0"
          onChange={(event) => setQuery(event.target.value)}
          onKeyDown={(event) => {
            if (event.key === "Escape") {
              event.preventDefault();
              onClose();
              return;
            }
            if (event.key === "ArrowDown") {
              event.preventDefault();
              if (suggestions.length === 0) return;
              setPreviewArmed(true);
              setActiveIndex((index) => Math.min(index + 1, suggestions.length - 1));
              if (!expanded && hasMore) setExpanded(true);
              return;
            }
            if (event.key === "ArrowUp") {
              event.preventDefault();
              if (suggestions.length === 0) return;
              setPreviewArmed(true);
              setActiveIndex((index) => Math.max(index - 1, 0));
              return;
            }
            if (event.key === "Enter") {
              event.preventDefault();
              if (activeItem) onCommit(activeItem);
            }
          }}
        />
        {hasMore ? (
          <button
            type="button"
            aria-label={expanded ? collapseSuggestionsLabel : showAllSuggestionsLabel}
            className="text-muted-foreground hover:bg-muted/40 hover:text-foreground shrink-0 rounded px-half text-2xs"
            onClick={() => setExpanded((value) => !value)}
          >
            {expanded ? "▴️" : "▾️"}
          </button>
        ) : null}
      </div>
      <div
        className={flowSpotlightSuggestionListScrollClass(expanded && hasMore)}
        role="listbox"
        onWheel={(event) => event.stopPropagation()}
      >
        {visible.length === 0 ? (
          <div className="text-muted-foreground px-single py-half text-2xs">{noMatchesLabel}</div>
        ) : (
          visible.map((item, index) => {
            const globalIndex = expanded ? index : 0;
            const active = globalIndex === activeIndex && shouldPreview;
            const key = `${item.kind}:${item.neuronKind ?? item.action ?? item.format ?? item.name}`;
            return (
              <button
                key={key}
                ref={active ? activeItemRef : undefined}
                type="button"
                role="option"
                aria-selected={active}
                className={cn(floatingMenuItemClass, active && "bg-active-base text-emphasized")}
                onPointerEnter={() => {
                  setPreviewArmed(true);
                  setActiveIndex(globalIndex);
                }}
                onPointerDown={(event) => {
                  event.preventDefault();
                  event.stopPropagation();
                  onCommit(item);
                }}
              >
                <span className="truncate">{item.name}</span>
                {item.neuronKind ? <span className="text-muted-foreground truncate text-2xs">{item.neuronKind}</span> : null}
              </button>
            );
          })
        )}
      </div>
    </div>
  );
}
//#endregion FlowCatalogueSpotlight

function portLabel(port: NodeGraphPortRecord): string {
  if (port.label) return port.label;
  const segments = port.id.split("@");
  return segments[segments.length - 1] ?? port.id;
}

// 🩹️ `port.id` is the wire-level `nodeId@portId` key (see `NodeGraphPortRecord`), but React Flow's
// `Handle id` must match `sourceHandle`/`targetHandle`, which carry the bare port id (`NodeGraphEdgeRecord.
// sourcePortId`/`targetPortId`) — strip the node-id prefix here so per-port anchoring and onConnect's
// round-trip back to `sourcePortId`/`targetPortId` both resolve against the same bare id.
function portHandleId(port: NodeGraphPortRecord): string {
  const segments = port.id.split("@");
  return segments[segments.length - 1] ?? port.id;
}

function workflowNodesToDiagramNodes(records: readonly NodeGraphNodeRecord[]): Node<WorkflowNodeData>[] {
  return records.map((record) => ({
    id: record.id,
    type: "workflow",
    position: { x: record.x, y: record.y },
    data: {
      label: record.label?.trim() || record.instanceId || record.id,
      inputs: record.inputs,
      outputs: record.outputs,
      width: record.width,
      height: record.height,
    },
  }));
}

function workflowEdgesToDiagramEdges(records: readonly NodeGraphEdgeRecord[]): Edge[] {
  return records.map((record) => ({
    id: record.id,
    source: record.sourceNodeId,
    target: record.targetNodeId,
    sourceHandle: record.sourcePortId,
    targetHandle: record.targetPortId,
  }));
}
//#endregion Parsing

//#region Keyboard
function isEditableGraphKeyTarget(target: EventTarget | null): boolean {
  if (!(target instanceof HTMLElement)) return false;
  const tag = target.tagName;
  if (tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT") return true;
  if (target.isContentEditable) return true;
  return target.closest("[contenteditable='true'], [role='textbox']") != null;
}

function handleGraphKeyboard(event: KeyboardEvent<HTMLDivElement>, editable: boolean, _parsedNodes: readonly NodeGraphNodeRecord[], dispatch: (action: string, args?: Record<string, unknown>) => void) {
  if (!editable || isEditableGraphKeyTarget(event.target)) return;
  if (event.key === "Escape") {
    event.preventDefault();
    dispatch("setMediaNodeSelection", { nodeIds: [] });
  }
}
//#endregion Keyboard

//#region DiagramNode
function WorkflowDiagramNode({ data }: NodeProps<WorkflowNodeData>) {
  const inputCount = Math.max(data.inputs.length, 1);
  const outputCount = Math.max(data.outputs.length, 1);
  const rowCount = Math.max(inputCount, outputCount);
  const rowHeight = 18;
  const bodyHeight = Math.max(data.height, 56 + rowCount * rowHeight);
  return (
    <div className="rounded border border-border bg-background text-foreground shadow-sm" style={{ width: data.width, minHeight: bodyHeight }}>
      <div className="border-b border-border px-2 py-1 text-xs font-medium">{data.label}</div>
      <div className="relative px-2 py-1 text-[10px] leading-[18px]">
        {Array.from({ length: rowCount }, (_, rowIndex) => {
          const input = data.inputs[rowIndex];
          const output = data.outputs[rowIndex];
          const top = 8 + rowIndex * rowHeight;
          return (
            <div key={`${input?.id ?? "in"}:${output?.id ?? "out"}:${rowIndex}`} className="relative h-[18px]">
              {input ? (
                <>
                  <Handle id={portHandleId(input)} type="target" position={Position.Left} className="!size-2 !border-panel !bg-foreground" style={{ top }} />
                  <span className="pl-3 text-muted-foreground">{portLabel(input)}</span>
                </>
              ) : null}
              {output ? (
                <>
                  <Handle id={portHandleId(output)} type="source" position={Position.Right} className="!size-2 !border-panel !bg-foreground" style={{ top }} />
                  <span className="absolute right-3 top-0 text-right text-muted-foreground">{portLabel(output)}</span>
                </>
              ) : null}
            </div>
          );
        })}
      </div>
    </div>
  );
}

const workflowNodeTypes: NodeTypes = { workflow: WorkflowDiagramNode };
//#endregion DiagramNode

//#region WasmGraphSurface
function WasmGraphSurface({
  scene,
  surfaceId,
  controllerId,
  editable,
  requestContextMenu,
  onAction,
}: {
  readonly scene: NodeGraphScene;
  readonly surfaceId: string;
  readonly controllerId: string;
  readonly editable: boolean;
  readonly requestContextMenu?: (request: PluginContextMenuRequest) => Promise<readonly ContextMenuItemSpec[]>;
  readonly onAction: (action: ActionDescriptor) => void;
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

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({ controllerId, action, args: { surfaceId, ...args } });
    },
    [controllerId, onAction, surfaceId],
  );

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
      });
    } catch {
      /* gpu not ready */
    }
    setSelectionBounds(parseDagSelectionUnionBoundsScreen(session.selectionUnionBoundsScreenJson()));
    setMarquee(computeDagMarqueeOverlay(session.selectionPreviewPointsJson(), session.selectionPreviewCrossing(), session.selectionPreviewMethod?.() ?? "rectangle"));
    try {
      const nextSliderJson = session.sliderOverlayStateJson();
      setSliderStateJson((prev) => (prev === nextSliderJson ? prev : nextSliderJson));
    } catch {
      /* session not ready */
    }
    setOverlaySize((prev) => (prev.w === rect.width && prev.h === rect.height ? prev : { w: rect.width, h: rect.height }));
  }, []);

  useEffect(() => {
    try {
      sessionRef.current?.syncFromScenePack?.(scenePack);
      paintOverlays();
    } catch (error) {
      console.warn("[DEBUG] WasmGraphSurface sync failed", error instanceof Error ? error.message : String(error));
    }
  }, [scenePack, paintOverlays]);

  const onSessionReady = useCallback(
    (session: GraphWasmSession) => {
      sessionRef.current = session as FrameworkGraphSession;
      syncSessionCanvasTheme(sessionRef.current);
      try {
        sessionRef.current.syncFromScenePack?.(scenePack);
        paintOverlays();
      } catch (error) {
        console.warn("[DEBUG] WasmGraphSurface ready sync failed", error instanceof Error ? error.message : String(error));
      }
    },
    [scenePack, paintOverlays],
  );

  const wasmGraphSurfaceShellScope = useShellScopeOptional();
  useCanvasAppearanceSync(
    () => {
      syncSessionCanvasTheme(sessionRef.current);
      try {
        sessionRef.current?.renderFrame();
      } catch {
        /* gpu not ready */
      }
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
      wheelScreen: () => {},
      labelOverlayPaintStateJson: () => '{"labels":[]}',
      sliderOverlayStateJson: () => "{}",
      selectionUnionBoundsScreenJson: () => "{}",
      selectionPreviewPointsJson: () => "[]",
      selectionPreviewCrossing: () => false,
      selectionPreviewMethod: () => "rectangle",
      selectedNodeIdsJson: () => "[]",
      hoveredNodeId: () => null,
      hoveredChannelJson: () => "{}",
      cameraJson: () => JSON.stringify(scene.viewport ?? DEFAULT_NODE_GRAPH_VIEWPORT),
      pickTargetsAtScreenJson: () => "[]",
      setHover: () => {},
      setHoverChannel: () => {},
      alignSelection: () => {},
      fixtureJson: () => "{}",
      takePendingOpenInstanceId: () => null,
    } satisfies FrameworkGraphSession;
  }, [scene.viewport, wasmSession]);

  const emitInteractionState = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    try {
      const nodeIds = JSON.parse(session.selectedNodeIdsJson()) as string[];
      dispatch(nodeGraphActions.select, { nodeIds });
      const hovered = session.hoveredNodeId();
      dispatch(nodeGraphActions.hover, { hoverJson: hovered ? JSON.stringify({ nodeId: hovered }) : null });
      dispatch(nodeGraphActions.viewport, nodeGraphViewportActionArgs(session.cameraJson()));
      const openId = session.takePendingOpenInstanceId?.();
      if (openId) dispatch("openInstance", { instanceId: openId });
    } catch {
      /* session not ready */
    }
    paintOverlays();
  }, [dispatch, paintOverlays]);

  const commitGraphFixture = useCallback(() => {
    const session = sessionRef.current;
    if (!session?.fixtureJson) return;
    try {
      const fixtureJson = session.fixtureJson();
      dispatch(nodeGraphActions.edit, { operations: [{ operation: "setFixture", fixtureJson }] });
    } catch {
      /* session not ready */
    }
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
      if (!target) {
        session.setHover?.(null);
      } else if (target.portId) {
        session.setHoverChannel?.(target.id, target.portId);
      } else {
        session.setHover?.(target.id);
      }
      try {
        const hovered = session.hoveredNodeId();
        dispatch(nodeGraphActions.hover, { hoverJson: hovered ? JSON.stringify({ nodeId: hovered }) : null });
      } catch {
        /* session not ready */
      }
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
              menu: { id: "nodeGraph" },
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
      <GraphWasmCanvas className="absolute inset-0" sessionFactory={sessionFactory} onSessionReady={onSessionReady} enablePointer={false} />
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
        className="absolute inset-0 z-30"
        onPointerDown={(event) => {
          if (!editable) return;
          if (event.button === 2) return;
          const session = sessionRef.current;
          if (!session?.pointerDownScreen) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const client = { x: event.clientX, y: event.clientY };
          pickInteraction.onCanvasPointerDown(client);
          session.pointerDownScreen(event.clientX - rect.left, event.clientY - rect.top, event.button, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey);
          session.renderFrame();
          paintOverlays();
        }}
        onPointerMove={(event) => {
          const session = sessionRef.current;
          if (!session?.pointerMoveScreen) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const client = { x: event.clientX, y: event.clientY };
          pickInteraction.onCanvasPointerMove(client);
          session.pointerMoveScreen(event.clientX - rect.left, event.clientY - rect.top, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey);
          session.renderFrame();
          paintOverlays();
        }}
        onPointerUp={(event) => {
          const session = sessionRef.current;
          if (!session?.pointerUpScreen) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const client = { x: event.clientX, y: event.clientY };
          pickInteraction.onCanvasPointerUp(client, { shift: event.shiftKey, ctrlOrMeta: event.metaKey || event.ctrlKey, alt: event.altKey });
          session.pointerUpScreen(event.clientX - rect.left, event.clientY - rect.top, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey);
          session.renderFrame();
          emitInteractionState();
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
            commitGraphFixture();
            session.renderFrame();
            emitInteractionState();
          }}
        />
      ) : null}
      <GraphSliderOverlays stateJson={sliderStateJson} logicalW={overlaySize.w} logicalH={overlaySize.h} editable={editable} onSliderChange={(widgetId, value) => dispatch(nodeGraphActions.edit, { operator: "setSlider", widgetId, value })} />
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

//#region DiagramFallback
function DiagramGraphFallback({
  scene,
  node,
  editable,
  parsedNodes,
  parsedEdges,
  findItems,
  requestContextMenu,
  onAction,
}: {
  readonly scene: NodeGraphScene;
  readonly node: UiComponentSceneNode;
  readonly editable: boolean;
  readonly parsedNodes: readonly NodeGraphNodeRecord[];
  readonly parsedEdges: readonly NodeGraphEdgeRecord[];
  readonly findItems: readonly NodeGraphFindItem[];
  readonly requestContextMenu?: (request: PluginContextMenuRequest) => Promise<readonly ContextMenuItemSpec[]>;
  readonly onAction: (action: ActionDescriptor) => void;
}) {
  const viewport = scene.viewport ?? DEFAULT_NODE_GRAPH_VIEWPORT;
  const initialNodes = useMemo(() => workflowNodesToDiagramNodes(parsedNodes), [parsedNodes]);
  const initialEdges = useMemo(() => workflowEdgesToDiagramEdges(parsedEdges), [parsedEdges]);
  const [nodes, setNodes] = useState(initialNodes);
  const [edges, setEdges] = useState(initialEdges);
  useEffect(() => {
    setNodes(initialNodes);
    setEdges(initialEdges);
  }, [initialNodes, initialEdges]);

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({ controllerId: node.controllerId, action, args: { surfaceId: node.surfaceId, ...args } });
    },
    [node.controllerId, node.surfaceId, onAction],
  );

  const mapContextMenu = useMapContextMenuSpecs(dispatch);
  const shellContextMenuFallback = useShellContextMenuFallback();

  const containerRef = useRef<HTMLDivElement>(null);
  const [contextMenu, setContextMenu] = useState<(SurfaceContextMenuResult & { readonly x: number; readonly y: number }) | null>(null);
  const contextMenuTitleLabel = useLabel(contextMenu?.titleKey ?? "ui.surfaceContextMenu.node");

  return (
    <div
      ref={containerRef}
      className={cn("relative h-full w-full", surfaceClass)}
      data-level="base"
      onDragOver={(event) => {
        if (editable && event.dataTransfer.types.includes(CATALOGUE_DRAG_MIME)) event.preventDefault();
      }}
      onDrop={(event: DragEvent<HTMLDivElement>) => {
        if (!editable) return;
        event.preventDefault();
        const raw = event.dataTransfer.getData(CATALOGUE_DRAG_MIME);
        if (!raw) return;
        let payload: { readonly pluginId?: string; readonly appId?: string };
        try {
          payload = JSON.parse(raw) as { readonly pluginId?: string; readonly appId?: string };
        } catch {
          return;
        }
        if (!payload.pluginId || !payload.appId) return;
        const rect = containerRef.current?.getBoundingClientRect();
        if (!rect) return;
        const x = (event.clientX - rect.left - viewport.x) / viewport.zoom;
        const y = (event.clientY - rect.top - viewport.y) / viewport.zoom;
        dispatch("spawnApp", { pluginId: payload.pluginId, appId: payload.appId, x, y });
      }}
      onContextMenu={(event) => {
        if (!editable || !requestContextMenu) return;
        event.preventDefault();
        event.stopPropagation();
        void (async () => {
          const menu = await openSurfaceContextMenu(
            requestContextMenu,
            {
              menu: { id: "nodeGraph" },
              surface: { surfaceId: node.surfaceId, kind: "nodeGraph", hits: [], selection: [] },
              point: { x: event.clientX, y: event.clientY },
            },
            mapContextMenu,
            shellContextMenuFallback,
          );
          setContextMenu({ x: event.clientX, y: event.clientY, ...menu });
        })();
      }}
    >
      <Diagram
        className="h-full w-full"
        nodeTypes={workflowNodeTypes}
        nodes={nodes}
        edges={edges}
        fitView={false}
        defaultViewport={viewport}
        minZoom={0.05}
        maxZoom={32}
        panOnDrag={[0, 1]}
        selectionOnDrag
        elementsSelectable
        nodesDraggable={editable}
        nodesConnectable={editable}
        edgesReconnectable={editable}
        onNodesChange={(nextNodes) => setNodes(nextNodes as Node<WorkflowNodeData>[])}
        onEdgesChange={(nextEdges) => setEdges(nextEdges)}
        onNodeDragStop={
          editable
            ? (_event, draggedNode) => {
                dispatch(nodeGraphActions.edit, {
                  operations: [{ operation: "move", nodeId: draggedNode.id, x: draggedNode.position.x, y: draggedNode.position.y }],
                });
              }
            : undefined
        }
        onConnect={
          editable
            ? (connection) => {
                if (!connection.source || !connection.target || !connection.sourceHandle || !connection.targetHandle) return;
                dispatch(nodeGraphActions.edit, {
                  operations: [
                    {
                      operation: "connect",
                      sourceNodeId: connection.source,
                      sourcePortId: connection.sourceHandle,
                      targetNodeId: connection.target,
                      targetPortId: connection.targetHandle,
                    },
                  ],
                });
              }
            : undefined
        }
        onNodeClick={(_event, clickedNode) => {
          const record = parsedNodes.find((entry) => entry.id === clickedNode.id);
          if (record?.instanceId) dispatch("selectInstance", { instanceId: record.instanceId });
          dispatch(nodeGraphActions.select, { nodeIds: [clickedNode.id] });
        }}
        onNodeDoubleClick={(_event, clickedNode) => {
          const record = parsedNodes.find((entry) => entry.id === clickedNode.id);
          if (record?.instanceId) dispatch("openInstance", { instanceId: record.instanceId });
        }}
        onSelectionChange={(selection) => {
          const nodeIds = selection.nodes.map((entry) => entry.id);
          dispatch(nodeGraphActions.select, { nodeIds });
        }}
      />
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
//#endregion DiagramFallback

//#region NodeGraphHost
//#region Helpers
// 🚪️ Exported (unlike the rest of this Helpers subregion) — `TextEditorHost` in the sibling `TextEditor`
// element also needs the SSR-safe client-mount gate; shared here rather than duplicated.
export const useClient = () => {
  const [client, setClient] = useState(false);
  useEffect(() => setClient(true), []);
  return client;
};

function PresencePeersOverlay({ peers }: { readonly peers: readonly PresencePeer[] }) {
  if (peers.length === 0) return null;
  return (
    <div className={cn("pointer-events-none absolute right-2 top-2 z-panel flex max-w-[14rem] flex-col gap-1 rounded border border-border/60 px-2 py-1 text-xs shadow-sm", glassClass)} data-level="pane">
      {peers.map((peer) => (
        <div key={peer.clientId} className="flex items-center justify-between gap-2 text-muted-foreground">
          <span className="truncate font-medium text-foreground">{peer.name}</span>
          <span>{peer.selectionCount} selected</span>
        </div>
      ))}
    </div>
  );
}
//#endregion Helpers

//#region Component
export function NodeGraphHost({ node, onAction, requestContextMenu }: ComponentSceneHostProps) {
  const scene = node.nodeGraph;
  const editable = scene?.editable ?? true;
  const parsedNodes = useMemo(() => scene?.nodes ?? [], [scene?.nodes]);
  const parsedEdges = useMemo(() => scene?.edges ?? [], [scene?.edges]);
  const findItems = useMemo(() => scene?.findItems ?? [], [scene?.findItems]);
  const presencePeers = useMemo(() => parseJsonArray<PresencePeer>(scene?.presencePeersJson), [scene?.presencePeersJson]);
  const isClient = useClient();
  const emptySceneLabel = useLabel("ui.host.emptyScene");

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({ controllerId: node.controllerId, action, args: { surfaceId: node.surfaceId, ...args } });
    },
    [node.controllerId, node.surfaceId, onAction],
  );

  const findContext = useUIFindSafe();
  const onFindItemRef = useRef<(itemId: string) => void>(() => {});
  onFindItemRef.current = (itemId: string) => {
    const mediaNode = parsedNodes.find((entry) => entry.instanceId === itemId);
    if (!mediaNode) return;
    dispatch(nodeGraphActions.select, { nodeIds: [mediaNode.id] });
    dispatch("selectInstance", { instanceId: mediaNode.instanceId! });
  };

  useEffect(() => {
    if (!findContext?.setFindItems || findItems.length === 0) return;
    findContext.setFindItems(findItems);
  }, [findContext?.setFindItems, findItems]);

  useEffect(() => {
    if (!findContext?.setOnFindItem || findItems.length === 0) return;
    findContext.setOnFindItem((itemId) => onFindItemRef.current(itemId));
    return () => findContext.setOnFindItem?.(undefined);
  }, [findContext?.setOnFindItem, findItems.length]);

  if (!scene) return <div className="semio-node-graph-empty">{emptySceneLabel}</div>;

  const useFlowEngine = isFlowGraphScene(scene.capabilitiesJson) || Boolean(scene.fixtureJson);

  return (
    <div
      className="semio-node-graph-host relative h-full min-h-0 w-full overflow-hidden"
      data-surface-id={node.surfaceId}
      data-status-json={scene.statusJson ?? undefined}
      data-fixture-json={scene.fixtureJson ?? undefined}
      tabIndex={editable ? 0 : undefined}
      onKeyDown={(event) => handleGraphKeyboard(event, editable, parsedNodes, dispatch)}
    >
      {isClient ? (
        useFlowEngine ? (
          <FlowGraphCanvasHost scene={scene} surfaceId={node.surfaceId} controllerId={node.controllerId} editable={editable} requestContextMenu={requestContextMenu} onAction={onAction} />
        ) : (
          <WasmGraphSurface scene={scene} surfaceId={node.surfaceId} controllerId={node.controllerId} editable={editable} requestContextMenu={requestContextMenu} onAction={onAction} />
        )
      ) : (
        <DiagramGraphFallback scene={scene} node={node} editable={editable} parsedNodes={parsedNodes} parsedEdges={parsedEdges} findItems={findItems} requestContextMenu={requestContextMenu} onAction={onAction} />
      )}
      <PresencePeersOverlay peers={presencePeers} />
    </div>
  );
}
//#endregion Component
//#endregion NodeGraphHost

//#region 🔖️graph-canvas-overlays

//#region DagOverlayTypes
export type DagLabelOverlayRow = {
  readonly id: string;
  readonly kind?: "port" | "node" | string;
  readonly text: string;
  readonly layout: "horizontal" | "vertical";
  readonly align?: "left" | "center" | "right";
  readonly x: number;
  readonly y: number;
  readonly nodeW: number;
  readonly nodeH: number;
  readonly fontScreenPx?: number;
  readonly maxScreenH?: number;
  readonly ghost?: boolean;
};

export type DagPreselectSnapshot = {
  readonly ids: readonly string[];
  readonly removedIds: readonly string[];
};

export type DagLabelOverlayInteraction = {
  readonly hoveredId: string | null;
  readonly selectedIds: readonly string[];
  readonly preselect: DagPreselectSnapshot;
  readonly dimmedIds?: readonly string[];
};

export type DagMarqueeOverlay = {
  readonly kind: "rect" | "lasso";
  readonly x?: number;
  readonly y?: number;
  readonly width?: number;
  readonly height?: number;
  readonly points?: readonly { readonly x: number; readonly y: number }[];
  readonly coverage?: "full" | "partial";
};

export type DagCameraState = { readonly x: number; readonly y: number; readonly zoom: number };

export type DagSliderOverlayRow = {
  readonly widgetId: string;
  readonly value: number;
  readonly min: number;
  readonly max: number;
  readonly step: number;
  readonly x: number;
  readonly y: number;
  readonly w: number;
  readonly h: number;
};

export type DagSelectionBounds = {
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
};
//#endregion DagOverlayTypes

//#region DagOverlayGeometry
export function parseDagCameraState(json: string): DagCameraState {
  try {
    const parsed = JSON.parse(json) as Partial<DagCameraState>;
    return { x: Number(parsed.x ?? 0), y: Number(parsed.y ?? 0), zoom: Number(parsed.zoom ?? 1) };
  } catch {
    return { x: 0, y: 0, zoom: 1 };
  }
}

export function dagWorldToScreen(camera: DagCameraState, width: number, height: number, wx: number, wy: number): { readonly x: number; readonly y: number } {
  const zoom = camera.zoom > 0 ? camera.zoom : 1;
  const cx = width * 0.5;
  const cy = height * 0.5;
  return { x: (wx - camera.x) * zoom + cx, y: (wy - camera.y) * zoom + cy };
}

export function dagScreenToWorld(camera: DagCameraState, width: number, height: number, sx: number, sy: number): { readonly x: number; readonly y: number } {
  const zoom = camera.zoom > 0 ? camera.zoom : 1;
  const cx = width * 0.5;
  const cy = height * 0.5;
  return { x: (sx - cx) / zoom + camera.x, y: (sy - cy) / zoom + camera.y };
}
//#endregion DagOverlayGeometry

//#region DagOverlayPaint
const DAG_LABEL_SCREEN_PX = 11;
const DAG_LABEL_FONT_FAMILY = "ui-sans-serif, system-ui, sans-serif";

export function parseDagNodeIdArray(json: string): string[] {
  try {
    const parsed = JSON.parse(json) as unknown;
    return Array.isArray(parsed) ? parsed.filter((value): value is string => typeof value === "string") : [];
  } catch {
    return [];
  }
}

export function parseDagPreselectJson(json: string): DagPreselectSnapshot {
  try {
    const parsed = JSON.parse(json) as { ids?: unknown; removedIds?: unknown };
    const ids = Array.isArray(parsed.ids) ? parsed.ids.filter((value): value is string => typeof value === "string") : [];
    const removedIds = Array.isArray(parsed.removedIds) ? parsed.removedIds.filter((value): value is string => typeof value === "string") : [];
    return { ids, removedIds };
  } catch {
    return { ids: [], removedIds: [] };
  }
}

export function dagElementInteractionChrome(selectionIds: Iterable<string>, preselection: DagPreselectSnapshot): { readonly selectedIds: Set<string>; readonly highlightedIds: Set<string> } {
  if (!preselection.ids.length && !preselection.removedIds.length) {
    return { selectedIds: new Set(selectionIds), highlightedIds: new Set() };
  }
  return { selectedIds: new Set(preselection.ids), highlightedIds: new Set(preselection.removedIds) };
}

export function parseDagLabelRows(stateJson: string): DagLabelOverlayRow[] {
  try {
    const parsed = JSON.parse(stateJson) as {
      readonly labels?: readonly Record<string, unknown>[];
      readonly rows?: readonly Record<string, unknown>[];
    };
    const raw = parsed.labels ?? parsed.rows ?? [];
    return raw
      .map((row) => {
        const text = typeof row.text === "string" ? row.text.trim() : "";
        if (!text) return null;
        const align = row.align === "left" || row.align === "right" || row.align === "center" ? row.align : undefined;
        return {
          id: String(row.id ?? ""),
          kind: typeof row.kind === "string" ? row.kind : undefined,
          text,
          layout: row.layout === "vertical" ? "vertical" : "horizontal",
          align,
          x: Number(row.x ?? 0),
          y: Number(row.y ?? 0),
          nodeW: Number(row.nodeW ?? row.width ?? 0),
          nodeH: Number(row.nodeH ?? row.height ?? 0),
          fontScreenPx: typeof row.fontScreenPx === "number" ? row.fontScreenPx : undefined,
          maxScreenH: typeof row.maxScreenH === "number" ? row.maxScreenH : undefined,
          ghost: row.ghost === true,
        } satisfies DagLabelOverlayRow;
      })
      .filter((row): row is DagLabelOverlayRow => row !== null);
  } catch {
    return [];
  }
}

function dagClampLabelFontPx(ctx: CanvasRenderingContext2D, text: string, targetPx: number, maxW: number, maxH: number): number {
  let px = Math.max(4, Math.round(targetPx));
  ctx.font = `${px}px ${DAG_LABEL_FONT_FAMILY}`;
  if (ctx.measureText(text).width <= maxW && px * 1.2 <= maxH) {
    return px;
  }
  let low = 4;
  let high = px;
  let best = 4;
  while (low <= high) {
    const mid = Math.floor((low + high) / 2);
    ctx.font = `${mid}px ${DAG_LABEL_FONT_FAMILY}`;
    const w = ctx.measureText(text).width;
    const h = mid * 1.2;
    if (w <= maxW && h <= maxH) {
      best = mid;
      low = mid + 1;
    } else {
      high = mid - 1;
    }
  }
  return best;
}

function dagClampPortLabelFontPx(ctx: CanvasRenderingContext2D, text: string, targetPx: number, maxW: number, maxH: number): number {
  let px = Math.max(8, Math.round(targetPx));
  ctx.font = `${px}px ${DAG_LABEL_FONT_FAMILY}`;
  if (ctx.measureText(text).width <= maxW && px * 1.25 <= maxH) {
    return px;
  }
  let low = 8;
  let high = px;
  let best = 8;
  while (low <= high) {
    const mid = Math.floor((low + high) / 2);
    ctx.font = `${mid}px ${DAG_LABEL_FONT_FAMILY}`;
    if (ctx.measureText(text).width <= maxW) {
      best = mid;
      low = mid + 1;
    } else {
      high = mid - 1;
    }
  }
  return best;
}

export function parseDagSliderOverlays(stateJson: string): readonly DagSliderOverlayRow[] {
  try {
    const parsed = JSON.parse(stateJson) as { readonly sliders?: DagSliderOverlayRow[] };
    return parsed.sliders ?? [];
  } catch {
    return [];
  }
}

/** @emoji 🎯️ The subset of `FlowWasmSession`/`FrameworkGraphSession` {@link dagIntroductionResolver} needs
 * — factored out because both session interfaces expose the same overlay/entity JSON shape and both host
 * components (`FlowGraphCanvasHost`, `WasmGraphSurface`) register the identical resolver logic. */
type DagIntroductionSession = {
  readonly labelOverlayPaintStateJson: () => string;
  readonly sliderOverlayStateJson: () => string;
  readonly entityScreenJson?: (domain: string, id: string) => string;
};

/** @emoji 🎯️ Builds the `IntroductionSurfaceResolver` for a dag-engine-backed graph surface. Reads the
 * session and container via refs (not React state) every call — cheap, and lets registration skip
 * re-running whenever the surface re-renders. `entity`'s `"slider"` domain resolves entirely from the
 * already-fetched `sliderOverlayStateJson()` (no Rust round trip); every other domain (`"node"`,
 * `"handle"`, `"edge"`) goes through `entityScreenJson`, added to the dag engine specifically for this. */
function dagIntroductionResolver(sessionRef: React.RefObject<DagIntroductionSession | null>, containerRef: React.RefObject<HTMLElement | null>): IntroductionSurfaceResolver {
  return {
    canvasPoint: (x, y) => {
      const session = sessionRef.current;
      const container = containerRef.current;
      if (!session || !container) return null;
      const rect = container.getBoundingClientRect();
      const camera = parseDagOverlayCamera(session.labelOverlayPaintStateJson());
      const screen = dagWorldToScreen(camera, rect.width, rect.height, x, y);
      return { x: rect.left + screen.x, y: rect.top + screen.y, visible: true };
    },
    entity: (domain, entityId): IntroductionResolvedGeometry | null => {
      const session = sessionRef.current;
      const container = containerRef.current;
      if (!session || !container) return null;
      const rect = container.getBoundingClientRect();
      if (domain === "slider") {
        const sliders = parseDagSliderOverlays(session.sliderOverlayStateJson());
        const slider = entityId === "*" ? sliders[0] : sliders.find((row) => row.widgetId === entityId);
        if (!slider) return null;
        const camera = parseDagOverlayCamera(session.labelOverlayPaintStateJson());
        const anchor = dagWorldToScreen(camera, rect.width, rect.height, slider.x, slider.y);
        return {
          point: { x: rect.left + anchor.x, y: rect.top + anchor.y },
          rect: { x: rect.left + anchor.x - slider.w / 2, y: rect.top + anchor.y - slider.h / 2, width: slider.w, height: slider.h },
          domain: { min: slider.min, max: slider.max, axis: "x" },
          visible: true,
        };
      }
      if (!session.entityScreenJson) return null;
      try {
        const geometry = JSON.parse(session.entityScreenJson(domain, entityId)) as {
          readonly visible: boolean;
          readonly x?: number;
          readonly y?: number;
          readonly rect?: readonly [number, number, number, number];
          readonly polyline?: readonly (readonly [number, number])[];
        };
        if (!geometry.visible || geometry.x === undefined || geometry.y === undefined) return null;
        return {
          point: { x: rect.left + geometry.x, y: rect.top + geometry.y },
          rect: geometry.rect ? { x: rect.left + geometry.rect[0], y: rect.top + geometry.rect[1], width: geometry.rect[2], height: geometry.rect[3] } : undefined,
          polyline: geometry.polyline?.map(([px, py]) => ({ x: rect.left + px, y: rect.top + py })),
          visible: true,
        };
      } catch {
        return null;
      }
    },
  };
}

export function parseDagOverlayCamera(stateJson: string): DagCameraState {
  try {
    const parsed = JSON.parse(stateJson) as { readonly camera?: DagCameraState; readonly width?: number; readonly height?: number };
    return parseDagCameraState(JSON.stringify(parsed.camera ?? {}));
  } catch {
    return { x: 0, y: 0, zoom: 1 };
  }
}
export function dagOverlayLabelFill(nodeId: string, ghost: boolean, hoveredId: string | null, chrome: { readonly selectedIds: Set<string>; readonly highlightedIds: Set<string> }, dimmedIds: readonly string[] = []): string {
  if (ghost) return "var(--color-secondary)";
  if (dimmedIds.includes(nodeId)) return "var(--color-border)";
  if (chrome.selectedIds.has(nodeId)) return "var(--color-foreground)";
  if (chrome.highlightedIds.has(nodeId)) return "var(--color-secondary)";
  if (hoveredId === nodeId) return "var(--color-foreground)";
  return "var(--color-muted-foreground)";
}

/** @emoji 🎨️ Resolves {@link dagOverlayLabelFill} to a Canvas2D-safe `#rrggbb` — CSS `var()` strings are not valid `fillStyle` values and silently paint as black. */
export function dagOverlayLabelFillHex(nodeId: string, ghost: boolean, hoveredId: string | null, chrome: { readonly selectedIds: Set<string>; readonly highlightedIds: Set<string> }, dimmedIds: readonly string[] = []): string {
  const expression = dagOverlayLabelFill(nodeId, ghost, hoveredId, chrome, dimmedIds);
  const appearanceFallback = currentStylingAppearanceName() === "dark" ? "light" : "dark";
  if (expression === "var(--color-secondary)") return resolveColorHex(expression, "secondary");
  if (expression === "var(--color-border)" || expression === "var(--color-muted-foreground)") return resolveColorHex(expression, "gray");
  return resolveColorHex(expression, appearanceFallback);
}

export function parseDagMinimapWidgetOccluder(stateJson: string): { readonly x: number; readonly y: number; readonly width: number; readonly height: number } | null {
  try {
    const parsed = JSON.parse(stateJson) as { readonly minimapWidget?: { readonly x?: number; readonly y?: number; readonly width?: number; readonly height?: number } };
    const rect = parsed.minimapWidget;
    if (rect?.x == null || rect?.y == null || rect?.width == null || rect?.height == null) return null;
    return { x: rect.x, y: rect.y, width: rect.width, height: rect.height };
  } catch {
    return null;
  }
}

export function parseDagMinimapWidgetCursor(stateJson: string): string | undefined {
  try {
    const parsed = JSON.parse(stateJson) as { readonly minimapWidget?: { readonly cursor?: string | null } };
    const cursor = parsed.minimapWidget?.cursor;
    return typeof cursor === "string" && cursor.length > 0 ? cursor : undefined;
  } catch {
    return undefined;
  }
}

export function paintDagLabelOverlays(stateJson: string, canvas: HTMLCanvasElement, logicalW: number, logicalH: number, dpr: number, interaction: DagLabelOverlayInteraction): void {
  let state: { readonly camera?: DagCameraState; readonly width?: number; readonly height?: number; readonly labels?: readonly DagLabelOverlayRow[] };
  try {
    state = JSON.parse(stateJson) as typeof state;
  } catch {
    return;
  }
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  const pixelW = Math.max(1, Math.round(logicalW * dpr));
  const pixelH = Math.max(1, Math.round(logicalH * dpr));
  if (canvas.width !== pixelW || canvas.height !== pixelH) {
    canvas.width = pixelW;
    canvas.height = pixelH;
  }
  canvas.style.width = `${logicalW}px`;
  canvas.style.height = `${logicalH}px`;
  ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
  ctx.clearRect(0, 0, logicalW, logicalH);
  const zoom = Math.max(0.05, Number(state.camera?.zoom) || 1);
  const camera = {
    x: Number(state.camera?.x) || 0,
    y: Number(state.camera?.y) || 0,
    zoom,
  };
  const viewportW = Number(state.width) || logicalW;
  const viewportH = Number(state.height) || logicalH;
  const chrome = dagElementInteractionChrome(interaction.selectedIds, interaction.preselect);
  const dimmedIds = interaction.dimmedIds ?? [];
  const rows = state.labels ?? parseDagLabelRows(stateJson);
  const occluder = parseDagMinimapWidgetOccluder(stateJson);
  const inset = 0.88;
  for (const row of rows) {
    const anchor = dagWorldToScreen(camera, viewportW, viewportH, row.x, row.y);
    if (occluder && anchor.x >= occluder.x && anchor.x <= occluder.x + occluder.width && anchor.y >= occluder.y && anchor.y <= occluder.y + occluder.height) {
      continue;
    }
    const isPort = row.kind === "port" || row.align === "left" || row.align === "right";
    const maxW = Math.max(4, Number(row.nodeW) * zoom * inset);
    const maxH = Math.max(4, isPort && Number.isFinite(Number(row.maxScreenH)) && Number(row.maxScreenH) > 0 ? Number(row.maxScreenH) : Number(row.nodeH) * zoom * inset);
    const fontScreenPx = Number(row.fontScreenPx);
    const targetPx = Number.isFinite(fontScreenPx) && fontScreenPx > 0 ? fontScreenPx : DAG_LABEL_SCREEN_PX;
    const fontPx = isPort ? dagClampPortLabelFontPx(ctx, row.text, targetPx, maxW, maxH) : dagClampLabelFontPx(ctx, row.text, targetPx, maxW, maxH);
    ctx.font = `${fontPx}px ${DAG_LABEL_FONT_FAMILY}`;
    ctx.fillStyle = dagOverlayLabelFillHex(row.id, row.ghost === true, interaction.hoveredId, chrome, dimmedIds);
    ctx.globalAlpha = row.ghost ? 0.85 : dimmedIds.includes(row.id) ? 0.5 : 1;
    if (row.layout === "vertical") {
      ctx.save();
      ctx.translate(anchor.x, anchor.y);
      ctx.rotate(-Math.PI / 2);
      ctx.textAlign = "center";
      ctx.textBaseline = "middle";
      ctx.fillText(row.text, 0, 0);
      ctx.restore();
    } else {
      const align = row.align === "left" || row.align === "right" ? row.align : "center";
      ctx.textAlign = align;
      ctx.textBaseline = "middle";
      ctx.fillText(row.text, anchor.x, anchor.y);
    }
    ctx.globalAlpha = 1;
  }
}

export function parseDagSelectionUnionBoundsScreen(json: string): DagSelectionBounds | null {
  try {
    const parsed = JSON.parse(json) as Partial<DagSelectionBounds>;
    if (parsed.x == null || parsed.y == null || parsed.width == null || parsed.height == null) return null;
    return { x: parsed.x, y: parsed.y, width: parsed.width, height: parsed.height };
  } catch {
    return null;
  }
}

/** @emoji 🧿️ Normalizes one selection-preview point from the rust `[[x,y],…]` wire format or `{x,y}` objects. */
function parseDagMarqueePoint(value: unknown): { readonly x: number; readonly y: number } | null {
  if (Array.isArray(value) && value.length >= 2 && typeof value[0] === "number" && typeof value[1] === "number" && Number.isFinite(value[0]) && Number.isFinite(value[1])) {
    return { x: value[0], y: value[1] };
  }
  if (value && typeof value === "object") {
    const x = (value as { readonly x?: unknown }).x;
    const y = (value as { readonly y?: unknown }).y;
    if (typeof x === "number" && typeof y === "number" && Number.isFinite(x) && Number.isFinite(y)) return { x, y };
  }
  return null;
}

/** @emoji 🧿️ Rectangle wire format is always four axis-aligned corners; anything else is a lasso path. */
function inferDagMarqueeMethod(points: readonly { readonly x: number; readonly y: number }[]): "lasso" | "rectangle" {
  if (points.length !== 4) return points.length >= 3 ? "lasso" : "rectangle";
  const xs = new Set(points.map((point) => point.x));
  const ys = new Set(points.map((point) => point.y));
  return xs.size === 2 && ys.size === 2 ? "rectangle" : "lasso";
}

/** @emoji 🧿️ Builds the shared `SelectionMarquee` overlay from board preview points (`[[x,y],…]` from rust). */
export function computeDagMarqueeOverlay(pointsJson: string, crossing: boolean, method?: string): DagMarqueeOverlay | null {
  let raw: unknown;
  try {
    raw = JSON.parse(pointsJson);
  } catch {
    return null;
  }
  if (!Array.isArray(raw)) return null;
  const points: { readonly x: number; readonly y: number }[] = [];
  for (const entry of raw) {
    const point = parseDagMarqueePoint(entry);
    if (!point) return null;
    points.push(point);
  }
  if (points.length < 2) return null;
  const coverage = crossing ? "partial" : "full";
  const resolvedMethod = method === "lasso" || method === "rectangle" ? method : inferDagMarqueeMethod(points);
  if (resolvedMethod === "lasso") return { kind: "lasso", points, coverage };
  const xs = points.map((point) => point.x);
  const ys = points.map((point) => point.y);
  const x = Math.min(...xs);
  const y = Math.min(...ys);
  return { kind: "rect", x, y, width: Math.max(...xs) - x, height: Math.max(...ys) - y, coverage };
}

export function sceneToSyncPack(scene: NodeGraphScene): Uint8Array {
  return new Uint8Array(encodePackValue(scene));
}

export function sceneToSyncJson(scene: NodeGraphScene): string {
  return JSON.stringify(scene);
}

//#region DagDomOverlays
export function GraphSliderOverlays({
  stateJson,
  logicalW,
  logicalH,
  editable,
  onSliderChange,
  onSliderPointerDown,
  onSliderPointerUp,
  occluderRect = null,
}: {
  readonly stateJson: string;
  readonly logicalW: number;
  readonly logicalH: number;
  readonly editable: boolean;
  readonly onSliderChange: (widgetId: string, value: number) => void;
  readonly onSliderPointerDown?: () => void;
  readonly onSliderPointerUp?: () => void;
  readonly occluderRect?: { readonly x: number; readonly y: number; readonly width: number; readonly height: number } | null;
}) {
  const camera = parseDagOverlayCamera(stateJson);
  const sliders = parseDagSliderOverlays(stateJson);
  const zoom = camera.zoom > 0 ? camera.zoom : 1;
  if (sliders.length === 0) return null;
  return (
    <div className="pointer-events-none absolute inset-0 z-45">
      {sliders.map((slider) => {
        const screen = dagWorldToScreen(camera, logicalW, logicalH, slider.x, slider.y);
        if (occluderRect && screen.x >= occluderRect.x && screen.x <= occluderRect.x + occluderRect.width && screen.y >= occluderRect.y && screen.y <= occluderRect.y + occluderRect.height) {
          return null;
        }
        // 🎚️ Lay out in world units and scale the whole control (track + knob tokens) with zoom —
        // multiplying only the box left the CSS thumb (`size-small`) and track (`h-single`) fixed.
        const w = slider.w;
        const h = Math.max(slider.h, 16 / zoom);
        return (
          <div
            key={slider.widgetId}
            className="pointer-events-auto absolute flex items-center"
            data-graph-slider-zoom={zoom}
            style={{ left: screen.x, top: screen.y, width: w, height: h, transform: `translate(-50%, -50%) scale(${zoom})`, transformOrigin: "center" }}
            onPointerDown={(event) => event.stopPropagation()}
          >
            <Slider
              className="h-full w-full min-w-0"
              max={slider.max}
              min={slider.min}
              step={slider.step}
              value={[slider.value]}
              disabled={!editable}
              showValue={false}
              onValueChange={(values) => onSliderChange(slider.widgetId, values[0] ?? slider.value)}
              onPointerDown={onSliderPointerDown}
              onPointerUp={onSliderPointerUp}
              onPointerCancel={onSliderPointerUp}
            />
          </div>
        );
      })}
    </div>
  );
}

const ALIGN_MODES = [
  { id: "left", label: "⬅️" },
  { id: "center-h", label: "↔" },
  { id: "right", label: "➡️" },
  { id: "top", label: "⬆️" },
  { id: "center-v", label: "↕️" },
  { id: "bottom", label: "⬇️" },
] as const;

export function alignModeToDag(mode: string): string {
  const map: Record<string, string> = {
    left: "alignLeft",
    right: "alignRight",
    top: "alignTop",
    bottom: "alignBottom",
    "center-h": "alignHorizontal",
    "center-v": "alignVertical",
  };
  return map[mode] ?? mode;
}

export function SelectionAlignChrome({ bounds, onAlign }: { readonly bounds: DagSelectionBounds; readonly onAlign: (mode: string) => void }) {
  return (
    <div className={cn("pointer-events-auto absolute z-50 flex gap-0.5 rounded border border-border p-0.5 shadow-sm", glassClass)} data-level="pane" style={{ left: bounds.x, top: Math.max(0, bounds.y - 28) }}>
      {ALIGN_MODES.map((mode) => (
        <button key={mode.id} type="button" className="size-5 rounded text-xs hover:bg-active-base" aria-label={mode.id} onPointerDown={(event) => event.stopPropagation()} onClick={() => onAlign(mode.id)}>
          {mode.label}
        </button>
      ))}
    </div>
  );
}
//#endregion DagDomOverlays
//#endregion DagOverlayPaint
//#endregion 🔖️graph-canvas-overlays

//#region 🔖️flow-graph-canvas-host

//#region Sync
// @emoji 🎥️ `applyCamera` must stay false for every resync after the first: live pan/zoom lives in the
// FlowWasmSession (and plugin runtime via `nodeGraphViewport`), while `scene.viewport` often lags.
// Applying it on hover/eval/edit-triggered resync would snap the camera; `loadFixtureJson` also
// preserves the live camera so fixture content reloads never reset the view.
function applyNodeGraphHoverFromScene(session: FlowWasmSession, hover: NodeGraphHover | undefined): void {
  if (hover === undefined) return;
  session.setHover?.(hover.nodeId ?? null);
}

function syncFlowSessionEvalFromScene(session: FlowWasmSession, scene: NodeGraphScene): void {
  if (scene.evalJson) session.applyEvalOutputsJson(scene.evalJson);
  if (scene.statusJson) session.setNodeStatuses?.(scene.statusJson);
  else if (scene.computingJson) session.setComputingProgress(scene.computingJson);
}

function syncFlowSessionStructureFromScene(
  session: FlowWasmSession,
  scene: NodeGraphScene,
  applyCamera: boolean,
  skipFixture = false,
): void {
  if (scene.operators) session.setNeuronKindInfosJson(JSON.stringify(scene.operators));
  if (!skipFixture && scene.fixtureJson) {
    if (session.resyncFixtureJson) session.resyncFixtureJson(scene.fixtureJson);
    else session.loadFixtureJson(scene.fixtureJson);
  }
  if (scene.selection) session.setSelection(JSON.stringify(scene.selection));
  applyNodeGraphHoverFromScene(session, scene.hover);
  if (scene.previewOffJson) session.setPreviewOff(scene.previewOffJson);
  if (scene.catalogueJson != null) session.setCatalogueJson(scene.catalogueJson);
  if (scene.lodJson) {
    try {
      const lod = parseSceneJsonField<{ readonly automatic?: boolean; readonly forcedLabel?: string }>(scene.lodJson);
      session.setAutomaticLod(lod.automatic !== false);
      if (lod.forcedLabel) session.setForcedDrawLodLabel(lod.forcedLabel);
    } catch {
      /* ignore */
    }
  }
  if (!applyCamera) return;
  const viewport = scene.viewport ?? DEFAULT_NODE_GRAPH_VIEWPORT;
  session.setCamera(viewport.x, viewport.y, viewport.zoom);
}

function syncFlowSessionFromScene(session: FlowWasmSession, scene: NodeGraphScene, applyCamera: boolean): void {
  syncFlowSessionStructureFromScene(session, scene, applyCamera);
  // 🧵️ Apply results from the plugin's off-main-thread `flowEvalTick` chain BEFORE computingJson —
  // applyEvalOutputsJson clears computing chrome, so applying computingJson first would have it
  // immediately wiped by this call on the same sync pass.
  syncFlowSessionEvalFromScene(session, scene);
}
//#endregion Sync

//#region FlowGraphCanvasHost
export function FlowGraphCanvasHost({
  scene,
  surfaceId,
  controllerId,
  editable,
  requestContextMenu,
  onAction,
}: {
  readonly scene: NodeGraphScene;
  readonly surfaceId: string;
  readonly controllerId: string;
  readonly editable: boolean;
  readonly requestContextMenu?: (request: PluginContextMenuRequest) => Promise<readonly ContextMenuItemSpec[]>;
  readonly onAction: (action: ActionDescriptor) => void;
}) {
  const windowInstanceId = useContext(WindowInstanceIdContext);
  const sessionRef = useRef<FlowWasmSession | null>(null);
  const schedulerRef = useRef<ReturnType<typeof createDemandFrameScheduler> | null>(null);
  const gpuCanvasRef = useRef<HTMLCanvasElement | null>(null);
  const labelCanvasRef = useRef<HTMLCanvasElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const [contextMenu, setContextMenu] = useState<SurfaceContextMenuResult & {
    readonly x: number;
    readonly y: number;
    readonly widgetId?: string;
  } | null>(null);
  const contextMenuTitleLabel = useLabel(contextMenu?.titleKey ?? "ui.surfaceContextMenu.flow");
  const [selectionBounds, setSelectionBounds] = useState<ReturnType<typeof parseDagSelectionUnionBoundsScreen>>(null);
  const [marquee, setMarquee] = useState<ReturnType<typeof computeDagMarqueeOverlay>>(null);
  const [labelStateJson, setLabelStateJson] = useState("{}");
  const [sliderStateJson, setSliderStateJson] = useState("{}");
  const [containerSize, setContainerSize] = useState({ w: 800, h: 600 });
  const [sessionReady, setSessionReady] = useState(false);
  const [spotlight, setSpotlight] = useState<FlowSpotlightState | null>(null);
  const sceneSignature = useMemo(() => JSON.stringify(scene), [scene]);
  // Always holds the latest `scene` without forcing effects to depend on (and re-run per) it.
  const sceneRef = useRef(scene);
  sceneRef.current = scene;

  useEffect(() => {
    if (!windowInstanceId) return;
    return registerIntroductionSurfaceResolver(windowElementId(windowInstanceId), dagIntroductionResolver(sessionRef, containerRef));
  }, [windowInstanceId]);

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({ controllerId, action, args: { surfaceId, ...args } });
    },
    [controllerId, onAction, surfaceId],
  );

  const flowMenuKeysByActionId = useAppKeybindingsByActionId();
  const shellContextMenuFallback = useShellContextMenuFallback();
  /** 🖱️ Builds the dispatch sink for one flow-graph context-menu opening — bound to that opening's own
   * `widgetId`/coordinates rather than closing over the (async, not-yet-committed) `contextMenu` state,
   * so `openInstance`'s fixture lookup always resolves against the widget actually right-clicked. */
  const buildFlowMenuDispatch = useCallback(
    (widgetId: string | undefined, x: number, y: number) => (action: string, args?: Record<string, unknown>) => {
      if (action === "openSpotlight") {
        const host = containerRef.current;
        if (host) openSpotlightAtClient(x, y, host);
        return;
      }
      dispatch(action, action === "openInstance" ? { ...args, instanceId: resolveFixtureWidgetInstanceId(scene.fixtureJson, widgetId) } : args);
    },
    [dispatch, scene.fixtureJson],
  );

  // 🧵️ Dispatches the mutated fixture to the plugin and returns immediately — evaluation happens
  // off the main thread in the plugin worker's `flowEvalTick` chain, never here. The next scene
  // resync applies its `evalJson`/`computingJson` back onto this session (`syncFlowSessionFromScene`).
  const commitFixture = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    try {
      const fixtureJson = session.fixtureJson();
      dispatch(nodeGraphActions.edit, { operations: [{ operation: "setFixture", fixtureJson }] });
    } catch {
      /* session not ready */
    }
  }, [dispatch]);

  // A continuous gesture (e.g. dragging a slider) fires many onValueChange ticks per second, each
  // committing the whole document through an async plugin round-trip; concurrent in-flight commits
  // can resolve out of order, and the scene-resync effect below would apply whichever one lands
  // last — visibly reverting the drag mid-gesture. isGestureActiveRef suppresses that resync while
  // a gesture is active, and commitFixtureThrottled caps how many concurrent commits are in flight.
  const isGestureActiveRef = useRef(false);
  const lastCommitAtRef = useRef(0);
  const pendingCommitTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const GESTURE_COMMIT_THROTTLE_MS = 80;

  const commitFixtureThrottled = useCallback(() => {
    if (pendingCommitTimeoutRef.current != null) {
      clearTimeout(pendingCommitTimeoutRef.current);
      pendingCommitTimeoutRef.current = null;
    }
    const elapsed = Date.now() - lastCommitAtRef.current;
    if (elapsed >= GESTURE_COMMIT_THROTTLE_MS) {
      lastCommitAtRef.current = Date.now();
      commitFixture();
    } else {
      pendingCommitTimeoutRef.current = setTimeout(() => {
        pendingCommitTimeoutRef.current = null;
        lastCommitAtRef.current = Date.now();
        commitFixture();
      }, GESTURE_COMMIT_THROTTLE_MS - elapsed);
    }
  }, [commitFixture]);

  const handleGesturePointerDown = useCallback(() => {
    isGestureActiveRef.current = true;
    schedulerRef.current?.beginContinuous("gesture");
  }, []);

  useEffect(() => {
    return () => {
      if (pendingCommitTimeoutRef.current != null) clearTimeout(pendingCommitTimeoutRef.current);
    };
  }, []);

  const paintOverlays = useCallback(() => {
    const session = sessionRef.current;
    const labelCanvas = labelCanvasRef.current;
    const container = containerRef.current;
    if (!session || !labelCanvas || !container) return;
    const rect = container.getBoundingClientRect();
    const dpr = globalThis.devicePixelRatio || 1;
    setContainerSize((prev) => (prev.w === rect.width && prev.h === rect.height ? prev : { w: rect.width, h: rect.height }));
    try {
      const labelJson = session.labelOverlayPaintStateJson();
      setLabelStateJson((prev) => (prev === labelJson ? prev : labelJson));
      const minimapCursor = parseDagMinimapWidgetCursor(labelJson);
      if (gpuCanvasRef.current) {
        gpuCanvasRef.current.style.cursor = minimapCursor ?? "";
      }
      const selectedIds = parseDagNodeIdArray(session.selectedWidgetIds());
      const preselect = parseDagPreselectJson(session.preselectWidgetIdsJson());
      const dimmedIds = parseDagNodeIdArray(session.previewOffWidgetIds());
      paintDagLabelOverlays(labelJson, labelCanvas, rect.width, rect.height, dpr, {
        hoveredId: session.hoveredWidgetId() ?? null,
        selectedIds,
        preselect,
        dimmedIds,
      });
      const nextSliderJson = session.sliderOverlayStateJson();
      setSliderStateJson((prev) => (prev === nextSliderJson ? prev : nextSliderJson));
    } catch {
      /* gpu not ready */
    }
    setSelectionBounds(parseDagSelectionUnionBoundsScreen(session.selectionUnionBoundsScreenJson()));
    setMarquee(computeDagMarqueeOverlay(session.selectionPreviewPointsJson(), session.selectionPreviewCrossing(), session.selectionPreviewMethod?.()));
  }, []);

  const handleGesturePointerUp = useCallback(() => {
    isGestureActiveRef.current = false;
    schedulerRef.current?.endContinuous("gesture");
    schedulerRef.current?.invalidate();
    if (pendingCommitTimeoutRef.current != null) {
      clearTimeout(pendingCommitTimeoutRef.current);
      pendingCommitTimeoutRef.current = null;
    }
    const session = sessionRef.current;
    if (session) {
      // 🎚️ Gesture end: resync catalogue/selection/lod from the lagging scene document, but keep the wasm
      // session's live fixture (slider seeds) and skip stale `evalJson`/`statusJson` until the plugin
      // worker's `flowEvalTick` chain publishes a fresh scene after `commitFixture`.
      syncFlowSessionStructureFromScene(session, sceneRef.current, false, true);
      try {
        session.renderFrame();
      } catch {
        /* gpu not ready */
      }
      paintOverlays();
    }
    lastCommitAtRef.current = Date.now();
    commitFixture();
  }, [commitFixture, paintOverlays]);

  const emitInteractionState = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    try {
      const domains = parseSelectionDomainsFromSession(session.selectionDomainsJson?.() ?? session.selectedWidgetIds());
      dispatch(nodeGraphActions.select, { nodeIds: domains.nodes, edgeIds: domains.edges, handleIds: domains.handles });
      const hovered = session.hoveredWidgetId();
      const channelJson = session.hoveredChannelJson();
      dispatch(nodeGraphActions.hover, { hoverJson: hovered ? channelJson : null });
      const cameraJson = session.cameraJson?.();
      if (cameraJson) dispatch(nodeGraphActions.viewport, nodeGraphViewportActionArgs(cameraJson));
    } catch {
      /* session not ready */
    }
    paintOverlays();
  }, [dispatch, paintOverlays]);

  useEffect(() => {
    let cancelled = false;
    void createFlowSession().then((session) => {
      if (cancelled) {
        session.free();
        return;
      }
      sessionRef.current = session;
      setSessionReady(true);
    });
    return () => {
      cancelled = true;
      // 🪶️ REDUCE-DEMONSTRATOR-IDLE-MEMORY-FOOTPRINT: was never freed on unmount — the wasm-side
      // session (and everything it retains) leaked for the rest of the document's lifetime.
      sessionRef.current?.free();
      sessionRef.current = null;
    };
  }, []);

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
    session
      .attachCanvas(canvas, Math.round(rect.width), Math.round(rect.height), dpr)
      .then(() => {
        if (cancelled) return;
        syncFlowSessionFromScene(session, sceneRef.current, true);
        syncSessionCanvasTheme(session);
        const resize = () => {
          const next = container.getBoundingClientRect();
          const nextDpr = globalThis.devicePixelRatio || 1;
          session.setSize(Math.round(next.width), Math.round(next.height), nextDpr);
          session.renderFrame();
          paintOverlays();
        };
        resize();
        const ro = new ResizeObserver(resize);
        ro.observe(container);
        // 🪶️ REDUCE-DEMONSTRATOR-IDLE-MEMORY-FOOTPRINT: was an unconditional 60fps `requestAnimationFrame`
        // loop for the surface's entire lifetime — see `createDemandFrameScheduler`'s docstring.
        const scheduler = createDemandFrameScheduler(() => {
          session.renderFrame();
          paintOverlays();
        });
        schedulerRef.current = scheduler;
        scheduler.invalidate();
        cleanupAttached = () => {
          ro.disconnect();
          scheduler.dispose();
          schedulerRef.current = null;
        };
      })
      .catch(() => {
        /* already attached (e.g. a stale re-run) or transient failure; nothing to clean up */
      });
    return () => {
      cancelled = true;
      cleanupAttached?.();
    };
  }, [sessionReady, paintOverlays]);

  useEffect(() => {
    const session = sessionRef.current;
    if (!session || !sessionReady) return;
    // 🎚️ While a slider (or other continuous) gesture is active, the wasm session already holds the live
    // fixture edits via `setSliderValue`; applying `scene.evalJson` here would install a stale baseline
    // (new slider seeds + old channel outputs) and wipe computing chrome mid-drag. Full resync waits for
    // `handleGesturePointerUp`.
    if (!isGestureActiveRef.current) {
      syncFlowSessionFromScene(session, scene, false);
    }
    session.renderFrame();
    paintOverlays();
    schedulerRef.current?.invalidate();
  }, [sceneSignature, paintOverlays, scene, sessionReady]);

  const flowGraphCanvasHostShellScope = useShellScopeOptional();
  useCanvasAppearanceSync(
    () => {
      syncSessionCanvasTheme(sessionRef.current);
      try {
        sessionRef.current?.renderFrame();
      } catch {
        /* gpu not ready */
      }
      paintOverlays();
      schedulerRef.current?.invalidate();
    },
    true,
    flowGraphCanvasHostShellScope?.rootRef.current ?? undefined,
  );

  const pickInteraction = useCanvasPickInteraction({
    resolveTargetsAtClient: (client) => {
      const session = sessionRef.current;
      const container = containerRef.current;
      if (!session || !container) return [];
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
      if (!target) {
        session.setHover?.(null);
      } else if (target.portId) {
        session.setHoverChannel?.(target.id, target.portId);
      } else {
        session.setHover?.(target.id);
      }
      try {
        const hovered = session.hoveredWidgetId();
        const channelJson = session.hoveredChannelJson();
        dispatch(nodeGraphActions.hover, { hoverJson: hovered ? channelJson : null });
      } catch {
        /* session not ready */
      }
      session.renderFrame();
      paintOverlays();
    },
    onSelectTarget: () => {
      emitInteractionState();
    },
  });

  const clearGhostPreview = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    session.clearGhostWidget();
    session.renderFrame();
    paintOverlays();
  }, [paintOverlays]);

  const closeSpotlight = useCallback(() => {
    setSpotlight(null);
    clearGhostPreview();
  }, [clearGhostPreview]);

  const previewSpotlightItem = useCallback(
    (item: FlowCatalogueItem | null) => {
      const session = sessionRef.current;
      const open = spotlight;
      if (!session || !open) return;
      if (!item) {
        session.clearGhostWidget();
        session.renderFrame();
        paintOverlays();
        return;
      }
      console.log("[DEBUG] flow spotlight preview", item.kind, item.neuronKind ?? item.name, open.world);
      session.setGhostWidget(flowCatalogueItemDescriptor(item), open.world.x, open.world.y);
      session.renderFrame();
      paintOverlays();
    },
    [paintOverlays, spotlight],
  );

  const commitSpotlightItem = useCallback(
    (item: FlowCatalogueItem) => {
      const session = sessionRef.current;
      const open = spotlight;
      if (!session || !open) return;
      console.log("[DEBUG] flow spotlight commit", item.kind, item.neuronKind ?? item.name, open.world);
      try {
        session.addWidget(flowCatalogueItemDescriptor(item), open.world.x, open.world.y);
        commitFixture();
        emitInteractionState();
      } catch {
        /* invalid descriptor */
      }
      setSpotlight(null);
      clearGhostPreview();
    },
    [clearGhostPreview, commitFixture, emitInteractionState, spotlight],
  );

  const openSpotlightAtClient = useCallback(
    (clientX: number, clientY: number, target: HTMLElement) => {
      const session = sessionRef.current;
      if (!session || !editable) return;
      const rect = target.getBoundingClientRect();
      const sx = clientX - rect.left;
      const sy = clientY - rect.top;
      let world = { x: sx, y: sy };
      try {
        const parsed = JSON.parse(session.worldFromScreen(sx, sy)) as { readonly x?: number; readonly y?: number };
        world = { x: parsed.x ?? sx, y: parsed.y ?? sy };
      } catch {
        const camera = parseDagOverlayCamera(labelStateJson);
        world = dagScreenToWorld(camera, rect.width, rect.height, sx, sy);
      }
      console.log("[DEBUG] flow spotlight open", { screen: { x: sx, y: sy }, world });
      setSpotlight({ screen: { x: sx, y: sy }, world });
    },
    [editable, labelStateJson],
  );

  const onDragOverCanvas = useCallback(
    (event: DragEvent<HTMLDivElement>) => {
      if (!editable) return;
      event.preventDefault();
      const session = sessionRef.current;
      const container = containerRef.current;
      if (!session || !container) return;
      const encoded = getActiveCatalogueDragPayload();
      if (!encoded) return;
      const catalogueApp = parseCatalogueAppDragPayload(encoded);
      if (!catalogueApp) return;
      const rect = container.getBoundingClientRect();
      const sx = event.clientX - rect.left;
      const sy = event.clientY - rect.top;
      let world = { x: sx, y: sy };
      try {
        const parsed = JSON.parse(session.worldFromScreen(sx, sy)) as { readonly x?: number; readonly y?: number };
        world = { x: parsed.x ?? sx, y: parsed.y ?? sy };
      } catch {
        const camera = parseDagOverlayCamera(labelStateJson);
        world = dagScreenToWorld(camera, rect.width, rect.height, sx, sy);
      }
      session.setGhostWidget(catalogueGhostDescriptorJson(catalogueApp), world.x, world.y);
      session.renderFrame();
      paintOverlays();
    },
    [editable, labelStateJson, paintOverlays],
  );

  const onDrop = useCallback(
    (event: DragEvent<HTMLDivElement>) => {
      if (!editable) return;
      clearGhostPreview();
      const raw = event.dataTransfer.getData(CATALOGUE_DRAG_MIME) || event.dataTransfer.getData("text/plain") || getActiveCatalogueDragPayload() || "";
      if (!raw) return;
      event.preventDefault();
      const session = sessionRef.current;
      const container = containerRef.current;
      if (!session || !container) return;
      const rect = container.getBoundingClientRect();
      const sx = event.clientX - rect.left;
      const sy = event.clientY - rect.top;
      let world = { x: sx, y: sy };
      try {
        const parsed = JSON.parse(session.worldFromScreen(sx, sy)) as { readonly x?: number; readonly y?: number };
        world = { x: parsed.x ?? sx, y: parsed.y ?? sy };
      } catch {
        const camera = parseDagOverlayCamera(labelStateJson);
        world = dagScreenToWorld(camera, rect.width, rect.height, sx, sy);
      }
      const catalogueApp = parseCatalogueAppDragPayload(raw);
      if (catalogueApp) {
        dispatch("spawnApp", { pluginId: catalogueApp.pluginId, appId: catalogueApp.appId, x: world.x, y: world.y });
        return;
      }
      try {
        const descriptor = raw.startsWith("{") ? raw : JSON.stringify({ kind: raw });
        session.addWidget(descriptor, world.x, world.y);
        commitFixture();
        emitInteractionState();
      } catch {
        /* invalid descriptor */
      }
    },
    [clearGhostPreview, commitFixture, dispatch, editable, emitInteractionState, labelStateJson],
  );

  const onCanvasDoubleClick = useCallback(
    (event: MouseEvent<HTMLDivElement>) => {
      if (!editable) return;
      const session = sessionRef.current;
      if (!session) return;
      const hovered = session.hoveredWidgetId();
      if (hovered) {
        const instanceId = resolveFixtureWidgetInstanceId(scene.fixtureJson, hovered);
        if (instanceId) {
          dispatch("openInstance", { instanceId });
          return;
        }
        return;
      }
      openSpotlightAtClient(event.clientX, event.clientY, event.currentTarget);
    },
    [dispatch, editable, openSpotlightAtClient, scene.fixtureJson],
  );

  useEffect(() => clearGhostPreview, [clearGhostPreview]);

  const spotlightSections = useMemo(() => {
    if (!spotlight || !sessionReady) return [] as FlowCatalogueSection[];
    const session = sessionRef.current;
    if (!session) return parseFlowCatalogueSections(scene.catalogueJson);
    try {
      return parseFlowCatalogueSections(session.catalogueJson());
    } catch {
      return parseFlowCatalogueSections(scene.catalogueJson);
    }
  }, [scene.catalogueJson, sessionReady, spotlight]);

  return (
    <div
      ref={containerRef}
      className="relative h-full w-full"
      onDragOver={onDragOverCanvas}
      onDragLeave={() => {
        if (!editable) return;
        clearGhostPreview();
      }}
      onDrop={onDrop}
      onContextMenu={(event) => {
        if (!editable || !requestContextMenu) return;
        event.preventDefault();
        event.stopPropagation();
        void (async () => {
          const session = sessionRef.current;
          const container = containerRef.current;
          let widgetId: string | undefined;
          let hits: NonNullable<PluginContextMenuSurfaceTarget["hits"]> = [];
          let domains = { nodes: [] as string[], edges: [] as string[], handles: [] as string[] };
          if (session) {
            try {
              domains = parseSelectionDomainsFromSession(session.selectionDomainsJson?.() ?? session.selectedWidgetIds());
            } catch {
              domains = { nodes: [], edges: [], handles: [] };
            }
          }
          if (session && container) {
            const rect = container.getBoundingClientRect();
            const sx = event.clientX - rect.left;
            const sy = event.clientY - rect.top;
            try {
              const targets = JSON.parse(session.pickTargetsAtScreenJson(sx, sy)) as CanvasPickTarget[];
              hits = targets.map((target) => ({ domain: target.domain, id: target.id, label: target.label }));
              widgetId = pickMostSpecificCanvasTarget(targets)?.id;
            } catch {
              widgetId = undefined;
            }
          }
          if (!widgetId) {
            widgetId = session?.hoveredWidgetId() ?? undefined;
          }
          if (widgetId && !domains.nodes.includes(widgetId)) {
            domains = { nodes: [widgetId], edges: [], handles: [] };
            try {
              session?.setSelection?.(JSON.stringify(domains));
              session?.renderFrame();
            } catch {
              /* session not ready */
            }
            dispatch("contextMenuAt", { id: widgetId });
          } else if (widgetId) {
            dispatch("contextMenuAt", { id: widgetId });
          }
          const menu = await openSurfaceContextMenu(
            requestContextMenu,
            {
              menu: { id: "nodeGraph" },
              surface: {
                surfaceId,
                kind: "nodeGraph",
                hits,
                selection: selectionGroupsFromDomains(domains),
              },
              windowInstanceId: windowInstanceId ?? undefined,
              point: { x: event.clientX, y: event.clientY },
            },
            (specs) => mapContextMenuSpecs(specs, buildFlowMenuDispatch(widgetId, event.clientX, event.clientY), flowMenuKeysByActionId),
            shellContextMenuFallback,
          );
          setContextMenu({ x: event.clientX, y: event.clientY, widgetId, ...menu });
          paintOverlays();
        })();
      }}
    >
      <canvas ref={gpuCanvasRef} className="absolute inset-0 block h-full w-full" />
      <canvas ref={labelCanvasRef} className="pointer-events-none absolute inset-0 z-40" />
      <GraphSliderOverlays
        stateJson={sliderStateJson}
        logicalW={containerSize.w}
        logicalH={containerSize.h}
        editable={editable}
        occluderRect={parseDagMinimapWidgetOccluder(labelStateJson)}
        onSliderChange={(widgetId, value) => {
          const session = sessionRef.current;
          if (!session) return;
          session.setSliderValue(widgetId, value);
          try {
            session.renderFrame();
          } catch {
            /* gpu not ready */
          }
          commitFixtureThrottled();
          paintOverlays();
        }}
        onSliderPointerDown={handleGesturePointerDown}
        onSliderPointerUp={handleGesturePointerUp}
      />
      {selectionBounds ? (
        <>
          <div className="pointer-events-none absolute z-20 border-2 border-accent" style={{ left: selectionBounds.x, top: selectionBounds.y, width: selectionBounds.width, height: selectionBounds.height }} />
          {editable ? (
            <SelectionAlignChrome
              bounds={selectionBounds}
              onAlign={(mode) => {
                sessionRef.current?.alignSelection(mode);
                commitFixture();
                paintOverlays();
              }}
            />
          ) : null}
        </>
      ) : null}
      {marquee ? (
        marquee.kind === "lasso" ? (
          <SelectionMarquee className="z-50" coverage={marquee.coverage ?? "full"} shape="polygon" points={marquee.points ?? []} />
        ) : (
          <SelectionMarquee className="z-50" coverage={marquee.coverage ?? "full"} shape="rect" rect={{ x: marquee.x ?? 0, y: marquee.y ?? 0, width: marquee.width ?? 0, height: marquee.height ?? 0 }} />
        )
      ) : null}
      <div
        className="absolute inset-0 z-30"
        onPointerDown={(event) => {
          if (!editable) return;
          // 🖱️ Secondary button opens the context menu — never start node drag / marquee from it.
          if (event.button === 2) return;
          const session = sessionRef.current;
          if (!session) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const client = { x: event.clientX, y: event.clientY };
          pickInteraction.onCanvasPointerDown(client);
          session.pointerDownScreen(event.clientX - rect.left, event.clientY - rect.top, event.button, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey, event.button === 1 || event.buttons === 4);
          session.renderFrame();
          paintOverlays();
        }}
        onPointerMove={(event) => {
          const session = sessionRef.current;
          if (!session) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const client = { x: event.clientX, y: event.clientY };
          pickInteraction.onCanvasPointerMove(client);
          session.pointerMoveScreen(event.clientX - rect.left, event.clientY - rect.top, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey);
          session.renderFrame();
          paintOverlays();
        }}
        onPointerUp={(event) => {
          if (event.button === 2) return;
          const session = sessionRef.current;
          if (!session) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const client = { x: event.clientX, y: event.clientY };
          pickInteraction.onCanvasPointerUp(client, { shift: event.shiftKey, ctrlOrMeta: event.metaKey || event.ctrlKey, alt: event.altKey });
          session.pointerUpScreen(event.clientX - rect.left, event.clientY - rect.top, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey);
          session.renderFrame();
          commitFixture();
          emitInteractionState();
        }}
        onPointerLeave={() => pickInteraction.onCanvasPointerLeave()}
        onDoubleClick={onCanvasDoubleClick}
        onWheel={(event) => {
          event.preventDefault();
          const session = sessionRef.current;
          if (!session) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const delta = event.deltaMode === 1 ? event.deltaY * 16 : event.deltaMode === 2 ? event.deltaY * 400 : event.deltaY;
          session.wheelScreen(event.clientX - rect.left, event.clientY - rect.top, 0, delta, true);
          session.renderFrame();
          const cameraJson = session.cameraJson?.();
          if (cameraJson) dispatch(nodeGraphActions.viewport, nodeGraphViewportActionArgs(cameraJson));
          paintOverlays();
        }}
      />
      <CanvasPickMenu request={pickInteraction.pickMenu} hoveredKey={pickInteraction.menuHoveredKey} onHoverKey={pickInteraction.onMenuHoverKey} onPick={pickInteraction.onMenuPick} onDismiss={pickInteraction.dismissPickMenu} />
      {spotlight ? (
        <FlowSpotlight state={spotlight} sections={spotlightSections} onPreview={previewSpotlightItem} onCommit={commitSpotlightItem} onClose={closeSpotlight} />
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
//#endregion FlowGraphCanvasHost
//#endregion 🔖️flow-graph-canvas-host
//#endregion 🔖️NodeGraphHost
