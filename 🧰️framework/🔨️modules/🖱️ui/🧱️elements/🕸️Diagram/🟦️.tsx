// #region 🧲️Header
// 💻️ framework/ui/elements/📊️Diagram/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import type { Connection, ConnectionLineComponentProps, Edge, EdgeProps, EdgeTypes, MiniMapNodeProps, Node, NodeProps, NodeTypes, OnSelectionChangeParams, ReactFlowInstance } from "@xyflow/react";
import { applyNodeChanges, Background, BackgroundVariant, BaseEdge, getBezierPath, Handle, Position, ReactFlow, ReactFlowProvider, SelectionMode, useInternalNode, useReactFlow, useStoreApi, ViewportPortal } from "@xyflow/react";
import { reactHostPort } from "../🔌️Ports/🟦️.tsx";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { surfaceClass } from "../../🔨️modules/🌈️surface-presentation/🟦️.ts";
import { loadingBorderClass } from "../../🔨️modules/🌀️status-border-presentation/🟦️.ts";
import { HostReactFlow, HostReactFlowProvider } from "../🔌️Ports/🟦️.tsx";
import { DiagramInner, type DiagramProps } from "./🧩️inner/🟦️.tsx";
export { DIAGRAM_UNIT, DIAGRAM_LAYOUT_CODEC_KIND, DIAGRAM_LAYOUT_INGRESS_BYTES, DIAGRAM_LAYOUT_INGRESS_ITEMS, DIAGRAM_LAYOUT_MAX_EDGE_BYTES, DIAGRAM_LAYOUT_MAX_ID_CHARACTERS, DIAGRAM_LAYOUT_MAX_INPUT_ITEMS, DIAGRAM_LAYOUT_MAX_NODE_BYTES, DIAGRAM_LAYOUT_MAX_RESERVED_BYTES, DIAGRAM_LAYOUT_OUTPUT_ITEMS } from "./📐️layout/📜️wire/🟦️.ts";
export type { DiagramLayoutDescriptor, DiagramLayoutDirection, DiagramLayoutEdgeWire, DiagramLayoutIngressPage, DiagramLayoutNodeWire, DiagramLayoutOptions, DiagramLayoutPositionPage, DiagramLayoutTerminal } from "./📐️layout/📜️wire/🟦️.ts";
export type { DiagramLayoutPosition } from "./📐️layout/📜️wire/🟦️.ts";
// #endregion 🔌️Adapters

// Interactive node-edge diagram built on ReactFlow and owned layout ports.
// Consumers MUST provide nodes and edges arrays.

export { applyNodeChanges, Background, BackgroundVariant, BaseEdge, getBezierPath, Handle, Position, ReactFlow, ReactFlowProvider, useInternalNode, useReactFlow, useStoreApi, ViewportPortal };
export type { Connection, ConnectionLineComponentProps, Edge, EdgeProps, EdgeTypes, MiniMapNodeProps, Node, NodeProps, NodeTypes, ReactFlowInstance, Connection as RFConnection };

/**
 * Diagram holds the data fields for a Diagram record.
 **/
const Diagram: React.FC<DiagramProps> = (props) => {
  return (
    <HostReactFlowProvider>
      <DiagramInner {...props} />
    </HostReactFlowProvider>
  );
};

export { Diagram, SelectionMode };
export type { OnSelectionChangeParams };

/**
 * DiagramSkeletonProps holds the data fields for a DiagramSkeletonProps record.
 **/
interface DiagramSkeletonProps {
  nodeCount?: number;
  edgeCount?: number;
  className?: string;
}

/**
 * Skeleton loading placeholder for a diagram.
 **/
export const DiagramSkeleton: React.FC<DiagramSkeletonProps> = ({ nodeCount = 5, edgeCount = 4, className = "" }) => {
  const skeletonNodes: Node[] = reactHostPort.useMemo(
    () =>
      Array.from({ length: nodeCount }).map((_, i) => ({
        id: `skeleton-node-${i}`,
        type: "default",
        position: { x: (i % 3) * 150 + 50, y: Math.floor(i / 3) * 150 + 50 },
        data: { label: " " },
        draggable: false,
      })),
    [nodeCount],
  );
  const skeletonEdges: Edge[] = reactHostPort.useMemo(
    () =>
      Array.from({ length: edgeCount }).map((_, i) => ({
        id: `skeleton-edge-${i}`,
        source: `skeleton-node-${i}`,
        target: `skeleton-node-${Math.min(i + 1, nodeCount - 1)}`,
        animated: false,
      })),
    [edgeCount, nodeCount],
  );
  return (
    <div className={cn("relative w-full h-full", loadingBorderClass, className)}>
      <HostReactFlow
        nodes={skeletonNodes}
        edges={skeletonEdges}
        nodeTypes={{}}
        edgeTypes={{}}
        nodesDraggable={false}
        elementsSelectable={false}
        panOnDrag={false}
        zoomOnScroll={false}
        zoomOnPinch={false}
        proOptions={{ hideAttribution: true }}
        className={cn(surfaceClass, "animate-pulse opacity-50")}
      ></HostReactFlow>
    </div>
  );
};