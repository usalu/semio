/** 🔺️ jack change-data-property/🔺️diff — mirror of the entity-dispatched key/valueJson patch. */
import type { ChangeDataProperty } from "../🟦️component.ts";

export function diff(payload: ChangeDataProperty): { nodes?: { patched: unknown[] }; edges?: { patched: unknown[] } } {
  const patch = { key: payload.key, valueJson: JSON.stringify(payload.new_value) };
  return payload.entity.entity === "node" ? { nodes: { patched: [{ id: payload.entity.id, patch }] } } : { edges: { patched: [{ id: payload.entity.id, patch }] } };
}
