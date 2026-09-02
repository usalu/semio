/** ↩️ rewriting remove-parameter-binding/↩️inverse — mirror of the BASE-lookup restore inverse. */
import type { RemoveParameterBinding } from "../🟦️.ts";
import type { ChangeParameterBinding } from "../../🔧️change-parameter-binding/🟦️.ts";

export function inverse(payload: RemoveParameterBinding, baseValue: unknown): ChangeParameterBinding[] {
  return baseValue === undefined ? [] : [{ key: payload.key, newValue: baseValue }];
}
