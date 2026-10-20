/** 🟢️ The part4 phase of DiagramInner. */
import type { Edge } from "@xyflow/react";
import { reactHostPort } from "../../../🔌️Ports/⚛️react/🟦️.ts";
import type { useForceConfigState } from "../🔶️force-config/🟦️.tsx";

/** 🔵 Runs the effects and registrations of DiagramInner. */
export function usePart4({ isControlled, onEdgesChangeProp, internalEdges, handoff }: Pick<ReturnType<typeof useForceConfigState>, "isControlled" | "internalEdges" | "handoff"> & { readonly onEdgesChangeProp: ((edges: Edge[]) => void) | undefined }) {
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
}
