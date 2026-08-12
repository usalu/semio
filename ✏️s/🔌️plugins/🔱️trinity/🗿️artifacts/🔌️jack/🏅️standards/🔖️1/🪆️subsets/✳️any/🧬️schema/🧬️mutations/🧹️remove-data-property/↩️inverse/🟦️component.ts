/** ↩️ jack remove-data-property/↩️inverse — mirror of the BASE-lookup old-value restore inverse. */
import type { RemoveDataProperty } from "../🦠️mutation/🟦️component.ts";
import type { ChangeDataProperty } from "../../🔧️change-data-property/🦠️mutation/🟦️component.ts";

export function inverse(payload: RemoveDataProperty, baseValue: unknown): ChangeDataProperty[] {
  return baseValue === undefined ? [] : [{ entity: payload.entity, key: payload.key, newValue: baseValue }];
}
