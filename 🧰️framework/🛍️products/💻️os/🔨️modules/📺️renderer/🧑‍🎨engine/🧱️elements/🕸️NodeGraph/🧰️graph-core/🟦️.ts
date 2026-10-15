/** 🧰️ The shared graph session and pick contract, port type rules, viewport and selection action arguments, edit-row builders, keyboard handling, pinch-wheel plan and JSON text helpers every graph surface reads.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | NodeGraph}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { type DagTextDecodeControl } from "../../../../../♾️infinite/🎲️board/🚪️io/📝️text/🎯️dag-input/🟦️.ts";
import type { DagWireTypeRefusal } from "../../../../../♾️infinite/🎲️board/🧬️schema/🎯️dag-input/🟦️.ts";
import { NativeDecodeControl } from "../../../../../../../../🔨️modules/🌱️value/🛬️decode/🟦️.ts";
import { useMemo, type KeyboardEvent } from "react";
import { type WasmCanvasSession } from "@semio-tech/canvas-react-renderer";
import { STYLING_METRICS, syncSessionCanvasTheme } from "@semio-tech/ui-styling";
import {
  diagramKeyboardAnnouncement,
  diagramKeyboardStep,
  useDiagramTranslate,
  useLabel,
  type CanvasPickTarget
} from "@semio-tech/ui-react";
import {
  nodeGraphActions,
  pinchZoomNotches,
  parseViewport2d,
  type ContextMenuItemSpec,
  type PinchStep,
  type NodeGraphNodeRecord,
  type NodeGraphInteractionDomain,
  type NodeGraphPortRecord,
  type NodeGraphScene,
  type Viewport2d
} from "@semio-tech/framework";

//#region 🔖️NodeGraphHost
//#region Types
export type WorkflowNodeData = {
  readonly label: string;
  readonly inputs: readonly NodeGraphPortRecord[];
  readonly outputs: readonly NodeGraphPortRecord[];
  readonly width: number;
  readonly height: number;
};

type GraphContextMenuItem = ContextMenuItemSpec;

export type FrameworkGraphSession = WasmCanvasSession & {
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
  /** 🔗️ Drains what the last gesture (or align) did to the graph as `nodeGraphEdit` arguments (design §13.3). */
  takeGraphEditsJson?(): string;
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

/** 🎛️ Admits physical graph facts under the request's live cancellation and bounded publication policy. */
export function nodeGraphInputDecodeControl(alive:()=>boolean,progress:()=>void):DagTextDecodeControl{
  const deadline=performance.now()+100;const cancelled=()=>!alive()||performance.now()>=deadline;
  return {syntax:{maximumBytes:65536,maximumNodes:4096,maximumDepth:16,chunk:256,cancelled,progress,yield:()=>new Promise<void>(resolve=>setTimeout(resolve,0))},ownership:new NativeDecodeControl(256*1024,()=>!cancelled())};
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

/** 🏷️ The localized noun for each declared port type, so a refusal reads in the user's own language
 * instead of echoing the schema id. An id the table does not carry falls back to the id itself. */
export function usePortTypeLabels(): Readonly<Record<string, string>> {
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

export function syncOptionalGraphCanvasTheme(session: FrameworkGraphSession | null): void {
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

export function publishNodeGraphSelection(dispatch: (action: string, args?: Record<string, unknown>) => unknown, domain: NodeGraphInteractionDomain | undefined, ids: NodeGraphInteractionIds): void {
  const args = nodeGraphSelectionActionArgs(domain, ids);
  if (args) dispatch(nodeGraphActions.select, args);
}

export function publishNodeGraphHover(
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
export const DEFAULT_NODE_GRAPH_VIEWPORT: Viewport2d = { x: 0, y: 0, zoom: 1 };

/** 🔎️ Resolves a flow host snapshot widget id to the workflow instance id it previews, used to open an app instance without depending on plugin-side selection state. */
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

//#region FlowCatalogueSpotlight
/** 🔗️ The `nodeGraphEdit` rows a host journal answered (`{"operations":[…]}`, design §13.3: `connect`, `disconnect`,
 * `move`, `setSlider`, `insertPort`); none for anything else. */
export function nodeGraphEditRows(json: string): readonly Record<string, unknown>[] {
  try {
    const operations = (JSON.parse(json) as { readonly operations?: unknown } | null)?.operations;
    return Array.isArray(operations) ? operations.filter((row): row is Record<string, unknown> => typeof row === "object" && row !== null && typeof (row as { operation?: unknown }).operation === "string") : [];
  } catch {
    return [];
  }
}

/** ➕️ The `addWidget` add-node record of a catalogue descriptor JSON dropped or committed at world `(x, y)` — the
 * same arguments the wgpu renderer dispatches for a catalogue drop; the owner mints the widget id. */
export function flowAddWidgetArgs(descriptorJson: string, x: number, y: number): Record<string, unknown> {
  let descriptor: Record<string, unknown> = {};
  try {
    const parsed = JSON.parse(descriptorJson) as unknown;
    if (typeof parsed === "object" && parsed !== null) descriptor = parsed as Record<string, unknown>;
  } catch {
    descriptor = {};
  }
  const text = (key: string) => (typeof descriptor[key] === "string" ? { [key]: descriptor[key] } : {});
  return { kind: typeof descriptor.kind === "string" ? descriptor.kind : "inputSlider", ...text("neuronKind"), ...text("label"), ...text("action"), ...text("format"), x, y };
}
//#endregion FlowCatalogueSpotlight

export function portLabel(port: NodeGraphPortRecord): string {
  if (port.label) return port.label;
  const segments = port.id.split("@");
  return segments[segments.length - 1] ?? port.id;
}

// 🩹️ `port.id` is the wire-level `nodeId@portId` key (see `NodeGraphPortRecord`), but React Flow's
// `Handle id` must match `sourceHandle`/`targetHandle`, which carry the bare port id (`NodeGraphEdgeRecord.
// sourcePortId`/`targetPortId`) — strip the node-id prefix here so per-port anchoring and onConnect's
// round-trip back to `sourcePortId`/`targetPortId` both resolve against the same bare id.
export function portHandleId(port: NodeGraphPortRecord): string {
  const segments = port.id.split("@");
  return segments[segments.length - 1] ?? port.id;
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
export type GraphKeyboardPort = {
  readonly selectedIds: () => readonly string[];
  readonly focus: (nodeId: string) => void;
  readonly select: (nodeIds: readonly string[], focusedId: string | null) => void;
};

/** ♿️ The canvas node-graph surfaces' side of `🕸️Diagram`'s keyboard law: the SAME
 * {@link diagramKeyboardStep} decides, the surface's {@link GraphKeyboardPort} applies and publishes it
 * like a pointer pick, and the SAME localized sentence ({@link diagramKeyboardAnnouncement}) is spoken.
 * A key the React Flow fallback already handled (`defaultPrevented`) is left alone so a graph never takes
 * two steps for one press. */
export function handleGraphKeyboard(
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
//#endregion WasmGraphSurface

//#region 🔖️flow-graph-canvas-host
//#region Sync
export function flowJsonText(value: unknown): string {
  return typeof value === "string" ? value : JSON.stringify(value ?? null);
}

export function flowBoolean(value: unknown): boolean {
  return value === true || value === 1 || value === "\u0001";
}
//#endregion Sync
//#endregion 🔖️flow-graph-canvas-host
//#endregion 🔖️NodeGraphHost
