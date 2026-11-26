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
import { basename, dirname, extname, isAbsolute, join, parse, posix, relative, resolve, sep } from "node:path";
import { createInterface } from "node:readline";
import { fileURLToPath } from "node:url";
import { captureSchedulerReceiptV1, readSchedulerReceiptV1, schedulerIdentityV1, schedulerReceiptFileV1, schedulerReceiptHoldsV1, writeSchedulerReceiptV1 } from "./🔖️scheduler/🟦️.ts";
import { acquireQueuedResourceLease } from "../../../🔒️leases/🟦️.ts";
import { terminateOwnedChildTree } from "../../../🪓️termination/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { hostAvailableBytesV1 } from "../../../🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts";
import { readHostCapacityPlanV1, readLiveHostCapacityReservationV1 } from "../../../🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts";
import { hostFloorMegabytesV1, hostInteractiveReserveMegabytesV1, readHostCapacityPolicyV1 } from "../../../🏗️build/🌡️host-capacity/📏️decision/🟦️.ts";
import buildPolicy from "../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json" with {type:"json"};
import { rustTokens } from "../../../../📚️compiler/📖️syntax/🦀️rust/📦️packages/🟦️typescript/🟦️.ts";
import { readRustSourceClosureV1 } from "../📥️source/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import { carvingOfUnitV1, decideGrantGrowthV1, hostHoldableMegabytesV1, unitBudgetKindV1, unitBudgetMegabytesV1, unitKeyPartsV1, type CompileBudgetKindV1, type CompileBudgetPolicyV1, type GrowthHostV1 } from "./🏔️ceiling/🟦️.ts";

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

/** 🪆️ Projects compiler memory from the active parent ledger, independently of its carried tool envelope. */
export function parentCompilerMemoryAllowanceBytesV1(environment:Readonly<Record<string,string|undefined>>):number|undefined{
 const plan=readHostCapacityPlanV1(environment),claimed=environment.SEMIO_HOST_CAPACITY_GRANT;
 if(!plan)return claimed===undefined?undefined:0;
 if(claimed!==undefined&&claimed!==plan.id)return 0;
 const reservation=readLiveHostCapacityReservationV1(environment,readHostCapacityPolicyV1(buildPolicy.hostCapacity));
 const bytes=(reservation?.reservedMegabytes??0)*1048576;
 if(!Number.isSafeInteger(bytes))throw Error("Parent compiler memory reservation exceeds the byte accounting range");
 return bytes;
}

/** 📨️ The wrapper's admission request (`AdmitRequestV1`). */
export type AdmitRequestV1 = Readonly<{ v: 1; op: "admit"; pass: string; pid: number; crate: string; package: string; version: string; target: string; crateTypes: readonly string[]; outDir: string; extraFilename: string; source: string; manifestDir: string; primary: boolean; buildScriptOutDir: string | null; externs: readonly string[]; localInputKey?: string }>;

/** 🏁️ The wrapper's completion report (`DoneReportV1`). */
export type DoneReportV1 = Readonly<{ v: 1; op: "done"; exit: number; peakRssBytes: number; userMs: number; sysMs: number; startMs: number; endMs: number; rmetaMs: number | null }>;

/** 📦️ The measurements recorded when a cached unit was first built (`ProvenanceV1`). */
export type CachedProvenanceV1 = Readonly<{ peakRssBytes: number; userMs: number; sysMs: number; wallMs: number; rmetaOffsetMs: number | null }>;

/** 📣️ A wrapper's report that a unit was served from the unit cache instead of compiled (`CachedRequestV1`): the admission facts, the cache key and the recorded measurements. */
export type CachedRequestV1 = Readonly<Omit<AdmitRequestV1, "op"> & { op: "cached"; cacheKey: string; provenance: CachedProvenanceV1 | null }>;

/** 🔭️ A wrapper's question for the finished record of the unit its scheduler just reported (`RecallRequestV1`). */
export type RecallRequestV1 = Readonly<{ v: 1; op: "recall"; pass: string; crate: string; outDir: string; extraFilename: string; localInputKey?: string }>;

export type {CompileBudgetClassV1,CompileBudgetKindV1,CompileBudgetPolicyV1,CompileCarvingEntryV1,CompileCarvingFamilyV1,GrantGrowthVerdictV1,GrowthHostV1,RuntimeCeilingV1} from "./🏔️ceiling/🟦️.ts";
export {UNIT_CEILING_POLICY_V1,carvingOfUnitV1,compileUnitOfCommandV1,decideGrantGrowthV1,hostHoldableMegabytesV1,runtimeCeilingOfCommandV1,unitBudgetKindV1,unitBeyondHostV1,unitBudgetMegabytesV1,unitKeyPartsV1} from "./🏔️ceiling/🟦️.ts";

/** 🏔️ The compile-memory budget applied to recorded observations (`UnitCeilingVerdictV1`). */
export type UnitCeilingVerdictV1 = Readonly<{ version: 1; workerClass: string; libraryMegabytes: number; finalMegabytes: number; exceeded: readonly Readonly<{ key: string; crate: string; target: string; kind: CompileBudgetKindV1; peakMegabytes: number; budgetMegabytes: number; listed: boolean }>[]; resolved: readonly Readonly<{ crate: string; target: string; peakMegabytes: number; budgetMegabytes: number }>[]; ok: boolean }>;

/** 📒️ One recorded observation of a unit key (`ObservationV1`), the scheduling knowledge `units.jsonl` keeps. */
export type UnitObservationV1 = Readonly<{ v: 1; key: string; peakRssBytes: number; outputBytes: number; wallMs: number; cpuMs: number; remainingMs: number; atMs: number; lines?: number }>;

/** 🔗️ One dependency of a unit by the artifact identities it may name and what it consumes: metadata only (pipelined) or the complete library. */
export type UnitEdgeV1 = Readonly<{ artifacts: readonly string[]; edge: "rmeta" | "full" }>;

/** 🧾️ One admitted rustc unit as the broker measured it. */
export type UnitRecordV1 = Readonly<{ key: string; pass: string; profile: string; target: string; crate: string; package: string; version: string; crateTypes: readonly string[]; primary: boolean; artifact: string; dependencies: readonly UnitEdgeV1[]; buildScriptOutDir: string | null; lines?: number; predictedBytes: number; requestMs: number; admittedMs: number; startMs: number; endMs: number; rmetaMs: number | null; peakRssBytes: number; userMs: number; sysMs: number; exit: number; cache?: Readonly<{ key: string; recorded: boolean }> | null; localInputKey?: string }>;

/** 🧳️ A unit waiting for admission, as the ordering and decision laws see it. */
export type AdmissionWaiterV1 = Readonly<{ predictedBytes: number; expectedWallMs: number | null; remainingMs: number; arrival: number; serial?: boolean }>;

/** 🏃️ A running unit, as the decision law sees it. */
export type AdmissionRunningV1 = Readonly<{ predictedBytes: number; expectedEndMs: number | null }>;

/** 🧮️ The machine state one admission decision is taken in. */
export type AdmissionStateV1 = Readonly<{ budgetBytes: number; parentMemoryAllowanceBytes?: number; frozen: boolean; nowMs: number; running: readonly AdmissionRunningV1[] }>;

/** 🛂️ Validates one value against a named admission schema definition. */
function admitDefinition<T>(name: keyof typeof schema.definitions, value: unknown): T {
  const errors = validateJsonSchemaSubset(schema.definitions[name], value, schema);
  if (errors.length) throw Error(`Invalid ${name}: ${errors.join("; ")}`);
  return value as T;
}

const buildScriptCrateV1 = "build_script_build";

/** 🔑️ The scheduling identity of a unit, including the package version of Cargo's generically named build scripts. */
export function unitKeyV1(request: Pick<AdmitRequestV1, "package" | "crate" | "target" | "crateTypes"> & Partial<Pick<AdmitRequestV1, "version">>, profile: string): string {
  const owner = request.crate === buildScriptCrateV1 && request.version ? `${request.package}#${request.version}` : request.package;
  return `${owner}@${request.crate}|${request.target}|${profile}|${[...request.crateTypes].sort().join(",")}`;
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

/** 🧊️ The rows of the ledger that still describe the source: per unit key only rows that record the source size (`lines`) and lie within the policy tolerance of the latest such row, because a row without it, or one of a source that was split or has grown, measured another unit. */
export function freshUnitObservationsV1<T extends Pick<UnitObservationV1, "key" | "atMs" | "lines">>(observations: readonly T[]): T[] {
  const byKey = new Map<string, T[]>();
  for (const row of observations) if (row.lines !== undefined) byKey.set(row.key, [...byKey.get(row.key) ?? [], row]);
  return [...byKey.values()].flatMap((rows) => {
    const latest = rows.reduce((left, right) => right.atMs >= left.atMs ? right : left);
    return forecastHistoryV1(rows as unknown as UnitObservationV1[], latest.lines!) as unknown as T[];
  });
}

/** 🗜️ The peak a completed unit is recorded with: the larger of its peak resident bytes and its largest charged sample (`phys_footprint` on macOS). Under memory pressure the OS compresses and swaps a compiler's pages, so resident stays far below what the process is charged with and killed for: the cad component final of 2026-11-24 peaked at 2198 MB resident and 8036 MB charged.
 * @see ./🧪️tests/🗜️charged-peak/🟦️.ts */
export function unitPeakBytesV1(peakRssBytes: number, peakChargedBytes: number): number { return Math.max(peakRssBytes, peakChargedBytes); }

/** 🗻️ The compile-memory budget of the policy applied to ledger rows: the largest peak per unit key (rounded up to megabytes), the units above the budget of their kind (`listed` when on the carving list), the carving entries whose units fit again, and `ok` when no unlisted unit exceeds its budget and no entry is stale (the list may only shrink). Rows with a malformed key are ignored; order is peak descending, then key. */
export function unitCeilingVerdictV1(policy: CompileBudgetPolicyV1, observations: readonly Pick<UnitObservationV1, "key" | "peakRssBytes">[]): UnitCeilingVerdictV1 {
  const budget = policy.classes[policy.workerClass];
  if (!budget) throw Error(`Unknown worker class ${policy.workerClass}`);
  const peaks = new Map<string, number>(), crates = new Map<string, { peak: number; budget: number; fits: boolean }>();
  for (const row of observations) {
    const parts = unitKeyPartsV1(row.key), megabytes = Math.ceil(row.peakRssBytes / 1048576);
    if (!parts || !Number.isFinite(megabytes)) continue;
    peaks.set(row.key, Math.max(peaks.get(row.key) ?? 0, megabytes));
  }
  for (const [key, peak] of peaks) {
    const { crate, target } = unitKeyPartsV1(key)!, group = `${crate}|${target}`, own = unitBudgetMegabytesV1(policy, key), before = crates.get(group);
    crates.set(group, { peak: Math.max(before?.peak ?? 0, peak), budget: Math.max(before?.budget ?? 0, own), fits: (before?.fits ?? true) && peak <= own });
  }
  const listed = (crate: string, target: string) => carvingOfUnitV1(policy, { crate, target }) !== undefined;
  const exceeded = [...peaks].filter(([key, peak]) => peak > unitBudgetMegabytesV1(policy, key)).map(([key, peak]) => { const { crate, target } = unitKeyPartsV1(key)!; return { key, crate, target, kind: unitBudgetKindV1(key), peakMegabytes: peak, budgetMegabytes: unitBudgetMegabytesV1(policy, key), listed: listed(crate, target) }; })
    .sort((left, right) => right.peakMegabytes - left.peakMegabytes || (left.key < right.key ? -1 : left.key > right.key ? 1 : 0));
  const resolved = policy.carving.flatMap((entry) => { const group = crates.get(`${entry.crate}|${entry.target}`); return group !== undefined && group.fits ? [{ crate: entry.crate, target: entry.target, peakMegabytes: group.peak, budgetMegabytes: group.budget }] : []; })
    .sort((left, right) => right.peakMegabytes - left.peakMegabytes || (left.crate < right.crate ? -1 : left.crate > right.crate ? 1 : left.target < right.target ? -1 : left.target > right.target ? 1 : 0));
  return { version: 1, workerClass: policy.workerClass, libraryMegabytes: budget.libraryMegabytes, finalMegabytes: budget.finalMegabytes, exceeded, resolved, ok: exceeded.every((unit) => unit.listed) && !resolved.length };
}

const fillMessageV1 = (line: string, values: Readonly<Record<string, string | number>>) => line.replace(/\{([a-z]+)\}/gu, (match, name: string) => name in values ? String(values[name]) : match);

/** 🔔️ The lines that name what breaks the compile-memory budget (every unlisted unit above its budget, every stale carving entry) in `language`; empty when the verdict is `ok`. */
export function formatUnitCeilingV1(language: "en" | "de", verdict: UnitCeilingVerdictV1): string[] {
  const text = schema.definitions.Messages.const[language].unitCeiling;
  return [
    ...verdict.exceeded.filter((unit) => !unit.listed).map((unit) => fillMessageV1(text.exceeded, { key: unit.key, peak: unit.peakMegabytes, kind: text.kinds[unit.kind], budget: unit.budgetMegabytes, class: verdict.workerClass })),
    ...verdict.resolved.map((entry) => fillMessageV1(text.resolved, { crate: entry.crate, peak: entry.peakMegabytes, budget: entry.budgetMegabytes })),
  ];
}

/** 🛎️ The one bilingual line (English first, German second) a completed unit raises when its own peak is above its budget and it is not on the carving list, or `null`. */
export function unitCeilingAlarmV1(policy: CompileBudgetPolicyV1, row: Pick<UnitObservationV1, "key" | "peakRssBytes">): string | null {
  const verdict = unitCeilingVerdictV1(policy, [row]), unit = verdict.exceeded.find((one) => !one.listed);
  return unit ? (["en", "de"] as const).map((language) => formatUnitCeilingV1(language, { ...verdict, exceeded: [unit], resolved: [] })[0]!).join(" / ") : null;
}

/** 🥶️ The one bilingual line (English first, German second) a unit without fresh history raises when its cold estimate (`predictedBytes` of the cold prior) is above its budget and it is not on the carving list, or `null`: a warning, never a refusal. */
export function coldBudgetWarningV1(policy: CompileBudgetPolicyV1, key: string, predictedBytes: number): string | null {
  const parts = unitKeyPartsV1(key), budget = unitBudgetMegabytesV1(policy, key), peak = Math.ceil(predictedBytes / 1048576);
  if (!parts || peak <= budget || carvingOfUnitV1(policy, parts) !== undefined) return null;
  return (["en", "de"] as const).map((language) => { const text = schema.definitions.Messages.const[language].unitCeiling; return fillMessageV1(text.cold, { key, peak, kind: text.kinds[unitBudgetKindV1(key)], budget, class: policy.workerClass }); }).join(" / ");
}

/** 🔮️ Predicted peak resident bytes of a unit: `max(headroom × p95(last window peaks), floor)`; without history the cold-start model. A component final (`component` = recorded peaks of its kind) without history is predicted from that evidence, `max(headroom × quantile(peaks, componentColdQuantile), floor)`, or the `componentFinalBytes` bound while fewer than `componentColdMinimumRows` were recorded. */
export function predictPeakBytesV1(peaks: readonly number[], cold: Readonly<{ kloc: number; component: readonly number[] | null; external?: boolean; budgetBytes?: number }>): number {
  const policy = ADMISSION_POLICY_V1;
  if (peaks.length) return Math.max(Math.ceil(policy.headroom * quantileR7V1(peaks.slice(-policy.window), policy.quantile)), policy.floorBytes);
  if (cold.component === null) {
    if (cold.external) return Math.max(Math.min(Math.ceil(policy.coldExternalBaseBytes + policy.coldExternalBytesPerKloc * cold.kloc), cold.budgetBytes ?? Infinity), policy.floorBytes);
    return Math.max(Math.ceil(policy.coldBaseBytes + policy.coldBytesPerKloc * cold.kloc), policy.floorBytes);
  }
  return cold.component.length < policy.componentColdMinimumRows ? policy.componentFinalBytes : Math.max(Math.ceil(policy.headroom * quantileR7V1(cold.component, policy.componentColdQuantile)), policy.floorBytes);
}

/** 📦️ Whether a crate source lies outside the workspace (a registry, git checkout or vendor directory): its cold estimate follows the external class, since third-party crates peak far below the per-line growth the workspace's generic-heavy crates need (measured: `syn` 51 kloc 398 MB).
 * @see predictPeakBytesV1 */
export function externalSourceV1(source: string): boolean {
  const path = source.replaceAll("\\", "/");
  return ADMISSION_POLICY_V1.externalSourceMarkers.some((marker) => path.includes(marker));
}

/** 🧬️ The rows of a unit key's history that were recorded for a source of comparable size (`lines` within the policy tolerance of the current `lines`): the forecast follows the unit's inputs, so a crate that was split or grown is predicted from its size, not from a peak measured for other sources; rows without a recorded size are stale. */
export function forecastHistoryV1(history: readonly UnitObservationV1[], lines: number): UnitObservationV1[] {
  const bound = 1 + ADMISSION_POLICY_V1.forecastSizeToleranceBasisPoints / 10000;
  return history.filter((row) => row.lines !== undefined && (lines === 0 || row.lines === 0 ? row.lines === lines : Math.max(row.lines, lines) <= bound * Math.min(row.lines, lines)));
}

function localObservationKey(unit: string, input: string): string {
  return `${unit}|${input}`;
}

/** 🎯️ Adds local evidence only when the managed compiler inputs match exactly. */
export function admissionForecastHistoryV1(
  key: string, lines: number,
  history: ReadonlyMap<string, readonly UnitObservationV1[]>,
  localHistory?: ReadonlyMap<string, readonly UnitObservationV1[]>,
  localInputKey?: string,
): readonly UnitObservationV1[] {
  const known = forecastHistoryV1(history.get(key) ?? [], lines);
  if (known.length || !localInputKey) return known;
  return forecastHistoryV1(localHistory?.get(localObservationKey(key, localInputKey)) ?? [], lines);
}

/** 🧬️ The rows recorded for the same unit under another profile or other inputs (same owner, crate, target and crate types, a source of comparable size): the evidence a unit has when its exact key was never measured, e.g. a `check` of a crate that only built before. It replaces the cold size model and never the exact history. An `external` crate (registry, git or vendored source; its version fixes its source) also takes the rows that carry no recorded size, e.g. the fleet ledger rows of its other profile.
 * @see admissionForecastHistoryV1 */
export function admissionSiblingHistoryV1(
  key: string, lines: number,
  history: ReadonlyMap<string, readonly UnitObservationV1[]>,
  localHistory?: ReadonlyMap<string, readonly UnitObservationV1[]>,
  external = false,
): readonly UnitObservationV1[] {
  const own = key.split("|");
  if (own.length < 4) return [];
  const rows: UnitObservationV1[] = [];
  for (const source of localHistory ? [history, localHistory] : [history]) {
    for (const [other, observed] of source) {
      const parts = other.split("|");
      if (parts.length >= 4 && parts[0] === own[0] && parts[1] === own[1] && parts[3] === own[3]) rows.push(...observed);
    }
  }
  return external ? [...rows.filter((row) => row.lines === undefined), ...forecastHistoryV1(rows, lines)] : forecastHistoryV1(rows, lines);
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
 * left beside it. With nothing running one waiter can exceed the machine's soft share within the hard parent allowance. A frozen broker starts nothing
 * while anything runs. Returns indices into `ordered`. */
export function admitWaitersV1(state: AdmissionStateV1, ordered: readonly AdmissionWaiterV1[]): number[] {
  const admitted: number[] = [];
  const releases = state.running.map((row) => ({ bytes: row.predictedBytes, endMs: row.expectedEndMs ?? Infinity }));
  const parentAllowance=state.parentMemoryAllowanceBytes??Infinity;
  let available = Math.min(state.budgetBytes,parentAllowance) - releases.reduce((sum, row) => sum + row.bytes, 0), running = releases.length;
  let shadow: { atMs: number; extraBytes: number } | null = null;
  for (let index = 0; index < ordered.length; index++) {
    const waiter = ordered[index]!, endMs = waiter.expectedWallMs === null ? Infinity : state.nowMs + waiter.expectedWallMs;
    if (waiter.serial === true && running === 0) { admitted.push(index); running++; available -= waiter.predictedBytes; releases.push({ bytes: waiter.predictedBytes, endMs }); continue; }
    if (waiter.predictedBytes > parentAllowance) continue;
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
  const policy = ADMISSION_POLICY_V1;
  if (sample.actualBytes > sample.budgetBytes || admissionSwappingV1(sample.swapOutBytesPerSecond)) return true;
  return wasFrozen && sample.actualBytes >= policy.resumeFraction * sample.budgetBytes;
}

const admissionSwappingV1 = (rate: number | null | undefined): boolean => (rate ?? 0) > ADMISSION_POLICY_V1.swapOutBytesPerSecond;

/** 📈️ What the broker needs of the host to grow a parent grant: the memory facts of this machine now (`local`, `undefined` when unknown), those of the other machines of the fleet (`fleet`, `undefined` when unknown) and `acquire`, which books `extraMegabytes` more beside the grant in the fair queue of the host capacity gate, waits at most `waitMilliseconds`, ends at once when `signal` aborts and returns the booking to give back (`release`) or why there is none.
 * @see ./🏔️ceiling/🟦️.ts (`decideGrantGrowthV1`) */
export type GrantGrowthPortV1 = Readonly<{ local: () => GrowthHostV1 | undefined; fleet: () => readonly GrowthHostV1[] | undefined; acquire: (request: Readonly<{ unit: string; extraMegabytes: number; signal: AbortSignal; waitMilliseconds: number; write: (line: string) => void }>) => Promise<Readonly<{ granted: true; release: () => void }> | Readonly<{ granted: false; reason: "timeout" | "cancelled" }>> }>;

/** 🧱️ The most memory reservations can ever hold on a machine of `totalMegabytes` under the host-capacity policy of the build (its total less the free-memory floor and the interactive reserve, times the share reservations may take): a unit whose peak is above it never runs there, whatever else is idle.
 * @see ./🏔️ceiling/🟦️.ts (`hostHoldableMegabytesV1`) */
export function localHoldableMegabytesV1(totalMegabytes: number): number {
  const policy = readHostCapacityPolicyV1(buildPolicy.hostCapacity), host = { totalMegabytes };
  return hostHoldableMegabytesV1({ totalMegabytes, floorMegabytes: hostFloorMegabytesV1(policy, host), interactiveReserveMegabytes: hostInteractiveReserveMegabytesV1(policy, host), budgetBasisPoints: policy.memory.budgetBasisPoints });
}

/** 📣️ Describes one step of a grant growth (`Messages.growth`) in the given language. */
export function formatGrowthV1(language: "en" | "de", name: keyof typeof schema.definitions.Messages.const.en.growth, values: Readonly<Record<string, string | number>>): string {
  return schema.definitions.Messages.const[language].growth[name].replace(/\{([a-z]+)\}/gu, (match, key: string) => key in values ? String(values[key]) : match);
}

/** ⏱️ One unit that waits in the broker: for a grant growth (`growth`), in its queue behind running units (`queue`) or in its queue while nothing runs (`idle`, the stall); `place` and `count` are 0 for a growth.
 * @see ./🧬️schema/🔣️.json (`Policy.waits`) */
export type UnitWaitV1 = Readonly<{ unit: string; reason: "growth" | "queue" | "idle"; predictedMegabytes: number; seconds: number; place: number; count: number; running: number; heldMegabytes: number; budgetMegabytes: number; grantMegabytes: number }>;

/** 🔢️ The placeholder values of the texts of one waiting unit. */
export function waitValuesV1(row: UnitWaitV1): Readonly<Record<string, string | number>> {
  return { unit: row.unit, predicted: row.predictedMegabytes, seconds: row.seconds, place: row.place, count: row.count, running: row.running, held: row.heldMegabytes, budget: row.budgetMegabytes, grant: row.grantMegabytes };
}

/** 📣️ Describes one waiting unit (`Messages.waits`) in the given language. */
export function formatWaitV1(language: "en" | "de", name: keyof typeof schema.definitions.Messages.const.en.waits, values: Readonly<Record<string, string | number>>): string {
  return schema.definitions.Messages.const[language].waits[name].replace(/\{([a-z]+)\}/gu, (match, key: string) => key in values ? String(values[key]) : match);
}

/** 🌡️ The reason a compiler unit starts or waits under the host memory policy. */
export type AvailableAdmissionV1 = Readonly<{ admit: boolean; reason: "fits" | "available" | "warden" | "swap" | "waited-out" | "unknown" }>;

/** 🌡️ The inputs of `availableAdmissionV1`: `runningUnits` counts the units of this broker that run, `cancelled` marks a unit the host warden cancelled before (its wait bound doubles). */
export type AvailableAdmissionInputV1 = Readonly<{ predictedBytes: number; availableBytes: number | undefined; totalBytes: number; runningUnrealizedBytes: number; runningUnits: number; waitedMilliseconds: number; requirePhysicalRoom?: boolean; wardenLevel: "calm" | "guard" | "danger" | undefined; swapOutBytesPerSecond?: number | null; cancelled?: boolean; policy: Pick<typeof ADMISSION_POLICY_V1.available, "marginBasisPoints" | "waitMilliseconds"> }>;

/** 🌡️ Holds compilation at warden guard or danger and during swap growth. A managed final with a reserved parent also waits for measured physical room. Other work can start on unknown availability or after the idle wait bound with a calm warden.
 * @see ../../../🏗️build/🌡️host-capacity/🛡️warden/🔁️loop/🟦️.ts (`readHostWardenStateV1`) */
export function availableAdmissionV1(input: AvailableAdmissionInputV1): AvailableAdmissionV1 {
  if (input.wardenLevel === "guard" || input.wardenLevel === "danger") return { admit: false, reason: "warden" };
  if (admissionSwappingV1(input.swapOutBytesPerSecond)) return { admit: false, reason: "swap" };
  if (input.availableBytes === undefined) return input.requirePhysicalRoom ? { admit: false, reason: "available" } : { admit: true, reason: "unknown" };
  if (input.predictedBytes <= input.availableBytes - Math.floor(input.totalBytes * input.policy.marginBasisPoints / 10000) - input.runningUnrealizedBytes) return { admit: true, reason: "fits" };
  return !input.requirePhysicalRoom && input.wardenLevel === "calm" && input.runningUnits === 0 && input.waitedMilliseconds >= input.policy.waitMilliseconds * (input.cancelled ? 2 : 1) ? { admit: true, reason: "waited-out" } : { admit: false, reason: "available" };
}

/** 📣️ Describes why a compiler unit is waiting. */
export function formatAdmissionHeldV1(language: "en" | "de", kind: "available" | "warden" | "swap" | "cancelled", values: Readonly<Record<"unit" | "predicted" | "available" | "margin" | "level" | "seconds", string | number>>): string {
  return schema.definitions.Messages.const[language].held[kind].replace(/\{([a-z]+)\}/gu, (match, name: string) => name in values ? String(values[name as keyof typeof values]) : match);
}

/** 🗄️ The default telemetry store: `<cache>/build-telemetry/units.jsonl`, never cleared by fresh builds. */
export function unitHistoryPathV1(cacheRoot: string): string {
  return join(cacheRoot, ADMISSION_POLICY_V1.storeDirectory, ADMISSION_POLICY_V1.storeFile);
}

/** 🖥️ Native Cargo measurements follow the compiler host even for cross-target builds. */
export function localUnitHistoryPathV1(cacheRoot: string, platform = process.platform, architecture = process.arch): string {
  return join(cacheRoot, ADMISSION_POLICY_V1.storeDirectory, `units-${platform}-${architecture}.jsonl`);
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

const klocCache = new Map<string, Promise<number>>(), closureCache = new Map<string, Promise<number | undefined>>();

const sourceLinesV1 = (bytes: Uint8Array): number => {
  let lines = 0;
  for (const byte of bytes) if (byte === 10) lines++;
  return lines;
};

async function buildScriptLinesV1(source: string, manifestDirectory: string): Promise<number | undefined> {
  const root = resolve(manifestDirectory), lengths = new Map<string, number>();
  try {
    const inputs = await readRustSourceClosureV1([resolve(source)], root, [], async path => {
      const local = relative(root, path);
      if (isAbsolute(local) || local === ".." || local.startsWith(".." + sep)) throw Error("Build script input is outside its package");
      const bytes = await readFile(path).catch(() => null);
      if (bytes) {
        const tokens = rustTokens(bytes.toString("utf8"));
        if (tokens.some((token, index) => token.text === "[" && (tokens[index + 1]?.text === "cfg" || tokens[index + 1]?.text === "cfg_attr") &&
          (tokens[index - 1]?.text === "#" || tokens[index - 1]?.text === "!" && tokens[index - 2]?.text === "#"))) {
          throw Error("Build script input depends on compiler configuration");
        }
        lengths.set(local.replaceAll("\\", "/"), sourceLinesV1(bytes));
      }
      return bytes;
    }, { manifestDirectory: root, generated: () => { throw Error("Build script input requires compiler expansion"); } });
    if (inputs.some(input => input.kind !== "source")) return undefined;
    return inputs.reduce((total, input) => total + lengths.get(input.path)!, 0);
  } catch { return undefined; }
}

/** 📚️ The lines of the modules a library or binary root pulls in through `mod` declarations and `#[path]` mounts (disabled cfg branches excluded), `undefined` when the closure cannot be resolved without the compiler: the unit's own sources, never those of the packages that share its taxonomy folder.
 * @see buildScriptLinesV1 */
async function moduleClosureLinesV1(source: string): Promise<number | undefined> {
  const entry = resolve(source), root = parse(entry).root, lengths = new Map<string, number>();
  try {
    const inputs = await readRustSourceClosureV1([entry], root, [], async path => {
      const bytes = await readFile(path).catch(() => null);
      if (bytes) lengths.set(relative(root, path).replaceAll("\\", "/"), sourceLinesV1(bytes));
      return bytes;
    });
    return inputs.filter(input => input.kind === "source").reduce((total, input) => total + lengths.get(input.path)!, 0);
  } catch { return undefined; }
}

/** 📏️ Thousands of Rust source lines for the cold estimate. Build scripts use their source closure when it can be resolved without compiler cfg; every other unit uses its module closure, else the sources below its directory without the nested packages and skipped directories. */
export async function sourceKlocV1(source: string, buildScriptManifestDirectory?: string): Promise<number> {
  if (buildScriptManifestDirectory) {
    const lines = await buildScriptLinesV1(source, buildScriptManifestDirectory);
    if (lines !== undefined) return lines / 1000;
  }
  const entry = resolve(source);
  if (!closureCache.has(entry)) closureCache.set(entry, moduleClosureLinesV1(entry));
  const closure = await closureCache.get(entry);
  if (closure !== undefined) return closure / 1000;
  const root = dirname(resolve(source));
  let cached = klocCache.get(root);
  if (!cached) {
    const walk = async (directory: string, nested: boolean): Promise<number> => {
      let lines = 0;
      const entries = await readdir(directory, { withFileTypes: true }).catch(() => []);
      if (nested && entries.some((entry) => entry.isFile() && entry.name === ADMISSION_POLICY_V1.packageManifestFile)) return 0;
      for (const entry of entries) {
        if (entry.isDirectory() && !ADMISSION_POLICY_V1.sourceSkipDirectories.includes(entry.name)) lines += await walk(join(directory, entry.name), true);
        else if (entry.isFile() && entry.name.endsWith(ADMISSION_POLICY_V1.sourceExtension)) lines += sourceLinesV1(await readFile(join(directory, entry.name)));
      }
      return lines;
    };
    cached = walk(root, false).then((lines) => lines / 1000);
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
export type BrokerPassV1 = Readonly<{ id: string; profile: string; target: string; placement?: "fleet-only" }>;

/** 📡️ Live broker counters for the timeline and the stall watchdog. */
/** 🧮 Current compiler counts, memory budget, resident bytes and pressure charge. */
export type BrokerStatsV1 = Readonly<{ running: number; waiting: number; frozen: boolean; budgetBytes: number; predictedRunningBytes: number; actualRunningBytes: number; chargedRunningBytes: number; lastActivityMs: number }>;

/** 🔏️ The driver's admission broker. */
export interface AdmissionBrokerV1 {
  readonly endpoint: string;
  registerPass(pass: BrokerPassV1): void;
  placementFailed(passId: string): boolean;
  runningSchedulerPids(): readonly number[];
  observe(sample: Readonly<{ nonRustcBytes: number; swapOutBytesPerSecond: number | null }>): void;
  stats(): BrokerStatsV1;
  units(): readonly UnitRecordV1[];
  close(): Promise<void>;
}

type Entry = { serial?: boolean; grown?: Readonly<{ extraBytes: number; release: () => void }>; request: AdmitRequestV1; key: string; lines: number; profile: string; predictedBytes: number; expectedWallMs: number | null; remainingMs: number; arrival: number; requestMs: number; admittedMs: number; rssBytes: number; chargedBytes: number; peakChargedBytes: number; cancelled: boolean; heldLoggedMs: number; socket: Socket; forced?: boolean; pingedMs?: number };
type AdmissionCancellationV1 = Readonly<{ command: string; megabytes: number; reason?: string; buildScriptOutput?: string }>;

/** ▶️ Starts the admission broker: wrappers connect, ask for admission, stream resident memory and report completion.
 * The budget accounts for the inherited reservation and the rolling resident footprint of the driver's other children. */
export async function startAdmissionBrokerV1(options: Readonly<{ physicalBytes: number; reserveBytes?: number; parentMemoryAllowanceBytes?: number; growth?: GrantGrowthPortV1; history: ReadonlyMap<string, readonly UnitObservationV1[]>; endpoint?: string; now?: () => number; onCompleted?: (unit: UnitRecordV1) => void; localHistory?: ReadonlyMap<string, readonly UnitObservationV1[]>; onVerifiedLocal?: (unit: UnitRecordV1) => void; totalBytes?: number; available?: () => number | undefined; warden?: () => "calm" | "guard" | "danger" | undefined; cancels?: (sinceMs: number) => readonly AdmissionCancellationV1[] | Promise<readonly AdmissionCancellationV1[]>; log?: (line: string) => void; onWaits?: (rows: readonly UnitWaitV1[]) => void; waits?: Partial<typeof ADMISSION_POLICY_V1.waits>; bounds?: Partial<typeof ADMISSION_POLICY_V1.bounds>; growthPolicy?: Partial<typeof ADMISSION_POLICY_V1.growth> }>): Promise<AdmissionBrokerV1> {
  const physicalBytes=admissionMemoryBytesV1(options.physicalBytes,0),reserve=options.reserveBytes??admissionReserveBytesV1(physicalBytes);
  let parentAllowance=options.parentMemoryAllowanceBytes;
  if(!Number.isSafeInteger(reserve)||reserve<0||reserve>physicalBytes||parentAllowance!==undefined&&(!Number.isSafeInteger(parentAllowance)||parentAllowance<0))throw Error("Invalid compiler broker memory allowance");
  const now = options.now ?? Date.now, endpoint = options.endpoint ?? brokerEndpointV1(), passes = new Map<string, BrokerPassV1>(), bounds = { ...ADMISSION_POLICY_V1.bounds, ...options.bounds }, growthPolicy = { ...ADMISSION_POLICY_V1.growth, ...options.growthPolicy };
  const waiting: Entry[] = [], running = new Set<Entry>(), completed: UnitRecordV1[] = [], nonRustc: { atMs: number; bytes: number }[] = [];
  const verifiedLocal = new Set<UnitRecordV1>(), placementFailures=new Set<string>();
  const availablePolicy = ADMISSION_POLICY_V1.available, totalBytes = options.totalBytes ?? admissionMemoryBytesV1(), heldLogMs = Math.max(availablePolicy.pollMilliseconds, Math.floor(availablePolicy.waitMilliseconds / 6));
  const readAvailable = options.available ?? hostAvailableBytesV1, readLevel = options.warden ?? (() => wardenLoaded?.readHostWardenStateV1()?.level);
  const readCancels = options.cancels ?? (async (sinceMs: number) => (await loadWardenV1())?.readHostWardenCancelsSinceV1(sinceMs) ?? []);
  let poll: ReturnType<typeof setTimeout> | undefined, closed = false;
  void loadWardenV1().then((module) => { try { module?.ensureHostWardenV1(); } catch { return; } });
  let frozen = false, swapOutBytesPerSecond: number | null = null, arrivals = 0, lastActivityMs = now();
  const nonCompiler = (): number => Math.max(0, ...nonRustc.map((row) => row.bytes));
  const parentBudget = ():number|undefined => parentAllowance===undefined?undefined:Math.max(0,parentAllowance-nonCompiler());
  const budget = (): number => Math.max(0,Math.min(physicalBytes-reserve,parentAllowance??Infinity)-nonCompiler());
  const verdict = (socket: Socket, admitted: boolean): void => { if (!socket.destroyed) socket.write(`{"v":1,"op":"verdict","admitted":${admitted}}\n`); };
  const charged = (): number => [...running].reduce((sum, entry) => sum + entry.chargedBytes, 0);
  const stateNow = (isFrozen: boolean): AdmissionStateV1 => ({ budgetBytes: budget(), parentMemoryAllowanceBytes:parentBudget(), frozen: isFrozen, nowMs: now(), running: [...running].map((entry) => ({ predictedBytes: Math.max(entry.predictedBytes, entry.chargedBytes), expectedEndMs: entry.expectedWallMs === null ? null : entry.admittedMs + entry.expectedWallMs })) });
  const schedule = (): void => {
    frozen = admissionFrozenV1(frozen, { actualBytes: charged(), budgetBytes: budget(), swapOutBytesPerSecond });
    const ordered = admissionOrderV1(waiting), state = stateNow(frozen);
    const admitted = admitWaitersV1(state, ordered);
    const forced = running.size === 0 && !admitted.length ? ordered.findIndex((entry) => entry.forced) : -1;
    if (forced >= 0) admitted.push(forced);
    let held = false;
    if (admitted.length) {
      const safely = <T>(read: () => T): T | undefined => { try { return read(); } catch { return undefined; } };
      const availableBytes = safely(readAvailable), wardenLevel = safely(readLevel), margin = Math.floor(totalBytes * availablePolicy.marginBasisPoints / 10000);
      for (const index of admitted) {
        const entry = ordered[index]!, atMs = now(), unrealized = [...running].reduce((sum, row) => sum + Math.max(0, row.predictedBytes - row.chargedBytes), 0);
        const requirePhysicalRoom=parentAllowance!==undefined&&entry.request.target!=="host"&&entry.request.crateTypes.includes("cdylib");
        const decision = availableAdmissionV1({ requirePhysicalRoom, predictedBytes: entry.predictedBytes, availableBytes, totalBytes, runningUnrealizedBytes: unrealized, runningUnits: running.size, waitedMilliseconds: atMs - entry.requestMs, wardenLevel, swapOutBytesPerSecond, cancelled: entry.cancelled, policy: availablePolicy });
        if (!decision.admit) {
          held = true;
          if (options.log && atMs - entry.heldLoggedMs >= heldLogMs) {
            entry.heldLoggedMs = atMs;
            const megabytes = (bytes: number): number => Math.round(bytes / 1048576), kind = decision.reason === "warden" || decision.reason === "swap" ? decision.reason : entry.cancelled ? "cancelled" : "available";
            const values = { unit: entry.request.crate === buildScriptCrateV1 ? entry.key : entry.request.crate, predicted: megabytes(entry.predictedBytes), available: availableBytes === undefined ? "?" : megabytes(availableBytes), margin: megabytes(margin), level: wardenLevel ?? "unknown", seconds: Math.round((atMs - entry.requestMs) / 1000) };
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
  const giveBackGrowth = (entry: Entry): void => {
    const grown = entry.grown;
    if (!grown) return;
    entry.grown = undefined;
    parentAllowance = Math.max(0, (parentAllowance ?? 0) - grown.extraBytes);
    try { grown.release(); } catch { return; }
  };
  const forget = (entry: Entry | undefined): void => {
    if (!entry) return;
    const index = waiting.indexOf(entry);
    if (index >= 0) waiting.splice(index, 1);
    running.delete(entry);
    giveBackGrowth(entry);
    schedule();
  };
  const growing = new Set<Readonly<{ unit: string; predictedMegabytes: number; sinceMs: number }>>(), queuedLogged = new WeakMap<Entry, number>();
  let waitsPublished = false;
  const waitsTick = (): void => {
    const at = now(), mebi = 1048576, policy = { ...ADMISSION_POLICY_V1.waits, ...options.waits }, ordered = admissionOrderV1(waiting), runningCount = running.size;
    const held = Math.round([...running].reduce((sum, entry) => sum + Math.max(entry.predictedBytes, entry.chargedBytes), 0) / mebi), budgetMb = Math.round(budget() / mebi), grantMb = parentAllowance === undefined ? 0 : Math.floor(parentAllowance / mebi);
    const rows: UnitWaitV1[] = [...growing].map((grow) => ({ unit: grow.unit, reason: "growth", predictedMegabytes: grow.predictedMegabytes, seconds: Math.round((at - grow.sinceMs) / 1000), place: 0, count: 0, running: runningCount, heldMegabytes: held, budgetMegabytes: budgetMb, grantMegabytes: grantMb }));
    ordered.forEach((entry, index) => {
      const unit = entry.request.crate === buildScriptCrateV1 ? entry.key : entry.request.crate, row: UnitWaitV1 = { unit, reason: runningCount === 0 ? "idle" : "queue", predictedMegabytes: Math.ceil(entry.predictedBytes / mebi), seconds: Math.round((at - entry.requestMs) / 1000), place: index + 1, count: ordered.length, running: runningCount, heldMegabytes: held, budgetMegabytes: budgetMb, grantMegabytes: grantMb };
      rows.push(row);
      if (!options.log || at - Math.max(queuedLogged.get(entry) ?? entry.requestMs, entry.heldLoggedMs) < policy.progressMilliseconds) return;
      queuedLogged.set(entry, at);
      const name = row.reason === "idle" ? "idle" : "queued", values = waitValuesV1(row);
      try { options.log(`[admission] ${formatWaitV1("en", name, values)} / ${formatWaitV1("de", name, values)}`); } catch { return; }
    });
    if (!rows.length && !waitsPublished) return;
    waitsPublished = rows.length > 0;
    try { options.onWaits?.(rows); } catch { return; }
  };
  const rescuing = new Set<Entry>();
  const rescueStalled = (): void => {
    const first = admissionOrderV1(waiting)[0];
    if (!options.growth || running.size > 0 || parentAllowance === undefined || parentAllowance <= 0 || !first || rescuing.has(first) || now() - first.requestMs < (options.waits?.progressMilliseconds ?? ADMISSION_POLICY_V1.waits.progressMilliseconds)) return;
    const need = first.predictedBytes + nonCompiler();
    if (first.predictedBytes > parentAllowance || need <= parentAllowance || parentBudget()! >= first.predictedBytes) return;
    const control = new AbortController(), abort = (): void => control.abort();
    first.socket.once("close", abort);
    rescuing.add(first);
    void growParent(first.request.crate === buildScriptCrateV1 ? first.key : first.request.crate, need, control.signal, () => undefined, ADMISSION_POLICY_V1.growth.overheadWaitMilliseconds).then((outcome) => {
      if (outcome.kind !== "grown") {
        if (outcome.kind === "refused" && outcome.reason === "parent-memory" && waiting.includes(first) && !control.signal.aborted) { first.serial = true; first.predictedBytes = Math.min(first.predictedBytes, parentAllowance ?? first.predictedBytes); schedule(); }
        return;
      }
      if (!waiting.includes(first) || control.signal.aborted) { outcome.release(); return; }
      const before = first.grown;
      first.grown = { extraBytes: (before?.extraBytes ?? 0) + outcome.extraBytes, release: () => { before?.release(); outcome.release(); } };
      parentAllowance = (parentAllowance ?? 0) + outcome.extraBytes;
      schedule();
    }).catch(() => undefined).finally(() => { rescuing.delete(first); first.socket.off("close", abort); });
  };
  const watchdog = (): void => {
    const at = now(), say = (name: keyof typeof schema.definitions.Messages.const.en.waits, values: Readonly<Record<string, string | number>>): void => { try { options.log?.(`[admission] ${formatWaitV1("en", name, values)} / ${formatWaitV1("de", name, values)}`); } catch { return; } };
    const nameOf = (entry: Entry): string => entry.request.crate === buildScriptCrateV1 ? entry.key : entry.request.crate;
    let changed = false;
    for (const entry of [...waiting]) {
      const waitedMs = at - entry.requestMs;
      if (waitedMs >= bounds.queueMilliseconds) {
        waiting.splice(waiting.indexOf(entry), 1);
        giveBackGrowth(entry);
        lastActivityMs = at;
        changed = true;
        say("expired", { unit: nameOf(entry), seconds: Math.round(waitedMs / 1000), minutes: Math.ceil(bounds.queueMilliseconds / 60000) });
        const wire = admitDefinition("RefusalV1", { v: 1, op: "refused", reason: "parent-memory", predictedBytes: entry.predictedBytes, allowanceBytes: parentAllowance ?? 0 });
        if (!entry.socket.destroyed) entry.socket.end(JSON.stringify(wire) + "\n");
        continue;
      }
      if (at - (entry.pingedMs ?? entry.requestMs) >= bounds.heartbeatMilliseconds) { entry.pingedMs = at; verdict(entry.socket, false); }
    }
    const first = admissionOrderV1(waiting)[0];
    if (!first || running.size > 0 || first.forced || rescuing.has(first) || at - first.requestMs < bounds.stallMilliseconds) { if (changed) schedule(); return; }
    first.forced = true;
    say("stalled", { unit: nameOf(first), seconds: Math.round((at - first.requestMs) / 1000), predicted: Math.ceil(first.predictedBytes / 1048576), budget: Math.round(budget() / 1048576) });
    schedule();
  };
  const waitsTimer = setInterval(() => { waitsTick(); rescueStalled(); watchdog(); }, options.waits?.publishMilliseconds ?? ADMISSION_POLICY_V1.waits.publishMilliseconds);
  waitsTimer.unref?.();
  const componentPeaks = new Map<string, readonly number[]>();
  const coldWarned = new Set<string>();
  const predict = async (request: AdmitRequestV1, pass: BrokerPassV1): Promise<Readonly<{ key: string; history: readonly UnitObservationV1[]; lines: number; predictedBytes: number; cancelled: boolean; cold: boolean }>> => {
    const key = unitKeyV1(request, pass.profile), buildScript = request.crate === buildScriptCrateV1, kloc = await sourceKlocV1(request.source, buildScript ? request.manifestDir : undefined), lines = Math.round(kloc * 1000), componentFinal = request.crateTypes.includes("cdylib") && request.target !== "host";
    const history = admissionForecastHistoryV1(key, lines, options.history, options.localHistory, request.localInputKey);
    if (componentFinal && !componentPeaks.has(pass.profile)) componentPeaks.set(pass.profile, componentFinalPeaksV1(options.history, pass.profile));
    const sibling = history.length || componentFinal ? [] : admissionSiblingHistoryV1(key, lines, options.history, options.localHistory, externalSourceV1(request.source));
    const cold = !history.length && !sibling.length && !componentFinal;
    const predicted = predictPeakBytesV1((history.length ? history : sibling).map((row) => row.peakRssBytes), { kloc: cold ? kloc : 0, component: componentFinal ? componentPeaks.get(pass.profile)! : null, external: externalSourceV1(request.source), budgetBytes: cold ? unitBudgetMegabytesV1(ADMISSION_POLICY_V1.unitCeiling, key) * 1048576 : undefined });
    const coldWarning = history.length || sibling.length || componentFinal || coldWarned.has(key) ? null : coldBudgetWarningV1(ADMISSION_POLICY_V1.unitCeiling, key, predicted);
    if (coldWarning) { coldWarned.add(key); options.log?.(`[native-build] ${coldWarning}`); }
    const records = await Promise.resolve().then(() => readCancels(now() - availablePolicy.cancelledMemoryMilliseconds)).catch(() => []), needle = `--crate-name ${request.crate} `;
    const named = records.filter((record) => record.reason !== "ceiling" && (buildScript
      ? record.buildScriptOutput === request.outDir
      : record.command.includes(needle)));
    return { key, history, lines, cold, cancelled: named.length > 0, predictedBytes: named.length ? Math.max(predicted, Math.ceil(Math.max(...named.map((record) => record.megabytes)) * 1048576 * availablePolicy.cancelledFactorBasisPoints / 10000)) : predicted };
  };
  type GrowthOutcome = Readonly<{ kind: "grown"; extraBytes: number; release: () => void } | { kind: "refused"; reason: "beyond-host" | "no-host-fits" | "parent-memory"; allowanceBytes: number } | { kind: "cancelled" }>;
  const growParent = async (unit: string, predictedBytes: number, signal: AbortSignal, queued: () => void, waitMilliseconds: number = growthPolicy.waitMilliseconds): Promise<GrowthOutcome> => {
    const port = options.growth!, policy = growthPolicy, mebi = 1048576, allowance = parentAllowance!, held = (): GrowthOutcome => ({ kind: "refused", reason: "parent-memory", allowanceBytes: allowance });
    const safely = <T>(read: () => T): T | undefined => { try { return read(); } catch { return undefined; } };
    const say = (name: keyof typeof schema.definitions.Messages.const.en.growth, values: Readonly<Record<string, string | number>>): void => { try { options.log?.(`[admission] ${formatGrowthV1("en", name, values)} / ${formatGrowthV1("de", name, values)}`); } catch { return; } };
    const local = safely(port.local);
    if (!local) return held();
    const need = Math.ceil(predictedBytes / mebi), grant = Math.floor(allowance / mebi), base = { unit, predicted: need, grant };
    const verdict = decideGrantGrowthV1({ needMegabytes: need, grantMegabytes: grant, local, fleet: safely(port.fleet) });
    if (verdict.kind === "fits") return { kind: "grown", extraBytes: 0, release: () => undefined };
    if (verdict.kind === "beyond-host") { say("beyondHost", { ...base, holdable: verdict.holdableMegabytes, largest: verdict.largestMegabytes }); return { kind: "refused", reason: "beyond-host", allowanceBytes: verdict.holdableMegabytes * mebi }; }
    if (verdict.kind === "refused") { say("refused", { ...base, origin: verdict.largestOrigin, largest: verdict.largestMegabytes }); return { kind: "refused", reason: "no-host-fits", allowanceBytes: verdict.largestMegabytes * mebi }; }
    const extra = verdict.extraMegabytes, startedMs = now(), minutes = Math.ceil(waitMilliseconds / 60000), seconds = (): number => Math.round((now() - startedMs) / 1000);
    say("asking", { ...base, extra, minutes });
    queued();
    const ticker = setInterval(() => { const facts = safely(port.local); say("waiting", { ...base, extra, available: facts?.availableMegabytes ?? "?", reserve: facts ? facts.floorMegabytes + facts.interactiveReserveMegabytes : "?", seconds: seconds() }); queued(); }, policy.progressMilliseconds);
    ticker.unref?.();
    const growth = Object.freeze({ unit, predictedMegabytes: need, sinceMs: startedMs });
    growing.add(growth);
    const inner = new AbortController(), relay = (): void => inner.abort();
    signal.addEventListener("abort", relay, { once: true });
    let guard: ReturnType<typeof setTimeout> | undefined;
    try {
      const acquired = port.acquire({ unit, extraMegabytes: extra, signal: inner.signal, waitMilliseconds, write: (line) => { try { options.log?.(line); } catch { return; } } });
      const late = new Promise<Readonly<{ granted: false; reason: "timeout" }>>((accept) => { guard = setTimeout(() => { inner.abort(); accept({ granted: false, reason: "timeout" }); }, waitMilliseconds + bounds.growthGraceMilliseconds); guard.unref?.(); });
      void acquired.then((outcome) => { if (outcome.granted && inner.signal.aborted && !signal.aborted) outcome.release(); }, () => undefined);
      const answer = await Promise.race([acquired, late]);
      if (answer.granted) { say("grown", { ...base, granted: need, seconds: seconds() }); return { kind: "grown", extraBytes: extra * mebi, release: answer.release }; }
      if (answer.reason === "cancelled") { say("cancelled", { ...base, seconds: seconds() }); return { kind: "cancelled" }; }
      say("timeout", { ...base, minutes });
      return held();
    } finally { clearInterval(ticker); clearTimeout(guard); signal.removeEventListener("abort", relay); growing.delete(growth); }
  };
  const server = createServer((socket) => {
    let entry: Entry | undefined, finished = false, chain = Promise.resolve();
    const gone = new AbortController();
    socket.on("error", () => undefined);
    socket.once("close", () => gone.abort());
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
          if (record && request.localInputKey && record.localInputKey === request.localInputKey && !verifiedLocal.has(record)) {
            verifiedLocal.add(record);
            try { options.onVerifiedLocal?.(record); } catch (error) { console.error(`[admission] ${error instanceof Error ? error.message : String(error)}`); }
          }
          if (!socket.destroyed) socket.end(`${JSON.stringify({ v: 1, op: "unit", record })}\n`);
          finished = true;
          return;
        }
        if (message.op === "probe" && !entry) {
          const request = admitDefinition<AdmitRequestV1>("AdmitRequestV1", { ...message, op: "admit" }), pass = passes.get(request.pass);
          if (!pass) throw Error(`Unknown build pass ${request.pass}`);
          let wait = true;
          if(pass.placement!=="fleet-only"){
            const { history, predictedBytes } = await predict(request, pass);
            wait = admissionWouldWaitV1(stateNow(admissionFrozenV1(frozen, { actualBytes: charged(), budgetBytes: budget(), swapOutBytesPerSecond })), waiting, { predictedBytes, expectedWallMs: expectedWallMsV1(history), remainingMs: history.at(-1)?.remainingMs ?? 0, arrival: arrivals });
          }
          if (!socket.destroyed) socket.end(`{"v":1,"op":"headroom","wait":${wait}${pass.placement==="fleet-only"?",\"fleetOnly\":true":""}}\n`);
          finished = true;
          return;
        }
        if (message.op === "admit" && !entry) {
          const request = admitDefinition<AdmitRequestV1>("AdmitRequestV1", message), pass = passes.get(request.pass);
          if (!pass) throw Error(`Unknown build pass ${request.pass}`);
          const { key, history, lines, predictedBytes: predicted, cancelled, cold } = await new Promise<Awaited<ReturnType<typeof predict>>>((accept, reject) => {
            const timer = setTimeout(() => { try { options.log?.(`[admission] ${formatWaitV1("en", "late", { unit: request.crate, seconds: Math.round(bounds.predictMilliseconds / 1000) })} / ${formatWaitV1("de", "late", { unit: request.crate, seconds: Math.round(bounds.predictMilliseconds / 1000) })}`); } catch { void 0; } reject(Error(`Admission estimate of ${request.crate} exceeded ${bounds.predictMilliseconds} ms`)); }, bounds.predictMilliseconds);
            timer.unref?.();
            predict(request, pass).then(accept, reject).finally(() => clearTimeout(timer));
          });
          let predictedBytes = predicted;
          const overheadBytes = nonCompiler();
          let overheadGrowthTried = false, serial = false;
          const fleetOnly=pass.placement==="fleet-only";
          let grown: Entry["grown"], refusal: Readonly<{ reason: "parent-memory" | "fleet-placement" | "beyond-host" | "no-host-fits"; allowanceBytes: number }> | undefined;
          if (!fleetOnly&&options.growth&&parentAllowance!==undefined&&parentAllowance>0&&predictedBytes>parentAllowance) {
            const outcome = await growParent(request.crate === buildScriptCrateV1 ? key : request.crate, predictedBytes, gone.signal, () => verdict(socket, false));
            if (outcome.kind === "cancelled") { finished = true; socket.destroy(); return; }
            if (outcome.kind === "refused") refusal = outcome;
            else if (gone.signal.aborted) { outcome.release(); finished = true; return; }
            else { grown = { extraBytes: outcome.extraBytes, release: outcome.release }; parentAllowance = (parentAllowance ?? 0) + outcome.extraBytes; }
          }
          if (!fleetOnly && options.growth && refusal === undefined && grown === undefined && parentAllowance !== undefined && parentAllowance > 0 && predictedBytes <= parentAllowance && predictedBytes + nonCompiler() > parentAllowance) {
            overheadGrowthTried = true;
            const outcome = await growParent(request.crate === buildScriptCrateV1 ? key : request.crate, predictedBytes + nonCompiler(), gone.signal, () => verdict(socket, false), growthPolicy.overheadWaitMilliseconds);
            if (outcome.kind === "cancelled") { finished = true; socket.destroy(); return; }
            if (outcome.kind === "grown") {
              if (gone.signal.aborted) { outcome.release(); finished = true; return; }
              grown = { extraBytes: outcome.extraBytes, release: outcome.release };
              parentAllowance = (parentAllowance ?? 0) + outcome.extraBytes;
            }
          }
          if ((cold || overheadGrowthTried) && !fleetOnly && parentAllowance !== undefined && predictedBytes + overheadBytes > parentAllowance && (refusal === undefined || refusal.reason === "parent-memory")) {
            try { options.log?.(`[admission] ${formatGrowthV1("en", "serial", { unit: request.crate, predicted: Math.ceil(predictedBytes / 1048576), grant: Math.floor(parentAllowance / 1048576) })} / ${formatGrowthV1("de", "serial", { unit: request.crate, predicted: Math.ceil(predictedBytes / 1048576), grant: Math.floor(parentAllowance / 1048576) })}`); } catch { refusal = undefined; }
            predictedBytes = Math.min(predictedBytes, parentAllowance);
            serial = true;
            refusal = undefined;
          }
          const allowanceBytes=fleetOnly?0:refusal?.allowanceBytes??parentAllowance;
          if (allowanceBytes!==undefined&&(fleetOnly||refusal!==undefined||predictedBytes>allowanceBytes)) {
            finished = true;
            lastActivityMs = now();
            if(fleetOnly)placementFailures.add(pass.id);
            const wire=admitDefinition("RefusalV1",{v:1,op:"refused",reason:fleetOnly?"fleet-placement":refusal?.reason??"parent-memory",predictedBytes,allowanceBytes});
            if (!socket.destroyed) socket.end(JSON.stringify(wire)+"\n");
            return;
          }
          entry = { ...serial ? { serial } : {}, grown, request, key, lines, profile: pass.profile, predictedBytes, expectedWallMs: expectedWallMsV1(history), remainingMs: history.at(-1)?.remainingMs ?? 0, arrival: arrivals++, requestMs: now(), admittedMs: 0, rssBytes: 0, chargedBytes: 0, peakChargedBytes: 0, cancelled, heldLoggedMs: -Infinity, socket };
          waiting.push(entry);
          lastActivityMs = entry.requestMs;
          schedule();
          if (waiting.includes(entry)) verdict(socket, false);
        } else if (message.op === "rss" && entry) {
          const sample = admitDefinition<{ bytes: number; chargedBytes: number }>("RssReportV1", message);
          entry.rssBytes = sample.bytes;
          entry.chargedBytes = sample.chargedBytes;
          entry.peakChargedBytes = Math.max(entry.peakChargedBytes, sample.chargedBytes);
          schedule();
        } else if (message.op === "done" && entry && running.has(entry)) {
          const done = admitDefinition<DoneReportV1>("DoneReportV1", message), request = entry.request;
          finished = true;
          running.delete(entry);
          giveBackGrowth(entry);
          lastActivityMs = now();
          socket.end();
          completed.push(Object.freeze({ key: entry.key, pass: request.pass, profile: entry.profile, target: request.target, crate: request.crate, package: request.package, version: request.version, crateTypes: request.crateTypes, primary: request.primary, artifact: unitArtifactV1(request), dependencies: request.externs.map((path) => { const extern = externArtifactsV1(path); return { artifacts: extern.candidates, edge: extern.edge }; }), buildScriptOutDir: request.buildScriptOutDir, lines: entry.lines, ...request.localInputKey ? { localInputKey: request.localInputKey } : {}, predictedBytes: entry.predictedBytes, requestMs: entry.requestMs, admittedMs: entry.admittedMs, startMs: done.startMs, endMs: done.endMs, rmetaMs: done.rmetaMs, peakRssBytes: unitPeakBytesV1(done.peakRssBytes, entry.peakChargedBytes), userMs: done.userMs, sysMs: done.sysMs, exit: done.exit }));
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
    registerPass: (pass) => { passes.set(pass.id, pass); placementFailures.delete(pass.id); },
    placementFailed: (passId) => placementFailures.has(passId),
    runningSchedulerPids: () => [...running].map(entry=>entry.request.pid),
    observe: (sample) => {
      const at = now();
      nonRustc.push({ atMs: at, bytes: sample.nonRustcBytes });
      while (nonRustc.length && nonRustc[0]!.atMs < at - ADMISSION_POLICY_V1.nonRustcWindowMs) nonRustc.shift();
      swapOutBytesPerSecond = sample.swapOutBytesPerSecond;
      schedule();
    },
    stats: () => ({ running: running.size, waiting: waiting.length, frozen, budgetBytes: budget(), predictedRunningBytes: [...running].reduce((sum, entry) => sum + entry.predictedBytes, 0), actualRunningBytes: [...running].reduce((sum, entry) => sum + entry.rssBytes, 0), chargedRunningBytes: charged(), lastActivityMs }),
    units: () => completed,
    close: () => new Promise<void>((accept) => { closed = true; if (poll) clearTimeout(poll); clearInterval(waitsTimer); if (waitsPublished) { waitsPublished = false; try { options.onWaits?.([]); } catch { void 0; } } server.close(() => { if (process.platform !== "win32") rmSync(endpoint, { force: true }); accept(); }); for (const entry of [...waiting, ...running]) entry.socket.destroy(); }),
  };
}

/** 🗃️ The compiler chain of a build whose environment names a unit cache (`SEMIO_UNIT_CACHE_ROOT` and the executable wrapper `SEMIO_UNIT_CACHE_WRAPPER`): Cargo runs the cache's wrapper as `RUSTC_WRAPPER` (`RUSTC` stays the real compiler, never the wrapper), which serves a unit from the cache or falls through to this driver's scheduler, and reports every served unit to the broker so the unit list, critical path and provenance stay complete. Empty when no cache is named. */
export function compilerWrapperEnvironmentV1(scheduler: string, laneRoot: string, environment: Readonly<Record<string, string | undefined>>): Record<string, string> {
  const root = environment.SEMIO_UNIT_CACHE_ROOT, wrapper = environment.SEMIO_UNIT_CACHE_WRAPPER;
  if (!root || !wrapper || environment.SEMIO_UNIT_CACHE === "off") return {};
  return { SEMIO_UNIT_CACHE_SERIAL_LOCAL_FINAL: "1", ...unitCacheChainEnvironmentV1({ wrapper, cacheRoot: root, lane: "g", laneRoot, inner: [scheduler, environment.RUSTC ?? "rustc"], focus: [], slots: Number(environment.SEMIO_UNIT_CACHE_SLOTS ?? availableParallelism()), ...(environment.SEMIO_UNIT_CACHE_LIGHT_SLOTS ? { lightSlots: Number(environment.SEMIO_UNIT_CACHE_LIGHT_SLOTS) } : {}), environment, features: { buildScripts: true, brokerReplay: true }, ...(environment.SEMIO_UNIT_CACHE_TOOLCHAIN ? { toolchain: environment.SEMIO_UNIT_CACHE_TOOLCHAIN } : {}) }) };
}

/** 🪜️ The observation of one successful compiled unit, `null` for a failed or cached one: peak, measured output footprint, wall, CPU and remaining critical path, at the time the unit ended. */
export function unitObservationV1(unit: UnitRecordV1, remainingMs: number, outputBytes: number): UnitObservationV1 | null {
  return unit.exit === 0 && !unit.cache ? { v: 1, key: unit.key, peakRssBytes: unit.peakRssBytes, outputBytes, wallMs: Math.max(0, unit.endMs - unit.startMs), cpuMs: unit.userMs + unit.sysMs, remainingMs: Math.max(0, Math.round(remainingMs)), atMs: unit.endMs, ...unit.lines === undefined ? {} : { lines: unit.lines } } : null;
}

/** 📈️ Only a successful local compiler completion supplies a forecast measurement. */
export function localUnitObservationV1(unit: UnitRecordV1, outputBytes: (artifact: string) => number): UnitObservationV1 | null {
  if (unit.cache || unit.exit !== 0 || !unit.peakRssBytes || unit.lines === undefined || !unit.localInputKey) return null;
  return unitObservationV1({ ...unit, key: localObservationKey(unit.key, unit.localInputKey) }, Math.max(0, unit.endMs - unit.startMs), outputBytes(unit.artifact));
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
  const child = spawn(registeredToolExecutableV1("cargo"), [...args,"--message-format=json-render-diagnostics"], {windowsHide:true, cwd: options.cwd, env: { ...options.environment, CARGO_TARGET_DIR: options.targetDirectory, CARGO_BUILD_BUILD_DIR: options.buildDirectory, RUSTC_WRAPPER: "", CARGO_BUILD_RUSTC_WRAPPER: "" }, stdio: ["ignore", "pipe", "pipe"], detached: process.platform !== "win32" });
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
