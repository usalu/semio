import type { QueryResult } from "../../../../../../../../../../../../../🧬️schema/🌳️ast/🟦️";

/** 📊️ Ephemeral execution output for one concrete Jack results window. */
export interface JackResultsWindowTransient {
  readonly queryExecutionId: string | null;
  readonly result: QueryResult | null;
  readonly queryError: string | null;
}
