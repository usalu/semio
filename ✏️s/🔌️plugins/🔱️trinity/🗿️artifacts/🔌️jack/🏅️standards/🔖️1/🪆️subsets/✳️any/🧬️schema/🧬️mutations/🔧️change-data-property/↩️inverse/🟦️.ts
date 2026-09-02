/** ↩️ jack change-data-property/↩️inverse — mirror of the BASE-lookup old-value inverse builder. */
import type { ChangeDataProperty } from "../🟦️.ts";
import type { RemoveDataProperty } from "../../🧹️remove-data-property/🟦️.ts";

export function inverse(payload: ChangeDataProperty, baseValue: unknown): [ChangeDataProperty] | [RemoveDataProperty] {
  return baseValue === undefined ? [{ entity: payload.entity, key: payload.key }] : [{ entity: payload.entity, key: payload.key, new_value: baseValue }];
}
