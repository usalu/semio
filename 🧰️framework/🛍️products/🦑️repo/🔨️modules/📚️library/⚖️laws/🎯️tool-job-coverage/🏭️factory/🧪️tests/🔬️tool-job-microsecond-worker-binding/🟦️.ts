import { join } from "node:path";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { findWorkspaceRoot } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { toolJobMicrosecondWorkerExact, toolJobMicrosecondWorkerSources, type ToolJobMicrosecondSources } from "../../🟦️.ts";
const WORKSPACE_ROOT = findWorkspaceRoot(import.meta.dir);

/** 🪢️ Binds the exact microsecond worker law to the actual plugin, job, trace and loan sources with independent hostile mutations. */
export function toolJobMicrosecondWorkerBindingSelfTests(): number {
  const base = join(WORKSPACE_ROOT, "🧰️framework/🔨️modules/🧵️job/⏱️budget");
  const Ajv = createRequire(import.meta.url)("ajv");
  const binding = JSON.parse(readFileSync(join(base, "🪢️binding.json"), "utf8"));
  const production = toolJobMicrosecondWorkerSources(WORKSPACE_ROOT);
  const mutations: Record<string, [keyof ToolJobMicrosecondSources, string, string]> = {
    none: ["dispatch", "", ""],
    "wrong-helper": ["dispatch", "contract: Some(admission.proof.contract()),", "contract: Some(admission.proof.contract().wrong_contract()),"],
    "divided-grant": ["mounted", "step_budget_us:u64::from(contract.max_step_micros)}", "step_budget_us:u64::from(contract.max_step_micros)/1_000}"],
    "rounded-grant": ["mounted", "now.saturating_add(u64::from(contract.max_step_micros))", "now.saturating_add(u64::from(contract.max_step_micros).max(1_000))"],
    "coarse-clock": ["job", "semio_framework_trace::try_now_us()", "semio_framework_trace::try_now_ms()"],
    "overflow-fallback": ["job", "start_us.checked_add(duration_us)", "Some(start_us.saturating_add(duration_us))"],
    "expired-entry": ["loan", "if cx.fuel_remaining()==0||cx.latest_us().map(|now|now>=cx.deadline_us()).unwrap_or_else(||cx.deadline_exceeded())", "if cx.fuel_remaining()==0"],
    "synthetic-clock": ["trace", "pub(super) fn default_clock_us() -> Option<u64> {\n    None\n}", "pub(super) fn default_clock_us() -> Option<u64> { Some(0) }"],
    "missing-output-limit": ["dispatch", "ArtifactOutputChunks::new(admission.proof.contract().max_output_bytes)", "ArtifactOutputChunks::new(u64::MAX)"],
    "unbound-step": ["mounted", "cx.with_original_work_loan(step_fuel,step_deadline_us,|same|session.pump_one_for_interactive_turn(same))", "session.pump_one_for_interactive_turn(cx)"],
    "zero-budget": ["mounted", "step_budget_us:u64::from(contract.max_step_micros)}", "step_budget_us:0}"],
  };
  const validBinding = new Ajv({ strict: true }).compile({ const: "none" });
  for (const law of binding.cases) {
    const [key, before, after] = mutations[law.mutation]!;
    const sources = { ...production };
    if (law.mutation !== "none") {
      if (!sources[key].includes(before)) throw new Error(`microsecond hostile target is stale: ${law.mutation}`);
      sources[key] = sources[key].replaceAll(before, after);
    }
    if (validBinding(law.mutation) !== law.admitted || toolJobMicrosecondWorkerExact(sources) !== law.admitted) throw new Error(`microsecond exact worker binding: ${law.mutation}`);
  }
  return binding.cases.length * 2;
}
