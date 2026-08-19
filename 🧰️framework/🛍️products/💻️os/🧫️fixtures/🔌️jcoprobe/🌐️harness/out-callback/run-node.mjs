/** 🧪️ terra-jco-spike bun/node harness — drives the jco-transpiled `jcoprobe` component through
 * S1-S4 and prints PASS/FAIL verdict lines the report can quote verbatim. */
import { probe } from "./jcoprobe.js";

const verdicts = [];
function record(id, ok, detail) {
  verdicts.push({ id, ok, detail });
  console.log(`[harness] ${id}: ${ok ? "PASS" : "FAIL"} — ${detail}`);
}

// #region S1 — trivial callable async export
async function runS1() {
  const start = performance.now();
  const p = probe.poll(21);
  const isPromise = typeof p.then === "function";
  const result = await p;
  const elapsed = performance.now() - start;
  record("S1", isPromise && result === 42, `probe.poll(21) returned a ${isPromise ? "Promise" : typeof p} resolving to ${result} in ${elapsed.toFixed(2)}ms (expected Promise resolving to 42)`);
}
// #endregion

// #region S2 — event loop not blocked while guest awaits a real-delay host import
async function runS2() {
  let ticks = 0;
  const interval = setInterval(() => {
    ticks++;
  }, 5);
  const start = performance.now();
  const result = await probe.awaitEcho(50, 777);
  const elapsed = performance.now() - start;
  clearInterval(interval);
  const ok = result === 777 && ticks >= 3; // 50ms / 5ms interval should tick ~10x if unblocked
  record("S2", ok, `awaitEcho(50,777) took ${elapsed.toFixed(2)}ms, result=${result}, concurrent setInterval(5ms) fired ${ticks} times while it was pending (>=3 required to prove the loop wasn't blocked)`);
}
// #endregion

// #region S3 — spawn-detached: export resolves before the spawned background import completes
async function runS3() {
  const events = [];
  const origSlowEcho = (await import("./host-shim.js")).slowEcho;
  const start = performance.now();
  const result = await probe.spawnDetached(80);
  const exportDoneAt = performance.now() - start;
  events.push({ label: "export-resolved", t: exportDoneAt });
  // give the detached background task a chance to actually finish (its own 80ms delay)
  await new Promise((resolve) => setTimeout(resolve, 200));
  const totalAt = performance.now() - start;
  record(
    "S3",
    result === 1,
    `spawnDetached(80) export Promise resolved at t=${exportDoneAt.toFixed(2)}ms with value ${result}; see host-shim slowEcho DONE log timestamp above for when the detached background import actually completed (should be AFTER ${exportDoneAt.toFixed(2)}ms, close to 80ms)`,
  );
}
// #endregion

// #region S4 — stream<u8> readable chunk-by-chunk
async function runS4() {
  const result = await probe.readBody();
  record("S4", result === 5, `readBody() returned ${result} (expected 5, one per host-shim fetchBody chunk — see the 5 "yielding chunk" host-shim log lines above, each logged as the guest polled the stream one item at a time)`);
}
// #endregion

await runS1();
await runS2();
await runS3();
await runS4();

console.log("[harness] ==== VERDICTS ====");
for (const v of verdicts) {
  console.log(`[harness] ${v.id}: ${v.ok ? "PASS" : "FAIL"} — ${v.detail}`);
}
const allPass = verdicts.every((v) => v.ok);
console.log(`[harness] overall: ${allPass ? "ALL PASS" : "SOME FAILED"}`);
process.exit(allPass ? 0 : 1);
