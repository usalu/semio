/** 🔺️ jack rename-node/🔺️diff — mirror of the single-field node patch delta builder. */
import type { RenameNode } from "../🟦️.ts";

export function diff(payload: RenameNode): { nodes: { patched: Array<{ id: string; patch: { name: string } }> } } {
  return { nodes: { patched: [{ id: payload.id, patch: { name: payload.new_name } }] } };
}
