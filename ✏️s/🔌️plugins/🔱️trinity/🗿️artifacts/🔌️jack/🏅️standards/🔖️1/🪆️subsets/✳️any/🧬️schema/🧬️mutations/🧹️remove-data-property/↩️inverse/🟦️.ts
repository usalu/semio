/** ↩️ jack remove-data-property/↩️inverse — mirror of the BASE-lookup old-value restore inverse. */
import type { RemoveDataProperty } from "../🟦️.ts";
import type { ChangeDataProperty } from "../../🔧️change-data-property/🟦️.ts";

export function inverse(payload: RemoveDataProperty, baseValue: unknown): ChangeDataProperty[] {
  return baseValue === undefined ? [] : [{ entity: payload.entity, key: payload.key, new_value: baseValue }];
}
