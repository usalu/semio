/** 🔺️ jack set-query/🔺️diff — mirror of the query-slot delta builder. */
import type { SetQuery } from "../🟦️.ts";

export function diff(payload: SetQuery, baseQuery: string): { query: string } | null {
  return baseQuery === payload.value ? null : { query: payload.value };
}
