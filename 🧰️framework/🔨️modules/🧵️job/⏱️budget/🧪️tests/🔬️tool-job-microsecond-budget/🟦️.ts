import { join } from "node:path";
import { readFileSync } from "node:fs";
import { microsecondsFromMilliseconds } from "../../🟨️.js";
import { findWorkspaceRoot } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
const WORKSPACE_ROOT = findWorkspaceRoot(import.meta.dir);

/** 🧪️ Executes tool job microsecond budget policy assertions. */
export function toolJobMicrosecondBudgetSelfTests(): number {
  const base = join(WORKSPACE_ROOT, "🧰️framework/🔨️modules/🧵️job/⏱️budget");
  const fixture = JSON.parse(readFileSync(join(base, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
  const maximum = (1n << 64n) - 1n;
  for (const law of fixture.cases) {
    const sum = BigInt(law.start) + BigInt(law.grant);
    const deadline = sum > maximum ? null : sum.toString();
    if (deadline !== law.deadline) throw new Error(`microsecond checked deadline: ${law.id}`);
    const expired = law.samples.map((sample: string) => deadline === null || BigInt(sample) >= BigInt(deadline));
    const yielded = expired.map((value: boolean) => value || law.fuel === 0);
    if (JSON.stringify(expired) !== JSON.stringify(law.expired) || JSON.stringify(yielded) !== JSON.stringify(law.yielded)) throw new Error(`microsecond deadline boundary: ${law.id}`);
  }
  const clocks = JSON.parse(readFileSync(join(base, "🕰️clock.json"), "utf8"));
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
  return fixture.cases.length + clocks.browser.length + clocks.wasi.length + clocks.installation.length + clocks.watchdog.length + 2;
}
