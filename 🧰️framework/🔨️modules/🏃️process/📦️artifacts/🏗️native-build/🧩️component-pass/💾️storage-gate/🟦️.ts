import { appendFileSync, existsSync, mkdirSync, readdirSync, readFileSync, statSync } from "node:fs";
import { dirname, join } from "node:path";
import { quantileR7V1 } from "../../🎟️admission/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

const policy = schema.definitions.Policy.const, messages = schema.definitions.Messages.const;
const MiB = 1024 ** 2;
const mebibytes = (bytes: number): string => String(Math.round(bytes / MiB));
const fill = (text: string, values: Readonly<Record<string, string | number>>): string => text.replace(/\{(\w+)\}/gu, (_, name: string) => String(values[name] ?? ""));

/** 🗄️ Names the closed policy of the storage gate: poll and progress intervals, the growth ledger and the grow rules. */
export const COMPONENT_STORAGE_GATE_POLICY_V1 = policy;

/** 🏃️ One running pass as the gate sees it: what it predicted and the output bytes its finished units already wrote. */
export type StorageRunV1 = Readonly<{ id: string; predictedBytes: number; writtenBytes: number }>;

/** 💰️ What a running pass still charges: its prediction less the bytes it already wrote (the volume's free space already shows those), never negative. Pure and language-agnostic.
 * @see ./🧪️tests/🐍️oracle/🐍️.py */
export function storageChargeBytesV1(row: Pick<StorageRunV1, "predictedBytes" | "writtenBytes">): number {
  return Math.max(0, row.predictedBytes - row.writtenBytes);
}

/** 🚪️ What the gate reads: the free bytes now, the reserve to keep, the pass asking and the passes running. */
export type StorageGateFactsV1 = Readonly<{ headroomBytes: number; reserveBytes: number; predictedBytes: number; running: readonly StorageRunV1[] }>;

/** 🧾️ What the gate decides: `neededBytes` (prediction plus reserve), `chargedBytes` (what running passes still have to write), `freeBytes` (free now less the charges) and the `shortBytes` that would admit the pass. */
export type StorageGateV1 = Readonly<{ admitted: boolean; neededBytes: number; chargedBytes: number; freeBytes: number; shortBytes: number }>;

/** 🧱️ Preserves an admitted pass's measured storage requirement for its outer image owner. */
export class NativeBuildStorageShortfallV1 extends Error {
  readonly minimumFreeBytes: number;
  constructor(minimumFreeBytes: number, message: string) {
    super(message);
    this.minimumFreeBytes = minimumFreeBytes;
    this.name = "NativeBuildStorageShortfallV1";
  }
}

/** ⚖️ Admits one pass when the free bytes, less what the running passes still have to write, hold its prediction and the reserve. Pure and language-agnostic.
 * @see ./🧪️tests/🐍️oracle/🐍️.py */
export function storageGateV1(facts: StorageGateFactsV1): StorageGateV1 {
  const neededBytes = facts.predictedBytes + facts.reserveBytes, chargedBytes = facts.running.reduce((sum, row) => sum + storageChargeBytesV1(row), 0), freeBytes = facts.headroomBytes - chargedBytes;
  return Object.freeze({ admitted: freeBytes >= neededBytes, neededBytes, chargedBytes, freeBytes, shortBytes: Math.max(0, neededBytes - freeBytes) });
}

/** 🌱️ What a grow decision reads: the bytes missing, the passes running, the image's filesystem bytes, the host's free bytes (`undefined` when unreadable), the host reserve and the image's alignment and initial extent. */
export type StorageGrowFactsV1 = Readonly<{ needBytes: number; running: number; imageBytes: number; hostFreeBytes: number | undefined; hostReserveBytes: number; alignBytes: number; initialBytes: number }>;

/** 🌱️ The verdict: `grow` to `targetBytes`, or why not: `busy` (an attached image cannot be resized, so only an idle one grows), `host-reserve` (the host must hold the whole larger envelope and would drop below its hard reserve) or `host-unknown`. */
export type StorageGrowDecisionV1 = Readonly<{ kind: "grow"; targetBytes: number; addedBytes: number; hostFreeAfterBytes: number } | { kind: "busy" } | { kind: "host-reserve"; addedBytes: number; hostFreeAfterBytes: number } | { kind: "host-unknown" }>;

/** 📈️ Decides whether the image may grow by what is missing: never while a pass runs on it, never into the host's hard reserve (CAS7); the host must hold the whole larger envelope plus the initial extent (`hdiutil resize` refuses less, W8). Pure and language-agnostic.
 * @see ../../💾️storage/🍎️image/📈️grow/🟦️.ts
 * @see ./🧪️tests/🐍️oracle/🐍️.py */
export function storageGrowDecisionV1(facts: StorageGrowFactsV1): StorageGrowDecisionV1 {
  if (facts.running > 0) return Object.freeze({ kind: "busy" });
  if (facts.hostFreeBytes === undefined) return Object.freeze({ kind: "host-unknown" });
  const targetBytes = Math.ceil((facts.imageBytes + facts.needBytes) / facts.alignBytes) * facts.alignBytes, addedBytes = targetBytes - facts.imageBytes, hostFreeAfterBytes = facts.hostFreeBytes - (addedBytes + facts.initialBytes);
  return Object.freeze(hostFreeAfterBytes < facts.hostReserveBytes ? { kind: "host-reserve", addedBytes, hostFreeAfterBytes } : { kind: "grow", targetBytes, addedBytes, hostFreeAfterBytes });
}

/** 🔌️ How the gate reaches the image: what it holds now and the one step that grows it to a target (the image is attached while any pass runs, so the step is only called when none does). */
export type StorageGrowPortV1 = Readonly<{ facts(): Readonly<{ imageBytes: number; hostFreeBytes: number | undefined }>; run(targetBytes: number): Promise<void> }>;

/** 🔌️ Everything one admission needs, so it is testable without a build: the numbers, the running passes, the waiting, the collector, the grow step, cancellation and the log. */
export type StorageGatePortsV1 = Readonly<{
  pass: string; predictedBytes: number; measured: boolean; reserveBytes: number;
  headroom(): number; running(): readonly StorageRunV1[]; settled(): readonly Promise<unknown>[];
  reconcile(): void; reclaim?(request: Readonly<{ needBytes: number }>): Promise<void>; grow?: StorageGrowPortV1;
  hold?(): void; check(): void; log(line: string): void; now(): number; sleep(milliseconds: number): Promise<void>;
  pollMilliseconds?: number; progressMilliseconds?: number;
}>;

/** 💬️ Renders one closed gate message in both languages on one line (English first, German second), the way the driver logs its other bilingual lines. */
export function describeStorageGateV1(name: "waiting" | "reclaiming" | "grown" | "growthRefused" | "refused" | "credit", values: Readonly<Record<string, string | number>>, growth?: Readonly<{ key: keyof typeof messages.en.growth; values: Readonly<Record<string, string | number>> }>): string {
  return (["en", "de"] as const).map((language) => {
    const catalogue = messages[language], text = growth === undefined ? "" : fill(catalogue.growth[growth.key], growth.values);
    return fill(catalogue[name], { ...values, unmeasured: values.unmeasured === 1 ? catalogue.unmeasured : "", growth: text, reason: values.reason ?? text });
  }).join(" / ");
}

/** 🚦️ Admits one pass to the shared build directory: it waits while the free bytes less the running passes' remaining charges cannot hold it (re-read every poll, because a running pass lowers its own charge with every byte it writes, not only when it settles), then asks the cache collector, then grows the image when no pass runs and the host keeps its reserve, and refuses with the bilingual error only when all of that failed. The wait line says needed, charged, free and what would release it, every `progressMilliseconds`; cancellation is honored at every wake.
 * @see ./🧬️schema/🔣️.json */
export async function admitPassStorageV1(ports: StorageGatePortsV1): Promise<StorageGateV1> {
  const poll = ports.pollMilliseconds ?? policy.pollMilliseconds, progress = ports.progressMilliseconds ?? policy.progressMilliseconds, unmeasured = ports.measured ? 0 : 1;
  const read = (): StorageGateV1 => storageGateV1({ headroomBytes: ports.headroom(), reserveBytes: ports.reserveBytes, predictedBytes: ports.predictedBytes, running: ports.running() });
  const values = (gate: StorageGateV1, extra: Readonly<Record<string, string | number>> = {}) => ({ pass: ports.pass, needed: mebibytes(gate.neededBytes), predicted: mebibytes(ports.predictedBytes), reserve: mebibytes(ports.reserveBytes), free: mebibytes(gate.freeBytes), charged: mebibytes(gate.chargedBytes), short: mebibytes(gate.shortBytes), running: ports.running().length, written: mebibytes(ports.running().reduce((sum, row) => sum + row.writtenBytes, 0)), unmeasured, ...extra });
  const admit = (value: StorageGateV1): StorageGateV1 => { ports.hold?.(); return value; };
  let gate = read(), said = Number.NEGATIVE_INFINITY;
  while (!gate.admitted && (ports.running().length > 0 || ports.settled().length > 0)) {
    ports.check();
    if (ports.now() - said >= progress) { said = ports.now(); ports.log(describeStorageGateV1("waiting", values(gate), { key: "busy", values: {} })); }
    await Promise.race([...ports.settled(), ports.sleep(poll)]);
    ports.check();
    gate = read();
  }
  if (gate.admitted) return admit(gate);
  ports.reconcile();
  gate = read();
  if (gate.admitted) return admit(gate);
  if (ports.reclaim) { ports.log(describeStorageGateV1("reclaiming", values(gate))); await ports.reclaim({ needBytes: gate.shortBytes }); ports.check(); ports.reconcile(); gate = read(); if (gate.admitted) return admit(gate); }
  let growth: Readonly<{ key: keyof typeof messages.en.growth; values: Readonly<Record<string, string | number>> }> = { key: "none", values: {} };
  if (ports.grow) {
    const held = ports.grow.facts(), decision = storageGrowDecisionV1({ needBytes: gate.shortBytes, running: ports.running().length, imageBytes: held.imageBytes, hostFreeBytes: held.hostFreeBytes, hostReserveBytes: policy.growHostReserveBytes, alignBytes: policy.growAlignBytes, initialBytes: policy.growInitialBytes });
    if (decision.kind === "grow") {
      try {
        await ports.grow.run(decision.targetBytes); ports.check(); ports.reconcile();
        ports.log(describeStorageGateV1("grown", values(gate, { added: mebibytes(decision.addedBytes), target: mebibytes(decision.targetBytes), hostFree: mebibytes(decision.hostFreeAfterBytes), hostReserve: mebibytes(policy.growHostReserveBytes) })));
        gate = read();
        if (gate.admitted) return admit(gate);
        growth = { key: "grow", values: { added: mebibytes(decision.addedBytes) } };
      } catch (error) { ports.check(); growth = { key: "failed", values: { error: (error instanceof Error ? error.message : String(error)).split("\n")[0]!.slice(0, 200) } }; ports.log(describeStorageGateV1("growthRefused", values(gate), growth)); }
    } else {
      growth = { key: decision.kind, values: { added: mebibytes("addedBytes" in decision ? decision.addedBytes : 0), hostFree: mebibytes("hostFreeAfterBytes" in decision ? decision.hostFreeAfterBytes : 0), hostReserve: mebibytes(policy.growHostReserveBytes) } };
      ports.log(describeStorageGateV1("growthRefused", values(gate), growth));
    }
  }
  throw new NativeBuildStorageShortfallV1(gate.neededBytes, describeStorageGateV1("refused", values(gate), growth));
}

/** 🔑️ The ledger key of one batch: its profile, target and sorted component names, so the same batch finds its own measured growth again. */
export function storageGrowthKeyV1(pass: Readonly<{ profile: string; target: string; packages: readonly Readonly<{ name: string }>[] }>): string {
  return [pass.profile, pass.target, pass.packages.map((row) => row.name).sort().join(",")].join("|");
}

/** 📒️ Appends what one finished batch really wrote (the output bytes of the units it compiled, not the ones replayed from the unit cache) to the growth ledger; a failed write is not fatal (the volume may be the thing that is full). */
export function recordStorageGrowthV1(cacheRoot: string, key: string, bytes: number, atMs: number): void {
  if (!(bytes > 0)) return;
  try { const path = join(cacheRoot, policy.ledgerFile); mkdirSync(dirname(path), { recursive: true }); appendFileSync(path, JSON.stringify({ v: 1, key, bytes, atMs }) + "\n"); } catch { return; }
}

/** 📖️ Reads the growth ledger: the newest `ledgerMaximumRows` measurements per key; torn or foreign lines are skipped. */
export function readStorageGrowthV1(cacheRoot: string): ReadonlyMap<string, readonly number[]> {
  const path = join(cacheRoot, policy.ledgerFile), rows = new Map<string, number[]>();
  if (!existsSync(path)) return rows;
  for (const line of readFileSync(path, "utf8").split("\n")) {
    try { const row = JSON.parse(line) as { v?: number; key?: unknown; bytes?: unknown }; if (row.v === 1 && typeof row.key === "string" && typeof row.bytes === "number" && row.bytes > 0) rows.set(row.key, [...rows.get(row.key) ?? [], row.bytes].slice(-policy.ledgerMaximumRows)); } catch { continue; }
  }
  return rows;
}

/** 🎓️ The bytes a batch is predicted to add: the `ledgerQuantile` of its own measured growth once the ledger holds `ledgerMinimumRows` of it, else the footprint computed from the recorded units. Pure and language-agnostic.
 * @see ./🧪️tests/🐍️oracle/🐍️.py */
export function learnedBatchBytesV1(computed: Readonly<{ bytes: number; measured: boolean }>, rows: readonly number[] | undefined): Readonly<{ bytes: number; measured: boolean }> {
  return rows !== undefined && rows.length >= policy.ledgerMinimumRows ? Object.freeze({ bytes: Math.ceil(quantileR7V1(rows, policy.ledgerQuantile)), measured: true }) : computed;
}

/** ♻️ The identities a batch adds to the shared build directory: those no earlier batch of the lane already accounted, which it then records in `present`. Pure and language-agnostic.
 * @see ./🧪️tests/🐍️oracle/🐍️.py */
export function batchAdditionsV1(identities: ReadonlySet<string>, present: Set<string>): ReadonlySet<string> {
  const added = new Set([...identities].filter((identity) => !present.has(identity)));
  for (const identity of identities) present.add(identity);
  return added;
}

/** 🪨️ The bytes already on disk as unit outputs of one profile in the shared build directories: below each root, in the profile's directory (found within `presentDepth` levels, so `<root>/<profile>` and `<root>/<target>/<profile>`), the files of its `deps` directory (the classic layout) and of every `build/<package>/<hash>/out` directory (the per-unit layout of Cargo's build directory); incremental sessions and fingerprints are not walked. It is what a restarted build finds from the run it replaces. Unreadable is zero. */
export function measurePresentDepsBytesV1(roots: readonly string[], profile: string): number {
  const mode = profile === "dev" ? "debug" : profile;
  const files = (directory: string): number => { let total = 0; try { for (const file of readdirSync(directory, { withFileTypes: true })) if (file.isFile()) total += statSync(join(directory, file.name)).size; } catch { return total; } return total; };
  const directories = (directory: string): string[] => { try { return readdirSync(directory, { withFileTypes: true }).filter((entry) => entry.isDirectory()).map((entry) => join(directory, entry.name)); } catch { return []; } };
  const measureProfile = (directory: string): number => files(join(directory, "deps")) + directories(join(directory, "build")).reduce((sum, pack) => sum + directories(pack).reduce((inner, hash) => inner + files(join(hash, "out")), 0), 0);
  const find = (directory: string, level: number): number => directories(directory).reduce((sum, child) => child.split(/[\\/]/u).pop() === mode ? sum + measureProfile(child) : level < policy.presentDepth ? sum + find(child, level + 1) : sum, 0);
  return roots.reduce((sum, root) => sum + find(root, 1), 0);
}

/** 🎁️ Credits the bytes already present in a lane against its batches in planned order: a batch's prediction falls by what is on disk already, never below zero, and what one batch used up the next one does not get again. Returns each row with its remaining `bytes` and the `credit` taken. Pure and language-agnostic.
 * @see ./🧪️tests/🐍️oracle/🐍️.py */
export function creditPresentBytesV1<T extends Readonly<{ bytes: number }>>(rows: readonly T[], laneOf: (row: T) => string, presentBytes: (lane: string) => number): readonly (T & Readonly<{ credit: number }>)[] {
  const left = new Map<string, number>();
  return rows.map((row) => {
    const lane = laneOf(row);
    if (!left.has(lane)) left.set(lane, Math.max(0, presentBytes(lane)));
    const credit = Math.min(row.bytes, left.get(lane)!);
    left.set(lane, left.get(lane)! - credit);
    return Object.freeze({ ...row, bytes: row.bytes - credit, credit });
  });
}
