/** 🧪️ terra-jco-spike Web Worker harness — same S1-S4 sequence as 📜️script.ts, run inside a real
 * browser Web Worker (importing the jco-transpiled ES module directly), reporting results back to
 * the page via postMessage so the Browser pane's console/page-text tools can read them. */
import { probe } from "./jcoprobe.js";

function log(text) {
  postMessage({ kind: "log", text });
  console.log(text);
}

const verdicts = [];
function record(id, ok, detail) {
  verdicts.push({ id, ok, detail });
  log(`${id}: ${ok ? "PASS" : "FAIL"} — ${detail}`);
}

async function runS1() {
  const start = performance.now();
  const p = probe.poll(21);
  const isPromise = typeof p.then === "function";
  const result = await p;
  const elapsed = performance.now() - start;
  record("S1", isPromise && result === 42, `probe.poll(21) -> ${result} (Promise=${isPromise}) in ${elapsed.toFixed(2)}ms`);
}

async function runS2() {
  let ticks = 0;
  const interval = setInterval(() => {
    ticks++;
  }, 5);
  const start = performance.now();
  const result = await probe.awaitEcho(50, 777);
  const elapsed = performance.now() - start;
  clearInterval(interval);
  const ok = result === 777 && ticks >= 3;
  record("S2", ok, `awaitEcho(50,777) took ${elapsed.toFixed(2)}ms, result=${result}, setInterval(5ms) ticks=${ticks} while pending`);
}

async function runS3() {
  const start = performance.now();
  const result = await probe.spawnDetached(80);
  const exportDoneAt = performance.now() - start;
  await new Promise((resolve) => setTimeout(resolve, 200));
  record("S3", result === 1, `spawnDetached(80) export resolved at t=${exportDoneAt.toFixed(2)}ms value=${result} (see host-shim slowEcho DONE log for actual detached completion time)`);
}

async function runS4() {
  const result = await probe.readBody();
  record("S4", result === 5, `readBody() -> ${result} (expected 5)`);
}

try {
  await runS1();
  await runS2();
  await runS3();
  await runS4();
} catch (err) {
  log(`WORKER EXCEPTION: ${err && err.stack ? err.stack : err}`);
}

const overall = verdicts.length > 0 && verdicts.every((v) => v.ok);
postMessage({ kind: "done", overall, verdicts });
