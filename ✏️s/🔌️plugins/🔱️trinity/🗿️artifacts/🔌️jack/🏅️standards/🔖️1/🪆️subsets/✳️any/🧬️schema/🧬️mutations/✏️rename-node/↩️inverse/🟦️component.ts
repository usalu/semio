/** ↩️ jack rename-node/↩️inverse — mirror of the BASE-lookup old-name inverse builder. */
import type { RenameNode } from "../🦠️mutation/🟦️component.ts";

export function inverse(payload: RenameNode, baseName: string | undefined): RenameNode[] {
  return baseName === undefined ? [] : [{ id: payload.id, newName: baseName }];
}
