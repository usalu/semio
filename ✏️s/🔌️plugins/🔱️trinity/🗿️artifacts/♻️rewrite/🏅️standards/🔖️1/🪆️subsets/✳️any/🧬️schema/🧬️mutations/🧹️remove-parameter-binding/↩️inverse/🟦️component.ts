/** ↩️ rewrite remove-parameter-binding/↩️inverse — mirror of the BASE-lookup restore inverse. */
import type { RemoveParameterBinding } from "../🦠️mutation/🟦️component.ts";
import type { ChangeParameterBinding } from "../../🔧️change-parameter-binding/🦠️mutation/🟦️component.ts";

export function inverse(payload: RemoveParameterBinding, baseValue: unknown): ChangeParameterBinding[] {
  return baseValue === undefined ? [] : [{ key: payload.key, newValue: baseValue }];
}
