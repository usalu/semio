import {phaseCompilerAdmissionControlV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import Ajv from "ajv";
import installationSchema from "../../📦️installation/🧬️schema/🔣️.json";
const installation=()=>({mode:"prototype" as const,control:phaseCompilerAdmissionControlV1(new AbortController().signal,120000,()=>undefined,()=>new Promise(done=>setImmediate(done)))});
import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, statSync, utimesSync, writeFileSync } from "node:fs";
import { basename, dirname, join } from "node:path";
import { captureDashboardSources, digestDashboardSources, installDashboard, dashboardInvocation, mayRunStale, staleDashboard } from "../../📦️installation/🟦️.ts";
import { dashboardExecutable } from "../../📦️packages/🦀️rust/📜️script.ts";

test("native launch arguments match shared vectors while Nx keeps build and installation scheduling", async () => {
  const fixture=JSON.parse(readFileSync(new URL("../🧫️fixtures/🧊️execution/🔣️.json",import.meta.url),"utf8"));
  for(const vector of fixture.cases)expect(dashboardInvocation(vector.args)??null).toEqual(vector.expected);
  for(const vector of fixture.mayRunStale)expect(mayRunStale(vector.args),vector.args.join(" ")).toBe(vector.expected);
  const {createTaskGraph}=await import("nx/src/tasks-runner/create-task-graph.js");
  const project=JSON.parse(readFileSync(new URL("../../📦️packages/🦀️rust/📋️project.json",import.meta.url),"utf8"));
  const graph={nodes:{[project.name]:{name:project.name,type:"lib",data:{root:"dashboard",targets:project.targets}}},dependencies:{[project.name]:[]}};
  expect(Object.keys(createTaskGraph(graph,{},[project.name],["run"],undefined,{}).tasks)).toEqual([`${project.name}:run`]);
  expect(Object.keys(createTaskGraph(graph,{},[project.name],["install"],undefined,{}).tasks).sort()).toEqual([`${project.name}:build`,`${project.name}:install`]);
});

test("dashboard execution preserves old bytes across artifact publication and reuses identical builds", async () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required");
  const root = mkdtempSync(join(output, "de-")), packageRoot = join(root, "package"), name = process.platform === "win32" ? "semio.exe" : "semio";
  mkdirSync(packageRoot,{recursive:true});writeFileSync(join(packageRoot,"Cargo.toml"),'[package]\nname="dashboard-fixture"\nversion="0.0.0"\n');
  mkdirSync(join(packageRoot, "dist/build/prototype"), { recursive: true });
  const source = join(packageRoot, "dist/build/prototype", name), first = Buffer.from("first executable"), second = Buffer.from("second executable");
  writeFileSync(source, first);
  const a = await installDashboard(packageRoot, root, installation());
  expect(basename(dirname(a))).toBe(new Bun.CryptoHasher("sha256").update(first).digest("hex"));
  expect(await dashboardExecutable(packageRoot, root, installation())).toBe(a);
  writeFileSync(source, second);
  expect(await dashboardExecutable(packageRoot, root, installation())).toBe(a);
  const b = await installDashboard(packageRoot, root, installation());
  expect(b).not.toBe(a);
  expect(readFileSync(a)).toEqual(first);
  expect(readFileSync(b)).toEqual(second);
});

function stalenessTree(output: string) {
  const fixture = JSON.parse(readFileSync(new URL("../🧫️fixtures/🧊️execution/🔣️.json", import.meta.url), "utf8")).staleness;
  const root = mkdtempSync(join(output, "ds-"));
  for (const [path, content] of Object.entries<string>(fixture.files)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
  return { fixture, root, packageRoot: join(root, fixture.package) };
}

test("source digest names the crate closure and changes exactly when a compiled source changes", async () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required");
  const { fixture } = stalenessTree(output);
  for (const mutation of fixture.mutations) {
    const { root, packageRoot } = stalenessTree(output);
    const before = await captureDashboardSources(packageRoot, root);
    expect([...before.roots], mutation.name).toEqual(fixture.roots);
    for (const [path, content] of Object.entries<string>(mutation.write ?? {})) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
    for (const [path, seconds] of Object.entries<number>(mutation.touch ?? {})) { const time = statSync(join(root, path)).mtimeMs / 1000 + seconds; utimesSync(join(root, path), time, time); }
    expect(await digestDashboardSources(root, before.roots) !== before.digest, mutation.name).toBe(mutation.stale);
  }
});

test("a recorded installation is stale after a source edit, not after a build output edit, and reports why", async () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required");
  const { root, packageRoot } = stalenessTree(output), name = process.platform === "win32" ? "semio.exe" : "semio";
  expect(await staleDashboard(root,"prototype")).toBe("it is not installed");
  mkdirSync(join(packageRoot, "dist/build/prototype"), { recursive: true });
  writeFileSync(join(packageRoot, "dist/build/prototype", name), "first executable");
  await installDashboard(packageRoot, root, installation());
  expect(await staleDashboard(root,"prototype")).toBeUndefined();
  writeFileSync(join(packageRoot, "dist/build/prototype", name), "second executable");
  expect(await staleDashboard(root,"prototype")).toBeUndefined();
  writeFileSync(join(root, "mod/🎮️registry/🦀️.rs"), "pub fn registry() { 2; }\n");
  expect(await staleDashboard(root,"prototype")).toMatch(/^its sources changed since installation \(\d+ files\)$/);
  await installDashboard(packageRoot, root, installation());
  expect(await staleDashboard(root,"prototype")).toBeUndefined();
  writeFileSync(join(root, "mod/🎮️registry/🆕️.rs"), "pub fn added() {}\n");
  expect(await staleDashboard(root,"prototype")).toMatch(/\d+ -> \d+ files/);
});

test("dashboard installation records and binaries coexist for both selected modes",async()=>{
 const fixture=JSON.parse(readFileSync(new URL("../🧫️fixtures/🧊️execution/🔣️.json",import.meta.url),"utf8")),root=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"dm-")),packageRoot=join(root,"p"),name=process.platform==="win32"?"semio.exe":"semio",oracle=new Ajv({strict:true}).compile(installationSchema),paths=[] as string[];
 mkdirSync(packageRoot,{recursive:true});writeFileSync(join(packageRoot,"Cargo.toml"),'[package]\nname="dashboard-fixture"\nversion="0.0.0"\n');
 for(const mode of fixture.installationModes){mkdirSync(join(packageRoot,"dist/build",mode),{recursive:true});writeFileSync(join(packageRoot,"dist/build",mode,name),mode);const options={...installation(),mode};paths.push(await installDashboard(packageRoot,root,options));const record=JSON.parse(readFileSync(join(root,".🧬semio/🦑️repo/⚡️cache/🎛️dashboard",mode,"installed.json"),"utf8"));expect(oracle(record)).toBe(true);expect(record.mode).toBe(mode);expect(record.path).toContain(`/dashboard-cli/${mode}/`);}
 expect(paths[0]).not.toBe(paths[1]);for(const [index,mode] of fixture.installationModes.entries())expect(await dashboardExecutable(packageRoot,root,{...installation(),mode})).toBe(paths[index]);
});
