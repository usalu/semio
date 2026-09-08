import { join } from "node:path";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { WORKSPACE_ROOT } from "../../../../../../📜️script.ts";

/** 🧪️ Executes tool job telemetry contention policy assertions. */
export function toolJobTelemetryContentionSelfTests(): number {
  const base = join(WORKSPACE_ROOT, "🧰️framework/🔨️modules/⏱️trace/⏱️clock");
  const fixture = JSON.parse(readFileSync(join(base, "🧪️contention/🔣️.json"), "utf8"));
  const module = JSON.parse(readFileSync(join(base, "🧬️contention/🧬️schema/🔣️.json"), "utf8"));
  const Ajv = createRequire(import.meta.url)("ajv");
  const validate = new Ajv({ strict: true, allErrors: true }).addSchema(module).getSchema(`${module.$id}#/$defs/Contention`)!;
  if (!validate(fixture)) throw new Error(`telemetry contention schema: ${JSON.stringify(validate.errors)}`);
  const ids = new Set<string>();
  for (const law of fixture.cases) {
    if (ids.has(law.id)) throw new Error(`duplicate telemetry contention law: ${law.id}`);
    ids.add(law.id);
    if (law.violationRetained !== (law.callback === "watchdog") || law.returnedEvent !== (law.callback === "event")) throw new Error(`telemetry exact authority oracle: ${law.id}`);
    if (law.callback === "event" ? law.lock !== "event" : law.callback === "timer" ? law.lock !== "site" : !["site", "violation"].includes(law.lock)) throw new Error(`telemetry contention owner oracle: ${law.id}`);
  }
  for (const change of [{ returnsWhileHeld: false }, { unboundedFallback: true }]) {
    if (validate({ ...fixture, cases: [{ ...fixture.cases[0], ...change }, ...fixture.cases.slice(1)] })) throw new Error("telemetry contention schema accepted callback waiting or unknown authority");
  }
  for (const law of fixture.verdicts) {
    const clockFault = law.start === null || law.end === null ? "Missing" : BigInt(law.end) < BigInt(law.start) ? "Backward" : null;
    const fault = clockFault !== null || BigInt(law.end) - BigInt(law.start) >= 8_000n;
    if (clockFault !== law.clockFault || fault !== law.fault) throw new Error(`exact callback verdict oracle: ${law.id}`);
  }
  return fixture.cases.length + fixture.verdicts.length + 2;
}
