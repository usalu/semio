/** ↩️ jack rename-node/↩️inverse — mirror of the BASE-lookup old-name inverse builder. */
import type { RenameNode } from "../🟦️.ts";

export function inverse(payload: RenameNode, baseName: string | undefined): RenameNode[] {
  return baseName === undefined ? [] : [{ id: payload.id, new_name: baseName }];
}
