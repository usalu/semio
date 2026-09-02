/** 🔺️ rewriting remove-parameter-binding/🔺️diff — mirror of the per-key clear delta builder. */
import type { RemoveParameterBinding } from "../🟦️.ts";

export function diff(payload: RemoveParameterBinding): { parameterBindings: Record<string, null> } {
  return { parameterBindings: { [payload.key]: null } };
}
