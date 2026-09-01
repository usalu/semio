/** ↩️ jack create-edge/↩️inverse — mirror of the id-only delete-edge inverse builder. */
import type { CreateEdge } from "../🟦️component.ts";

export function inverse(payload: CreateEdge): Array<{ id: string }> {
  return [{ id: payload.edge.id }];
}
