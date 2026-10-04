import type {PropertyValue} from "../../../🟦️.ts";
/** ↩️ rewriting remove-parameter-binding/↩️inverse — mirror of the BASE-lookup restore inverse. */
import type { RemoveParameterBinding } from "../🟦️.ts";
import type { ChangeParameterBinding } from "../../🔧️change-parameter/🟦️.ts";

export function inverse(payload: RemoveParameterBinding, baseValue: PropertyValue | undefined): ChangeParameterBinding[] {
  return baseValue === undefined ? [] : [{ key: payload.key, newValue: baseValue }];
}
