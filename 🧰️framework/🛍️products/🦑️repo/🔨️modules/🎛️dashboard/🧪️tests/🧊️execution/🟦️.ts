import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, statSync, utimesSync, writeFileSync } from "node:fs";
import { basename, dirname, join } from "node:path";
import { captureDashboardSources, digestDashboardSources, installDashboard, dashboardInvocation, mayRunStale, staleDashboard } from "../../📦️installation/🟦️.ts";
import { dashboardExecutable } from "../../📦️packages/🦀️rust/📜️script.ts";

test("native launch arguments match shared vectors while Nx keeps build and installation scheduling", async () => {
  const fixture=JSON.parse(readFileSync(new URL("../../🧫️fixtures/🧊️execution/🔣️.json",import.meta.url),"utf8"));
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
  const root = mkdtempSync(join(output, "dashboard-execution-")), packageRoot = join(root, "package"), name = process.platform === "win32" ? "semio.exe" : "semio";
  mkdirSync(join(packageRoot, "dist/build"), { recursive: true });
  const source = join(packageRoot, "dist/build", name), first = Buffer.from("first executable"), second = Buffer.from("second executable");
  writeFileSync(source, first);
  const a = await installDashboard(packageRoot, root);
  expect(basename(dirname(a))).toBe(new Bun.CryptoHasher("sha256").update(first).digest("hex"));
  expect(await dashboardExecutable(packageRoot, root)).toBe(a);
  writeFileSync(source, second);
  expect(await dashboardExecutable(packageRoot, root)).toBe(a);
  const b = await installDashboard(packageRoot, root);
  expect(b).not.toBe(a);
  expect(readFileSync(a)).toEqual(first);
  expect(readFileSync(b)).toEqual(second);
});

function stalenessTree(output: string) {
  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🧊️execution/🔣️.json", import.meta.url), "utf8")).staleness;
  const root = mkdtempSync(join(output, "dashboard-staleness-"));
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
  expect(await staleDashboard(root)).toBe("it is not installed");
  mkdirSync(join(packageRoot, "dist/build"), { recursive: true });
  writeFileSync(join(packageRoot, "dist/build", name), "first executable");
  await installDashboard(packageRoot, root);
  expect(await staleDashboard(root)).toBeUndefined();
  writeFileSync(join(packageRoot, "dist/build", name), "second executable");
  expect(await staleDashboard(root)).toBeUndefined();
  writeFileSync(join(root, "mod/🎮️registry/🦀️.rs"), "pub fn registry() { 2; }\n");
  expect(await staleDashboard(root)).toMatch(/^its sources changed since installation \(\d+ files\)$/);
  await installDashboard(packageRoot, root);
  expect(await staleDashboard(root)).toBeUndefined();
  writeFileSync(join(root, "mod/🎮️registry/🆕️.rs"), "pub fn added() {}\n");
  expect(await staleDashboard(root)).toMatch(/\d+ -> \d+ files/);
});
