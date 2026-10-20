/** 🔶️ The force config phase of DiagramInner. */
import type { Node, Edge, OnNodeDrag, ReactFlowInstance, OnSelectionChangeParams } from "@xyflow/react";
import * as React from "react";
import { useStoreApi, applyNodeChanges } from "@xyflow/react";
import { reactHostPort } from "../../../🔌️Ports/⚛️react/🟦️.ts";
import { useLabel } from "../../../🏷️Label/🟦️.tsx";
import { cn } from "../../../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { type DiagramForceConfig, type DiagramForceSimulation, defaultDiagramForceConfig } from "../../📋️force-model/🟦️.ts";
import { DiagramHandoffQueue, type DiagramHandoffStatus } from "../../🤝️handoff/🟦️.ts";
import { type DiagramForceViewportPage, type ForceNode, diagramForceHostPage, queryDiagramElement } from "../../🔭️live-force/🟦️.ts";
import { DIAGRAM_KEYBOARD_FOCUS_CLASS, diagramKeyboardAnnouncement, diagramKeyboardStep, useDiagramTranslate } from "../../⌨️keyboard/🟦️.tsx";

/** 🔷 Builds forceConfig, simulationRef, forcePageRef, draggingNodeRef of DiagramInner. */
export function useForceConfigState({ forceConfigProp, handoffStatusRef, controlledNodes, controlledEdges, initialNodes, initialEdges, onNodesChangeReactFlow, onNodeDragStartProp, onNodeDragProp, onNodeDragStopProp, onInitProp, onConnect, onMoveStart, onMoveEnd, onSelectionChange, reactFlowInstanceRef }: { readonly forceConfigProp: Partial<DiagramForceConfig> | undefined; readonly handoffStatusRef: React.RefObject<DiagramHandoffStatus | null> | undefined; readonly controlledNodes: Node[] | undefined; readonly controlledEdges: Edge[] | undefined; readonly initialNodes: Node[]; readonly initialEdges: Edge[]; readonly onNodesChangeReactFlow: ((changes: any[]) => void) | undefined; readonly onNodeDragStartProp: OnNodeDrag<Node> | undefined; readonly onNodeDragProp: OnNodeDrag<Node> | undefined; readonly onNodeDragStopProp: OnNodeDrag<Node> | undefined; readonly onInitProp: ((instance: ReactFlowInstance) => void) | undefined; readonly onConnect: ((connection: any) => void) | undefined; readonly onMoveStart: (() => void) | undefined; readonly onMoveEnd: (() => void) | undefined; readonly onSelectionChange: ((selection: OnSelectionChangeParams) => void) | undefined; readonly reactFlowInstanceRef: React.RefObject<ReactFlowInstance | null> | undefined }) {
  const forceConfig = reactHostPort.useMemo(() => ({ ...defaultDiagramForceConfig, ...forceConfigProp }), [forceConfigProp]);
  const simulationRef = reactHostPort.useRef<DiagramForceSimulation<ForceNode> | null>(null);
  const forcePageRef = reactHostPort.useRef<DiagramForceViewportPage | null>(null);
  const draggingNodeRef = reactHostPort.useRef(false);
  const dragOriginRef = reactHostPort.useRef({ x: 0, y: 0 });
  const dragOffsetRef = reactHostPort.useRef({ x: 0, y: 0 });
  const dragHandoffGenerationRef = reactHostPort.useRef(0);
  const handoffRef = reactHostPort.useRef<DiagramHandoffQueue | null>(null);
  handoffRef.current ??= new DiagramHandoffQueue();
  const handoff = handoffRef.current;
  if (handoffStatusRef) (handoffStatusRef as React.MutableRefObject<DiagramHandoffStatus | null>).current = handoff;
  reactHostPort.useEffect(() => () => handoff.stop(), [handoff]);
  const isControlled = controlledNodes !== undefined && controlledEdges !== undefined;
  const rfStoreApi = useStoreApi();
  reactHostPort.useEffect(() => {
    const original = rfStoreApi.setState;
    const api = rfStoreApi as any;
    api.__suppressTransform = false;
    api.__pendingTransform = null;
    api.__original = original;
    rfStoreApi.setState = ((partial: any, replace: any) => {
      if (typeof partial === "object" && partial !== null && !replace) {
        const state = rfStoreApi.getState();
        const keys = Object.keys(partial);
        if (keys.length > 0 && keys.every((k) => Object.is((state as any)[k], partial[k]))) return;
        if (api.__suppressTransform && keys.length === 1 && keys[0] === "transform") {
          const t = partial.transform;
          const el = queryDiagramElement<HTMLElement>(".react-flow__viewport");
          if (el) el.style.transform = `translate(${t[0]}px, ${t[1]}px) scale(${t[2]})`;
          api.__pendingTransform = t;
          return;
        }
      }
      return original(partial, replace);
    }) as typeof original;
    return () => {
      rfStoreApi.setState = original;
    };
  }, [rfStoreApi]);

  const diagramLabel = useLabel("ui.diagram.label");
  const diagramRoleDescriptionLabel = useLabel("ui.diagram.roleDescription");
  const diagramKeyboardHelpLabel = useLabel("ui.diagram.keyboardHelp");
  const diagramNodesLabel = useLabel("ui.diagram.nodes");
  const diagramEdgesLabel = useLabel("ui.diagram.edges");
  const diagramKeyboardHelpId = reactHostPort.useId();
  const [keyboardFocusedNodeId, setKeyboardFocusedNodeId] = reactHostPort.useState<string | null>(null);
  const [keyboardAnnouncement, setKeyboardAnnouncement] = reactHostPort.useState("");
  const diagramTranslate = useDiagramTranslate();

  const [internalNodes, setInternalNodes] = reactHostPort.useState<Node[]>(initialNodes);
  const [internalEdges, setInternalEdges] = reactHostPort.useState<Edge[]>(initialEdges);

  const finalNodes = isControlled ? controlledNodes : internalNodes;
  const finalEdges = isControlled ? controlledEdges : internalEdges;
  const [hostNodes, setHostNodes] = reactHostPort.useState<Node[]>([]);
  const [hostEdges, setHostEdges] = reactHostPort.useState<Edge[]>([]);
  const virtualizedHost = forceConfig.enabled && finalNodes.length > diagramForceHostPage.maxNodes;
  const renderedNodes = virtualizedHost ? hostNodes : finalNodes;
  const renderedEdges = virtualizedHost ? hostEdges : finalEdges;
  const renderedNodesRef = reactHostPort.useRef(renderedNodes);
  renderedNodesRef.current = renderedNodes;
  const keyboardFocusedNodeIdRef = reactHostPort.useRef(keyboardFocusedNodeId);
  keyboardFocusedNodeIdRef.current = keyboardFocusedNodeId;
  const accessibleNodes = reactHostPort.useMemo(
    () => (keyboardFocusedNodeId === null ? renderedNodes : renderedNodes.map((node) => (node.id === keyboardFocusedNodeId ? { ...node, className: cn(node.className, DIAGRAM_KEYBOARD_FOCUS_CLASS), domAttributes: { ...node.domAttributes, "data-keyboard-focused": "" } } : node))),
    [keyboardFocusedNodeId, renderedNodes],
  );

  const onNodesChangeReactFlowRef = reactHostPort.useRef(onNodesChangeReactFlow);
  onNodesChangeReactFlowRef.current = onNodesChangeReactFlow;
  const onNodeDragStartPropRef = reactHostPort.useRef(onNodeDragStartProp);
  onNodeDragStartPropRef.current = onNodeDragStartProp;
  const onNodeDragPropRef = reactHostPort.useRef(onNodeDragProp);
  onNodeDragPropRef.current = onNodeDragProp;
  const onNodeDragStopPropRef = reactHostPort.useRef(onNodeDragStopProp);
  onNodeDragStopPropRef.current = onNodeDragStopProp;
  const onInitPropRef = reactHostPort.useRef(onInitProp);
  onInitPropRef.current = onInitProp;
  const onConnectRef = reactHostPort.useRef(onConnect);
  onConnectRef.current = onConnect;
  const onMoveStartRef = reactHostPort.useRef(onMoveStart);
  onMoveStartRef.current = onMoveStart;
  const onMoveEndRef = reactHostPort.useRef(onMoveEnd);
  onMoveEndRef.current = onMoveEnd;
  const onSelectionChangeRef = reactHostPort.useRef(onSelectionChange);
  onSelectionChangeRef.current = onSelectionChange;
  const handleNodesChange = reactHostPort.useCallback(
    (changes: any[]) => {
      onNodesChangeReactFlowRef.current?.(changes);
      if (!isControlled) {
        setInternalNodes((nds) => applyNodeChanges(changes, nds));
      }
    },
    [isControlled],
  );
  const handleNodesChangeRef = reactHostPort.useRef(handleNodesChange);
  handleNodesChangeRef.current = handleNodesChange;

  /**
   * ♿️ The `application` role's side of the bargain: arrows walk nodes, Enter selects (Shift+Enter adds
   * or removes), Escape clears. Selection travels as a real `select` node change through the SAME
   * `handleNodesChange` lane a mouse pick uses, so a keyboard-driven selection is indistinguishable
   * downstream from a pointer-driven one — no second selection path to drift.
   */
  const handleDiagramKeyDown = reactHostPort.useCallback(
    (event: React.KeyboardEvent<HTMLDivElement>) => {
      const target = event.target as HTMLElement | null;
      if (target && (target.tagName === "INPUT" || target.tagName === "TEXTAREA" || target.isContentEditable)) return;
      const nodes = renderedNodesRef.current;
      const selectedIds = nodes.filter((node) => node.selected).map((node) => node.id);
      const step = diagramKeyboardStep(nodes, keyboardFocusedNodeIdRef.current, selectedIds, event.key, event.shiftKey, true);
      if (!step) return;
      event.preventDefault();
      const labelOf = (id: string) => {
        const label = nodes.find((node) => node.id === id)?.data?.label;
        return typeof label === "string" && label.length > 0 ? label : id;
      };
      setKeyboardAnnouncement(diagramKeyboardAnnouncement(step, nodes, labelOf, diagramTranslate));
      if (step.kind === "focus") {
        setKeyboardFocusedNodeId(step.focusedId);
        return;
      }
      if (step.kind === "clear") {
        setKeyboardFocusedNodeId(null);
        handleNodesChangeRef.current(nodes.filter((node) => node.selected).map((node) => ({ id: node.id, type: "select", selected: false })));
        return;
      }
      handleNodesChangeRef.current(nodes.filter((node) => Boolean(node.selected) !== step.selectedIds.includes(node.id)).map((node) => ({ id: node.id, type: "select", selected: step.selectedIds.includes(node.id) })));
    },
    [diagramTranslate],
  );

  const handleEdgesChange = reactHostPort.useCallback(
    (changes: any[]) => {
      if (!isControlled) {
        setInternalEdges((eds) => {
          const updated = [...eds];
          for (const change of changes) {
            if (change.type === "remove") {
              const idx = updated.findIndex((e) => e.id === change.id);
              if (idx !== -1) updated.splice(idx, 1);
            }
          }
          return updated;
        });
      }
    },
    [isControlled],
  );

  const handleInit = reactHostPort.useCallback(
    (instance: ReactFlowInstance) => {
      if (reactFlowInstanceRef) {
        (reactFlowInstanceRef as any).current = instance;
      }
      onInitPropRef.current?.(instance);
    },
    [reactFlowInstanceRef],
  );
  return { draggingNodeRef, dragOriginRef, dragOffsetRef, dragHandoffGenerationRef, handoff, virtualizedHost, forcePageRef, forceConfig, simulationRef, onNodeDragStartPropRef, onNodeDragPropRef, onNodeDragStopPropRef, onConnectRef, onMoveStartRef, onMoveEndRef, onSelectionChangeRef, finalNodes, finalEdges, setHostNodes, setHostEdges, isControlled, setInternalNodes, setInternalEdges, internalNodes, internalEdges, diagramRoleDescriptionLabel, diagramLabel, renderedNodes, diagramNodesLabel, renderedEdges, diagramEdgesLabel, diagramKeyboardHelpId, keyboardFocusedNodeId, handleDiagramKeyDown, diagramKeyboardHelpLabel, keyboardAnnouncement, accessibleNodes, handleNodesChange, handleEdgesChange, handleInit };
}
