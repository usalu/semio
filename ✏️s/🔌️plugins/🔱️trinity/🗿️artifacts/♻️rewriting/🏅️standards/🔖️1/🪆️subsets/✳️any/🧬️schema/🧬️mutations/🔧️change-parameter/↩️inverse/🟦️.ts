/** ↩️ rewriting change-parameter-binding/↩️inverse — mirror of the BASE-lookup old-value inverse. */
import type { ChangeParameterBinding } from "../🟦️.ts";
import type { RemoveParameterBinding } from "../../🧹️remove-parameter-binding/🟦️.ts";

export function inverse(payload: ChangeParameterBinding, baseValue: unknown): [ChangeParameterBinding] | [RemoveParameterBinding] {
  return baseValue === undefined ? [{ key: payload.key }] : [{ key: payload.key, newValue: baseValue }];
}
