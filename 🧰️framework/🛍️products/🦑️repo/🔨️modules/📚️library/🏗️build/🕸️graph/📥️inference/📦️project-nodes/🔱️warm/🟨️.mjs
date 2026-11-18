import{machinePathV1}from"../../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import { createHash } from "node:crypto";
import { appendFileSync, mkdirSync, readFileSync, readdirSync, rmSync, statSync } from "node:fs";
import { join } from "node:path";
import { brotliDecompressSync } from "node:zlib";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

/** 🔱️ The numbers of the warm pass (the schema's `policy` pins them): the part directory below the owner records, how long a durable row counts, the pause between progress lines, the owner count below which the pass is not worth its children, and the megabytes the parent and a child are planned at. @see ./🧬️schema/🔣️.json */
export const WARM_POLICY_V1 = Object.freeze({ directory: "warm", maximumAgeMilliseconds: 1_200_000, progressMilliseconds: 5_000, minimumOwners: 64, parentMegabytes: 1_500, childMegabytes: 1_700 });

/** 🧬️ The outcomes a part row may carry. */
export const WARM_OUTCOMES_V1 = Object.freeze([...schema.$defs.outcome.enum]);

/** 🔑️ The first 24 hex digits of the sha256 of a record key text: what a part row names the key by. */
export const warmDigestV1 = text => createHash("sha256").update(text).digest("hex").slice(0, 24);

/** 📂️ The directory of the part files of a checkout: below its machine cache, outside the graph store (the graph store is written under custody only, and a part is a hint, never a fact). */
export const warmDirectoryV1 = workspaceRoot => machinePathV1(workspaceRoot, ".cache", "f", WARM_POLICY_V1.directory);

/** 📖️ Reads a stored owner record file with plain file access (a farm child holds no custody of the graph store and writes nothing there): the brotli text holds the header line and then the record value; the closure rows to settle are the recipes the value names. Answers undefined when the file is missing or torn.
 * @see ../../../../👁️observation/🟨️.mjs */
export function readStoredRecordV1(path, workspaceRoot) {
  let text;
  try { text = brotliDecompressSync(readFileSync(path)).toString("utf8"); } catch { return undefined; }
  const cut = text.indexOf("\n");
  if (cut < 0) return undefined;
  let header, value;
  try { header = JSON.parse(text.slice(0, cut)); value = JSON.parse(text.slice(cut + 1)); } catch { return undefined; }
  return { header: { digest: header?.digest, observations: Object.keys(value?.closureRecipes ?? {}).map(key => ({ kind: "closure", path: JSON.stringify([workspaceRoot, key, ["digest"]]) })) }, value };
}

/** 🧾️ The stamp of a stored record file: its size and modification time, or undefined when it is not there. */
export function warmStampV1(path) {
  try { const stat = statSync(path); return stat.size + ":" + Math.round(stat.mtimeMs); } catch { return undefined; }
}

/** 🧪️ Whether a value is a well formed part row (the schema's `partRow`). */
export function isWarmRowV1(row) {
  const shape = schema.$defs.partRow;
  if (row === null || typeof row !== "object" || Array.isArray(row)) return false;
  for (const key of shape.required) if (!(key in row)) return false;
  for (const key of Object.keys(row)) if (!(key in shape.properties)) return false;
  return row.v === 1 && typeof row.n === "string" && row.n !== "" && /^[0-9a-f]{24}$/u.test(row.k) && typeof row.d === "string" && WARM_OUTCOMES_V1.includes(row.o) && Number.isInteger(row.t) && row.t >= 0 && (row.c === undefined || Number.isInteger(row.c) && row.c >= 0) && (row.m === undefined || Number.isInteger(row.m) && row.m >= 0);
}

/** ✍️ Appends one durable row to the part file of this process: one `appendFileSync` of a whole line, so a killed process leaves at most one torn last line, which readers skip. */
export function appendWarmRowV1(directory, row, pid = process.pid) {
  mkdirSync(directory, { recursive: true });
  appendFileSync(join(directory, pid + ".part"), JSON.stringify(row) + "\n");
}

/** 📚️ The durable rows of a part directory as a membership test: a row counts while it is younger than `maximumAgeMilliseconds`, a torn or foreign line is skipped, and only a string per row is held (no row object), so the parent's footprint is the key text per owner. */
export function readWarmIndexV1(directory, { now = Date.now, maximumAgeMilliseconds = WARM_POLICY_V1.maximumAgeMilliseconds } = {}) {
  const held = new Set(), at = now();
  let files = [];
  try { files = readdirSync(directory).filter(name => name.endsWith(".part")); } catch { files = []; }
  for (const file of files) {
    let text = "";
    try { text = readFileSync(join(directory, file), "utf8"); } catch { continue; }
    for (let start = 0; start < text.length;) {
      let end = text.indexOf("\n", start);
      if (end < 0) end = text.length;
      const line = text.slice(start, end);
      start = end + 1;
      if (line === "") continue;
      let row;
      try { row = JSON.parse(line); } catch { continue; }
      if (!isWarmRowV1(row) || at - row.t > maximumAgeMilliseconds || row.o === "absent") continue;
      held.add(row.n + "\0" + row.k + "\0" + row.d);
    }
  }
  return Object.freeze({ size: held.size, has: (name, key, stamp) => held.has(name + "\0" + key + "\0" + stamp) });
}

/** 🧹️ Removes the part files nobody wrote to for twice the age a row counts. */
export function sweepWarmPartsV1(directory, { now = Date.now, maximumAgeMilliseconds = WARM_POLICY_V1.maximumAgeMilliseconds } = {}) {
  let removed = 0;
  try { for (const file of readdirSync(directory)) { if (!file.endsWith(".part")) continue; try { if (now() - statSync(join(directory, file)).mtimeMs > 2 * maximumAgeMilliseconds) { rmSync(join(directory, file), { force: true }); removed++; } } catch {} } } catch {}
  return removed;
}

const spread = text => parseInt(createHash("sha1").update(text).digest("hex").slice(0, 8), 16);

/** 🔱️ Splits owners into `lanes` lanes: stable (the same owners and costs always give the same lanes, whatever their order), every lane gets about the same recorded cost because the costliest owners are placed first on the lightest lane, ties broken by a hash of the record key. `owners` are `{ name, key, cost }`; the result holds the names only. */
export function planWarmLanesV1(owners, lanes) {
  const count = Math.max(1, Math.floor(lanes)), out = Array.from({ length: count }, () => ({ names: [], load: 0 }));
  const ordered = [...owners].sort((a, b) => (b.cost ?? 0) - (a.cost ?? 0) || spread(a.key) - spread(b.key) || (a.name < b.name ? -1 : a.name > b.name ? 1 : 0));
  for (const owner of ordered) { let at = 0; for (let index = 1; index < count; index++) if (out[index].load < out[at].load) at = index; out[at].names.push(owner.name); out[at].load += (owner.cost ?? 0) + 1; }
  return out.map(lane => lane.names.reverse());
}

/**
 * 🤝️ Runs the warm pass beside the parent's own take: the take starts at once and never waits for the children, so `taken` and `warmed` both advance from the first seconds; the pass is awaited at the end, also when the take fails.
 * @see ./🧬️schema/🔣️.json
 */
export async function runWarmBesideTakeV1(warm, take) {
  const warming = (async () => warm())();
  try { await take(); } finally { await warming; }
}

/** 🧮️ How many lanes run at once: one less than the cores (the parent keeps one), what the memory ledger grants for the planned child size on top of the parent's, what the farm offers, and never more than the owners. */
export function planWarmWidthV1({ cores, grantedChildren = Infinity, farmLoops = Infinity, limitMegabytes = Infinity, parentMegabytes = WARM_POLICY_V1.parentMegabytes, childMegabytes = WARM_POLICY_V1.childMegabytes, owners = Infinity }) {
  const memory = Number.isFinite(limitMegabytes) ? Math.floor((limitMegabytes - parentMegabytes) / childMegabytes) : Infinity;
  return Math.max(0, Math.min(Math.max(1, cores - 1), grantedChildren, farmLoops, memory, owners));
}

/** 🏁️ The text of the summed progress line in both languages. */
export const WARM_TEXTS_V1 = Object.freeze({ en: "warmed {done} of {total} owners in {lanes} children ({settled} now, {durable} durable from an earlier try, {failed} left to the parent; child peak {peak} MB)", de: "abgeglichen {done} von {total} Owners in {lanes} Kindprozessen ({settled} jetzt, {durable} dauerhaft aus einem früheren Versuch, {failed} dem Elternprozess überlassen; Spitze eines Kindes {peak} MB)" });

/** 🗣️ Renders the progress line of one language. */
export const warmLineV1 = (language, fields) => WARM_TEXTS_V1[language === "de" ? "de" : "en"].replace(/\{(\w+)\}/gu, (_, name) => String(fields[name] ?? 0)) + (fields.why ? " [" + fields.why + "]" : "");

/**
 * 🚀️ Runs the lanes: `lanes` are arrays of owner names, every lane is worked through one owner after the other and the lanes run side by side; `inspect(name)` gives `{ key, stamp }` of the owner (looked up when its turn comes, so nothing is held per owner), an owner with a durable row in `index` is skipped, `warm(name)` settles one owner in a child and answers `{ peakMegabytes }` or undefined when no child could. `tell(fields)` is called at most every `progressMilliseconds` and once at the end with the summed counters. Cancelling `signal` ends the lanes after their running owner.
 * @see ./🧬️schema/🔣️.json
 */
export async function runWarmLanesV1({ lanes, inspect, warm, index, signal, why, tell = () => {}, progressMilliseconds = WARM_POLICY_V1.progressMilliseconds, now = () => performance.now() }) {
  const total = lanes.reduce((sum, lane) => sum + lane.length, 0), counters = { total, done: 0, settled: 0, durable: 0, failed: 0, lanes: lanes.length, peak: 0 };
  let told = -Infinity;
  const say = force => { if (why !== undefined) counters.why = why(); const at = now(); if (force || at - told >= progressMilliseconds) { told = at; tell({ ...counters }); } };
  const beat = progressMilliseconds > 0 ? setInterval(() => say(true), progressMilliseconds) : undefined;
  beat?.unref?.();
  try { await Promise.all(lanes.map(async lane => {
    for (const name of lane) {
      if (signal?.aborted) return;
      const { key, stamp } = inspect(name);
      if (index !== undefined && stamp !== undefined && index.has(name, key, stamp)) { counters.durable++; counters.done++; say(false); continue; }
      let reply;
      try { reply = await warm(name); } catch { reply = undefined; }
      if (reply === undefined) counters.failed++; else { counters.settled++; counters.peak = Math.max(counters.peak, reply.peakMegabytes ?? 0); }
      counters.done++;
      say(false);
    }
  })); } finally { clearInterval(beat); }
  say(true);
  return counters;
}

/**
 * ▶️ The work of one child for one owner: reads the stored record of the owner, brings the closure entries it names up to date (the stale ones are derived by this child into the closure store) and appends a durable row. `ports.read(path)` answers `{ header: { digest, observations }, value }` or undefined, `ports.settle(observations, value)` settles, `ports.keyText(job)` is the record key text of the owner. The row is written only after the settle returned, so a killed child leaves no row for an owner it did not finish.
 * @see ./🧬️schema/🔣️.json
 */
export async function warmOwnerV1(job, { directory, cachePath, ports, now = Date.now }) {
  const cpu = process.cpuUsage(), stamp = warmStampV1(cachePath), key = warmDigestV1(ports.keyText(job));
  let outcome = "absent";
  const record = stamp === undefined ? undefined : ports.read(cachePath);
  if (record !== undefined) {
    await ports.settle(record.header.observations, record.value);
    outcome = record.header.digest === createHash("sha256").update(ports.keyText(job)).digest("hex") ? "settled" : "other";
  }
  const used = process.cpuUsage(cpu);
  appendWarmRowV1(directory, { v: 1, n: job.name, k: job.warmKey ?? key, d: stamp ?? "", o: outcome, t: now(), c: Math.round((used.user + used.system) / 1000), m: 0 });
  return { outcome, cpuMilliseconds: Math.round((used.user + used.system) / 1000) };
}

/**
 * ⏳️ Settles one owner on a child and never gives it up while a child can still come: `pick()` answers a handle or undefined, `run(handle)` an outcome (`{ reply }`, `{ failure }` or `{ lost }`), `hopeless()` a reason when no child will ever come; an owner waits (`wait`) for a child that is starting or being replaced, runs again on a fresh child when its child was lost (at most `attempts` losses of its own) and ends with `{ why }` per owner, never with a silent miss.
 * @see ./🧬️schema/🔣️.json
 */
export async function warmThroughV1({ pick, run, hopeless, wait, now = () => performance.now(), deadlineMilliseconds = 180_000, attempts = 3 }) {
  const until = now() + deadlineMilliseconds;
  let lost = 0, last = "";
  for (;;) {
    const reason = hopeless();
    if (reason !== undefined) return { why: reason };
    if (now() >= until) return { why: "no child within " + Math.round(deadlineMilliseconds / 1000) + " s" + (last === "" ? "" : ", " + last) };
    const handle = pick();
    if (handle === undefined) { await wait(500); continue; }
    const outcome = await run(handle);
    if (outcome.reply !== undefined) return { reply: outcome.reply };
    if (outcome.failure !== undefined) return { why: "failure: " + String(outcome.failure?.message ?? outcome.failure).replace(/\s+/gu, " ").slice(0, 70) };
    last = "lost: " + String(outcome.lost).replace(/\s+/gu, " ").slice(0, 50);
    if (++lost >= attempts) return { why: last };
    if (/^(closed|aborted)/u.test(String(outcome.lost))) return { why: last };
  }
}

/**
 * 🌊️ The warm pass of one wave: engages the farm, waits for its first child, plans the lanes (one less than the cores, what the farm and the memory plan grant, the child budget the farm really runs with), skips the owners an earlier try made durable and settles the others in the farm's children, lane by lane. `describe(name)` answers `{ key, stamp, cost, job }` of an owner (`key` names the record state the owner is warmed under, `stamp` the stored record file, `cost` the recorded weight, `job` what the child needs); the pass answers its counters, or undefined when it is off (`SEMIO_GRAPH_WARM=off`), there is no farm or no child came in time, so the parent takes the owners itself exactly as before.
 * @see ./🧬️schema/🔣️.json
 */
export async function warmWaveV1({ farm, names, describe, directory, cores, limitMegabytes = Infinity, tell, signal, environment = process.env, waitMilliseconds = 90_000 }) {
  if (environment.SEMIO_GRAPH_WARM === "off" || farm === undefined || typeof farm.warm !== "function" || names.length < WARM_POLICY_V1.minimumOwners) return undefined;
  farm.engage?.();
  if (!(await farm.whenReady(waitMilliseconds))) return undefined;
  const width = planWarmWidthV1({ cores, farmLoops: farm.width(), limitMegabytes, childMegabytes: farm.workerMegabytes?.() ?? WARM_POLICY_V1.childMegabytes, owners: names.length });
  if (width < 1) return undefined;
  sweepWarmPartsV1(directory);
  const index = readWarmIndexV1(directory), lanes = planWarmLanesV1(names.map(name => { const row = describe(name); return { name, key: row.key, cost: row.cost }; }), width);
  return runWarmLanesV1({ lanes, inspect: name => { const row = describe(name); return { key: row.key, stamp: row.stamp }; }, warm: name => farm.warm(describe(name).job), index, signal, tell, why: () => Object.entries(farm.warmReasons?.() ?? {}).map(([reason, count]) => reason + " " + count).join(", ") });
}
