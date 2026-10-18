import type { Edge, Node } from "@xyflow/react";
import { DIAGRAM_UNIT } from "../📐️layout/📜️wire/🟦️.ts";
import type { DiagramForceConfig, DiagramForceNode, DiagramForceSimulation, DiagramForceSource } from "../📋️force-model/🟦️.ts";
import { OwnedDiagramForceSimulation, finiteForceValue } from "../🧮️force-simulation/🟦️.ts";

/**
 * ForceNode holds the data fields for a ForceNode record.
 **/
export interface ForceNode extends DiagramForceNode {
  id: string;
  data: any;
}

export interface DiagramForceViewportPage {
  maximumX: number;
  maximumY: number;
  minimumX: number;
  minimumY: number;
  readonly resolvedEdges: Array<{ edge: Edge; sourceIndex: number; targetIndex: number }>;
  readonly selectedNodes: Node[];
}

export const diagramForceHostPage = Object.freeze({ extent: 2_048, maxEdges: 256, maxNodes: 128 });

export function queryDiagramElement<ElementType extends Element>(selector: string): ElementType | null {
  return typeof document === "undefined" ? null : document.querySelector<ElementType>(selector);
}

export function setDiagramForceViewport(page: DiagramForceViewportPage, viewport?: { x: number; y: number; zoom: number }): void {
  const zoom = Math.max(0.01, finiteForceValue(viewport?.zoom, 1));
  page.minimumX = -finiteForceValue(viewport?.x, 0) / zoom - DIAGRAM_UNIT;
  page.minimumY = -finiteForceValue(viewport?.y, 0) / zoom - DIAGRAM_UNIT;
  page.maximumX = page.minimumX + diagramForceHostPage.extent / zoom;
  page.maximumY = page.minimumY + diagramForceHostPage.extent / zoom;
}

export function offsetDiagramForceSelection(nodes: Node[], deltaX: number, deltaY: number): Node[] {
  if (deltaX === 0 && deltaY === 0) return nodes;
  return new Proxy(nodes, {
    get(target, property, receiver) {
      const value = Reflect.get(target, property, receiver);
      if (typeof property !== "string" || !/^(0|[1-9]\d*)$/.test(property) || !value) return value;
      const node = value as Node;
      return { ...node, position: { x: node.position.x + deltaX, y: node.position.y + deltaY } };
    },
  });
}

export function createLiveDiagramForceSimulation(
  nodes: readonly Node[],
  edges: readonly Edge[],
  config: DiagramForceConfig,
  viewport?: { x: number; y: number; zoom: number },
): { page: DiagramForceViewportPage; simulation: DiagramForceSimulation<ForceNode> } {
  const forceNodes: ForceNode[] = [];
  const page: DiagramForceViewportPage = { maximumX: 0, maximumY: 0, minimumX: 0, minimumY: 0, resolvedEdges: [], selectedNodes: [] };
  setDiagramForceViewport(page, viewport);
  const source: DiagramForceSource<ForceNode> = {
    link: (index) => {
      const edge = edges[index]!;
      return { id: edge.id, index, source: edge.source, target: edge.target };
    },
    linkCount: edges.length,
    linkResolved: (link, sourceIndex, targetIndex) => {
      page.resolvedEdges.push({ edge: edges[link.index]!, sourceIndex, targetIndex });
    },
    node: (index) => {
      const node = nodes[index]!;
      if (node.selected) page.selectedNodes.push(node);
      return { data: node.data, id: node.id, x: node.position.x, y: node.position.y };
    },
    nodeCount: nodes.length,
  };
  return { page, simulation: new OwnedDiagramForceSimulation(forceNodes, source, true, config).stop() };
}

export interface DiagramForceProjection {
  cursor: number;
  edgeCursor: number;
  generation: number;
  hostEdges: Edge[];
  hostNodeIndices: Set<number>;
  hostNodes: Node[];
  nodes: Node[];
  phase: "edges" | "nodes";
}
