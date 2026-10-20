/** 🔸️ The handle node drag phase of DiagramInner. */
import type { Node, OnSelectionChangeParams } from "@xyflow/react";
import { reactHostPort } from "../../../🔌️Ports/⚛️react/🟦️.ts";
import { offsetDiagramForceSelection, setDiagramForceViewport } from "../../🔭️live-force/🟦️.ts";
import type { useForceConfigState } from "../🔶️force-config/🟦️.tsx";

/** 🔹 Builds handleNodeDragStart, handleNodeDrag, handleNodeDragStop, stableOnConnect of DiagramInner. */
export function useHandleNodeDragState({ draggingNodeRef, dragOriginRef, dragOffsetRef, dragHandoffGenerationRef, handoff, virtualizedHost, forcePageRef, forceConfig, simulationRef, onNodeDragStartPropRef, onNodeDragPropRef, onNodeDragStopPropRef, onConnectRef, onMoveStartRef, onMoveEndRef, onSelectionChangeRef }: Pick<ReturnType<typeof useForceConfigState>, "draggingNodeRef" | "dragOriginRef" | "dragOffsetRef" | "dragHandoffGenerationRef" | "handoff" | "virtualizedHost" | "forcePageRef" | "forceConfig" | "simulationRef" | "onNodeDragStartPropRef" | "onNodeDragPropRef" | "onNodeDragStopPropRef" | "onConnectRef" | "onMoveStartRef" | "onMoveEndRef" | "onSelectionChangeRef">) {
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
  return { stableOnConnect, handleNodeDragStart, handleNodeDrag, handleNodeDragStop, stableOnMoveStart, stableOnMoveEnd, stableOnSelectionChange };
}
