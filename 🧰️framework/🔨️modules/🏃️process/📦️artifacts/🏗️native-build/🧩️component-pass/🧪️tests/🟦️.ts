import { blake3 } from "@noble/hashes/blake3.js";
import { pathToFileURL } from "node:url";
import { testPythonV1 } from "../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import { requireGrantedCompileUnitsV1 } from "../../../../🏗️build/🎟️capacity/🟦️.ts";
import { expect, test } from "bun:test";
import Ajv from "ajv";
import { spawnSync } from "node:child_process";
import { CargoSourceAttemptV1 } from "../../🧾️receipt/📎️source-witness/🟦️.ts";
import { formatStampProtocolV1 } from "../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧷️format-stamps/🟨️.mjs";
import { existsSync, mkdtempSync, readdirSync, readFileSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import { componentCompileUnitsV1, nativeBuildReusedUnitsV1, announcedRunIdentityV1, announcedUnitIdentityV1, componentBatchesV1, reclaimablePassFilesV1, unitFootprintKindV1, unitGraphHistoryKeyV1, componentArtifactV1, NATIVE_BUILD_DRIVER_POLICY_V1, nativeBuildGenerationKindV1, nativeBuildGenerationV1, nativeBuildPassArgsV1, nativeBuildJobWorkerBudgetV1, profileBuildModeV1, unitGraphClosuresV1, unitGraphIdentityV1 } from "../🟦️.ts";
import { ConcurrentEditErrorV1, concurrentEditV1 } from "../../../../../📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import { failedCompilerMessageFilesV1, failedPassConcurrentEditV1 } from "../🔁️failure-churn/🟦️.ts";
import { nativeBuildJobAttemptLogPathV1, nativeBuildJobLogNamesV1, runNativeBuildJobAttemptsV1 } from "../🔁️job-attempts/🟦️.ts";
import { admitNativeBuildPlanV1 } from "../📋️contract/🟦️.ts";
import { artifacts } from "./🎭️stub/🟦️.ts";

test("component compiler requests and accepts one free slot without widening its capacity", () => {
  const row = fixture.componentCompileCapacity;
  const hostDecision = { ...row.hostDecision, reason: "memory" as const };
  const { decision, request } = componentCompileUnitsV1(hostDecision);
  expect(decision).toEqual({ ...hostDecision, minimum: 1 });
  expect<unknown>(request).toEqual(row.request);
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
      if(!drift){expect<unknown>([attempt+1,attempt+1]).toEqual([scenario.attempts,scenario.staged]);break;}
      expect(drift).toMatchObject({path:source});
      const key=JSON.stringify(drift);
      if(seen.has(key)||seen.size===retryLimit){expect([attempt+1,null]).toEqual([scenario.attempts,scenario.staged]);break;}
      seen.add(key);
    }
  }
});

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
  expect<unknown>(readdirSync(directory).sort()).toEqual([log, tail].sort());
  expect(readFileSync(join(directory, log)).length).toBe(limit);
  const rows = readFileSync(join(directory, tail!), "utf8").trim().split("\n");
  expect(rows).toEqual([law.overflow.repeat(law.limits.tailLineChars), stderr]);
});

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
