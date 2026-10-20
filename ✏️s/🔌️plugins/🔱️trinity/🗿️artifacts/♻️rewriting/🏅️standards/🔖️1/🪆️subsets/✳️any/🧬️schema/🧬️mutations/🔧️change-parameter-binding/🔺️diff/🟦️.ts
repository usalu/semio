/** 🔺️ rewriting change-parameter-binding/🔺️diff — mirror of the per-key upsert delta builder. */
import type { ChangeParameterBinding } from "../🟦️.ts";
import type {PropertyValue} from "../../../🟦️.ts";

export function diff(payload: ChangeParameterBinding): { parameterBindings: Record<string, PropertyValue> } {
  return { parameterBindings: { [payload.key]: payload.newValue } };
}
