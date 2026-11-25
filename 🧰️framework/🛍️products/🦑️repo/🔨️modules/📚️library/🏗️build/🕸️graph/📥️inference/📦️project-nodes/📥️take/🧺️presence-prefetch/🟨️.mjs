import { readFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { brotliDecompressSync } from "node:zlib";
import { TAKE_POLICY_V1 } from "../🟨️.mjs";

/** 🪶️ The result of one stored project record without its rows, or undefined when nothing readable is stored at `cachePath`: the record is the brotli text of a header line and the result, the header names the digest of the result (`resultSha256`), no row block is expanded. A hint for a prefetch only; every look still reads and verifies the record in full.
 * @see ../../../../../👁️observation/🟨️.mjs `readInferencePhysicalRecordV1` (the same layout, with custody and rows) */
export function takeStoredRecordResultV1(cachePath, read = readFileSync) {
  try {
    const text = brotliDecompressSync(read(cachePath)).toString("utf8"), cut = text.indexOf("\n");
    if (cut < 0) return undefined;
    const header = JSON.parse(text.slice(0, cut)), result = text.slice(cut + 1);
    return header?.resultSha256 === createHash("sha256").update(result).digest("hex") ? JSON.parse(result) : undefined;
  } catch { return undefined; }
}

/** 🧺️ Verifies the inventory names of every stored record of a take pass before its looks ask one by one: the records are read in order (`records`, one cache path each), the names their results carry (`capture(value)`) go to `verify(names)` in asks of the names of `prefetchRecordsPerAsk` records, at most `prefetchAsksInFlight` asks at once, so the asks join one session of the store as consecutive batches and the mounted image is verified once for the pass instead of once per wave of looks; `verify` remembers what it found present, the looks answer from that memory. A record that cannot be read or captured is skipped (its look decides it), a failed ask only loses its hint; `signal` stops the reading, `progress(row)` is told after every ask. Answers the counts: records read, skipped, names asked, names found present, asks, failed asks, milliseconds.
 * @see ../🧬️schema/🔣️.json `prefetchRecordsPerAsk`, `prefetchAsksInFlight` */
export async function prefetchTakePresenceV1({ records, capture, verify, signal, progress = () => {}, read = takeStoredRecordResultV1, policy = TAKE_POLICY_V1, now = () => performance.now() }) {
  const started = now(), counts = { records: 0, skipped: 0, names: 0, present: 0, asks: 0, failed: 0, milliseconds: 0 }, flying = new Set(), seen = new Set();
  let batch = [], inBatch = 0;
  const send = () => {
    if (batch.length === 0) { inBatch = 0; return; }
    const names = batch; batch = []; inBatch = 0; counts.asks++; counts.names += names.length;
    const ask = Promise.resolve().then(() => verify(names)).then(row => { counts.present += Number(row?.present) || 0; }, () => { counts.failed++; }).finally(() => { flying.delete(ask); counts.milliseconds = Math.round(now() - started); try { progress({ ...counts }); } catch {} });
    flying.add(ask);
  };
  for (const cachePath of records) {
    if (signal?.aborted) break;
    const value = read(cachePath);
    let names;
    try { names = value === undefined ? undefined : capture(value); } catch { names = undefined; }
    if (names === undefined) { counts.skipped++; continue; }
    counts.records++;
    for (const name of names) if (!seen.has(name)) { seen.add(name); batch.push(name); }
    if (++inBatch < policy.prefetchRecordsPerAsk) continue;
    send();
    while (flying.size >= policy.prefetchAsksInFlight) await Promise.race(flying);
    await new Promise(resolve => setImmediate(resolve));
  }
  if (!signal?.aborted) send();
  await Promise.allSettled([...flying]);
  counts.milliseconds = Math.round(now() - started);
  return Object.freeze({ ...counts });
}
