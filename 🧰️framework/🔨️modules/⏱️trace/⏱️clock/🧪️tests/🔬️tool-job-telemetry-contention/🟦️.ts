import { join } from "node:path";
import { readFileSync } from "node:fs";
import { WORKSPACE_ROOT } from "../../../../../../📜️script.ts";

/** 🧪️ Executes tool job telemetry contention policy assertions. */
export function toolJobTelemetryContentionSelfTests(): number {
  const base = join(WORKSPACE_ROOT, "🧰️framework/🔨️modules/⏱️trace/⏱️clock");
  const fixture = JSON.parse(readFileSync(join(base, "🧫️fixtures/🧪️contention/🔣️.json"), "utf8"));
  const ids = new Set<string>();
  for (const law of fixture.cases) {
    if (ids.has(law.id)) throw new Error(`duplicate telemetry contention law: ${law.id}`);
    ids.add(law.id);
    if (law.violationRetained !== (law.callback === "watchdog") || law.returnedEvent !== (law.callback === "event")) throw new Error(`telemetry exact authority oracle: ${law.id}`);
    if (law.callback === "event" ? law.lock !== "event" : law.callback === "timer" ? law.lock !== "site" : !["site", "violation"].includes(law.lock)) throw new Error(`telemetry contention owner oracle: ${law.id}`);
  }
  for (const law of fixture.verdicts) {
    const clockFault = law.start === null || law.end === null ? "Missing" : BigInt(law.end) < BigInt(law.start) ? "Backward" : null;
    const fault = clockFault !== null || BigInt(law.end) - BigInt(law.start) >= 8_000n;
    if (clockFault !== law.clockFault || fault !== law.fault) throw new Error(`exact callback verdict oracle: ${law.id}`);
    if (law.sessionTerminal !== (clockFault !== null)) throw new Error(`single-step quarantine oracle: ${law.id}`);
  }
  for (const law of fixture.overruns) {
    let consecutive = 0;
    let longestRun = 0;
    let total = 0;
    let worstElapsedUs = 0;
    let terminalIndex: number | null = null;
    law.elapsedUs.forEach((elapsedUs: number, index: number) => {
      if (elapsedUs < 8_000) {
        consecutive = 0;
        return;
      }
      consecutive += 1;
      longestRun = Math.max(longestRun, consecutive);
      total += 1;
      worstElapsedUs = Math.max(worstElapsedUs, elapsedUs);
      if (consecutive >= fixture.sustainedOverrunSteps && terminalIndex === null) terminalIndex = index;
    });
    if (terminalIndex !== law.terminalIndex || consecutive !== law.consecutive || longestRun !== law.longestRun || total !== law.total || worstElapsedUs !== law.worstElapsedUs) {
      throw new Error(`sustained overrun ledger oracle: ${law.id}`);
    }
  }
  return fixture.cases.length + fixture.verdicts.length + fixture.overruns.length;
}
