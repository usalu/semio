import {registeredToolExecutableV1} from "../../../../🧰️tools/🏃️run/🟦️.ts";
import {pathToFileURL} from "node:url";
import {testPythonV1} from "../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import { phaseCompilerAdmissionControlV1, scriptCompilerAdmissionControlV1 } from "../../../../🏗️build/🎟️capacity/🟦️.ts";
import { withCompilerCapacityConsumptionV1 } from "../../../../🏗️build/🎟️capacity/🪆️scope/🤲️consumption/🟦️.ts";
import { withCompilerWorkspaceLifetimeV1, withCompilerWorkspaceStageWriteV1, withCompilerWriterCustodyV1 } from "../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts";
import { currentLeaseLineageV1, heldLeasesV1, leaseLedgerSnapshotV1 } from "../../../../🔒️leases/🧭️order/🟦️.ts";
import { availableParallelism } from "node:os";
import { registerIsolatedBuildTestV1, withBuildTestInvocationV1 } from "../../../../🧪️testing/📥️invocation/🟦️.ts";
import { captureOwnedCommandV1 } from "../../../../🎛️owned-execution/🟦️.ts";
import { expect, test } from "bun:test";
import Ajv from "ajv";
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { decodeCargoProvenanceV1 } from "../../🧾️receipt/🟦️.ts";
import { formatStampProtocolV1 } from "../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧷️format-stamps/🟨️.mjs";
import { existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import { readUnitHistoryV1, unitHistoryPathV1 } from "../../🎟️admission/🟦️.ts";
import { lawSourceBoundaryV1, lawStoragePolicyV1, schedulerBinaries, withLawStorageV1, type LawStorageV1 } from "../../🧪️tests/🦀️cargo/🟦️.ts";
import { readTimelineV1 } from "../../📈️telemetry/🟦️.ts";
import { nativeBuildReusedUnitsV1, nativeBuildComponentStagePathV1, announcedRunIdentityV1, announcedUnitIdentityV1, componentBatchesV1, reclaimablePassFilesV1, unitFootprintKindV1, unitGraphHistoryKeyV1, componentArtifactV1, NATIVE_BUILD_DRIVER_POLICY_V1, nativeBuildGenerationKindV1, nativeBuildGenerationV1, nativeBuildPassArgsV1, nativeBuildJobWorkerBudgetV1, profileBuildModeV1, runNativeBuildDriverV1, unitGraphClosuresV1, unitGraphIdentityV1, type LinkedComponentV1, type NativeBuildOutcomeV1, type NativeBuildStageV1 } from "../🟦️.ts";
import { ConcurrentEditErrorV1, churnProtocolV1, concurrentEditV1, installRestartableFailureHandlerV1 } from "../../../../../📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import { failedCompilerMessageFilesV1, failedPassConcurrentEditV1 } from "../🔁️failure-churn/🟦️.ts";
import { nativeBuildJobAttemptLogPathV1, nativeBuildJobLogNamesV1, runNativeBuildJobAttemptsV1 } from "../🔁️job-attempts/🟦️.ts";
import{admitNativeBuildPlanV1,type NativeBuildJobV1,type NativeBuildPassV1,type NativeBuildPlanV1} from "../📋️contract/🟦️.ts";

const artifacts = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!);
mkdirSync(artifacts, { recursive: true });
const header = Buffer.from(NATIVE_BUILD_DRIVER_POLICY_V1.componentHeader);

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
  writeFileSync(stub, `import { appendFileSync, mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
const args = process.argv.slice(2), profile = args[args.indexOf("--profile") + 1], manifest = args[args.indexOf("--manifest-path") + 1];
const packages = args.flatMap((arg, index) => args[index - 1] === "-p" ? [arg] : []), root = manifest.slice(0, -"/Cargo.toml".length);
const observing = process.env.STUB_MODE==="observe-linked";
if (args.includes("--unit-graph")) {
  if(process.env.STUB_MODE==="unit-hang"){appendFileSync(${JSON.stringify(join(root,"unit-calls.jsonl"))},JSON.stringify({pid:process.pid})+"\\n");await new Promise(()=>setInterval(()=>{},1000));}
  const units = observing ? packages.map(name=>({pkg_id:"path+"+pathToFileURL(join(root,name)).href+"#"+name+"@0.1.0",target:{name,kind:["cdylib"],crate_types:["cdylib"],src_path:join(root,name,"lib.rs")},platform:"wasm32-wasip2",mode:"build",features:[],dependencies:[]})) : [];
  process.stdout.write(JSON.stringify({ version: 1, units, roots: units.map((_,index)=>index) })); process.exit(0);
}
appendFileSync(${JSON.stringify(calls)}, JSON.stringify({ at: Date.now(), pid: process.pid, profile, manifest, jobserver: process.env.CARGO_MAKEFLAGS ?? null, wrapper: process.env.RUSTC_WRAPPER ?? null, unification: process.env.CARGO_RESOLVER_FEATURE_UNIFICATION ?? null }) + "\\n");
if (process.env.STUB_MODE === "hang") await new Promise(() => setInterval(() => {}, 1000));
const failing = ["error-last", "error-touch"].includes(process.env.STUB_MODE), linked = process.env.STUB_MODE === "omit-last" || failing ? packages.slice(0, -1) : packages;
for (const name of linked) {
  await new Promise((accept) => setTimeout(accept, ${fixture.stub.delayMs} / packages.length));
  const directory = join(process.env.CARGO_TARGET_DIR, "wasm32-wasip2", profile), wasm = join(directory, name.replaceAll("-", "_") + ".wasm"), source = join(root, name, "lib.rs");
  mkdirSync(directory, { recursive: true });
  writeFileSync(wasm, Buffer.concat([Buffer.from(${JSON.stringify([...header])}), Buffer.from(name + profile)]));
  if(observing) writeFileSync(wasm.replace(/\\.wasm$/u,".d"), wasm+": "+source.replaceAll(" ","\\\\ ")+"\\n");
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
async function runStub(root: string, plan: NativeBuildPlanV1, cargo: ReturnType<typeof stubCargo>, overrides: Partial<Parameters<typeof runNativeBuildDriverV1>[0]> & { cancel?: Readonly<{ signal: AbortSignal; milliseconds: number }>; verifyStageWriters?:boolean } = {}, mode = "ok", inspect?: (run: Readonly<{ outcome?: NativeBuildOutcomeV1; held: LawStorageV1 }>) => void | Promise<void>): Promise<NativeBuildOutcomeV1> {
  const { cancel, verifyStageWriters, ...driver } = overrides;
  const settled = await withBuildTestInvocationV1(async (original): Promise<Readonly<{ failed: boolean; failure?: unknown; outcome?: NativeBuildOutcomeV1 }>> => {
    const control = driver.control ?? scriptCompilerAdmissionControlV1(original, "test:component-pass", cancel?.milliseconds ?? 120000, cancel?.signal);
    try {
      return { failed: false, outcome: await withLawStorageV1(control, mode === "observe-linked" ? lawStoragePolicyV1.realImageBytes : lawStoragePolicyV1.stubImageBytes, async (held, control) => {
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

test("the driver seals compiler provenance while independent stages run", async () => {
  const expected = fixture.provenanceSeal;
  for (const scenario of expected.cases) {
    const root=mkdtempSync(join(artifacts,"z-")), cargo=stubCargo(root), pass=stubWorkspace(root,"app",["seal-a"],"release");
    const source=join(dirname(pass.packages[0]!.manifest),"lib.rs");
    writeFileSync(source,expected.originalSource);
    let signalSeal!:()=>void, sealed=false, edited=false;
    const seal=new Promise<void>(resolve=>{signalSeal=resolve;});
    const run=runStub(root,{version:1,passes:[pass]},cargo,{
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
          writeFileSync(scenario.mutation==="source"?source:linked.staged,expected.editedSource);
          edited=true;
        }}];
      },
    },"observe-linked",({outcome,held})=>{
      const generation=outcome?.generation??nativeBuildGenerationV1(join(held.root,"cache"),{target:join(held.root,"target"),build:join(held.root,"build")},"warm");
      const receipts=join(generation.report,NATIVE_BUILD_DRIVER_POLICY_V1.provenanceDirectory);
      const name=readdirSync(receipts).find(name=>name.startsWith("cargo-unit-provenance-"));
      expect(Boolean(name)).toBe(scenario.publishesReceipt);
      if(name){
        const receipt=decodeCargoProvenanceV1(readFileSync(join(receipts,name),"utf8"));
        const original=receipt.units[0]!.inputs.find(row=>row.path===source);
        expect(original?.sha256).toBe(createHash("sha256").update(expected.originalSource).digest("hex"));
      }
    });
    if(scenario.publishesReceipt) expect((await run).staged).toEqual(["seal-a:release"]);
    else await expect(run).rejects.toThrow(/Current physical bytes changed before transfer/u);
    expect(edited).toBe(true);
    expect(sealed).toBe(scenario.timing==="after-seal");
  }
},120_000);

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
  expect(failedCompilerMessageFilesV1({ package_id: "registry+https://example.org/index#foreign@1.0.0", manifest_path: "/deps/foreign/Cargo.toml", message: { spans: [{ file_name: "src/lib.rs" }, { file_name: "<proc-macro source code>" }] } }, cwd)).toEqual([]);
  expect(failedCompilerMessageFilesV1({ package_id: "path+file:///w/plugin/packages/rust#window-config@0.1.0", manifest_path: manifest, message: { spans: [{ file_name: "window-config-0.1.0/../../native/lib.rs" }] } }, cwd)).toEqual([manifest, join(cwd, "plugin", "native", "lib.rs")]);
  expect(failedCompilerMessageFilesV1({ manifest_path: join(cwd, "c", "Cargo.toml"), message: { spans: [{ file_name: "c/src/lib.rs" }, { file_name: join(cwd, "c", "src", "lib.rs") }, { file_name: resolve("/rustc/library/core.rs") }] } }, cwd)).toEqual([join(cwd, "c", "Cargo.toml"), join(cwd, "c", "src", "lib.rs")]);
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
  const finalized: Record<string, { after: number; outcome: string }> = {}, failedAt: Record<string, number> = {};
  const stages = (component: LinkedComponentV1): readonly NativeBuildStageV1[] => [
    { name: "assemble", capacity: "single", after: [], run: async () => { if (component.package.name === "comp-b") { failedAt[component.package.name] = Date.now(); throw Error("assembly refused"); } } },
    { name: "publish", capacity: "single", after: ["assemble"], run: async () => undefined },
    { name: "discard", capacity: "cleanup", after: ["publish"], run: async (context) => { finalized[component.package.name] = { after: Date.now(), outcome: context.signal.aborted ? "aborted" : "settled" }; } },
  ];
  await expect(runStub(root, { version: 1, passes: [pass] }, cargo, { stages })).rejects.toThrow(/Native build failed: comp-b \(assemble\)/u);
  expect(Object.keys(finalized).sort()).toEqual(["comp-a", "comp-b"]);
  expect(finalized["comp-b"]!.after).toBeGreaterThanOrEqual(failedAt["comp-b"]!);
  expect(Object.values(finalized).map((row) => row.outcome)).toEqual(["settled", "settled"]);
  checkpoint();
}, /Native build failed: comp-b/u);


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
import { createHash } from "node:crypto";
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
