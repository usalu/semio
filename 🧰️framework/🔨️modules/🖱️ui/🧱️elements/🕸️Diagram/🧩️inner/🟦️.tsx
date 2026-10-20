import type { NodeTypes, EdgeTypes, Node, Edge, Connection, OnNodeDrag, ReactFlowInstance, OnSelectionChangeParams } from "@xyflow/react";
import * as React from "react";
import { BackgroundVariant, SelectionMode, ConnectionMode, MiniMap } from "@xyflow/react";
import { HostReactFlow } from "../../🔌️Ports/🟦️.tsx";
import { surfaceClass } from "../../../🔨️modules/🌈️surface-presentation/🟦️.ts";
import { type DiagramForceConfig } from "../📋️force-model/🟦️.ts";
import { type DiagramHandoffStatus } from "../🤝️handoff/🟦️.ts";
import { DiagramLiveRegion } from "../⌨️keyboard/🟦️.tsx";
import { useForceConfigState } from "./🔶️force-config/🟦️.tsx";
import { useHandleNodeDragState } from "./🔸️handle-node-drag/🟦️.tsx";
import { usePart3 } from "./🟠️part3/🟦️.tsx";
import { usePart4 } from "./🟢️part4/🟦️.tsx";

/**
 * 📊️ Props interface for the Diagram component.
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
 * 📈️ DiagramInner holds the data fields for a DiagramInner record.
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
  const { draggingNodeRef, dragOriginRef, dragOffsetRef, dragHandoffGenerationRef, handoff, virtualizedHost, forcePageRef, forceConfig, simulationRef, onNodeDragStartPropRef, onNodeDragPropRef, onNodeDragStopPropRef, onConnectRef, onMoveStartRef, onMoveEndRef, onSelectionChangeRef, finalNodes, finalEdges, setHostNodes, setHostEdges, isControlled, setInternalNodes, setInternalEdges, internalNodes, internalEdges, diagramRoleDescriptionLabel, diagramLabel, renderedNodes, diagramNodesLabel, renderedEdges, diagramEdgesLabel, diagramKeyboardHelpId, keyboardFocusedNodeId, handleDiagramKeyDown, diagramKeyboardHelpLabel, keyboardAnnouncement, accessibleNodes, handleNodesChange, handleEdgesChange, handleInit } = useForceConfigState({ forceConfigProp, handoffStatusRef, controlledNodes, controlledEdges, initialNodes, initialEdges, onNodesChangeReactFlow, onNodeDragStartProp, onNodeDragProp, onNodeDragStopProp, onInitProp, onConnect, onMoveStart, onMoveEnd, onSelectionChange, reactFlowInstanceRef });
  const { stableOnConnect, handleNodeDragStart, handleNodeDrag, handleNodeDragStop, stableOnMoveStart, stableOnMoveEnd, stableOnSelectionChange } = useHandleNodeDragState({ draggingNodeRef, dragOriginRef, dragOffsetRef, dragHandoffGenerationRef, handoff, virtualizedHost, forcePageRef, forceConfig, simulationRef, onNodeDragStartPropRef, onNodeDragPropRef, onNodeDragStopPropRef, onConnectRef, onMoveStartRef, onMoveEndRef, onSelectionChangeRef });
  usePart3({ forceConfig, finalNodes, simulationRef, finalEdges, defaultViewport, forcePageRef, handoff, setHostNodes, setHostEdges, isControlled, onNodesChangeProp, setInternalNodes, focusedItemId, reactFlowInstanceRef, onFocusComplete, initialNodes, setInternalEdges, initialEdges, internalNodes });
  usePart4({ isControlled, onEdgesChangeProp, internalEdges, handoff });
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
