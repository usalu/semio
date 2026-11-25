import { registeredToolExecutableV1 } from "../../../../../🧰️tools/🏃️run/🟦️.ts";
import { scriptCompilerAdmissionControlV1, withCompilerCapacityScopeV1 } from "../../../../../🏗️build/🎟️capacity/🟦️.ts";
import { registerIsolatedBuildTestV1, withBuildTestInvocationV1 } from "../../../../../🧪️testing/📥️invocation/🟦️.ts";
import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { createJobserverV1, defaultJobserverStyleV1 } from "../../../🎰️jobserver/🟦️.ts";
import { withNativeBuildStageCapacityV1 } from "../../🎟️stage/🟦️.ts";
import { optimizePluginCoreModuleAsync } from "../../../../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🪶️wasm-opt/🟦️.ts";
import { Database } from "bun:sqlite";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import fixture from "../🧫️fixtures/🔣️.json" with { type: "json" };
import { readUnitHistoryV1, unitHistoryPathV1 } from "../../../🎟️admission/🟦️.ts";
import { lawSourceBoundaryV1, lawStoragePolicyV1, schedulerBinaries, withLawStorageV1 } from "../../../🧪️tests/🦀️cargo/🟦️.ts";
import { readTimelineV1 } from "../../../📈️telemetry/🟦️.ts";
import { nativeBuildComponentStagePathV1, runNativeBuildDriverV1 } from "../../🟦️.ts";
import { type NativeBuildPlanV1 } from "../../📋️contract/🟦️.ts";
import { artifacts, header } from "../🎭️stub/🟦️.ts";

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

registerIsolatedBuildTestV1(import.meta.path, "a real build over wasm32-wasip2 components: both profiles, admitted units, streamed stages, timeline, report and recorded telemetry", { bodyMilliseconds: 600_000, scope: { kind: "body" } }, async () => {
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
  const lines: string[] = [], scheduler = (await schedulerBinaries()).get("semio-build-scheduler")!;
  await withBuildTestInvocationV1(original => withLawStorageV1(scriptCompilerAdmissionControlV1(original, "test:component-pass-real", 600_000), lawStoragePolicyV1.realImageBytes, async (held, control) => {
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
  }, original));
});
