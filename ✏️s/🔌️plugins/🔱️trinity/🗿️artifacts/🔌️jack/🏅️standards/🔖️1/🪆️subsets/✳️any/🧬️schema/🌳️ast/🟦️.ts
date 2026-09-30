/** 🔎️ Query output defined by the Jack artifact schema QueryResult. */
import type { DslValue } from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🟦️.ts";
import type { JackSnapshot } from "../📸️snapshot/🟦️.ts";

export interface QueryResult {
  readonly kind: "table" | "graph";
  readonly columns: readonly string[];
  readonly rows: readonly (readonly DslValue[])[];
  readonly graphFixture?: JackSnapshot;
}
