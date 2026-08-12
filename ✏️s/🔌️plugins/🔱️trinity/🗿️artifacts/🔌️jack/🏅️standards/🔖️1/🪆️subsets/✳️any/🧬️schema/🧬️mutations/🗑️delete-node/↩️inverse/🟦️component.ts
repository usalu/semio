/** ↩️ jack delete-node/↩️inverse — mirror of the BASE-lookup recreate-node(+edges) inverse. */
import type { DeleteNode } from "../🦠️mutation/🟦️component.ts";
import type { JackNode } from "../../🌱️create-node/🦠️mutation/🟦️component.ts";

export function inverse(payload: DeleteNode, baseNode: JackNode | undefined): Array<{ node: JackNode }> {
  return baseNode ? [{ node: baseNode }] : [];
}
