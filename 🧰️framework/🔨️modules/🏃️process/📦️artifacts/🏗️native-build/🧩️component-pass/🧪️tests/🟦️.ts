import { expect, test } from "bun:test";
import Ajv from "ajv";
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import fixture from "../🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import { readUnitHistoryV1, unitHistoryPathV1 } from "../../🎟️admission/🟦️.ts";
import { schedulerBinaries } from "../../🧪️tests/🦀️cargo/🟦️.ts";
import { readTimelineV1 } from "../../📈️telemetry/🟦️.ts";
import { admitReleasePlanV1, announcedRunIdentityV1, announcedUnitIdentityV1, componentArtifactV1, RELEASE_DRIVER_POLICY_V1, releaseGenerationKindV1, releaseGenerationV1, releasePassArgsV1, runReleaseDriverV1, unitGraphClosuresV1, unitGraphIdentityV1, type LinkedComponentV1, type ReleaseJobV1, type ReleasePassV1, type ReleasePlanV1, type ReleaseStageV1 } from "../🟦️.ts";

const artifacts = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!);
const header = Buffer.from(RELEASE_DRIVER_POLICY_V1.componentHeader);

/** 🎭️ A stub Cargo: logs its pass, links every planned component after a delay (`omit-last` skips the last, `hang` never answers). */
function stubCargo(root: string): { readonly command: string; readonly args: readonly string[]; readonly calls: string } {
  const calls = join(root, "calls.jsonl"), stub = join(root, "📜️script.ts");
  writeFileSync(stub, `import { appendFileSync, mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const args = process.argv.slice(2), profile = args[args.indexOf("--profile") + 1], manifest = args[args.indexOf("--manifest-path") + 1];
if (args.includes("--unit-graph")) { process.stdout.write(JSON.stringify({ version: 1, units: [], roots: [] })); process.exit(0); }
const packages = args.flatMap((arg, index) => args[index - 1] === "-p" ? [arg] : []), root = manifest.slice(0, -"/Cargo.toml".length);
appendFileSync(${JSON.stringify(calls)}, JSON.stringify({ at: Date.now(), pid: process.pid, profile, manifest, jobserver: process.env.CARGO_MAKEFLAGS ?? null, wrapper: process.env.RUSTC_WRAPPER ?? null, unification: process.env.CARGO_RESOLVER_FEATURE_UNIFICATION ?? null }) + "\\n");
if (process.env.STUB_MODE === "hang") await new Promise(() => setInterval(() => {}, 1000));
const linked = process.env.STUB_MODE === "omit-last" ? packages.slice(0, -1) : packages;
for (const name of linked) {
  await new Promise((accept) => setTimeout(accept, ${fixture.stub.delayMs} / packages.length));
  const directory = join(process.env.CARGO_TARGET_DIR, "wasm32-wasip2", profile), wasm = join(directory, name.replaceAll("-", "_") + ".wasm"), source = join(root, name, "lib.rs");
  mkdirSync(directory, { recursive: true });
  writeFileSync(wasm, Buffer.concat([Buffer.from(${JSON.stringify([...header])}), Buffer.from(name + profile)]));
  process.stdout.write(JSON.stringify({ reason: "compiler-artifact", package_id: "path+file://" + root + "/" + name + "#" + name + "@0.1.0", manifest_path: join(root, name, "Cargo.toml"), target: { kind: ["cdylib"], crate_types: ["cdylib"], name, src_path: source }, profile: { test: false }, features: [], filenames: [wasm], executable: null, fresh: false }) + "\\n");
}
appendFileSync(${JSON.stringify(calls)}, JSON.stringify({ end: Date.now(), profile, manifest }) + "\\n");
process.exit(linked.length === packages.length ? 0 : 101);
`);
  return { command: process.execPath, args: [stub], calls };
}

/** 🗂️ A stub workspace whose planned packages own real manifests and sources. */
function stubWorkspace(root: string, name: string, packages: readonly string[], profile: string): ReleasePassV1 {
  const directory = join(root, name);
  for (const row of packages) { mkdirSync(join(directory, row), { recursive: true }); writeFileSync(join(directory, row, "Cargo.toml"), `[package]\nname = "${row}"\n`); writeFileSync(join(directory, row, "lib.rs"), "\n"); }
  writeFileSync(join(directory, "Cargo.toml"), "[workspace]\n");
  return { id: `${profile}-${name}`, manifest: join(directory, "Cargo.toml"), packages: packages.map((row) => ({ name: row, manifest: join(directory, row, "Cargo.toml") })), target: "wasm32-wasip2", profile, output: profile === "wasm-release" ? "dist/components/release" : "dist/components/prototype" };
}

/** 🚀️ Runs the driver against a stub Cargo in a private cache root. */
function runStub(root: string, plan: ReleasePlanV1, cargo: ReturnType<typeof stubCargo>, overrides: Partial<Parameters<typeof runReleaseDriverV1>[0]> = {}, mode = "ok") {
  return runReleaseDriverV1({ plan, cwd: root, cacheRoot: join(root, "cache"), shared: { target: join(root, "target"), build: join(root, "build") }, generation: "warm", leaseDirectory: artifacts, environment: { ...process.env, STUB_MODE: mode }, signal: new AbortController().signal, tokens: fixture.stub.tokens, physicalBytes: fixture.stub.physicalGiB * 1024 ** 3, scheduler: process.execPath, cargo, log: () => undefined, ...overrides });
}

test("the release plan is schema-first, agrees with Ajv, and refuses duplicate passes and packages", () => {
  const validate = new Ajv({ strict: true, allowUnionTypes: true }).compile(schema);
  for (const plan of fixture.plans.valid) { expect(validate(plan), JSON.stringify(validate.errors)).toBe(true); expect(admitReleasePlanV1(plan)).toEqual(plan as never); }
  for (const plan of fixture.plans.invalid) { expect(validate(plan), JSON.stringify(plan)).toBe(false); expect(() => admitReleasePlanV1(plan)).toThrow(/Invalid release plan/u); }
  for (const plan of fixture.plans.refused) { expect(validate(plan)).toBe(true); expect(() => admitReleasePlanV1(plan)).toThrow(/duplicate/u); }
});

test("unit-graph identities, closures and announced identities follow the language-agnostic vectors", () => {
  const vector = fixture.unitGraph as any;
  expect(vector.graph.units.map(unitGraphIdentityV1)).toEqual(vector.identities);
  const closures = unitGraphClosuresV1(vector.graph, vector.pass);
  expect(Object.fromEntries([...closures].map(([name, closure]) => [name, [...closure].sort()]))).toEqual(vector.closures);
  for (const row of vector.announced) expect(announcedUnitIdentityV1(row.message, row.target)).toBe(row.identity);
  for (const row of vector.runs) expect(announcedRunIdentityV1(row.packageId, row.outDir, row.target)).toBe(row.identity);
});

test("generations: the environment selects warm or fresh; fresh is a new one-letter-store directory, warm compiles into the shared stores", () => {
  for (const row of fixture.generations.environments) expect(releaseGenerationKindV1(row.environment)).toBe(row.kind as never);
  for (const value of fixture.generations.invalid) expect(() => releaseGenerationKindV1({ SEMIO_RELEASE_GENERATION: value })).toThrow();
  const root = mkdtempSync(join(artifacts, "g-")), shared = { target: join(root, "shared-t"), build: join(root, "shared-b") };
  const first = releaseGenerationV1("fresh", root, shared), second = releaseGenerationV1("fresh", root, shared), warm = releaseGenerationV1("warm", root, shared);
  expect(first.root).not.toBe(second.root);
  for (const generation of [first, second]) {
    expect(generation.root.slice(root.length + 1).startsWith(RELEASE_DRIVER_POLICY_V1.generationPrefix)).toBe(true);
    for (const [store, name] of Object.entries(RELEASE_DRIVER_POLICY_V1.stores)) expect((generation as Record<string, unknown>)[store]).toBe(join(generation.root, name));
  }
  expect([warm.target, warm.build]).toEqual([shared.target, shared.build]);
  const another = releaseGenerationV1("warm", root, shared);
  expect(another.root).not.toBe(warm.root);
  for (const run of [warm, another]) { expect(dirname(run.root)).toBe(join(root, RELEASE_DRIVER_POLICY_V1.warmRoot)); expect(run.report).toBe(join(run.root, RELEASE_DRIVER_POLICY_V1.stores.report)); }
  expect(warm.stages).toBe(join(root, "stages"));
  expect(first.stages).toBe(join(first.root, "s"));
});

test("pass arguments and component detection follow the vectors", () => {
  expect(releasePassArgsV1(fixture.args.pass)).toEqual(fixture.args.args);
  for (const row of fixture.artifacts) expect(componentArtifactV1(row.message, fixture.args.pass, row.targetDirectory)).toEqual(row.expected);
});

test("the driver streams each component the moment Cargo links it, runs profiles in parallel lanes and workspaces of one profile in sequence, under one jobserver", async () => {
  const root = mkdtempSync(join(artifacts, "s-")), cargo = stubCargo(root);
  const passes = [stubWorkspace(root, "hub", ["comp-a", "comp-b"], "wasm-release"), stubWorkspace(root, "ext", ["comp-c"], "wasm-release"), stubWorkspace(root, "hub2", ["comp-d"], "wasm-prototype")];
  const outcome = await runStub(root, { version: 1, passes }, cargo);
  expect([...outcome.staged].sort()).toEqual(["comp-a:wasm-release", "comp-b:wasm-release", "comp-c:wasm-release", "comp-d:wasm-prototype"]);
  for (const pass of passes) for (const row of pass.packages) expect([...readFileSync(join(row.manifest, "..", pass.output, `${row.name.replaceAll("-", "_")}.wasm`)).subarray(0, 8)]).toEqual([...header]);
  const calls = readFileSync(cargo.calls, "utf8").trim().split("\n").map((line) => JSON.parse(line));
  const span = (manifest: string) => ({ start: calls.find((row) => row.at && row.manifest === manifest).at, end: calls.find((row) => row.end && row.manifest === manifest).end });
  expect(span(passes[1]!.manifest).start).toBeGreaterThanOrEqual(span(passes[0]!.manifest).end);
  expect(span(passes[2]!.manifest).start).toBeLessThan(span(passes[0]!.manifest).end);
  for (const row of calls.filter((call) => call.at)) { expect(row.jobserver).toMatch(/--jobserver-auth=fifo:/u); expect(row.wrapper).toBe(process.execPath); expect(row.unification).toBe(RELEASE_DRIVER_POLICY_V1.featureUnification); }
  const units = JSON.parse(readFileSync(join(outcome.generation.report, "units.json"), "utf8"));
  const stageA = units.stages.find((stage: any) => stage.component === "comp-a" && stage.stage === "stage");
  expect(stageA.startMs).toBeLessThan(span(passes[0]!.manifest).end);
  expect(units.stages.filter((stage: any) => stage.stage === "provenance").length).toBe(passes.length);
  const provenance = readdirSync(join(outcome.generation.report, RELEASE_DRIVER_POLICY_V1.provenanceDirectory));
  expect(provenance.filter((name) => name.startsWith("cargo-unit-provenance-"))).toHaveLength(passes.length);
  expect(provenance).toEqual(expect.arrayContaining([RELEASE_DRIVER_POLICY_V1.provenanceLeavesFile, RELEASE_DRIVER_POLICY_V1.provenanceReadsFile]));
  const reads = JSON.parse(readFileSync(join(outcome.generation.report, RELEASE_DRIVER_POLICY_V1.provenanceDirectory, RELEASE_DRIVER_POLICY_V1.provenanceReadsFile), "utf8"));
  expect(reads, JSON.stringify(reads.excess)).toMatchObject({ closed: true, maximumReads: 2 });
  expect(readFileSync(outcome.report, "utf8")).toContain("## Critical path");
}, 120_000);

test("a component Cargo did not link fails the release, loses its stale output and still writes the report", async () => {
  const root = mkdtempSync(join(artifacts, "f-")), cargo = stubCargo(root), pass = stubWorkspace(root, "hub", ["comp-a", "comp-b"], "wasm-release");
  const stale = join(pass.packages[1]!.manifest, "..", pass.output);
  mkdirSync(stale, { recursive: true });
  writeFileSync(join(stale, "comp_b.wasm"), header);
  await expect(runStub(root, { version: 1, passes: [pass] }, cargo, {}, "omit-last")).rejects.toThrow(/Release build failed: comp-b/u);
  expect(existsSync(stale)).toBe(false);
  expect(readdirSync(join(root, "cache", RELEASE_DRIVER_POLICY_V1.warmRoot)).some((run) => existsSync(join(root, "cache", RELEASE_DRIVER_POLICY_V1.warmRoot, run, RELEASE_DRIVER_POLICY_V1.stores.report, "report.md")))).toBe(true);
}, 120_000);

test("cancellation stops every Cargo tree, and the stall watchdog stops a pass in which nothing starts or finishes", async () => {
  const root = mkdtempSync(join(artifacts, "x-")), cargo = stubCargo(root), pass = stubWorkspace(root, "hub", ["comp-a"], "wasm-release"), controller = new AbortController();
  const cancelled = runStub(root, { version: 1, passes: [pass] }, cargo, { signal: controller.signal }, "hang");
  setTimeout(() => controller.abort(Error("cancelled by law")), fixture.stub.cancelAfterMs);
  await expect(cancelled).rejects.toThrow();
  const pid = JSON.parse(readFileSync(cargo.calls, "utf8").trim().split("\n")[0]!).pid;
  await Bun.sleep(200);
  expect(() => process.kill(pid, 0)).toThrow();
  const stalledRoot = mkdtempSync(join(artifacts, "y-")), stalledCargo = stubCargo(stalledRoot);
  await expect(runStub(stalledRoot, { version: 1, passes: [stubWorkspace(stalledRoot, "hub", ["comp-a"], "wasm-release")] }, stalledCargo, { stallMs: fixture.stub.stallMs }, "hang")).rejects.toThrow(/stalled/u);
}, 120_000);

test("a real release over wasm32-wasip2 components: both profiles, admitted units, streamed stages, timeline, report and recorded telemetry", async () => {
  const root = mkdtempSync(join(artifacts, "r-")), workspace = join(root, "w");
  mkdirSync(workspace, { recursive: true });
  writeFileSync(join(workspace, "Cargo.toml"), `[workspace]\nmembers = ["base", ${fixture.real.components.map((name) => JSON.stringify(name)).join(", ")}]\nresolver = "2"\n\n[profile.wasm-release]\ninherits = "release"\nopt-level = "s"\n\n[profile.wasm-prototype]\ninherits = "dev"\n`);
  mkdirSync(join(workspace, "base", "src"), { recursive: true });
  writeFileSync(join(workspace, "base", "Cargo.toml"), `[package]\nname = "base"\nversion = "0.1.0"\nedition = "2021"\n`);
  writeFileSync(join(workspace, "base", "src", "lib.rs"), `pub fn mix(seed: u64) -> u64 { (0..seed).fold(seed, |acc, value| acc.rotate_left(5) ^ value.wrapping_mul(0x9e37_79b9_7f4a_7c15)) }\n`);
  for (const name of fixture.real.components) {
    mkdirSync(join(workspace, name, "src"), { recursive: true });
    writeFileSync(join(workspace, name, "Cargo.toml"), `[package]\nname = "${name}"\nversion = "0.1.0"\nedition = "2021"\n\n[lib]\ncrate-type = ["cdylib"]\n\n[dependencies]\nbase = { path = "../base" }\n`);
    writeFileSync(join(workspace, name, "src", "lib.rs"), `#[unsafe(no_mangle)]\npub extern "C" fn run(seed: u64) -> u64 { base::mix(seed) }\n`);
  }
  expect(spawnSync("cargo", ["generate-lockfile", "--offline", "--manifest-path", join(workspace, "Cargo.toml")], { encoding: "utf8" }).status).toBe(0);
  const plan: ReleasePlanV1 = { version: 1, passes: fixture.real.profiles.map((profile, index) => ({ id: `${profile}-${String(index).padStart(2, "0")}`, manifest: join(workspace, "Cargo.toml"), packages: fixture.real.components.map((name) => ({ name, manifest: join(workspace, name, "Cargo.toml") })), target: "wasm32-wasip2", profile, output: profile === "wasm-release" ? "dist/components/release" : "dist/components/prototype" })) };
  const lines: string[] = [];
  const outcome = await runReleaseDriverV1({ plan, cwd: workspace, cacheRoot: join(root, "cache"), shared: { target: join(root, "t"), build: join(root, "b") }, generation: "fresh", leaseDirectory: artifacts, environment: process.env, signal: AbortSignal.timeout(600_000), tokens: fixture.real.tokens, scheduler: (await schedulerBinaries()).get("semio-build-scheduler")!, progressMs: 1000, log: (line) => lines.push(line) });
  expect(outcome.failed).toEqual([]);
  expect(outcome.staged).toHaveLength(fixture.real.components.length * fixture.real.profiles.length);
  for (const pass of plan.passes) for (const row of pass.packages) expect([...readFileSync(join(workspace, row.name, pass.output, `${row.name.replaceAll("-", "_")}.wasm`)).subarray(0, 8)]).toEqual([...header]);
  const units = JSON.parse(readFileSync(join(outcome.generation.report, "units.json"), "utf8"));
  expect(new Set(units.units.map((unit: any) => unit.profile))).toEqual(new Set(fixture.real.profiles));
  for (const unit of units.units) { expect(unit.peakRssBytes).toBeGreaterThan(0); expect(unit.exit).toBe(0); }
  expect(units.criticalPath.at(-1).label).toMatch(/cdylib/u);
  expect(readTimelineV1(join(outcome.generation.report, "timeline.jsonl")).length).toBeGreaterThan(0);
  const history = await readUnitHistoryV1(unitHistoryPathV1(join(root, "cache")));
  expect([...history.keys()].filter((key) => key.startsWith("comp-a@comp_a|wasm32-wasip2|")).sort()).toEqual(fixture.real.profiles.map((profile) => `comp-a@comp_a|wasm32-wasip2|${profile}|cdylib`).sort());
  expect(existsSync(outcome.generation.target)).toBe(false);
  expect(existsSync(outcome.generation.build)).toBe(false);
  expect(lines.some((line) => line.includes("component(s) staged"))).toBe(true);
  console.log(`[component-pass] real release: ${units.units.length} unit(s), ${outcome.staged.length} component(s), report ${outcome.report}`);
}, 600_000);

/** 🧾️ A job command that records when it ran and which jobserver it joined. */
function stubJob(root: string, id: string, after: readonly string[], delayMs: number, exit = 0): ReleaseJobV1 {
  const script = join(root, `job-${id}.ts`);
  writeFileSync(script, `import { appendFileSync } from "node:fs";\nconst start = Date.now();\nawait new Promise((accept) => setTimeout(accept, ${delayMs}));\nappendFileSync(${JSON.stringify(join(root, "jobs.jsonl"))}, JSON.stringify({ id: ${JSON.stringify(id)}, start, end: Date.now(), jobserver: process.env.CARGO_MAKEFLAGS ?? null, pass: process.env.SEMIO_BUILD_PASS ?? null }) + "\\n");\nprocess.exit(${exit});\n`);
  return { id, cwd: root, command: [process.execPath, script], after, profile: "release", target: "host" };
}

test("jobs start as soon as their completions happen, post-link stages stream per component, and a job waits for a whole component stage", async () => {
  const root = mkdtempSync(join(artifacts, "j-")), cargo = stubCargo(root), pass = stubWorkspace(root, "hub", ["comp-a", "comp-b"], "wasm-release");
  const checked: Record<string, number> = {};
  const plan: ReleasePlanV1 = { version: 1, passes: [pass], jobs: [stubJob(root, "tool", [], 50), stubJob(root, "pages", ["job:tool", "components:check"], 10)] };
  const outcome = await runStub(root, plan, cargo, { stages: (component) => [{ name: "check", after: ["job:tool"], run: async () => { checked[component.package.name] = Date.now(); } }] });
  expect(outcome.failed).toEqual([]);
  const jobs = readFileSync(join(root, "jobs.jsonl"), "utf8").trim().split("\n").map((line) => JSON.parse(line)), byId = Object.fromEntries(jobs.map((row) => [row.id, row]));
  const calls = readFileSync(cargo.calls, "utf8").trim().split("\n").map((line) => JSON.parse(line)), passStart = calls.find((row) => row.at).at;
  expect(byId.tool.start).toBeLessThan(passStart + 2000);
  for (const name of ["comp-a", "comp-b"]) expect(checked[name]).toBeGreaterThanOrEqual(byId.tool.end);
  expect(byId.pages.start).toBeGreaterThanOrEqual(Math.max(byId.tool.end, ...Object.values(checked)));
  for (const row of jobs) { expect(row.jobserver).toMatch(/--jobserver-auth=fifo:/u); expect(row.pass).toBe(row.id); }
  const units = JSON.parse(readFileSync(join(outcome.generation.report, "units.json"), "utf8"));
  expect(units.stages.filter((stage: any) => stage.stage === "check").map((stage: any) => stage.component).sort()).toEqual(["comp-a", "comp-b"]);
  expect(readFileSync(outcome.report, "utf8")).toContain("## Stage cache");
}, 120_000);

test("a component Cargo did not link fails every stage and job that waits for it, and a failed job fails the release", async () => {
  const root = mkdtempSync(join(artifacts, "k-")), cargo = stubCargo(root), pass = stubWorkspace(root, "hub", ["comp-a", "comp-b"], "wasm-release");
  let ran = 0;
  const plan: ReleasePlanV1 = { version: 1, passes: [pass], jobs: [stubJob(root, "pages", ["components:check"], 0), stubJob(root, "broken", [], 0, 3)] };
  await expect(runStub(root, plan, cargo, { stages: () => [{ name: "check", after: [], run: async () => { ran++; } }] }, "omit-last")).rejects.toThrow(/Release build failed: (?=.*comp-b)(?=.*job:broken)/u);
  expect(ran).toBe(1);
  expect(readFileSync(join(root, "jobs.jsonl"), "utf8")).not.toContain('"pages"');
}, 120_000);

test("a lane job runs inside its profile's Cargo lane before that lane's passes, and an idle stage widens onto free slots", async () => {
  const root = mkdtempSync(join(artifacts, "l-")), cargo = stubCargo(root), pass = stubWorkspace(root, "hub", ["comp-a"], "wasm-release");
  const widths: number[] = [];
  const plan: ReleasePlanV1 = { version: 1, passes: [pass], jobs: [{ ...stubJob(root, "module", [], 200), lane: "wasm-release", profile: "wasm-release", target: "wasm32-unknown-unknown" }] };
  const outcome = await runStub(root, plan, cargo, { stages: () => [{ name: "wide", after: [], run: async (context) => { widths.push(context.widen(3)); } }] });
  expect(outcome.failed).toEqual([]);
  const module = JSON.parse(readFileSync(join(root, "jobs.jsonl"), "utf8").trim()), passStart = readFileSync(cargo.calls, "utf8").trim().split("\n").map((line) => JSON.parse(line)).find((row) => row.at).at;
  expect(module.end).toBeLessThanOrEqual(passStart);
  expect(widths).toHaveLength(1);
  expect(widths[0]).toBeGreaterThanOrEqual(1);
  expect(widths[0]).toBeLessThanOrEqual(3);
}, 120_000);

test("a finalizer runs once its stages settled, also after one failed, and the failure still fails the release", async () => {
  const root = mkdtempSync(join(artifacts, "z-")), cargo = stubCargo(root), pass = stubWorkspace(root, "hub", ["comp-a", "comp-b"], "wasm-release");
  const finalized: Record<string, { after: number; outcome: string }> = {}, failedAt: Record<string, number> = {};
  const stages = (component: LinkedComponentV1): readonly ReleaseStageV1[] => [
    { name: "assemble", after: [], run: async () => { if (component.package.name === "comp-b") { failedAt[component.package.name] = Date.now(); throw Error("assembly refused"); } } },
    { name: "publish", after: ["assemble"], run: async () => undefined },
    { name: "discard", after: ["publish"], finalizer: true, run: async (context) => { finalized[component.package.name] = { after: Date.now(), outcome: context.signal.aborted ? "aborted" : "settled" }; } },
  ];
  await expect(runStub(root, { version: 1, passes: [pass] }, cargo, { stages })).rejects.toThrow(/Release build failed: comp-b \(assemble\)/u);
  expect(Object.keys(finalized).sort()).toEqual(["comp-a", "comp-b"]);
  expect(finalized["comp-b"]!.after).toBeGreaterThanOrEqual(failedAt["comp-b"]!);
  expect(Object.values(finalized).map((row) => row.outcome)).toEqual(["settled", "settled"]);
}, 120_000);
