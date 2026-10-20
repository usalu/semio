/** 🟠️ The part3 phase of DiagramInner. */
import type { Node, Edge, ReactFlowInstance } from "@xyflow/react";
import * as React from "react";
import { reactHostPort } from "../../../🔌️Ports/⚛️react/🟦️.ts";
import { diagramForceBudget } from "../../📋️force-model/🟦️.ts";
import { type DiagramForceProjection, createLiveDiagramForceSimulation, diagramForceHostPage } from "../../🔭️live-force/🟦️.ts";
import { finiteForceValue } from "../../🧮️force-simulation/🟦️.ts";
import type { useForceConfigState } from "../🔶️force-config/🟦️.tsx";

/** 🟡 Runs the effects and registrations of DiagramInner. */
export function usePart3({ forceConfig, finalNodes, simulationRef, finalEdges, defaultViewport, forcePageRef, handoff, setHostNodes, setHostEdges, isControlled, onNodesChangeProp, setInternalNodes, focusedItemId, reactFlowInstanceRef, onFocusComplete, initialNodes, setInternalEdges, initialEdges, internalNodes }: Pick<ReturnType<typeof useForceConfigState>, "forceConfig" | "finalNodes" | "simulationRef" | "finalEdges" | "forcePageRef" | "handoff" | "setHostNodes" | "setHostEdges" | "isControlled" | "setInternalNodes" | "setInternalEdges" | "internalNodes"> & { readonly defaultViewport: { x: number; y: number; zoom: number; } | undefined; readonly onNodesChangeProp: ((nodes: Node[]) => void) | undefined; readonly focusedItemId: string | undefined; readonly reactFlowInstanceRef: React.RefObject<ReactFlowInstance | null> | undefined; readonly onFocusComplete: (() => void) | undefined; readonly initialNodes: Node[]; readonly initialEdges: Edge[] }) {
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
}
