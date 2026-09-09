import { join } from "node:path";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { microsecondsFromMilliseconds } from "../../🟨️.js";
import { WORKSPACE_ROOT, toolJobMicrosecondWorkerExact } from "../../../../../../📜️script.ts";

/** 🧪️ Executes tool job microsecond budget policy assertions. */
export function toolJobMicrosecondBudgetSelfTests(): number {
  const base = join(WORKSPACE_ROOT, "🧰️framework/🔨️modules/🧵️job/⏱️budget");
  const fixture = JSON.parse(readFileSync(join(base, "🧫️fixtures/🔣️.json"), "utf8"));
  const module = JSON.parse(readFileSync(join(base, "🧬️schema/🔣️.json"), "utf8"));
  const Ajv = createRequire(import.meta.url)("ajv");
  const ajv = new Ajv({ strict: true, allErrors: true }).addSchema(module);
  const validate = ajv.getSchema(`${module.$id}#/$defs/Budget`)!;
  if (!validate(fixture)) throw new Error(`microsecond budget schema: ${JSON.stringify(validate.errors)}`);
  const maximum = (1n << 64n) - 1n;
  for (const law of fixture.cases) {
    const sum = BigInt(law.start) + BigInt(law.grant);
    const deadline = sum > maximum ? null : sum.toString();
    if (deadline !== law.deadline) throw new Error(`microsecond checked deadline: ${law.id}`);
    const expired = law.samples.map((sample: string) => deadline === null || BigInt(sample) >= BigInt(deadline));
    const yielded = expired.map((value: boolean) => value || law.fuel === 0);
    if (JSON.stringify(expired) !== JSON.stringify(law.expired) || JSON.stringify(yielded) !== JSON.stringify(law.yielded)) throw new Error(`microsecond deadline boundary: ${law.id}`);
  }
  for (const change of [{ unit: "milliseconds" }, { extra: true }]) {
    if (validate({ ...fixture, ...change })) throw new Error("microsecond schema accepted a forged unit or field");
  }
  const clocks = JSON.parse(readFileSync(join(base, "🕰️clock.json"), "utf8"));
  const validateClocks = ajv.getSchema(`${module.$id}#/$defs/Clock`)!;
  if (!validateClocks(clocks)) throw new Error(`host clock schema: ${JSON.stringify(validateClocks.errors)}`);
  for (const law of clocks.browser) {
    const [integer, fraction = ""] = law.milliseconds.split(".");
    const numerator = BigInt(integer) * 10n ** BigInt(fraction.length) + BigInt(fraction || "0");
    const exact = numerator * 1_000n / 10n ** BigInt(fraction.length);
    const expected = numerator < 0n || exact > maximum ? null : exact.toString();
    if (expected !== law.microseconds) throw new Error(`browser clock rational oracle: ${law.milliseconds}`);
    const actual = microsecondsFromMilliseconds(Number(law.milliseconds));
    if ((actual === null ? null : actual.toString()) !== expected) throw new Error(`browser clock conversion: ${law.milliseconds}`);
  }
  for (const law of clocks.wasi) {
    if ((BigInt(law.nanoseconds) <= maximum) !== law.accepted) throw new Error(`WASI checked nanoseconds: ${law.nanoseconds}`);
  }
  for (const law of clocks.installation) {
    const authority = new Map<string, string>();
    if (law.current !== null) authority.set("clock", law.current);
    const accepted = !authority.has("clock") || authority.get("clock") === law.requested;
    if (accepted) authority.set("clock", law.requested);
    if (accepted !== law.accepted || authority.get("clock") !== law.retained) throw new Error(`exact clock installation oracle: ${law.current}/${law.requested}`);
  }
  for (const law of clocks.watchdog) {
    if ((BigInt(law.elapsedMicroseconds) >= 8_000n) !== law.violated) throw new Error(`strict watchdog boundary oracle: ${law.elapsedMicroseconds}`);
  }
  for (const invalid of [Number.NaN, Number.POSITIVE_INFINITY]) if (microsecondsFromMilliseconds(invalid) !== null) throw new Error("invalid monotonic source was admitted");
  const binding = JSON.parse(readFileSync(join(base, "🪢️binding.json"), "utf8"));
  const validateBinding = ajv.getSchema(`${module.$id}#/$defs/Binding`)!;
  if (!validateBinding(binding)) throw new Error(`microsecond binding schema: ${JSON.stringify(validateBinding.errors)}`);
  const plugin = readFileSync(join(WORKSPACE_ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"), "utf8");
  const job = readFileSync(join(base, "../🦀️.rs"), "utf8");
  const trace = readFileSync(join(base, "../../⏱️trace/🦀️.rs"), "utf8");
  const mutations: Record<string, [number, string, string]> = {
    none: [0, "", ""],
    "wrong-helper": [0, "self.start_typed_command_operation(command, admission, meta, operation_id, None).await", "self.wrong_operation(command, admission, meta, operation_id, None).await"],
    "divided-grant": [0, "step_budget_us: u64::from(admission.proof.contract().max_step_micros),", "step_budget_us: u64::from(admission.proof.contract().max_step_micros) / 1_000,"],
    "rounded-grant": [0, "step_budget_us: u64::from(admission.proof.contract().max_step_micros),", "step_budget_us: u64::from(admission.proof.contract().max_step_micros).max(1_000),"],
    "coarse-clock": [0, "now_us: semio_framework_job::default_now_us,", "now_us: semio_framework_job::default_now_ms,"],
    "overflow-fallback": [1, "start_us.checked_add(duration_us)", "Some(start_us.saturating_add(duration_us))"],
    "expired-entry": [1, "if budget.fuel == 0 || now_us().is_none_or(|now_us| now_us >= budget.deadline_us)", "if budget.fuel == 0"],
    "synthetic-clock": [2, "fn default_clock_us() -> Option<u64> { None }", "fn default_clock_us() -> Option<u64> { Some(0) }"],
    "missing-output-limit": [0, "ArtifactOutputChunks::new(admission.proof.contract().max_output_bytes)", "ArtifactOutputChunks::new(u64::MAX)"],
  };
  const validBinding = new Ajv({ strict: true }).compile({ const: "none" });
  for (const law of binding.cases) {
    const [index, before, after] = mutations[law.mutation]!;
    const sources = [plugin, job, trace];
    if (law.mutation !== "none") {
      if (!sources[index]!.includes(before)) throw new Error(`microsecond hostile target is stale: ${law.mutation}`);
      sources[index] = sources[index]!.replaceAll(before, after);
    }
    if (validBinding(law.mutation) !== law.admitted || toolJobMicrosecondWorkerExact(sources[0]!, sources[1]!, sources[2]!) !== law.admitted) throw new Error(`microsecond exact worker binding: ${law.mutation}`);
  }
  return fixture.cases.length + clocks.browser.length + clocks.wasi.length + clocks.installation.length + clocks.watchdog.length + 4 + binding.cases.length * 2;
}
