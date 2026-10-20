//#region 🧲️Header
// 2025-2026 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0 — Transaction Plan/Journal v2 crash, recovery, and concurrency proof.
//#endregion 🧲️Header
import { describe } from "bun:test";
import { registerKeepsLanguageNeutralGoldenTests } from "./🔶️keeps-language-neutral-golden/🟦️.ts";
import { registerRejectsIncompletePlansPlanTests } from "./🔸️rejects-incomplete-plans-plan/🟦️.ts";
import { registerRejectsStaleBaselineSourceTests } from "./🟠️rejects-stale-baseline-source/🟦️.ts";
import { registerKeepsDoublePlanBytesTests } from "./🟢️keeps-double-plan-bytes/🟦️.ts";

describe("transaction plan journal v2 aggregate", () => {
  registerKeepsLanguageNeutralGoldenTests();
  registerRejectsIncompletePlansPlanTests();
  registerRejectsStaleBaselineSourceTests();
  registerKeepsDoublePlanBytesTests();
});
