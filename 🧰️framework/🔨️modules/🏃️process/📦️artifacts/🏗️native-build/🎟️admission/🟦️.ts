import {registeredToolExecutableV1} from "../../../🧰️tools/🏃️run/🟦️.ts";
import {requireNativeStorageReservationV1,type NativeStorageReservationV1} from "../💾️storage/🟦️.ts";
import {requireNativeStoragePathV1} from "../💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {unitCacheChainEnvironmentV1} from "../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🚪️launch/🟦️.ts";
import {cargoPlanningOutputBytesV1,captureCargoCompilationGraphV1,actualCargoCompilerPlanningPortV1} from "../🕸️units/📥️capture/🟦️.ts";
import {admitCargoSourceBoundariesV1,actualCargoSourceAdmissionPortV1,type CargoSourceBoundaryPolicyV1} from "../📥️source/🎟️admission/🟦️.ts";
import { compilerAdmissionBudgetV1, type CompilerAdmissionControl } from "../../../🏗️build/🎟️capacity/🟦️.ts";
import { checkResourceLeaseControl, watchResourceLeaseControlDeadlineV1, requireMaterializedPathV1 } from "../../../🔒️leases/🟦️.ts";
import { spawn } from "node:child_process";
import { randomBytes } from "node:crypto";
import { existsSync, rmSync } from "node:fs";
import { mkdir, open, readFile, readdir, rename, writeFile, rm } from "node:fs/promises";
import { createServer, type Socket } from "node:net";
import { availableParallelism, tmpdir, totalmem } from "node:os";
import { basename, dirname, extname, join, posix, resolve } from "node:path";
import { createInterface } from "node:readline";
import { fileURLToPath } from "node:url";
import { captureSchedulerReceiptV1, readSchedulerReceiptV1, schedulerIdentityV1, schedulerReceiptFileV1, schedulerReceiptHoldsV1, writeSchedulerReceiptV1 } from "./🔖️scheduler/🟦️.ts";
import { acquireQueuedResourceLease } from "../../../🔒️leases/🟦️.ts";
import { terminateOwnedChildTree } from "../../../🪓️termination/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { hostAvailableBytesV1 } from "../../../🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

type WardenModuleV1 = typeof import("../../../🏗️build/🌡️host-capacity/🛡️warden/🔁️loop/🟦️.ts");
let wardenLoading: Promise<WardenModuleV1 | null> | undefined, wardenLoaded: WardenModuleV1 | null = null;
const loadWardenV1 = (): Promise<WardenModuleV1 | null> => wardenLoading ??= import("../../../🏗️build/🌡️host-capacity/🛡️warden/🔁️loop/🟦️.ts").then((module) => (wardenLoaded = module), () => null);

/** 🎟️ The admission policy constants: reserve, prediction headroom, cold-start model and feedback thresholds. */
export const ADMISSION_POLICY_V1 = schema.definitions.Policy.const;

/** 🧠️ Bounds physical memory by an actual positive process constraint; zero reports no known bound. */
export function admissionMemoryBytesV1(physicalBytes=totalmem(),constrainedBytes=process.constrainedMemory()):number{
 if(!Number.isSafeInteger(physicalBytes)||physicalBytes<1||!Number.isSafeInteger(constrainedBytes)||constrainedBytes<0)throw Error("Actual finite memory capacity required");
 return constrainedBytes>0?Math.min(physicalBytes,constrainedBytes):physicalBytes;
}

/** 🛟️ The memory kept back from admission: the policy share of the effective capacity, capped by the policy maximum. */
export function admissionReserveBytesV1(effectiveBytes:number):number{
 if(!Number.isSafeInteger(effectiveBytes)||effectiveBytes<1)throw Error("Actual finite memory capacity required");
 return Math.min(ADMISSION_POLICY_V1.maximumReserveBytes,Math.floor(effectiveBytes*ADMISSION_POLICY_V1.reserveBasisPoints/10000));
}

/** ⚖️ Shares effective memory after the unchanged reserve without reconstructing capacity above its real bound. */
export function admissionMemoryShareV1(effectiveBytes:number,tokens:number,parallelism:number):number{
 if(![effectiveBytes,tokens,parallelism].every(value=>Number.isSafeInteger(value)&&value>0)||tokens>parallelism)throw Error("Actual bounded memory share required");
 const reserve=admissionReserveBytesV1(effectiveBytes);return Math.min(effectiveBytes,reserve+Math.floor((effectiveBytes-reserve)*tokens/parallelism));
}


/** 📨️ The wrapper's admission request (`AdmitRequestV1`). */
export type AdmitRequestV1 = Readonly<{ v: 1; op: "admit"; pass: string; pid: number; crate: string; package: string; version: string; target: string; crateTypes: readonly string[]; outDir: string; extraFilename: string; source: string; manifestDir: string; primary: boolean; buildScriptOutDir: string | null; externs: readonly string[] }>;

/** 🏁️ The wrapper's completion report (`DoneReportV1`). */
export type DoneReportV1 = Readonly<{ v: 1; op: "done"; exit: number; peakRssBytes: number; userMs: number; sysMs: number; startMs: number; endMs: number; rmetaMs: number | null }>;

/** 📦️ The measurements recorded when a cached unit was first built (`ProvenanceV1`). */
export type CachedProvenanceV1 = Readonly<{ peakRssBytes: number; userMs: number; sysMs: number; wallMs: number; rmetaOffsetMs: number | null }>;

/** 📣️ A wrapper's report that a unit was served from the unit cache instead of compiled (`CachedRequestV1`): the admission facts, the cache key and the recorded measurements. */
export type CachedRequestV1 = Readonly<Omit<AdmitRequestV1, "op"> & { op: "cached"; cacheKey: string; provenance: CachedProvenanceV1 | null }>;

/** 🔭️ A wrapper's question for the finished record of the unit its scheduler just reported (`RecallRequestV1`). */
export type RecallRequestV1 = Readonly<{ v: 1; op: "recall"; pass: string; crate: string; outDir: string; extraFilename: string }>;

/** 🏔️ The unit ceiling applied to recorded observations (`UnitCeilingVerdictV1`). */
export type UnitCeilingVerdictV1 = Readonly<{ version: 1; ceilingMegabytes: number; exceeded: readonly Readonly<{ key: string; crate: string; target: string; peakMegabytes: number; listed: boolean }>[]; resolved: readonly Readonly<{ crate: string; target: string; peakMegabytes: number }>[]; ok: boolean }>;

/** 📒️ One recorded observation of a unit key (`ObservationV1`), the scheduling knowledge `units.jsonl` keeps. */
export type UnitObservationV1 = Readonly<{ v: 1; key: string; peakRssBytes: number; outputBytes: number; wallMs: number; cpuMs: number; remainingMs: number; atMs: number; lines?: number }>;

/** 🔗️ One dependency of a unit by the artifact identities it may name and what it consumes: metadata only (pipelined) or the complete library. */
export type UnitEdgeV1 = Readonly<{ artifacts: readonly string[]; edge: "rmeta" | "full" }>;

/** 🧾️ One admitted rustc unit as the broker measured it. */
export type UnitRecordV1 = Readonly<{ key: string; pass: string; profile: string; target: string; crate: string; package: string; version: string; crateTypes: readonly string[]; primary: boolean; artifact: string; dependencies: readonly UnitEdgeV1[]; buildScriptOutDir: string | null; lines?: number; predictedBytes: number; requestMs: number; admittedMs: number; startMs: number; endMs: number; rmetaMs: number | null; peakRssBytes: number; userMs: number; sysMs: number; exit: number; cache?: Readonly<{ key: string; recorded: boolean }> | null }>;

/** 🧳️ A unit waiting for admission, as the ordering and decision laws see it. */
export type AdmissionWaiterV1 = Readonly<{ predictedBytes: number; expectedWallMs: number | null; remainingMs: number; arrival: number }>;

/** 🏃️ A running unit, as the decision law sees it. */
export type AdmissionRunningV1 = Readonly<{ predictedBytes: number; expectedEndMs: number | null }>;

/** 🧮️ The machine state one admission decision is taken in. */
export type AdmissionStateV1 = Readonly<{ budgetBytes: number; frozen: boolean; nowMs: number; running: readonly AdmissionRunningV1[] }>;

/** 🛂️ Validates one value against a named admission schema definition. */
function admitDefinition<T>(name: keyof typeof schema.definitions, value: unknown): T {
  const errors = validateJsonSchemaSubset(schema.definitions[name], value, schema);
  if (errors.length) throw Error(`Invalid ${name}: ${errors.join("; ")}`);
  return value as T;
}

/** 🔑️ The scheduling identity of a unit: package, crate, target, profile and crate types. */
export function unitKeyV1(request: Pick<AdmitRequestV1, "package" | "crate" | "target" | "crateTypes">, profile: string): string {
  return `${request.package}@${request.crate}|${request.target}|${profile}|${[...request.crateTypes].sort().join(",")}`;
}

/** 🏷️ The artifact identity a unit produces: its output directory and file stem (`<crate><extra-filename>`). */
export function unitArtifactV1(request: Pick<AdmitRequestV1, "outDir" | "crate" | "extraFilename">): string {
  return join(resolve(request.outDir), `${request.crate}${request.extraFilename}`);
}

/** 🏺️ The artifact identities an `--extern` path may name (`lib` prefixes are ambiguous for crates named `lib*`) and its edge kind. */
export function externArtifactsV1(path: string): { readonly candidates: readonly string[]; readonly edge: "rmeta" | "full" } {
  const directory = dirname(resolve(path)), stem = basename(path, extname(path));
  return { candidates: [...new Set([join(directory, stem.replace(/^lib/u, "")), join(directory, stem)])], edge: extname(path) === ".rmeta" ? "rmeta" : "full" };
}

/** 📐️ The `q` quantile with linear interpolation between closest ranks (R-7, the NumPy and d3 default). */
export function quantileR7V1(values: readonly number[], q: number): number {
  if (!values.length) throw Error("Quantile of no values");
  const sorted = [...values].sort((left, right) => left - right), position = (sorted.length - 1) * q, lower = Math.floor(position);
  return lower + 1 >= sorted.length ? sorted[lower]! : sorted[lower]! + (position - lower) * (sorted[lower + 1]! - sorted[lower]!);
}

/** 🧊️ Whether a history key names a component final (a non-host `cdylib`) of `profile`: the population the cold component model is derived from. */
export function componentFinalKeyV1(key: string, profile: string): boolean {
  const [, target = "host", keyProfile = "", types = ""] = key.split("|");
  return target !== "host" && keyProfile === profile && types.split(",").includes("cdylib");
}

/** 🗂️ Every recorded peak of the component finals of `profile`: the evidence the cold component model reads. */
export function componentFinalPeaksV1(history: ReadonlyMap<string, readonly UnitObservationV1[]>, profile: string): number[] {
  return [...history].filter(([key]) => componentFinalKeyV1(key, profile)).flatMap(([, rows]) => rows.map((row) => row.peakRssBytes));
}

/** 🧾️ The largest recorded peak of every crate that has a component final (a non-host `cdylib`) in the history, in megabytes, keyed by the crate name `rustc` uses: the evidence that sizes a final for the fleet. */
export function componentFinalCratePeaksV1(history: ReadonlyMap<string, readonly UnitObservationV1[]>): Map<string, number> {
  const crates = new Map<string, number>();
  for (const [key, rows] of history) {
    const [identity = "", target = "host", , types = ""] = key.split("|"), crate = identity.slice(identity.indexOf("@") + 1);
    if (target === "host" || !types.split(",").includes("cdylib") || !crate || !rows.length) continue;
    crates.set(crate, Math.max(crates.get(crate) ?? 0, Math.ceil(Math.max(...rows.map((row) => row.peakRssBytes)) / 1048576)));
  }
  return crates;
}

/** 🚧️ The crate and target of a unit key (`<package>@<crate>|<target>|<profile>|<crate types>`), or `null` for a malformed key. */
export function unitKeyPartsV1(key: string): Readonly<{ crate: string; target: string }> | null {
  const [identity = "", target = "", profile, types, ...rest] = key.split("|"), crate = identity.slice(identity.indexOf("@") + 1);
  return identity.includes("@") && crate && target && profile !== undefined && types !== undefined && !rest.length ? { crate, target } : null;
}

/** 🗻️ The unit ceiling of the policy applied to ledger rows: the largest peak per unit key (rounded up to megabytes), the units above the ceiling (`listed` when on the carving list), the carving entries that no longer exceed it, and `ok` when no unlisted unit exceeds it and no entry is stale (the list may only shrink). Rows with a malformed key are ignored; order is peak descending, then key. */
export function unitCeilingVerdictV1(policy: Readonly<{ megabytes: number; carving: readonly Readonly<{ crate: string; target: string }>[] }>, observations: readonly Pick<UnitObservationV1, "key" | "peakRssBytes">[]): UnitCeilingVerdictV1 {
  const peaks = new Map<string, number>(), crates = new Map<string, number>();
  for (const row of observations) {
    const parts = unitKeyPartsV1(row.key), megabytes = Math.ceil(row.peakRssBytes / 1048576);
    if (!parts || !Number.isFinite(megabytes)) continue;
    peaks.set(row.key, Math.max(peaks.get(row.key) ?? 0, megabytes));
    crates.set(`${parts.crate}|${parts.target}`, Math.max(crates.get(`${parts.crate}|${parts.target}`) ?? 0, megabytes));
  }
  const listed = (crate: string, target: string) => policy.carving.some((entry) => entry.crate === crate && entry.target === target);
  const exceeded = [...peaks].filter(([, peak]) => peak > policy.megabytes).map(([key, peak]) => { const { crate, target } = unitKeyPartsV1(key)!; return { key, crate, target, peakMegabytes: peak, listed: listed(crate, target) }; })
    .sort((left, right) => right.peakMegabytes - left.peakMegabytes || (left.key < right.key ? -1 : left.key > right.key ? 1 : 0));
  const resolved = policy.carving.flatMap((entry) => { const peak = crates.get(`${entry.crate}|${entry.target}`); return peak !== undefined && peak <= policy.megabytes ? [{ crate: entry.crate, target: entry.target, peakMegabytes: peak }] : []; })
    .sort((left, right) => right.peakMegabytes - left.peakMegabytes || (left.crate < right.crate ? -1 : left.crate > right.crate ? 1 : left.target < right.target ? -1 : left.target > right.target ? 1 : 0));
  return { version: 1, ceilingMegabytes: policy.megabytes, exceeded, resolved, ok: exceeded.every((unit) => unit.listed) && !resolved.length };
}

/** 🔔️ The lines that name what breaks the unit ceiling (every unlisted unit above it, every stale carving entry) in `language`; empty when the verdict is `ok`. */
export function formatUnitCeilingV1(language: "en" | "de", verdict: UnitCeilingVerdictV1): string[] {
  const text = schema.definitions.Messages.const[language].unitCeiling, fill = (line: string, values: Readonly<Record<string, string | number>>) => line.replace(/\{([a-z]+)\}/gu, (match, name: string) => name in values ? String(values[name]) : match);
  return [
    ...verdict.exceeded.filter((unit) => !unit.listed).map((unit) => fill(text.exceeded, { key: unit.key, peak: unit.peakMegabytes, ceiling: verdict.ceilingMegabytes })),
    ...verdict.resolved.map((entry) => fill(text.resolved, { crate: entry.crate, peak: entry.peakMegabytes, ceiling: verdict.ceilingMegabytes })),
  ];
}

/** 🛎️ The one bilingual line (English first, German second) a completed unit raises when its own peak is above the unit ceiling and it is not on the carving list, or `null`. */
export function unitCeilingAlarmV1(policy: Parameters<typeof unitCeilingVerdictV1>[0], row: Pick<UnitObservationV1, "key" | "peakRssBytes">): string | null {
  const verdict = unitCeilingVerdictV1(policy, [row]), unit = verdict.exceeded.find((one) => !one.listed);
  return unit ? (["en", "de"] as const).map((language) => formatUnitCeilingV1(language, { ...verdict, exceeded: [unit], resolved: [] })[0]!).join(" / ") : null;
}

/** 🔮️ Predicted peak resident bytes of a unit: `max(headroom × p95(last window peaks), floor)`; without history the cold-start model. A component final (`component` = recorded peaks of its kind) without history is predicted from that evidence, `max(headroom × quantile(peaks, componentColdQuantile), floor)`, or the `componentFinalBytes` bound while fewer than `componentColdMinimumRows` were recorded. */
export function predictPeakBytesV1(peaks: readonly number[], cold: Readonly<{ kloc: number; component: readonly number[] | null }>): number {
  const policy = ADMISSION_POLICY_V1;
  if (peaks.length) return Math.max(Math.ceil(policy.headroom * quantileR7V1(peaks.slice(-policy.window), policy.quantile)), policy.floorBytes);
  if (cold.component === null) return Math.max(Math.ceil(policy.coldBaseBytes + policy.coldBytesPerKloc * cold.kloc), policy.floorBytes);
  return cold.component.length < policy.componentColdMinimumRows ? policy.componentFinalBytes : Math.max(Math.ceil(policy.headroom * quantileR7V1(cold.component, policy.componentColdQuantile)), policy.floorBytes);
}

/** 🧬️ The rows of a unit key's history that were recorded for a source of comparable size (`lines` within the policy tolerance of the current `lines`): the forecast follows the unit's inputs, so a crate that was split or grown is predicted from its size, not from a peak measured for other sources; rows without a recorded size are stale. */
export function forecastHistoryV1(history: readonly UnitObservationV1[], lines: number): UnitObservationV1[] {
  const bound = 1 + ADMISSION_POLICY_V1.forecastSizeToleranceBasisPoints / 10000;
  return history.filter((row) => row.lines !== undefined && (lines === 0 || row.lines === 0 ? row.lines === lines : Math.max(row.lines, lines) <= bound * Math.min(row.lines, lines)));
}

/** ⏳️ Expected wall time of a key: the median of its recorded walls in the window, rounded half up; `null` without history. */
export function expectedWallMsV1(history: readonly UnitObservationV1[]): number | null {
  return history.length ? Math.round(quantileR7V1(history.slice(-ADMISSION_POLICY_V1.window).map((row) => row.wallMs), 0.5)) : null;
}

/** 🥇️ Waiter priority: longest recorded remaining path first, then the largest predicted peak, then arrival order. */
export function admissionOrderV1<T extends AdmissionWaiterV1>(waiters: readonly T[]): T[] {
  return [...waiters].sort((left, right) => right.remainingMs - left.remainingMs || right.predictedBytes - left.predictedBytes || left.arrival - right.arrival);
}

/** 🚦️ Which ordered waiters start now. A waiter starts while the predicted total fits the budget; the first waiter that does not fit holds a
 * reservation (EASY backfilling): later waiters only start if they fit and either end before the reservation's shadow time or fit in the memory
 * left beside it. With nothing running the first waiter always starts, so progress never depends on a prediction. A frozen broker starts nothing
 * while anything runs. Returns indices into `ordered`. */
export function admitWaitersV1(state: AdmissionStateV1, ordered: readonly AdmissionWaiterV1[]): number[] {
  const admitted: number[] = [];
  const releases = state.running.map((row) => ({ bytes: row.predictedBytes, endMs: row.expectedEndMs ?? Infinity }));
  let available = state.budgetBytes - releases.reduce((sum, row) => sum + row.bytes, 0), running = releases.length;
  let shadow: { atMs: number; extraBytes: number } | null = null;
  for (let index = 0; index < ordered.length; index++) {
    const waiter = ordered[index]!, endMs = waiter.expectedWallMs === null ? Infinity : state.nowMs + waiter.expectedWallMs;
    if (running === 0) { admitted.push(index); running++; available -= waiter.predictedBytes; releases.push({ bytes: waiter.predictedBytes, endMs }); continue; }
    if (state.frozen) break;
    if (shadow === null) {
      if (waiter.predictedBytes <= available) { admitted.push(index); running++; available -= waiter.predictedBytes; releases.push({ bytes: waiter.predictedBytes, endMs }); continue; }
      let free = available, atMs = state.nowMs;
      for (const release of [...releases].sort((left, right) => left.endMs - right.endMs)) { if (free >= waiter.predictedBytes) break; free += release.bytes; atMs = release.endMs; }
      shadow = { atMs, extraBytes: free - waiter.predictedBytes };
      continue;
    }
    if (waiter.predictedBytes > available || !(endMs < shadow.atMs || waiter.predictedBytes <= shadow.extraBytes)) continue;
    admitted.push(index);
    running++;
    available -= waiter.predictedBytes;
    if (endMs >= shadow.atMs) shadow.extraBytes -= waiter.predictedBytes;
  }
  return admitted;
}

/** 🔭️ Whether a unit that asked now would queue: the ordered decision law run once more with the candidate among the waiters, without changing any state. */
export function admissionWouldWaitV1(state: AdmissionStateV1, waiting: readonly AdmissionWaiterV1[], candidate: AdmissionWaiterV1): boolean {
  const ordered = admissionOrderV1([...waiting, candidate]);
  return !admitWaitersV1(state, ordered).includes(ordered.indexOf(candidate));
}

/** 🧊️ Admission freeze with hysteresis: freeze when measured rustc memory exceeds the budget or swap-out exceeds its rate; resume below the resume fraction. */
export function admissionFrozenV1(wasFrozen: boolean, sample: Readonly<{ actualBytes: number; budgetBytes: number; swapOutBytesPerSecond: number | null }>): boolean {
  const policy = ADMISSION_POLICY_V1, swapping = (sample.swapOutBytesPerSecond ?? 0) > policy.swapOutBytesPerSecond;
  if (sample.actualBytes > sample.budgetBytes || swapping) return true;
  return wasFrozen && sample.actualBytes >= policy.resumeFraction * sample.budgetBytes;
}

/** 🌡️ The verdict of the available-memory law: whether a unit starts now, and why (`fits` it fits the room, `available` it waits for room, `warden` the host warden is at guard or danger, `waited-out` the wait bound ended on an idle broker, `unknown` the host reports no availability). */
export type AvailableAdmissionV1 = Readonly<{ admit: boolean; reason: "fits" | "available" | "warden" | "waited-out" | "unknown" }>;

/** 🌡️ The inputs of `availableAdmissionV1`: `runningUnits` counts the units of this broker that run, `cancelled` marks a unit the host warden cancelled before (its wait bound doubles). */
export type AvailableAdmissionInputV1 = Readonly<{ predictedBytes: number; availableBytes: number | undefined; totalBytes: number; runningUnrealizedBytes: number; runningUnits: number; waitedMilliseconds: number; wardenLevel: "calm" | "guard" | "danger" | undefined; cancelled?: boolean; policy: Pick<typeof ADMISSION_POLICY_V1.available, "marginBasisPoints" | "waitMilliseconds"> }>;

/** 🌡️ Admits a unit against the measured available memory: unknown availability admits; a warden at guard or danger holds (the wait bound never overrides it); otherwise the unit starts when its predicted peak fits the available bytes less the margin share of total memory less the unrealized remainder of the running units, or when it waited the policy bound (twice for a cancelled unit) with nothing running.
 * @see ../../../🏗️build/🌡️host-capacity/🛡️warden/🔁️loop/🟦️.ts (`readHostWardenStateV1`) */
export function availableAdmissionV1(input: AvailableAdmissionInputV1): AvailableAdmissionV1 {
  if (input.availableBytes === undefined) return { admit: true, reason: "unknown" };
  if (input.wardenLevel === "guard" || input.wardenLevel === "danger") return { admit: false, reason: "warden" };
  if (input.predictedBytes <= input.availableBytes - Math.floor(input.totalBytes * input.policy.marginBasisPoints / 10000) - input.runningUnrealizedBytes) return { admit: true, reason: "fits" };
  return input.runningUnits === 0 && input.waitedMilliseconds >= input.policy.waitMilliseconds * (input.cancelled ? 2 : 1) ? { admit: true, reason: "waited-out" } : { admit: false, reason: "available" };
}

/** 📣️ The line (`language`) that says why a unit is held back: `available` (not enough room), `warden` (the host warden suspends or cancels) or `cancelled` (the warden cancelled this unit before). */
export function formatAdmissionHeldV1(language: "en" | "de", kind: "available" | "warden" | "cancelled", values: Readonly<Record<"unit" | "predicted" | "available" | "margin" | "level" | "seconds", string | number>>): string {
  return schema.definitions.Messages.const[language].held[kind].replace(/\{([a-z]+)\}/gu, (match, name: string) => name in values ? String(values[name as keyof typeof values]) : match);
}

/** 🗄️ The default telemetry store: `<cache>/build-telemetry/units.jsonl`, never cleared by fresh builds. */
export function unitHistoryPathV1(cacheRoot: string): string {
  return join(cacheRoot, ADMISSION_POLICY_V1.storeDirectory, ADMISSION_POLICY_V1.storeFile);
}

/** 🗜️ Keeps the newest `window` observations of every key, ordered by time. An observation is identified by its key and time: a later row of the same identity (the refinement of an incrementally recorded one) replaces the earlier. */
export function compactUnitHistoryV1(rows: readonly UnitObservationV1[]): Map<string, UnitObservationV1[]> {
  const identities = new Map<string, UnitObservationV1>(), history = new Map<string, UnitObservationV1[]>();
  for (const row of rows) identities.set(`${row.atMs}|${row.key}`, row);
  for (const row of [...identities.values()].sort((left, right) => left.atMs - right.atMs)) history.set(row.key, [...(history.get(row.key) ?? []), row].slice(-ADMISSION_POLICY_V1.window));
  return history;
}

/** 📖️ Reads the store. A record is complete exactly when its line is newline-terminated: the unterminated tail of an interrupted append is dropped, every terminated line must satisfy `ObservationV1`. */
export async function readUnitHistoryV1(path: string): Promise<Map<string, UnitObservationV1[]>> {
  if (!existsSync(path)) return new Map();
  const text = await readFile(path, "utf8");
  return compactUnitHistoryV1(text.slice(0, text.lastIndexOf("\n") + 1).split("\n").filter(Boolean).map((line) => admitDefinition<UnitObservationV1>("ObservationV1", JSON.parse(line))));
}

/** ✍️ Merges new observations into the store under its exclusive lease and rewrites it compacted, atomically. */
export async function recordUnitHistoryV1(path: string, rows: readonly UnitObservationV1[], options: Readonly<{ leaseDirectory: string; control: CompilerAdmissionControl }>): Promise<void> {
  checkResourceLeaseControl(options.control);
  path = requireMaterializedPathV1(path);
  const temporary = requireMaterializedPathV1(`${path}.${process.pid}.${randomBytes(4).toString("hex")}.tmp`);
  for (const row of rows) admitDefinition<UnitObservationV1>("ObservationV1", row);
  const lease = await acquireQueuedResourceLease({ directory: options.leaseDirectory, resource: JSON.stringify([ADMISSION_POLICY_V1.storeNamespace, resolve(path)]), mode: "exclusive", owner: randomBytes(16).toString("base64url"), signal: options.control.signal, control: options.control });
  try {
    const merged = compactUnitHistoryV1([...[...(await readUnitHistoryV1(path)).values()].flat(), ...rows]);
    checkResourceLeaseControl(options.control);
    await mkdir(dirname(path), { recursive: true });
    checkResourceLeaseControl(options.control);
    await writeFile(temporary, [...merged.values()].flat().sort((left, right) => left.atMs - right.atMs).map((row) => `${JSON.stringify(row)}\n`).join(""));
    checkResourceLeaseControl(options.control);
    await rename(temporary, path);
    checkResourceLeaseControl(options.control);
  } finally { try { await rm(temporary, { force: true }); } finally { lease.release(); } }
}

/** 🪵️ Appends observations durably as units complete: under the store's exclusive lease it cuts an unterminated tail left by a killed writer, writes whole newline-terminated records with one append and syncs them, so a crash loses at most the record being written and concurrent writers never interleave. Rows are refined later by recording the same key and time again; `recordUnitHistoryV1` compacts the store. */
export async function appendUnitHistoryV1(path: string, rows: readonly UnitObservationV1[], options: Readonly<{ leaseDirectory: string; control: CompilerAdmissionControl }>): Promise<void> {
  checkResourceLeaseControl(options.control);
  path = requireMaterializedPathV1(path);
  for (const row of rows) admitDefinition<UnitObservationV1>("ObservationV1", row);
  if (!rows.length) return;
  const lease = await acquireQueuedResourceLease({ directory: options.leaseDirectory, resource: JSON.stringify([ADMISSION_POLICY_V1.storeNamespace, resolve(path)]), mode: "exclusive", owner: randomBytes(16).toString("base64url"), signal: options.control.signal, control: options.control });
  try {
    checkResourceLeaseControl(options.control);
    await mkdir(dirname(path), { recursive: true });
    const handle = await open(path, "a+");
    try {
      const { size } = await handle.stat(), block = Buffer.alloc(Math.min(size, 65536));
      let end = size;
      while (end > 0) {
        const start = Math.max(0, end - block.length), { bytesRead } = await handle.read(block, 0, end - start, start), cut = block.subarray(0, bytesRead).lastIndexOf(10);
        if (cut >= 0) { end = start + cut + 1; break; }
        end = start;
      }
      if (end < size) await handle.truncate(end);
      await handle.appendFile(rows.map((row) => `${JSON.stringify(row)}\n`).join(""));
      await handle.sync();
    } finally { await handle.close(); }
  } finally { lease.release(); }
}

const klocCache = new Map<string, Promise<number>>();

/** 📏️ Thousands of Rust source lines under a crate root's directory, the cold-start size estimate (cached per directory). */
export function sourceKlocV1(source: string): Promise<number> {
  const root = dirname(resolve(source));
  let cached = klocCache.get(root);
  if (!cached) {
    const walk = async (directory: string): Promise<number> => {
      let lines = 0;
      for (const entry of await readdir(directory, { withFileTypes: true }).catch(() => [])) {
        if (entry.isDirectory() && !ADMISSION_POLICY_V1.sourceSkipDirectories.includes(entry.name)) lines += await walk(join(directory, entry.name));
        else if (entry.isFile() && entry.name.endsWith(ADMISSION_POLICY_V1.sourceExtension)) { const text = await readFile(join(directory, entry.name)); for (const byte of text) if (byte === 10) lines++; }
      }
      return lines;
    };
    cached = walk(root).then((lines) => lines / 1000);
    klocCache.set(root, cached);
  }
  return cached;
}

/** 🐧️ The bytes a Unix socket path must stay below on macOS (104) and Linux (108). */
const SOCKET_PATH_BYTES_V1 = 100;

/** 🔌️ A private broker endpoint: a short Unix socket path (sockets are limited to ~104 bytes) or a Windows named pipe. */
export function brokerEndpointV1(platform: NodeJS.Platform = process.platform, temporaryDirectory = tmpdir()): string {
  const name = `semio-broker-${process.pid}-${randomBytes(6).toString("hex")}`;
  if (platform === "win32") return `\\\\.\\pipe\\${name}`;
  const preferred = posix.join(temporaryDirectory, `${name}.sock`);
  return Buffer.byteLength(preferred) < SOCKET_PATH_BYTES_V1 ? preferred : posix.join("/tmp", `${name}.sock`);
}

/** 🛣️ One registered Cargo pass, so units can be keyed by profile. */
export type BrokerPassV1 = Readonly<{ id: string; profile: string; target: string }>;

/** 📡️ Live broker counters for the timeline and the stall watchdog. */
/** 🧮 Current compiler counts, memory budget, resident bytes and pressure charge. */
export type BrokerStatsV1 = Readonly<{ running: number; waiting: number; frozen: boolean; budgetBytes: number; predictedRunningBytes: number; actualRunningBytes: number; chargedRunningBytes: number; lastActivityMs: number }>;

/** 🔏️ The driver's admission broker. */
export interface AdmissionBrokerV1 {
  readonly endpoint: string;
  registerPass(pass: BrokerPassV1): void;
  observe(sample: Readonly<{ nonRustcBytes: number; swapOutBytesPerSecond: number | null }>): void;
  stats(): BrokerStatsV1;
  units(): readonly UnitRecordV1[];
  close(): Promise<void>;
}

type Entry = { request: AdmitRequestV1; key: string; lines: number; profile: string; predictedBytes: number; expectedWallMs: number | null; remainingMs: number; arrival: number; requestMs: number; admittedMs: number; rssBytes: number; chargedBytes: number; cancelled: boolean; heldLoggedMs: number; socket: Socket };

/** ▶️ Starts the admission broker: wrappers connect, ask for admission, stream resident memory and report completion.
 * The budget is `physical − reserve − rolling maximum of the driver's other children`; measured overshoot or swap-out freezes admission. */
export async function startAdmissionBrokerV1(options: Readonly<{ physicalBytes: number; history: ReadonlyMap<string, readonly UnitObservationV1[]>; endpoint?: string; now?: () => number; onCompleted?: (unit: UnitRecordV1) => void; totalBytes?: number; available?: () => number | undefined; warden?: () => "calm" | "guard" | "danger" | undefined; cancels?: (sinceMs: number) => readonly Readonly<{ command: string; megabytes: number }>[] | Promise<readonly Readonly<{ command: string; megabytes: number }>[]>; log?: (line: string) => void }>): Promise<AdmissionBrokerV1> {
  const now = options.now ?? Date.now, endpoint = options.endpoint ?? brokerEndpointV1(), passes = new Map<string, BrokerPassV1>();
  const waiting: Entry[] = [], running = new Set<Entry>(), completed: UnitRecordV1[] = [], nonRustc: { atMs: number; bytes: number }[] = [];
  const availablePolicy = ADMISSION_POLICY_V1.available, totalBytes = options.totalBytes ?? admissionMemoryBytesV1(), heldLogMs = Math.max(availablePolicy.pollMilliseconds, Math.floor(availablePolicy.waitMilliseconds / 6));
  const readAvailable = options.available ?? hostAvailableBytesV1, readLevel = options.warden ?? (() => wardenLoaded?.readHostWardenStateV1()?.level);
  const readCancels = options.cancels ?? (async (sinceMs: number) => (await loadWardenV1())?.readHostWardenCancelsSinceV1(sinceMs) ?? []);
  let poll: ReturnType<typeof setTimeout> | undefined, closed = false;
  void loadWardenV1().then((module) => { try { module?.ensureHostWardenV1(); } catch { return; } });
  let frozen = false, swapOutBytesPerSecond: number | null = null, arrivals = 0, lastActivityMs = now();
  const reserve = admissionReserveBytesV1(options.physicalBytes);
  const budget = (): number => options.physicalBytes - reserve - Math.max(0, ...nonRustc.map((row) => row.bytes));
  const verdict = (socket: Socket, admitted: boolean): void => { if (!socket.destroyed) socket.write(`{"v":1,"op":"verdict","admitted":${admitted}}\n`); };
  const charged = (): number => [...running].reduce((sum, entry) => sum + entry.chargedBytes, 0);
  const stateNow = (isFrozen: boolean): AdmissionStateV1 => ({ budgetBytes: budget(), frozen: isFrozen, nowMs: now(), running: [...running].map((entry) => ({ predictedBytes: Math.max(entry.predictedBytes, entry.chargedBytes), expectedEndMs: entry.expectedWallMs === null ? null : entry.admittedMs + entry.expectedWallMs })) });
  const schedule = (): void => {
    frozen = admissionFrozenV1(frozen, { actualBytes: charged(), budgetBytes: budget(), swapOutBytesPerSecond });
    const ordered = admissionOrderV1(waiting), state = stateNow(frozen);
    const admitted = admitWaitersV1(state, ordered);
    let held = false;
    if (admitted.length) {
      const safely = <T>(read: () => T): T | undefined => { try { return read(); } catch { return undefined; } };
      const availableBytes = safely(readAvailable), wardenLevel = safely(readLevel), margin = Math.floor(totalBytes * availablePolicy.marginBasisPoints / 10000);
      for (const index of admitted) {
        const entry = ordered[index]!, atMs = now(), unrealized = [...running].reduce((sum, row) => sum + Math.max(0, row.predictedBytes - row.chargedBytes), 0);
        const decision = availableAdmissionV1({ predictedBytes: entry.predictedBytes, availableBytes, totalBytes, runningUnrealizedBytes: unrealized, runningUnits: running.size, waitedMilliseconds: atMs - entry.requestMs, wardenLevel, cancelled: entry.cancelled, policy: availablePolicy });
        if (!decision.admit) {
          held = true;
          if (options.log && atMs - entry.heldLoggedMs >= heldLogMs) {
            entry.heldLoggedMs = atMs;
            const megabytes = (bytes: number): number => Math.round(bytes / 1048576), kind = decision.reason === "warden" ? "warden" : entry.cancelled ? "cancelled" : "available";
            const values = { unit: entry.request.crate, predicted: megabytes(entry.predictedBytes), available: availableBytes === undefined ? "?" : megabytes(availableBytes), margin: megabytes(margin), level: wardenLevel ?? "unknown", seconds: Math.round((atMs - entry.requestMs) / 1000) };
            try { options.log(`[admission] ${formatAdmissionHeldV1("en", kind, values)} / ${formatAdmissionHeldV1("de", kind, values)}`); } catch { held = true; }
          }
          continue;
        }
        waiting.splice(waiting.indexOf(entry), 1);
        entry.admittedMs = now();
        running.add(entry);
        lastActivityMs = entry.admittedMs;
        verdict(entry.socket, true);
      }
    }
    if (held && !poll && !closed) { poll = setTimeout(() => { poll = undefined; schedule(); }, availablePolicy.pollMilliseconds); poll.unref?.(); }
  };
  const forget = (entry: Entry | undefined): void => {
    if (!entry) return;
    const index = waiting.indexOf(entry);
    if (index >= 0) waiting.splice(index, 1);
    running.delete(entry);
    schedule();
  };
  const componentPeaks = new Map<string, readonly number[]>();
  const predict = async (request: AdmitRequestV1, pass: BrokerPassV1): Promise<Readonly<{ key: string; history: readonly UnitObservationV1[]; lines: number; predictedBytes: number; cancelled: boolean }>> => {
    const key = unitKeyV1(request, pass.profile), kloc = await sourceKlocV1(request.source), lines = Math.round(kloc * 1000), history = forecastHistoryV1(options.history.get(key) ?? [], lines), componentFinal = request.crateTypes.includes("cdylib") && request.target !== "host";
    if (componentFinal && !componentPeaks.has(pass.profile)) componentPeaks.set(pass.profile, componentFinalPeaksV1(options.history, pass.profile));
    const predicted = predictPeakBytesV1(history.map((row) => row.peakRssBytes), { kloc: history.length || componentFinal ? 0 : kloc, component: componentFinal ? componentPeaks.get(pass.profile)! : null });
    const records = await Promise.resolve().then(() => readCancels(now() - availablePolicy.cancelledMemoryMilliseconds)).catch(() => []), needle = `--crate-name ${request.crate} `, named = records.filter((record) => record.command.includes(needle));
    return { key, history, lines, cancelled: named.length > 0, predictedBytes: named.length ? Math.max(predicted, Math.ceil(Math.max(...named.map((record) => record.megabytes)) * 1048576 * availablePolicy.cancelledFactorBasisPoints / 10000)) : predicted };
  };
  const server = createServer((socket) => {
    let entry: Entry | undefined, finished = false, chain = Promise.resolve();
    socket.on("error", () => undefined);
    socket.once("close", () => { chain = chain.then(() => { if (!finished) forget(entry); }); });
    const lines = createInterface({ input: socket, crlfDelay: Infinity });
    lines.on("error", () => socket.destroy());
    lines.on("line", (line) => {
      chain = chain.then(async () => {
        if (finished) return;
        const message = JSON.parse(line) as { op?: string };
        if (message.op === "cached" && !entry) {
          const request = admitDefinition<CachedRequestV1>("CachedRequestV1", message), pass = passes.get(request.pass);
          if (!pass) throw Error(`Unknown build pass ${request.pass}`);
          const at = now(), recorded = request.provenance;
          completed.push(Object.freeze({ key: unitKeyV1(request, pass.profile), pass: request.pass, profile: pass.profile, target: request.target, crate: request.crate, package: request.package, version: request.version, crateTypes: request.crateTypes, primary: request.primary, artifact: unitArtifactV1(request), dependencies: request.externs.map((path) => { const extern = externArtifactsV1(path); return { artifacts: extern.candidates, edge: extern.edge }; }), buildScriptOutDir: request.buildScriptOutDir, predictedBytes: 0, requestMs: at, admittedMs: at, startMs: at, endMs: at, rmetaMs: recorded?.rmetaOffsetMs == null ? null : at, peakRssBytes: recorded?.peakRssBytes ?? 0, userMs: recorded?.userMs ?? 0, sysMs: recorded?.sysMs ?? 0, exit: 0, cache: Object.freeze({ key: request.cacheKey, recorded: recorded !== null }) }));
          finished = true;
          lastActivityMs = at;
          verdict(socket, true);
          socket.end();
          return;
        }
        if (message.op === "recall" && !entry) {
          const request = admitDefinition<RecallRequestV1>("RecallRequestV1", message), artifact = unitArtifactV1(request);
          const record = [...completed].reverse().find((row) => row.pass === request.pass && row.exit === 0 && !row.cache && row.artifact === artifact) ?? null;
          if (!socket.destroyed) socket.end(`${JSON.stringify({ v: 1, op: "unit", record })}\n`);
          finished = true;
          return;
        }
        if (message.op === "probe" && !entry) {
          const request = admitDefinition<AdmitRequestV1>("AdmitRequestV1", { ...message, op: "admit" }), pass = passes.get(request.pass);
          if (!pass) throw Error(`Unknown build pass ${request.pass}`);
          const { history, predictedBytes } = await predict(request, pass), wait = admissionWouldWaitV1(stateNow(admissionFrozenV1(frozen, { actualBytes: charged(), budgetBytes: budget(), swapOutBytesPerSecond })), waiting, { predictedBytes, expectedWallMs: expectedWallMsV1(history), remainingMs: history.at(-1)?.remainingMs ?? 0, arrival: arrivals });
          if (!socket.destroyed) socket.end(`{"v":1,"op":"headroom","wait":${wait}}\n`);
          finished = true;
          return;
        }
        if (message.op === "admit" && !entry) {
          const request = admitDefinition<AdmitRequestV1>("AdmitRequestV1", message), pass = passes.get(request.pass);
          if (!pass) throw Error(`Unknown build pass ${request.pass}`);
          const { key, history, lines, predictedBytes, cancelled } = await predict(request, pass);
          entry = { request, key, lines, profile: pass.profile, predictedBytes, expectedWallMs: expectedWallMsV1(history), remainingMs: history.at(-1)?.remainingMs ?? 0, arrival: arrivals++, requestMs: now(), admittedMs: 0, rssBytes: 0, chargedBytes: 0, cancelled, heldLoggedMs: -Infinity, socket };
          waiting.push(entry);
          lastActivityMs = entry.requestMs;
          schedule();
          if (waiting.includes(entry)) verdict(socket, false);
        } else if (message.op === "rss" && entry) {
          const sample = admitDefinition<{ bytes: number; chargedBytes: number }>("RssReportV1", message);
          entry.rssBytes = sample.bytes;
          entry.chargedBytes = sample.chargedBytes;
          schedule();
        } else if (message.op === "done" && entry && running.has(entry)) {
          const done = admitDefinition<DoneReportV1>("DoneReportV1", message), request = entry.request;
          finished = true;
          running.delete(entry);
          lastActivityMs = now();
          socket.end();
          completed.push(Object.freeze({ key: entry.key, pass: request.pass, profile: entry.profile, target: request.target, crate: request.crate, package: request.package, version: request.version, crateTypes: request.crateTypes, primary: request.primary, artifact: unitArtifactV1(request), dependencies: request.externs.map((path) => { const extern = externArtifactsV1(path); return { artifacts: extern.candidates, edge: extern.edge }; }), buildScriptOutDir: request.buildScriptOutDir, lines: entry.lines, predictedBytes: entry.predictedBytes, requestMs: entry.requestMs, admittedMs: entry.admittedMs, startMs: done.startMs, endMs: done.endMs, rmetaMs: done.rmetaMs, peakRssBytes: done.peakRssBytes, userMs: done.userMs, sysMs: done.sysMs, exit: done.exit }));
          try { options.onCompleted?.(completed[completed.length - 1]!); } catch (error) { console.error(`[admission] ${error instanceof Error ? error.message : String(error)}`); }
          schedule();
        } else throw Error(`Unexpected broker message ${line.slice(0, 200)}`);
      }).catch((error) => { console.error(`[admission] ${error instanceof Error ? error.message : String(error)}`); socket.destroy(); });
    });
  });
  if (process.platform !== "win32") rmSync(endpoint, { force: true });
  await new Promise<void>((accept, reject) => { server.once("error", reject); server.listen(endpoint, () => { server.off("error", reject); accept(); }); });
  return {
    endpoint,
    registerPass: (pass) => { passes.set(pass.id, pass); },
    observe: (sample) => {
      const at = now();
      nonRustc.push({ atMs: at, bytes: sample.nonRustcBytes });
      while (nonRustc.length && nonRustc[0]!.atMs < at - ADMISSION_POLICY_V1.nonRustcWindowMs) nonRustc.shift();
      swapOutBytesPerSecond = sample.swapOutBytesPerSecond;
      schedule();
    },
    stats: () => ({ running: running.size, waiting: waiting.length, frozen, budgetBytes: budget(), predictedRunningBytes: [...running].reduce((sum, entry) => sum + entry.predictedBytes, 0), actualRunningBytes: [...running].reduce((sum, entry) => sum + entry.rssBytes, 0), chargedRunningBytes: charged(), lastActivityMs }),
    units: () => completed,
    close: () => new Promise<void>((accept) => { closed = true; if (poll) clearTimeout(poll); server.close(() => { if (process.platform !== "win32") rmSync(endpoint, { force: true }); accept(); }); for (const entry of [...waiting, ...running]) entry.socket.destroy(); }),
  };
}

/** 🗃️ The compiler chain of a build whose environment names a unit cache (`SEMIO_UNIT_CACHE_ROOT` and the executable wrapper `SEMIO_UNIT_CACHE_WRAPPER`): Cargo runs the cache's wrapper as `RUSTC_WRAPPER` (`RUSTC` stays the real compiler, never the wrapper), which serves a unit from the cache or falls through to this driver's scheduler, and reports every served unit to the broker so the unit list, critical path and provenance stay complete. Empty when no cache is named. */
export function compilerWrapperEnvironmentV1(scheduler: string, laneRoot: string, environment: Readonly<Record<string, string | undefined>>): Record<string, string> {
  const root = environment.SEMIO_UNIT_CACHE_ROOT, wrapper = environment.SEMIO_UNIT_CACHE_WRAPPER;
  if (!root || !wrapper || environment.SEMIO_UNIT_CACHE === "off") return {};
  return unitCacheChainEnvironmentV1({ wrapper, cacheRoot: root, lane: "g", laneRoot, inner: [scheduler, environment.RUSTC ?? "rustc"], focus: [], slots: Number(environment.SEMIO_UNIT_CACHE_SLOTS ?? availableParallelism()), ...(environment.SEMIO_UNIT_CACHE_LIGHT_SLOTS ? { lightSlots: Number(environment.SEMIO_UNIT_CACHE_LIGHT_SLOTS) } : {}), environment, features: { buildScripts: true, brokerReplay: true }, ...(environment.SEMIO_UNIT_CACHE_TOOLCHAIN ? { toolchain: environment.SEMIO_UNIT_CACHE_TOOLCHAIN } : {}) });
}

/** 🪜️ The observation of one successful compiled unit, `null` for a failed or cached one: peak, measured output footprint, wall, CPU and remaining critical path, at the time the unit ended. */
export function unitObservationV1(unit: UnitRecordV1, remainingMs: number, outputBytes: number): UnitObservationV1 | null {
  return unit.exit === 0 && !unit.cache ? { v: 1, key: unit.key, peakRssBytes: unit.peakRssBytes, outputBytes, wallMs: Math.max(0, unit.endMs - unit.startMs), cpuMs: unit.userMs + unit.sysMs, remainingMs: Math.max(0, Math.round(remainingMs)), atMs: unit.endMs, ...unit.lines === undefined ? {} : { lines: unit.lines } } : null;
}

/** 🪜️ Recorded observations of a finished build: every successful unit's observation with its remaining critical path by unit artifact; they refine the rows recorded as the units completed (same key and time). */
export function unitObservationsV1(units: readonly UnitRecordV1[], remainingMs: ReadonlyMap<string, number>, outputBytes: ReadonlyMap<string, number>): UnitObservationV1[] {
  return units.filter((unit) => unit.exit === 0 && !unit.cache).map((unit) => { const bytes = outputBytes.get(unit.artifact); if (bytes === undefined) throw Error(`Unit output footprint was never measured: ${unit.key}`); return unitObservationV1(unit, remainingMs.get(unit.artifact) ?? 0, bytes)!; });
}

/** 🖨️ The bytes every output of one unit occupies: the files of its output directory named by its artifact stem (`<stem>.*` or `lib<stem>.*`). */
export function unitOutputBytesV1(artifact: string, entries: readonly string[], size: (name: string) => number): number {
  const stem = basename(artifact);
  return entries.filter((name) => name.startsWith(stem + ".") || name.startsWith("lib" + stem + ".")).reduce((sum, name) => sum + size(name), 0);
}

/** 🧰️ Declares the complete locked host selection for scheduler provisioning and offline planning. */
export function schedulerCargoSelectionV1(examples=false,mode:"prototype"|"release"="release"):Readonly<{manifest:string;args:readonly string[]}>{
 const manifest=fileURLToPath(new URL("./📦️packages/🦀️rust/Cargo.toml",import.meta.url));return Object.freeze({manifest,args:Object.freeze(["build","--locked",...(mode==="release"?["--release"]:["--profile",mode]),"--manifest-path",manifest,"--bins",...(examples?["--examples"]:[])])});
}

/** 🔖️ Returns only executables held by the current toolchain, environment and scheduler source receipt. */
export function readSchedulerExecutablesV1(targetDirectory:string,toolchain:string,environment:Readonly<Record<string,string|undefined>>):ReadonlyMap<string,string>|undefined{
 const {manifest}=schedulerCargoSelectionV1();return schedulerReceiptHoldsV1(readSchedulerReceiptV1(join(targetDirectory,schedulerReceiptFileV1())),schedulerIdentityV1(toolchain,environment),manifest);
}

/** 🛠️ Compiles the scheduler crate for its admitted host profile and returns its executables by name, examples included on request. */
export async function buildSchedulerV1(options: Readonly<{ cwd: string; targetDirectory: string; buildDirectory: string; environment: Readonly<Record<string, string | undefined>>; examples?: boolean; arguments?:readonly string[]; toolchain?: string; control: CompilerAdmissionControl; storage:NativeStorageReservationV1;sourceBoundary:CargoSourceBoundaryPolicyV1 }>): Promise<ReadonlyMap<string, string>> {
  requireNativeStorageReservationV1(options.storage,options.control);await options.storage.recheck();for(const path of [options.targetDirectory,options.buildDirectory,...["CARGO_HOME","TMPDIR","TMP","TEMP"].map(name=>{const path=options.environment[name];if(!path)throw Error("Bounded scheduler environment required: "+name);return path;})])requireNativeStoragePathV1(options.storage.backing.root,path);
  compilerAdmissionBudgetV1(options.control);const signal = options.control.signal;
  const selection=schedulerCargoSelectionV1(options.examples),manifest=selection.manifest,args=options.arguments??selection.args;
  const receiptFile=join(options.targetDirectory,schedulerReceiptFileV1()),identity=options.toolchain&&!options.examples?schedulerIdentityV1(options.toolchain,options.environment):undefined,held=identity?schedulerReceiptHoldsV1(readSchedulerReceiptV1(receiptFile),identity,manifest):undefined;if(held)return held;
  const graph=await captureCargoCompilationGraphV1({workspaceRoot:options.cwd,cwd:options.cwd,program:"cargo",prefix:[],args,environment:options.environment,maximumOutputBytes:cargoPlanningOutputBytesV1},options.control,actualCargoCompilerPlanningPortV1());await admitCargoSourceBoundariesV1(graph,options.cwd,options.sourceBoundary,options.control,actualCargoSourceAdmissionPortV1(options.environment,options.cwd,options.storage));await options.storage.recheck();
  const child = spawn(registeredToolExecutableV1("cargo"), [...args,"--message-format=json-render-diagnostics"], { cwd: options.cwd, env: { ...options.environment, CARGO_TARGET_DIR: options.targetDirectory, CARGO_BUILD_BUILD_DIR: options.buildDirectory, RUSTC_WRAPPER: "", CARGO_BUILD_RUSTC_WRAPPER: "" }, stdio: ["ignore", "pipe", "pipe"], detached: process.platform !== "win32" });
  const cancel = (): void => terminateOwnedChildTree(child);
  const stopDeadline = watchResourceLeaseControlDeadlineV1(options.control, () => cancel());
  signal.addEventListener("abort", cancel, { once: true });
  const executables = new Map<string, string>(), diagnostics: string[] = [];
  try {
    const [status] = await Promise.all([
      new Promise<number>((accept) => { child.once("error", () => accept(1)); child.once("close", (code) => accept(code ?? 1)); }),
      (async () => { for await (const line of createInterface({ input: child.stdout!, crlfDelay: Infinity })) { try { const message = JSON.parse(line); if (message.reason === "compiler-artifact" && typeof message.executable === "string") executables.set(message.target.name, message.executable); } catch { diagnostics.push(line); } } })(),
      (async () => { for await (const line of createInterface({ input: child.stderr!, crlfDelay: Infinity })) if (diagnostics.length < 200) diagnostics.push(line); })(),
    ]);
    checkResourceLeaseControl(options.control);
    if (status !== 0 || !executables.has("semio-build-scheduler")) throw Error(`Build scheduler compilation failed (${status}):\n${diagnostics.join("\n")}`);
    if (identity) { const receipt = captureSchedulerReceiptV1(identity, manifest, executables); if (receipt) writeSchedulerReceiptV1(receiptFile, receipt); }
    return executables;
  } finally { stopDeadline(); signal.removeEventListener("abort", cancel); }
}
