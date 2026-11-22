import {sha256} from "@noble/hashes/sha2.js";
import {blake3} from "@noble/hashes/blake3.js";
import warmGraphFixture from "../🔁️failure-churn/🧪️tests/🔣️.json" with {type:"json"};

import {registeredToolExecutableV1} from "../../../../🧰️tools/🏃️run/🟦️.ts";
import {pathToFileURL} from "node:url";
import {testPythonV1} from "../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import { requireGrantedCompileUnitsV1, phaseCompilerAdmissionControlV1, scriptCompilerAdmissionControlV1, withCompilerCapacityScopeV1 } from "../../../../🏗️build/🎟️capacity/🟦️.ts";
import { withCompilerCapacityConsumptionV1 } from "../../../../🏗️build/🎟️capacity/🪆️scope/🤲️consumption/🟦️.ts";
import { withCompilerWorkspaceLifetimeV1, withCompilerWorkspaceStageWriteV1, withCompilerWriterCustodyV1 } from "../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts";
import { currentLeaseLineageV1, heldLeasesV1, leaseLedgerSnapshotV1 } from "../../../../🔒️leases/🧭️order/🟦️.ts";
import { availableParallelism } from "node:os";
import { registerIsolatedBuildTestV1, withBuildTestInvocationV1 } from "../../../../🧪️testing/📥️invocation/🟦️.ts";
import { captureOwnedCommandV1 } from "../../../../🎛️owned-execution/🟦️.ts";
import { expect, test } from "bun:test";
import Ajv from "ajv";
import { spawnSync } from "node:child_process";
import {createJobserverV1,defaultJobserverStyleV1} from "../../🎰️jobserver/🟦️.ts";
import {withNativeBuildStageCapacityV1} from "../🎟️stage/🟦️.ts";
import {optimizePluginCoreModuleAsync} from "../../../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🪶️wasm-opt/🟦️.ts";
import {optimizeComponentCoresV1} from "../../../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🚀️commands/🟦️.ts";
import { createHash } from "node:crypto";
import { Database } from "bun:sqlite";
import { decodeCargoProvenanceV1 } from "../../🧾️receipt/🟦️.ts";
import {CargoSourceAttemptV1,CargoSourceWitnessV1} from "../../🧾️receipt/📎️source-witness/🟦️.ts";
import { formatStampProtocolV1 } from "../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧷️format-stamps/🟨️.mjs";
import { chmodSync, existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import { readUnitHistoryV1, unitHistoryPathV1 } from "../../🎟️admission/🟦️.ts";
import { lawSourceBoundaryV1, lawStoragePolicyV1, schedulerBinaries, withLawStorageV1, type LawStorageV1 } from "../../🧪️tests/🦀️cargo/🟦️.ts";
import { readTimelineV1 } from "../../📈️telemetry/🟦️.ts";
import { componentCompileUnitsV1, nativeBuildReusedUnitsV1, nativeBuildComponentStagePathV1, announcedRunIdentityV1, announcedUnitIdentityV1, componentBatchesV1, reclaimablePassFilesV1, unitFootprintKindV1, unitGraphHistoryKeyV1, componentArtifactV1, NATIVE_BUILD_DRIVER_POLICY_V1, nativeBuildGenerationKindV1, nativeBuildGenerationV1, nativeBuildPassArgsV1, nativeBuildJobWorkerBudgetV1, profileBuildModeV1, runNativeBuildDriverV1, unitGraphClosuresV1, unitGraphIdentityV1, type LinkedComponentV1, type NativeBuildOutcomeV1, type NativeBuildStageV1 } from "../🟦️.ts";
import { ConcurrentEditErrorV1, churnProtocolV1, concurrentEditOfV1, concurrentEditV1, installRestartableFailureHandlerV1 } from "../../../../../📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import { failedCompilerMessageFilesV1, failedPassConcurrentEditV1 } from "../🔁️failure-churn/🟦️.ts";
import { nativeBuildJobAttemptLogPathV1, nativeBuildJobLogNamesV1, runNativeBuildJobAttemptsV1 } from "../🔁️job-attempts/🟦️.ts";
import{admitNativeBuildPlanV1,type NativeBuildJobV1,type NativeBuildPassV1,type NativeBuildPlanV1} from "../📋️contract/🟦️.ts";

const artifacts = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!);
mkdirSync(artifacts, { recursive: true });
const header = Buffer.from(NATIVE_BUILD_DRIVER_POLICY_V1.componentHeader);

test("component compiler requests and accepts one free slot without widening its capacity", () => {
  const row = fixture.componentCompileCapacity;
  const hostDecision = { ...row.hostDecision, reason: "memory" as const };
  const { decision, request } = componentCompileUnitsV1(hostDecision);
  expect(decision).toEqual({ ...hostDecision, minimum: 1 });
  expect(request).toEqual(row.request);
  const oracle = new Ajv({ strict: true }).compile({ type: "integer", minimum: row.request.minimumParallelism, maximum: row.request.maximumParallelism });
  for (const { slots, admitted } of row.grants) {
    expect(oracle(slots)).toBe(admitted);
    if (admitted) expect(requireGrantedCompileUnitsV1(decision, slots)).toBe(slots);
    else expect(() => requireGrantedCompileUnitsV1(decision, slots)).toThrow();
  }
});

test("nested compiler budgets use reserved tokens or the shared jobserver protocol", () => {
  for (const row of fixture.workerBudgets) {
    let calls = 0;
    const reserved = (maximum: number) => { calls++; expect(maximum).toBe(row.tokens); return row.available; };
    expect(nativeBuildJobWorkerBudgetV1(row.protocol as "jobserver" | "reserved" | "exclusive", row.tokens, reserved)).toBe(row.expected);
    expect(calls).toBe(row.protocol === "jobserver" ? 0 : 1);
  }
});

/** 🎭️ A stub Cargo: logs its pass, links every planned component after a delay (`omit-last` skips the last, `hang` never answers). */
function stubCargo(root: string): { readonly command: string; readonly args: readonly string[]; readonly calls: string } {
  const calls = join(root, "calls.jsonl"), stub = join(root, "📜️script.ts");
  writeFileSync(stub, `import { appendFileSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { basename, join } from "node:path";
import { pathToFileURL } from "node:url";
const args = process.argv.slice(2), profile = args[args.indexOf("--profile") + 1], manifest = args[args.indexOf("--manifest-path") + 1];
if (["locate-project", "metadata", "update"].includes(args[0])) {
  const result = spawnSync(${JSON.stringify(registeredToolExecutableV1("cargo"))}, args, { stdio: "inherit" });
  if (result.error) throw result.error;
  process.exit(result.status ?? 1);
}
const packages = args.flatMap((arg, index) => args[index - 1] === "-p" ? [arg] : []), root = manifest.slice(0, -"/Cargo.toml".length);
const observing = process.env.STUB_MODE.startsWith("observe-") || process.env.STUB_MODE === "reclaim-error";
const announced = process.env.STUB_MODE === "observe-announced";
const beforeAnnouncement = process.env.STUB_MODE.startsWith("observe-before-announcement");
const failedSpan = process.env.STUB_MODE.startsWith("observe-failed-span:");
const consumed = announced || beforeAnnouncement || failedSpan || process.env.STUB_MODE === "observe-consumed";
if (args.includes("--unit-graph")) {
  if(process.env.STUB_MODE==="unit-hang"){appendFileSync(${JSON.stringify(join(root,"unit-calls.jsonl"))},JSON.stringify({pid:process.pid})+"\\n");await new Promise(()=>setInterval(()=>{},1000));}
  const units = observing ? packages.map(name=>({pkg_id:"path+"+pathToFileURL(join(root,name)).href+"#"+name+"@0.1.0",target:{name,kind:["cdylib"],crate_types:["cdylib"],src_path:join(root,name,"lib.rs")},platform:"wasm32-wasip2",mode:"build",features:[],dependencies:[]})) : [];
  process.stdout.write(JSON.stringify({ version: 1, units, roots: units.map((_,index)=>index) })); process.exit(0);
}
appendFileSync(${JSON.stringify(calls)}, JSON.stringify({ at: Date.now(), pid: process.pid, profile, manifest, jobserver: process.env.CARGO_MAKEFLAGS ?? null, wrapper: process.env.RUSTC_WRAPPER ?? null, unification: process.env.CARGO_RESOLVER_FEATURE_UNIFICATION ?? null }) + "\\n");
if (process.env.STUB_MODE === "hang") await new Promise(() => setInterval(() => {}, 1000));
const failing = ["error-last", "error-touch"].includes(process.env.STUB_MODE) || process.env.STUB_MODE === "reclaim-error" && basename(root) === "hub";
const linked = process.env.STUB_MODE === "omit-last" || failing ? packages.slice(0, -1) : packages;
for (const name of linked) {
  await new Promise((accept) => setTimeout(accept, ${fixture.stub.delayMs} / packages.length));
  const directory = join(process.env.CARGO_TARGET_DIR, "wasm32-wasip2", profile), wasm = join(directory, name.replaceAll("-", "_") + ".wasm"), source = join(root, name, "lib.rs");
  const original = readFileSync(source, "utf8") === ${JSON.stringify(fixture.provenanceSeal.originalSource)};
  if (failedSpan && original) {
    const scenario = ${JSON.stringify(fixture.provenanceSeal.failedSpan.cases)}.find(row=>process.env.STUB_MODE.endsWith(":"+row.name));
    if (scenario.changed) writeFileSync(source, ${JSON.stringify(fixture.provenanceSeal.editedSource)});
    process.stdout.write(JSON.stringify({reason:"compiler-message", package_id:"path+"+pathToFileURL(join(root,name)).href+"#"+name+"@0.1.0", manifest_path:join(root,name,"Cargo.toml"),target:{name,src_path:source},message:{level:"error",code:{code:"E0407"},message:"method is not a member of trait",spans:[{file_name:name+"-0.1.0/lib.rs",is_primary:scenario.primary}]}}) + "\\n");
    await new Promise(accept=>setTimeout(accept, ${fixture.provenanceSeal.failedSpan.keepGoingMilliseconds}));
    appendFileSync(${JSON.stringify(calls)}, JSON.stringify({keepGoing: true}) + "\\n");
    process.exit(101);
  }
  if (beforeAnnouncement && !original && process.env.STUB_MODE.endsWith("-unbacked")) {
    process.stderr.write("retry stopped by the fixture after the unbacked source edit\\n");
    process.exit(101);
  }
  mkdirSync(directory, { recursive: true });
  writeFileSync(wasm, Buffer.concat([Buffer.from(${JSON.stringify([...header])}), Buffer.from(name + profile)]));
  if(consumed) {
    if(announced) writeFileSync(source, ${JSON.stringify(fixture.provenanceSeal.originalSource)});
    const result = spawnSync("rustc", ["--crate-type=lib", "-Zunstable-options", "-Zchecksum-hash-algorithm=blake3", "--emit=dep-info="+wasm.slice(0,-5)+".d", source], {encoding:"utf8"});
    if(result.status!==0)throw Error("actual rustc dep-info failed: "+result.stderr+" "+result.error);
  } else if(observing) writeFileSync(wasm.replace(/\\.wasm$/u,".d"), wasm+": "+source.replaceAll(" ","\\\\ ")+"\\n");
  if (beforeAnnouncement && original) {
    writeFileSync(source, ${JSON.stringify(fixture.provenanceSeal.editedSource)});
    await new Promise(accept => setTimeout(accept, ${fixture.provenanceSeal.beforeAnnouncement.watchMilliseconds}));
  }
  process.stdout.write(JSON.stringify({ reason: "compiler-artifact", package_id: "path+" + pathToFileURL(join(root,name)).href + "#" + name + "@0.1.0", manifest_path: join(root, name, "Cargo.toml"), target: { kind: ["cdylib"], crate_types: ["cdylib"], name, src_path: source }, profile: { test: false }, features: [], filenames: [wasm], executable: null, fresh: false }) + "\\n");
}
if (failing) {
  const name = packages.at(-1), source = join(root, name, "lib.rs");
  process.stdout.write(JSON.stringify({ reason: "compiler-message", package_id: "path+file://" + root + "/" + name + "#" + name + "@0.1.0", manifest_path: join(root, name, "Cargo.toml"), target: { name }, message: { level: "error", message: "cannot find crate", rendered: "error: cannot find crate", spans: [{ file_name: name + "-0.1.0/lib.rs" }] } }) + "\\n");
  if (process.env.STUB_MODE === "error-touch") { await new Promise((accept) => setTimeout(accept, 50)); writeFileSync(source, "// edited by a peer\\n"); }
}
appendFileSync(${JSON.stringify(calls)}, JSON.stringify({ end: Date.now(), profile, manifest }) + "\\n");
if (process.env.STUB_MODE === "stderr-exit") process.stderr.write("wrapper: build broker unreachable: All pipe instances are busy.\\n");
process.exit(linked.length === packages.length && process.env.STUB_MODE !== "stderr-exit" ? 0 : 101);
`);
  return { command: process.execPath, args: [stub], calls };
}

/** 🗂️ A stub workspace whose planned packages own real, locked Cargo manifests and sources, so the driver's actual planning and Source admission run before
 * the stub Cargo links anything. */
function stubWorkspace(root: string, name: string, packages: readonly string[], profile: string): NativeBuildPassV1 {
  const directory = join(root, name);
  for (const row of packages) { mkdirSync(join(directory, row), { recursive: true }); writeFileSync(join(directory, row, "Cargo.toml"), `[package]\nname = "${row}"\nversion = "0.1.0"\nedition = "2021"\n\n[lib]\npath = "lib.rs"\ncrate-type = ["cdylib"]\n`); writeFileSync(join(directory, row, "lib.rs"), "\n"); }
  writeFileSync(join(directory, "Cargo.toml"), `[workspace]\nmembers = ${JSON.stringify(packages)}\nresolver = "2"\n\n[profile.release]\n\n[profile.prototype]\ninherits = "dev"\n`);
  expect(spawnSync(registeredToolExecutableV1("cargo"), ["generate-lockfile", "--offline", "--manifest-path", join(directory, "Cargo.toml")], { encoding: "utf8" }).status).toBe(0);
  return { id: `${profile}-${name}`, manifest: join(directory, "Cargo.toml"), packages: packages.map((row) => ({ name: row, manifest: join(directory, row, "Cargo.toml") })), target: "wasm32-wasip2", profile };
}

/** 🚀️ Runs the driver against a stub Cargo inside one original compiler reservation bound to its control (cache, target and build stores live in that
 * reservation); `inspect` reads the run's generation, whether it succeeded or failed, before the reservation retires. */
async function runStub(root: string, plan: NativeBuildPlanV1, cargo: ReturnType<typeof stubCargo>, overrides: Partial<Parameters<typeof runNativeBuildDriverV1>[0]> & { cancel?: Readonly<{ signal: AbortSignal; milliseconds: number }>; verifyStageWriters?:boolean; storageBytes?:number } = {}, mode = "ok", inspect?: (run: Readonly<{ outcome?: NativeBuildOutcomeV1; held: LawStorageV1 }>) => void | Promise<void>): Promise<NativeBuildOutcomeV1> {
  const { cancel, verifyStageWriters, storageBytes, ...driver } = overrides;
  const settled = await withBuildTestInvocationV1(async (original): Promise<Readonly<{ failed: boolean; failure?: unknown; outcome?: NativeBuildOutcomeV1 }>> => {
    const control = driver.control ?? scriptCompilerAdmissionControlV1(original, "test:component-pass", cancel?.milliseconds ?? 120000, cancel?.signal);
    try {
      return { failed: false, outcome: await withLawStorageV1(control, storageBytes ?? (mode.startsWith("observe-") ? lawStoragePolicyV1.realImageBytes : lawStoragePolicyV1.stubImageBytes), async (held, control) => {
        let outcome: NativeBuildOutcomeV1 | undefined;
        const withCapacity:NonNullable<Parameters<typeof runNativeBuildDriverV1>[0]["withCapacity"]> = compile => {
          if(verifyStageWriters){
            const [task,...ancestors]=currentLeaseLineageV1()!;
            expect(heldLeasesV1(task!,ancestors).some(lease=>lease.resource.startsWith("artifact-capture:"))).toBe(false);
          }
          const consume=(writer:typeof control)=>withCompilerCapacityConsumptionV1({ request: { kind:"exact", parallelism:Math.min(fixture.stub.tokens,availableParallelism()) }, directory:artifacts, control:writer, environment:process.env }, (facts,held)=>compile(facts,held,undefined));
          return verifyStageWriters?withCompilerWriterCustodyV1({root:held.storage.backing.root,maximumPhysicalBytes:held.storage.backing.maximumPhysicalBytes,recheck:held.storage.recheck},artifacts,control,{version:1,wait:true,mode:"shared"},consume):consume(control);
        };
        const withStageWriter:NonNullable<Parameters<typeof runNativeBuildDriverV1>[0]["withStageWriter"]> = (phase,write)=>{
          const [task,...ancestors]=currentLeaseLineageV1()!,observed=heldLeasesV1(task!,ancestors);
          const capture=observed.find(lease=>lease.directory===artifacts&&lease.mode==="exclusive"&&lease.resource.startsWith("artifact-capture:"));
          expect(capture).toBeDefined();
          expect(observed.some(lease=>lease.resource==="artifact:"+capture!.resource.slice("artifact-capture:".length))).toBe(true);
          expect(observed.some(lease=>lease.resource==="semio.compiler.host-capacity/v1"||lease.resource.startsWith("repo:compiler-writer:"))).toBe(false);
          expect(leaseLedgerSnapshotV1().holds.some(lease=>lease.resource==="semio.compiler.host-capacity/v1")).toBe(true);
          return withCompilerWorkspaceStageWriteV1(held.storage,artifacts,phase,write);
        };
        const run=()=>runNativeBuildDriverV1({ plan, storage: held.storage, sourceBoundary: lawSourceBoundaryV1, cwd: root, cacheRoot: join(held.root, "cache"), shared: { target: join(held.root, "target"), build: join(held.root, "build") }, generation: "warm", leaseDirectory: artifacts, environment: { ...process.env, ...held.environment, CARGO_TARGET_DIR: join(held.root, "target"), STUB_MODE: mode }, control, tokens: fixture.stub.tokens, withCapacity, ...(verifyStageWriters?{withStageWriter}:{}), physicalBytes: fixture.stub.physicalGiB * 1024 ** 3, storageReserveBytes: lawStoragePolicyV1.reserveBytes, scheduler: process.execPath, cargo, log: () => undefined, ...driver });
        try { outcome = await (verifyStageWriters?withCompilerWorkspaceLifetimeV1(held.storage,control,run):run()); return outcome; }
        finally { await inspect?.({ outcome, held }); }
      }, original) };
    } catch (failure) { return { failed: true, failure }; }
  });
  if (settled.failed) throw settled.failure;
  return settled.outcome!;
}

test("the native build plan is schema-first, agrees with Ajv, and refuses duplicate passes and packages", () => {
  const validate = new Ajv({ strict: true, allowUnionTypes: true }).compile(schema);
  for (const plan of fixture.plans.valid) { expect(validate(plan), JSON.stringify(validate.errors)).toBe(true); expect(admitNativeBuildPlanV1(plan)).toEqual(plan as never); }
  for (const plan of fixture.plans.invalid) { expect(validate(plan), JSON.stringify(plan)).toBe(false); expect(() => admitNativeBuildPlanV1(plan)).toThrow(/Invalid native build plan/u); }
  for (const plan of fixture.plans.refused) { expect(validate(plan)).toBe(true); expect(() => admitNativeBuildPlanV1(plan)).toThrow(/duplicate/u); }
});

test("unit-graph identities, closures and announced identities follow the language-agnostic vectors", () => {
  const vector = JSON.parse(JSON.stringify(fixture.unitGraph).replaceAll("path+file:///w", "path+" + pathToFileURL(resolve("/w")).href));
  expect(vector.graph.units.map(unitGraphIdentityV1)).toEqual(vector.identities);
  const closures = unitGraphClosuresV1(vector.graph, vector.pass);
  expect(Object.fromEntries([...closures].map(([name, closure]) => [name, [...closure].sort()]))).toEqual(vector.closures);
  for (const row of vector.announced) expect(announcedUnitIdentityV1(row.message, row.target)).toBe(row.identity);
  for (const row of vector.runs) expect(announcedRunIdentityV1(row.packageId, row.outDir, row.target)).toBe(row.identity);
  expect(reclaimablePassFilesV1(vector.reclaim.messages, vector.reclaim.target, new Set(vector.reclaim.live))).toEqual(vector.reclaim.files);
  expect(reclaimablePassFilesV1(vector.reclaim.messages, vector.reclaim.target, new Set(vector.reclaim.messages.map((message: any) => announcedUnitIdentityV1(message, vector.reclaim.target))))).toEqual([]);
});

test("generations: the environment selects warm or fresh; fresh is a new one-letter-store directory, warm compiles into the shared stores", () => {
  for (const row of fixture.generations.environments) expect(nativeBuildGenerationKindV1(row.environment)).toBe(row.kind as never);
  for (const value of fixture.generations.invalid) expect(() => nativeBuildGenerationKindV1({ SEMIO_NATIVE_BUILD_GENERATION: value })).toThrow();
  const root = mkdtempSync(join(artifacts, "g-")), shared = { target: join(root, "shared-t"), build: join(root, "shared-b") };
  const first = nativeBuildGenerationV1("fresh", root, shared), second = nativeBuildGenerationV1("fresh", root, shared), warm = nativeBuildGenerationV1("warm", root, shared);
  expect(first.root).not.toBe(second.root);
  for (const generation of [first, second]) {
    expect(generation.root.slice(root.length + 1).startsWith(NATIVE_BUILD_DRIVER_POLICY_V1.generationPrefix)).toBe(true);
    for (const [store, name] of Object.entries(NATIVE_BUILD_DRIVER_POLICY_V1.stores)) expect((generation as Record<string, unknown>)[store]).toBe(join(generation.root, name));
  }
  expect([warm.target, warm.build]).toEqual([shared.target, shared.build]);
  const another = nativeBuildGenerationV1("warm", root, shared);
  expect(another.root).not.toBe(warm.root);
  for (const run of [warm, another]) { expect(dirname(run.root)).toBe(join(root, NATIVE_BUILD_DRIVER_POLICY_V1.warmRoot)); expect(run.report).toBe(join(run.root, NATIVE_BUILD_DRIVER_POLICY_V1.stores.report)); }
  expect(warm.stages).toBe(join(root, "stages"));
  expect(first.stages).toBe(join(first.root, "s"));
});

test("component batches follow the recorded p95 output footprint, estimate unmeasured units by kind (p95 of the kind, else its upper bound, also on a cold empty history) and keep a fitting pass whole", () => {
  const vector = JSON.parse(JSON.stringify(fixture.batches).replaceAll("path+file:///w", "path+" + pathToFileURL(resolve("/w")).href));
  expect(vector.graph.units.map((unit: any) => unitGraphHistoryKeyV1(unit, vector.pass.profile))).toEqual(vector.keys);
  for (const row of vector.kinds) expect(unitFootprintKindV1(row.crate, row.types)).toBe(row.kind);
  for (const row of vector.cases) expect(componentBatchesV1(vector.pass, vector.graph, new Map(Object.entries(vector.histories[row.history])) as any, row.budgetBytes, vector.footprints).map((batch) => ({ id: batch.pass.id, packages: batch.pass.packages.map((item) => item.name), bytes: batch.bytes, measured: batch.measured }))).toEqual(row.expected);
});

test("pass arguments and component detection follow the vectors", () => {
  expect(nativeBuildPassArgsV1(fixture.args.pass)).toEqual(fixture.args.args);
  for (const row of fixture.artifacts) expect(componentArtifactV1(row.message, fixture.args.pass, row.targetDirectory)).toEqual(row.expected);
});

test("the driver streams each component the moment Cargo links it, runs profiles in parallel lanes and workspaces of one profile in sequence, under one jobserver", async () => {
  const root = mkdtempSync(join(artifacts, "s-")), cargo = stubCargo(root);
  const passes = [stubWorkspace(root, "hub", ["comp-a", "comp-b"], "release"), stubWorkspace(root, "ext", ["comp-c"], "release"), stubWorkspace(root, "hub2", ["comp-d"], "prototype")];
  await runStub(root, { version: 1, passes }, cargo, {verifyStageWriters:true}, "ok", ({ outcome: run }) => {
  if (!run) return;
  const outcome = run;
  expect([...outcome.staged].sort()).toEqual(["comp-a:release", "comp-b:release", "comp-c:release", "comp-d:prototype"]);
  for (const pass of passes) for (const row of pass.packages) expect([...readFileSync(nativeBuildComponentStagePathV1(outcome.generation,pass,row)).subarray(0, 8)]).toEqual([...header]);
  const calls = readFileSync(cargo.calls, "utf8").trim().split("\n").map((line) => JSON.parse(line));
  const span = (manifest: string) => ({ start: calls.find((row) => row.at && row.manifest === manifest).at, end: calls.find((row) => row.end && row.manifest === manifest).end });
  expect(span(passes[1]!.manifest).start).toBeGreaterThanOrEqual(span(passes[0]!.manifest).end);
  expect(span(passes[2]!.manifest).start).toBeLessThan(span(passes[0]!.manifest).end);
  for (const row of calls.filter((call) => call.at)) { expect(row.jobserver).toMatch(/--jobserver-auth=(?:fifo:|semio-jobserver-)/u); expect(row.wrapper).toBe(process.execPath); expect(row.unification).toBe(NATIVE_BUILD_DRIVER_POLICY_V1.featureUnification); }
  const units = JSON.parse(readFileSync(join(outcome.generation.report, "units.json"), "utf8"));
  const stageA = units.stages.find((stage: any) => stage.component === "comp-a" && stage.stage === "stage");
  expect(stageA.startMs).toBeLessThan(span(passes[0]!.manifest).end);
  expect(units.stages.filter((stage: any) => stage.stage === "provenance").length).toBe(passes.length);
  const provenance = readdirSync(join(outcome.generation.report, NATIVE_BUILD_DRIVER_POLICY_V1.provenanceDirectory));
  expect(provenance.filter((name) => name.startsWith("cargo-unit-provenance-"))).toHaveLength(passes.length);
  expect(provenance).toEqual(expect.arrayContaining([NATIVE_BUILD_DRIVER_POLICY_V1.provenanceLeavesFile, NATIVE_BUILD_DRIVER_POLICY_V1.provenanceReadsFile]));
  const reads = JSON.parse(readFileSync(join(outcome.generation.report, NATIVE_BUILD_DRIVER_POLICY_V1.provenanceDirectory, NATIVE_BUILD_DRIVER_POLICY_V1.provenanceReadsFile), "utf8"));
  expect(reads, JSON.stringify(reads.excess)).toMatchObject({ closed: true, maximumReads: 2 });
  expect(readFileSync(outcome.report, "utf8")).toContain("## Critical path");
  });
}, 120_000);

for(const correction of warmGraphFixture.secondSourceCorrections) registerIsolatedBuildTestV1(import.meta.path,correction.name,{bodyMilliseconds:120_000,scope:{kind:"body"}},async checkpoint=>{
  installRestartableFailureHandlerV1(() => undefined);
  const vector=warmGraphFixture.driverReplacement,directSecond=correction.source==="diagnostic",growth=correction.source==="growth";
  const root=mkdtempSync(join(artifacts,"wr-")), original=stubWorkspace(root,"app",[vector.application,vector.secondary,vector.prior,vector.replacement],"release");
  const pass={...original,packages:original.packages.slice(0,2)}, workspace=dirname(pass.manifest);
  const path=(name:string,leaf:string)=>join(workspace,name,leaf);
  const manifest=(name:string,dependency:string)=>[
    "[package]",'name = "'+name+'"','version = "0.1.0"','edition = "2021"',"[lib]",'path = "lib.rs"','crate-type = ["cdylib"]',
    "[dependencies]",...([vector.prior,vector.replacement].map(name=>name+' = { path = "../'+name+'", optional = true }')),
    "[features]",'default = ["dep:'+dependency+'"'+(growth&&dependency===vector.replacement?', "dep:'+vector.prior+'"':'')+']',"",
  ].join("\n");
  for (const name of [vector.prior,vector.replacement]) {
    const file=path(name,"Cargo.toml");
    writeFileSync(file,readFileSync(file,"utf8").replace('"cdylib"','"rlib"'));
    writeFileSync(path(name,"lib.rs"),vector.source);
  }
  for (const row of pass.packages) writeFileSync(row.manifest,manifest(row.name,growth?vector.replacement:vector.prior));
  expect(spawnSync(registeredToolExecutableV1("cargo"),["generate-lockfile","--offline","--manifest-path",pass.manifest],{encoding:"utf8"}).status).toBe(0);
  const command=join(root,"warm-graph.ts"),calls=join(root,"calls.jsonl"),target=pass.target;
  if(directSecond)writeFileSync(path(vector.application,"lib.rs"),vector.source);
  const config={directSecond,growth,cargo:registeredToolExecutableV1("cargo"),secondCode:correction.code,vector,workspace,calls,target,priorChecksum:Buffer.from(blake3(Buffer.from(vector.source))).toString("hex"),manifests:pass.packages.map(row=>[row.manifest,manifest(row.name,vector.replacement)]),header:[...header]};
  writeFileSync(command, String.raw`import {appendFileSync,mkdirSync,readFileSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import {pathToFileURL} from "node:url";
const {directSecond,growth,cargo,secondCode,vector:v,workspace:w,calls,target,priorChecksum,manifests,header}=${JSON.stringify(config)};
if(process.argv[2]!=="build"){
 const result=spawnSync(cargo,process.argv.slice(2),{stdio:"inherit"});
 if(result.error)throw result.error;
 process.exit(result.status??1);
}
const path=(name,leaf)=>join(w,name,leaf),app=path(v.application,"Cargo.toml");
const selection=readFileSync(app,"utf8"),larger=growth&&readFileSync(path(v.prior,"lib.rs"),"utf8")!==v.source,first=!larger&&(growth||selection.includes("dep:"+v.prior)),child=first?v.prior:v.replacement;
const unit=(name,kind,features,dependencies)=>({pkg_id:"path+"+pathToFileURL(join(w,name)).href+"#"+name+"@0.1.0",target:{name,kind:[kind],crate_types:[kind],src_path:path(name,"lib.rs")},platform:target,mode:"build",features,dependencies:dependencies.map(index=>({index}))});
const edges=larger?[1,3]:[1],units=[unit(v.application,"cdylib",["default"],edges),unit(child,"rlib",[],[]),unit(v.secondary,"cdylib",["default"],edges)];
if(larger)units.push(unit(v.prior,"rlib",[],[]));
if(process.argv.includes("--unit-graph")){console.log(JSON.stringify({version:1,units,roots:[0,2]}));process.exit(0);}
const directory=join(process.env.CARGO_TARGET_DIR,target,"release");
appendFileSync(calls,JSON.stringify({child,directory,build:process.env.CARGO_BUILD_BUILD_DIR})+"\n");
mkdirSync(directory,{recursive:true});
const announce=(unit,file,checksum)=>{
 const source=unit.target.src_path,dep=file.replace(/\.(rlib|wasm)$/u,".d");
 writeFileSync(file,Buffer.concat([Buffer.from(header),Buffer.from(unit.target.name)]));
 writeFileSync(dep,file+": "+source.replaceAll(" ","\\ ")+"\n"+(checksum?"# checksum:blake3="+checksum+" file_len:"+Buffer.byteLength(v.source)+" "+source+"\n":""));
 console.log(JSON.stringify({reason:"compiler-artifact",package_id:unit.pkg_id,manifest_path:path(unit.target.name,"Cargo.toml"),target:unit.target,features:unit.features,profile:{test:false},filenames:[file],executable:null,fresh:false}));
};
const diagnostic=directSecond&&!first,edited=path(diagnostic?v.application:child,"lib.rs");
const changed=readFileSync(edited,"utf8")!==v.source;
announce(units[1],join(directory,"lib"+child+"-abc.rlib"),changed?undefined:priorChecksum);
if(!changed){
 writeFileSync(edited,v.editedSource);
 if(first&&!growth) for (const [path, manifest] of manifests) writeFileSync(path,manifest);
 console.log(JSON.stringify({reason:"compiler-message",package_id:units[0].pkg_id,manifest_path:app,target:units[0].target,message:{level:"error",code:{code:first?"E0599":secondCode},message:"selected source changed",spans:diagnostic?[{file_name:edited,is_primary:true}]:[]}}));
 process.exit(v.originalExit);
}
for (const index of [0,2]) announce(units[index],join(directory,units[index].target.name.replaceAll("-","_")+".wasm"),undefined);
process.exit(v.finalExit);`);
  if(growth&&"heldBytes" in correction&&"replacementBytes" in correction){
    let report="";
    const retry=runStub(root,{version:1,passes:[pass]},{command:process.execPath,args:[command],calls},
      {storageBytes:vector.storageBytes},"observe-linked",({held})=>{
        const generations=join(held.root,"cache",NATIVE_BUILD_DRIVER_POLICY_V1.warmRoot);
        const run=join(generations,readdirSync(generations)[0]!);
        report=readFileSync(join(run,NATIVE_BUILD_DRIVER_POLICY_V1.stores.report,"report.md"),"utf8");
        expect(existsSync(join(run,NATIVE_BUILD_DRIVER_POLICY_V1.stores.report,NATIVE_BUILD_DRIVER_POLICY_V1.provenanceDirectory,"cargo-unit-provenance-"+pass.id+".json"))).toBe(false);
      });
    const failure=await retry.catch(error=>error);
    expect(failure).toBeInstanceOf(ConcurrentEditErrorV1);
    const message=String(failure);
    const bounds=message.match(/held ([0-9]+) bytes, replacement ([0-9]+) bytes, admissible (-?[0-9]+) bytes, available (-?[0-9]+) bytes, floor ([0-9]+) bytes/u);
    expect(bounds?.slice(1,3).map(Number)).toEqual([correction.heldBytes,correction.replacementBytes]);
    const [,held,,admissible,available,floor]=bounds!;
    using independent=new Database(":memory:");
    const predicted=independent.query("select min(?1,?2-?3) admissible").get(Number(held),Number(available),Number(floor));
    expect(predicted).toEqual({admissible:Number(admissible)});
    expect(report).toContain(bounds![0]);
    expect(readFileSync(calls,"utf8").trim().split("\\n").map(line=>JSON.parse(line).child)).toEqual([vector.prior]);
    checkpoint();
    return;
  }
  const seen:string[]=[];
  const outcome=await runStub(root,{version:1,passes:[pass]},{command:process.execPath,args:[command],calls}, {
    log:line=>seen.push(line),
    storageBytes:vector.storageBytes,
    stages:()=>[{name:"inspect",after:[],capacity:"io",async run(context){expect((await context.provenance()).units).toBe(vector.artifacts);}}],
  },"observe-linked",({outcome})=>{
    if(!outcome)return;
    const receipt=readFileSync(join(outcome.generation.logs,"unit-graph-"+pass.id+"-retry-2.json"),"utf8");
    expect(JSON.parse(receipt).units.map((unit:{target:{name:string}})=>unit.target.name)).toEqual([vector.application,vector.replacement,vector.secondary]);
    const directory=join(outcome.generation.report,NATIVE_BUILD_DRIVER_POLICY_V1.provenanceDirectory);
    const file=readdirSync(directory).find(name=>name.startsWith("cargo-unit-provenance-"))!;
    const sources=decodeCargoProvenanceV1(readFileSync(join(directory,file),"utf8")).units.flatMap(unit=>unit.inputs);
    const paths=sources.map(input=>input.path);
    if(directSecond)expect(sources.find(input=>input.path===path(vector.application,"lib.rs"))?.sha256).toBe(Buffer.from(sha256(Buffer.from(vector.editedSource))).toString("hex"));
    expect(paths).toContain(path(vector.replacement,"lib.rs"));
    expect(paths).not.toContain(path(vector.prior,"lib.rs"));
  });
  expect([...outcome.staged].sort()).toEqual([vector.application+":release",vector.secondary+":release"]);
  const rows=readFileSync(calls,"utf8").trim().split("\n").map(line=>JSON.parse(line));
  expect(rows.map(row=>row.child)).toEqual([vector.prior,vector.replacement,vector.replacement]);
  expect(new Set(rows.map(row=>row.directory)).size).toBe(1);
  expect(new Set(rows.map(row=>row.build)).size).toBe(1);
  expect(seen.filter(line=>line.includes("recompiling in the held lane"))).toHaveLength(2);
  checkpoint();
},correction.source==="growth"?/^Replacement Cargo unit graph does not fit the held component plan \(held /u:undefined);

test("mixed compiler claims agree with the language-neutral retry oracle",()=>{
  const {cases,retryLimit,sourceRevisions}=fixture.mixedWarmDepInfo;
  const oracle=spawnSync(testPythonV1()??"python3",["-c",String.raw`import json,sys
data=json.load(sys.stdin)
def run(case):
    seen=set()
    for turn,claims in enumerate(case["claims"],1):
        proof=frozenset(claims)
        if len(proof)==1:
            return [turn,turn]
        if proof in seen or len(seen)==data["retryLimit"]:
            return [turn,None]
        seen.add(proof)
print(json.dumps([run(case) for case in data["cases"]]))
`],{input:JSON.stringify({cases,retryLimit}),encoding:"utf8"});
  expect(oracle.status,oracle.stderr).toBe(0);
  expect(JSON.parse(oracle.stdout)).toEqual(cases.map(({attempts,staged})=>[attempts,staged]));
  const source=resolve("source.rs"),external=resolve("external.rs");
  const claim=(path:string,index:number)=>({path,blake3:Buffer.from(blake3(Buffer.from(sourceRevisions[index]!))).toString("hex"),length:Buffer.byteLength(sourceRevisions[index]!)});
  for(const scenario of cases){
    const admitted=new Map([[source,"admitted"]]),seen=new Set<string>();
    for(const [attempt,[before,after]] of scenario.claims.entries()){
      const proof=new CargoSourceAttemptV1(admitted);
      proof.record("first",claim(external,0));
      proof.record("second",claim(external,1));
      proof.record("first",claim(source,before!));
      proof.record("first",claim(source,after!));
      expect(proof.conflict).toBeUndefined();
      proof.record("second",claim(source,after!));
      const drift=proof.conflict;
      if(!drift){expect([attempt+1,attempt+1]).toEqual([scenario.attempts,scenario.staged]);break;}
      expect(drift).toMatchObject({path:source});
      const key=JSON.stringify(drift);
      if(seen.has(key)||seen.size===retryLimit){expect([attempt+1,null]).toEqual([scenario.attempts,scenario.staged]);break;}
      seen.add(key);
    }
  }
});

for(const scenario of [...fixture.mixedWarmDepInfo.cases,...fixture.mixedWarmDepInfo.missingWitnessCases]) registerIsolatedBuildTestV1(import.meta.path,scenario.name,{bodyMilliseconds:120_000,scope:{kind:"body"}},async checkpoint=>{
  installRestartableFailureHandlerV1(() => undefined);
  const vector=fixture.mixedWarmDepInfo;
  const root=mkdtempSync(join(artifacts,"wm-")),workspace=stubWorkspace(root,"app",[vector.application,vector.dependency],"release");
  const [application,dependency]=workspace.packages;
  const pass={...workspace,packages:[application!]};
  writeFileSync(application!.manifest,readFileSync(application!.manifest,"utf8")+`\n[dependencies]\n${vector.dependency} = { path = "../${vector.dependency}" }\n`);
  writeFileSync(dependency!.manifest,readFileSync(dependency!.manifest,"utf8").replace('"cdylib"','"rlib"'));
  writeFileSync(join(dirname(dependency!.manifest),"lib.rs"),`include!("../${vector.application}/lib.rs");\n`);
  const source=join(dirname(application!.manifest),"lib.rs");
  const initialRevision="initialRevision" in scenario?scenario.initialRevision:0;
  const liveRevisions="liveRevisions" in scenario?scenario.liveRevisions:undefined;
  writeFileSync(source,vector.sourceRevisions[initialRevision]!);
  expect(spawnSync(registeredToolExecutableV1("cargo"),["generate-lockfile","--offline","--manifest-path",pass.manifest],{encoding:"utf8"}).status).toBe(0);
  const command=join(root,"mixed.ts"),calls=join(root,"mixed.jsonl");
  writeFileSync(command,String.raw`import {appendFileSync,existsSync,mkdirSync,readFileSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {dirname,join} from "node:path";
import {pathToFileURL} from "node:url";
const {cargo,calls,source,units:packages,target,profile,header,claims,revisions,liveRevisions}=${JSON.stringify({
    cargo:registeredToolExecutableV1("cargo"),calls,source,units:[dependency,application],target:pass.target,profile:pass.profile,header:[...header],claims:scenario.claims,liveRevisions,
    revisions:vector.sourceRevisions.map(body=>({body,hash:Buffer.from(blake3(Buffer.from(body))).toString("hex"),length:Buffer.byteLength(body)})),
  })};
const args=process.argv.slice(2);
if(args[0]!=="build"){
  const result=spawnSync(cargo,args,{stdio:"inherit"});
  if(result.error)throw result.error;
  process.exit(result.status??1);
}
const units=packages.map(({name,manifest},index)=>{
  const pkg_id="path+"+pathToFileURL(dirname(manifest)).href+"#"+name+"@0.1.0";
  return {pkg_id,target:{name,kind:[index===0?"lib":"cdylib"],crate_types:[index===0?"rlib":"cdylib"],src_path:join(dirname(manifest),"lib.rs")},platform:target,mode:"build",features:[],dependencies:index===0?[]:[{index:0}]};
});
if(process.argv.includes("--unit-graph")){console.log(JSON.stringify({version:1,units,roots:[1]}));process.exit(0);}
const attempt=existsSync(calls)?readFileSync(calls,"utf8").trim().split("\n").length:0;
const directory=join(process.env.CARGO_TARGET_DIR,target,profile);
appendFileSync(calls,JSON.stringify({directory,build:process.env.CARGO_BUILD_BUILD_DIR})+"\n");
mkdirSync(directory,{recursive:true});
writeFileSync(source,revisions[claims[attempt][1]].body);
for(const [index,unit] of units.entries()){
  const file=join(directory,index===0?"libmixed_dep-abc.rlib":"mixed_app.wasm");
  const {hash,length}=revisions[claims[attempt][index]];
  const dep=file.replace(/\.(?:rlib|wasm)$/u,".d"),inputs=[...new Set([unit.target.src_path,source])];
  writeFileSync(file,Buffer.concat([Buffer.from(header),Buffer.from([attempt+1])]));
  writeFileSync(dep,file+": "+inputs.map(path=>path.replaceAll(" ","\\ ")).join(" ")+"\n# checksum:blake3="+hash+" file_len:"+length+" "+source+"\n");
   if(index===0&&liveRevisions)writeFileSync(source,revisions[liveRevisions[attempt]].body);
  console.log(JSON.stringify({reason:"compiler-artifact",package_id:unit.pkg_id,manifest_path:packages[index].manifest,target:unit.target,features:[],profile:{test:false},filenames:[file],executable:null,fresh:attempt>0||index===0}));
}
`);
  const planned:number[]=[],staged:number[]=[],log:string[]=[];
  const result=await runStub(root,{version:1,passes:[pass]},{command:process.execPath,args:[command],calls},{
    storageBytes:warmGraphFixture.driverReplacement.storageBytes,
    log:line=>log.push(line),
    stages:()=>{
      planned.push(readFileSync(calls,"utf8").trim().split("\n").length);
      return [{name:"inspect",after:[],capacity:"io",async run(context){
        expect((await context.provenance()).units).toBe(2);
      }}];
    },
  },"observe-mixed",({outcome})=>{
    if(!outcome)return;
    const stage=nativeBuildComponentStagePathV1(outcome.generation,pass,application!);
    if(existsSync(stage))staged.push(readFileSync(stage).at(-1)!);
  }).then(outcome=>({outcome}),error=>({error}));
  if(!existsSync(calls)&&"error"in result)throw result.error;
  const attempts=readFileSync(calls,"utf8").trim().split("\n").map(line=>JSON.parse(line));
  expect(attempts).toHaveLength(scenario.attempts);
  expect(new Set(attempts.map(row=>row.directory)).size).toBe(1);
  expect(new Set(attempts.map(row=>row.build)).size).toBe(1);
  expect(log.filter(line=>line.includes("recompiling in the held lane"))).toHaveLength(scenario.attempts-1);
  expect(planned).toEqual(scenario.staged===null?[]:[scenario.staged]);
  expect(staged).toEqual(scenario.staged===null?[]:[scenario.staged]);
  if(scenario.staged===null){
    expect("error"in result&&concurrentEditOfV1(result.error)).toMatchObject({reason:"bytes",path:source});
  }else expect(result).toHaveProperty("outcome");
  checkpoint();
},scenario.staged===null?/^Compiler consumed inconsistent source versions \(BLAKE3 and byte lengths /u:undefined);

registerIsolatedBuildTestV1(import.meta.path,"an optimizer waits without holding partial slots and preserves real Binaryen output",{bodyMilliseconds:120_000,scope:{kind:"body"}},async checkpoint=>{
  const vector=fixture.optimizerParallel,root=mkdtempSync(join(artifacts,"o-")),input=join(root,"core.wat");
  const outputs=[join(root,"parallel.wasm"),join(root,"single.wasm")];
  writeFileSync(input,vector.source);
  await withBuildTestInvocationV1(async original=>{
    const control=scriptCompilerAdmissionControlV1(original,"test:optimizer-parallel",120000);
    const pool=await createJobserverV1({version:1,tokens:vector.pool,style:defaultJobserverStyleV1()},join(root,"j"));
    const directory=artifacts,environment={...process.env};
    const audit=new Database(":memory:");
    audit.exec("create table running (kind text primary key, granted integer, held integer, external integer)");
    try{
      await withCompilerCapacityScopeV1({directory,control,environment,request:{kind:"exact",parallelism:vector.pool}},async(_,control)=>{
        const options={directory,control,environment,jobserver:pool};
        const blocker=await pool.acquire(control.signal),release=await pool.acquire(control.signal);
        let notify!:()=>void,entered=false;
        const waiting=new Promise<void>(resolve=>{notify=resolve;});
        const optimize=(cores:number,output:string,control:Parameters<typeof withNativeBuildStageCapacityV1>[0]["control"])=>optimizePluginCoreModuleAsync(input,output,{repoRoot:process.cwd(),preview2VendorDir:"",cores,control,environment});
        const parallel=withNativeBuildStageCapacityV1({...options,capacity:"parallel",ready:async()=>{notify();return true;}},async facts=>{
          entered=true;
          audit.query("insert into running values (?,?,?,?)").run("parallel",facts.parallelism,pool.held(),1);
          await optimize(facts.parallelism,outputs[0]!,facts.control);
        });
        await waiting;await control.yieldContinuation();
        expect(entered).toBe(false);
        expect(pool.held()).toBe(vector.blocked);
        const competing=await pool.acquire(control.signal);
        expect(pool.held()).toBe(vector.pool);
        competing.release();
        expect(entered).toBe(false);
        release.release();
        await parallel;
        expect(pool.held()).toBe(1);
        const last=await pool.acquire(control.signal);
        const stop=new AbortController(),cancelled=Error("optimizer no longer wanted");
        const withdrawn=pool.acquireAvailable(vector.parallel,30_000,stop.signal);
        stop.abort(cancelled);
        await expect(withdrawn).rejects.toBe(cancelled);
        expect(pool.held()).toBe(vector.blocked);
        const settled=await pool.acquireAvailable(vector.parallel,1,control.signal);
        expect(settled).toHaveLength(vector.single);
        for(const slot of settled)slot.release();
        last.release();
        blocker.release();
        expect(pool.held()).toBe(0);
        await withNativeBuildStageCapacityV1({...options,capacity:"single",ready:async()=>true},async facts=>{
          audit.query("insert into running values (?,?,?,?)").run("single",facts.parallelism,pool.held(),0);
          await optimize(facts.parallelism,outputs[1]!,facts.control);
        });
        expect(pool.held()).toBe(0);
      });
      expect(audit.query("select kind,granted from running where held = granted + external and held <= ? order by kind").all(vector.pool)).toEqual([
        {kind:"parallel",granted:vector.parallel},{kind:"single",granted:vector.single},
      ]);
      const parallel=readFileSync(outputs[0]!),single=readFileSync(outputs[1]!);
      expect(parallel.equals(single)).toBe(true);
      const instance=new WebAssembly.Instance(new WebAssembly.Module(parallel));
      expect((instance.exports.result as ()=>number)()).toBe(vector.result);
    }finally{
      audit.close();
      pool.close();
    }
  });
  checkpoint();
});

const compilerSealMutationError = /Current physical bytes changed before transfer/u;

for (const scenario of fixture.provenanceSeal.beforeAnnouncement.cases) {
  const name = `the driver seals compiler provenance across a pre-announcement source edit (${scenario.retained ? "retained" : "unbacked"})`;
  const refusal = /retry stopped by the fixture after the unbacked source edit/u;
  registerIsolatedBuildTestV1(import.meta.path,name,{bodyMilliseconds:120_000,scope:{kind:"body"}},async checkpoint=>{
    const root=mkdtempSync(join(artifacts,"z-")),cargo=stubCargo(root),pass=stubWorkspace(root,"app",["seal-a"],"release");
    const source=join(dirname(pass.packages[0]!.manifest),"lib.rs"),expected=fixture.provenanceSeal;
    writeFileSync(source,expected.originalSource);
    const log:string[]=[];
    const run=runStub(root,{version:1,passes:[pass]},cargo,{
      log:line=>log.push(line),
      ...(scenario.retained?{}:{retainCompilationBytes:1_048_576,maximumObservationBytes:1_048_576,consume:async()=>{throw Error("unbacked fixture must not publish");}}),
    },scenario.retained?"observe-before-announcement":"observe-before-announcement-unbacked",({outcome})=>{
      if(!outcome)return;
      const receipts=join(outcome.generation.report,NATIVE_BUILD_DRIVER_POLICY_V1.provenanceDirectory);
      const files=readdirSync(receipts).filter(name=>name.startsWith("cargo-unit-provenance-"));
      expect(files).toHaveLength(1);
      const units=decodeCargoProvenanceV1(readFileSync(join(receipts,files[0]!),"utf8")).units;
      expect(units.flatMap(unit=>unit.inputs).find(row=>row.path===source)?.sha256)
        .toBe(createHash("sha256").update(expected.originalSource).digest("hex"));
      expect(units.flatMap(unit=>unit.depInfo).flatMap(row=>row.checksums).find(row=>resolve(row.path)===source))
        .toMatchObject({blake3:Buffer.from(blake3(Buffer.from(expected.originalSource))).toString("hex"),length:Buffer.byteLength(expected.originalSource)});
    });
    if(scenario.retained)expect((await run).staged).toEqual(["seal-a:release"]);
    else await expect(run).rejects.toThrow(refusal);
    const calls=readFileSync(cargo.calls,"utf8").trim().split("\n").map(line=>JSON.parse(line));
    expect(calls.filter(row=>row.pid)).toHaveLength(scenario.attempts);
    expect(calls.filter(row=>row.end)).toHaveLength(scenario.retained?1:0);
    expect(log.filter(line=>line.includes("recompiling in the held lane"))).toHaveLength(scenario.attempts-1);
    expect(readFileSync(source,"utf8")).toBe(expected.editedSource);
    checkpoint();
  },scenario.retained?undefined:refusal);
}

for (const scenario of fixture.provenanceSeal.failedSpan.cases) {
  registerIsolatedBuildTestV1(import.meta.path,"the driver recaptures an authenticated failed span ("+scenario.name+")",{bodyMilliseconds:120_000,scope:{kind:"body"}},async checkpoint=>{
    const root=mkdtempSync(join(artifacts,"e-")),cargo=stubCargo(root),pass=stubWorkspace(root,"app",["seal-a"],"release");
    const source=join(dirname(pass.packages[0]!.manifest),"lib.rs"),expected=fixture.provenanceSeal;
    writeFileSync(source,expected.originalSource);
    const run=runStub(root,{version:1,passes:[pass]},cargo,{},"observe-failed-span:"+scenario.name,({outcome})=>{
      if(!outcome)return;
      const receipts=join(outcome.generation.report,NATIVE_BUILD_DRIVER_POLICY_V1.provenanceDirectory);
      const record=readdirSync(receipts).find(name=>name.startsWith("cargo-unit-provenance-"));
      expect(record).toBeDefined();
      const units=decodeCargoProvenanceV1(readFileSync(join(receipts,record!),"utf8")).units;
      expect(units.flatMap(unit=>unit.depInfo).flatMap(row=>row.checksums).find(row=>resolve(row.path)===source))
        .toMatchObject({blake3:Buffer.from(blake3(Buffer.from(expected.editedSource))).toString("hex"),length:Buffer.byteLength(expected.editedSource)});
    });
    if(scenario.changed)expect((await run).staged).toEqual(["seal-a:release"]);
    else await expect(run).rejects.toThrow(/^Native build failed:/u);
    const calls=readFileSync(cargo.calls,"utf8").trim().split("\n").map(line=>JSON.parse(line));
    expect(calls.filter(row=>row.pid)).toHaveLength(scenario.attempts);
    expect(calls.some(row=>row.keepGoing)).toBe(!scenario.retired);
    checkpoint();
  },scenario.changed?undefined:/^Native build failed:/u);
}

for (const scenario of fixture.provenanceSeal.cases) {
  const packages = scenario.packages ?? ["seal-a"];
  const name = "the driver seals compiler provenance while independent stages run (" + scenario.mutation + " " + scenario.timing + (packages.length > 1 ? " pair" : "") + ")";
  registerIsolatedBuildTestV1(import.meta.path,name,{bodyMilliseconds:120_000,scope:{kind:"body"}},async checkpoint=>{
    if (!scenario.publishesReceipt) installRestartableFailureHandlerV1(() => undefined);
    const expected=fixture.provenanceSeal, announced=scenario.timing==="after-announcement", withChecksum=announced||scenario.timing==="after-compiler-checksum";
    const root=mkdtempSync(join(artifacts,"z-")), cargo=stubCargo(root), pass=stubWorkspace(root,"app",packages,"release");
    const sources=pass.packages.map(row=>join(dirname(row.manifest),"lib.rs"));
    for(const source of sources)writeFileSync(source,announced?expected.admittedSource:expected.originalSource);
    let signalSeal!:()=>void, sealed=false, edited=false;
    const seal=new Promise<void>(resolve=>{signalSeal=resolve;});
    const run=runStub(root,{version:1,passes:[pass]},cargo,{
      verifyStageWriters:true,
      log(line) {
        if(line==="[native-build] compiler provenance sealed"){sealed=true;signalSeal();}
      },
      stages(linked) {
        return [{name:"independent",after:[],capacity:"io",async run(context){
          const provenance=await context.provenance();
          expect(provenance.units).toBe(1);
          if(scenario.timing==="after-seal") {
            let timer:ReturnType<typeof setTimeout>|undefined;
            try{await Promise.race([seal,new Promise<void>(resolve=>{timer=setTimeout(resolve,5000);})]);}
            finally{if(timer!==undefined)clearTimeout(timer);}
          }
          writeFileSync(scenario.mutation==="source"?join(dirname(linked.package.manifest),"lib.rs"):linked.staged,expected.editedSource);
          edited=true;
        }}];
      },
    },announced?"observe-announced":withChecksum?"observe-consumed":"observe-linked",({outcome,held})=>{
      const runs=join(held.root,"cache",NATIVE_BUILD_DRIVER_POLICY_V1.warmRoot);
      const receipts=join(outcome?.generation.report??join(runs,readdirSync(runs)[0]!,NATIVE_BUILD_DRIVER_POLICY_V1.stores.report),NATIVE_BUILD_DRIVER_POLICY_V1.provenanceDirectory);
      const files=readdirSync(receipts).filter(name=>name.startsWith("cargo-unit-provenance-"));
      expect(files.length>0).toBe(scenario.publishesReceipt);
      if(!files.length)return;
      const units=files.flatMap(name=>decodeCargoProvenanceV1(readFileSync(join(receipts,name),"utf8")).units);
      for(const source of sources){
        const original=units.flatMap(unit=>unit.inputs).find(row=>row.path===source);
        expect(original?.sha256).toBe(createHash("sha256").update(expected.originalSource).digest("hex"));
        if(withChecksum){
          const compiler=units.flatMap(unit=>unit.depInfo).flatMap(dep=>dep.checksums).find(row=>resolve(row.path)===source);
          expect(compiler?.blake3).toBe(Buffer.from(blake3(Buffer.from(expected.originalSource))).toString("hex"));
          expect(compiler?.length).toBe(Buffer.byteLength(expected.originalSource));
          expect(readFileSync(source,"utf8")).toBe(expected.editedSource);
        }
      }
    });
    if (scenario.publishesReceipt) expect([...(await run).staged].sort()).toEqual(packages.map(name=>name+":release").sort());
    else await expect(run).rejects.toThrow(compilerSealMutationError);
    expect(edited).toBe(true);
    expect(sealed).toBe(scenario.publishesReceipt||scenario.timing==="after-seal");
    checkpoint();
  },scenario.publishesReceipt?undefined:compilerSealMutationError);
}

registerIsolatedBuildTestV1(import.meta.path,"compiler source witnesses refuse mixed consumed versions in either order and reject altered blobs",{bodyMilliseconds:120_000,scope:{kind:"body"}},async checkpoint=>{
  const vector=fixture.provenanceSeal,root=mkdtempSync(join(artifacts,"m-")),source=join(root,"source.rs");
  const original=Buffer.from(vector.originalSource),revised=Buffer.from(vector.editedSource);
  const physical=()=>({maxBytes:65536,maxWork:100,chunkBytes:4096,cancelled:()=>false,remainingMs:()=>10000,onProgress:()=>{}});
  const claim=(bytes:Buffer)=>({path:source,blake3:Buffer.from(blake3(bytes)).toString("hex"),length:bytes.byteLength});
  await withBuildTestInvocationV1(originalInvocation=>{
    const control=scriptCompilerAdmissionControlV1(originalInvocation,"test:source-witness",120000);
    return withLawStorageV1(control,lawStoragePolicyV1.stubImageBytes,async(held,control)=>{
      for(const order of vector.mixedOrders){
        const directory=join(held.root,"c"),witness=new CargoSourceWitnessV1(root,directory,held.storage,control);
        writeFileSync(source,original);
        await witness.remember(source,original,control);
        writeFileSync(source,revised);
        const missing=Buffer.concat([original,revised]);
        await expect(witness.input(source,claim(missing),physical())).rejects.toThrow("Compiler consumed bytes have no matching retained source witness");
        const first=order[0]==="retained"?original:revised,second=first===original?revised:original;
        const selected=await witness.input(source,claim(first),physical());
        expect(selected.retained).toBe(first===original);
        expect(selected.row.sha256).toBe(createHash("sha256").update(first).digest("hex"));
        await expect(witness.input(source,claim(second),physical())).rejects.toThrow("Compiler consumed inconsistent source versions");
        if(selected.retained){
          const blob=join(directory,selected.row.sha256!);
          chmodSync(blob,0o600);
          writeFileSync(blob,revised);
          await expect(witness.recheck(physical)).rejects.toThrow("Retained compiler source witness changed before transfer");
        }
        await witness.close();
      }
    },originalInvocation);
  });
  checkpoint();
});

registerIsolatedBuildTestV1(import.meta.path, "a failed cargo exit with no failed unit surfaces cargo's own stderr in the error and the report", { bodyMilliseconds: 120_000, scope: { kind: "body" } }, async (checkpoint) => {
  const root = mkdtempSync(join(artifacts, "g-")), cargo = stubCargo(root), pass = stubWorkspace(root, "hub", ["comp-a"], "release");
  let report = "";
  await expect(runStub(root, { version: 1, passes: [pass] }, cargo, {}, "stderr-exit", ({ held }) => { const runs = join(held.root, "cache", NATIVE_BUILD_DRIVER_POLICY_V1.warmRoot); for (const run of readdirSync(runs)) { const file = join(runs, run, NATIVE_BUILD_DRIVER_POLICY_V1.stores.report, "report.md"); if (existsSync(file)) report = readFileSync(file, "utf8"); } })).rejects.toThrow(/cargo exit 101[\s\S]*build broker unreachable: All pipe instances are busy\./u);
  expect(report).toContain("## Cargo diagnostics");
  expect(report).toContain("build broker unreachable: All pipe instances are busy.");
  checkpoint();
}, /cargo exit 101/u);

registerIsolatedBuildTestV1(import.meta.path, "an unlinked component fails the native build without deleting unowned checkout bytes and still writes the report", { bodyMilliseconds: 120_000, scope: { kind: "body" } }, async (checkpoint) => {
  const root = mkdtempSync(join(artifacts, "f-")), cargo = stubCargo(root), pass = stubWorkspace(root, "hub", ["comp-a", "comp-b"], "release");
  const stale = join(pass.packages[1]!.manifest,"..","unowned-stale");
  mkdirSync(stale, { recursive: true });
  writeFileSync(join(stale, "comp_b.wasm"), header);
  let reported = false;
  await expect(runStub(root, { version: 1, passes: [pass] }, cargo, {}, "omit-last", ({ held }) => { const runs = join(held.root, "cache", NATIVE_BUILD_DRIVER_POLICY_V1.warmRoot); reported = readdirSync(runs).some((run) => existsSync(join(runs, run, NATIVE_BUILD_DRIVER_POLICY_V1.stores.report, "report.md"))); })).rejects.toThrow(/Native build failed: comp-b/u);
  expect(readFileSync(join(stale,"comp_b.wasm"))).toEqual(header);
  expect(reported).toBe(true);
  checkpoint();
}, /Native build failed: comp-b/u);



registerIsolatedBuildTestV1(import.meta.path, "a failed cargo pass whose failing unit's inputs did not change stays a hard failure", { bodyMilliseconds: 120_000, scope: { kind: "body" } }, async (checkpoint) => {
  const root = mkdtempSync(join(artifacts, "e-")), pass = stubWorkspace(root, "hub", ["comp-a", "comp-b"], "release");
  const failure = await runStub(root, { version: 1, passes: [pass] }, stubCargo(root), {}, "error-last").then(() => undefined, (error: unknown) => error);
  expect(failure).not.toBeInstanceOf(ConcurrentEditErrorV1);
  expect(String((failure as Error).message)).toMatch(/Native build failed: comp-b/u);
  checkpoint();
}, /Native build failed: comp-b/u);

for (const count of [1, 2]) registerIsolatedBuildTestV1(import.meta.path, `reclaim preserves Cargo's hard failure after ${count} pass(es)`, { bodyMilliseconds: 60000, scope: { kind: "body" } }, async checkpoint => {
  const root = mkdtempSync(join(artifacts, "reclaim-")), pass = stubWorkspace(root, "hub", ["comp-a", "comp-b"], "release");
  const passes = count === 2 ? [stubWorkspace(root, "earlier", ["comp-earlier"], "release"), pass] : [pass];
  const logs: string[] = [];
  const failure = await runStub(root, { version: 1, passes }, stubCargo(root), {
    generation: "fresh", storageBytes: 2 * lawStoragePolicyV1.realImageBytes, log: line => logs.push(line),
  }, "reclaim-error").then(() => undefined, (error: unknown) => error);
  expect(failure).not.toBeInstanceOf(ConcurrentEditErrorV1);
  expect(String((failure as Error).message)).toMatch(/Native build failed: comp-b/u);
  for (const { id } of passes) expect(logs.some(line => line.includes(`${id}: reclaimed `) && /reclaimed [1-9]/u.test(line)), logs.join("\n")).toBe(true);
  checkpoint();
}, /Native build failed: comp-b/u);

registerIsolatedBuildTestV1(import.meta.path, "a failed cargo pass whose failing unit's inputs changed after Cargo started is a restartable concurrent edit naming the file", { bodyMilliseconds: 120_000, scope: { kind: "body" } }, async (checkpoint) => {
  installRestartableFailureHandlerV1(() => undefined);
  const root = mkdtempSync(join(artifacts, "e-")), pass = stubWorkspace(root, "hub", ["comp-a", "comp-b"], "release");
  const failure = await runStub(root, { version: 1, passes: [pass] }, stubCargo(root), {}, "error-touch").then(() => undefined, (error: unknown) => error) as { code?: string; reason?: string; path?: string } | undefined;
  expect(failure).toBeInstanceOf(ConcurrentEditErrorV1);
  expect(failure).toMatchObject({ code: churnProtocolV1().concurrentEditCode, reason: "bytes", path: join(dirname(pass.manifest), "comp-b", "lib.rs") });
  checkpoint();
}, /modified after Cargo started/u);

test("failed pass inputs: a changed custody digest, a vanished file or a file modified after Cargo started is a concurrent edit; untouched inputs and foreign paths are not", async () => {
  const modified = (table: Record<string, number | undefined>) => (path: string) => table[path], clean = async () => undefined;
  expect(await failedPassConcurrentEditV1({ startedMs: 100, files: ["/w/a.rs"], recheck: clean, modifiedMs: modified({ "/w/a.rs": 100 }) })).toBeUndefined();
  expect(await failedPassConcurrentEditV1({ startedMs: 100, files: ["/w/a.rs", "/w/b.rs"], recheck: clean, modifiedMs: modified({ "/w/a.rs": 50, "/w/b.rs": 101 }) })).toMatchObject({ reason: "bytes", path: "/w/b.rs" });
  expect(await failedPassConcurrentEditV1({ startedMs: 100, files: ["/w/gone.rs"], recheck: clean, modifiedMs: modified({}) })).toMatchObject({ reason: "vanished", path: "/w/gone.rs" });
  const digest = await failedPassConcurrentEditV1({ startedMs: 100, files: [], recheck: async () => { throw concurrentEditV1("bytes", "/w/dep.rs", "digest differs"); } });
  expect(digest).toBeInstanceOf(ConcurrentEditErrorV1);
  expect(digest).toMatchObject({ reason: "bytes", path: "/w/dep.rs" });
  expect(await failedPassConcurrentEditV1({ startedMs: 100, files: [], recheck: async () => { throw Error("not an edit"); } })).toBeUndefined();
  const cwd = resolve("/w");
  const manifest = join(cwd, "plugin", "packages", "rust", "Cargo.toml");
  expect(failedCompilerMessageFilesV1({ package_id: "registry+https://example.org/index#foreign@1.0.0", manifest_path: "/deps/foreign/Cargo.toml", message: { spans: [{ file_name: "src/lib.rs" }, { file_name: "<proc-macro source code>" }] } }, cwd, join(cwd, "hub"))).toEqual([]);
  expect(failedCompilerMessageFilesV1({ package_id: "path+file:///w/plugin/packages/rust#window-config@0.1.0", manifest_path: manifest, message: { spans: [{ file_name: "window-config-0.1.0/../../native/lib.rs" }] } }, cwd, join(cwd, "hub"))).toEqual([manifest, join(cwd, "plugin", "native", "lib.rs")]);
  expect(failedCompilerMessageFilesV1({ manifest_path: join(cwd, "c", "Cargo.toml"), message: { spans: [{ file_name: "c/src/lib.rs" }, { file_name: join(cwd, "c", "src", "lib.rs") }, { file_name: resolve("/rustc/library/core.rs") }] } }, cwd, cwd)).toEqual([join(cwd, "c", "Cargo.toml"), join(cwd, "c", "src", "lib.rs")]);
});

registerIsolatedBuildTestV1(import.meta.path, "a batch the free storage cannot cover above the hard reserve is refused before Cargo starts", { bodyMilliseconds: 120_000, scope: { kind: "body" } }, async (checkpoint) => {
  const root = mkdtempSync(join(artifacts, "h-")), cargo = stubCargo(root), pass = stubWorkspace(root, "hub", ["comp-a"], "release");
  const lines: string[] = [];
  await expect(runStub(root, { version: 1, passes: [pass] }, cargo, { storageReserveBytes: lawStoragePolicyV1.stubImageBytes * 2, log: line => lines.push(line) })).rejects.toThrow(/Native build storage refused: .*reserve/u);
  expect(lines.findLast(line => line.includes("failure(s) in "))).toContain("1 failure(s) in ");
  expect(existsSync(cargo.calls)).toBe(false);
  checkpoint();
}, /Native build storage refused/u);

registerIsolatedBuildTestV1(import.meta.path, "cancellation stops every Cargo tree", { bodyMilliseconds: 120_000, scope: { kind: "body" } }, async (checkpoint) => {
  const root = mkdtempSync(join(artifacts, "x-")), cargo = stubCargo(root), pass = stubWorkspace(root, "hub", ["comp-a"], "release"), controller = new AbortController();
  const cancelled = runStub(root, { version: 1, passes: [pass] }, cargo, { cancel: { signal: controller.signal, milliseconds: 30000 } }, "hang");
  const cancel=setInterval(()=>{if(existsSync(cargo.calls)){clearInterval(cancel);controller.abort(Error("cancelled by law"));}},fixture.stub.cancelAfterMs);
  try{await expect(cancelled).rejects.toThrow();}finally{clearInterval(cancel);}
  const pid = JSON.parse(readFileSync(cargo.calls, "utf8").trim().split("\n")[0]!).pid;
  await Bun.sleep(200);
  expect(() => process.kill(pid, 0)).toThrow();
  checkpoint();
}, /./su);

registerIsolatedBuildTestV1(import.meta.path, "the stall watchdog stops a pass in which nothing starts or finishes", { bodyMilliseconds: 120_000, scope: { kind: "body" } }, async (checkpoint) => {
  const stalledRoot = mkdtempSync(join(artifacts, "y-")), stalledCargo = stubCargo(stalledRoot);
  await expect(runStub(stalledRoot, { version: 1, passes: [stubWorkspace(stalledRoot, "hub", ["comp-a"], "release")] }, stalledCargo, { stallMs: fixture.stub.stallMs }, "hang")).rejects.toThrow(/stalled/u);
  checkpoint();
}, /stalled/u);

test("a real build over wasm32-wasip2 components: both profiles, admitted units, streamed stages, timeline, report and recorded telemetry", async () => {
  const root = mkdtempSync(join(artifacts, "r-")), workspace = join(root, "w");
  mkdirSync(workspace, { recursive: true });
  writeFileSync(join(workspace, "Cargo.toml"), `[workspace]\nmembers = ["base", ${fixture.real.components.map((name) => JSON.stringify(name)).join(", ")}]\nresolver = "2"\n\n[profile.release]\nopt-level = "s"\n\n[profile.prototype]\ninherits = "dev"\n`);
  mkdirSync(join(workspace, "base", "src"), { recursive: true });
  writeFileSync(join(workspace, "base", "Cargo.toml"), `[package]\nname = "base"\nversion = "0.1.0"\nedition = "2021"\n`);
  writeFileSync(join(workspace, "base", "src", "lib.rs"), `pub fn mix(seed: u64) -> u64 { (0..seed).fold(seed, |acc, value| acc.rotate_left(5) ^ value.wrapping_mul(0x9e37_79b9_7f4a_7c15)) }\n`);
  for (const name of fixture.real.components) {
    mkdirSync(join(workspace, name, "src"), { recursive: true });
    writeFileSync(join(workspace, name, "Cargo.toml"), `[package]\nname = "${name}"\nversion = "0.1.0"\nedition = "2021"\n\n[lib]\ncrate-type = ["cdylib"]\n\n[dependencies]\nbase = { path = "../base" }\n`);
    writeFileSync(join(workspace, name, "src", "lib.rs"), `#[unsafe(no_mangle)]\npub extern "C" fn run(seed: u64) -> u64 { base::mix(seed) }\n`);
  }
  expect(spawnSync(registeredToolExecutableV1("cargo"), ["generate-lockfile", "--offline", "--manifest-path", join(workspace, "Cargo.toml")], { encoding: "utf8" }).status).toBe(0);
  const plan: NativeBuildPlanV1 = { version: 1, passes: fixture.real.profiles.map((profile, index) => ({ id: `${profile}-${String(index).padStart(2, "0")}`, manifest: join(workspace, "Cargo.toml"), packages: fixture.real.components.map((name) => ({ name, manifest: join(workspace, name, "Cargo.toml") })), target: "wasm32-wasip2", profile })) };
  const lines: string[] = [], scheduler = (await schedulerBinaries()).get("semio-build-scheduler")!, control = phaseCompilerAdmissionControlV1(AbortSignal.timeout(600_000),600_000,()=>undefined,()=>new Promise(done=>setImmediate(done)));
  await withLawStorageV1(control, lawStoragePolicyV1.realImageBytes, async (held, control) => {
  const outcome = await runNativeBuildDriverV1({ plan, storage: held.storage, sourceBoundary: lawSourceBoundaryV1, cwd: workspace, cacheRoot: join(held.root, "cache"), shared: { target: join(held.root, "t"), build: join(held.root, "b") }, generation: "fresh", leaseDirectory: artifacts, environment: { ...process.env, ...held.environment, CARGO_TARGET_DIR: join(held.root, "t") }, control, tokens: fixture.real.tokens, storageReserveBytes: lawStoragePolicyV1.reserveBytes, scheduler, progressMs: 1000, log: (line) => lines.push(line) });
  expect(outcome.failed).toEqual([]);
  expect(outcome.staged).toHaveLength(fixture.real.components.length * fixture.real.profiles.length);
  for (const pass of plan.passes) for (const row of pass.packages) expect([...readFileSync(nativeBuildComponentStagePathV1(outcome.generation,pass,row)).subarray(0, 8)]).toEqual([...header]);
  const units = JSON.parse(readFileSync(join(outcome.generation.report, "units.json"), "utf8"));
  expect(new Set(units.units.map((unit: any) => unit.profile))).toEqual(new Set(fixture.real.profiles));
  for (const unit of units.units) { expect(unit.peakRssBytes).toBeGreaterThan(0); expect(unit.exit).toBe(0); }
  expect(units.criticalPath.at(-1).label).toMatch(/cdylib/u);
  expect(readTimelineV1(join(outcome.generation.report, "timeline.jsonl")).length).toBeGreaterThan(0);
  const history = await readUnitHistoryV1(unitHistoryPathV1(join(held.root, "cache")));
  expect([...history.keys()].filter((key) => key.startsWith("comp-a@comp_a|wasm32-wasip2|")).sort()).toEqual(fixture.real.profiles.map((profile) => `comp-a@comp_a|wasm32-wasip2|${profile}|cdylib`).sort());
  expect(existsSync(outcome.generation.target)).toBe(false);
  expect(existsSync(outcome.generation.build)).toBe(false);
  expect(lines.some((line) => line.includes("component(s) staged"))).toBe(true);
  expect(lines.filter((line) => /-b\d+: 1 component\(s\), predicted .*unmeasured units estimated by kind/u.test(line))).toHaveLength(fixture.real.components.length * fixture.real.profiles.length);
  console.log(`[component-pass] real release: ${units.units.length} unit(s), ${outcome.staged.length} component(s), report ${outcome.report}`);
  });
}, 600_000);

/** 🧾️ A job command that records when it ran and which jobserver it joined. */
function stubJob(root: string, id: string, after: readonly string[], delayMs: number, exit = 0): NativeBuildJobV1 {
  const script = join(root, `j-${id}`, "📜️script.ts");
  mkdirSync(dirname(script), { recursive: true });
  writeFileSync(script, `import { appendFileSync } from "node:fs";\nconst start = Date.now();\nawait new Promise((accept) => setTimeout(accept, ${delayMs}));\nappendFileSync(${JSON.stringify(join(root, "jobs.jsonl"))}, JSON.stringify({ id: ${JSON.stringify(id)}, start, end: Date.now(), jobserver: process.env.CARGO_MAKEFLAGS ?? null, pass: process.env.SEMIO_BUILD_PASS ?? null, parallelism: Number(process.env.SEMIO_COMPILER_PARALLELISM), capacityHeld: process.env.SEMIO_COMPILER_CAPACITY_HELD, makeflags: process.env.MAKEFLAGS ?? null }) + "\\n");\nprocess.exit(${exit});\n`);
  return { id, cwd: root, command: [process.execPath, script], after, profile: "release", target: "host", workerProtocol: "jobserver" };
}

test("jobs start as soon as their completions happen, post-link stages stream per component, and a job waits for a whole component stage", async () => {
  const root = mkdtempSync(join(artifacts, "j-")), cargo = stubCargo(root), pass = stubWorkspace(root, "hub", ["comp-a", "comp-b"], "release");
  const checked: Record<string, number> = {};
  const plan: NativeBuildPlanV1 = { version: 1, passes: [pass], jobs: [stubJob(root, "tool", [], 50), stubJob(root, "pages", ["job:tool", "components:check"], 10)] };
  let units: any, report = "";
  const outcome = await runStub(root, plan, cargo, { stages: (component) => [{ name: "check", capacity: "single", after: ["job:tool"], run: async () => { checked[component.package.name] = Date.now(); } }] }, "ok", ({ outcome: run }) => { if (!run) return; units = JSON.parse(readFileSync(join(run.generation.report, "units.json"), "utf8")); report = readFileSync(run.report, "utf8"); });
  expect(outcome.failed).toEqual([]);
  const jobs = readFileSync(join(root, "jobs.jsonl"), "utf8").trim().split("\n").map((line) => JSON.parse(line)), byId = Object.fromEntries(jobs.map((row) => [row.id, row]));
  const calls = readFileSync(cargo.calls, "utf8").trim().split("\n").map((line) => JSON.parse(line)), passStart = calls.find((row) => row.at).at;
  expect(byId.tool.start).toBeLessThan(passStart + 2000);
  for (const name of ["comp-a", "comp-b"]) expect(checked[name]).toBeGreaterThanOrEqual(byId.tool.end);
  expect(byId.pages.start).toBeGreaterThanOrEqual(Math.max(byId.tool.end, ...Object.values(checked)));
  for (const row of jobs) { expect(row.jobserver).toMatch(/--jobserver-auth=(?:fifo:|semio-jobserver-)/u); expect(row.pass).toBe(row.id); }
  expect(units.stages.filter((stage: any) => stage.stage === "check").map((stage: any) => stage.component).sort()).toEqual(["comp-a", "comp-b"]);
  expect(report).toContain("## Stage cache");
}, 120_000);

test("reserved nested compiler processes receive held worker slots and release them before the next compiler", async () => {
  const root = mkdtempSync(join(artifacts, "b-")), cargo = stubCargo(root), pass = stubWorkspace(root, "hub", ["comp-a"], "release");
  const jobs: NativeBuildJobV1[] = [
    { ...stubJob(root, "reserved", [], 20), lane: "release", workerProtocol: "reserved" },
    { ...stubJob(root, "shared", [], 20), lane: "release" },
  ];
  const outcome = await runStub(root, { version: 1, passes: [pass], jobs }, cargo);
  expect(outcome.failed).toEqual([]);
  const [reserved, shared] = readFileSync(join(root, "jobs.jsonl"), "utf8").trim().split("\n").map((line) => JSON.parse(line));
  expect(reserved.parallelism).toBe(fixture.stub.tokens);
  expect(reserved.capacityHeld).toBe("true");
  expect(reserved.jobserver).toBeNull();
  expect(reserved.makeflags).toBeNull();
  expect(shared.parallelism).toBe(fixture.stub.tokens);
  expect(shared.jobserver).toMatch(/--jobserver-auth=(?:fifo:|semio-jobserver-)/u);
  expect(shared.start).toBeGreaterThanOrEqual(reserved.end);
  console.log(`[component-pass] reserved ${reserved.parallelism} workers; shared successor and Cargo completed`);
}, 120_000);

test("exclusive compiler jobs and stages hold the whole pool without deadlocking concurrent requests", async () => {
  const root = mkdtempSync(join(artifacts, "x-")), cargo = stubCargo(root), pass = stubWorkspace(root, "hub", ["comp-a", "comp-b"], "release");
  const jobs = ["a", "b"].map((id) => ({ ...stubJob(root, id, [], 15), workerProtocol: "exclusive" as const }));
  const stages: { start: number; end: number; workers: number; mode: string | undefined }[] = [];
  const outcome = await runStub(root, { version: 1, passes: [pass], jobs }, cargo, { stages: () => [{ name: "whole-host", capacity: "whole-host", after: [], run: async (context) => {
    const start = Date.now();
    expect(context.environment.SEMIO_COMPILER_CAPACITY_HELD).toBe("true");
    expect(context.environment.CARGO_MAKEFLAGS).toBeUndefined();
    await Bun.sleep(15);
    stages.push({ start, end: Date.now(), workers: Number(context.environment.SEMIO_COMPILER_PARALLELISM), mode: context.environment.SEMIO_BUILD_MODE });
  } }] });
  expect(outcome.failed).toEqual([]);
  const children = readFileSync(join(root, "jobs.jsonl"), "utf8").trim().split("\n").map((line) => JSON.parse(line));
  expect(children).toHaveLength(2);
  for (const child of children) { expect(child.parallelism).toBe(fixture.stub.tokens); expect(child.jobserver).toBeNull(); }
  expect(children[1].start).toBeGreaterThanOrEqual(children[0].end);
  expect(stages).toHaveLength(2);
  for (const stage of stages) { expect(stage.workers).toBe(fixture.stub.tokens); expect(stage.mode).toBe("release"); }
  expect(stages[1]!.start).toBeGreaterThanOrEqual(stages[0]!.end);
  console.log(`[component-pass] exclusive jobs=2 stages=2 workers=${fixture.stub.tokens}; shared Cargo completed`);
}, 120_000);

registerIsolatedBuildTestV1(import.meta.path, "a component Cargo did not link fails every stage and job that waits for it, and a failed job fails the release", { bodyMilliseconds: 120_000, scope: { kind: "body" } }, async (checkpoint) => {
  const root = mkdtempSync(join(artifacts, "k-")), cargo = stubCargo(root), pass = stubWorkspace(root, "hub", ["comp-a", "comp-b"], "release");
  let ran = 0;
  const plan: NativeBuildPlanV1 = { version: 1, passes: [pass], jobs: [stubJob(root, "pages", ["components:check"], 0), stubJob(root, "broken", [], 0, 3)] };
  await expect(runStub(root, plan, cargo, { stages: () => [{ name: "check", capacity: "single", after: [], run: async () => { ran++; } }] }, "omit-last")).rejects.toThrow(/Native build failed: (?=.*comp-b)(?=.*job:broken)/u);
  expect(ran).toBe(1);
  expect(readFileSync(join(root, "jobs.jsonl"), "utf8")).not.toContain('"pages"');
  checkpoint();
}, /Native build failed: comp-b/u);

test("a lane job runs inside its profile's Cargo lane before that lane's passes, and an idle stage widens onto free slots", async () => {
  const root = mkdtempSync(join(artifacts, "l-")), cargo = stubCargo(root), pass = stubWorkspace(root, "hub", ["comp-a"], "release");
  const widths: number[] = [];
  const plan: NativeBuildPlanV1 = { version: 1, passes: [pass], jobs: [{ ...stubJob(root, "module", [], 200), lane: "release", profile: "release", target: "wasm32-unknown-unknown" }] };
  const outcome = await runStub(root, plan, cargo, { stages: () => [{ name: "wide", capacity: "available", after: [], run: async (context) => { widths.push(context.parallelism); } }] });
  expect(outcome.failed).toEqual([]);
  const module = JSON.parse(readFileSync(join(root, "jobs.jsonl"), "utf8").trim()), passStart = readFileSync(cargo.calls, "utf8").trim().split("\n").map((line) => JSON.parse(line)).find((row) => row.at).at;
  expect(module.end).toBeLessThanOrEqual(passStart);
  expect(widths).toHaveLength(1);
  expect(widths[0]).toBeGreaterThanOrEqual(1);
  expect(widths[0]).toBeLessThanOrEqual(3);
}, 120_000);

registerIsolatedBuildTestV1(import.meta.path, "a finalizer runs once its stages settled, also after one failed, and the failure still fails the release", { bodyMilliseconds: 120_000, scope: { kind: "body" } }, async (checkpoint) => {
  const root = mkdtempSync(join(artifacts, "z-")), cargo = stubCargo(root), pass = stubWorkspace(root, "hub", ["comp-a", "comp-b"], "release");
  const law=fixture.finalizerSettlement, failedAt:Record<string,number>={}, events:string[]=[];
  const assemblyError=Error(law.primaryError);
  const stages = (component: LinkedComponentV1): readonly NativeBuildStageV1[] => {
    const name=component.package.name;
    const row=law.components.find(row=>row.name===name)!;
    return [
      { name: "assemble", capacity: "single", after: [], run: async () => {
        events.push(name+":assemble");
        if (!row.assembleSucceeds) { failedAt[name]=Date.now(); throw assemblyError; }
      } },
      { name: "publish", capacity: "single", after: ["assemble"], run: async () => { events.push(name+":publish"); } },
      { name: "discard", capacity: "cleanup", after: ["publish"], run: async (context) => {
        expect(context.signal.aborted).toBe(false);
        expect(context.parallelism).toBe(0);
        if (!row.assembleSucceeds) expect(Date.now()).toBeGreaterThanOrEqual(failedAt[name]!);
        events.push(name+":discard");
      } },
    ];
  };
  const failure=await runStub(root,{version:1,passes:[pass]},cargo,{stages,verifyStageWriters:true}).then(()=>undefined,error=>error);
  expect(failure?.message).toMatch(/Native build failed: comp-b \(assemble\)/u);
  expect(failure?.cause).toBe(assemblyError);
  const oracle=new Database(":memory:");
  try {
    oracle.exec("create table component(name text, assembled integer);");
    for (const row of law.components) oracle.query("insert into component values(?,?)").run(row.name,Number(row.assembleSucceeds));
    const expected=oracle.query<{event:string},[]>("select name||':assemble' as event from component union all select name||':publish' from component where assembled=1 union all select name||':discard' from component order by event").all();
    expect(events.sort()).toEqual(expected.map(row=>row.event));
  } finally { oracle.close(); }
  checkpoint();
}, /Native build failed: comp-b/u);


registerIsolatedBuildTestV1(import.meta.path,"a blocked browser release drains real optimize before deleting its work",{bodyMilliseconds:120_000,scope:{kind:"body"}},async checkpoint=>{
  const vector=fixture.optimizerDrain,root=mkdtempSync(join(artifacts,"d-")),cargo=stubCargo(root),pass=stubWorkspace(root,"hub",[vector.root],"release");
  const failure=Error(vector.failure),events:string[]=[],logs:string[]=[];
  const stored=Promise.withResolvers<void>(),modelFailed=Promise.withResolvers<void>(),discarded=Promise.withResolvers<void>();
  await withBuildTestInvocationV1(async invocation=>{
    const result=await runStub(root,{version:1,passes:[pass]},cargo,{
      log(line){logs.push(line);if(line.includes(vector.failure)){events.push("model-failed");modelFailed.resolve();}},
      stages(component){
        let workspace:string,transpiled:string;
        const crate=component.package.name,file=crate.replaceAll("-","_")+".core.wasm";
        return [
          {name:"transpile",after:[],capacity:"single",async run(context){
            workspace=join(context.generation.root,"w",crate);
            transpiled=join(workspace,"transpiled");
            mkdirSync(transpiled,{recursive:true});
            writeFileSync(join(transpiled,file),Buffer.from(vector.core));
          }},
          {name:"model",after:[`provenance:${component.pass.id}`],capacity:"io",async run(context){
            await context.provenance();
            await stored.promise;
            throw failure;
          }},
          {name:"optimize",after:["transpile"],capacity:"parallel",async run(context){
            const cache:typeof context.cache={...context.cache,run(key,output,produce,signal,slot){
              return context.cache.run(key,output,async directory=>{
                await produce(directory);
                events.push("cas-ready");
                stored.resolve();
                await modelFailed.promise;
                await Promise.race([discarded.promise,Bun.sleep(150)]);
              },signal,slot);
            }};
            const target={pluginId:crate,packageId:crate,cratePath:root,packageName:crate,wasmOut:component.staged,role:"plugin" as const,consumes:[],dependsOn:[]};
            const deliverable={profile:"release" as const,directory:dirname(component.staged),wasm:component.staged,receipt:join(dirname(component.staged),"receipt.json")};
            const output=await optimizeComponentCoresV1({invocation,repoRoot:process.cwd(),target,deliverable,cache,work:workspace,environment:context.environment,control:context.control},transpiled,context.parallelism);
            expect(output).toBe(join(workspace,"optimized"));
            expect(WebAssembly.validate(readFileSync(join(output!,file)))).toBe(true);
            events.push("optimize-finished");
          }},
          {name:"browser",after:["model","optimize"],capacity:"single",async run(){events.push("browser");}},
          {name:"complete",after:["browser"],capacity:"io",async run(){events.push("complete");}},
          {name:"discard",after:["complete"],capacity:"cleanup",async run(){
            rmSync(workspace,{recursive:true,force:true});
            events.push("discard");
            discarded.resolve();
          }},
        ];
      },
    },"observe-linked").then(()=>undefined,error=>error);
    expect(result?.cause).toBe(failure);
    expect(logs.filter(line=>line.includes("ENOENT"))).toEqual([]);
    const oracle=new Database(":memory:");
    try{
      const ordered=oracle.query<{event:string},[string]>("select value as event from json_each(?) order by key");
      expect(events).toEqual(ordered.all(JSON.stringify(vector.events)).map(row=>row.event));
    }finally{oracle.close();}
  });
  checkpoint();
},/Native build failed: opt-drain \(model\)/u);

registerIsolatedBuildTestV1(import.meta.path, "a job failing at once rejects the driver, naming it, and terminates the sibling job still running", { bodyMilliseconds: 60_000, scope: { kind: "body" } }, async (checkpoint) => {
  const root = mkdtempSync(join(artifacts, "ff-")), cargo = stubCargo(root), pass = stubWorkspace(root, "hub", ["comp-a"], "release"), pids = join(root, "long.pid");
  const long = stubJob(root, "long", [], 0);
  writeFileSync(long.command[1]!, `import { writeFileSync } from "node:fs";\nwriteFileSync(${JSON.stringify(pids)}, String(process.pid));\nawait Bun.sleep(120000);\n`);
  const failing = stubJob(root, "broken", [], 0, 3);
  const started = performance.now();
  await expect(runStub(root, { version: 1, passes: [pass], jobs: [long, failing] }, cargo)).rejects.toThrow(/job failed.*job:broken/su);
  expect(performance.now() - started).toBeLessThan(30_000);
  if (existsSync(pids)) { const pid = Number(readFileSync(pids, "utf8")); await Bun.sleep(300); expect(() => process.kill(pid, 0)).toThrow(); }
  checkpoint();
}, /./su);

test("a job that exits with the shared restart status runs again once with the shared backoff instead of failing the build", async () => {
  const root = mkdtempSync(join(artifacts, "rs-")), cargo = stubCargo(root), pass = stubWorkspace(root, "hub", ["comp-a"], "release"), marker = join(root, "restarted");
  const restarting = stubJob(root, "restarting", [], 0);
  writeFileSync(restarting.command[1]!, `import { existsSync, writeFileSync, appendFileSync } from "node:fs";\nappendFileSync(${JSON.stringify(join(root, "starts.log"))}, "x");\nif (!existsSync(${JSON.stringify(marker)})) { writeFileSync(${JSON.stringify(marker)}, ""); process.exit(${formatStampProtocolV1().exitCode}); }\n`);
  const outcome = await runStub(root, { version: 1, passes: [pass], jobs: [restarting] }, cargo);
  expect(outcome.failed).toEqual([]);
  expect(readFileSync(join(root, "starts.log"), "utf8")).toBe("xx");
}, 60_000);

test("job attempt and tail files stay distinct on case-insensitive filesystems", () => {
  const ids = fixture.jobRetryDiagnostics.pathIds, names = nativeBuildJobLogNamesV1(ids), maximum = 1 + formatStampProtocolV1().maximumReexecutions;
  const paths = [...names.values()].flatMap(name => Array.from({ length: maximum }, (_, index) => {
    const log = nativeBuildJobAttemptLogPathV1(name, index + 1);
    return [log, log + ".tail"];
  })).flat();
  expect(paths).toHaveLength(ids.length * maximum * 2);
  expect(new Set(paths.map(path => path.toLowerCase())).size).toBe(paths.length);
});

test("a restartable job retains its attempt logs and identifies each failure to operators", async () => {
  const law = fixture.jobRetryDiagnostics, directory = mkdtempSync(join(artifacts, "rs-")), notices: string[] = [];
  const signal = new AbortController().signal;
  await runNativeBuildJobAttemptsV1(
    { id: law.jobId, logPath: join(directory, law.attempts[0]!.log), log: line => notices.push(line), signal, check: () => signal.throwIfAborted(), ...law.limits },
    async (capture, attempt) => {
      const row = law.attempts[attempt - 1]!;
      capture(row.stdout);
      capture(row.stderr);
      return attempt < law.attempts.length ? formatStampProtocolV1().exitCode : 0;
    },
  );
  const expected = Object.fromEntries(law.attempts.flatMap(row => [
    [row.log, [row.stdout, row.stderr]],
    ...(row.tail ? [[row.tail, [row.stdout, row.stderr]]] : []),
  ]));
  const observed = Object.fromEntries(readdirSync(directory).map(file => [file, readFileSync(join(directory, file), "utf8").trim().split("\n")]));
  const oracle = new Ajv({ strict: true }).compile({ const: expected });
  expect(oracle(observed), JSON.stringify(observed)).toBe(true);
  expect(notices).toHaveLength(law.attempts.length - 1);
  for (const [index, notice] of notices.entries()) {
    const row = law.attempts[index]!;
    expect(notice).toContain(`attempt ${index + 1}`);
    expect(notice).toContain(row.log);
    expect(notice).toContain(row.stderr);
  }
});

test("a restartable job retains the bounded late tail when cancelled before the next attempt", async () => {
  const law = fixture.jobRetryDiagnostics, directory = mkdtempSync(join(artifacts, "rc-"));
  const controller = new AbortController(), reason = Error("job cancelled during retry backoff"), limit = law.limits.logLimitBytes;
  const settled = runNativeBuildJobAttemptsV1(
    { id: law.jobId, logPath: join(directory, law.attempts[0]!.log), log: () => controller.abort(reason), signal: controller.signal, check: () => controller.signal.throwIfAborted(), ...law.limits },
    async capture => {
      capture(law.overflow.repeat(limit));
      capture(law.attempts[0]!.stderr);
      return formatStampProtocolV1().exitCode;
    },
  );
  await expect(settled).rejects.toBe(reason);
  const { log, tail, stderr } = law.attempts[0]!;
  expect(readdirSync(directory).sort()).toEqual([log, tail].sort());
  expect(readFileSync(join(directory, log)).length).toBe(limit);
  const rows = readFileSync(join(directory, tail!), "utf8").trim().split("\n");
  expect(rows).toEqual([law.overflow.repeat(law.limits.tailLineChars), stderr]);
});

test("queued exclusive compiler cancellation drains rejected work and permits a successor",async()=>{
 const root=mkdtempSync(join(artifacts,"q-")),cargo=stubCargo(root),pass=stubWorkspace(root,"hub",["comp-a"],"release"),law=fixture.queuedExclusiveCancellation;
 const holder={...stubJob(root,"holder",[],law.holdMs),workerProtocol:"reserved" as const},queued={...stubJob(root,"queued",[],0),workerProtocol:"exclusive" as const};
 const calls=join(root,"started.jsonl");writeFileSync(holder.command[1]!,`import {appendFileSync} from "node:fs";appendFileSync(${JSON.stringify(calls)},JSON.stringify({id:"holder",pid:process.pid})+"\\n");await Bun.sleep(${law.holdMs});`);
 const script=join(root,"h","📜️script.ts");mkdirSync(dirname(script));
 const options={plan:{version:1,passes:[pass],jobs:[holder,queued]},cwd:root,generation:"warm",leaseDirectory:artifacts,environment:{...process.env,STUB_MODE:"ok"},tokens:fixture.stub.tokens,physicalBytes:fixture.stub.physicalGiB*1024**3,storageReserveBytes:lawStoragePolicyV1.reserveBytes,scheduler:process.execPath,cargo,sourceBoundary:lawSourceBoundaryV1};
 const source=new URL("../🟦️.ts",import.meta.url).href;
 writeFileSync(script,`import {runNativeBuildDriverV1} from ${JSON.stringify(source)};import {withLawStorageV1,lawStoragePolicyV1} from ${JSON.stringify(new URL("../../🧪️tests/🦀️cargo/🟦️.ts",import.meta.url).href)};import {phaseCompilerAdmissionControlV1} from ${JSON.stringify(new URL("../../../../🏗️build/🎟️capacity/🟦️.ts",import.meta.url).href)};import {existsSync,readFileSync} from "node:fs";import {join} from "node:path";const run=(control,extra)=>withLawStorageV1(control,lawStoragePolicyV1.stubImageBytes,(held,control)=>runNativeBuildDriverV1({...options,...extra,storage:held.storage,cacheRoot:join(held.root,"cache"),shared:{target:join(held.root,"target"),build:join(held.root,"build")},environment:{...options.environment,...held.environment,CARGO_TARGET_DIR:join(held.root,"target")},control,log:()=>undefined}));const unhandled=[];process.on("unhandledRejection",error=>unhandled.push(String(error)));const controller=new AbortController(),options=${JSON.stringify(options)};const cancel=setInterval(()=>{if(existsSync(${JSON.stringify(calls)})){clearInterval(cancel);controller.abort(Error("cancel queued exclusive by law"));}},${law.pollMs});let refused=false;try{await run(phaseCompilerAdmissionControlV1(controller.signal,30000,()=>undefined,()=>new Promise(done=>setImmediate(done))),{});}catch{refused=true;}finally{clearInterval(cancel);}await Bun.sleep(${law.settleMs});const successor=await run(phaseCompilerAdmissionControlV1(new AbortController().signal,30000,()=>undefined,()=>new Promise(done=>setImmediate(done))),{plan:{version:1,passes:[options.plan.passes[0]],jobs:[]}});console.log(JSON.stringify({refused,unhandled,children:JSON.parse(readFileSync(${JSON.stringify(calls)},"utf8").trim()).id,resumed:successor.failed.length===0}));`);
 const child=Bun.spawn([process.execPath,script],{cwd:root,stdout:"pipe",stderr:"pipe"}),output=await new Response(child.stdout).text(),diagnostic=await new Response(child.stderr).text();expect(await child.exited,diagnostic).toBe(0);
 const result=JSON.parse(output.trim().split("\n").at(-1)!);expect(result.refused).toBe(true);expect(result.unhandled).toHaveLength(law.expectedUnhandled);expect([result.children]).toEqual(law.expectedChildren);expect(result.resumed).toBe(law.resumed);expect(existsSync(join(root,"jobs.jsonl"))).toBe(false);
 const pid=JSON.parse(readFileSync(calls,"utf8").trim()).pid;expect(()=>process.kill(pid,0)).toThrow();
},120_000);


test("unit graph compilation cancellation stops its metadata child and releases admission",async()=>{
 const root=mkdtempSync(join(artifacts,"ug-")),cargo=stubCargo(root),pass=stubWorkspace(root,"metadata",["unit"],"release"),controller=new AbortController();
 const running=runStub(root,{version:1,passes:[pass]},cargo,{cancel: { signal: controller.signal, milliseconds: 30000 }},"unit-hang");
 const calls=join(root,"unit-calls.jsonl");while(!existsSync(calls))await new Promise(resolve=>setTimeout(resolve,10));
 const pid=JSON.parse(readFileSync(calls,"utf8").split("\n")[0]!).pid;controller.abort(Error("metadata cancellation"));try{await running;}catch(error){expect(controller.signal.aborted).toBe(true);}
 await new Promise(resolve=>setTimeout(resolve,50));let live=true;try{process.kill(pid,0);}catch{live=false;}
 if(live)process.kill(pid,"SIGKILL");expect(live).toBe(false);
 expect((await runStub(root,{version:1,passes:[pass]},cargo)).staged).toEqual(["unit:release"]);
},120000);

test("profile mode selection follows only admitted compiler profiles",()=>{for(const row of fixture.profileModes)expect(profileBuildModeV1(row.profile) as string).toBe(row.mode);for(const profile of fixture.invalidProfiles)expect(()=>profileBuildModeV1(profile as Parameters<typeof profileBuildModeV1>[0])).toThrow();});

test("generation roots and fixed leaves are admitted as resolved absolute paths before effects", () => {
  const law = fixture.generationPathAdmission, root = mkdtempSync(join(artifacts, "p-"));
  const validator = new Ajv({ strict: true }).compile({ type: "string", maxLength: law.maximum });
  for (const kind of law.kinds) for (const field of law.cases) {
    const parent = join(root, `${kind}-${field}`), long = join(parent, "x".repeat(law.maximum - [...parent].length));
    const cache = field === "cache" || field === "stage-cache" ? long : field === "relative-cache" ? relative(process.cwd(), long) : join(parent, "cache");
    const shared = { target: field === "shared-target" ? long : join(parent, "target"), build: field === "shared-build" ? long : join(parent, "build") };
    expect(validator(resolve(long))).toBe(false);
    expect(() => nativeBuildGenerationV1(kind as "warm" | "fresh", cache, shared)).toThrow(/256/u);
    expect(existsSync(parent)).toBe(law.expectedEffects);
  }
  const absolute = join(root, "relative"), result = nativeBuildGenerationV1("warm", relative(process.cwd(), absolute), { target: relative(process.cwd(), join(root, "t")), build: relative(process.cwd(), join(root, "b")) });
  for (const value of [result.root, result.target, result.build, result.report, result.logs, result.jobserver, result.stages]) expect(isAbsolute(value)).toBe(true);
});

test("pending stages and finalizers retain original numeric authority and settle without aborting its signal", async () => {
  const law = fixture.originalStageAuthority, validator = new Ajv({ strict: true }).compile({ type: "number", exclusiveMinimum: 0, maximum: 1 });
  expect(validator(law.fractionalRemaining)).toBe(true);
  for (const stage of law.stages) {
    const root = mkdtempSync(join(artifacts, "a-")), cargo = stubCargo(root), pass = stubWorkspace(root, "authority", ["unit"], "prototype"), controller = new AbortController();
    let entered = false, published = 0, continued = 0, fractional = false;
    const start = performance.now(), original = { signal: controller.signal, remainingMilliseconds: () => fractional ? law.fractionalRemaining : law.milliseconds - (performance.now() - start), publish: async (progress: { resource: string }) => { await Bun.sleep(5); if (progress.resource === "native-stage-law") published++; }, yieldContinuation: async () => { await Bun.sleep(5); if (entered) continued++; } };
    const safety = setTimeout(() => controller.abort(Error("Test safety cancellation")), law.safetyMilliseconds);
    try {
      await expect(runStub(root, { version: 1, passes: [pass] }, cargo, { control: original, stages: () => [{ name: "authority", capacity: stage === "finalizer"?"cleanup":"single", after: [], run: async (context) => {
        entered = true;
        expect(context.signal).toBe(original.signal);
        expect(context.control.signal).toBe(original.signal);
        expect(context.control.remainingMilliseconds()!).toBeLessThan(law.milliseconds);
        await context.control.publish({ resource: "native-stage-law", mode: "exclusive", elapsedMs: 0 });
        await context.control.yieldContinuation();
        fractional = true;
        expect(context.control.remainingMilliseconds()).toBe(law.fractionalRemaining);
        fractional = false;
        if (stage === "owned-child") return captureOwnedCommandV1(process.execPath, ["-e", `import {writeFileSync} from "node:fs";writeFileSync(${JSON.stringify(join(root, "owned-stage.json"))},JSON.stringify({pid:process.pid}));await new Promise(()=>setInterval(()=>{},1000));`], root, { control: context.control }).then(() => undefined);
        await new Promise<void>(() => undefined);
      } }] })).rejects.toThrow(new RegExp(law.expectedError, "u"));
    } finally { clearTimeout(safety); }
    expect(entered).toBe(true);
    expect(controller.signal.aborted).toBe(law.expectedSignalAborted);
    expect(published).toBe(law.expectedPublished);
    expect(continued).toBe(law.expectedContinuations);
    if (stage === "owned-child") {
      await Bun.sleep(law.ownedChildSettleMilliseconds);
      const pid = JSON.parse(readFileSync(join(root, "owned-stage.json"), "utf8")).pid;
      let live = true;
      try { process.kill(pid, 0); } catch { live = false; }
      if (live) process.kill(pid, "SIGKILL");
      expect(live).toBe(false);
    }
    expect((await runStub(root, { version: 1, passes: [pass] }, cargo)).failed).toEqual([]);
  }
}, 90_000);

test("the original finite caller budget stops running metadata and returns admission", async () => {
  const vector = fixture.finiteCallerBudget, root = mkdtempSync(join(artifacts, "d-")), cargo = stubCargo(root), pass = stubWorkspace(root, "deadline", ["unit"], "prototype"), controller = new AbortController();
  const safety = setTimeout(() => controller.abort(Error("Test safety cancellation")), vector.safetyMilliseconds);
  try {
    await expect(runStub(root, { version: 1, passes: [pass] }, cargo, { cancel: { signal: controller.signal, milliseconds: vector.milliseconds } }, "unit-hang")).rejects.toThrow(vector.expectedError);
  } finally { clearTimeout(safety); }
  const pid = JSON.parse(readFileSync(join(root, "unit-calls.jsonl"), "utf8").split("\n")[0]!).pid;
  expect(() => process.kill(pid, 0)).toThrow();
  expect(controller.signal.aborted).toBe(false);
  expect((await runStub(root, { version: 1, passes: [pass] }, cargo)).failed).toEqual([]);
}, 45_000);


test("unlaned same-profile Cargo jobs reserve tokens before mutable state", async () => {
  const vector = fixture.unlanedStateOrder, root = mkdtempSync(join(artifacts, "o-")), cargo = stubCargo(root), pass = stubWorkspace(root, "w", ["order-component"], vector.profile), marker = join(root, "job.json"), script = join(root, "job", "📜️script.ts");
  mkdirSync(dirname(script), { recursive: true });
  const leaseModule = new URL("../../🔒️lease/🟦️.ts", import.meta.url).href, resourceModule = new URL("../../../../🔒️leases/🟦️.ts", import.meta.url).href;
  writeFileSync(script, `import { acquireCargoBuildLeaseV1 } from ${JSON.stringify(leaseModule)};
import { resourceLeasePathV1 } from ${JSON.stringify(resourceModule)};
import { phaseCompilerAdmissionControlV1 } from ${JSON.stringify(new URL("../../../../🏗️build/🎟️capacity/🟦️.ts", import.meta.url).href)};
import { spawnSync } from "node:child_process";
const signal = AbortSignal.timeout(${vector.deadlineMs - 1000});
const lease = await acquireCargoBuildLeaseV1({ directory: ${JSON.stringify(artifacts)}, buildDirectory: process.env.CARGO_BUILD_BUILD_DIR, targetDirectory: process.env.CARGO_TARGET_DIR, args: ["--profile", ${JSON.stringify(vector.profile)}], signal, control: phaseCompilerAdmissionControlV1(signal, ${vector.deadlineMs - 1000}, () => undefined, () => new Promise(done => setImmediate(done))) });
try {
 const oracle = spawnSync(testPythonV1()??"python3", ["-c", "import sqlite3,sys;d=sqlite3.connect(sys.argv[1],timeout=0);\\ntry:d.execute('begin exclusive');print('unheld')\\nexcept sqlite3.OperationalError:print('held')", resourceLeasePathV1(${JSON.stringify(artifacts)}, lease.resource)], { encoding: "utf8" });
 if (oracle.status !== 0 || oracle.stdout.trim() !== ${JSON.stringify(vector.oracle)}) throw Error("Independent state lease oracle disagrees: " + oracle.stderr);
 await Bun.write(${JSON.stringify(marker)}, JSON.stringify({ participant: "job", oracle: oracle.stdout.trim() }));
} finally { lease.release(); }
`);
  const job: NativeBuildJobV1 = { id: "same-state", cwd: root, command: [process.execPath, script], after: [], profile: vector.profile, target: "wasm32-wasip2", workerProtocol: "jobserver" };
  expect(new Ajv({ strict: true, allowUnionTypes: true }).compile(schema)({ version: 1, passes: [pass], jobs: [job] })).toBe(true);
  const result = await runStub(root, { version: 1, passes: [pass], jobs: [job] }, cargo, { tokens: vector.tokens, cancel: { signal: AbortSignal.timeout(vector.deadlineMs), milliseconds: vector.deadlineMs } });
  expect(result.staged).toEqual(["order-component:" + vector.profile]);
  const observed = JSON.parse(readFileSync(marker, "utf8"));
  expect(observed.oracle).toBe(vector.oracle);
  expect([observed.participant, ...(result.staged.length ? ["pass"] : [])].sort()).toEqual(vector.expectedParticipants);
}, 45_000);

test("the driver declares the scheduler registry selection beside its pass selections so the spanning reader covers it", async () => {
  const { nativeBuildSchedulerInputRequestV1 } = await import("../🟦️.ts"), { cargoRegistryReadSelectionV1 } = await import("../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟢️ready/📋️selection/🟦️.ts"), root = resolve(import.meta.dir, "../../../../../../../");
  const control = {} as never, request = nativeBuildSchedulerInputRequestV1({ cwd: root, build: join(root, "b"), target: join(root, "t"), environment: { SEMIO_BUILD_MODE: "prototype" }, control });
  const manifest = join(root, "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/📦️packages/🦀️rust/Cargo.toml");
  expect(request.manifest).toBe(manifest);
  expect(request.arguments).toContain("--locked");
  expect(request.arguments).not.toContain("--offline");
  expect(cargoRegistryReadSelectionV1({ manifest: request.manifest, cwd: request.cwd, offline: false })).toBe(JSON.stringify([manifest, root, false]));
  expect(request.environment.CARGO_TARGET_DIR).toBe(join(root, "t"));
  expect(request.environment.CARGO_BUILD_BUILD_DIR).toBe(join(root, "b"));
});

test("a resumed build counts the units Cargo reused apart from the ones it compiled", () => {
  expect(nativeBuildReusedUnitsV1([{ reason: "compiler-artifact", fresh: true }, { reason: "compiler-artifact", fresh: false }, { reason: "build-script-executed" }, { reason: "compiler-artifact", fresh: true }, { reason: "compiler-artifact" }])).toBe(2);
  expect(nativeBuildReusedUnitsV1([])).toBe(0);
});

/** 🧩️ Proves that a failed pass only observes the complete closure of the components it did link. */
for (const scenario of fixture.partialProvenance.rows) {
  const refusal = scenario.exit === 0 ? undefined : scenario.missing !== null
    ? /Native build failed: stdio .*\(provenance\)/u : scenario.accepted
      ? /^Native build failed: stdio \(release-app\);/u : /Compiler consumed bytes have no matching retained source witness: .*common[\\/]lib\.rs/u;
  registerIsolatedBuildTestV1(import.meta.path, "closure-scoped partial Cargo provenance: " + scenario.name,
    {bodyMilliseconds:120_000,scope:{kind:"body"}}, async checkpoint => {
    if (refusal) installRestartableFailureHandlerV1(() => undefined);
    const vector=fixture.partialProvenance, root=mkdtempSync(join(artifacts,"p-"));
    const pass=stubWorkspace(root,"app",vector.components,"release"), workspace=dirname(pass.manifest);
    const path=(name:string,file:string)=>join(workspace,name,file);
    for(const dependency of vector.dependencies){
      mkdirSync(join(workspace,dependency));
      writeFileSync(path(dependency,"Cargo.toml"),`[package]\nname="${dependency}"\nversion="0.1.0"\nedition="2021"\n[lib]\npath="lib.rs"\n`);
      writeFileSync(path(dependency,"lib.rs"),vector.edited);
      writeFileSync(path(dependency,"build.rs"),"fn main() {}\n");
    }
    const work=readFileSync(pass.manifest,"utf8");
    writeFileSync(pass.manifest,work.replace(JSON.stringify(vector.components),JSON.stringify([...vector.components,...vector.dependencies])));
    for(const [index,row] of pass.packages.entries()){
      const deps=vector.edges.filter(([source])=>source===index).map(([,dep])=>vector.nodes[dep!]![0]);
      writeFileSync(row.manifest,readFileSync(row.manifest,"utf8")+"\n[dependencies]\n"+deps.map(name=>`${name} = { path = "../${name}" }\n`).join(""));
    }
    expect(spawnSync(registeredToolExecutableV1("cargo"),["generate-lockfile","--offline","--manifest-path",pass.manifest],{encoding:"utf8"}).status).toBe(0);
    const command=join(root,"partial.ts"),calls=join(root,"calls"), config={workspace,vector,scenario,calls,target:pass.target,header:[...header],staleHash:Buffer.from(blake3(Buffer.from(vector.original))).toString("hex")};
    writeFileSync(command,String.raw`import {appendFileSync,mkdirSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {pathToFileURL} from "node:url";
import {spawnSync} from "node:child_process";
const args=process.argv.slice(2);
if (["locate-project","update","metadata"].includes(args[0])) {
  const cargo=spawnSync(${JSON.stringify(registeredToolExecutableV1("cargo"))},args,{stdio:"inherit"});
  process.exit(cargo.status??1);
}
const {workspace,vector,scenario,target,calls,header,staleHash}=${JSON.stringify(config)};
const path=(name,file)=>join(workspace,name,file);
const units=vector.nodes.map(([packageName,kind],index)=>{
  const script=kind==="run-custom-build"||kind==="custom-build";
  return {pkg_id:"path+"+pathToFileURL(join(workspace,packageName)).href+"#"+packageName+"@0.1.0",
    target:{name:script?"build-script-build":packageName,kind:[script?"custom-build":kind],crate_types:[script?"bin":kind],src_path:path(packageName,script?"build.rs":"lib.rs")},
    platform:kind==="custom-build"?null:target,mode:kind==="run-custom-build"?kind:"build",features:[],
    dependencies:vector.edges.filter(([from])=>from===index).map(([,child])=>({index:child})),
  };
});
if(process.argv.includes("--unit-graph")){console.log(JSON.stringify({version:1,units,roots:vector.roots}));process.exit(0);}
appendFileSync(calls,"build\n");
const emit=(message)=>console.log(JSON.stringify(message));
for(const [index,unit] of units.entries()){
  if(scenario.missing===index||vector.roots.includes(index)&&!scenario.linked.includes(index))continue;
  const name=vector.nodes[index][0],pkg=unit.pkg_id;
  if(unit.mode==="run-custom-build"){
    const out=join(process.env.CARGO_BUILD_BUILD_DIR,target,name,"out");
    mkdirSync(out,{recursive:true});
    writeFileSync(join(out,"semio-runtime-resource-inputs.jsonl"),index===scenario.runStale?"unparseable unrelated journal\n":JSON.stringify({kind:"read",path:path(name,"build.rs")})+"\n");
    emit({reason:"build-script-executed",package_id:pkg,out_dir:out,linked_libs:[],linked_paths:[],cfgs:[],env:[]});
    continue;
  }
  const out=join(process.env.CARGO_TARGET_DIR,...(unit.platform?[unit.platform]:[]),"release"),kind=unit.target.kind[0],source=unit.target.src_path;
  mkdirSync(out,{recursive:true});
  const stem=kind==="custom-build"?name+"-build":name,file=join(out,stem+(kind==="cdylib"?".wasm":".rlib"));
  writeFileSync(file,Buffer.concat([Buffer.from(header),Buffer.from(name)]));
  writeFileSync(join(out,stem+".d"),file+": "+source.replaceAll(" ","\\ ")+"\n"+
   (scenario.stale===index?"# checksum:blake3="+staleHash+" file_len:"+Buffer.byteLength(vector.original)+" "+source+"\n":""));
  emit({reason:"compiler-artifact",package_id:pkg,manifest_path:path(name,"Cargo.toml"),target:unit.target,
    features:unit.features,profile:{test:false},filenames:[file],executable:null,fresh:false});
}
if(scenario.exit!==0){
  const failed=units[2];
  emit({reason:"compiler-message",package_id:failed.pkg_id,manifest_path:path("stdio","Cargo.toml"),target:failed.target,
    message:{level:"error",code:{code:"E0599"},message:vector.cargoError,rendered:vector.cargoError,spans:[]}});
}
process.exit(scenario.exit);`);
    const db=new Database(":memory:");
    const expected=new Map<string,number>();
    try{
      db.exec("create table edges(src integer,dst integer); create table nodes(id integer primary key)");
      for(const [src,dst] of vector.edges)db.query("insert into edges values(?,?)").run(src,dst);
      for(const i of vector.nodes.keys())db.query("insert into nodes values(?)").run(i);
      for(const node of scenario.linked){
        const {count}=db.query<{count:number},[number]>("with recursive closure(id) as (select ? union select dst from edges join closure on src=id) select count(*) as count from nodes join closure using(id)").get(node)!;
        expected.set(vector.nodes[node]![0]!,count);
      }
    }finally{db.close();}
    const seen=new Map<string,number>();
    const logs:string[]=[];
    let finish!:()=>void;
    const passFinished=new Promise<void>(resolve=>{finish=resolve;});
    const run=runStub(root,{version:1,passes:[pass]},{command:process.execPath,args:[command],calls},{
      storageBytes:vector.storageBytes,
      log:line=>{logs.push(line);if(line.startsWith("[native-build] "+pass.id)&&line.includes("component(s) linked"))finish();},
      stages:linked=>[{name:"proof",capacity:"io",after:[],async run(context){
        await passFinished;
        const provenance=await context.provenance();
        seen.set(linked.package.name,provenance.units);
      }}],
    },"observe-partial",({outcome,held})=>{
      expect(readFileSync(calls,"utf8").trim().split("\n")).toHaveLength(1);
      const runs=join(held.root,"cache",NATIVE_BUILD_DRIVER_POLICY_V1.warmRoot);
      const report=outcome?.generation.report??join(runs,readdirSync(runs)[0]!,NATIVE_BUILD_DRIVER_POLICY_V1.stores.report);
      const provenance=join(report,NATIVE_BUILD_DRIVER_POLICY_V1.provenanceDirectory);
      expect(readdirSync(provenance).some(name=>name.startsWith("cargo-unit-provenance-"))).toBe(scenario.exit===0);
      if(scenario.exit!==0)expect(logs.join("\n")).toContain(vector.cargoError);
    });
    if(scenario.exit===0)expect((await run).staged).toHaveLength(scenario.linked.length);
    else await expect(run).rejects.toThrow(refusal);
    expect(seen).toEqual(scenario.accepted?expected:new Map());
    checkpoint();
  }, refusal);
}
