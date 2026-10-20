/** 🔸️ The dispatch graph edits phase of WasmGraphSurface. */
import React, { useCallback, useEffect } from "react";
import { useCanvasPickInteraction, type CanvasPickTarget } from "@semio-tech/ui-react";
import { nodeGraphActions } from "@semio-tech/framework";
import { type GraphKeyboardPort, nodeGraphEditRows, nodeGraphPickChannel, publishNodeGraphHover } from "../../🧰️graph-core/🟦️.ts";
import { parseDagNodeIdArray } from "../../🖼️dag-overlays/🟦️.tsx";
import type { useWindowInstanceIdState } from "../🔶️window-instance-id/🟦️.tsx";

/** 🔹 Builds dispatchGraphEdits, pickInteraction of WasmGraphSurface. */
export function useDispatchGraphEditsState({ keyboardPort, sessionRef, paintOverlays, emitInteractionState, dispatch, containerRef, sceneRef }: { readonly keyboardPort: React.MutableRefObject<GraphKeyboardPort | null> } & Pick<ReturnType<typeof useWindowInstanceIdState>, "sessionRef" | "paintOverlays" | "emitInteractionState" | "dispatch" | "containerRef" | "sceneRef">) {
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

  /** 🔗️ Dispatches what the last gesture (or align) did to the graph — the rows the shared DAG journal narrates
   * (`connect`, `disconnect`, `move`, `setSlider`, `insertPort`; design §13.3), the very rows the wgpu renderer writes
   * from the same journal — and nothing when it did nothing. The whole fixture is never published. */
  const dispatchGraphEdits = useCallback(() => {
    const session = sessionRef.current;
    if (!session?.takeGraphEditsJson) return;
    const operations = nodeGraphEditRows(session.takeGraphEditsJson());
    if (operations.length > 0) dispatch(nodeGraphActions.edit, { operations });
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
      } catch {}
      session.renderFrame();
      paintOverlays();
    },
    onSelectTarget: () => {
      emitInteractionState();
    },
  });
  return { pickInteraction, dispatchGraphEdits };
}
