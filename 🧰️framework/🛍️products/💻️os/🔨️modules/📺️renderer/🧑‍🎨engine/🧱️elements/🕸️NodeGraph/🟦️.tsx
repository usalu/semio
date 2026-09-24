// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/NodeGraph/component.tsx
/** @emoji 🕸️ `NodeGraph` — the node-graph/flow program scene host: wasm dag-engine canvas surface,
 * the flow-engine (React Flow) canvas host, the catalogue double-click spotlight, label/slider/marquee
 * canvas overlays shared by both engines, and the SSR-safe `Diagram`-based fallback. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import React, { useCallback, useContext, useEffect, useMemo, useRef, useState, type DragEvent, type KeyboardEvent, type MouseEvent } from "react";
import { type GraphWasmSession, GraphWasmCanvas } from "@semio-tech/infinite-canvas-react-renderer";
import { STYLING_METRICS, currentStylingAppearanceName, resolveColorHex, serializeCanvasThemeJson, syncSessionCanvasTheme } from "@semio-tech/ui-styling";
import {
  borderNormalBottomClass,
  CanvasPickMenu,
  CATALOGUE_DRAG_MIME,
  cn,
  ContextMenuController,
  Diagram,
  DiagramLiveRegion,
  diagramKeyboardAnnouncement,
  diagramKeyboardStep,
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
  useDiagramTranslate,
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
  GestureRecognizer,
  createContinuousGestureLane,
  nodeGraphActions,
  pinchZoomNotches,
  parseViewport2d,
  windowElementId,
  type ActionDescriptor,
  type ContinuousGestureLane,
  type ComponentSceneHostProps,
  type ContextMenuItemSpec,
  type PinchStep,
  type NodeGraphEdgeRecord,
  type NodeGraphFindItem,
  type NodeGraphHover,
  type NodeGraphNodeRecord,
  type NodeGraphInteractionDomain,
  type NodeGraphPortRecord,
  type NodeGraphScene,
  type Viewport2d,
  type PluginContextMenuRequest,
  type PluginContextMenuSurfaceTarget,
  type PresencePeer,
  type UiComponentSceneNode,
  type AppCatalogue,
} from "@semio-tech/framework";
import { encodePackValue } from "@semio-tech/framework-os";
import { openSurfaceContextMenu, parseSceneJsonField, useAppCatalogue, useShellContextMenuFallback, type SurfaceContextMenuResult } from "../🗣️Interpreter/🟦️.tsx";
import { mapContextMenuSpecs, parseJsonArray, parseSelectionDomainsFromSession, selectionGroupsFromDomains, WindowInstanceIdContext } from "../🌐️World3dHost/🟦️.tsx";
import { createDemandFrameScheduler, createFlowSession, createGraphSession, isFlowGraphScene, type FlowTask, type FlowWasmSession } from "../🪪️WasmSessionLoader/🟦️.tsx";
import { useAppKeybindingsByActionId, useMapContextMenuSpecs } from "../🏛️ShellHost/🟦️.tsx";
import { useUIFindSafe } from "../🔎️ShellSearch/🟦️.tsx";
import { hopTrace } from "../../../../../../../🔨️modules/⏱️trace/🟦️.ts";
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
  pointerCancelScreen(): void;
  wheelScreen(sx: number, sy: number, deltaX: number, deltaY: number, zoomGesture: boolean): void;
  labelOverlayPaintStateJson(): string;
  sliderOverlayStateJson(): string;
  selectionUnionBoundsScreenJson(): string;
  selectionPreviewPointsJson(): string;
  selectionPreviewCrossing(): boolean;
  selectionPreviewMethod?(): string;
  selectedNodeIdsJson(): string;
  selectionDomainsJson?(): string;
  hoveredNodeId(): string | null | undefined;
  hoveredChannelJson(): string;
  viewport(): unknown;
  takePendingOpenInstanceId(): string | null | undefined;
  pickTargetsAtScreenJson(sx: number, sy: number): string;
  /** 🎯️ Screen-space geometry for a live entity (`domain`/`id` in the pick-target grammar) — powers
   * introduction-demonstration semantic targeting. */
  entityScreenJson?(domain: string, id: string): string;
  setHover?(widgetId: string | null): void;
  setHoverChannel?(widgetId: string | null, port?: string | null): void;
  syncInteraction?(selectedIdsJson: string, hoveredId?: string | null): void;
  alignSelection?(mode: string): void;
  hostSnapshotJson?(): string;
  setCanvasThemeJson?(json: string): void;
};
//#endregion Types

//#region 🎯️GraphPickContract
/** 🎯️ Decodes the native handle target grammar emitted by the shared DAG engine. */
export function nodeGraphPickChannel(target: Pick<CanvasPickTarget, "domain" | "id"> | null): { readonly nodeId: string; readonly portId: string } | null {
  if (target?.domain !== "handle") return null;
  const boundary = target.id.indexOf("@");
  if (boundary <= 0 || boundary === target.id.length - 1) return null;
  return { nodeId: target.id.slice(0, boundary), portId: target.id.slice(boundary + 1) };
}

/** 🔌️ Decodes a `hoveredChannelJson()`/`hoveredChannelJson:interaction` `DagChannelRef` payload
 * (`{widgetId, port, direction}`, or `"null"`) into the `"{nodeId}@{portId}"` pick-id halves. */
function parseDagChannelRefJson(json: string): { readonly nodeId: string; readonly portId: string } | null {
  try {
    const parsed = JSON.parse(json) as { readonly widgetId?: unknown; readonly port?: unknown } | null;
    if (!parsed || typeof parsed.widgetId !== "string" || typeof parsed.port !== "string") return null;
    return { nodeId: parsed.widgetId, portId: parsed.port };
  } catch {
    return null;
  }
}

/** 🔤️ The value schemas a port declares, as the comma-joined `valueType` the scene carries them in.
 * An empty list is an undeclared port, which stays connectable. */
export function portValueTypes(port: NodeGraphPortRecord | undefined): readonly string[] {
  return port?.valueType ? port.valueType.split(",").filter((entry) => entry.length > 0) : [];
}

/** 🔌️ The ONE port-compatibility rule, in TypeScript — the twin of
 * `neural_engine::Registry::channel_compatible` and `semio_framework_os_flow::port_value_types_compatible`.
 * A pair is refused only when both sides declare and the declared sets are disjoint, so an
 * undeclared channel is never retro-refused.
 *
 * @see `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧫️fixtures/🔌️port-types/🔣️.json` — the oracle
 */
export function portValueTypesCompatible(source: readonly string[], target: readonly string[]): boolean {
  if (source.length === 0 || target.length === 0) return true;
  return source.some((provided) => target.includes(provided));
}

/** 🔌️ Whether the wire a drag would draw between two node records may land — the predicate React
 * Flow asks before it paints a snap target as droppable, and again before it fires `onConnect`. */
export function nodeGraphConnectionIsValid(records: readonly NodeGraphNodeRecord[], connection: { readonly source?: string | null; readonly target?: string | null; readonly sourceHandle?: string | null; readonly targetHandle?: string | null }): boolean {
  const sourceNode = records.find((record) => record.id === connection.source);
  const targetNode = records.find((record) => record.id === connection.target);
  const sourcePort = sourceNode?.outputs.find((port) => portHandleId(port) === connection.sourceHandle);
  const targetPort = targetNode?.inputs.find((port) => portHandleId(port) === connection.targetHandle);
  return portValueTypesCompatible(portValueTypes(sourcePort), portValueTypes(targetPort));
}

/** 🚫️ The port pair a live wire drag is hovering that the declared port types forbid, as the board
 * publishes it alongside the hovered channel. */
export type DagWireTypeRefusal = {
  readonly source: string;
  readonly sourceTypes: readonly string[];
  readonly target: string;
  readonly targetTypes: readonly string[];
};

/** 🚫️ Decodes the `refusal` the board rides on `hoveredChannelJson()` — `null` whenever the drag is
 * over open canvas or over a port it may legally land on. */
export function parseDagWireTypeRefusalJson(json: string): DagWireTypeRefusal | null {
  try {
    const parsed = JSON.parse(json) as { readonly refusal?: unknown } | null;
    const refusal = parsed?.refusal as Partial<DagWireTypeRefusal> | undefined;
    if (!refusal || typeof refusal.source !== "string" || typeof refusal.target !== "string") return null;
    return {
      source: refusal.source,
      sourceTypes: Array.isArray(refusal.sourceTypes) ? refusal.sourceTypes.filter((entry): entry is string => typeof entry === "string") : [],
      target: refusal.target,
      targetTypes: Array.isArray(refusal.targetTypes) ? refusal.targetTypes.filter((entry): entry is string => typeof entry === "string") : [],
    };
  } catch {
    return null;
  }
}

/** 🏷️ The localized noun for each declared port type, so a refusal reads in the user's own language
 * instead of echoing the schema id. An id the table does not carry falls back to the id itself. */
function usePortTypeLabels(): Readonly<Record<string, string>> {
  const geometry = useLabel("ui.nodeGraph.portType.geometry");
  const vector = useLabel("ui.nodeGraph.portType.vector");
  const point = useLabel("ui.nodeGraph.portType.point");
  const numberLabel = useLabel("ui.nodeGraph.portType.number");
  const text = useLabel("ui.nodeGraph.portType.text");
  const booleanLabel = useLabel("ui.nodeGraph.portType.boolean");
  const list = useLabel("ui.nodeGraph.portType.list");
  return useMemo(() => ({ geometry, vector, point, number: numberLabel, text, boolean: booleanLabel, list }), [booleanLabel, geometry, list, numberLabel, point, text, vector]);
}

/** 🚫️ The interpolation a refusal hint needs: both port names and both declared type lists, each
 * type spelled in the reader's language. */
export function wireRefusalLabelOptions(refusal: DagWireTypeRefusal | null, portTypeLabels: Readonly<Record<string, string>>): Record<string, string> {
  const spell = (types: readonly string[]) => types.map((entry) => portTypeLabels[entry] ?? entry).join(" / ");
  return {
    source: refusal?.source ?? "",
    sourceType: spell(refusal?.sourceTypes ?? []),
    target: refusal?.target ?? "",
    targetType: spell(refusal?.targetTypes ?? []),
  };
}

function syncOptionalGraphCanvasTheme(session: FrameworkGraphSession | null): void {
  if (session?.setCanvasThemeJson) syncSessionCanvasTheme({ setCanvasThemeJson: session.setCanvasThemeJson.bind(session) });
}
//#endregion 🎯️GraphPickContract

//#region Viewport
export function parseNodeGraphSessionViewport(value: unknown): Viewport2d {
  return parseViewport2d(value);
}

export function nodeGraphViewportActionArgs(viewport: Viewport2d): { readonly viewport: Viewport2d } {
  return { viewport: parseViewport2d(viewport) };
}

type NodeGraphInteractionIds = {
  readonly nodeIds: readonly string[];
  readonly edgeIds?: readonly string[];
  readonly handleIds?: readonly string[];
};

export function nodeGraphSelectionActionArgs(domain: NodeGraphInteractionDomain | undefined, ids: NodeGraphInteractionIds) {
  if (!domain) return undefined;
  const targets = [
    ...ids.nodeIds.map((id) => ({ granularity: "node", id: `${domain.nodeTargetPrefix}${id}` })),
    ...(ids.edgeIds ?? []).map((id) => ({ granularity: "edge", id: `${domain.edgeTargetPrefix}${id}` })),
    ...(ids.handleIds ?? []).map((id) => ({ granularity: "handle", id: `${domain.handleTargetPrefix}${id}` })),
  ];
  return { domainId: domain.id, targets: JSON.stringify(targets), merge: "replace", method: "pick" };
}

export function nodeGraphHoverActionArgs(domain: NodeGraphInteractionDomain | undefined, nodeId: string | null | undefined, portId?: string | null) {
  if (!domain) return undefined;
  const targets = nodeId
    ? [portId ? { granularity: "handle", id: `${domain.handleTargetPrefix}${nodeId}@${portId}` } : { granularity: "node", id: `${domain.nodeTargetPrefix}${nodeId}` }]
    : [];
  return { domainId: domain.id, channel: "pointer", targets: JSON.stringify(targets) };
}

function publishNodeGraphSelection(dispatch: (action: string, args?: Record<string, unknown>) => unknown, domain: NodeGraphInteractionDomain | undefined, ids: NodeGraphInteractionIds): void {
  const args = nodeGraphSelectionActionArgs(domain, ids);
  if (args) dispatch(nodeGraphActions.select, args);
}

function publishNodeGraphHover(
  dispatch: (action: string, args?: Record<string, unknown>) => unknown,
  domain: NodeGraphInteractionDomain | undefined,
  nodeId: string | null | undefined,
  portId?: string | null,
): void {
  const args = nodeGraphHoverActionArgs(domain, nodeId, portId);
  if (args) dispatch(nodeGraphActions.hover, args);
}
//#endregion Viewport

//#region Parsing
const DEFAULT_NODE_GRAPH_VIEWPORT: Viewport2d = { x: 0, y: 0, zoom: 1 };

/** @emoji 🔎️ Resolves a flow host snapshot widget id to the workflow instance id it previews, used to open an app instance without depending on plugin-side selection state. */
export function resolveHostSnapshotWidgetInstanceId(hostSnapshotJson: string | undefined, widgetId: string | undefined | null): string | undefined {
  if (!hostSnapshotJson || !widgetId) return undefined;
  try {
    const hostSnapshot = JSON.parse(hostSnapshotJson) as {
      readonly widgets?: readonly { readonly id?: string; readonly params?: { readonly instanceId?: string } }[];
    };
    return hostSnapshot.widgets?.find((widget) => widget.id === widgetId)?.params?.instanceId;
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
  if (item.kind === "inputSlider") descriptor.label = item.name;
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
  const inputId = React.useId();
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
          id={`flow-spotlight-${inputId}`}
          aria-label={typeToAddLabel}
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

/** 🎯️ Diagram nodes for the React-Flow fallback board, carrying the PLUGIN's selection as their own
 * `selected` flag — see `DiagramGraphFallback`'s ledger docstring for why an unreflected selection is
 * an erased one. Exported for `🧪️tests/🫱️interaction-publication`. */
export function workflowNodesToDiagramNodes(records: readonly NodeGraphNodeRecord[], selectedNodeIds: readonly string[] = []): Node<WorkflowNodeData>[] {
  const selected = new Set(selectedNodeIds);
  return records.map((record) => ({
    id: record.id,
    type: "workflow",
    selected: selected.has(record.id),
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

/** ⎋️ `Escape` on a graph surface clears the FRAMEWORK selection — `clearSelection`, one of the six
 * reserved interaction verbs every app answers, which is what retires the mark the Artifact panel's
 * outline rows and the canvas both paint.
 *
 * It used to dispatch `setMediaNodeSelection`, an action builder ticket
 * 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM DELETED when selection became framework-owned
 * (`space_interaction_select`'s own doc names it among the four it replaced). No window kind declares
 * it any more, so the shell dropped every press — measured on generation3d 6018,
 * `🗑️generated/react-reds/outline-selection/console.txt`: `dropped action "setMediaNodeSelection"
 * dispatched from window kind "procedural-main"` between the last arrow and the re-entry, with the
 * previously selected outline row still `aria-selected="true"` afterwards. */
/** ♿️ What a canvas graph surface lends its host's keyboard law: the selection it is PAINTING (the dag
 * engine keeps a domain-less graph's selection in its own session, exactly as a pointer pick leaves it),
 * and the two writes a key press makes — a focus highlight and a selection — which the surface applies to
 * its session and then publishes through its OWN `emitInteractionState`, the lane a pointer pick uses. */
type GraphKeyboardPort = {
  readonly selectedIds: () => readonly string[];
  readonly focus: (nodeId: string) => void;
  readonly select: (nodeIds: readonly string[], focusedId: string | null) => void;
};

/** ♿️ The canvas node-graph surfaces' side of `🕸️Diagram`'s keyboard law: the SAME
 * {@link diagramKeyboardStep} decides, the surface's {@link GraphKeyboardPort} applies and publishes it
 * like a pointer pick, and the SAME localized sentence ({@link diagramKeyboardAnnouncement}) is spoken.
 * A key the React Flow fallback already handled (`defaultPrevented`) is left alone so a graph never takes
 * two steps for one press. */
function handleGraphKeyboard(
  event: KeyboardEvent<HTMLDivElement>,
  editable: boolean,
  scene: NodeGraphScene,
  focusedId: string | null,
  port: GraphKeyboardPort | null,
  dispatch: (action: string, args?: Record<string, unknown>) => void,
  onStep: (focusedId: string | null, announcement: string) => void,
  translate: ReturnType<typeof useDiagramTranslate>,
) {
  if (event.defaultPrevented || isEditableGraphKeyTarget(event.target)) return;
  const nodes = (scene.nodes ?? []).map((record) => ({ id: record.id, position: { x: record.x, y: record.y } }));
  const step = diagramKeyboardStep(nodes, focusedId, port?.selectedIds() ?? scene.selection ?? [], event.key, event.shiftKey, editable);
  if (!step) return;
  event.preventDefault();
  const labelOf = (id: string) => (scene.nodes ?? []).find((record) => record.id === id)?.label || id;
  const announcement = diagramKeyboardAnnouncement(step, nodes, labelOf, translate);
  if (step.kind === "focus") {
    if (port) port.focus(step.focusedId);
    else publishNodeGraphHover(dispatch, scene.interactionDomain, step.focusedId);
    onStep(step.focusedId, announcement);
    return;
  }
  if (step.kind === "clear") {
    port?.select([], null);
    dispatch(nodeGraphActions.clearSelection);
    onStep(null, announcement);
    return;
  }
  if (port) port.select(step.selectedIds, step.focusedId);
  else if (step.selectedIds.length === 0) dispatch(nodeGraphActions.clearSelection);
  else publishNodeGraphSelection(dispatch, scene.interactionDomain, { nodeIds: [...step.selectedIds] });
  onStep(step.focusedId, announcement);
}
//#endregion Keyboard

//#region DiagramNode
function WorkflowDiagramNode({ data }: NodeProps<Node<WorkflowNodeData>>) {
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
/** 🔍️ One wheel notch of the node-graph engines' zoom — the schema tokens `camera.wheelZoomInFactor` /
 * `camera.wheelZoomOutFactor` that the flow host reads as `WHEEL_ZOOM_IN_FACTOR`/`WHEEL_ZOOM_OUT_FACTOR`
 * and the dag `GraphHost::plan_wheel` applies as the same `1.1`/`0.9`: both engines zoom by a fixed factor
 * per wheel event whatever its delta, so a pinch reaches them as whole notches. */
export const GRAPH_WHEEL_ZOOM_NOTCH = { in: STYLING_METRICS.camera.wheelZoomInFactor, out: STYLING_METRICS.camera.wheelZoomOutFactor } as const;

/** 🤏️ One `wheelScreen(sx, sy, deltaX, deltaY, zoomGesture)` call a node-graph surface replays for a pinch. */
export type GraphPinchWheelCall = { readonly sx: number; readonly sy: number; readonly deltaX: number; readonly deltaY: number; readonly zoomGesture: boolean };

/**
 * 🤏️ Translates one shared-recognizer {@link PinchStep} into a node-graph session's existing wheel lane —
 * the ONE pinch path of the dag surface and the flow surface: whole zoom notches anchored at the
 * centroid (the sub-notch remainder is carried in `pendingLogScale`), then the centroid travel as one
 * non-zoom wheel, which both engines apply as `camera -= delta / zoom` so the graph stays under the
 * fingers — horizontally too: the dag `GraphHost::plan_wheel` pans by `deltaX / zoom` like the flow engine.
 */
export function graphPinchWheelPlan(step: PinchStep, pendingLogScale: number): { readonly calls: readonly GraphPinchWheelCall[]; readonly pendingLogScale: number } {
  const zoom = pinchZoomNotches(pendingLogScale, step.scale, GRAPH_WHEEL_ZOOM_NOTCH);
  const calls: GraphPinchWheelCall[] = Array.from({ length: Math.abs(zoom.notches) }, () => ({ sx: step.centroidX, sy: step.centroidY, deltaX: 0, deltaY: zoom.notches > 0 ? -1 : 1, zoomGesture: true }));
  if (step.panX !== 0 || step.panY !== 0) calls.push({ sx: step.centroidX, sy: step.centroidY, deltaX: step.panX, deltaY: step.panY, zoomGesture: false });
  return { calls, pendingLogScale: zoom.pendingLogScale };
}

function WasmGraphSurface({
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
  const { sliderLane, beginSliderGesture } = useGraphSliderLanes(surfaceId, dispatchRef);
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
    try {
      container.setAttribute("data-viewport-camera-json", JSON.stringify(session.viewport()));
      container.setAttribute("data-session-selection-json", session.selectedNodeIdsJson());
    } catch {
      /* session not ready */
    }
  }, []);

  useEffect(() => {
    try {
      sessionRef.current?.syncFromScenePack?.(scenePack);
      paintOverlays();
    } catch (error) {
    }
  }, [scenePack, paintOverlays]);

  const onSessionReady = useCallback(
    (session: GraphWasmSession) => {
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
      hoveredChannelJson: () => "{}",
      syncInteraction: () => {},
      viewport: () => scene.viewport ?? DEFAULT_NODE_GRAPH_VIEWPORT,
      pickTargetsAtScreenJson: () => "[]",
      setHover: () => {},
      setHoverChannel: () => {},
      alignSelection: () => {},
      hostSnapshotJson: () => "{}",
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
    } catch {
      /* session not ready */
    }
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

  const commitGraphFixture = useCallback(() => {
    const session = sessionRef.current;
    if (!session?.hostSnapshotJson) return;
    try {
      const hostSnapshotJson = session.hostSnapshotJson();
      dispatch(nodeGraphActions.edit, { operations: [{ operation: "setHostSnapshot", hostSnapshotJson }] });
    } catch {
      /* session not ready */
    }
  }, [dispatch]);

  /** 🕹️ The graph's own EDITED shape — node ids/positions and wire endpoints, and deliberately not the
   * camera, the selection or the hover. A gesture that only panned, zoomed or picked leaves this
   * string identical, which is what lets a pointer-up commit be sent ONLY when the user really changed
   * the graph (ticket 26/09/18/EXTRACT-WFC-PLUGIN: until this existed, dragging a node or drawing a
   * wire on the wasm node-graph surface reached no plugin at all — `commitGraphFixture` was called
   * from the align chrome and nowhere else, so every drag was silently discarded on pointer-up). */
  const graphEditSignature = useCallback((): string | null => {
    const session = sessionRef.current;
    if (!session?.hostSnapshotJson) return null;
    try {
      const snapshot = JSON.parse(session.hostSnapshotJson()) as {
        readonly nodes?: readonly { readonly id?: string; readonly x?: number; readonly y?: number }[];
        readonly edges?: readonly { readonly id?: string; readonly source?: string; readonly target?: string }[];
      };
      const nodes = (snapshot.nodes ?? []).map((entry) => [entry.id, entry.x, entry.y]);
      const edges = (snapshot.edges ?? []).map((entry) => [entry.id, entry.source, entry.target]);
      return JSON.stringify([nodes, edges]);
    } catch {
      return null;
    }
  }, []);

  /** 🕹️ The signature captured at pointer-DOWN, so a commit is decided against the graph as it was
   * before THIS gesture rather than against whatever the session laid out at load. */
  const gestureSignatureRef = useRef<string | null>(null);

  const commitGraphFixtureIfEdited = useCallback(() => {
    const before = gestureSignatureRef.current;
    gestureSignatureRef.current = null;
    if (before === null) return;
    const after = graphEditSignature();
    if (after === null || after === before) return;
    commitGraphFixture();
  }, [commitGraphFixture, graphEditSignature]);

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
        className="absolute inset-0 z-30 touch-none"
        data-gesture-surface="dag"
        onPointerDown={(event) => {
          const rect = event.currentTarget.getBoundingClientRect();
          const verdict = gestureRecognizer.down({ pointerId: event.pointerId, x: event.clientX - rect.left, y: event.clientY - rect.top });
          if (verdict.kind === "pinchBegin") {
            const session = sessionRef.current;
            gestureSignatureRef.current = null;
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
          gestureSignatureRef.current = graphEditSignature();
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
          commitGraphFixtureIfEdited();
        }}
        onPointerCancel={(event) => {
          const session = sessionRef.current;
          const verdict = gestureRecognizer.up(event.pointerId);
          if (verdict.kind === "pinchEnd") {
            pinchLogScaleRef.current = 0;
            emitInteractionState();
          }
          if (verdict.kind !== "single") return;
          gestureSignatureRef.current = null;
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
            commitGraphFixture();
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
        onSliderPointerDown={beginSliderGesture}
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
  /** 🎯️ The selection the PLUGIN holds, as this surface's own baseline. React Flow fires
   * `onSelectionChange` once at mount with an empty list, and publishing that emptiness is how a
   * selection dispatched a few milliseconds BEFORE this surface existed was wiped: measured on
   * generation3d, `interactionSelect targets:[] domainId:graph` left this component 15 ms before
   * `node-graph host mount`, with no pointer event anywhere in the run
   * (`📓️hot-swap-board-remount-2026-09-15.md` §2). Adopted, never published — the ledger's own rule for
   * a mark that arrived FROM the plugin — and reflected onto the mounted nodes, so a selection made
   * before mount is APPLIED at mount instead of dropped. */
  const sceneSelection = useMemo(() => [...(scene.selection ?? [])], [scene.selection]);
  const interactionLedger = useMemo(() => createNodeGraphInteractionLedger(), []);
  const adoptedSelectionRef = useRef<string | null>(null);
  const sceneSelectionKey = nodeGraphSelectionMarkKey({ nodeIds: sceneSelection });
  if (adoptedSelectionRef.current !== sceneSelectionKey) {
    adoptedSelectionRef.current = sceneSelectionKey;
    interactionLedger.adoptSelection({ nodeIds: sceneSelection });
  }
  const initialNodes = useMemo(() => workflowNodesToDiagramNodes(parsedNodes, sceneSelection), [parsedNodes, sceneSelection]);
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
              menu: { id: "nodeGraph", args: null },
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
        isValidConnection={(connection) => nodeGraphConnectionIsValid(parsedNodes, connection)}
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
                if (!nodeGraphConnectionIsValid(parsedNodes, connection)) {
                  return;
                }
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
          if (interactionLedger.publishSelection({ nodeIds: [clickedNode.id] })) publishNodeGraphSelection(dispatch, scene.interactionDomain, { nodeIds: [clickedNode.id] });
        }}
        onNodeDoubleClick={(_event, clickedNode) => {
          const record = parsedNodes.find((entry) => entry.id === clickedNode.id);
          if (record?.instanceId) dispatch("openInstance", { instanceId: record.instanceId });
        }}
        onSelectionChange={(selection) => {
          const nodeIds = selection.nodes.map((entry) => entry.id);
          if (interactionLedger.publishSelection({ nodeIds })) publishNodeGraphSelection(dispatch, scene.interactionDomain, { nodeIds });
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
// 🚪️ Exported (unlike the rest of this Helpers subregion) — `TextEditorHost` in the sibling `✏️TextEditor`
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
/**
 * 🪜️ The graph host's root class. `isolate` is load-bearing: it keeps this host's OWN layer numbers —
 * the full-bleed pointer overlay at `z-30`, the label canvas at `z-40`, the marquee at `z-50` — inside
 * its own stacking context.
 *
 * Without it those raw numbers competed directly with the shell's z ladder (`--z-pane: 20`,
 * `🖌️ui/🎨️.css`), so the pointer overlay painted ON TOP of the window's floating pane chrome and ate
 * every click on `Window Options` (which is where the LOD picker lives), `Actions` and `Utilities`.
 * The chips stayed visible and focusable and were completely dead to a pointer, while the same chips
 * on a sibling `World3dHost` window worked — that host has no such overlay. Measured on the React
 * serve 2026-09-14: `elementsFromPoint` over the chip's centre answered the graph overlay, and a
 * programmatic `.click()` opened the rail the pointer could not reach
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
 */
export const NODE_GRAPH_HOST_CLASS = "semio-node-graph-host isolate relative h-full min-h-0 w-full overflow-hidden";

/** 🎯️ What THIS graph surface has selected, hovered and highlighted, exactly as the guest published it
 * in `NodeGraphScene` — the graph twin of `🌐️World3dHost`'s `data-selection-json`.
 *
 * A node picked from the Artifact panel's outline row, from the canvas, or by a traversal chord is one
 * and the same `graph`-domain selection, and until this existed the ONLY surface in the shell that
 * published a selection lane was the 3D pane — whose ids are mesh ids (`extrude@solid`), never node ids.
 * So "selecting through the document panel does not select" could not be told from "the surface that
 * holds the selection never says so": measured on generation3d 6018, clicking
 * `panel:procedural-play-graph/height` marked its row and invoked `interactionSelect`, while the only
 * `[data-selection-json]` in the document was the preview's, with `selectedIds: []`
 * (`🗑️generated/react-reds/recon/recon.json` `doc-panel-select`). */
export function nodeGraphSurfaceSelectionDomV1(scene: NodeGraphScene | undefined): Record<string, unknown> {
  const hover = scene?.hover;
  return {
    selectedIds: [...(scene?.selection ?? [])],
    highlightedIds: [...(scene?.highlighted ?? [])],
    hoverTarget: hover?.nodeId ? { nodeId: hover.nodeId, portId: hover.portId ?? null } : null,
    editable: scene?.editable ?? true,
  };
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
  const graphLabel = useLabel("ui.diagram.label");
  const graphRoleDescriptionLabel = useLabel("ui.diagram.roleDescription");
  const graphKeyboardHelpLabel = useLabel("ui.diagram.keyboardHelp");
  const graphNodesLabel = useLabel("ui.diagram.nodes");
  const graphEdgesLabel = useLabel("ui.diagram.edges");
  const graphKeyboardHelpId = React.useId();
  const diagramTranslate = useDiagramTranslate();
  const [keyboardFocusedNodeId, setKeyboardFocusedNodeId] = useState<string | null>(null);
  const keyboardPortRef = useRef<GraphKeyboardPort | null>(null);
  const [keyboardAnnouncement, setKeyboardAnnouncement] = useState("");
  const liveKeyboardFocusedNodeId = keyboardFocusedNodeId !== null && parsedNodes.some((record) => record.id === keyboardFocusedNodeId) ? keyboardFocusedNodeId : null;

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
    publishNodeGraphSelection(dispatch, scene?.interactionDomain, { nodeIds: [mediaNode.id] });
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

  const useFlowEngine = isFlowGraphScene(scene.capabilitiesJson) || Boolean(scene.hostSnapshotJson);

  return (
    <div
      className={NODE_GRAPH_HOST_CLASS}
      data-surface-id={node.surfaceId}
      data-status-json={scene.statusJson ?? undefined}
      data-host-snapshot-json={scene.hostSnapshotJson ?? undefined}
      data-selection-json={JSON.stringify(nodeGraphSurfaceSelectionDomV1(scene))}
      data-diagram-focused-node={liveKeyboardFocusedNodeId ?? undefined}
      role="application"
      tabIndex={0}
      aria-roledescription={graphRoleDescriptionLabel}
      aria-label={`${graphLabel} — ${parsedNodes.length} ${graphNodesLabel}, ${parsedEdges.length} ${graphEdgesLabel}`}
      aria-describedby={graphKeyboardHelpId}
      onKeyDown={(event) =>
        handleGraphKeyboard(event, editable, scene, liveKeyboardFocusedNodeId, keyboardPortRef.current, dispatch, (focusedId, announcement) => {
          setKeyboardFocusedNodeId(focusedId);
          setKeyboardAnnouncement(announcement);
        }, diagramTranslate)
      }
    >
      <span id={graphKeyboardHelpId} className="sr-only">
        {graphKeyboardHelpLabel}
      </span>
      <DiagramLiveRegion text={keyboardAnnouncement} />
      {isClient ? (
        useFlowEngine ? (
          <FlowGraphCanvasHost scene={scene} surfaceId={node.surfaceId} controllerId={node.controllerId} editable={editable} requestContextMenu={requestContextMenu} onAction={onAction} keyboardPort={keyboardPortRef} />
        ) : (
          <WasmGraphSurface scene={scene} surfaceId={node.surfaceId} controllerId={node.controllerId} editable={editable} requestContextMenu={requestContextMenu} onAction={onAction} keyboardPort={keyboardPortRef} />
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
  /** 📐️ The caption's own screen-width budget, published by the host: only it knows whether a
   * caption sits inside the node body or above it. */
  readonly maxScreenW?: number;
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
  /** ✨️ Extra ids — nodes, edges, or `"{nodeId}@{portId}"` ports — painted highlighted regardless
   * of preselection, e.g. `NodeGraphScene.highlighted` pushed in by a plugin. */
  readonly highlightIds?: readonly string[];
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
  readonly label: string;
  readonly value: number;
  readonly min: number;
  readonly max: number;
  readonly step: number;
  readonly x: number;
  readonly y: number;
  readonly w: number;
  readonly h: number;
  /** 🔢 Readout typography the board publishes alongside the row, in SCREEN pixels — the overlay
   * divides by zoom because its wrapper is already scaled. Absent on a board that predates it. */
  readonly fontScreenPx?: number;
  readonly gapScreenPx?: number;
};

/** 🔢 The ONE text a published slider value is read as — the readout beside the track and the
 * `aria-valuenow` a screen reader announces are this same number, never two. */
export function dagSliderValueText(value: number): string {
  return value.toFixed(1);
}

/** 📐️ A screen-pixel metric the board published, or the overlay's own floor when the board is older
 * than the field — never `NaN`, which would silently drop the style it lands in. */
function screenMetric(published: number | undefined, fallback: number): number {
  return typeof published === "number" && Number.isFinite(published) && published > 0 ? published : fallback;
}

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

//#region 📷️ContentFraming
/** 🖼️ Screen margin a fit leaves around the graph it frames, per side. Twin of
 * `canvas::camera::CONTENT_FIT_PADDING_PX`. */
export const DAG_CONTENT_FIT_PADDING_PX = 24;
/** 🖼️ How much of the graph a STORED camera must already show to be adopted on a first attach. */
export const DAG_CONTENT_FRAMED_MIN_COVERAGE = 0.85;
/** 🖼️ How little of the graph has to be left on screen before a graph that CHANGED under a live
 * camera (an example switch) is re-fitted. Far below {@link DAG_CONTENT_FRAMED_MIN_COVERAGE}: a
 * camera the viewer set is never yanked back for an ordinary edit. */
export const DAG_CONTENT_REFIT_MAX_COVERAGE = 0.05;
const DAG_CAMERA_ZOOM_MIN = 0.05;
const DAG_CAMERA_ZOOM_MAX = 32;

export type DagContentBounds = { readonly minX: number; readonly minY: number; readonly maxX: number; readonly maxY: number };

/** 🖼️ World bounds of every node a scene carries — what a fit frames. `null` for an empty graph,
 * which has nothing to frame. Twin of `DagHost::content_world_bounds`. */
export function dagContentBounds(nodes: readonly NodeGraphNodeRecord[] | undefined): DagContentBounds | null {
  let bounds: { minX: number; minY: number; maxX: number; maxY: number } | null = null;
  for (const node of nodes ?? []) {
    const halfW = (Number(node.width) || 0) * 0.5;
    const halfH = (Number(node.height) || 0) * 0.5;
    const x = Number(node.x) || 0;
    const y = Number(node.y) || 0;
    if (!Number.isFinite(x) || !Number.isFinite(y)) continue;
    bounds = bounds
      ? { minX: Math.min(bounds.minX, x - halfW), minY: Math.min(bounds.minY, y - halfH), maxX: Math.max(bounds.maxX, x + halfW), maxY: Math.max(bounds.maxY, y + halfH) }
      : { minX: x - halfW, minY: y - halfH, maxX: x + halfW, maxY: y + halfH };
  }
  return bounds;
}

/** 🔖️ Identity of the LAYOUT a camera was framed against: the node ids and their boxes, nothing
 * else. An example switch changes it; hovering, evaluating or selecting never does, which is what
 * keeps a re-fit from firing on an ordinary edit. */
export function nodeGraphContentSignature(nodes: readonly NodeGraphNodeRecord[] | undefined): string {
  return (nodes ?? []).map((node) => `${node.id}:${node.x},${node.y},${node.width},${node.height}`).join("|");
}

/** 🖼️ Fraction (`0..1`) of the graph's own area a camera currently shows. Degenerate content (a
 * single node, a row of nodes at one `y`) is measured on whichever axes have extent, so a
 * zero-height graph never reads as invisible. Twin of `canvas::camera::content_coverage`. */
export function dagContentCoverage(content: DagContentBounds, camera: DagCameraState, width: number, height: number): number {
  const zoom = Math.max(camera.zoom, 1e-9);
  const halfW = Math.max(width, 1) / (2 * zoom);
  const halfH = Math.max(height, 1) / (2 * zoom);
  const view = { minX: camera.x - halfW, minY: camera.y - halfH, maxX: camera.x + halfW, maxY: camera.y + halfH };
  const contentW = Math.max(content.maxX - content.minX, 0);
  const contentH = Math.max(content.maxY - content.minY, 0);
  const overlapX = Math.max(Math.min(content.maxX, view.maxX) - Math.max(content.minX, view.minX), 0);
  const overlapY = Math.max(Math.min(content.maxY, view.maxY) - Math.max(content.minY, view.minY), 0);
  const insideX = content.minX >= view.minX && content.maxX <= view.maxX;
  const insideY = content.minY >= view.minY && content.maxY <= view.maxY;
  if (contentW > 0 && contentH > 0) return (overlapX * overlapY) / (contentW * contentH);
  if (contentW > 0) return (insideY ? 1 : 0) * (overlapX / contentW);
  if (contentH > 0) return (insideX ? 1 : 0) * (overlapY / contentH);
  return insideX && insideY ? 1 : 0;
}

/** 🖼️ The camera that frames the whole graph with `paddingPx` of screen margin per side, zoom
 * clamped to the canvas camera range. Twin of `canvas::camera::fit_camera`. */
export function dagFitCamera(content: DagContentBounds, width: number, height: number, paddingPx = DAG_CONTENT_FIT_PADDING_PX): DagCameraState {
  const usableW = Math.max(Math.max(width, 1) - paddingPx * 2, 1);
  const usableH = Math.max(Math.max(height, 1) - paddingPx * 2, 1);
  const contentW = Math.max(content.maxX - content.minX, 0);
  const contentH = Math.max(content.maxY - content.minY, 0);
  const zoomX = contentW > 0 ? usableW / contentW : Number.POSITIVE_INFINITY;
  const zoomY = contentH > 0 ? usableH / contentH : Number.POSITIVE_INFINITY;
  const zoom = Math.min(zoomX, zoomY);
  return {
    x: (content.minX + content.maxX) * 0.5,
    y: (content.minY + content.maxY) * 0.5,
    zoom: Math.min(Math.max(Number.isFinite(zoom) ? zoom : 1, DAG_CAMERA_ZOOM_MIN), DAG_CAMERA_ZOOM_MAX),
  };
}

/** 🖼️ The camera a node-graph surface OPENS on — the law this window used to lack. A stored camera
 * is honoured only when it already frames the graph it was stored for; otherwise (and whenever
 * there is no stored camera) the graph is fitted, and `fitted` tells the caller to persist that the
 * same way it persists a pan or a zoom gesture. Twin of `canvas::camera::startup_camera`. */
export function dagStartupCamera(
  stored: DagCameraState | null | undefined,
  content: DagContentBounds | null,
  width: number,
  height: number,
  minCoverage = DAG_CONTENT_FRAMED_MIN_COVERAGE,
): { readonly camera: DagCameraState; readonly fitted: boolean } {
  if (!content) return { camera: stored ?? { x: 0, y: 0, zoom: 1 }, fitted: false };
  if (stored && stored.zoom > 0 && dagContentCoverage(content, stored, width, height) >= minCoverage) return { camera: stored, fitted: false };
  return { camera: dagFitCamera(content, width, height), fitted: true };
}
//#endregion 📷️ContentFraming

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

export function dagElementInteractionChrome(
  selectionIds: Iterable<string>,
  preselection: DagPreselectSnapshot,
  extraHighlightIds: Iterable<string> = [],
): { readonly selectedIds: Set<string>; readonly highlightedIds: Set<string> } {
  const base = !preselection.ids.length && !preselection.removedIds.length
    ? { selectedIds: new Set(selectionIds), highlightedIds: new Set<string>() }
    : { selectedIds: new Set(preselection.ids), highlightedIds: new Set(preselection.removedIds) };
  for (const id of extraHighlightIds) base.highlightedIds.add(id);
  return base;
}

export function parseDagLabelRows(stateJson: string): DagLabelOverlayRow[] {
  try {
    const parsed = JSON.parse(stateJson) as {
      readonly labels?: readonly Record<string, unknown>[];
      readonly rows?: readonly Record<string, unknown>[];
    };
    const raw = parsed.labels ?? parsed.rows ?? [];
    return raw
      .map((row): DagLabelOverlayRow | null => {
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

/** ✂️ The one glyph a clipped caption ends on — the same character every presentation of this repo
 * appends (`canvas::text::LABEL_ELLIPSIS`). */
export const DAG_LABEL_ELLIPSIS = "…";
/** 🔠️ Smallest font an overlay caption may shrink to. Below this a caption is a smudge, not a word,
 * which is why width is answered by {@link dagEllipsizeByMeasure} and never by shrinking further. */
const DAG_LABEL_LEGIBLE_MIN_PX = 8;

/** ✂️ Longest prefix of `text` that still fits `maxWidth` once {@link DAG_LABEL_ELLIPSIS} is
 * appended, measured by the CALLER's own measure. The JavaScript twin of
 * `canvas::text::ellipsize_by_measure`; both are pinned to the rows of
 * `♾️infinite/🖼️canvas/🧫️fixtures/🏷️label-fit/🔣️.json`.
 *
 * Empty text and a non-positive budget draw nothing; a budget too narrow for even one glyph plus
 * the ellipsis draws the bare ellipsis, so a clipped caption is always visibly clipped. */
export function dagEllipsizeByMeasure(text: string, maxWidth: number, measure: (candidate: string) => number): string {
  const trimmed = text.trim();
  if (!trimmed || maxWidth <= 0) return "";
  if (measure(trimmed) <= maxWidth) return trimmed;
  const glyphs = [...trimmed];
  let best = "";
  for (let index = 1; index <= glyphs.length; index += 1) {
    const candidate = glyphs.slice(0, index).join("") + DAG_LABEL_ELLIPSIS;
    if (measure(candidate) > maxWidth) break;
    best = candidate;
  }
  return best || DAG_LABEL_ELLIPSIS;
}

/** ✂️ {@link dagEllipsizeByMeasure} driven by the overlay canvas's own `measureText` at `fontPx`. */
function dagEllipsizeOverlayLabel(ctx: CanvasRenderingContext2D, text: string, fontPx: number, maxW: number): string {
  ctx.font = `${fontPx}px ${DAG_LABEL_FONT_FAMILY}`;
  return dagEllipsizeByMeasure(text, maxW, (candidate) => ctx.measureText(candidate).width);
}

/** 📐️ Overlay caption font size: the row's own target, shrunk ONLY to fit the row's height, never
 * below {@link DAG_LABEL_LEGIBLE_MIN_PX}. Width is not a font decision — see
 * {@link dagEllipsizeOverlayLabel}. This used to binary-search the font down to 4px on width too,
 * which is how a caption wider than its node body became an unreadable smear. */
function dagClampLabelFontPx(ctx: CanvasRenderingContext2D, text: string, targetPx: number, maxH: number): number {
  const px = Math.max(DAG_LABEL_LEGIBLE_MIN_PX, Math.round(targetPx));
  ctx.font = `${px}px ${DAG_LABEL_FONT_FAMILY}`;
  if (px * 1.2 <= maxH) {
    return px;
  }
  let low = DAG_LABEL_LEGIBLE_MIN_PX;
  let high = px;
  let best = DAG_LABEL_LEGIBLE_MIN_PX;
  while (low <= high) {
    const mid = Math.floor((low + high) / 2);
    if (mid * 1.2 <= maxH) {
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
    const parsed = JSON.parse(stateJson) as { readonly sliders?: unknown } | null;
    if (!Array.isArray(parsed?.sliders)) return [];
    const ids = new Set<string>();
    return parsed.sliders.filter((value): value is DagSliderOverlayRow => {
      if (!value || typeof value !== "object") return false;
      const row = value as Partial<DagSliderOverlayRow>;
      if (typeof row.widgetId !== "string" || !row.widgetId.trim() || ids.has(row.widgetId) || typeof row.label !== "string" || !row.label.trim()) return false;
      if (![row.value, row.min, row.max, row.step, row.x, row.y, row.w, row.h].every((number) => typeof number === "number" && Number.isFinite(number))) return false;
      if (row.min! > row.max! || row.step! <= 0 || row.w! <= 0 || row.h! <= 0) return false;
      ids.add(row.widgetId);
      return true;
    });
  } catch {
    return [];
  }
}

/** @emoji 🔬️ One graph surface's geometry read-back, published as `window.__semioFlowGraphProbe[surfaceId]`.
 * `entity` is {@link dagIntroductionResolver}'s own resolver — `"node"`, `"handle"` (a port), `"edge"`
 * and `"slider"` all resolve to viewport pixels — so a scripted caller can aim a pointer gesture at a
 * node or a port on a canvas that paints itself and has no per-entity DOM. */
export type FlowGraphSurfaceProbe = {
  readonly entity: (domain: string, id: string) => IntroductionResolvedGeometry | null;
  /** 🪪️ The node ids this surface is actually painting — the input `entity("node", id)` needs, and the
   * only one a caller can get: `hostSnapshotJson` is a scene field the plugin may not carry at all. */
  readonly nodeIds: () => readonly string[];
  /** 🗺️ Where this surface is painting each node, in the graph's own world units — the read a caller
   * needs to answer "did a reorganize move anything". Same reason as {@link nodeIds}: it comes off the
   * scene the surface holds, not off a snapshot field the plugin may never send. */
  readonly nodeLayout: () => Readonly<Record<string, { readonly x: number; readonly y: number }>>;
  readonly viewport: () => { readonly x: number; readonly y: number; readonly zoom: number };
  readonly hostSnapshotJson: () => string | null;
  readonly rect: () => { readonly x: number; readonly y: number; readonly width: number; readonly height: number } | null;
};

/** @emoji 🎯️ The subset of `FlowWasmSession`/`FrameworkGraphSession` {@link dagIntroductionResolver} needs
 * — factored out because both session interfaces expose the same overlay/entity JSON shape and both host
 * components (`FlowGraphCanvasHost`, `WasmGraphSurface`) register the identical resolver logic. */
type DagIntroductionSession = {
  readonly labelOverlayPaintStateJson: () => string | FlowTask<unknown>;
  readonly sliderOverlayStateJson: () => string | FlowTask<unknown>;
  readonly entityScreenJson?: (domain: string, id: string) => string | FlowTask<unknown>;
};

/** @emoji 🎯️ Builds the `IntroductionSurfaceResolver` for a dag-engine-backed graph surface. Reads the
 * session and container via refs (not React state) every call — cheap, and lets registration skip
 * re-running whenever the surface re-renders. `entity`'s `"slider"` domain resolves entirely from the
 * already-fetched `sliderOverlayStateJson()` (no Rust round trip); every other domain (`"node"`,
 * `"handle"`, `"edge"`) goes through `entityScreenJson`, added to the dag engine specifically for this. */
function dagIntroductionResolver(sessionRef: React.RefObject<DagIntroductionSession | null>, containerRef: React.RefObject<HTMLElement | null>): IntroductionSurfaceResolver {
  const cache = new Map<string, string>();
  const active = new Map<string, FlowTask<unknown>>();
  const read = (key: string, value: string | FlowTask<unknown>): string | undefined => {
    if (typeof value === "string") return value;
    active.get(key)?.cancel();
    active.set(key, value);
    const unsubscribe = value.subscribe(() => {});
    void value.result.then((result) => cache.set(key, flowJsonText(result))).catch(() => {}).finally(() => {
      unsubscribe();
      if (active.get(key) === value) active.delete(key);
    });
    return cache.get(key);
  };
  return {
    canvasPoint: (x, y) => {
      const session = sessionRef.current;
      const container = containerRef.current;
      if (!session || !container) return null;
      const rect = container.getBoundingClientRect();
      const cameraJson = read("labels", session.labelOverlayPaintStateJson());
      if (!cameraJson) return null;
      const camera = parseDagOverlayCamera(cameraJson);
      const screen = dagWorldToScreen(camera, rect.width, rect.height, x, y);
      return { x: rect.left + screen.x, y: rect.top + screen.y, visible: true };
    },
    entity: (domain, entityId): IntroductionResolvedGeometry | null => {
      const session = sessionRef.current;
      const container = containerRef.current;
      if (!session || !container) return null;
      const rect = container.getBoundingClientRect();
      if (domain === "slider") {
        const slidersJson = read("sliders", session.sliderOverlayStateJson());
        if (!slidersJson) return null;
        const sliders = parseDagSliderOverlays(slidersJson);
        const slider = entityId === "*" ? sliders[0] : sliders.find((row) => row.widgetId === entityId);
        if (!slider) return null;
        const cameraJson = read("labels", session.labelOverlayPaintStateJson());
        if (!cameraJson) return null;
        const camera = parseDagOverlayCamera(cameraJson);
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
        const geometryJson = read(`entity:${domain}:${entityId}`, session.entityScreenJson(domain, entityId));
        if (!geometryJson) return null;
        const geometry = JSON.parse(geometryJson) as {
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

/** 🖼️ Sizes one canvas' backing store to `logicalW`×`logicalH` at `dpr` and returns whether the store
 * actually changed. Deliberately synchronous and free of any animation frame: a hidden/background tab
 * never fires `requestAnimationFrame`, so a canvas whose size is only reached from a rAF tick stays at
 * the HTML default 300×150 forever (`📓️runtime-verification-2026-09-09.md` boot #3). */
export function resizeCanvasBackingStore(canvas: HTMLCanvasElement | null | undefined, logicalW: number, logicalH: number, dpr: number): boolean {
  if (!canvas) return false;
  const pixelW = Math.max(1, Math.round(logicalW * dpr));
  const pixelH = Math.max(1, Math.round(logicalH * dpr));
  const changed = canvas.width !== pixelW || canvas.height !== pixelH;
  if (changed) {
    canvas.width = pixelW;
    canvas.height = pixelH;
  }
  canvas.style.width = `${Math.max(1, Math.round(logicalW))}px`;
  canvas.style.height = `${Math.max(1, Math.round(logicalH))}px`;
  return changed;
}

/** @emoji 🏷️ Paints the node captions over the engine canvas. Captions are centred on the MEASURED overlay
 * size, never on the session's reported `width`/`height`: the overlay is painted when the session is handed
 * over — before the engine canvas is attached and sized — and on later scene or interaction changes only,
 * so a static graph kept the 1×1 size the session reported at hand-over and drew every caption half a
 * canvas away from its node (dag, trinity-jack and mathematical play panes, 2026-09-23). */
export function paintDagLabelOverlays(stateJson: string, canvas: HTMLCanvasElement, logicalW: number, logicalH: number, dpr: number, interaction: DagLabelOverlayInteraction): void {
  let state: { readonly camera?: DagCameraState; readonly width?: number; readonly height?: number; readonly labels?: readonly DagLabelOverlayRow[] };
  try {
    state = JSON.parse(stateJson) as typeof state;
  } catch {
    return;
  }
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  resizeCanvasBackingStore(canvas, logicalW, logicalH, dpr);
  ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
  ctx.clearRect(0, 0, logicalW, logicalH);
  const zoom = Math.max(0.05, Number(state.camera?.zoom) || 1);
  const camera = {
    x: Number(state.camera?.x) || 0,
    y: Number(state.camera?.y) || 0,
    zoom,
  };
  const viewportW = logicalW;
  const viewportH = logicalH;
  const chrome = dagElementInteractionChrome(interaction.selectedIds, interaction.preselect, interaction.highlightIds ?? []);
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
    // 📐️ The host publishes the caption's own screen budget (`maxScreenW`) because only it knows
    // whether the caption sits INSIDE the node body or above it. The `nodeW` derivation is the
    // fallback for a row that predates that field.
    const publishedW = Number(row.maxScreenW);
    const maxW = Number.isFinite(publishedW) && publishedW > 0 ? publishedW : Math.max(4, Number(row.nodeW) * zoom * inset);
    const maxH = Math.max(4, isPort && Number.isFinite(Number(row.maxScreenH)) && Number(row.maxScreenH) > 0 ? Number(row.maxScreenH) : Number(row.nodeH) * zoom * inset);
    const fontScreenPx = Number(row.fontScreenPx);
    const targetPx = Number.isFinite(fontScreenPx) && fontScreenPx > 0 ? fontScreenPx : DAG_LABEL_SCREEN_PX;
    const fontPx = dagClampLabelFontPx(ctx, row.text, targetPx, maxH);
    // ✂️ A rotated caption runs along the node's HEIGHT; the host already publishes that as its
    // `maxScreenW`, so the fallback is the only place the axis has to be chosen here.
    const textBudget = Number.isFinite(publishedW) && publishedW > 0 ? publishedW : row.layout === "vertical" ? Math.max(4, Number(row.nodeH) * zoom * inset) : maxW;
    const text = dagEllipsizeOverlayLabel(ctx, row.text, fontPx, textBudget);
    if (!text) continue;
    ctx.font = `${fontPx}px ${DAG_LABEL_FONT_FAMILY}`;
    ctx.fillStyle = dagOverlayLabelFillHex(row.id, row.ghost === true, interaction.hoveredId, chrome, dimmedIds);
    ctx.globalAlpha = row.ghost ? 0.85 : dimmedIds.includes(row.id) ? 0.5 : 1;
    if (row.layout === "vertical") {
      ctx.save();
      ctx.translate(anchor.x, anchor.y);
      ctx.rotate(-Math.PI / 2);
      ctx.textAlign = "center";
      ctx.textBaseline = "middle";
      ctx.fillText(text, 0, 0);
      ctx.restore();
    } else {
      const align = row.align === "left" || row.align === "right" ? row.align : "center";
      ctx.textAlign = align;
      ctx.textBaseline = "middle";
      ctx.fillText(text, anchor.x, anchor.y);
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

//#region 🎚️SliderGestureLanes
/** 🎚️ One coalescing lane per slider widget of a graph surface, plus the gesture identity its edits
 * fold under.
 *
 * A dragged inline slider produces ~60 values a second. Sending one `nodeGraphEdit` per value costs
 * one retained command, one document edit, one history entry and one preview re-evaluation EACH —
 * measured on 6018 as 24 `toolRunStart`s and 29 history entries for ONE one-second drag, with the
 * mesh arriving three seconds behind the thumb. The lane keeps only the value the user is on now and
 * sends it when the previous round trip has landed (`📓️slider-preview-update-2026-09-15.md`).
 *
 * The gesture id is minted on press and travels with every edit of that press, so the guest folds a
 * whole drag into ONE undoable edit and the next drag starts a new one.
 *
 * 🩸️ Before this, the overlay dispatched `setGraphParameter` — an action NO app declares. The shell
 * dropped every tick (`dropped action "setGraphParameter" … no window kind declares it`), so the knob
 * moved, the local flow session moved, and the document and the preview never did: the user's
 * "moving a slider doesn't update the preview" in one line.
 */
function useGraphSliderLanes(surfaceId: string, dispatchRef: React.RefObject<(action: string, args?: Record<string, unknown>) => void | Promise<void>>) {
  const gestureIdsRef = useRef(new Map<string, string>());
  const lanesRef = useRef(new Map<string, ContinuousGestureLane<number>>());
  const beginSliderGesture = useCallback((widgetId: string) => {
    gestureIdsRef.current.set(widgetId, `${surfaceId}:${widgetId}:${Date.now()}`);
  }, [surfaceId]);
  const sliderLane = useCallback(
    (widgetId: string) => {
      const existing = lanesRef.current.get(widgetId);
      if (existing) return existing;
      const lane = createContinuousGestureLane<number>({
        send: (value, phase) =>
          dispatchRef.current?.(nodeGraphActions.edit, {
            operations: [{ operation: "setSlider", widgetId, value, gesture: gestureIdsRef.current.get(widgetId) ?? `${surfaceId}:${widgetId}`, commit: phase === "commit" }],
          }),
        onFault: (error) => undefined,
      });
      lanesRef.current.set(widgetId, lane);
      return lane;
    },
    [dispatchRef, surfaceId],
  );
  return { sliderLane, beginSliderGesture };
}
//#endregion 🎚️SliderGestureLanes

//#region DagDomOverlays
export function GraphSliderOverlays({
  scopeId,
  stateJson,
  logicalW,
  logicalH,
  editable,
  onSliderChange,
  onSliderCommit,
  onSliderPointerDown,
  onSliderPointerUp,
  occluderRect = null,
}: {
  readonly scopeId: string;
  readonly stateJson: string;
  readonly logicalW: number;
  readonly logicalH: number;
  readonly editable: boolean;
  readonly onSliderChange: (widgetId: string, value: number) => void;
  readonly onSliderCommit?: (widgetId: string, value: number) => void;
  readonly onSliderPointerDown?: (widgetId: string) => void;
  readonly onSliderPointerUp?: (widgetId: string) => void;
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
        const h = Math.max(slider.h, 8 / zoom);
        return (
          <div
            key={slider.widgetId}
            className="pointer-events-auto absolute flex items-center"
            data-graph-slider-zoom={zoom}
            style={{ left: screen.x, top: screen.y, width: w, height: h, transform: `translate(-50%, -50%) scale(${zoom})`, transformOrigin: "center" }}
            onPointerDown={(event) => event.stopPropagation()}
          >
            <span
              aria-hidden="true"
              className="pointer-events-none absolute whitespace-nowrap tabular-nums"
              data-graph-slider-value={slider.widgetId}
              style={{ right: "100%", marginRight: screenMetric(slider.gapScreenPx, 4) / zoom, fontSize: screenMetric(slider.fontScreenPx, 10) / zoom, lineHeight: 1 }}
            >
              {dagSliderValueText(slider.value)}
            </span>
            <Slider
              id={`graph-slider-${encodeURIComponent(JSON.stringify([scopeId, slider.widgetId]))}`}
              aria-label={slider.label}
              className="h-full w-full min-w-0"
              thumbClassName="size-tiny"
              max={slider.max}
              min={slider.min}
              step={slider.step}
              value={[slider.value]}
              disabled={!editable}
              showValue={false}
              onValueChange={(values) => onSliderChange(slider.widgetId, values[0] ?? slider.value)}
              onPointerDown={() => onSliderPointerDown?.(slider.widgetId)}
              onPointerUp={() => onSliderPointerUp?.(slider.widgetId)}
              onPointerCancel={() => onSliderPointerUp?.(slider.widgetId)}
              onValueCommit={(values) => onSliderCommit?.(slider.widgetId, values[0] ?? slider.value)}
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
// @emoji 🎥️ The camera is NEVER copied from `scene.viewport` on a resync: live pan/zoom lives in the
// FlowWasmSession (and plugin runtime via `nodeGraphViewport`), while `scene.viewport` often lags.
// Applying it on hover/eval/edit-triggered synchronization would snap the camera; document
// preserves the live camera so fixture content reloads never reset the view. The ONE moment a
// stored camera is considered at all is the surface's first attach, and even there it is a
// decision, not a copy — see `applyFlowStartupCamera`.
const activeFlowTasks = new WeakMap<FlowWasmSession, Map<string, FlowTask<unknown>>>();

function observeFlowTask<T>(session: FlowWasmSession, feature: string, task: FlowTask<T>, consume?: (value: T) => void, settled?: (delivered: boolean) => void): () => void {
  let features = activeFlowTasks.get(session);
  if (!features) {
    features = new Map();
    activeFlowTasks.set(session, features);
  }
  features.get(feature)?.cancel();
  features.set(feature, task as FlowTask<unknown>);
  const unsubscribe = task.subscribe(() => {});
  let delivered = false;
  void task.result
    .then((value) => {
      delivered = true;
      consume?.(value);
    })
    .catch(() => {})
    .finally(() => {
      unsubscribe();
      settled?.(delivered);
      if (features?.get(feature) === task) features.delete(feature);
    });
  return () => {
    unsubscribe();
    task.cancel();
    if (features?.get(feature) === task) features.delete(feature);
  };
}

/** 🧊️ Whether an overlay read produced the same picture as the last pass. Every pass re-parses its
 * JSON into fresh objects, so a plain `setState` would change the reference 60 times a second during
 * a gesture and commit the surface's whole React subtree for a picture that did not move. */
function sameOverlayValue(left: unknown, right: unknown): boolean {
  return left === right || JSON.stringify(left ?? null) === JSON.stringify(right ?? null);
}

/** 🖱️ Issues one gesture STEP — a wheel tick, a pointer down/move/up, a slider write — and never
 * pre-empts the previous one.
 *
 * `observeFlowTask` keeps one task per feature key and cancels the previous, which is right for a
 * query whose answer is superseded and wrong for an input event: a cancelled `wheelScreen` that had
 * not reached the session yet is a zoom tick the board NEVER applies, so a fast scroll silently
 * loses travel, and a cancelled `pointerMoveScreen` is a drag step the gesture never sees. A step is
 * an increment on board state; it has no successor that could carry its delta.
 *
 * The session's own operation order is the serialization, so steps stay in the order the user made
 * them; a closed session rejects them all and the whole surface is going away anyway. */
function issueFlowGestureStep<T>(task: FlowTask<T>, consume?: (value: T) => void): void {
  const unsubscribe = task.subscribe(() => {});
  void task.result
    .then((value) => consume?.(value))
    .catch(() => {})
    .finally(unsubscribe);
}

async function readFlowTask<T>(task: FlowTask<T>): Promise<T> {
  const unsubscribe = task.subscribe(() => {});
  try {
    return await task.result;
  } finally {
    unsubscribe();
  }
}

async function readObservedFlowTask<T>(session: FlowWasmSession, feature: string, task: FlowTask<T>): Promise<T> {
  observeFlowTask(session, feature, task);
  return task.result;
}

/** 📄️ Payloads a session has already been handed, and the ones being handed to it right now.
 *
 * `observeFlowTask` keeps ONE task per feature key and cancels the previous — right for a query whose
 * answer is superseded, fatal for a payload the guest has to APPLY. The scene changes on every
 * `flowEvalTick` (measured roughly 700 ms apart on a live graph), so an unguarded re-issue cancelled
 * the document sync and the operator-kind table before the guest ever applied them: the session kept
 * its default three-widget fixture and its kind-less operator names for the whole session, while the
 * scene had carried seven nodes since the first frame. The same pre-emption defect the paint lane
 * fixed for `renderCanvas`, on the state half of the sync.
 *
 * An identical payload is therefore never re-sent — and a send that did NOT deliver clears the
 * record, so a cancelled or failed send is retried by the next scene pass rather than lost. */
const flowSessionDeliveredPayloads = new WeakMap<FlowWasmSession, Map<string, string>>();
const flowSessionSendingPayloads = new WeakMap<FlowWasmSession, Map<string, string>>();

function flowSessionPayloadRecord(map: WeakMap<FlowWasmSession, Map<string, string>>, session: FlowWasmSession): Map<string, string> {
  let record = map.get(session);
  if (!record) {
    record = new Map();
    map.set(session, record);
  }
  return record;
}

//#region 📦️SharedFlowPayloads
/** 📦️ Marks a content-addressed flow payload. `@<digest>\n<body>` CARRIES a body and registers it in
 * the guest process under that digest; `@<digest>` alone REFERENCES one the guest already holds.
 * Twin of `FLOW_SHARED_PAYLOAD_PREFIX` / `resolve_flow_shared_payload` in `🌊️flow/🖥️host/🦀️.rs`. */
const FLOW_SHARED_PAYLOAD_REFERENCE_PREFIX = "@";

/** 🧩️ Separates the PARTS of one composed payload. The operator table is the app-static catalogue
 * plus whatever records the scene derived; carried part by part, appending 1 805 B of scene operators
 * names the 98 642 B of app operators the guest already holds instead of re-crossing them. Twin of
 * `FLOW_SHARED_PAYLOAD_PART_SEPARATOR`. */
const FLOW_SHARED_PAYLOAD_PART_SEPARATOR = "";

/** 🗂️ Digests this page has confirmed the guest PROCESS holds.
 *
 * Module-scoped, not per session, because every flow session in a page lives in ONE wasm module with
 * one linear memory, while the app-static catalogue is the same bytes for all of them: the operator
 * kind infos are 88 438 B and the palette sections 22 849 B, and both crossed the ABI again for every
 * board that attached (`📓️flow-scroll-render-perf-2026-09-15.md` §9). A catalogue GENERATION that
 * really changed hashes differently and therefore carries its body; a second surface on the same
 * generation names it. */
const flowSharedPayloadDigests = new Set<string>();

/** 🔢 A content address for a flow payload — FNV-1a over the body in two independent lanes, tagged
 * with its length. Not a security digest: the guest treats it as an opaque key and answers
 * `unknown shared payload` when it does not hold it, which puts the body back on the wire. */
function flowContentDigest(body: string): string {
  let low = 0x811c9dc5;
  let high = 0x01000193;
  for (let index = 0; index < body.length; index += 1) {
    const code = body.charCodeAt(index);
    low = Math.imul(low ^ code, 0x01000193) >>> 0;
    high = Math.imul(high ^ (code + index), 0x85ebca6b) >>> 0;
  }
  return `${body.length.toString(36)}.${low.toString(36)}.${high.toString(36)}`;
}

/** ⏲️ How long an app-static payload waits for a better version of itself before it crosses.
 *
 * The app catalogue arrives in STAGES: the operator table crossed at 98 644 B and again 8 ms later at
 * 100 449 B, because a later refresh pass carried more operators than the first
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). Content addressing cannot help — the two bodies really
 * are different — so the payload waits out the stage instead. */
export const FLOW_SHARED_PAYLOAD_COALESCE_MS = 24;

/** 🔌️ What the coalescer needs from its host, injected so a law drives the real rule over a virtual
 * clock instead of a browser. */
export type FlowSharedPayloadCoalescerPorts = Readonly<{
  readonly send: (feature: string, parts: readonly string[]) => void;
  readonly schedule: (run: () => void, delayMs: number) => unknown;
  readonly cancel: (handle: unknown) => void;
}>;

/** 📦️ Holds an app-static payload for one settle window so a catalogue that arrives in stages crosses
 * ONCE, carrying its final content, instead of once per stage. Each feature settles on its own clock:
 * a new offer for the same feature replaces the pending one and restarts its window. */
export function createFlowSharedPayloadCoalescer(ports: FlowSharedPayloadCoalescerPorts, settleMs: number = FLOW_SHARED_PAYLOAD_COALESCE_MS) {
  const pending = new Map<string, readonly string[]>();
  const timers = new Map<string, unknown>();
  const flush = (feature: string) => {
    timers.delete(feature);
    const parts = pending.get(feature);
    if (!parts) return;
    pending.delete(feature);
    ports.send(feature, parts);
  };
  return {
    offer(feature: string, parts: readonly string[]): void {
      pending.set(feature, parts);
      const running = timers.get(feature);
      if (running !== undefined) ports.cancel(running);
      timers.set(feature, ports.schedule(() => flush(feature), settleMs));
    },
    pendingFeatures(): readonly string[] {
      return [...pending.keys()];
    },
    dispose(): void {
      for (const handle of timers.values()) ports.cancel(handle);
      timers.clear();
      pending.clear();
    },
  };
}

const flowSessionSharedCoalescers = new WeakMap<FlowWasmSession, ReturnType<typeof createFlowSharedPayloadCoalescer>>();
const flowSessionSharedIssues = new WeakMap<FlowWasmSession, Map<string, (payload: string) => FlowTask<unknown>>>();
//#endregion 📦️SharedFlowPayloads

/** 📤️ Sends `payload` under `feature` exactly once per distinct value — see the docstring above.
 *
 * A `shared` payload is app-static and identical for every session in the page, so it crosses the ABI
 * as a content-addressed reference once the guest process has confirmed the body. A reference the
 * guest cannot resolve fails the send, which both clears the digest and clears this session's
 * delivery record, so the next scene pass carries the body again. */
function sendFlowPayloadOnce(session: FlowWasmSession, feature: string, payload: string, issue: (payload: string) => FlowTask<unknown>): void {
  crossFlowPayload(session, feature, [payload], issue, false);
}

/** 📦️ Sends a payload composed of parts, each content-addressed: a part the guest process already
 * holds crosses as its digest alone, and only the parts it has never seen carry their bytes.
 *
 * This is where the app-static operator catalogue stops re-crossing. The table is built from the
 * app catalogue and the scene's own derived records, and the two arrive a few milliseconds apart, so
 * the whole 98 642 B table used to cross twice — once for the app half, once to append 1 805 B of
 * scene records. A part the guest cannot resolve fails the send, which clears both the digest and
 * this session's delivery record so the next scene pass carries the bytes again. */
function sendFlowPayloadPartsOnce(session: FlowWasmSession, feature: string, parts: readonly string[], issue: (payload: string) => FlowTask<unknown>): void {
  let issues = flowSessionSharedIssues.get(session);
  if (!issues) {
    issues = new Map();
    flowSessionSharedIssues.set(session, issues);
  }
  issues.set(feature, issue);
  let coalescer = flowSessionSharedCoalescers.get(session);
  if (!coalescer) {
    coalescer = createFlowSharedPayloadCoalescer({
      send: (sentFeature, sentParts) => {
        const sentIssue = flowSessionSharedIssues.get(session)?.get(sentFeature);
        if (sentIssue) crossFlowPayload(session, sentFeature, sentParts, sentIssue, true);
      },
      schedule: (run, delayMs) => setTimeout(run, delayMs),
      cancel: (handle) => clearTimeout(handle as ReturnType<typeof setTimeout>),
    });
    flowSessionSharedCoalescers.set(session, coalescer);
  }
  coalescer.offer(feature, parts);
}

/** 📮️ Puts one composed payload on the wire, naming every part the guest process already holds. */
function crossFlowPayload(session: FlowWasmSession, feature: string, parts: readonly string[], issue: (payload: string) => FlowTask<unknown>, shared: boolean): void {
  const delivered = flowSessionPayloadRecord(flowSessionDeliveredPayloads, session);
  const sending = flowSessionPayloadRecord(flowSessionSendingPayloads, session);
  const body = parts.join(FLOW_SHARED_PAYLOAD_PART_SEPARATOR);
  if (delivered.get(feature) === body || sending.get(feature) === body) return;
  sending.set(feature, body);
  const digests = shared ? parts.map(flowContentDigest) : [];
  const named = digests.filter((digest) => flowSharedPayloadDigests.has(digest)).length;
  const wire = shared
    ? parts
        .map((part, index) => (flowSharedPayloadDigests.has(digests[index]!) ? `${FLOW_SHARED_PAYLOAD_REFERENCE_PREFIX}${digests[index]}` : `${FLOW_SHARED_PAYLOAD_REFERENCE_PREFIX}${digests[index]}\n${part}`))
        .join(FLOW_SHARED_PAYLOAD_PART_SEPARATOR)
    : body;
  observeFlowTask(session, feature, issue(wire), undefined, (landed) => {
    if (sending.get(feature) !== body) return;
    sending.delete(feature);
    if (landed) {
      delivered.set(feature, body);
      for (const digest of digests) flowSharedPayloadDigests.add(digest);
    } else {
      delivered.delete(feature);
      for (const digest of digests) flowSharedPayloadDigests.delete(digest);
    }
  });
}

function cancelFlowTasks(session: FlowWasmSession): void {
  for (const task of activeFlowTasks.get(session)?.values() ?? []) task.cancel();
  activeFlowTasks.delete(session);
  flowSessionSharedCoalescers.get(session)?.dispose();
  flowSessionSharedCoalescers.delete(session);
  flowSessionSharedIssues.delete(session);
}

function syncFlowCanvasTheme(session: FlowWasmSession): void {
  observeFlowTask(session, "setCanvasThemeJson", session.setCanvasThemeJson(serializeCanvasThemeJson()));
}

function flowJsonText(value: unknown): string {
  return typeof value === "string" ? value : JSON.stringify(value ?? null);
}

function flowBoolean(value: unknown): boolean {
  return value === true || value === 1 || value === "\u0001";
}

function applyNodeGraphHoverFromScene(session: FlowWasmSession, hover: NodeGraphHover | undefined): void {
  if (hover === undefined) return;
  if (hover.nodeId && hover.portId) {
    observeFlowTask(session, "setHoverChannel", session.setHoverChannel(hover.nodeId, hover.portId));
  } else {
    observeFlowTask(session, "setHover", session.setHover(hover.nodeId ?? null));
  }
}

function syncFlowSessionEvalFromScene(session: FlowWasmSession, scene: NodeGraphScene): void {
  if (scene.evalJson) observeFlowTask(session, "applyEvalOutputsJson", session.applyEvalOutputsJson(scene.evalJson));
  if (scene.statusJson) observeFlowTask(session, "setNodeStatuses", session.setNodeStatuses(scene.statusJson));
  else if (scene.computingJson) observeFlowTask(session, "setComputingProgress", session.setComputingProgress(scene.computingJson));
}

/** 🔌️ The operator kind infos a flow session lays node ports out from: the app-static registered
 * catalogue first, then whatever DOCUMENT-derived records this particular scene carries (the OS
 * workflow window derives one per workflow node). `setNeuronKindInfosJson` replaces the session's whole
 * table, so the two sources are always pushed together, never one after the other. */
function syncFlowOperatorInfos(session: FlowWasmSession, catalogue: AppCatalogue, scene: NodeGraphScene): void {
  // 🧩️ The two sources are carried as two PARTS, not concatenated into one body: they arrive
  // milliseconds apart, so a single body made the whole app-static table cross again just to append
  // the scene's own records.
  const parts = [catalogue.operators ?? [], scene.operators ?? []].filter((source) => source.length > 0).map((source) => JSON.stringify(source));
  if (parts.length === 0) return;
  sendFlowPayloadPartsOnce(session, "setNeuronKindInfosJson", parts, (json) => session.setNeuronKindInfosJson(json));
}

/** 🛍️ Installs the app-static catalogue on a flow session: the operator kind infos the canvas lays
 * ports out from, and the palette sections its spotlight ranks. Its own pass, run once per app instance
 * rather than per scene sync — see {@link AppCatalogueContext}. */
function syncFlowSessionAppCatalogue(session: FlowWasmSession, catalogue: AppCatalogue, scene: NodeGraphScene): void {
  syncFlowOperatorInfos(session, catalogue, scene);
  if (catalogue.sections) sendFlowPayloadPartsOnce(session, "setCatalogueJson", [JSON.stringify(catalogue.sections)], (json) => session.setCatalogueJson(json));
}

function syncFlowSessionStructureFromScene(session: FlowWasmSession, scene: NodeGraphScene, catalogue: AppCatalogue, skipFixture = false): void {
  if (scene.operators?.length) syncFlowOperatorInfos(session, catalogue, scene);
  if (!skipFixture && scene.hostSnapshotJson) sendFlowPayloadOnce(session, "synchronizeSnapshotJson", scene.hostSnapshotJson, (json) => session.synchronizeSnapshotJson(json));
  if (scene.selection) observeFlowTask(session, "setSelection", session.setSelection(JSON.stringify(scene.selection)));
  applyNodeGraphHoverFromScene(session, scene.hover);
  if (scene.previewOffJson) observeFlowTask(session, "setPreviewOff", session.setPreviewOff(scene.previewOffJson));
  if (scene.lodJson) {
    try {
      const lod = parseSceneJsonField<{ readonly automatic?: boolean; readonly forcedLabel?: string }>(scene.lodJson);
      observeFlowTask(session, "setAutomaticLod", session.setAutomaticLod(lod.automatic !== false));
      if (lod.forcedLabel) observeFlowTask(session, "setForcedDrawLodLabel", session.setForcedDrawLodLabel(lod.forcedLabel));
    } catch {
      /* ignore */
    }
  }
}

/** 🖼️ The opening camera of a node-graph surface: the stored one when it already frames this graph,
 * the fit otherwise. Returns the camera it installed together with whether the fit won — the caller
 * persists a fit exactly the way it persists a pan or a zoom gesture, so the next open honours it.
 *
 * Twin of `DagHost::adopt_camera_or_fit`; the law itself is {@link dagStartupCamera}, pinned by
 * `♾️infinite/🖼️canvas/🧫️fixtures/📷️camera-fit/🔣️.json`. */
function applyFlowStartupCamera(session: FlowWasmSession, scene: NodeGraphScene, width: number, height: number): { readonly camera: DagCameraState; readonly fitted: boolean } {
  const stored = scene.viewport ?? DEFAULT_NODE_GRAPH_VIEWPORT;
  const decision = dagStartupCamera(stored, dagContentBounds(scene.nodes), width, height);
  observeFlowTask(session, "setCamera", session.setCamera(decision.camera.x, decision.camera.y, decision.camera.zoom));
  return decision;
}

/** 🔀️ A graph that CHANGED under a live camera — an example switch — is re-framed only when the
 * change left essentially nothing on screen. Twin of `DagHost::refit_camera_if_content_left_view`. */
function refitFlowCameraIfContentLeftView(session: FlowWasmSession, scene: NodeGraphScene, camera: DagCameraState, width: number, height: number): DagCameraState | null {
  const content = dagContentBounds(scene.nodes);
  if (!content || dagContentCoverage(content, camera, width, height) > DAG_CONTENT_REFIT_MAX_COVERAGE) return null;
  const fitted = dagFitCamera(content, width, height);
  observeFlowTask(session, "setCamera", session.setCamera(fitted.x, fitted.y, fitted.zoom));
  return fitted;
}

function syncFlowSessionFromScene(session: FlowWasmSession, scene: NodeGraphScene, catalogue: AppCatalogue): void {
  syncFlowSessionStructureFromScene(session, scene, catalogue);
  // 🧵️ Apply results from the plugin's off-main-thread `flowEvalTick` chain BEFORE computingJson —
  // applyEvalOutputsJson clears computing chrome, so applying computingJson first would have it
  // immediately wiped by this call on the same sync pass.
  syncFlowSessionEvalFromScene(session, scene);
}
//#endregion Sync

//#region FlowGraphCanvasHost
export function flowSurfaceRenderAllowed(surfaceReady: boolean): boolean {
  return surfaceReady;
}

//#region 🫱️InteractionPublication
/** 🫱️ What a surface can tell the plugin it selected. */
export type NodeGraphSelectionMarks = Readonly<{
  readonly nodeIds: readonly string[];
  readonly edgeIds?: readonly string[];
  readonly handleIds?: readonly string[];
}>;

/** 🫱️ What a surface can tell the plugin the pointer is over. */
export type NodeGraphHoverMark = Readonly<{ readonly hoveredId?: string; readonly portId?: string }>;

/** 🔑️ The exact selection a publication would carry, as one comparable value. Order is the domain's
 * own — the guest answers its ids in its own order and the plugin stores them that way, so a reorder
 * IS a different mark. */
export function nodeGraphSelectionMarkKey(marks: NodeGraphSelectionMarks): string {
  return JSON.stringify([marks.nodeIds, marks.edgeIds ?? [], marks.handleIds ?? []]);
}

/** 🔑️ The exact hover a publication would carry, as one comparable value. */
export function nodeGraphHoverMarkKey(mark: NodeGraphHoverMark): string {
  return `${mark.hoveredId ?? ""} ${mark.portId ?? ""}`;
}

/** 🧾️ What this surface has last told the plugin about its interaction marks, and therefore which
 * lanes a new reading actually OWES it.
 *
 * `emitInteractionState` published selection AND hover on every non-pan pointer-up, including a plain
 * click that changed neither — and the same pointer-up reaches it twice (the pick hook's
 * `onSelectTarget` and the surface's own `onPointerUp`), so one click on a node cost four guest
 * invocations and one click on empty canvas cost two. Each is a `performInvocation` → `refreshUi` →
 * React commit over the whole interaction scope (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
 * `📓️interaction-scope-narrowing-2026-09-15.md` §5).
 *
 * `adopt*` is the other half: marks that arrive FROM the plugin on a scene are what the plugin
 * already holds, so they set the baseline without owing a hop — and a plugin-side change (a keyboard
 * `selectAll`, an outline pick) can never be shadowed by a stale note of what we last sent. */
export function createNodeGraphInteractionLedger() {
  let selection: string | null = null;
  let hover: string | null = null;
  return {
    publishSelection(marks: NodeGraphSelectionMarks): boolean {
      const key = nodeGraphSelectionMarkKey(marks);
      if (key === selection) return false;
      selection = key;
      return true;
    },
    publishHover(mark: NodeGraphHoverMark): boolean {
      const key = nodeGraphHoverMarkKey(mark);
      if (key === hover) return false;
      hover = key;
      return true;
    },
    adoptSelection(marks: NodeGraphSelectionMarks): void {
      selection = nodeGraphSelectionMarkKey(marks);
    },
    adoptHover(mark: NodeGraphHoverMark): void {
      hover = nodeGraphHoverMarkKey(mark);
    },
    retire(): void {
      selection = null;
      hover = null;
    },
  };
}
//#endregion 🫱️InteractionPublication

/** ⏲️ How long after the last wheel tick a zoom gesture counts as settled. Long enough that one
 * continuous scroll is ONE gesture on a trackpad's own inter-tick spacing, short enough that the
 * camera the next open honours is published while the user still thinks of it as this gesture. */
export const FLOW_CAMERA_GESTURE_SETTLE_MS = 140;

/** 🔌️ What a camera gesture needs from its host, injected so a law drives the real rule over a
 * virtual clock instead of a browser. */
export type FlowCameraGesturePorts = Readonly<{
  readonly begin: (reason: string) => void;
  readonly end: (reason: string) => void;
  readonly invalidate: () => void;
  readonly publish: () => void;
  readonly schedule: (run: () => void, delayMs: number) => unknown;
  readonly cancel: (handle: unknown) => void;
}>;

/** 🎥️ The camera-gesture rule, as one unit: N ticks open ONE gesture and repaint through the
 * scheduler, and the plugin hears about the camera exactly once, when the ticks stop.
 *
 * A wheel gesture has no release event, so its end is the absence of the next tick — every tick
 * restarts the settle. Publishing per tick instead cost a `performInvocation` → `refreshUi` → React
 * commit for every notch of the wheel; publishing never would lose the camera the next open honours. */
export function createFlowCameraGesture(ports: FlowCameraGesturePorts, settleMs: number = FLOW_CAMERA_GESTURE_SETTLE_MS) {
  let settle: unknown = null;
  return {
    tick(): void {
      if (settle === null) ports.begin("wheel");
      else ports.cancel(settle);
      settle = ports.schedule(() => {
        settle = null;
        ports.end("wheel");
        ports.publish();
      }, settleMs);
      ports.invalidate();
    },
    active(): boolean {
      return settle !== null;
    },
    dispose(): void {
      if (settle !== null) ports.cancel(settle);
      settle = null;
    },
  };
}

/** 🎥️ Whether a press starts a camera PAN rather than a content gesture — the middle button, or a
 * pointer already held on it. A pan moves the board's own camera and changes neither selection nor
 * fixture, so its release owes the plugin one viewport publication and nothing else. */
export function flowGestureIsCameraPan(button: number, buttons: number): boolean {
  return button === 1 || buttons === 4;
}

/** 🪧️ The one frame verdict this host has to act on — `FlowPresentation.unpresentable` in
 * `🖥️flow-host.js`, spelled here the way every other flow surface word (`"created"`,
 * `"cancelled"`, `"device-lost"`) is spelled on both sides of the byte ABI.
 *
 * It means the frame wants the 2D draw-list replay and the canvas element cannot give a 2D context:
 * a WebGPU context is bound to it and its device is gone (or a bring-up bound one and then failed).
 * A canvas admits exactly one context kind for its whole life, so that element is finished — the
 * presentation can only be decided again for a NEW one, which is what {@link flowSurfaceCanvasKey}
 * mints. */
const flowUnpresentablePresentation = "unpresentable";

/** 🔑️ React key for the flow scene canvas. A new generation is a new DOM element, which is the only
 * way a surface that lost its device can be presented again. */
export function flowSurfaceCanvasKey(surfaceId: string, generation: number): string {
  return `${surfaceId}#${generation}`;
}

export function FlowGraphCanvasHost({
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
  const sessionRef = useRef<FlowWasmSession | null>(null);
  const schedulerRef = useRef<ReturnType<typeof createDemandFrameScheduler> | null>(null);
  const surfaceReadyRef = useRef(false);
  const drewOnceRef = useRef(false);
  const gpuCanvasRef = useRef<HTMLCanvasElement | null>(null);
  const labelCanvasRef = useRef<HTMLCanvasElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  // 🔁️ One presentation decision per canvas ELEMENT: bumping the generation mints a new element,
  // which re-runs the attach effect and lets the guest decide GPU-or-replay again for it.
  const [canvasGeneration, setCanvasGeneration] = useState(0);
  // 🖼️ The layout the live camera was last framed against — see `nodeGraphContentSignature`.
  const framedGraphSignatureRef = useRef<string | null>(null);
  const attachedSurfaceRef = useRef<{ readonly surface: number; readonly surfaceGeneration: number; readonly presentsOnGpu: boolean } | null>(null);
  const replaceUnpresentableCanvasRef = useRef<() => void>(() => {});
  const [contextMenu, setContextMenu] = useState<SurfaceContextMenuResult & {
    readonly x: number;
    readonly y: number;
    readonly widgetId?: string;
  } | null>(null);
  const contextMenuTitleLabel = useLabel(contextMenu?.titleKey ?? "ui.surfaceContextMenu.flow");
  const [wireRefusal, setWireRefusal] = useState<DagWireTypeRefusal | null>(null);
  const portTypeLabels = usePortTypeLabels();
  const wireRefusalText = useLabel("ui.nodeGraph.incompatiblePorts", wireRefusalLabelOptions(wireRefusal, portTypeLabels));
  const [selectionBounds, setSelectionBounds] = useState<ReturnType<typeof parseDagSelectionUnionBoundsScreen>>(null);
  const [marquee, setMarquee] = useState<ReturnType<typeof computeDagMarqueeOverlay>>(null);
  const [labelStateJson, setLabelStateJson] = useState("{}");
  const [sliderStateJson, setSliderStateJson] = useState("{}");
  const [containerSize, setContainerSize] = useState({ w: 800, h: 600 });
  const [sessionReady, setSessionReady] = useState(false);
  const overlayRequestRef = useRef(0);
  const [spotlight, setSpotlight] = useState<FlowSpotlightState | null>(null);
  const [spotlightSections, setSpotlightSections] = useState<readonly FlowCatalogueSection[]>([]);
  const appCatalogue = useAppCatalogue();
  // Always holds the latest app catalogue without forcing the scene-sync effects to depend on it.
  const appCatalogueRef = useRef(appCatalogue);
  appCatalogueRef.current = appCatalogue;
  const pickTargetsRef = useRef<readonly CanvasPickTarget[]>([]);
  const sceneSignature = useMemo(() => JSON.stringify(scene), [scene]);
  // Always holds the latest `scene` without forcing effects to depend on (and re-run per) it.
  const sceneRef = useRef(scene);
  sceneRef.current = scene;

  useEffect(() => {
    if (!windowInstanceId) return;
    return registerIntroductionSurfaceResolver(windowElementId(windowInstanceId), dagIntroductionResolver(sessionRef, containerRef));
  }, [windowInstanceId]);

  // 🔬️ `window.__semioFlowGraphProbe[surfaceId]` — the graph canvas's geometry read-back, modelled on
  // `ShellHost`'s `__semioOsCatalogProbe`/`__semioMountedGisMapProbe`. The canvas paints ITSELF (wasm,
  // one `<canvas>`, no per-node DOM), so a scripted or assistive caller has no way to find the screen
  // position of a node or a port `handle` and therefore no way to drive the one gesture only this
  // surface offers: dragging a wire from an output port to an input port. That gesture was the single
  // interaction in this app with no runtime proof of any kind
  // (`📓️audit-user-journey-gaps-2026-09-13.md` gap #3). Reuses `dagIntroductionResolver` verbatim —
  // the same resolver demonstration targeting already registers — so there is ONE entity→screen
  // implementation, not a probe-only second one. Warm-up is asynchronous (the first call for an entity
  // starts the session read and returns `null`); callers poll.
  useEffect(() => {
    const resolver = dagIntroductionResolver(sessionRef, containerRef);
    const host = window as unknown as { __semioFlowGraphProbe?: Record<string, FlowGraphSurfaceProbe> };
    const registry = (host.__semioFlowGraphProbe ??= {});
    registry[surfaceId] = {
      entity: (domain, id) => resolver.entity?.(domain, id) ?? null,
      nodeIds: () => (sceneRef.current.nodes ?? []).map((node) => node.id),
      nodeLayout: () => Object.fromEntries((sceneRef.current.nodes ?? []).map((node) => [node.id, { x: node.x, y: node.y }])),
      viewport: () => sceneRef.current.viewport ?? DEFAULT_NODE_GRAPH_VIEWPORT,
      hostSnapshotJson: () => sceneRef.current.hostSnapshotJson ?? null,
      rect: () => {
        const measured = containerRef.current?.getBoundingClientRect();
        return measured ? { x: measured.x, y: measured.y, width: measured.width, height: measured.height } : null;
      },
    };
    return () => {
      delete registry[surfaceId];
    };
  }, [surfaceId]);

  // 🔁️ Returns what `onAction` returns. `ComponentSceneHostProps.onAction` is declared
  // `void | Promise<void>` — a settling shell answers with the promise, and dropping it here is what
  // left every continuous gesture (a dragged slider) with no way to know when its last value had
  // landed, so it could only queue one round trip per tick.
  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => onAction({ controllerId, action, args: { surfaceId, ...args } }),
    [controllerId, onAction, surfaceId],
  );

  // 📮️ Effects that must NOT re-run per render (attach, scene sync) reach the dispatcher through this
  // ref: `dispatch`'s identity follows the `onAction` prop, and depending on it would re-attach the
  // canvas — and re-issue `synchronizeSnapshotJson` — on every parent render.
  const dispatchRef = useRef(dispatch);
  dispatchRef.current = dispatch;

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
      dispatch(action, action === "openInstance" ? { ...args, instanceId: resolveHostSnapshotWidgetInstanceId(scene.hostSnapshotJson, widgetId) } : args);
    },
    [dispatch, scene.hostSnapshotJson],
  );

  // 🧵️ Dispatches the mutated fixture to the plugin and returns immediately — evaluation happens
  // off the main thread in the plugin worker's `flowEvalTick` chain, never here. The next scene
  // resync applies its `evalJson`/`computingJson` back onto this session (`syncFlowSessionFromScene`).
  const commitFixture = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    observeFlowTask(session, "snapshotJson:commit", session.snapshotJson(), (value) => {
      dispatch(nodeGraphActions.edit, { operations: [{ operation: "setHostSnapshot", hostSnapshotJson: flowJsonText(value) }] });
    });
  }, [dispatch]);

  /** 🔗️ What a released gesture did, read out of `pointerUpScreen`'s own result — the gesture answers
   * for itself, so there is no second round trip and no window in which a later read could drain the
   * journal first. Shape: `{operations:[…],hostSnapshotChanged:boolean}` — `operations` in the guest's own
   * `nodeGraphEdit` sub-operation vocabulary (`connect` with four ids, `disconnect` with a synapse
   * id), the identical payload the wgpu renderer writes (`⚙️EngineCanvas/🎯️targets/🧊️wgpu`'s
   * `write_graph_edit_action`); `hostSnapshotChanged` the host's own content predicate
   * (`🌊️flow/🖥️host/🦀️.rs`'s `commit_gesture_history`), which is the ONLY thing that may authorise the
   * whole-fixture commit. A gesture with neither changed nothing and is owed no dispatch at all. */
  const graphGestureAnswer = useCallback((value: unknown): { readonly operations: readonly Record<string, unknown>[]; readonly hostSnapshotChanged: boolean } => {
    try {
      const parsed = JSON.parse(flowJsonText(value)) as { readonly operations?: unknown; readonly hostSnapshotChanged?: unknown } | null;
      const operations = parsed?.operations;
      return { operations: Array.isArray(operations) ? (operations as readonly Record<string, unknown>[]) : [], hostSnapshotChanged: parsed?.hostSnapshotChanged === true };
    } catch {
      return { operations: [], hostSnapshotChanged: false };
    }
  }, []);

  /** 🤹 The gesture reasons holding this surface open right now. A surface can be under two at once
   * (a wheel gesture arriving mid-drag), so "is a gesture active" is a set membership rather than a
   * boolean that the second `end` would clear while the first still runs. */
  const gestureReasonsRef = useRef<Set<string>>(new Set());
  const cameraPanRef = useRef(false);
  const isGestureActive = useCallback(() => gestureReasonsRef.current.size > 0, []);

  /** 🎬️ Opens a gesture: the demand scheduler goes continuous for its duration, so every repaint the
   * gesture asks for is coalesced onto a rAF instead of issued per input event. */
  const beginGesture = useCallback((reason: string) => {
    gestureReasonsRef.current.add(reason);
    schedulerRef.current?.beginContinuous(reason);
    schedulerRef.current?.invalidate();
  }, []);

  /** 🧾️ A scene that arrived while a gesture held the session, and the camera-only gestures that owe
   * it an apply when they end.
   *
   * The scene effect skips its sync while a gesture is live — a drag holds live fixture edits the
   * session must not have overwritten — and it only ever runs again on the NEXT scene, so a scene
   * that arrived during a gesture was dropped for good. With gestures now covering wheel and pan as
   * well as slider drags, that window is wide enough to swallow a real edit: a `reorganize` issued
   * just after a zoom moved every widget in the plugin and none on the board
   * (`📓️flow-scroll-render-perf-2026-09-15.md` §7). A camera gesture holds no fixture edits, so its
   * end can simply apply what it deferred; a content gesture's own commit brings the next scene. */
  const deferredSceneRef = useRef(false);
  const cameraOnlyGestureEndRef = useRef(false);
  const applyDeferredSceneRef = useRef<() => void>(() => {});

  const endGesture = useCallback((reason: string) => {
    if (!gestureReasonsRef.current.delete(reason)) return;
    schedulerRef.current?.endContinuous(reason);
    if (gestureReasonsRef.current.size === 0 && deferredSceneRef.current && (reason === "wheel" || cameraOnlyGestureEndRef.current)) {
      deferredSceneRef.current = false;
      applyDeferredSceneRef.current();
    }
    schedulerRef.current?.invalidate();
  }, []);

  /** 🎥️ Publishes the live board camera to the plugin — ONCE, when a camera gesture has settled.
   *
   * The camera is board state, not plugin state: `wheelScreen`/`pointerMoveScreen` move it and the
   * board repaints from it with no guest involved. `nodeGraphViewport` exists so the NEXT open of
   * this graph honours where the user left the view, which is a per-gesture fact, not a per-tick one.
   * Dispatching it per wheel tick cost a `performInvocation` → `refreshUi` → React commit for every
   * notch of the wheel: measured at 30 ticks → 5–9 guest hops, 1 board paint, and a median
   * tick-to-paint of 653–1110 ms (`📓️flow-scroll-render-perf-2026-09-15.md` §2). */
  const publishCameraRef = useRef<() => void>(() => {});
  const publishCamera = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    observeFlowTask(session, "viewport:settle", session.viewport(), (value) => {
      dispatch(nodeGraphActions.viewport, nodeGraphViewportActionArgs(parseNodeGraphSessionViewport(value)));
    });
  }, [dispatch]);
  publishCameraRef.current = publishCamera;

  /** 🖱️ A wheel gesture has no release event, so its end is the absence of the next tick: every tick
   * restarts {@link FLOW_CAMERA_GESTURE_SETTLE_MS}, and the gesture settles once when they stop. */
  const wheelGesture = useMemo(
    () =>
      createFlowCameraGesture({
        begin: beginGesture,
        end: endGesture,
        invalidate: () => schedulerRef.current?.invalidate(),
        publish: () => publishCameraRef.current(),
        schedule: (run, delayMs) => setTimeout(run, delayMs),
        cancel: (handle) => clearTimeout(handle as ReturnType<typeof setTimeout>),
      }),
    [beginGesture, endGesture],
  );

  useEffect(() => () => wheelGesture.dispose(), [wheelGesture]);

  const handleGesturePointerDown = useCallback(() => {
    beginGesture("gesture");
  }, [beginGesture]);

  const { sliderLane, beginSliderGesture } = useGraphSliderLanes(surfaceId, dispatchRef);
  const [gestureRecognizer] = useState(() => new GestureRecognizer());
  const pinchLogScaleRef = useRef(0);

  /** 🏷️ Paints the label/slider/selection overlay, and COALESCES instead of pre-empting — the same
   * law `renderFlow` below carries, on the other canvas of this surface.
   *
   * Each pass reads eleven session queries through `observeFlowTask`, which keeps one task per feature
   * key and cancels the previous. A second pass starting while the first is in flight therefore
   * cancelled the first pass's own reads, its `Promise.all` rejected, and the overlay was NOT
   * painted. Since every invalidation calls this, the overlay painted exactly ONCE per session:
   * measured live as three `fillText` calls in a 60 s run, showing the node-graph's pre-sync picture
   * (the default two-widget fixture) for the whole session while the guest already held all seven.
   *
   * An in-flight pass is therefore left alone and a request arriving during one is re-issued when it
   * settles: at most one extra pass, never a dropped one. */
  const overlayInFlightRef = useRef(false);
  const overlayDirtyRef = useRef(false);
  const paintOverlaysRef = useRef<() => void>(() => {});
  const paintOverlays = useCallback(() => {
    const session = sessionRef.current;
    const labelCanvas = labelCanvasRef.current;
    const container = containerRef.current;
    if (!session || !labelCanvas || !container) return;
    if (overlayInFlightRef.current) {
      overlayDirtyRef.current = true;
      return;
    }
    overlayInFlightRef.current = true;
    const rect = container.getBoundingClientRect();
    const dpr = globalThis.devicePixelRatio || 1;
    setContainerSize((prev) => (prev.w === rect.width && prev.h === rect.height ? prev : { w: rect.width, h: rect.height }));
    const request = ++overlayRequestRef.current;
    void Promise.all([
      readObservedFlowTask(session, "labelOverlayPaintStateJson", session.labelOverlayPaintStateJson()),
      readObservedFlowTask(session, "selectedWidgetIds", session.selectedWidgetIds()),
      readObservedFlowTask(session, "preselectWidgetIdsJson", session.preselectWidgetIdsJson()),
      readObservedFlowTask(session, "previewOffWidgetIds", session.previewOffWidgetIds()),
      readObservedFlowTask(session, "hoveredWidgetId:overlay", session.hoveredWidgetId()),
      readObservedFlowTask(session, "sliderOverlayStateJson", session.sliderOverlayStateJson()),
      readObservedFlowTask(session, "selectionUnionBoundsScreenJson", session.selectionUnionBoundsScreenJson()),
      readObservedFlowTask(session, "selectionPreviewPointsJson", session.selectionPreviewPointsJson()),
      readObservedFlowTask(session, "selectionPreviewCrossing", session.selectionPreviewCrossing()),
      readObservedFlowTask(session, "selectionPreviewMethod", session.selectionPreviewMethod()),
      readObservedFlowTask(session, "hoveredChannelJson:overlay", session.hoveredChannelJson()),
    ]).then(([labelValue, selectedValue, preselectValue, dimmedValue, hoveredValue, sliderValue, boundsValue, pointsValue, crossingValue, methodValue, channelValue]) => {
      if (request !== overlayRequestRef.current || sessionRef.current !== session || labelCanvasRef.current !== labelCanvas) return;
      const labelJson = flowJsonText(labelValue);
      setLabelStateJson((prev) => (prev === labelJson ? prev : labelJson));
      const minimapCursor = parseDagMinimapWidgetCursor(labelJson);
      if (gpuCanvasRef.current) {
        gpuCanvasRef.current.style.cursor = minimapCursor ?? "";
      }
      const selectedIds = parseDagNodeIdArray(flowJsonText(selectedValue));
      const preselect = parseDagPreselectJson(flowJsonText(preselectValue));
      const dimmedIds = parseDagNodeIdArray(flowJsonText(dimmedValue));
      paintDagLabelOverlays(labelJson, labelCanvas, rect.width, rect.height, dpr, {
        hoveredId: typeof hoveredValue === "string" ? hoveredValue : null,
        selectedIds,
        preselect,
        dimmedIds,
        highlightIds: sceneRef.current.highlighted ?? [],
      });
      const nextSliderJson = flowJsonText(sliderValue);
      setSliderStateJson((prev) => (prev === nextSliderJson ? prev : nextSliderJson));
      const nextBounds = parseDagSelectionUnionBoundsScreen(flowJsonText(boundsValue));
      setSelectionBounds((prev) => (sameOverlayValue(prev, nextBounds) ? prev : nextBounds));
      const nextMarquee = computeDagMarqueeOverlay(flowJsonText(pointsValue), flowBoolean(crossingValue), typeof methodValue === "string" ? methodValue : undefined);
      setMarquee((prev) => (sameOverlayValue(prev, nextMarquee) ? prev : nextMarquee));
      const nextRefusal = parseDagWireTypeRefusalJson(flowJsonText(channelValue));
      setWireRefusal((prev) => (sameOverlayValue(prev, nextRefusal) ? prev : nextRefusal));
    })
      .catch(() => {})
      .finally(() => {
        overlayInFlightRef.current = false;
        if (!overlayDirtyRef.current) return;
        overlayDirtyRef.current = false;
        paintOverlaysRef.current();
      });
  }, []);
  paintOverlaysRef.current = paintOverlays;

  /** 🖼️ Presents one frame, and coalesces instead of pre-empting.
   *
   * `observeFlowTask` keeps ONE task per feature key and cancels the previous one, which is right for
   * a query whose answer is superseded — and wrong for the paint. A render reply is what actually puts
   * pixels on the canvas, and a resize has just cleared the backing store; cancelling the in-flight
   * render leaves the canvas blank until the NEXT render completes uncancelled, so a burst of
   * invalidations (mount, ResizeObserver, scene sync, theme sync — all of which arrive together)
   * repeatedly emptied a canvas the engine had in fact painted. Measured live: `render_frame` ran on
   * every invalidation (`[DEBUG] dag draw lod=detail zoom=1.784` repeating) while `renderFlowCanvas`
   * ran exactly once for the whole session. So an in-flight render is left alone and a request that
   * arrives during one is remembered and re-issued when it settles — at most one extra frame, never a
   * dropped one. */
  const renderInFlightRef = useRef(false);
  const renderDirtyRef = useRef(false);
  const renderFlowRef = useRef<() => void>(() => {});
  const renderFlow = useCallback(() => {
    const session = sessionRef.current;
    const canvas = gpuCanvasRef.current;
    if (!session || !canvas || !flowSurfaceRenderAllowed(surfaceReadyRef.current)) return;
    if (renderInFlightRef.current) {
      renderDirtyRef.current = true;
      return;
    }
    renderInFlightRef.current = true;
    const closePaint = hopTrace.open("surface.paint", { surfaceId });
    const task = session.renderCanvas(canvas);
    observeFlowTask(session, "renderCanvas", task, () => {
      if (drewOnceRef.current) return;
      drewOnceRef.current = true;
    });
    void task.result
      .then((state) => {
        if ((state as { presentation?: string } | undefined)?.presentation === flowUnpresentablePresentation) replaceUnpresentableCanvasRef.current();
      })
      .catch(() => {})
      .finally(() => {
        closePaint();
        renderInFlightRef.current = false;
        if (!renderDirtyRef.current) return;
        renderDirtyRef.current = false;
        renderFlowRef.current();
      });
  }, [surfaceId]);
  renderFlowRef.current = renderFlow;

  /** ♻️ Retires a canvas that can no longer be presented onto and mints its successor.
   *
   * The guest owns exactly one surface at a time and refuses a replacement that was not closed
   * first (`attach_surface` → `Flow surface requires exact close before replacement`), so the old
   * surface is cancelled — which also releases its `CanvasGpuSession` — before the new element's
   * attach effect runs. Idempotent per generation: a burst of frames all reporting the same dead
   * canvas must mint ONE successor, not one each.
   *
   * Only a canvas that WAS presenting on the GPU can be resurrected this way, and that bound is the
   * whole termination argument: a successor re-attaches, and an attach that finds no adapter leaves
   * the element 2D-capable, so there is at most one replacement per device loss. An element that
   * refuses a 2D context while its surface never had a device is not poisoned — it is an
   * environment with no 2D canvas at all (jsdom, a stripped embedder), where a successor would
   * refuse in exactly the same way and the host would remount forever. */
  const replaceUnpresentableCanvas = useCallback(() => {
    const session = sessionRef.current;
    const retired = attachedSurfaceRef.current;
    if (!retired?.presentsOnGpu) return;
    attachedSurfaceRef.current = null;
    surfaceReadyRef.current = false;
    // 🚪️ `"cancelled"`, not `"device-lost"`: the latter parks the surface as recoverable and keeps it
    // occupying the session's single slot, so the successor's `attachSurface` would be refused.
    if (session) observeFlowTask(session, "surfaceStatus:unpresentable", session.surfaceStatus({ surface: retired.surface, surfaceGeneration: retired.surfaceGeneration, status: "cancelled" }));
    setCanvasGeneration((generation) => generation + 1);
  }, [surfaceId]);
  replaceUnpresentableCanvasRef.current = replaceUnpresentableCanvas;

  /** 📏️ Single owner of both canvases' backing-store size: measures the container and writes the
   * device-pixel store synchronously, then forwards the logical size to the wasm surface when one is
   * already attached. Session-independent on purpose — before this, the GPU canvas was only ever sized
   * from inside `attachCanvas().then(...)` and the label canvas only from `paintOverlays`, so a boot
   * where the surface attach is slow (or a hidden tab, where the shared `GraphWasmCanvas` layout wait
   * never ticks) left both at the HTML default 300×150 in a 966×836 window. */
  const syncSurfaceSize = useCallback(() => {
    const container = containerRef.current;
    if (!container) return;
    const rect = container.getBoundingClientRect();
    const dpr = globalThis.devicePixelRatio || 1;
    resizeCanvasBackingStore(gpuCanvasRef.current, rect.width, rect.height, dpr);
    resizeCanvasBackingStore(labelCanvasRef.current, rect.width, rect.height, dpr);
    setContainerSize((prev) => (prev.w === rect.width && prev.h === rect.height ? prev : { w: rect.width, h: rect.height }));
    const session = sessionRef.current;
    if (session && surfaceReadyRef.current) observeFlowTask(session, "setSize", session.setSize(Math.max(1, Math.round(rect.width)), Math.max(1, Math.round(rect.height)), dpr));
  }, []);

  useEffect(() => {
    const container = containerRef.current;
    if (!container) return;
    syncSurfaceSize();
    const observer = new ResizeObserver(() => {
      syncSurfaceSize();
      renderFlow();
      paintOverlays();
    });
    observer.observe(container);
    return () => observer.disconnect();
  }, [paintOverlays, renderFlow, syncSurfaceSize]);

  /** 🧾️ What this surface last told the plugin about selection and hover — see
   * {@link createNodeGraphInteractionLedger}. */
  const interactionLedger = useMemo(() => createNodeGraphInteractionLedger(), []);

  const handleGesturePointerUp = useCallback(() => {
    endGesture("gesture");
    const session = sessionRef.current;
    if (session) {
      syncFlowSessionStructureFromScene(session, sceneRef.current, appCatalogueRef.current, true);
      renderFlow();
      paintOverlays();
    }
  }, [endGesture, paintOverlays, renderFlow]);

  const emitInteractionState = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    void Promise.all([
      readObservedFlowTask(session, "selectionDomainsJson:interaction", session.selectionDomainsJson()),
      readObservedFlowTask(session, "hoveredWidgetId:interaction", session.hoveredWidgetId()),
      readObservedFlowTask(session, "hoveredChannelJson:interaction", session.hoveredChannelJson()),
    ]).then(([domainsValue, hoveredValue, channelValue]) => {
      const domains = parseSelectionDomainsFromSession(flowJsonText(domainsValue));
      const selection = { nodeIds: domains.nodes, edgeIds: domains.edges, handleIds: domains.handles };
      const selectionDue = interactionLedger.publishSelection(selection);
      const hovered = typeof hoveredValue === "string" ? hoveredValue : undefined;
      const portId = parseDagChannelRefJson(flowJsonText(channelValue))?.portId;
      const hoverDue = interactionLedger.publishHover({ hoveredId: hovered, portId });
      if (selectionDue) publishNodeGraphSelection(dispatch, sceneRef.current.interactionDomain, selection);
      if (hoverDue) publishNodeGraphHover(dispatch, sceneRef.current.interactionDomain, hovered, portId);
    }).catch(() => {});
    paintOverlays();
  }, [dispatch, interactionLedger, paintOverlays]);

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
        observeFlowTask(session, "setSelection", session.setSelection(JSON.stringify(nodeIds)));
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
        const scheduler = createDemandFrameScheduler(() => {
          renderFlow();
          paintOverlays();
        });
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

  const flowGraphCanvasHostShellScope = useShellScopeOptional();
  useCanvasAppearanceSync(
    () => {
      if (sessionRef.current) syncFlowCanvasTheme(sessionRef.current);
      renderFlow();
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
      observeFlowTask(session, "pickTargetsAtScreenJson:pointer", session.pickTargetsAtScreenJson(sx, sy), (value) => {
        try {
          pickTargetsRef.current = JSON.parse(flowJsonText(value)) as CanvasPickTarget[];
        } catch {
          pickTargetsRef.current = [];
        }
      });
      return [...pickTargetsRef.current];
    },
    onHoverFocus: (focus) => {
      const session = sessionRef.current;
      if (!session) return;
      const target = focus.target;
      const channel = nodeGraphPickChannel(target);
      if (!target) {
        observeFlowTask(session, "setHover", session.setHover(null));
      } else if (channel) {
        observeFlowTask(session, "setHoverChannel", session.setHoverChannel(channel.nodeId, channel.portId));
      } else {
        observeFlowTask(session, "setHover", session.setHover(target.id));
      }
      void Promise.all([
        readObservedFlowTask(session, "hoveredWidgetId:pointer", session.hoveredWidgetId()),
        readObservedFlowTask(session, "hoveredChannelJson:pointer", session.hoveredChannelJson()),
      ]).then(([hoveredValue, channelValue]) => {
        const hoveredId = typeof hoveredValue === "string" ? hoveredValue : undefined;
        const portId = parseDagChannelRefJson(flowJsonText(channelValue))?.portId;
        if (interactionLedger.publishHover({ hoveredId, portId })) publishNodeGraphHover(dispatch, sceneRef.current.interactionDomain, hoveredId, portId);
      }).catch(() => {});
      schedulerRef.current?.invalidate();
    },
    onSelectTarget: () => {
      emitInteractionState();
    },
  });

  const clearGhostPreview = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    observeFlowTask(session, "clearGhostWidget", session.clearGhostWidget());
    renderFlow();
    paintOverlays();
  }, [paintOverlays, renderFlow]);

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
        observeFlowTask(session, "clearGhostWidget", session.clearGhostWidget());
        renderFlow();
        paintOverlays();
        return;
      }
      observeFlowTask(session, "setGhostWidget", session.setGhostWidget(flowCatalogueItemDescriptor(item), open.world.x, open.world.y));
      renderFlow();
      paintOverlays();
    },
    [paintOverlays, renderFlow, spotlight],
  );

  const commitSpotlightItem = useCallback(
    (item: FlowCatalogueItem) => {
      const session = sessionRef.current;
      const open = spotlight;
      if (!session || !open) return;
      observeFlowTask(session, "addWidget", session.addWidget(flowCatalogueItemDescriptor(item), open.world.x, open.world.y), () => {
        commitFixture();
        emitInteractionState();
      });
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
      const camera = parseDagOverlayCamera(labelStateJson);
      const world = dagScreenToWorld(camera, rect.width, rect.height, sx, sy);
      setSpotlight({ screen: { x: sx, y: sy }, world });
      observeFlowTask(session, "worldFromScreen:spotlight", session.worldFromScreen(sx, sy), (value) => {
        try {
          const parsed = JSON.parse(flowJsonText(value)) as { readonly x?: number; readonly y?: number };
          setSpotlight({ screen: { x: sx, y: sy }, world: { x: parsed.x ?? world.x, y: parsed.y ?? world.y } });
        } catch {
          /* retain projected world */
        }
      });
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
      const camera = parseDagOverlayCamera(labelStateJson);
      const world = dagScreenToWorld(camera, rect.width, rect.height, sx, sy);
      observeFlowTask(session, "setGhostWidget", session.setGhostWidget(catalogueGhostDescriptorJson(catalogueApp), world.x, world.y));
      observeFlowTask(session, "worldFromScreen:drag", session.worldFromScreen(sx, sy), (value) => {
        try {
          const parsed = JSON.parse(flowJsonText(value)) as { readonly x?: number; readonly y?: number };
          observeFlowTask(session, "setGhostWidget", session.setGhostWidget(catalogueGhostDescriptorJson(catalogueApp), parsed.x ?? world.x, parsed.y ?? world.y));
        } catch {
          /* retain projected world */
        }
      });
      renderFlow();
      paintOverlays();
    },
    [editable, labelStateJson, paintOverlays, renderFlow],
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
      const camera = parseDagOverlayCamera(labelStateJson);
      const fallback = dagScreenToWorld(camera, rect.width, rect.height, sx, sy);
      observeFlowTask(session, "worldFromScreen:drop", session.worldFromScreen(sx, sy), (value) => {
        let world = fallback;
        try {
          const parsed = JSON.parse(flowJsonText(value)) as { readonly x?: number; readonly y?: number };
          world = { x: parsed.x ?? fallback.x, y: parsed.y ?? fallback.y };
        } catch {
          /* retain projected world */
        }
        const catalogueApp = parseCatalogueAppDragPayload(raw);
        if (catalogueApp) {
          dispatch("spawnApp", { pluginId: catalogueApp.pluginId, appId: catalogueApp.appId, x: world.x, y: world.y });
          return;
        }
        const descriptor = raw.startsWith("{") ? raw : JSON.stringify({ kind: raw });
        observeFlowTask(session, "addWidget", session.addWidget(descriptor, world.x, world.y), () => {
          commitFixture();
          emitInteractionState();
        });
      });
    },
    [clearGhostPreview, commitFixture, dispatch, editable, emitInteractionState, labelStateJson],
  );

  const onCanvasDoubleClick = useCallback(
    (event: MouseEvent<HTMLDivElement>) => {
      if (!editable) return;
      const session = sessionRef.current;
      if (!session) return;
      observeFlowTask(session, "hoveredWidgetId:doubleClick", session.hoveredWidgetId(), (value) => {
        const hovered = typeof value === "string" ? value : undefined;
        if (!hovered) {
          openSpotlightAtClient(event.clientX, event.clientY, event.currentTarget);
          return;
        }
        const instanceId = resolveHostSnapshotWidgetInstanceId(scene.hostSnapshotJson, hovered);
        if (instanceId) {
          dispatch("openInstance", { instanceId });
        }
      });
    },
    [dispatch, editable, openSpotlightAtClient, scene.hostSnapshotJson],
  );

  useEffect(() => clearGhostPreview, [clearGhostPreview]);

  // 🛍️ The app-static catalogue is installed on its own pass, not with every scene sync: it changes
  // only when the app instance (or the installed operator extensions) changes, while a scene resyncs on
  // every fixture edit. See `AppCatalogueContext`.
  useEffect(() => {
    const session = sessionRef.current;
    if (!session || !sessionReady) return;
    syncFlowSessionAppCatalogue(session, appCatalogue, sceneRef.current);
  }, [appCatalogue, sessionReady]);

  useEffect(() => {
    if (!spotlight || !sessionReady) {
      setSpotlightSections([]);
      return;
    }
    const session = sessionRef.current;
    if (!session) {
      setSpotlightSections((appCatalogue.sections ?? []) as readonly FlowCatalogueSection[]);
      return;
    }
    return observeFlowTask(session, "catalogueJson:spotlight", session.catalogueJson(), (value) => {
      const sections = parseFlowCatalogueSections(flowJsonText(value));
      setSpotlightSections(sections.length > 0 ? sections : ((appCatalogue.sections ?? []) as readonly FlowCatalogueSection[]));
    });
  }, [appCatalogue, sessionReady, spotlight]);

  return (
    <div
      ref={containerRef}
      className="relative h-full w-full"
      // 🧱️ The board paints itself; nothing inside this box may make the document relayout or
      // restyle, so a gesture's 60 repaints never reach the shell's own style/layout work.
      style={{ contain: "layout paint" }}
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
              domains = parseSelectionDomainsFromSession(flowJsonText(await readFlowTask(session.selectionDomainsJson())));
            } catch {
              domains = { nodes: [], edges: [], handles: [] };
            }
          }
          if (session && container) {
            const rect = container.getBoundingClientRect();
            const sx = event.clientX - rect.left;
            const sy = event.clientY - rect.top;
            try {
              const targets = JSON.parse(flowJsonText(await readFlowTask(session.pickTargetsAtScreenJson(sx, sy)))) as CanvasPickTarget[];
              hits = targets.map((target) => ({ domain: target.domain, id: target.id, label: target.label }));
              widgetId = pickMostSpecificCanvasTarget(targets)?.id;
            } catch {
              widgetId = undefined;
            }
          }
          if (!widgetId) {
            const hovered = session ? await readFlowTask(session.hoveredWidgetId()).catch(() => undefined) : undefined;
            widgetId = typeof hovered === "string" ? hovered : undefined;
          }
          if (widgetId && !domains.nodes.includes(widgetId)) {
            domains = { nodes: [widgetId], edges: [], handles: [] };
            if (session) {
              observeFlowTask(session, "setSelection", session.setSelection(JSON.stringify(domains)));
              renderFlow();
            }
            dispatch("contextMenuAt", { id: widgetId });
          } else if (widgetId) {
            dispatch("contextMenuAt", { id: widgetId });
          }
          const menu = await openSurfaceContextMenu(
            requestContextMenu,
            {
              menu: { id: "nodeGraph", args: null },
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
      <canvas key={flowSurfaceCanvasKey(surfaceId, canvasGeneration)} ref={gpuCanvasRef} className="absolute inset-0 block h-full w-full" />
      <canvas ref={labelCanvasRef} className="pointer-events-none absolute inset-0 z-40" />
      <GraphSliderOverlays
        scopeId={JSON.stringify([windowInstanceId, controllerId, surfaceId])}
        stateJson={sliderStateJson}
        logicalW={containerSize.w}
        logicalH={containerSize.h}
        editable={editable}
        occluderRect={parseDagMinimapWidgetOccluder(labelStateJson)}
        onSliderChange={(widgetId, value) => {
          const session = sessionRef.current;
          if (!session) return;
          observeFlowTask(session, "setSliderValue", session.setSliderValue(widgetId, value));
          sliderLane(widgetId).offer(value);
          renderFlow();
          paintOverlays();
        }}
        onSliderCommit={(widgetId, value) => sliderLane(widgetId).commit(value)}
        onSliderPointerDown={(widgetId) => {
          beginSliderGesture(widgetId);
          handleGesturePointerDown();
        }}
        onSliderPointerUp={() => handleGesturePointerUp()}
      />
      {selectionBounds ? (
        <>
          <div className="pointer-events-none absolute z-20 border-2 border-accent" style={{ left: selectionBounds.x, top: selectionBounds.y, width: selectionBounds.width, height: selectionBounds.height }} />
          {editable ? (
            <SelectionAlignChrome
              bounds={selectionBounds}
              onAlign={(mode) => {
                const session = sessionRef.current;
                if (session) observeFlowTask(session, "alignSelection", session.alignSelection(mode));
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
        className="absolute inset-0 z-30 touch-none"
        data-gesture-surface="flow"
        onPointerDown={(event) => {
          const rect = event.currentTarget.getBoundingClientRect();
          const verdict = gestureRecognizer.down({ pointerId: event.pointerId, x: event.clientX - rect.left, y: event.clientY - rect.top });
          if (verdict.kind === "pinchBegin") {
            const session = sessionRef.current;
            pinchLogScaleRef.current = 0;
            cameraPanRef.current = false;
            cameraOnlyGestureEndRef.current = false;
            setWireRefusal(null);
            pickInteraction.onCanvasPointerLeave();
            for (const tracked of gestureRecognizer.pointers) event.currentTarget.setPointerCapture?.(tracked.pointerId);
            endGesture("gesture");
            if (session) issueFlowGestureStep(session.pointerCancelScreen(), () => schedulerRef.current?.invalidate());
            return;
          }
          if (verdict.kind !== "single") return;
          if (!editable) return;
          if (event.button === 2) return;
          const session = sessionRef.current;
          if (!session) return;
          const client = { x: event.clientX, y: event.clientY };
          // 🖱️ The gesture belongs to this canvas until the button comes back up. Without capture, a
          // drag that leaves the canvas — dragging a wire out to cut it, or past the edge on the way
          // to a far port — releases over whatever is underneath (the outline tree, another window),
          // and the host never sees `pointer_up_screen`: the wire stays in flight forever and the
          // gesture makes no edit at all. Measured on 6018: the press entered `InteractionMode::DrawEdge`
          // and no release ever arrived.
          try {
            event.currentTarget.setPointerCapture(event.pointerId);
          } catch {
            /* capture is unavailable for this pointer — the gesture still works inside the canvas */
          }
          pickInteraction.onCanvasPointerDown(client);
          cameraPanRef.current = flowGestureIsCameraPan(event.button, event.buttons);
          issueFlowGestureStep(session.pointerDownScreen(event.clientX - rect.left, event.clientY - rect.top, event.button, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey, cameraPanRef.current));
          handleGesturePointerDown();
        }}
        onPointerMove={(event) => {
          const session = sessionRef.current;
          if (!session) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const verdict = gestureRecognizer.move({ pointerId: event.pointerId, x: event.clientX - rect.left, y: event.clientY - rect.top });
          if (verdict.kind === "pinch") {
            const plan = graphPinchWheelPlan(verdict.step, pinchLogScaleRef.current);
            pinchLogScaleRef.current = plan.pendingLogScale;
            if (plan.calls.length === 0) return;
            for (const call of plan.calls) issueFlowGestureStep(session.wheelScreen(call.sx, call.sy, call.deltaX, call.deltaY, call.zoomGesture));
            wheelGesture.tick();
            return;
          }
          if (verdict.kind !== "single") return;
          pickInteraction.onCanvasPointerMove({ x: event.clientX, y: event.clientY });
          issueFlowGestureStep(session.pointerMoveScreen(event.clientX - rect.left, event.clientY - rect.top, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey));
          schedulerRef.current?.invalidate();
        }}
        onPointerUp={(event) => {
          const verdict = gestureRecognizer.up(event.pointerId);
          if (verdict.kind === "pinchEnd") pinchLogScaleRef.current = 0;
          if (verdict.kind !== "single") return;
          if (event.button === 2) return;
          const session = sessionRef.current;
          if (!session) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const client = { x: event.clientX, y: event.clientY };
          try {
            if (event.currentTarget.hasPointerCapture(event.pointerId)) event.currentTarget.releasePointerCapture(event.pointerId);
          } catch {
            /* nothing was captured */
          }
          pickInteraction.onCanvasPointerUp(client, { shift: event.shiftKey, ctrlOrMeta: event.metaKey || event.ctrlKey, alt: event.altKey });
          setWireRefusal(null);
          const wasCameraPan = cameraPanRef.current;
          cameraPanRef.current = false;
          cameraOnlyGestureEndRef.current = wasCameraPan;
          issueFlowGestureStep(session.pointerUpScreen(event.clientX - rect.left, event.clientY - rect.top, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey), (value) => {
            // 🔗️ A gesture that wired or cut dispatches THAT — four ids, or one synapse id — and never
            // the whole fixture on top of it: the guest replays the narrow intent and re-publishes the
            // graph itself, so a second `setFixture` would only race its own result.
            //
            // 🪶 A gesture that wired nothing dispatches the whole fixture only when the HOST says its
            // content moved (a node drag, an inline slider, a port insert). A plain click, a marquee, a
            // pan and a press that grabbed nothing change nothing, and used to dispatch a whole-fixture
            // `nodeGraphEdit` all the same — a retained command per click, and a re-armed preview
            // evaluation on a shell nobody touched.
            const { operations, hostSnapshotChanged } = graphGestureAnswer(value);
            if (operations.length > 0) dispatch(nodeGraphActions.edit, { operations });
            else if (hostSnapshotChanged) commitFixture();
          });
          handleGesturePointerUp();
          // 🎥️ A pan moved the camera and nothing else, so its settle owes the plugin the viewport and
          // never a selection/hover round trip the board's own state did not change.
          if (wasCameraPan) publishCameraRef.current();
          else emitInteractionState();
        }}
        onPointerCancel={(event) => {
          const verdict = gestureRecognizer.up(event.pointerId);
          if (verdict.kind === "pinchEnd") pinchLogScaleRef.current = 0;
          if (verdict.kind !== "single") return;
          const session = sessionRef.current;
          try {
            if (event.currentTarget.hasPointerCapture(event.pointerId)) event.currentTarget.releasePointerCapture(event.pointerId);
          } catch {
            /* nothing was captured */
          }
          setWireRefusal(null);
          cameraPanRef.current = false;
          cameraOnlyGestureEndRef.current = false;
          pickInteraction.onCanvasPointerLeave();
          endGesture("gesture");
          if (!session) return;
          issueFlowGestureStep(session.pointerCancelScreen(), () => {
            schedulerRef.current?.invalidate();
            paintOverlays();
          });
        }}
        onPointerLeave={() => {
          setWireRefusal(null);
          pickInteraction.onCanvasPointerLeave();
        }}
        onDoubleClick={onCanvasDoubleClick}
        onWheel={(event) => {
          event.preventDefault();
          const session = sessionRef.current;
          if (!session) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const delta = event.deltaMode === 1 ? event.deltaY * 16 : event.deltaMode === 2 ? event.deltaY * 400 : event.deltaY;
          issueFlowGestureStep(session.wheelScreen(event.clientX - rect.left, event.clientY - rect.top, 0, delta, true));
          wheelGesture.tick();
        }}
      />
      {wireRefusal ? (
        <div
          role="status"
          aria-live="polite"
          data-wire-refusal-json={JSON.stringify(wireRefusal)}
          className="pointer-events-none absolute bottom-2 left-1/2 z-30 -translate-x-1/2 rounded border border-destructive bg-panel px-2 py-1 text-[11px] text-destructive shadow-sm"
        >
          {wireRefusalText}
        </div>
      ) : null}
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
