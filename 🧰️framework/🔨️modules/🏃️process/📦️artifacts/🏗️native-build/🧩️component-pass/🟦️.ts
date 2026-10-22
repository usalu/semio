import {captureOwnedCommandResultV1} from "../../../🎛️owned-execution/📥️capture/🟦️.ts";
import { ensureCargoLockCurrentV1 } from "../../../🦀️cargo-workspace/🟦️.ts";
import{withCompilerCapacityConsumptionV1}from"../../../🏗️build/🎟️capacity/🪆️scope/🤲️consumption/🟦️.ts";
import{withNativeBuildStageCapacityV1,readNativeBuildStageCapacityV1,type NativeBuildStageCapacityV1}from"./🎟️stage/🟦️.ts";
import{admitNativeBuildPipelineV1,admitNativeBuildPlanV1,type NativeBuildPackageV1,type NativeBuildPassV1,type NativeBuildJobV1,type NativeBuildPipelineJobV1,type NativeBuildPipelineV1,type NativeBuildPlanV1,type NativeBuildGenerationKindV1,type NativeBuildGenerationV1}from"./📋️contract/🟦️.ts";
import {CargoGraphOperationsV1} from "./📥️planning/🟦️.ts";
import {nativeBuildPrerequisiteOrderV1} from "./📋️prerequisites/🟦️.ts";
import {CargoInputContinuationTransactionV1} from "../📥️inputs/🤲️continuations/🟦️.ts";
import type {CompletedCargoCompilationViewV1} from "../🧾️receipt/📦️retention/📥️snapshot/🟦️.ts";
import compilationSourceSchema from "../../📤️publication/📥️sources/🧬️schema/🔣️.json" with { type: "json" };
import{withCargoCompilerInputsV1,type CargoCompilerInputsPortV1,type CargoCompilerInputsRequestV1,type CargoCompilerInputsViewV1}from"../📥️inputs/🟦️.ts";
import {withIssuedResourceOperationV1,withRequestedIssuedResourceControlV1,requireIssuedResourceControlV1,requireIssuedResourceControlRefinementV1,scriptResourceProcessEnvironmentV1} from "../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {requireNativeStorageReservationV1,type NativeStorageReservationV1} from "../💾️storage/🟦️.ts";
import {requireNativeStoragePathV1} from "../💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {admitCargoSelectionV1,type CargoSourceBoundaryPolicyV1} from "../📥️source/🎟️admission/🟦️.ts";
import { checkResourceLeaseControl, requireMaterializedPathV1 } from "../../../🔒️leases/🟦️.ts";
import type { BuildMode } from "../../../🏗️build/🎚️mode/🟦️.ts";
import { compilerAdmissionBudgetV1, type CompilerAdmissionControl } from "../../../🏗️build/🎟️capacity/🟦️.ts";
import { execFile, spawn, type ChildProcess } from "node:child_process";
import { createHash } from "node:crypto";
import { closeSync, existsSync, mkdirSync, mkdtempSync, openSync, readdirSync, readSync, rmSync, statSync, writeFileSync, writeSync } from "node:fs";
import {nativeStatfsSyncV1} from "../💾️storage/🌐️platform/📋️contract/🟦️.ts";
import { homedir } from "node:os";
import { basename, dirname, join, relative, resolve, sep } from "node:path";
import { createInterface } from "node:readline";
import { fileURLToPath } from "node:url";
import { promisify } from "node:util";
import { validateJsonSchemaSubset } from "../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { terminateOwnedChildTree } from "../../../🪓️termination/🟦️.ts";
import { stageArtifacts } from "../../📤️publication/🟦️.ts";
import { acquireCargoBuildLeaseV1 } from "../🔒️lease/🟦️.ts";
import { CargoProvenanceSessionV1, cargoConfigurationArgumentsV1, cargoProvenancePhysicalControlV1, cargoProvenanceProgressSinkV1, sharedProvenanceCustodyV1, type CargoProvenanceStagedV1 } from "../🟦️.ts";
import { cargoBuildRunLeafV1, cargoProvenanceMerkleRootV1, cargoProvenanceRowsDigestV1, cargoProvenanceSourceV1, cargoUnitIdentityV1, cargoUnitLeafV1, type CargoProvenanceRootsV1, type CargoUnitObservationV1 } from "../🧾️receipt/🟦️.ts";
import { ADMISSION_POLICY_V1, admissionMemoryBytesV1, buildSchedulerV1, compilerWrapperEnvironmentV1, quantileR7V1, readUnitHistoryV1, recordUnitHistoryV1, startAdmissionBrokerV1, unitHistoryPathV1, unitKeyV1, unitObservationsV1, unitOutputBytesV1, type UnitObservationV1 } from "../🎟️admission/🟦️.ts";
import { createJobserverV1, defaultJobserverStyleV1, type JobSlotV1 } from "../🎰️jobserver/🟦️.ts";
import { criticalPathV1, startTimelineV1, TELEMETRY_POLICY_V1, writeBuildReportV1, type BuildScriptRunV1, type StageRecordV1 } from "../📈️telemetry/🟦️.ts";
import { STAGE_CACHE_POLICY_V1, stageCacheV1, type StageCacheV1 } from "../🗃️stage-cache/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

/** 🎚️ Maps only admitted compiler profiles to their exact selected build mode. */
export function profileBuildModeV1(value: unknown): BuildMode {
 if(value==="prototype")return "prototype";
 if(value==="release")return "release";
 throw Error("Invalid compiler build profile");
}

/** 🧩️ Driver policy: stall watchdog, output bounds, feature unification, generation layout and report file names. */
export const NATIVE_BUILD_DRIVER_POLICY_V1 = schema.definitions.Policy.const;





/** 🎟️ Jobserver clients borrow shared tokens; other compilers consume only slots reserved for their process. */
export function nativeBuildJobWorkerBudgetV1(protocol: NativeBuildJobV1["workerProtocol"], tokens: number, reserve: (maximum: number) => number): number {
  return protocol === "jobserver" ? tokens : reserve(tokens);
}






/** 🌳️ One linked component's provenance: the Merkle root over the leaves of exactly the units its unit graph closure holds, and the digest of the repository sources those units read. */
export type ComponentProvenanceV1 = Readonly<{ root: string; leaves: readonly string[]; sourceTreeDigest: string; units: number }>;

/** 🔗️ A component Cargo linked, as its post-link stages see it. */
export type LinkedComponentV1 = Readonly<{ pass: NativeBuildPassV1; package: NativeBuildPackageV1; wasm: string; staged: string }>;

/** 🧰️ Carries immutable actually held workers and the genuine accepted stage control. */
export type NativeBuildStageContextV1 = Readonly<{ control: CompilerAdmissionControl; signal: AbortSignal; parallelism: number; generation: NativeBuildGenerationV1; cache: StageCacheV1; environment: Readonly<Record<string, string | undefined>>; log: (line: string) => void; provenance: () => Promise<ComponentProvenanceV1> }>;
/** 🪜️ Declares one closed scheduling intent; cleanup is only the existing cheap finalizer. */
export type NativeBuildStageV1 = Readonly<{ name: string; after: readonly string[]; capacity: NativeBuildStageCapacityV1; run: (context: NativeBuildStageContextV1) => Promise<void> }>;

/** 🌡️ The generation a caller's environment selects (`SEMIO_NATIVE_BUILD_GENERATION`), warm by default. */
export function nativeBuildGenerationKindV1(environment: Readonly<Record<string, string | undefined>>): NativeBuildGenerationKindV1 {
  const value = environment[NATIVE_BUILD_DRIVER_POLICY_V1.generationEnvironment] ?? "warm";
  if (!(NATIVE_BUILD_DRIVER_POLICY_V1.generations as readonly string[]).includes(value)) throw Error(`Invalid ${NATIVE_BUILD_DRIVER_POLICY_V1.generationEnvironment}: ${value}`);
  return value as NativeBuildGenerationKindV1;
}

/** 🗂️ Lays out one generation: fresh is a new `<cache>/f…` directory with one-letter stores (keeping rustc output paths inside the 256-unit path budget) and its own
 * stage cache; warm compiles into the shared stores and caches stages under `<cache>/stages`, while its report, logs, jobserver and stage work live in a new
 * `<cache>/runs/w…` run directory, so concurrent warm drivers (two dev launches) never share one. Retirement requires original reader and writer custody. */
export function nativeBuildGenerationV1(kind: NativeBuildGenerationKindV1, cacheRoot: string, shared: Readonly<{ target: string; build: string }>): NativeBuildGenerationV1 {
  cacheRoot = requireMaterializedPathV1(cacheRoot);
  shared = { target: requireMaterializedPathV1(shared.target), build: requireMaterializedPathV1(shared.build) };
  const policy = NATIVE_BUILD_DRIVER_POLICY_V1, runs = join(cacheRoot, policy.warmRoot);
  const planned = nativeGenerationLayoutV1(kind, join(kind === "fresh" ? cacheRoot : runs, (kind === "fresh" ? policy.generationPrefix : policy.warmRunPrefix) + "x".repeat(policy.randomSuffixUnits)), cacheRoot, shared);
  admitGenerationPathsV1(planned);
  mkdirSync(kind === "fresh" ? cacheRoot : runs, { recursive: true });
  const root = kind === "fresh" ? mkdtempSync(join(cacheRoot, NATIVE_BUILD_DRIVER_POLICY_V1.generationPrefix)) : mkdtempSync(join(runs, NATIVE_BUILD_DRIVER_POLICY_V1.warmRunPrefix));
  const generation = nativeGenerationLayoutV1(kind, root, cacheRoot, shared);
  for (const directory of [generation.report, generation.logs, generation.jobserver]) { rmSync(directory, { recursive: true, force: true }); mkdirSync(directory, { recursive: true }); }
  return generation;
}

/** 🗺️ Resolves the complete generation layout without touching the filesystem. */
function nativeGenerationLayoutV1(kind: NativeBuildGenerationKindV1, root: string, cacheRoot: string, shared: Readonly<{ target: string; build: string }>): NativeBuildGenerationV1 {
  const stores = NATIVE_BUILD_DRIVER_POLICY_V1.stores;
  return { kind, root: resolve(root), target: resolve(kind === "fresh" ? join(root, stores.target) : shared.target), build: resolve(kind === "fresh" ? join(root, stores.build) : shared.build), report: resolve(root, stores.report), logs: resolve(root, stores.logs), jobserver: resolve(root, stores.jobserver), stages: kind === "fresh" ? resolve(root, STAGE_CACHE_POLICY_V1.freshStore) : resolve(cacheRoot, STAGE_CACHE_POLICY_V1.warmRoot) };
}

/** 📏️ Admits every fixed generation root and report leaf before materialization. */
function admitGenerationPathsV1(generation: NativeBuildGenerationV1): void {
  const policy = NATIVE_BUILD_DRIVER_POLICY_V1;
  const receipt = `cargo-unit-provenance-release-${"0".repeat(32)}.json`, temporary = `.${"x".repeat(36)}.tmp`;
  for (const path of [generation.root, generation.target, generation.build, generation.report, generation.logs, generation.jobserver, generation.stages, join(generation.jobserver, "jobserver.fifo"), join(generation.build, "semio-compiler-resources"), ...["", receipt, temporary].map(file => join(generation.build, policy.provenanceLedger, file)), ...[policy.provenanceLeavesFile, policy.provenanceReadsFile, receipt, temporary].map(file => join(generation.report, policy.provenanceDirectory, file)), ...[TELEMETRY_POLICY_V1.timelineFile, TELEMETRY_POLICY_V1.unitsFile, TELEMETRY_POLICY_V1.reportFile].map(file => join(generation.report, file))]) requireMaterializedPathV1(path);
}

/** 🏁️ The planned component whose linked `.wasm` a Cargo JSON message announces, with that file. */
export function componentArtifactV1(message: { reason?: string; package_id?: string; manifest_path?: string; target?: { kind?: string[] }; filenames?: string[] }, pass: NativeBuildPassV1, targetDirectory: string): Readonly<{ package: NativeBuildPackageV1; wasm: string }> | null {
  if (message.reason !== "compiler-artifact" || !message.target?.kind?.includes("cdylib")) return null;
  const wasm = message.filenames?.filter((file) => file.endsWith(".wasm")).sort((left, right) => Number(right.startsWith(targetDirectory)) - Number(left.startsWith(targetDirectory)))[0];
  const manifest = message.manifest_path ? resolve(message.manifest_path) : undefined, row = pass.packages.find((candidate) => resolve(candidate.manifest) === manifest);
  return wasm && row ? { package: row, wasm } : null;
}

/** 🧾️ The arguments of one pass: every planned package's library, one target, one profile, all failures reported. */
export function nativeBuildPassArgsV1(pass: NativeBuildPassV1): string[] {
  return ["build", "--locked", "--manifest-path", pass.manifest, ...pass.packages.flatMap((row) => ["-p", row.name]), "--lib", "--target", pass.target, "--profile", pass.profile, "--keep-going", "--message-format=json-render-diagnostics"];
}

/** 📁️ Names finite generation-owned component bytes without granting filesystem custody. */
export function nativeBuildComponentStagePathV1(generation:Pick<NativeBuildGenerationV1,"root">,pass:Pick<NativeBuildPassV1,"id"|"manifest"|"profile">,row:NativeBuildPackageV1):string{const key=createHash("sha256").update(JSON.stringify([pass.id,pass.manifest,pass.profile,row.name,row.manifest])).digest("hex").slice(0,24);return requireMaterializedPathV1(join(generation.root,"w",key,"component.wasm"));}

/** 🧾️ Names the observation of an actual admitted pass from its exact configured command arguments. */
export function nativeBuildPassObservationPathV1(generation:Pick<NativeBuildGenerationV1,"report"|"build">,pass:Pick<NativeBuildPassV1,"manifest">,arguments_:readonly string[]):string{
 const key=createHash("sha256").update(JSON.stringify([pass.manifest,arguments_,generation.build])).digest("hex").slice(0,32);return requireMaterializedPathV1(join(generation.report,NATIVE_BUILD_DRIVER_POLICY_V1.provenanceDirectory,`cargo-unit-provenance-release-${key}.json`));
}

/** 🕸️ The arguments that print one pass's unit graph without compiling anything. */
export function nativeBuildUnitGraphArgsV1(pass: NativeBuildPassV1): string[] {
  return ["build", "--locked", "--manifest-path", pass.manifest, ...pass.packages.flatMap((row) => ["-p", row.name]), "--lib", "--target", pass.target, "--profile", pass.profile, "-Z", "unstable-options", "--unit-graph"];
}

/** 🕸️ Cargo's unit graph (`--unit-graph`, version 1) as the driver reads it. */
export type CargoUnitGraphV1 = Readonly<{ version: 1; units: readonly Readonly<{ pkg_id: string; target: Readonly<{ name: string; kind: readonly string[]; crate_types: readonly string[]; src_path: string }>; platform: string | null; mode: string; features: readonly string[]; dependencies: readonly Readonly<{ index: number }>[] }>[]; roots: readonly number[] }>;

/** 🔑️ The identity of one unit-graph node, joinable with {@link announcedUnitIdentityV1}. */
export function unitGraphIdentityV1(unit: CargoUnitGraphV1["units"][number]): string {
  return unit.mode === "run-custom-build" ? JSON.stringify([unit.pkg_id, "run-custom-build", unit.platform]) : cargoUnitIdentityV1({ packageId: unit.pkg_id, target: unit.target.name, kinds: unit.target.kind, platform: unit.platform, features: unit.features });
}

/** 🔑️ The identity of a unit Cargo announced in a pass for `target`: the platform is the target when its outputs live under the target's directory. */
export function announcedUnitIdentityV1(message: Readonly<{ package_id?: unknown; target?: Readonly<{ name?: unknown; kind?: unknown }>; features?: unknown; filenames?: unknown }>, target: string): string {
  const files = Array.isArray(message.filenames) ? message.filenames.map(String) : [];
  return cargoUnitIdentityV1({ packageId: String(message.package_id), target: String(message.target?.name), kinds: Array.isArray(message.target?.kind) ? message.target.kind.map(String) : [], platform: files.some((file) => file.split(sep).includes(target)) ? target : null, features: Array.isArray(message.features) ? message.features.map(String) : [] });
}

/** 🔑️ The admission history key (`unitKeyV1`) of one unit-graph node compiled under `profile`, or `null` for a build-script run, which no rustc compiles. A
 * package id names its package after `#` (`…#name@version`), or by its directory when Cargo omits a name equal to it (`path+file:///…/name#version`). */
export function unitGraphHistoryKeyV1(unit: CargoUnitGraphV1["units"][number], profile: string): string | null {
  if (unit.mode === "run-custom-build") return null;
  const [location, fragment = ""] = unit.pkg_id.split("#"), name = fragment.includes("@") ? fragment.slice(0, fragment.lastIndexOf("@")) : location!.split("/").at(-1)!;
  return unitKeyV1({ package: decodeURIComponent(name), crate: unit.target.name.replaceAll("-", "_"), target: unit.platform ?? "host", crateTypes: unit.target.crate_types }, profile);
}

/** 📦️ One batch of a pass: the components it links, the output footprint predicted for their joint unit closure and whether every unit of it was measured. */
export type ComponentBatchV1 = Readonly<{ pass: NativeBuildPassV1; parent: string; bytes: number; measured: boolean }>;

/** 🧬️ The footprint kind of a unit, read from its crate name and crate types: a component final (`cdylib`), a proc-macro, a build script or a library. */
export type UnitFootprintKindV1 = "library" | "component" | "procMacro" | "buildScript";

/** 🧬️ Classifies one unit by its crate name and crate types (`unitKeyV1`'s `<package>@<crate>|<target>|<profile>|<types>`). */
export function unitFootprintKindV1(crate: string, crateTypes: readonly string[]): UnitFootprintKindV1 {
  return crateTypes.includes("cdylib") ? "component" : crateTypes.includes("proc-macro") ? "procMacro" : crate.startsWith("build_script_") ? "buildScript" : "library";
}

/** 📏️ How a batch's unmeasured units are estimated: the `quantile` of every recorded unit of the same kind, or the kind's documented upper bound when none
 * was ever recorded (`Policy.unitFootprintBoundsBytes`). The bounds come from this ticket's measurements: a component closure re-hashed 3 to 4 GB (4.05 GB
 * for process) over roughly 350 units, about 11 MiB per unit, so a library is bounded at 16 MiB (1.5 times that mean, summed over a closure); a component
 * final produced multiple GiB of outputs before codegen-units 16 and 50 to 110 MB of wasm after it, so it is bounded at 2 GiB; proc-macros (host dylibs) at
 * 64 MiB and build scripts (binary plus `OUT_DIR`) at 32 MiB.
 * @see ../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/📓️2026-10-13-scalable-build-architecture.md */
export type UnitFootprintPolicyV1 = Readonly<{ quantile: number; bounds: Readonly<Record<UnitFootprintKindV1, number>> }>;

/** 📦️ Splits one pass's components into batches whose joint unit closure is predicted to fit `budgetBytes`, in planned order: a unit's footprint is the
 * `quantile` of its own recorded `outputBytes`, else that of every recorded unit of its kind, else its kind's upper bound; shared units count once per batch.
 * A component whose own closure exceeds the budget is a batch of its own; a pass that fits whole keeps its identity, batches are `<pass>-b<n>`.
 * @see ../🎟️admission/🟦️.ts (`unitOutputBytesV1`, the store `units.jsonl`) */
export function componentBatchesV1(pass: NativeBuildPassV1, graph: CargoUnitGraphV1, history: ReadonlyMap<string, readonly UnitObservationV1[]>, budgetBytes: number, footprints: UnitFootprintPolicyV1): readonly ComponentBatchV1[] {
  const closures = unitGraphClosuresV1(graph, pass), units = new Map(graph.units.map((unit) => [unitGraphIdentityV1(unit), unit])), pools = new Map<UnitFootprintKindV1, number[]>();
  for (const [key, rows] of history) { const [head = "", , , types = ""] = key.split("|"), kind = unitFootprintKindV1(head.slice(head.indexOf("@") + 1), types.split(",")); pools.set(kind, [...pools.get(kind) ?? [], ...rows.map((row) => row.outputBytes)]); }
  const estimate = new Map([...pools].map(([kind, values]) => [kind, Math.ceil(quantileR7V1(values, footprints.quantile))]));
  const footprint = (identity: string): Readonly<{ bytes: number; measured: boolean }> => {
    const unit = units.get(identity)!, key = unitGraphHistoryKeyV1(unit, pass.profile), rows = key === null ? [] : history.get(key) ?? [];
    if (rows.length) return { bytes: Math.ceil(quantileR7V1(rows.map((row) => row.outputBytes), footprints.quantile)), measured: true };
    const kind = unit.mode === "run-custom-build" || unit.target.kind.includes("custom-build") ? "buildScript" : unitFootprintKindV1(unit.target.name.replaceAll("-", "_"), unit.target.crate_types);
    return { bytes: estimate.get(kind) ?? footprints.bounds[kind], measured: false };
  };
  const predict = (identities: ReadonlySet<string>): Readonly<{ bytes: number; measured: boolean }> => [...identities].map(footprint).reduce((sum, row) => ({ bytes: sum.bytes + row.bytes, measured: sum.measured && row.measured }), { bytes: 0, measured: true });
  const groups: { packages: NativeBuildPackageV1[]; identities: Set<string> }[] = [];
  for (const row of pass.packages) {
    const closure = closures.get(row.name) ?? new Set<string>(), current = groups.at(-1);
    if (current && predict(new Set([...current.identities, ...closure])).bytes <= budgetBytes) { current.packages.push(row); for (const identity of closure) current.identities.add(identity); }
    else groups.push({ packages: [row], identities: new Set(closure) });
  }
  return groups.map((group, index) => ({ pass: groups.length === 1 ? pass : { ...pass, id: `${pass.id}-b${index}`, packages: group.packages }, parent: pass.id, ...predict(group.identities) }));
}

/** ♻️ The files of one finished pass's `compiler-artifact` announcements for `target` that no remaining pass needs: every announced unit whose identity is
 * absent from `live` gives up its outputs and their dep-info, so a driver keeps only the live unit set on disk between passes.
 * @see ./🧬️schema/🔣️.json (`Policy.storageReserveBytes`) */
export function reclaimablePassFilesV1(messages: readonly Readonly<{ package_id?: unknown; target?: Readonly<{ name?: unknown; kind?: unknown }>; features?: unknown; filenames?: unknown }>[], target: string, live: ReadonlySet<string>): string[] {
  return [...new Set(messages.filter((message) => !live.has(announcedUnitIdentityV1(message, target))).flatMap((message) => (Array.isArray(message.filenames) ? message.filenames.map(String) : []).flatMap((file) => [file, file.replace(/\.[^./\\]+$/u, ".d")])))].sort();
}

/** 🔑️ The identity of a build-script run Cargo announced in a pass for `target`. */
export function announcedRunIdentityV1(packageId: string, outDir: string, target: string): string {
  return JSON.stringify([packageId, "run-custom-build", outDir.split(sep).includes(target) ? target : null]);
}

/** 📂️ The package directory a local Cargo package id (`path+file://<dir>#…`) names, or `null` for a registry or git package. */
export function cargoPackageDirectoryV1(packageId: string): string | null {
  return packageId.startsWith("path+file://") ? resolve(fileURLToPath(packageId.slice("path+".length).split("#")[0]!)) : null;
}

/** 🌲️ Every planned package's unit closure: the identities reachable from its root unit, the root being the unit whose package directory is the planned
 * manifest's directory (a taxonomy crate's sources live outside it, so the source path never decides ownership). */
export function unitGraphClosuresV1(graph: CargoUnitGraphV1, pass: NativeBuildPassV1): ReadonlyMap<string, ReadonlySet<string>> {
  const closures = new Map<string, Set<string>>(), owners = new Map(pass.packages.map((row) => [dirname(resolve(row.manifest)), row]));
  for (const index of graph.roots) {
    const directory = cargoPackageDirectoryV1(graph.units[index]!.pkg_id), owner = directory === null ? undefined : owners.get(directory);
    if (!owner) continue;
    const seen = new Set<number>(), identities = closures.get(owner.name) ?? new Set<string>(), stack = [index];
    while (stack.length) {
      const next = stack.pop()!;
      if (seen.has(next)) continue;
      seen.add(next);
      const unit = graph.units[next]!;
      identities.add(unitGraphIdentityV1(unit));
      for (const dependency of unit.dependencies) stack.push(dependency.index);
    }
    closures.set(owner.name, identities);
  }
  return closures;
}

/** 🍃️ Every observed unit's and build-script run's leaf in one invocation for `target`, keyed by identity, with the repository sources each unit read. */
export function invocationProvenanceLeavesV1(observed: Readonly<{ units: readonly CargoUnitObservationV1[]; buildResources: readonly Readonly<{ package_id: string; out_dir: string; path: string; sha256: string; resources: readonly any[] }>[] }>, target: string, roots: CargoProvenanceRootsV1): Readonly<{ leaves: ReadonlyMap<string, string>; sources: ReadonlyMap<string, readonly (readonly [string, string])[]> }> {
  const leaves = new Map<string, string>(), sources = new Map<string, (readonly [string, string])[]>();
  for (const unit of observed.units) {
    const identity = announcedUnitIdentityV1(unit.message as never, target);
    leaves.set(identity, cargoUnitLeafV1(unit, roots));
    sources.set(identity, unit.inputs.flatMap((input) => { const path = input.kind === "file" && input.sha256 ? cargoProvenanceSourceV1(input.path, roots) : null; return path ? [[path, input.sha256!] as const] : []; }));
  }
  for (const row of observed.buildResources) leaves.set(announcedRunIdentityV1(row.package_id, row.out_dir, target), cargoBuildRunLeafV1(row, roots));
  return Object.freeze({ leaves, sources });
}

/** 🌳️ One component's provenance from the leaves of the observed units its closure holds. */
export function componentProvenanceFromClosureV1(closure: ReadonlySet<string>, leaves: ReadonlyMap<string, string>, sources: ReadonlyMap<string, readonly (readonly [string, string])[]>): ComponentProvenanceV1 {
  const selected = [...closure].filter((identity) => leaves.has(identity));
  const rows = new Map<string, string>();
  for (const identity of selected) for (const [path, sha256] of sources.get(identity) ?? []) rows.set(path, sha256);
  if (!rows.size) throw Error("Component closure names no in-repository source");
  return Object.freeze({ root: cargoProvenanceMerkleRootV1(selected.map((identity) => leaves.get(identity)!)), leaves: Object.freeze(selected.map((identity) => leaves.get(identity)!).sort()), sourceTreeDigest: cargoProvenanceRowsDigestV1([...rows]), units: selected.length });
}

/** 🚦️ Driver outcome. */
export type NativeBuildOutcomeV1 = Readonly<{ generation: NativeBuildGenerationV1; report: string; staged: readonly string[]; failed: readonly Readonly<{ pass: string; package: string }>[]; units: number; observations:readonly Readonly<{pass:string;path:string}>[];compilations:readonly Readonly<{pass:string;capture:NativeBuildCompilationV1}>[] }>;

declare const nativeCompilationBrand:unique symbol;
export type NativeBuildCompilationV1=Readonly<{readonly[nativeCompilationBrand]:true}>;
type CompilationScopeV1={live:boolean;accepting:boolean;view:CompletedCargoCompilationViewV1;pending:Set<Promise<unknown>>;failures:unknown[]};
const nativeCompilationsV1=new WeakMap<NativeBuildCompilationV1,CompilationScopeV1>();
const compilationPendingMaximumV1=compilationSourceSchema.properties.maximumPendingOperations.const,compilationBytesMaximumV1=compilationSourceSchema.properties.maximumRetainedCompilationBytes.const;
/** 🧾️ Borrows an actual completed pass while its writer, retained allocation and whole-outcome consumer remain held. */
export function withNativeBuildCompilationTaskV1<T>(token:NativeBuildCompilationV1,body:(view:CompletedCargoCompilationViewV1)=>Promise<T>):Promise<T>{
 const scope=nativeCompilationsV1.get(token);if(!scope)throw Error("Actual completed Native pass capture required");const admitted=scope.live&&scope.accepting&&typeof body==="function"&&scope.pending.size<compilationPendingMaximumV1,task=withIssuedResourceOperationV1(scope.view.control,async()=>{await Promise.resolve();if(!admitted)throw Error("Live finite completed Native pass task required");await scope.view.recheck();const value=await body(Object.freeze({...scope.view,files:new Map(scope.view.files)}));await scope.view.recheck();return value;});if(admitted){scope.pending.add(task);void task.then(()=>scope.pending.delete(task),error=>{scope.pending.delete(task);scope.failures.push(error);});}return task;
}

/** 🎛️ What one driver run needs from its caller. */
export type NativeBuildDriverOptionsV1 = Readonly<{
  storage:NativeStorageReservationV1;
  sourceBoundary:CargoSourceBoundaryPolicyV1;
  plan: NativeBuildPlanV1;
  withCargoInputs?:CargoCompilerInputsPortV1;
  withPassInputs?:<T>(requests:readonly CargoCompilerInputsRequestV1[],body:(control:CompilerAdmissionControl)=>Promise<T>)=>Promise<T>;
  cwd: string;
  cacheRoot: string;
  shared: Readonly<{ target: string; build: string }>;
  generation: NativeBuildGenerationKindV1;
  leaseDirectory: string;
  environment: Readonly<Record<string, string | undefined>>;
  control: CompilerAdmissionControl;
  maximumObservationBytes?:number;
  retainCompilationBytes?:number;
  consume?: (outcome:NativeBuildOutcomeV1,control:CompilerAdmissionControl)=>Promise<void>;
  stages?: (component: LinkedComponentV1) => readonly NativeBuildStageV1[];
  jobEnvironment?: (job: NativeBuildJobV1, generation: NativeBuildGenerationV1) => Readonly<Record<string, string>>;
  tokens?: number;
  physicalBytes?: number;
  storageReserveBytes?: number;
  scheduler?: string;
  cargo?: Readonly<{ command: string; args: readonly string[] }>;
  stallMs?: number;
  progressMs?: number;
  log?: (line: string) => void;
}>;

/** 📥️ Admits each actual Cargo request while its exact configuration view remains live. */
async function withNativeBuildCargoInputsV1<T>(options:NativeBuildDriverOptionsV1,request:CargoCompilerInputsRequestV1,body:(inputs:CargoCompilerInputsViewV1)=>Promise<T>):Promise<T>{
 return withCargoCompilerInputsV1(request,options.withCargoInputs,async inputs=>{checkResourceLeaseControl(inputs.control);await admitCargoSelectionV1({workspaceRoot:options.cwd,cwd:request.cwd,args:inputs.arguments,environment:inputs.environment,sourceBoundary:options.sourceBoundary,storage:options.storage},inputs.control);checkResourceLeaseControl(inputs.control);await options.storage.recheck();checkResourceLeaseControl(inputs.control);const result=await body(inputs);checkResourceLeaseControl(inputs.control);return result;});
}

/** 🪵️ An append-only log file that stops growing at its byte cap. */
function boundedLog(path: string, limit: number): { write: (line: string) => void; close: () => void } {
  const descriptor = openSync(path, "w");
  let written = 0;
  return { write: (line) => { if (written < limit) written += writeSync(descriptor, `${line}\n`); }, close: () => closeSync(descriptor) };
}

const exec = promisify(execFile);

/** 🧮️ Resident bytes of the driver and its non-rustc children, the `nonRustc` term of the admission budget. */
async function nonRustcBytes(children: readonly ChildProcess[],control:CompilerAdmissionControl): Promise<number> {
  const pids = children.map((child) => child.pid).filter((pid): pid is number => pid !== undefined);
  if (!pids.length || process.platform === "win32") return process.memoryUsage().rss;
  const listed = await captureOwnedCommandResultV1("ps",["-o","rss=","-p",pids.join(",")],process.cwd(),{control,output:{maximumBytes:TELEMETRY_POLICY_V1.maximumOutputBytes,maximumLines:TELEMETRY_POLICY_V1.maximumOutputLines}});if(listed.signal!==null)throw Error("Telemetry process terminated by signal");if(listed.status!==0)return process.memoryUsage().rss;
  return process.memoryUsage().rss + listed.stdout.split(/\s+/u).filter(Boolean).reduce((sum, value) => sum + Number(value) * 1024, 0);
}

/** 🔬️ One pass's observed provenance: its staged receipt, every component's unit closure, every unit's leaf and the repository sources each unit read. */
type ObservedPassV1 = Readonly<{ staged: CargoProvenanceStagedV1; closures: ReadonlyMap<string, ReadonlySet<string>>; leaves: ReadonlyMap<string, string>; sources: ReadonlyMap<string, readonly (readonly [string, string])[]> }>;

/** ⏳️ A named completion any job or stage may wait for. */
type Completion = { promise: Promise<void>; resolve: () => void; reject: (error: unknown) => void; settled: boolean };

/** 🏁️ Installs terminal observation immediately after the actual child is born. */
function nativeBuildChildTerminalV1(child:ChildProcess):Promise<number>{const terminal=new Promise<number>((accept,refuse)=>{let failure:{error:unknown}|undefined;child.once("error",error=>{failure={error};});child.once("close",code=>failure?refuse(failure.error):accept(code??1));});void terminal.catch(()=>{});return terminal;}
/** 🪢️ Joins the real close and every stream before the enclosing CPU phase can retire. */
async function finishNativeBuildChildV1(child:ChildProcess,terminal:Promise<number>,streams:readonly Promise<unknown>[],primary:{error:unknown}|undefined,cleanups:readonly(()=>void)[]):Promise<void>{const failures:unknown[]=[];if(child.exitCode===null&&child.signalCode===null)try{terminateOwnedChildTree(child);}catch(error){failures.push(error);}for(const row of await Promise.allSettled([terminal,...streams]))if(row.status==="rejected"&&(!primary||!Object.is(primary.error,row.reason))&&!failures.some(error=>Object.is(error,row.reason)))failures.push(row.reason);for(const cleanup of cleanups)try{cleanup();}catch(error){if(!primary||!Object.is(primary.error,error))failures.push(error);}if(failures.length)throw new AggregateError(primary?[primary.error,...failures]:failures,"Native compiler child and cleanup failed");}

/** 🚀️ Runs one native component build: a GNU make jobserver of every core shared by all Cargo passes, jobs and post-link stages, a rustc wrapper admitting units by predicted
 * peak memory, one Cargo pass per planned workspace, profile and target (profiles in parallel lanes, workspaces of one profile in sequence so no two Cargo processes
 * contend for one unit), jobs started at t = 0 as soon as the completions they name happened (host tools, wasm-bindgen modules, product stages), each component
 * staged the moment Cargo links it and streamed through its post-link stages (each cached by content under the generation's stage cache), one provenance session
 * that hashes every file once and rechecks it once (a Merkle root per component closure), a CPU and memory timeline, a per-unit table, the critical path and a
 * report. The caller's remaining deadline bounds admission and every child. A stall watchdog additionally stops a run with no unit activity for `stallMs`
 * while no rustc runs, after sampling every Cargo. Original cancellation stops every Cargo tree, job, wrapper and stage.
 * @see ../../📓️2026-10-13-scalable-build-architecture.md §6.1–6.5, §7.1, §7.3 (ticket BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT) */
export function runNativeBuildDriverV1(options: NativeBuildDriverOptionsV1): Promise<NativeBuildOutcomeV1> {
 requireIssuedResourceControlV1(options.control);return withIssuedResourceOperationV1(options.control,async()=>{
  if(options.withCargoInputs&&!options.withPassInputs)throw Error("Actual Cargo configuration inputs require a retained pass transaction");
  nativeBuildPrerequisiteOrderV1(options.plan.jobs??[]);
  if(options.consume){requireIssuedResourceControlV1(options.control);if(typeof options.consume!=="function"||validateJsonSchemaSubset(schema.definitions.ObservationDeliveryV1,{version:1,maximumBytes:options.maximumObservationBytes},schema).length)throw Error("Finite actual Native observation delivery required");}
  if(options.retainCompilationBytes!==undefined&&(!options.consume||!Number.isSafeInteger(options.retainCompilationBytes)||options.retainCompilationBytes<1||options.retainCompilationBytes>compilationBytesMaximumV1||options.retainCompilationBytes>options.storage.allocation.bytes))throw Error("Finite cumulative completed pass retention required");
  options=Object.freeze({...options,environment:Object.freeze({...options.environment})});
  checkResourceLeaseControl(options.control);requireNativeStorageReservationV1(options.storage,options.control);if(options.storage.allocation.kind!=="compiler-workspace")throw Error("Original native driver compiler storage required");await options.storage.recheck();for(const path of[options.cacheRoot,options.shared.target,options.shared.build])requireNativeStoragePathV1(options.storage.backing.root,path);
  const plan = admitNativeBuildPlanV1(options.plan), policy = NATIVE_BUILD_DRIVER_POLICY_V1, log = options.log ?? ((line: string) => console.log(line)), jobs = plan.jobs ?? [], budget = compilerAdmissionBudgetV1(options.control);
  const planned = nativeGenerationLayoutV1(options.generation, join(resolve(options.cacheRoot), ...(options.generation === "warm" ? [policy.warmRoot] : []), (options.generation === "fresh" ? policy.generationPrefix : policy.warmRunPrefix) + "x".repeat(policy.randomSuffixUnits)), options.cacheRoot, options.shared);
  admitGenerationPathsV1(planned);
  if(options.retainCompilationBytes!==undefined){if(!options.environment.CARGO_HOME||!plan.passes.length||plan.passes.some(pass=>pass.profile!==plan.passes[0]!.profile))throw Error("One actual mode and bounded Cargo home required for completed retention");for(const path of[join(planned.root,"q","0".repeat(64)),join(planned.root,"q","r"+"0".repeat(64),"receipt.json")]){requireMaterializedPathV1(path);requireNativeStoragePathV1(options.storage.backing.root,path);}requireNativeStoragePathV1(options.storage.backing.root,options.environment.CARGO_HOME);}
  for (const pass of plan.passes) {
    for (const id of [pass.id, `${pass.id}-b${pass.packages.length}`]) for (const file of [`${policy.unitGraphFile}-${id}.json`, `cargo-${id}.jsonl`, `cargo-${id}.log`]) requireMaterializedPathV1(join(planned.logs, file));
    for (const row of pass.packages) requireMaterializedPathV1(join(planned.root, "w", row.name));
  }
  for (const job of jobs) requireMaterializedPathV1(join(planned.logs, `job-${job.id.replace(/[^A-Za-z0-9_.-]/gu, "_")}.log`));
  requireMaterializedPathV1(unitHistoryPathV1(options.cacheRoot));
  const generation = nativeBuildGenerationV1(options.generation, options.cacheRoot, options.shared), startedMs = Date.now(), cache = stageCacheV1(generation.stages);
  const signal = options.control.signal, children = new Set<ChildProcess>(), stages: StageRecordV1[] = [], runs: BuildScriptRunV1[] = [], staged: string[] = [], failures: { pass: string; package: string }[] = [];
  let stopped=false,stopReason:unknown;
  const stop = (reason:unknown):void => {if(!stopped){stopped=true;stopReason=reason;}for(const child of children)terminateOwnedChildTree(child);};
  const check = ():void => {if(stopped)throw stopReason;checkResourceLeaseControl(options.control);};
  const admission=options.control,inputContinuations=new CargoInputContinuationTransactionV1(options.control),planContinuations=new CargoInputContinuationTransactionV1(options.control);
  let deadline: ReturnType<typeof setTimeout> | undefined;
  const watchDeadline = (): void => {
    try { const remaining = checkResourceLeaseControl(options.control); if (remaining !== null) deadline = setTimeout(watchDeadline, Math.min(remaining, 1000)); }
    catch { if (!signal.aborted) stop(Error("Original native build deadline exhausted")); }
  };
  if (budget !== 0) watchDeadline();
  const observations:Readonly<{pass:string;path:string}>[]=[],compilations:Readonly<{pass:string;capture:NativeBuildCompilationV1}>[]=[],compilationScopes:CompilationScopeV1[]=[];let observationRosterBytes=2;
  const passRecords: { id: string; manifest: string; profile: string; target: string; startMs: number; endMs: number; exit: number; failed: string[] }[] = [];
  const failureLines: string[] = [], components = plan.passes.flatMap((pass) => pass.packages.map((row) => row.name));
  let stagesRunning = 0, stagesWaiting = 0, lastMessageMs = Date.now(), stallReason: string | null = null;
  log(`[native-build] ${generation.kind} generation ${generation.root}: ${plan.passes.length} pass(es), ${components.length} component build(s), ${jobs.length} job(s)`);
  let primaryFailed=false,dispose:undefined|(()=>Promise<void>);
  try {
  return await withCompilerCapacityConsumptionV1({request:{kind:"whole-host"},directory:options.leaseDirectory,control:admission,environment:options.environment},async(capacity,cpuControl)=>{
  const tokens = Math.min(options.tokens ?? capacity.parallelism, capacity.parallelism);
  options = { ...options, environment: { ...options.environment, SEMIO_COMPILER_PARALLELISM: String(tokens) } };
  const cargoHome = resolve(options.environment.CARGO_HOME ?? join(homedir(), ".cargo")), roots: CargoProvenanceRootsV1 = { repository: options.cwd, build: generation.build, cargoHome };
  const physicalControl = () => cargoProvenancePhysicalControlV1({ cancelled: () => signal.aborted, remainingMs: () => {inputContinuations.check();planContinuations.check();return checkResourceLeaseControl(admission);}, scale: Math.max(1, components.length), onProgress: cargoProvenanceProgressSinkV1(() => inputContinuations.check()) });
  const custody = sharedProvenanceCustodyV1(options.cwd, physicalControl);let sessionOwner:CargoProvenanceSessionV1|undefined;
  const scheduler = options.scheduler ?? (await buildSchedulerV1({ cwd: options.cwd, targetDirectory: generation.target, buildDirectory: generation.build, environment: options.environment, control: cpuControl,storage:options.storage,sourceBoundary:options.sourceBoundary })).get("semio-build-scheduler")!;
  const historyPath = unitHistoryPathV1(options.cacheRoot), history = await readUnitHistoryV1(historyPath);
  const jobserver = await createJobserverV1({ version: 1, tokens, style: defaultJobserverStyleV1() }, generation.jobserver);
  let broker:Awaited<ReturnType<typeof startAdmissionBrokerV1>>;
  try{broker=await startAdmissionBrokerV1({physicalBytes:options.physicalBytes??admissionMemoryBytesV1(),history});}catch(primary){try{jobserver.close();}catch(secondary){throw new AggregateError([primary,secondary],"Native broker initialization and jobserver retirement failed");}throw primary;}
  let timeline:ReturnType<typeof startTimelineV1>;
  try{timeline=startTimelineV1({
    control:cpuControl,
    path: join(generation.report, TELEMETRY_POLICY_V1.timelineFile),
    counters: () => { const stats = broker.stats(); return { rustcRunning: stats.running, admissionWaiting: stats.waiting, rustcRssBytes: stats.actualRunningBytes, frozen: stats.frozen, tokensHeld: jobserver.held(), stagesRunning, stagesWaiting }; },
    onRow: async (row,actualControl) => { const bytes=await nonRustcBytes([...children],actualControl);broker.observe({ nonRustcBytes: bytes, swapOutBytesPerSecond: row.swapOutBytesPerSecond }); },
  });}catch(primary){const secondary:unknown[]=[];try{await broker.close();}catch(error){secondary.push(error);}try{jobserver.close();}catch(error){secondary.push(error);}if(secondary.length)throw new AggregateError([primary,...secondary],"Native timeline initialization and resource retirement failed");throw primary;}
  const completions = new Map<string, Completion>();
  const completion = (name: string): Completion => {
    let row = completions.get(name);
    if (!row) {
      let accept!: () => void, refuse!: (error: unknown) => void;
      const promise = new Promise<void>((resolveCompletion, rejectCompletion) => { accept = resolveCompletion; refuse = rejectCompletion; });
      promise.catch(() => undefined);
      row = { promise, resolve: () => { if (!row!.settled) { row!.settled = true; accept(); } }, reject: (error) => { if (!row!.settled) { row!.settled = true; refuse(error); } }, settled: false };
      completions.set(name, row);
      const group = /^components:(.+)$/u.exec(name), owner = unlinked.get(name.slice(0, name.lastIndexOf(":")));
      if (group) Promise.all(components.map((component) => completion(`${component}:${group[1]}`).promise)).then(row.resolve, row.reject);
      else if (owner) row.reject(owner);
    }
    return row;
  };
  const unlinked = new Map<string, Error>();
  const failComponent = (component: string, error: Error): void => {
    unlinked.set(component, error);
    for (const [name, row] of completions) if (name.startsWith(`${component}:`)) row.reject(error);
  };
  const pending = new Set<Promise<void>>();
  const track = (task: Promise<void>): void => { pending.add(task); void task.then(() => pending.delete(task), () => pending.delete(task)); };
  const work = (component:string,name:string,after:readonly string[],run:(parallelism:number,control:CompilerAdmissionControl)=>Promise<void>,done?:Completion,intent:NativeBuildStageCapacityV1|"jobserver"="single"):void=>{
    const cleanup=intent==="cleanup",readyAt=cleanup?Promise.allSettled(after.map(dependency=>completion(dependency).promise)):Promise.all(after.map(dependency=>completion(dependency).promise));let waiting=false,startMs:number|undefined,readyMs=Date.now(),ok=false;
    const ready=async()=>{await readyAt;readyMs=Date.now();stagesWaiting++;waiting=true;};
    const invoke=async(parallelism:number,control:CompilerAdmissionControl)=>{if(waiting){stagesWaiting--;waiting=false;}stagesRunning++;startMs=Date.now();try{if(!cleanup)check();await run(parallelism,control);ok=true;}finally{stagesRunning--;}};
    let operation:Promise<void>;if(intent==="jobserver"){let slot:JobSlotV1|undefined;const accepted=withRequestedIssuedResourceControlV1(cpuControl,0,async control=>{await ready();checkResourceLeaseControl(control);slot=await jobserver.acquire(control.signal);await invoke(jobserver.config.tokens,control);});operation=withIssuedResourceOperationV1(cpuControl,async()=>{let primary:{error:unknown}|undefined;try{await accepted;}catch(error){primary={error};throw error;}finally{try{slot?.release();}catch(error){if(primary&&!Object.is(primary.error,error))throw new AggregateError([primary.error,error],"Native job and slot release failed");throw error;}}});}else operation=withNativeBuildStageCapacityV1({capacity:intent,directory:options.leaseDirectory,control:cpuControl,environment:options.environment,jobserver,ready},facts=>invoke(facts.parallelism,facts.control));
    track(operation.then(()=>done?.resolve(),error=>{done?.reject(error);failureLines.push(`[native-build] ${component} ${name}: ${error instanceof Error?error.message:String(error)}`);failures.push({pass:name,package:component});}).finally(()=>{if(waiting)stagesWaiting--;if(startMs!==undefined)stages.push({component,stage:name,readyMs,startMs,endMs:Date.now(),ok});}));
  };
  const provenanceOf = new Map<string, Promise<ObservedPassV1>>();
  const leavesReport: Record<string, unknown> = {};
  const runJob = (job: NativeBuildJobV1): void => work(`job:${job.id}`, "job", job.after, async (parallelism,jobControl) => {
    broker.registerPass({ id: job.id, profile: job.profile, target: job.target });
    parallelism=nativeBuildJobWorkerBudgetV1(job.workerProtocol,jobserver.config.tokens,()=>parallelism);
    const environment: Record<string, string | undefined> = { ...options.environment, ...job.environment, ...options.jobEnvironment?.(job, generation), SEMIO_BUILD_MODE: profileBuildModeV1(job.profile), ...jobserver.environment(), SEMIO_COMPILER_PARALLELISM: String(parallelism), RUSTC_WRAPPER: scheduler, ...compilerWrapperEnvironmentV1(scheduler, generation.root, options.environment), [ADMISSION_POLICY_V1.brokerEnvironment]: broker.endpoint, [ADMISSION_POLICY_V1.passEnvironment]: job.id, CARGO_BUILD_BUILD_DIR: generation.build, SEMIO_NATIVE_BUILD_GENERATION_ROOT: generation.root, CARGO_TERM_COLOR: "never" };
    delete environment.SEMIO_COMPILER_CAPACITY_HELD;
    if (job.workerProtocol !== "jobserver") { delete environment.MAKEFLAGS; delete environment.CARGO_MAKEFLAGS; }
    const execute=async(inputs:CargoCompilerInputsViewV1):Promise<void>=>withRequestedIssuedResourceControlV1(jobControl,0,async control=>{
    const signal=control.signal;checkResourceLeaseControl(control);await options.storage.recheck();checkResourceLeaseControl(inputs.control);for(const name of ["CARGO_HOME","CARGO_TARGET_DIR","CARGO_BUILD_BUILD_DIR","TMPDIR","TMP","TEMP"]){const path=inputs.environment[name];if(!path)throw Error("Bounded native job environment required: "+name);requireNativeStoragePathV1(options.storage.backing.root,path);}
    const output = boundedLog(join(generation.logs, `job-${job.id.replace(/[^A-Za-z0-9_.-]/gu, "_")}.log`), policy.logLimitBytes);
    const child = spawn(job.command[0]!, [...inputs.arguments], { cwd: job.cwd, detached: process.platform !== "win32", stdio: ["ignore", "pipe", "pipe"], env: scriptResourceProcessEnvironmentV1(control,inputs.environment) });
    const terminal=nativeBuildChildTerminalV1(child);children.add(child);
    const stop = (): void => terminateOwnedChildTree(child);
    const tail: string[] = [];
    const collect = async (stream: NodeJS.ReadableStream): Promise<void> => { for await (const line of createInterface({ input: stream, crlfDelay: Infinity })) { output.write(line); lastMessageMs = Date.now(); tail.push(line); if (tail.length > policy.failureLinesPerUnit) tail.shift(); } };
    const streams:Promise<void>[]=[];let primary:{error:unknown}|undefined;
    try {
      signal.addEventListener("abort",stop,{once:true});streams.push(collect(child.stdout!),collect(child.stderr!));
      const [exit] = await Promise.all([terminal,...streams]);
      checkResourceLeaseControl(control);checkResourceLeaseControl(inputs.control);if (exit !== 0) throw Error(`exit ${exit}\n${tail.slice(-Math.min(tail.length, 40)).join("\n")}`);
    } catch(error){primary={error};throw error;}finally {try{await finishNativeBuildChildV1(child,terminal,streams,primary,[()=>output.close()]);}finally{signal.removeEventListener("abort", stop);children.delete(child);}}
    },inputs.control.signal);
    if(job.command[0]==="cargo"){const args=job.command.slice(1),position=args.indexOf("--manifest-path"),manifest=position<0?args.find(arg=>arg.startsWith("--manifest-path="))?.slice(16)??join(job.cwd,"Cargo.toml"):args[position+1];if(!manifest)throw Error("Actual native job manifest required");await withNativeBuildCargoInputsV1(options,{manifest,cwd:job.cwd,arguments:args,environment,control:options.control},execute);}else await execute({arguments:job.command.slice(1),environment,control:options.control});
  }, completion(`job:${job.id}`), job.workerProtocol==="jobserver"?"jobserver":job.workerProtocol==="exclusive"?"whole-host":"available");

  const progress = setInterval(() => {
    const stats = broker.stats(), row = timeline.rows().at(-1), cached = cache.stats();
    log(`[native-build] ${broker.units().length} unit(s) done, ${stats.running} running · ${new Set(staged).size}/${components.length} component(s) staged · ${stagesRunning} stage(s) running · stage cache ${cached.hits} hit(s)/${cached.misses} miss(es) · cpu ${row?.cpuPercent ?? 0} % · rustc ${(stats.actualRunningBytes / 1024 ** 3).toFixed(1)}/${(stats.budgetBytes / 1024 ** 3).toFixed(1)} GiB · admitted-wait ${stats.waiting}${stats.frozen ? " · frozen" : ""} · ${((Date.now() - startedMs) / 60000).toFixed(1)} min`);
  }, options.progressMs ?? policy.progressMs);
  let stallWork:Promise<void>|undefined;
  const watchdog = setInterval(() => {
    const stats = broker.stats(), idleMs = Date.now() - Math.max(stats.lastActivityMs, lastMessageMs);
    if (children.size === 0 || stats.running > 0 || stagesRunning > 0 || idleMs < (options.stallMs ?? policy.stallMs)) return;
    if (stallReason) return;
    stallReason = `Native build stalled: no unit started or finished for ${Math.round(idleMs / 1000)} s with ${children.size} child process(es) and no rustc running; samples in ${generation.logs}`;
    const failure=Error(stallReason),pids=Object.freeze([...children].flatMap(child=>child.pid===undefined?[]:[child.pid]));
    try{stallWork=withRequestedIssuedResourceControlV1(cpuControl,0,async actual=>{
     try{if(pids.length>TELEMETRY_POLICY_V1.maximumStallSamples)throw Error("Bounded owned stall sample roster required");if(process.platform==="darwin")for(const pid of pids)await withRequestedIssuedResourceControlV1(actual,TELEMETRY_POLICY_V1.maximumStallSampleMilliseconds,async sampleControl=>{const result=await captureOwnedCommandResultV1("sample",[String(pid),"3"],options.cwd,{control:sampleControl,output:{maximumBytes:TELEMETRY_POLICY_V1.maximumOutputBytes,maximumLines:TELEMETRY_POLICY_V1.maximumOutputLines}});if(result.signal!==null)throw Error("Owned stall sample terminated by signal");checkResourceLeaseControl(sampleControl);writeFileSync(join(generation.logs,`stall-${pid}.txt`),result.stdout+result.stderr);});}
     catch(error){stop(error);throw error;}finally{stop(failure);}
    });}catch(error){stallWork=Promise.reject(error);stop(error);}void stallWork.catch(()=>{});
  }, Math.min(policy.watchdogMs, options.stallMs ?? policy.stallMs));
  let graphOperations:CargoGraphOperationsV1|undefined;
  let outcome: NativeBuildOutcomeV1 | undefined, custodyClosed = false, resourcesClosing:Promise<Awaited<ReturnType<typeof timeline.stop>>>|undefined;
  const closeResources=()=>resourcesClosing??=(async()=>{let failed=false,failure:unknown,rows:Awaited<ReturnType<typeof timeline.stop>>=[];try{rows=await timeline.stop();}catch(error){failed=true;failure=error;}try{await broker.close();}catch(error){if(!failed){failed=true;failure=error;}}try{jobserver.close();}catch(error){if(!failed){failed=true;failure=error;}}if(failed)throw failure;return rows;})();
  const disposeDriver=async()=>{clearInterval(progress);clearInterval(watchdog);graphOperations?.stop();for(const child of children)terminateOwnedChildTree(child);let failed=false,failure:unknown;try{await stallWork;}catch(error){failed=true;failure=error;}try{await graphOperations?.close();}catch(error){failed=true;failure=error;}while(pending.size)await Promise.allSettled([...pending]);try{if(!custodyClosed)await sessionOwner?.discard();}catch(error){if(!failed){failed=true;failure=error;}}try{await closeResources();}catch(error){if(!failed){failed=true;failure=error;}}if(failed)throw failure;};dispose=disposeDriver;
  const executePasses=async(actualControl:CompilerAdmissionControl):Promise<()=>Promise<NativeBuildOutcomeV1>>=>{
  requireIssuedResourceControlRefinementV1(options.control,actualControl);options={...options,control:actualControl};
  const session=new CargoProvenanceSessionV1(custody,options.retainCompilationBytes===undefined?undefined:{workspace:options.cwd,mode:profileBuildModeV1(plan.passes[0]?.profile),buildDirectory:generation.build,targetDirectory:generation.target,cargoHome,directory:join(generation.root,"q"),maximumBytes:options.retainCompilationBytes,storage:options.storage,control:actualControl});sessionOwner=session;
  const graphOperationsOwner=new CargoGraphOperationsV1(actualControl);graphOperations=graphOperationsOwner;
  const unitGraphs = new Map(plan.passes.map((pass) => [pass.id, graphOperationsOwner.start(()=>withNativeBuildCargoInputsV1(options,{manifest:pass.manifest,cwd:options.cwd,arguments:cargoConfigurationArgumentsV1(nativeBuildUnitGraphArgsV1(pass),undefined,generation.build),environment:{...options.environment,CARGO_TARGET_DIR:generation.target,CARGO_BUILD_BUILD_DIR:generation.build,CARGO_UNSTABLE_FEATURE_UNIFICATION:"true",CARGO_RESOLVER_FEATURE_UNIFICATION:policy.featureUnification,CARGO_TERM_COLOR:"never"},control:options.control},async inputs=>withRequestedIssuedResourceControlV1(cpuControl,0,async effectControl=>{
    const port = options.cargo ?? { command: "cargo", args: [] };
    check();graphOperationsOwner.check();checkResourceLeaseControl(inputs.control);
    const request = exec(port.command, [...port.args, ...inputs.arguments], { cwd: options.cwd, signal:effectControl.signal, timeout: compilerAdmissionBudgetV1(effectControl), maxBuffer: 512 * 1024 * 1024, env: scriptResourceProcessEnvironmentV1(effectControl,inputs.environment) });
    const terminal=nativeBuildChildTerminalV1(request.child);children.add(request.child);
    let output: Awaited<typeof request>,primary:{error:unknown}|undefined;
    try { output = await request; check();graphOperationsOwner.check();checkResourceLeaseControl(inputs.control); } catch (error) { primary={error};throw error; } finally {try{await finishNativeBuildChildV1(request.child,terminal,[request],primary,[]);}finally{children.delete(request.child);}}
    writeFileSync(join(generation.logs, `${policy.unitGraphFile}-${pass.id}.json`), output.stdout);
    return JSON.parse(output.stdout) as CargoUnitGraphV1;
  },inputs.control.signal)))]));
  for (const graph of unitGraphs.values()) graph.catch(() => undefined);
  const componentProvenance = (pass: NativeBuildPassV1, row: NativeBuildPackageV1): Promise<ComponentProvenanceV1> => (provenanceOf.get(pass.id) ?? Promise.reject(Error(`Pass ${pass.id} has no provenance`))).then((observed) => {
    const closure = observed.closures.get(row.name);
    if (!closure) throw Error(`${row.name}: the unit graph of ${pass.id} has no root for it`);
    return componentProvenanceFromClosureV1(closure, observed.leaves, observed.sources);
  });
  const streamComponent = (pass: NativeBuildPassV1, row: NativeBuildPackageV1, wasm: string): void => {
    const target = nativeBuildComponentStagePathV1(generation,pass,row);requireNativeStoragePathV1(options.storage.backing.root,target);const linked: LinkedComponentV1 = { pass, package: row, wasm, staged: target };
    work(row.name, "stage", [], async (_parallelism,control) => {
      const descriptor = openSync(wasm, "r"), header = Buffer.alloc(8);
      try { readSync(descriptor, header, 0, 8, 0); } finally { closeSync(descriptor); }
      if (JSON.stringify([...header]) !== JSON.stringify(policy.componentHeader)) throw Error(`Invalid WASI component header in ${wasm}`);
      await stageArtifacts(dirname(target),relative(options.cwd,resolve(row.manifest)).split(sep).join("/"),new Map([[basename(target),wasm]]),{control,signal:control.signal,leaseDirectory:options.leaseDirectory});
      staged.push(`${row.name}:${pass.profile}`);
    }, completion(`${row.name}:stage`));
    const declared = options.stages?.(linked) ?? [];
    for(const step of declared){const intent=readNativeBuildStageCapacityV1(step.capacity),name=step.name,after=[...step.after],run=step.run;if(typeof run!=="function")throw Error("Actual declared stage body required");work(row.name,name,[`${row.name}:stage`,...[...after,...plan.stageAfter?.[name]??[]].map(name=>name.includes(":")?name:`${row.name}:${name}`)],(parallelism,control)=>{const environment:Record<string,string|undefined>={...options.environment,SEMIO_BUILD_MODE:profileBuildModeV1(pass.profile),SEMIO_COMPILER_PARALLELISM:String(parallelism),GOMAXPROCS:String(parallelism)};delete environment.MAKEFLAGS;delete environment.CARGO_MAKEFLAGS;delete environment.SEMIO_COMPILER_CAPACITY_HELD;return run(Object.freeze({control,signal:control.signal,parallelism,generation,cache,environment:Object.freeze(environment),log,provenance:()=>componentProvenance(pass,row)}));},completion(`${row.name}:${name}`),intent);}

  };
  const prefetchQueue: { filenames?: string[] }[] = [];
  let prefetching = false;
  const prefetch = (message: { filenames?: string[] }): void => {
    prefetchQueue.push(message);
    if (prefetching) return;
    prefetching = true;
    track((async () => {
      const slot = await jobserver.acquire(signal);
      try { while (prefetchQueue.length && !signal.aborted) await session.prefetch(prefetchQueue.shift()!).catch((error) => { if (!signal.aborted) log(`[native-build] provenance prefetch: ${error instanceof Error ? error.message : String(error)}`); }); }
      finally { prefetching = false; slot.release(); }
    })());
  };
  const reserveBytes = options.storageReserveBytes ?? NATIVE_BUILD_DRIVER_POLICY_V1.storageReserveBytes, finishedPasses = new Set<string>(), reclaims = new Map<string, Promise<void>>(), pendingReclaims = new Set<Promise<void>>(), inflight = new Map<string, Readonly<{ bytes: number; settled: Promise<void> }>>(), outputBytes = new Map<string, number>();
  let batches: readonly ComponentBatchV1[] = [];
  const measureOutputs = (pass?: string): void => {
    const listed = new Map<string, readonly string[]>();
    for (const unit of broker.units()) {
      if (unit.exit !== 0 || outputBytes.has(unit.artifact) || pass !== undefined && unit.pass !== pass) continue;
      const directory = dirname(unit.artifact);
      if (!listed.has(directory)) listed.set(directory, existsSync(directory) ? readdirSync(directory) : []);
      outputBytes.set(unit.artifact, unitOutputBytesV1(unit.artifact, listed.get(directory)!, (name) => statSync(join(directory, name)).size));
    }
  };
  const graphOf = (id: string): Promise<CargoUnitGraphV1> => unitGraphs.get(batches.find((row) => row.pass.id === id)?.parent ?? id)!;
  const remainingBytes = (): number => Math.max(0, ...batches.filter((row) => !finishedPasses.has(row.pass.id)).map((row) => row.bytes));
  const headroomBytes = (): number => { const facts = nativeStatfsSyncV1(options.storage.backing.root); return facts.bavail * facts.bsize; };
  const reclaimPass = (pass: NativeBuildPassV1, messages: readonly unknown[]): void => {
    const done = completion(`reclaim:${pass.id}`), settled = done.promise.catch(() => undefined);
    reclaims.set(pass.id, settled);
    pendingReclaims.add(settled);
    void settled.finally(() => pendingReclaims.delete(settled));
    work(pass.id, "reclaim", [`provenance:${pass.id}`], async () => {
      if (generation.kind !== "fresh" && headroomBytes() >= remainingBytes() + reserveBytes) return;
      const live = new Set((await Promise.all(batches.filter((row) => !finishedPasses.has(row.pass.id)).map((row) => graphOf(row.pass.id)))).flatMap((graph) => graph.units.map(unitGraphIdentityV1)));
      let bytes = 0, files = 0;
      for (const file of reclaimablePassFilesV1(messages as Parameters<typeof reclaimablePassFilesV1>[0], pass.target, live)) {
        if (!existsSync(file)||!session.canReclaimOutput(file)) continue;
        requireNativeStoragePathV1(options.storage.backing.root, file);
        bytes += statSync(file).size; files++;
        rmSync(file, { force: true });
      }
      log(`[native-build] ${pass.id}: reclaimed ${files} file(s), ${(bytes / 1024 ** 2).toFixed(0)} MiB no remaining pass needs; ${(headroomBytes() / 1024 ** 3).toFixed(1)} GiB free`);
    }, done);
  };
  const availableBytes = (): number => headroomBytes() - [...inflight.values()].reduce((sum, row) => sum + row.bytes, 0);
  const runPass = async (pass: NativeBuildPassV1): Promise<void> => {
    await options.storage.recheck();
    const predicted = batches.find((row) => row.pass.id === pass.id)!;
    while (availableBytes() < predicted.bytes + reserveBytes && (inflight.size || pendingReclaims.size)) { check(); await Promise.race([...[...inflight.values()].map((row) => row.settled), ...pendingReclaims]); }
    if (availableBytes() < predicted.bytes + reserveBytes) throw Error(`Native build storage refused: ${pass.id} needs a predicted ${(predicted.bytes / 1024 ** 2).toFixed(0)} MiB${predicted.measured ? "" : " (unmeasured units estimated by kind)"} plus the ${(reserveBytes / 1024 ** 2).toFixed(0)} MiB reserve, ${(availableBytes() / 1024 ** 2).toFixed(0)} MiB remain after every running batch settled and every unit no remaining pass needs was reclaimed`);
    let release = (): void => undefined;
    inflight.set(pass.id, { bytes: predicted.bytes, settled: new Promise<void>((accept) => { release = accept; }) });
    try { await runAdmittedPass(pass); } finally { inflight.delete(pass.id); release(); }
  };
  const runAdmittedPass = (pass: NativeBuildPassV1): Promise<void> => inputContinuations.start(consume=>withNativeBuildCargoInputsV1(options,{manifest:pass.manifest,cwd:options.cwd,arguments:cargoConfigurationArgumentsV1(nativeBuildPassArgsV1(pass),undefined,generation.build),environment:{...options.environment,CARGO_TARGET_DIR:generation.target,CARGO_BUILD_BUILD_DIR:generation.build},control:options.control},consume),async inputs=>withRequestedIssuedResourceControlV1(cpuControl,0,async effectControl=>{
    const args = [...inputs.arguments],signal=effectControl.signal,checkPass=()=>{check();checkResourceLeaseControl(inputs.control);checkResourceLeaseControl(effectControl);}, startMs = Date.now(), units: { message: any; evidence: unknown }[] = [], buildScripts: unknown[] = [], linked = new Map<string, string>();
    const slot = await jobserver.acquire(signal);
    let lease: Awaited<ReturnType<typeof acquireCargoBuildLeaseV1>> | undefined;
    try {
      checkPass();lease = await acquireCargoBuildLeaseV1({ directory: options.leaseDirectory, buildDirectory: generation.build, targetDirectory: generation.target, args, signal, control: effectControl });
      checkPass();
      broker.registerPass({ id: pass.id, profile: pass.profile, target: pass.target });
      const port = options.cargo ?? { command: "cargo", args: [] };
      await ensureCargoLockCurrentV1({ workspaceRoot: options.cwd, cwd: options.cwd, program: port.command, prefix: port.args, args, environment: inputs.environment, control: effectControl, force: false });checkPass();
      const child = spawn(port.command, [...port.args, ...args], { cwd: options.cwd, detached: process.platform !== "win32", stdio: ["ignore", "pipe", "pipe"], env: scriptResourceProcessEnvironmentV1(effectControl,{ ...inputs.environment, SEMIO_BUILD_MODE: profileBuildModeV1(pass.profile), ...jobserver.environment(), RUSTC_WRAPPER: scheduler, ...compilerWrapperEnvironmentV1(scheduler, generation.root, options.environment), [ADMISSION_POLICY_V1.brokerEnvironment]: broker.endpoint, [ADMISSION_POLICY_V1.passEnvironment]: pass.id, CARGO_TARGET_DIR: generation.target, CARGO_BUILD_BUILD_DIR: generation.build, SEMIO_COMPILER_RESOURCE_ROOT: join(generation.build, "semio-compiler-resources"), CARGO_UNSTABLE_FEATURE_UNIFICATION: "true", CARGO_RESOLVER_FEATURE_UNIFICATION: policy.featureUnification, CARGO_TERM_COLOR: "never" }) });
      const terminal=nativeBuildChildTerminalV1(child);children.add(child);
      const stop = (): void => terminateOwnedChildTree(child),logs:ReturnType<typeof boundedLog>[]=[],streams:Promise<void>[]=[];let primary:{error:unknown}|undefined;
      try {
        signal.addEventListener("abort",stop,{once:true});
        const messages = boundedLog(join(generation.logs, `cargo-${pass.id}.jsonl`), policy.logLimitBytes);logs.push(messages);const diagnostics = boundedLog(join(generation.logs, `cargo-${pass.id}.log`), policy.logLimitBytes);logs.push(diagnostics);
        const perUnit = new Map<string, number>();
        streams.push(
          (async () => {
            for await (const line of createInterface({ input: child.stdout!, crlfDelay: Infinity })) {
              messages.write(line);
              lastMessageMs = Date.now();
              let message: any;
              try { message = JSON.parse(line); } catch { continue; }
              if (message.reason === "compiler-artifact") {
                units.push({ message, evidence: { version: 1, kind: "discovery", paths: [], producer: null } });
                prefetch(message);
                const component = componentArtifactV1(message, pass, generation.target);
                if (component && !linked.has(component.package.name)) { linked.set(component.package.name, component.wasm); streamComponent(pass, component.package, component.wasm); }
              } else if (message.reason === "build-script-executed") {
                buildScripts.push(message);
                runs.push({ pass: pass.id, package: String(message.package_id).split("#")[1]?.split("@")[0] ?? String(message.package_id), outDir: message.out_dir, endMs: Date.now() });
              } else if (message.reason === "compiler-message" && message.message?.level === "error") {
                const unit = `${message.package_id?.split("#")[1] ?? message.package_id} ${message.target?.name ?? ""}`, lines = String(message.message.rendered ?? message.message.message).split("\n"), used = perUnit.get(unit) ?? 0;
                const room = Math.max(0, Math.min(policy.failureLinesPerUnit - used, policy.failureLinesTotal - failureLines.length));
                failureLines.push(...lines.slice(0, room).map((text) => `[${pass.id}] ${unit}: ${text}`));
                perUnit.set(unit, used + Math.min(room, lines.length));
              }
            }
          })(),
          (async () => { for await (const line of createInterface({ input: child.stderr!, crlfDelay: Infinity })) diagnostics.write(line); })());
        const [exit]=await Promise.all([terminal,...streams]);
        checkPass();const missing = pass.packages.filter((row) => !linked.has(row.name));
        for (const row of missing) {
          failures.push({ pass: pass.id, package: row.name });
          rmSync(dirname(nativeBuildComponentStagePathV1(generation,pass,row)),{recursive:true,force:true});
          failComponent(row.name, Error(`${row.name} was not linked by ${pass.id}`));
        }
        if (exit !== 0 && !missing.length) failures.push({ pass: pass.id, package: `cargo exit ${exit}` });
        passRecords.push({ id: pass.id, manifest: pass.manifest, profile: pass.profile, target: pass.target, startMs, endMs: Date.now(), exit, failed: missing.map((row) => row.name) });
        log(`[native-build] ${pass.id} ${pass.profile} ${basename(dirname(pass.manifest))}: exit ${exit}, ${linked.size}/${pass.packages.length} component(s) linked, ${units.length} unit(s) in ${((Date.now() - startMs) / 1000).toFixed(0)} s`);
        completion(`pass:${pass.id}`).resolve();
        if (signal.aborted) return;
        const observationPath=nativeBuildPassObservationPathV1(generation,pass,args),name=basename(observationPath);
        let observed: ObservedPassV1 | undefined;
        const done = completion(`provenance:${pass.id}`);
        const provenance = done.promise.then(() => observed!);
        provenance.catch(() => undefined);
        provenanceOf.set(pass.id, provenance);
        work(pass.id, "provenance", [...linked.keys()].map((packageName) => `${packageName}:stage`), async () => {
          const stagedPaths = new Map([...linked].map(([packageName, wasm]) => [wasm, nativeBuildComponentStagePathV1(generation,pass,pass.packages.find(row=>row.name===packageName)!)]));
          const written = await session.write(observationPath, { manifest: pass.manifest, cwd: options.cwd, command: port.command, args: [...port.args, ...args], buildDirectory: generation.build, builtAtMs: startMs, status: exit, cancelled: false, units, buildScripts }, stagedPaths, cargoHome,inputs.control);
          const graph = await graphOf(pass.id), { leaves, sources } = invocationProvenanceLeavesV1(written, pass.target, roots);
          const closures = unitGraphClosuresV1(graph, pass);
          leavesReport[pass.id] = { receipt: name, units: Object.fromEntries([...leaves].sort()), components: Object.fromEntries([...closures].map(([packageName, closure]) => { try { const provenance = componentProvenanceFromClosureV1(closure, leaves, sources); return [packageName, { root: provenance.root, units: provenance.units, sourceTreeDigest: provenance.sourceTreeDigest }]; } catch (error) { return [packageName, { error: error instanceof Error ? error.message : String(error) }]; } })) };
          if(options.consume){const row=Object.freeze({pass:pass.id,path:observationPath}),bytes=Buffer.byteLength(JSON.stringify(row))+1;if(bytes>options.maximumObservationBytes!-observationRosterBytes)throw Error("Actual observation delivery roster extent exceeded");observationRosterBytes+=bytes;observations.push(row);}
          observed = { staged: written, closures, leaves, sources };
        }, done);
        measureOutputs(pass.id);
        finishedPasses.add(pass.id);
        reclaimPass(pass, units.map((row) => row.message));
      }catch(error){primary={error};throw error;}finally {try{await finishNativeBuildChildV1(child,terminal,streams,primary,logs.map(log=>()=>log.close()));}finally{signal.removeEventListener("abort",stop);children.delete(child);}}
    } finally { lease?.release(); slot.release(); }
  },inputs.control.signal));
  let executionFailed=false;
  try {
    const budget = headroomBytes();
    batches = (await Promise.all(plan.passes.map(async (pass) => componentBatchesV1(pass, await unitGraphs.get(pass.id)!, history, budget - reserveBytes, { quantile: ADMISSION_POLICY_V1.quantile, bounds: policy.unitFootprintBoundsBytes })))).flat();
    const passes = batches.map((row) => row.pass);
    for (const row of batches) if (row.pass.id !== row.parent) log(`[native-build] ${row.pass.id}: ${row.pass.packages.length} component(s), predicted ${(row.bytes / 1024 ** 2).toFixed(0)} MiB of ${((budget - reserveBytes) / 1024 ** 2).toFixed(0)} MiB free above the reserve${row.measured ? "" : " (unmeasured units estimated by kind)"}`);
  const lanes = [...new Set([...passes.map((pass) => pass.profile), ...jobs.flatMap((job) => job.lane === undefined ? [] : [job.lane])])].map((profile) => ({ jobs: jobs.filter((job) => job.lane === profile), passes: passes.filter((pass) => pass.profile === profile) }));
    await Promise.all(lanes.map(async (lane) => {
      for (const pass of lane.passes) { check(); await runPass(pass); }
    }));
    while (pending.size) await Promise.all([...pending]);
    check();
    if (!failures.length) {
      const startMs = Date.now();
      inputContinuations.check();await session.close();inputContinuations.check();
      if(options.retainCompilationBytes!==undefined)for(const[pass,observed]of provenanceOf){const view=await session.completedSnapshot((await observed).staged),token=Object.freeze({})as NativeBuildCompilationV1,scope:CompilationScopeV1={live:true,accepting:true,view,pending:new Set(),failures:[]};nativeCompilationsV1.set(token,scope);compilationScopes.push(scope);compilations.push(Object.freeze({pass,capture:token}));}
      custodyClosed = true;
      stages.push({ component: "release", stage: "provenance-recheck", readyMs: startMs, startMs, endMs: Date.now(), ok: true });
    }
  } catch(error){executionFailed=true;throw error;} finally {
    try{let cleanupFailed=false,cleanupFailure:unknown;try{await disposeDriver();}catch(error){cleanupFailed=true;cleanupFailure=error;}
    const rows = resourcesClosing?await resourcesClosing.catch(error=>{if(!executionFailed)throw error;return [];}):[], units = broker.units(), endedMs = Date.now(), critical = criticalPathV1(units, runs), cached = cache.stats(), reads = custody.files.observedReads();
    if(cleanupFailed)throw cleanupFailure;
    const readCounts = [...reads.files.values()].map((row) => row.reads), distinct = [...reads.files.values()].reduce((sum, row) => sum + row.bytes / Math.max(1, row.reads), 0);
    mkdirSync(join(generation.report, policy.provenanceDirectory), { recursive: true });
    writeFileSync(join(generation.report, policy.provenanceDirectory, policy.provenanceLeavesFile), `${JSON.stringify(leavesReport, null, 1)}\n`);
    writeFileSync(join(generation.report, policy.provenanceDirectory, policy.provenanceReadsFile), `${JSON.stringify({ files: reads.files.size, bytes: reads.bytes, distinctBytes: distinct, maximumReads: Math.max(0, ...readCounts), closed: custodyClosed, excess: [...reads.files].filter(([, row]) => row.reads > 2).slice(0, 50).map(([path, row]) => ({ path, reads: row.reads })), histogram: Object.fromEntries([...new Set(readCounts)].sort().map((count) => [count, readCounts.filter((value) => value === count).length])) }, null, 1)}\n`);
    const report = writeBuildReportV1(generation.report, { title: `Native build ${basename(generation.root)}`, startedMs, endedMs, timeline: rows, units, runs, stages, passes: passRecords, critical, sections: [
      { title: "Stage cache", rows: [["Root", generation.stages], ["Hits", cached.hits], ["Misses", cached.misses], ["Restored MiB", (cached.restoredBytes / 1024 ** 2).toFixed(1)], ["Stored MiB", (cached.storedBytes / 1024 ** 2).toFixed(1)]] },
      { title: "Provenance custody", rows: [["Files hashed", reads.files.size], ["Body bytes read", reads.bytes], ["Distinct file bytes", Math.round(distinct)], ["Reads per file (max)", Math.max(0, ...readCounts)], ["Final recheck", custodyClosed ? "passed" : "not reached"]] },
    ] });
    if (units.length && !signal.aborted && (options.control.remainingMilliseconds() ?? Infinity) > 0) { measureOutputs(); await recordUnitHistoryV1(historyPath, unitObservationsV1(units, critical.remainingMs, outputBytes, endedMs), { leaseDirectory: options.leaseDirectory, control: admission }); }
    for (const line of failureLines.slice(0, policy.failureLinesTotal)) log(line);
    log(`[native-build] ${units.length} unit(s), ${new Set(staged).size} component(s) staged, ${stages.length} stage run(s), stage cache ${cached.hits} hit(s)/${cached.misses} miss(es), provenance ${reads.files.size} file(s) ${(reads.bytes / 1024 ** 2).toFixed(0)} MiB, ${failures.length} failure(s) in ${((endedMs - startedMs) / 1000).toFixed(0)} s; report ${report}`);
    outcome = Object.freeze({ generation:Object.freeze(generation), report, staged:Object.freeze([...new Set(staged)]), failed:Object.freeze(failures), units:units.length, observations: Object.freeze([...observations].sort((a,b)=>a.pass.localeCompare(b.pass,"en"))),compilations:Object.freeze(compilations) });
    }catch(error){if(!executionFailed)throw error;}
  }
  if(!outcome)throw Error("Actual completed Native driver report required");
  if (stallReason) throw Error(stallReason);
  if (failures.length) throw Error(`Native build failed: ${failures.map((row) => `${row.package} (${row.pass})`).join(", ")}; report ${outcome.report}`);
  const completedOutcome=outcome;return async()=>{
  let consumerFailed=false;
  try{inputContinuations.check();planContinuations.check();if(options.consume)await withIssuedResourceOperationV1(options.control,()=>options.consume!(completedOutcome,options.control));for(const scope of compilationScopes)scope.accepting=false;for(const scope of compilationScopes){while(scope.pending.size)await Promise.allSettled([...scope.pending]);if(scope.failures.length)throw scope.failures[0];await scope.view.recheck();}return completedOutcome;}
  catch(error){consumerFailed=true;throw error;}
  finally{for(const scope of compilationScopes)scope.accepting=false;for(const scope of compilationScopes){while(scope.pending.size)await Promise.allSettled([...scope.pending]);scope.live=false;}let cleanupFailed=false,cleanupFailure:unknown;try{await inputContinuations.close();}catch(error){cleanupFailed=true;cleanupFailure=error;}if(options.retainCompilationBytes!==undefined)try{rmSync(join(generation.root,"q"),{recursive:true,force:true});}catch(error){if(!cleanupFailed){cleanupFailed=true;cleanupFailure=error;}}if (generation.kind === "fresh") for (const directory of [generation.target, generation.build])try{rmSync(directory,{recursive:true,force:true});}catch(error){if(!cleanupFailed){cleanupFailed=true;cleanupFailure=error;}}if(!consumerFailed&&cleanupFailed)throw cleanupFailure;}
  };
  };
  for(const id of nativeBuildPrerequisiteOrderV1(jobs)){const job=jobs.find(row=>row.id===id);if(!job)throw Error("Actual prerequisite job required");runJob(job);}
  await Promise.all(jobs.map(job=>completion(`job:${job.id}`).promise));while(pending.size)await Promise.all([...pending]);check();
  const requests=plan.passes.map(pass=>Object.freeze({manifest:pass.manifest,cwd:options.cwd,arguments:cargoConfigurationArgumentsV1(nativeBuildPassArgsV1(pass),undefined,generation.build),environment:Object.freeze({...options.environment,CARGO_TARGET_DIR:generation.target,CARGO_BUILD_BUILD_DIR:generation.build}),control:options.control}));
  const withPassInputs=options.withPassInputs,first=requests[0];if(!first)throw Error("Actual declared pass input required");
  let completed:undefined|(()=>Promise<NativeBuildOutcomeV1>);await planContinuations.start(consume=>withCargoCompilerInputsV1(first,withPassInputs?(request,consume)=>withPassInputs(requests,control=>consume(Object.freeze({...request,control}))):undefined,consume),async actual=>{completed=await executePasses(actual.control);});if(!completed)throw Error("Actual completed compiler continuation required");return completed;
  });
  } catch(error){primaryFailed=true;throw error;}finally {for(const scope of compilationScopes)scope.accepting=false;for(const scope of compilationScopes){while(scope.pending.size)await Promise.allSettled([...scope.pending]);scope.live=false;}let cleanupFailed=false,cleanupFailure:unknown;try{await dispose?.();}catch(error){cleanupFailed=true;cleanupFailure=error;}try{await inputContinuations.close();}catch(error){if(!cleanupFailed){cleanupFailed=true;cleanupFailure=error;}}try{if(options.retainCompilationBytes!==undefined)rmSync(join(generation.root,"q"),{recursive:true,force:true});}catch(error){if(!cleanupFailed){cleanupFailed=true;cleanupFailure=error;}}try{await planContinuations.close();}catch(error){if(!cleanupFailed){cleanupFailed=true;cleanupFailure=error;}}finally{if(deadline)clearTimeout(deadline);}if(!primaryFailed&&cleanupFailed)throw cleanupFailure; }
 });
}
