/** 🔺️ jack remove-data-property/🔺️diff — mirror of the entity-dispatched key-clear patch. */
import type { RemoveDataProperty } from "../🟦️.ts";

export function diff(payload: RemoveDataProperty): { nodes?: { patched: unknown[] }; edges?: { patched: unknown[] } } {
  const patch = { key: payload.key, valueJson: null };
  return payload.entity.entity === "node" ? { nodes: { patched: [{ id: payload.entity.id, patch }] } } : { edges: { patched: [{ id: payload.entity.id, patch }] } };
}
