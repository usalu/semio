import { registeredToolExecutableV1 } from "../../../../../🧰️tools/🏃️run/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../../🏗️build/🎟️capacity/🟦️.ts";
import { withCompilerCapacityConsumptionV1 } from "../../../../../🏗️build/🎟️capacity/🪆️scope/🤲️consumption/🟦️.ts";
import { withCompilerWorkspaceLifetimeV1, withCompilerWorkspaceStageWriteV1, withCompilerWriterCustodyV1 } from "../../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts";
import { currentLeaseLineageV1, heldLeasesV1, leaseLedgerSnapshotV1 } from "../../../../../🔒️leases/🧭️order/🟦️.ts";
import { availableParallelism } from "node:os";
import { withBuildTestInvocationV1 } from "../../../../../🧪️testing/📥️invocation/🟦️.ts";
import { expect } from "bun:test";
import { spawnSync } from "node:child_process";
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import fixture from "../🧫️fixtures/🔣️.json" with { type: "json" };
import admissionFixture from "../../../🎟️admission/🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };
import { lawSourceBoundaryV1, lawStoragePolicyV1, withLawStorageV1, type LawStorageV1 } from "../../../🧪️tests/🦀️cargo/🟦️.ts";
import { NATIVE_BUILD_DRIVER_POLICY_V1, runNativeBuildDriverV1, type NativeBuildOutcomeV1 } from "../../🟦️.ts";
import { type NativeBuildJobV1, type NativeBuildPassV1, type NativeBuildPlanV1 } from "../../📋️contract/🟦️.ts";

/** 📂️ The artifact directory every stub run writes below. */
export const artifacts = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!);
mkdirSync(artifacts, { recursive: true });
/** 🧱️ The component header every stub links. */
export const header = Buffer.from(NATIVE_BUILD_DRIVER_POLICY_V1.componentHeader);

/** 🎭️ A stub Cargo: logs its pass, links every planned component after a delay (`omit-last` skips the last, `hang` never answers). */
export function stubCargo(root: string): { readonly command: string; readonly args: readonly string[]; readonly calls: string } {
  const calls = join(root, "calls.jsonl"), stub = join(root, "📜️script.ts");
  writeFileSync(stub, `import { appendFileSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { createConnection } from "node:net";
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
appendFileSync(${JSON.stringify(calls)}, JSON.stringify({ at: Date.now(), pid: process.pid, profile, manifest, jobserver: process.env.CARGO_MAKEFLAGS ?? null, jobs: process.env.CARGO_BUILD_JOBS ?? null, light: process.env.SEMIO_UNIT_CACHE_LIGHT_SLOTS ?? null, wrapper: process.env.RUSTC_WRAPPER ?? null, unification: process.env.CARGO_RESOLVER_FEATURE_UNIFICATION ?? null }) + "\\n");
if (process.env.STUB_MODE === "fleet-fallback") {
  const request = { ...${JSON.stringify(admissionFixture.requests.valid[0])}, pass: process.env.SEMIO_BUILD_PASS, pid: process.pid };
  const reply = await new Promise((accept, reject) => {
    const socket = createConnection(process.env.SEMIO_BUILD_BROKER);
    let data = "";
    socket.on("connect", () => socket.write(JSON.stringify(request) + "\\n"));
    socket.on("data", chunk => { data += chunk; if (data.includes("\\n")) { socket.destroy(); accept(JSON.parse(data.split("\\n")[0])); } });
    socket.on("error", reject);
    socket.setTimeout(2000, () => socket.destroy(Error("no broker reply")));
  });
  appendFileSync(${JSON.stringify(calls)}, JSON.stringify({ reply }) + "\\n");
  process.exit(101);
}
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
export function stubWorkspace(root: string, name: string, packages: readonly string[], profile: string): NativeBuildPassV1 {
  const directory = join(root, name);
  for (const row of packages) { mkdirSync(join(directory, row), { recursive: true }); writeFileSync(join(directory, row, "Cargo.toml"), `[package]\nname = "${row}"\nversion = "0.1.0"\nedition = "2021"\n\n[lib]\npath = "lib.rs"\ncrate-type = ["cdylib"]\n`); writeFileSync(join(directory, row, "lib.rs"), "\n"); }
  writeFileSync(join(directory, "Cargo.toml"), `[workspace]\nmembers = ${JSON.stringify(packages)}\nresolver = "2"\n\n[profile.release]\n\n[profile.prototype]\ninherits = "dev"\n`);
  expect(spawnSync(registeredToolExecutableV1("cargo"), ["generate-lockfile", "--offline", "--manifest-path", join(directory, "Cargo.toml")], { encoding: "utf8" }).status).toBe(0);
  return { id: `${profile}-${name}`, manifest: join(directory, "Cargo.toml"), packages: packages.map((row) => ({ name: row, manifest: join(directory, row, "Cargo.toml") })), target: "wasm32-wasip2", profile };
}

/** 🚀️ Runs the driver against a stub Cargo inside one original compiler reservation bound to its control (cache, target and build stores live in that
 * reservation); `inspect` reads the run's generation, whether it succeeded or failed, before the reservation retires. */
export async function runStub(root: string, plan: NativeBuildPlanV1, cargo: ReturnType<typeof stubCargo>, overrides: Partial<Parameters<typeof runNativeBuildDriverV1>[0]> & { cancel?: Readonly<{ signal: AbortSignal; milliseconds: number }>; verifyStageWriters?:boolean; storageBytes?:number } = {}, mode = "ok", inspect?: (run: Readonly<{ outcome?: NativeBuildOutcomeV1; held: LawStorageV1 }>) => void | Promise<void>): Promise<NativeBuildOutcomeV1> {
  const { cancel, verifyStageWriters, storageBytes, environment, ...driver } = overrides;
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
        const run=()=>runNativeBuildDriverV1({ plan, storage: held.storage, sourceBoundary: lawSourceBoundaryV1, cwd: root, cacheRoot: join(held.root, "cache"), shared: { target: join(held.root, "target"), build: join(held.root, "build") }, generation: "warm", leaseDirectory: artifacts, environment: { ...process.env, ...held.environment, CARGO_TARGET_DIR: join(held.root, "target"), RUSTC: "rustc", RUSTC_WRAPPER: "", RUSTC_WORKSPACE_WRAPPER: "", STUB_MODE: mode, ...environment }, control, tokens: fixture.stub.tokens, withCapacity, ...(verifyStageWriters?{withStageWriter}:{}), physicalBytes: fixture.stub.physicalGiB * 1024 ** 3, storageReserveBytes: lawStoragePolicyV1.reserveBytes, scheduler: process.execPath, cargo, log: () => undefined, localJobs: { maximum: 1 }, ...driver });
        try { outcome = await (verifyStageWriters?withCompilerWorkspaceLifetimeV1(held.storage,control,run):run()); return outcome; }
        finally { await inspect?.({ outcome, held }); }
      }, original) };
    } catch (failure) { return { failed: true, failure }; }
  });
  if (settled.failed) throw settled.failure;
  return settled.outcome!;
}

/** 🧾️ A job command that records when it ran and which jobserver it joined. */
export function stubJob(root: string, id: string, after: readonly string[], delayMs: number, exit = 0): NativeBuildJobV1 {
  const script = join(root, `j-${id}`, "📜️script.ts");
  mkdirSync(dirname(script), { recursive: true });
  writeFileSync(script, `import { appendFileSync } from "node:fs";\nconst start = Date.now();\nawait new Promise((accept) => setTimeout(accept, ${delayMs}));\nappendFileSync(${JSON.stringify(join(root, "jobs.jsonl"))}, JSON.stringify({ id: ${JSON.stringify(id)}, start, end: Date.now(), jobserver: process.env.CARGO_MAKEFLAGS ?? null, jobs: process.env.CARGO_BUILD_JOBS ?? null, pass: process.env.SEMIO_BUILD_PASS ?? null, parallelism: Number(process.env.SEMIO_COMPILER_PARALLELISM), capacityHeld: process.env.SEMIO_COMPILER_CAPACITY_HELD, makeflags: process.env.MAKEFLAGS ?? null }) + "\\n");\nprocess.exit(${exit});\n`);
  return { id, cwd: root, command: [process.execPath, script], after, profile: "release", target: "host", workerProtocol: "jobserver" };
}
