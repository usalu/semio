/** ↩️ jack create-node/↩️inverse — mirror of the id-only delete-node inverse builder. */
import type { CreateNode } from "../🦠️mutation/🟦️component.ts";

export function inverse(payload: CreateNode): Array<{ id: string }> {
  return [{ id: payload.node.id }];
}
