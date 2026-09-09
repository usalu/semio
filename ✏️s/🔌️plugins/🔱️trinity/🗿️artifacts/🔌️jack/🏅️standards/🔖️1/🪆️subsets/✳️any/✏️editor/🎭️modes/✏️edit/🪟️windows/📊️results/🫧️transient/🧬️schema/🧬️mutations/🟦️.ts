import type { QueryResult } from "../../🟦️";

/** 🧬️ A replacement of one addressed results-window execution state. */
export type JackResultsWindowTransientMutation = {
  readonly kind: "replace-query-result";
  readonly executionId: string | null;
  readonly result: QueryResult | null;
  readonly error: string | null;
};
