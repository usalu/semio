/** 🧩️ Semantic benchmark stub owner. */

import { BenchBudgetDefinition } from "../📋️plan/🟦️.ts";



const BENCH_WEB_STUB_STATUS: Readonly<Record<number, { readonly passLabel: string; readonly failLabel: string }>> = {
  2: { passLabel: "pass-stub-worker", failLabel: "fail-stub-worker" },
  5: { passLabel: "pass-stub-worker", failLabel: "fail-stub-worker" },
};

function benchWebMeasuredRow(budget: BenchBudgetDefinition, renderer: string, raw: { readonly id: number; readonly ok: boolean; readonly measured: unknown; readonly note: string }): Record<string, unknown> {
  const stubLabels = BENCH_WEB_STUB_STATUS[budget.id];
  const status = stubLabels ? (raw.ok ? stubLabels.passLabel : stubLabels.failLabel) : raw.ok ? "pass" : "fail";
  return { id: budget.id, description: budget.description, status, measured: raw.measured, threshold: budget.webThreshold ?? null, note: `[${renderer}, harness-driven, see 📊️bench-web-harness/🟦️.ts header] ${raw.note}` };
}

export { BENCH_WEB_STUB_STATUS, benchWebMeasuredRow };
