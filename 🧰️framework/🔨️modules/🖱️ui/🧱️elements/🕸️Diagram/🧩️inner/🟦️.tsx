import type { NodeTypes, EdgeTypes, Node, Edge, Connection, OnNodeDrag, ReactFlowInstance, OnSelectionChangeParams } from "@xyflow/react";
import * as React from "react";
import { BackgroundVariant, SelectionMode, useStoreApi, applyNodeChanges, ConnectionMode, MiniMap } from "@xyflow/react";
import { reactHostPort } from "../../🔌️Ports/⚛️react/🟦️.ts";
import { useLabel } from "../../🏷️Label/🟦️.tsx";
import { cn } from "../../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { HostReactFlow } from "../../🔌️Ports/🟦️.tsx";
import { surfaceClass } from "../../../🔨️modules/🌈️surface-presentation/🟦️.ts";
import { type DiagramForceConfig, type DiagramForceSimulation, defaultDiagramForceConfig, diagramForceBudget } from "../📋️force-model/🟦️.ts";
import { DiagramHandoffQueue, type DiagramHandoffStatus } from "../🤝️handoff/🟦️.ts";
import { type DiagramForceProjection, type DiagramForceViewportPage, type ForceNode, createLiveDiagramForceSimulation, diagramForceHostPage, offsetDiagramForceSelection, queryDiagramElement, setDiagramForceViewport } from "../🔭️live-force/🟦️.ts";
import { DIAGRAM_KEYBOARD_FOCUS_CLASS, DiagramLiveRegion, diagramKeyboardAnnouncement, diagramKeyboardStep, useDiagramTranslate } from "../⌨️keyboard/🟦️.tsx";
import { finiteForceValue } from "../🧮️force-simulation/🟦️.ts";

/**
 * Props interface for the Diagram component.
 **/
export interface DiagramProps {
  nodeTypes: NodeTypes;
  edgeTypes?: EdgeTypes;
  initialNodes?: Node[];
  initialEdges?: Edge[];
  nodes?: Node[];
  edges?: Edge[];
  onNodesChange?: (nodes: Node[]) => void;
  onEdgesChange?: (edges: Edge[]) => void;
  onNodesChangeReactFlow?: (changes: any[]) => void;
  onEdgesChangeReactFlow?: (changes: any[]) => void;
  onConnect?: (connection: any) => void;
  /** 🚦️ Refuses a connection while it is being dragged — forwarded straight to React Flow, which is
   * the only place it can gate a drop. It was missing from this contract, so `🕸️NodeGraph`'s own
   * validity guard was being dropped at the boundary instead of being asked. */
  isValidConnection?: (connection: Connection | Edge) => boolean;
  onNodeClick?: (event: React.MouseEvent, node: Node) => void;
  onNodeDoubleClick?: (event: React.MouseEvent, node: Node) => void;
  onNodeMouseEnter?: (event: React.MouseEvent, node: Node) => void;
  onNodeMouseLeave?: (event: React.MouseEvent, node: Node) => void;
  onNodeDragStart?: OnNodeDrag<Node>;
  onNodeDrag?: OnNodeDrag<Node>;
  onNodeDragStop?: OnNodeDrag<Node>;
  onEdgeClick?: (event: React.MouseEvent, edge: Edge) => void;
  onEdgeMouseEnter?: (event: React.MouseEvent, edge: Edge) => void;
  onEdgeMouseLeave?: (event: React.MouseEvent, edge: Edge) => void;
  onPaneClick?: (event: React.MouseEvent) => void;
  onPaneDoubleClick?: (event: React.MouseEvent) => void;
  onMoveStart?: () => void;
  onMoveEnd?: () => void;
  reactFlowInstanceRef?: React.RefObject<ReactFlowInstance | null>;
  onInit?: (instance: ReactFlowInstance) => void;
  wrapperRef?: React.RefObject<HTMLDivElement> | ((node: HTMLDivElement | null) => void);
  showBackground?: boolean;
  backgroundVariant?: BackgroundVariant;
  showControls?: boolean;
  showMinimap?: boolean;
  panels?: React.ReactNode;
  className?: string;
  fitView?: boolean;
  minZoom?: number;
  maxZoom?: number;
  defaultZoom?: number;
  connectionMode?: "strict" | "loose";
  connectionLineComponent?: any;
  deleteKeyCode?: string | string[];
  panOnDrag?: boolean | number[];
  selectionOnDrag?: boolean;
  zoomOnScroll?: boolean;
  zoomOnPinch?: boolean;
  zoomOnDoubleClick?: boolean;
  elementsSelectable?: boolean;
  nodesFocusable?: boolean;
  edgesFocusable?: boolean;
  nodesDraggable?: boolean;
  nodesConnectable?: boolean;
  edgesReconnectable?: boolean;
  miniMapNodeComponent?: any;
  focusedItemId?: string;
  onFocusComplete?: () => void;
  forceConfig?: Partial<DiagramForceConfig>;
  handoffStatusRef?: React.RefObject<DiagramHandoffStatus | null>;
  selectionMode?: SelectionMode;
  panOnScroll?: boolean;
  proOptions?: { hideAttribution: boolean };
  onSelectionChange?: (selection: OnSelectionChangeParams) => void;
  onSelectionStart?: (event: React.MouseEvent) => void;
  onSelectionEnd?: (event: React.MouseEvent) => void;
  defaultViewport?: { x: number; y: number; zoom: number };
  autoPanOnNodeDrag?: boolean;
  selectNodesOnDrag?: boolean;
}

/**
 * DiagramInner holds the data fields for a DiagramInner record.
 **/
export const DiagramInner: React.FC<DiagramProps> = ({
  nodeTypes,
  edgeTypes,
  initialNodes = [],
  initialEdges = [],
  nodes: controlledNodes,
  edges: controlledEdges,
  onNodesChange: onNodesChangeProp,
  onEdgesChange: onEdgesChangeProp,
  onNodesChangeReactFlow,
  onEdgesChangeReactFlow,
  onConnect,
  isValidConnection,
  onNodeClick,
  onNodeDoubleClick,
  onNodeMouseEnter,
  onNodeMouseLeave,
  onNodeDragStart: onNodeDragStartProp,
  onNodeDrag: onNodeDragProp,
  onNodeDragStop: onNodeDragStopProp,
  onEdgeClick,
  onEdgeMouseEnter,
  onEdgeMouseLeave,
  onPaneClick,
  onPaneDoubleClick,
  onMoveStart,
  onMoveEnd,
  reactFlowInstanceRef,
  onInit: onInitProp,
  wrapperRef,
  showMinimap = false,
  panels,
  className = "",
  fitView = true,
  minZoom = 0.1,
  maxZoom = 12,
  connectionMode = "loose",
  connectionLineComponent,
  deleteKeyCode = "Delete",
  panOnDrag = [0],
  selectionOnDrag = false,
  zoomOnScroll = true,
  zoomOnPinch = true,
  zoomOnDoubleClick = false,
  elementsSelectable = false,
  nodesFocusable = false,
  edgesFocusable = false,
  nodesDraggable = true,
  nodesConnectable = true,
  edgesReconnectable = true,
  miniMapNodeComponent,
  focusedItemId,
  onFocusComplete,
  forceConfig: forceConfigProp,
  handoffStatusRef,
  selectionMode = SelectionMode.Partial,
  panOnScroll = false,
  proOptions = { hideAttribution: true },
  onSelectionChange,
  onSelectionStart,
  onSelectionEnd,
  defaultViewport,
  autoPanOnNodeDrag,
  selectNodesOnDrag,
}) => {
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

  const handleNodeDragStart = reactHostPort.useCallback(
    (event: MouseEvent | TouchEvent, node: Node, nodes: Node[]) => {
      draggingNodeRef.current = true;
      dragOriginRef.current = node.position;
      dragOffsetRef.current = { x: 0, y: 0 };
      const generation = ++dragHandoffGenerationRef.current;
      handoff.invalidate("drag-start", "drag-move", "drag-stop", "consumer-publication", "host-publication", "state-publication");
      const semanticNodes = virtualizedHost && node.selected && forcePageRef.current?.selectedNodes.length ? forcePageRef.current.selectedNodes : nodes;
      if (forceConfig.enabled && simulationRef.current) {
        simulationRef.current.drag("start", node, semanticNodes).alphaTarget(0.3).restart();
      }
      const consumer = onNodeDragStartPropRef.current;
      if (consumer)
        handoff.enqueue(
          "drag-start",
          consumer,
          () => dragHandoffGenerationRef.current === generation,
          () => consumer(event, node, semanticNodes),
        );
    },
    [forceConfig.enabled, handoff, virtualizedHost],
  );

  const handleNodeDrag = reactHostPort.useCallback(
    (event: MouseEvent | TouchEvent, node: Node, nodes: Node[]) => {
      if (!draggingNodeRef.current) return;
      const generation = dragHandoffGenerationRef.current;
      const delta = { x: node.position.x - dragOriginRef.current.x, y: node.position.y - dragOriginRef.current.y };
      dragOffsetRef.current = delta;
      const fullSelection = virtualizedHost && node.selected && forcePageRef.current?.selectedNodes.length ? forcePageRef.current.selectedNodes : undefined;
      const semanticNodes = fullSelection ? offsetDiagramForceSelection(fullSelection, delta.x, delta.y) : nodes;
      if (forceConfig.enabled && simulationRef.current) simulationRef.current.drag("move", node, semanticNodes);
      const consumer = onNodeDragPropRef.current;
      if (consumer)
        handoff.enqueue(
          "drag-move",
          consumer,
          () => dragHandoffGenerationRef.current === generation,
          () => consumer(event, node, semanticNodes),
        );
    },
    [forceConfig.enabled, handoff, virtualizedHost],
  );

  const handleNodeDragStop = reactHostPort.useCallback(
    (event: MouseEvent | TouchEvent, node: Node, nodes: Node[]) => {
      if (forceConfig.enabled && simulationRef.current) {
        simulationRef.current.drag("stop", node, nodes).alphaTarget(0);
      }
      draggingNodeRef.current = false;
      const generation = dragHandoffGenerationRef.current;
      const fullSelection = virtualizedHost && node.selected && forcePageRef.current?.selectedNodes.length ? forcePageRef.current.selectedNodes : undefined;
      const semanticNodes = fullSelection ? offsetDiagramForceSelection(fullSelection, dragOffsetRef.current.x, dragOffsetRef.current.y) : nodes;
      const consumer = onNodeDragStopPropRef.current;
      if (consumer)
        handoff.enqueue(
          "drag-stop",
          consumer,
          () => dragHandoffGenerationRef.current === generation,
          () => consumer(event, node, semanticNodes),
        );
    },
    [forceConfig.enabled, handoff, virtualizedHost],
  );

  const stableOnConnect = reactHostPort.useCallback((connection: any) => {
    onConnectRef.current?.(connection);
  }, []);
  const stableOnMoveStart = reactHostPort.useCallback(() => {
    onMoveStartRef.current?.();
  }, []);
  const stableOnMoveEnd = reactHostPort.useCallback((_event?: MouseEvent | TouchEvent | null, viewport?: { x: number; y: number; zoom: number }) => {
    if (viewport && forcePageRef.current) setDiagramForceViewport(forcePageRef.current, viewport);
    onMoveEndRef.current?.();
  }, []);
  const stableOnSelectionChange = reactHostPort.useCallback((selection: OnSelectionChangeParams) => {
    onSelectionChangeRef.current?.(selection);
  }, []);

  reactHostPort.useEffect(() => {
    if (!forceConfig.enabled || finalNodes.length === 0) {
      simulationRef.current = null;
      return;
    }

    const { page, simulation } = createLiveDiagramForceSimulation(finalNodes, finalEdges, forceConfig, defaultViewport);
    forcePageRef.current = page;
    const simulationNodes = simulation.nodes();
    let projection: DiagramForceProjection | undefined;
    let active = true;
    let publicationGeneration = -1;

    simulation.on("tick", (frame) => {
      if (!projection || projection.generation !== frame.generation) projection = { cursor: 0, edgeCursor: 0, generation: frame.generation, hostEdges: [], hostNodeIndices: new Set<number>(), hostNodes: [], nodes: [], phase: "nodes" };
      let projected = 0;
      while (projection.phase === "nodes" && projection.cursor < simulationNodes.length && projected < diagramForceBudget.maxProjectionNodesPerFrame && frame.take()) {
        const simulationNode = simulationNodes[projection.cursor]!;
        const original = finalNodes[projection.cursor];
        if (original) {
          const positionedNode = {
            ...original,
            position: {
              x: finiteForceValue(simulationNode.x, original.position.x),
              y: finiteForceValue(simulationNode.y, original.position.y),
            },
          };
          projection.nodes.push(positionedNode);
          if (
            projection.hostNodes.length < diagramForceHostPage.maxNodes &&
            positionedNode.position.x >= page.minimumX &&
            positionedNode.position.x <= page.maximumX &&
            positionedNode.position.y >= page.minimumY &&
            positionedNode.position.y <= page.maximumY
          ) {
            projection.hostNodeIndices.add(projection.cursor);
            projection.hostNodes.push(positionedNode);
          }
        }
        projection.cursor += 1;
        projected += 1;
      }
      if (projection.cursor < simulationNodes.length) return false;
      projection.phase = "edges";
      while (projection.edgeCursor < page.resolvedEdges.length && projected < diagramForceBudget.maxProjectionNodesPerFrame && frame.take()) {
        const resolved = page.resolvedEdges[projection.edgeCursor++]!;
        if (projection.hostEdges.length < diagramForceHostPage.maxEdges && projection.hostNodeIndices.has(resolved.sourceIndex) && projection.hostNodeIndices.has(resolved.targetIndex)) projection.hostEdges.push(resolved.edge);
        projected += 1;
      }
      if (projection.edgeCursor < page.resolvedEdges.length) return false;
      const positionedNodes = projection.nodes;
      publicationGeneration = frame.generation;
      const snapshot = Object.freeze({ generation: frame.generation, hostEdges: projection.hostEdges, hostNodes: projection.hostNodes, nodes: positionedNodes });
      projection = undefined;
      handoff.enqueue(
        "host-publication",
        setHostNodes,
        () => active && publicationGeneration === snapshot.generation && simulationRef.current === simulation,
        () => {
          setHostNodes(snapshot.hostNodes);
          setHostEdges(snapshot.hostEdges);
        },
      );
      const consumer = isControlled ? onNodesChangeProp : setInternalNodes;
      if (consumer)
        handoff.enqueue(
          isControlled ? "consumer-publication" : "state-publication",
          consumer,
          () => active && publicationGeneration === snapshot.generation && simulationRef.current === simulation,
          () => consumer(snapshot.nodes),
        );
      return true;
    });

    simulationRef.current = simulation;
    simulation.restart();

    return () => {
      active = false;
      publicationGeneration += 1;
      handoff.invalidate("consumer-publication", "host-publication", "state-publication");
      simulation.stop();
      if (forcePageRef.current === page) forcePageRef.current = null;
      simulationRef.current = null;
    };
  }, [
    defaultViewport,
    forceConfig.enabled,
    forceConfig.chargeStrength,
    forceConfig.linkDistance,
    forceConfig.collideRadius,
    forceConfig.centerStrength,
    forceConfig.updateIntervalMs,
    finalNodes.length,
    finalEdges.length,
    handoff,
    isControlled,
    onNodesChangeProp,
  ]);

  reactHostPort.useEffect(() => {
    if (focusedItemId && reactFlowInstanceRef?.current) {
      const node = finalNodes.find((n) => n.id === focusedItemId);
      const edge = finalEdges.find((e) => e.id === focusedItemId);

      if (node) {
        reactFlowInstanceRef.current.fitView({
          padding: 0.5,
          duration: 600,
          nodes: [node],
        });
      } else if (edge) {
        const sourceNode = finalNodes.find((n) => n.id === edge.source);
        const targetNode = finalNodes.find((n) => n.id === edge.target);
        const nodesToFit = [sourceNode, targetNode].filter(Boolean) as Node[];
        if (nodesToFit.length > 0) {
          reactFlowInstanceRef.current.fitView({
            padding: 0.5,
            duration: 600,
            nodes: nodesToFit,
          });
        }
      }

      if (onFocusComplete) {
        setTimeout(() => onFocusComplete(), 600);
      }
    }
  }, [focusedItemId, finalNodes, finalEdges, reactFlowInstanceRef, onFocusComplete]);

  reactHostPort.useEffect(() => {
    if (!isControlled) {
      setInternalNodes(initialNodes);
      setInternalEdges(initialEdges);
    }
  }, [initialNodes, initialEdges, isControlled]);

  reactHostPort.useEffect(() => {
    if (isControlled || !onNodesChangeProp) return;
    let active = true;
    const snapshot = internalNodes;
    handoff.enqueue(
      "consumer-publication",
      onNodesChangeProp,
      () => active,
      () => onNodesChangeProp(snapshot),
    );
    return () => {
      active = false;
      handoff.invalidate("consumer-publication");
    };
  }, [handoff, internalNodes, onNodesChangeProp, isControlled]);

  reactHostPort.useEffect(() => {
    if (isControlled || !onEdgesChangeProp) return;
    let active = true;
    const snapshot = internalEdges;
    handoff.enqueue(
      "edge-publication",
      onEdgesChangeProp,
      () => active,
      () => onEdgesChangeProp(snapshot),
    );
    return () => {
      active = false;
      handoff.invalidate("edge-publication");
    };
  }, [handoff, internalEdges, onEdgesChangeProp, isControlled]);

  return (
    <div
      ref={wrapperRef as any}
      className={`relative w-full h-full ${className}`}
      role="application"
      tabIndex={0}
      aria-roledescription={diagramRoleDescriptionLabel}
      aria-label={`${diagramLabel} — ${renderedNodes.length} ${diagramNodesLabel}, ${renderedEdges.length} ${diagramEdgesLabel}`}
      aria-describedby={diagramKeyboardHelpId}
      data-diagram-focused-node={keyboardFocusedNodeId ?? undefined}
      onKeyDown={handleDiagramKeyDown}
    >
      <span id={diagramKeyboardHelpId} className="sr-only">
        {diagramKeyboardHelpLabel}
      </span>
      <DiagramLiveRegion text={keyboardAnnouncement} />
      <HostReactFlow
        nodes={accessibleNodes}
        edges={renderedEdges}
        onNodesChange={handleNodesChange}
        onEdgesChange={handleEdgesChange}
        onConnect={stableOnConnect}
        isValidConnection={isValidConnection}
        onInit={handleInit}
        onNodeClick={onNodeClick}
        onNodeDoubleClick={onNodeDoubleClick}
        onNodeMouseEnter={onNodeMouseEnter}
        onNodeMouseLeave={onNodeMouseLeave}
        onNodeDragStart={handleNodeDragStart}
        onNodeDrag={handleNodeDrag}
        onNodeDragStop={handleNodeDragStop}
        onEdgeClick={onEdgeClick}
        onEdgeMouseEnter={onEdgeMouseEnter}
        onEdgeMouseLeave={onEdgeMouseLeave}
        onPaneClick={onPaneClick}
        onDoubleClick={onPaneDoubleClick}
        onMoveStart={stableOnMoveStart}
        onMoveEnd={stableOnMoveEnd}
        onSelectionChange={stableOnSelectionChange}
        onSelectionStart={onSelectionStart}
        onSelectionEnd={onSelectionEnd}
        nodeTypes={nodeTypes}
        edgeTypes={edgeTypes}
        connectionLineComponent={connectionLineComponent}
        fitView={fitView}
        minZoom={minZoom}
        maxZoom={maxZoom}
        defaultViewport={defaultViewport}
        connectionMode={connectionMode === "loose" ? ConnectionMode.Loose : ConnectionMode.Strict}
        deleteKeyCode={deleteKeyCode}
        panOnDrag={panOnDrag}
        panOnScroll={panOnScroll}
        preventScrolling={true}
        selectionOnDrag={selectionOnDrag}
        selectionMode={selectionMode}
        zoomOnScroll={zoomOnScroll}
        zoomOnPinch={zoomOnPinch}
        zoomOnDoubleClick={zoomOnDoubleClick}
        elementsSelectable={elementsSelectable}
        nodesFocusable={nodesFocusable}
        edgesFocusable={edgesFocusable}
        nodesDraggable={nodesDraggable}
        nodesConnectable={nodesConnectable}
        edgesReconnectable={edgesReconnectable}
        autoPanOnNodeDrag={autoPanOnNodeDrag}
        selectNodesOnDrag={selectNodesOnDrag}
        onlyRenderVisibleElements={true}
        proOptions={proOptions}
        className={surfaceClass}
      >
        {showMinimap && <MiniMap className="border" maskColor="var(--accent)" bgColor="var(--background)" nodeStrokeWidth={3} zoomable pannable nodeComponent={miniMapNodeComponent} />}
        {panels}
      </HostReactFlow>
    </div>
  );
};
