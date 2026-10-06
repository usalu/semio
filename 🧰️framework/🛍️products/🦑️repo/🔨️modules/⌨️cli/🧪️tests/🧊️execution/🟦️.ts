import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { basename, dirname, join } from "node:path";
import { installDashboard, dashboardInvocation } from "../../📦️installation/🟦️.ts";
import { dashboardExecutable } from "../../📦️packages/🦀️rust/📜️script.ts";

test("native launch arguments match shared vectors while Nx keeps build and installation scheduling", async () => {
  const fixture=JSON.parse(readFileSync(new URL("../../🧫️fixtures/🧊️execution/🔣️.json",import.meta.url),"utf8"));
  for(const vector of fixture.cases)expect(dashboardInvocation(vector.args)??null).toEqual(vector.expected);
  const {createTaskGraph}=await import("nx/src/tasks-runner/create-task-graph.js");
  const project=JSON.parse(readFileSync(new URL("../../📦️packages/🦀️rust/📋️project.json",import.meta.url),"utf8"));
  const graph={nodes:{[project.name]:{name:project.name,type:"lib",data:{root:"cli",targets:project.targets}}},dependencies:{[project.name]:[]}};
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
