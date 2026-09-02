/** ↩️ jack delete-edge/↩️inverse — mirror of the BASE-lookup recreate-edge inverse. */
import type { DeleteEdge } from "../🟦️.ts";
import type { JackEdge } from "../../🔗️create-edge/🟦️.ts";

export function inverse(payload: DeleteEdge, baseEdge: JackEdge | undefined): Array<{ edge: JackEdge }> {
  return baseEdge ? [{ edge: baseEdge }] : [];
}
