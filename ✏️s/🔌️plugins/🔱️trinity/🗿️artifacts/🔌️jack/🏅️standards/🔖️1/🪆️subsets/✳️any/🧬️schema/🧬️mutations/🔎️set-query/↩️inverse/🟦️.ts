/** ↩️ jack set-query/↩️inverse — mirror of the BASE-query inverse builder. */
import type { SetQuery } from "../🟦️.ts";

export function inverse(_payload: SetQuery, baseQuery: string): SetQuery[] {
  return [{ value: baseQuery }];
}
