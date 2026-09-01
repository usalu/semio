/** ↩️ rewrite change-parameter-binding/↩️inverse — mirror of the BASE-lookup old-value inverse. */
import type { ChangeParameterBinding } from "../🟦️component.ts";
import type { RemoveParameterBinding } from "../../🧹️remove-parameter-binding/🟦️component.ts";

export function inverse(payload: ChangeParameterBinding, baseValue: unknown): [ChangeParameterBinding] | [RemoveParameterBinding] {
  return baseValue === undefined ? [{ key: payload.key }] : [{ key: payload.key, newValue: baseValue }];
}
