/** 🔺️ rewriting change-parameter-binding/🔺️diff — mirror of the per-key upsert delta builder. */
import type { ChangeParameterBinding } from "../🟦️.ts";

export function diff(payload: ChangeParameterBinding): { parameterBindings: Record<string, unknown> } {
  return { parameterBindings: { [payload.key]: payload.newValue } };
}
