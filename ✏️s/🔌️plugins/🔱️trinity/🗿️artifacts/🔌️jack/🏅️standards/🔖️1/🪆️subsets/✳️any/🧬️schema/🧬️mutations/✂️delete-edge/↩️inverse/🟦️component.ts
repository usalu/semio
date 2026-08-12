/** ↩️ jack delete-edge/↩️inverse — mirror of the BASE-lookup recreate-edge inverse. */
import type { DeleteEdge } from "../🦠️mutation/🟦️component.ts";
import type { JackEdge } from "../../🔗️create-edge/🦠️mutation/🟦️component.ts";

export function inverse(payload: DeleteEdge, baseEdge: JackEdge | undefined): Array<{ edge: JackEdge }> {
  return baseEdge ? [{ edge: baseEdge }] : [];
}
