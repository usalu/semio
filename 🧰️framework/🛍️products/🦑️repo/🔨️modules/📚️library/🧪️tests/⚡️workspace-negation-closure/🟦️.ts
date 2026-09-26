/**
 * 🚧️ Workspace-negation closure: Nx subtracts a task's `!{workspaceRoot}/…` negations only from the workspace positives of its
 * expanded inputs; with none, the negation plans every other workspace file and the task hashes the whole repository. The vectors
 * pin that rule against Nx's own hash planner in a throwaway workspace (third-party oracle); nx.json, every 📋️project.json and
 * every named input the plugin derives must be closed.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/⚡️workspace-negation-closure/🔣️.json
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs
 */
import { expect, test } from "bun:test";
import { execFileSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import Ajv from "ajv";
import { testLevelAtLeast } from "../../🟦️.ts";
import { cacheInternals, libraryBootstrap } from "../../🟨️.mjs";

type Inputs = readonly unknown[];
type NamedInputs = Readonly<Record<string, Inputs>>;
type Case = Readonly<{ id: string; namedInputs: Readonly<Record<string, readonly string[]>>; closed: boolean; planned: readonly string[] }>;
type Vector = Readonly<{ version: 1; projectRoot: string; files: readonly string[]; cases: readonly Case[] }>;
type Project = Readonly<{ namedInputs?: NamedInputs; targets?: Readonly<Record<string, Readonly<{ inputs?: Inputs }>>> }>;

const root = resolve(import.meta.dir, "../../../../../../../");
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const vector = JSON.parse(readFileSync(join(root, library, "🧫️fixtures/⚡️workspace-negation-closure/🔣️.json"), "utf8")) as Vector;
const schema = JSON.parse(readFileSync(join(root, library, "🧬️schema/⚡️workspace-negation-closure/🔣️.json"), "utf8"));
const nxJson = JSON.parse(readFileSync(join(root, "nx.json"), "utf8")) as Project;
const projectFiles = () => execFileSync("git", ["ls-files", "-z", "*📋️project.json"], { cwd: root, encoding: "utf8", maxBuffer: 1 << 26 }).split("\0").filter(Boolean);

/** 🧮️ The file sets one input list hashes for its own project, named references expanded (project first, then the workspace). */
function filesets(inputs: Inputs, named: NamedInputs, seen: ReadonlySet<string> = new Set()): string[] {
  return inputs.flatMap((input) => typeof input !== "string" || input.startsWith("^") ? [] : input.includes("{") ? [input] : seen.has(input) || !named[input] ? [] : filesets(named[input]!, named, new Set([...seen, input])));
}

/** 🔒️ Closed: the expanded list carries no workspace negation, or at least one workspace positive it subtracts from. */
function closed(inputs: Inputs, named: NamedInputs): boolean {
  const sets = filesets(inputs, named);
  return !sets.some((set) => set.startsWith("!{workspaceRoot}/")) || sets.some((set) => set.startsWith("{workspaceRoot}/"));
}

/** 🔍️ Every input list of one project (its named inputs and its targets' inline inputs) that is not closed. */
function openLists(owner: string, project: Project): string[] {
  const named = { ...nxJson.namedInputs, ...project.namedInputs };
  const lists = [...Object.entries(project.namedInputs ?? {}).map(([name, inputs]) => [`${owner} namedInputs.${name}`, inputs] as const), ...Object.entries(project.targets ?? {}).flatMap(([name, target]) => target.inputs ? [[`${owner} targets.${name}.inputs`, target.inputs] as const] : [])];
  return lists.filter(([, inputs]) => !closed(inputs, named)).map(([label]) => label);
}

/** 🧭️ Nx's own hash planner (third-party oracle): the files one case's `subject` target plans in a throwaway workspace. */
function planned(testCase: Case): string[] {
  const workspace = mkdtempSync(join(tmpdir(), "semio-negation-closure-")), data = mkdtempSync(join(tmpdir(), "semio-negation-closure-nx-"));
  try {
    for (const file of ["nx.json", ...vector.files]) {
      mkdirSync(dirname(join(workspace, file)), { recursive: true });
      writeFileSync(join(workspace, file), file === "package.json" ? '{ "name": "semio-negation-closure", "private": true }' : file.endsWith(".json") ? "{}" : file);
    }
    const projectJson = `${vector.projectRoot}/project.json`;
    writeFileSync(join(workspace, projectJson), JSON.stringify({ name: "subject", namedInputs: testCase.namedInputs, targets: { subject: { command: "true", cache: true, inputs: ["subject"] } } }));
    const [graph, inspector] = ["nx/src/project-graph/project-graph", "nx/src/hasher/hash-plan-inspector"].map((specifier) => JSON.stringify(Bun.resolveSync(specifier, import.meta.dir)));
    const script = `const { createProjectGraphAsync } = await import(${graph}); const { HashPlanInspector } = await import(${inspector}); const planner = new HashPlanInspector(await createProjectGraphAsync({ exitOnError: true })); await planner.init(); console.log(JSON.stringify(planner.inspectHashPlan(["subject"], ["subject"], undefined, {}, {}, true)["subject:subject"]));`;
    const output = execFileSync(process.execPath, ["-e", script], { cwd: workspace, encoding: "utf8", env: { ...process.env, NX_DAEMON: "false", NX_NO_CLOUD: "true", NX_WORKSPACE_DATA_DIRECTORY: data } });
    const plan = JSON.parse(output.trim().split("\n").at(-1)!) as string[];
    return [...new Set(plan.filter((item) => item.startsWith("file:")).map((item) => item.slice(5)))].filter((file) => file !== "nx.json" && file !== projectJson).sort();
  } finally {
    rmSync(workspace, { recursive: true, force: true });
    rmSync(data, { recursive: true, force: true });
  }
}

test("Nx subtracts a workspace negation only from the workspace positives of the expanded inputs", () => {
  const validate = new Ajv({ strict: false }).compile(schema);
  expect(validate(vector), JSON.stringify(validate.errors)).toBe(true);
  for (const testCase of vector.cases) {
    expect(closed(testCase.namedInputs.subject!, testCase.namedInputs), testCase.id).toBe(testCase.closed);
    expect(planned(testCase), testCase.id).toEqual([...testCase.planned].sort());
  }
}, 120_000);

test("nx.json and every 📋️project.json declare only closed input lists", () => {
  const projects = projectFiles();
  expect(projects.length).toBeGreaterThan(0);
  expect([...openLists("nx.json", { namedInputs: nxJson.namedInputs }), ...projects.flatMap((path) => openLists(path, JSON.parse(readFileSync(join(root, path), "utf8"))))]).toEqual([]);
});

test("the workspace-root project's derived named inputs are closed", async () => {
  await libraryBootstrap;
  expect(openLists("derived .", { namedInputs: cacheInternals.projectInputs(JSON.parse(readFileSync(join(root, "📋️project.json"), "utf8")), ".", root, new Map()) })).toEqual([]);
}, 60_000);

test.if(testLevelAtLeast("long"))("every named input the plugin derives for a project is closed", async () => {
  await libraryBootstrap;
  const facts = new Map();
  const open = projectFiles().flatMap((path) => {
    const projectRoot = path.includes("/") ? dirname(path) : ".";
    return openLists(`derived ${projectRoot}`, { namedInputs: cacheInternals.projectInputs(JSON.parse(readFileSync(join(root, path), "utf8")), projectRoot, root, facts) });
  });
  expect(open).toEqual([]);
}, 600_000);
