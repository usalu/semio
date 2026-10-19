/** 🚨️ Testing breaches
 *
 * The breach kinds of the testing domain and the breach constructor.
 */

//#region 🔌️Adapters
import { type BreachRecord } from "@semio-tech/repo-lib/policy";
//#endregion 🔌️Adapters

//#region 🧾️Contract
/** 🧾️ Breach families this domain owns. Every finding carries one of these `kind` values. */
export const TESTING_BREACH_KINDS = ["testing/taxonomy", "testing/contract", "testing/fixture", "testing/oracle", "testing/dependency", "testing/discovery", "testing/layout"] as const;
export type TestingBreachKind = (typeof TESTING_BREACH_KINDS)[number];

export function breach(kind: TestingBreachKind, id: string, scope: string, summary: string, reason: string, solution: string, priority: BreachRecord["priority"] = "high"): BreachRecord {
  return { id, kind, scope, summary, reason, solution, priority };
}
//#endregion 🧾️Contract
