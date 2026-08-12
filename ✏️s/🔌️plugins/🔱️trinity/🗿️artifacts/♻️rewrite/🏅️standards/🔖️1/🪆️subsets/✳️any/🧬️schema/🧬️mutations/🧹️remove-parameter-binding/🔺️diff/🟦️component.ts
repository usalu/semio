/** 🔺️ rewrite remove-parameter-binding/🔺️diff — mirror of the per-key clear delta builder. */
import type { RemoveParameterBinding } from "../🦠️mutation/🟦️component.ts";

export function diff(payload: RemoveParameterBinding): { parameterBindings: Record<string, null> } {
  return { parameterBindings: { [payload.key]: null } };
}
