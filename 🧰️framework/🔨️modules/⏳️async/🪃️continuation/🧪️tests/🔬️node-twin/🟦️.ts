/** 🪞️ Node twin of the continuation scheduler's fixture suite — the same
 * `🧫️fixtures/🔣️.json` cases, run with NO test framework at all, on the platform's own event loop.
 *
 * 🧪️ Why a twin: the vitest suite proves our virtual clock agrees with our scheduler. That is a
 * closed loop. This runner replaces our virtual clock with the independent oracle — Node's real
 * `MessageChannel` and `setTimeout`, the same two primitives a browser supplies — and asserts the
 * fixture's declared order and deadlines survive it. If the platform's event loop and our model
 * disagree about a law, the law is wrong, not the runtime.
 *
 * ▶️ `bun nx run @semio-tech/framework-async:twin` (or `bun ./📜️script.ts twin` in the TS package).
 * Exit code 0 = every case held both ways; 1 = the faults are printed, one per line.
 *
 * 🇩🇪 Node-Zwilling der Fixture-Suite: dieselben Fälle ohne Test-Framework, auf der echten
 * Ereignisschleife der Plattform.
 */
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { continuationCaseFaults, createContinuationScheduler, runContinuationCase, runContinuationCaseLive, type ContinuationSuite } from "../../🟦️.ts";

/** 📏️ Slack allowed between a fixture's declared millisecond and the real loop's wall clock — a
 * timer never fires early, and a loaded machine can be late. */
const LIVE_TOLERANCE_MS = 250;
/** 📏️ The throttled-`setTimeout` floor this whole lane exists to escape: a hidden Chrome renderer
 * clamps a nested zero-delay timer chain to roughly one tick per second. */
const THROTTLED_TIMER_FLOOR_MS = 1_000;
/** 📏️ Hops in the chained re-arm the regression law drives. */
const CHAIN_HOPS = 10;

const suite = JSON.parse(readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../../🧫️fixtures/🔣️.json"), "utf8")) as ContinuationSuite;
const faults: string[] = [];

for (const testCase of suite.cases) {
  faults.push(...continuationCaseFaults(testCase, runContinuationCase(testCase)).map((fault) => `virtual · ${fault}`));
  faults.push(...continuationCaseFaults(testCase, await runContinuationCaseLive(testCase), LIVE_TOLERANCE_MS).map((fault) => `live · ${fault}`));
  console.log(`  ✓ ${testCase.name}`);
}

/** 🩺 The measurement the audit asked for, on the real loop: a ten-hop chain against the cost the
 * same ten hops would carry on a throttled nested timer. */
const scheduler = createContinuationScheduler();
const startedAtMs = Date.now();
await new Promise<void>((resolve) => {
  let hop = 0;
  const step = (): void => {
    hop += 1;
    if (hop >= CHAIN_HOPS) return resolve();
    scheduler.schedule(step, 0);
  };
  scheduler.schedule(step, 0);
});
const chainMs = Date.now() - startedAtMs;
scheduler.dispose();
const throttledMs = CHAIN_HOPS * THROTTLED_TIMER_FLOOR_MS;
console.log(`  ⏱️ ${CHAIN_HOPS} chained zero-delay hops on the real loop: ${chainMs} ms (a throttled nested-setTimeout chain: ~${throttledMs} ms)`);
if (chainMs > THROTTLED_TIMER_FLOOR_MS) faults.push(`live · a ${CHAIN_HOPS}-hop zero-delay chain took ${chainMs} ms, at or over the ${THROTTLED_TIMER_FLOOR_MS} ms throttled-timer floor it must never approach`);

if (faults.length > 0) {
  for (const fault of faults) console.error(`✗ ${fault}`);
  process.exitCode = 1;
} else {
  console.log(`🪞️ node twin: ${suite.cases.length} cases held on the virtual clock AND on the real event loop.`);
}
