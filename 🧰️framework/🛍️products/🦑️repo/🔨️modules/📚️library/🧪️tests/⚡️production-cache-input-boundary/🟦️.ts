import { expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import Ajv from "ajv";
import { minimatch } from "minimatch";
import { filterUsingGlobPatterns, getTargetInputs } from "nx/src/hasher/task-hasher";
import cachePlugin, { cacheInternals } from "../../🟨️.mjs";

type Vector = Readonly<{
  version: 1;
  projectName: string;
  projectRoot: string;
  nativeProject: Readonly<{ name: string; root: string; fixturePath: string }>;
  samples: readonly Readonly<{ id: string; role: "fixture" | "test" | "asset" | "code"; path: string; production: boolean; test: boolean }>[];
}>;

const root = resolve(import.meta.dir, "../../../../../../../");
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const vector = JSON.parse(readFileSync(join(root, library, "🧫️fixtures/⚡️production-cache-input-boundary/🔣️.json"), "utf8")) as Vector;
const schema = JSON.parse(readFileSync(join(root, library, "🧬️schema/⚡️production-cache-input-boundary/🔣️.json"), "utf8"));
const nxJson = JSON.parse(readFileSync(join(root, "nx.json"), "utf8"));

/** 🧭️ Applies file-set precedence independently with minimatch. */
function oracle(path: string, patterns: readonly string[], projectRoot = vector.projectRoot): boolean {
  const normalized = patterns.map((pattern) => pattern.replaceAll("{workspaceRoot}", ".").replaceAll("{projectRoot}", projectRoot).replace(/^(!?)\.\//, "$1"));
  const positives = normalized.filter((pattern) => !pattern.startsWith("!"));
  const negatives = normalized.filter((pattern) => pattern.startsWith("!")).map((pattern) => pattern.slice(1));
  return (positives.length === 0 || positives.some((pattern) => minimatch(path, pattern))) && negatives.every((pattern) => !minimatch(path, pattern));
}

test("production cache inputs exclude fixtures while test inputs retain them", () => {
  const validate = new Ajv({ strict: false }).compile(schema);
  expect(validate(vector), JSON.stringify(validate.errors)).toBe(true);
  expect(vector.samples.every((sample) => existsSync(join(root, sample.path)))).toBe(true);
  const projectJson = JSON.parse(readFileSync(join(root, vector.projectRoot, "📋️project.json"), "utf8"));
  const namedInputs = cacheInternals.projectInputs(projectJson, vector.projectRoot, root, new Map());
  const project = {
    name: vector.projectName,
    type: "lib" as const,
    data: {
      root: vector.projectRoot,
      namedInputs,
      targets: {
        production: { inputs: ["production"] },
        test: { inputs: ["default"] },
      },
    },
  };
  const files = vector.samples.map((sample) => ({ file: sample.path, hash: sample.id }));
  const select = (target: "production" | "test") => {
    const patterns = getTargetInputs(nxJson, project, target).selfInputs;
    const nxPatterns = patterns.map((pattern) => pattern.replaceAll("{workspaceRoot}", "."));
    const actual = new Set(filterUsingGlobPatterns(vector.projectRoot, files, nxPatterns).map((file) => file.file));
    const independent = new Set(files.filter((file) => oracle(file.file, patterns)).map((file) => file.file));
    expect(actual).toEqual(independent);
    return actual;
  };
  const production = select("production");
  const tests = select("test");
  for (const sample of vector.samples) {
    expect(production.has(sample.path), `production: ${sample.id}`).toBe(sample.production);
    expect(tests.has(sample.path), `test: ${sample.id}`).toBe(sample.test);
  }
  expect(namedInputs.production).toContain("!{workspaceRoot}/**/🧫️fixtures/**/*");
  expect(nxJson.namedInputs.production).toContain("!{workspaceRoot}/**/🧫️fixtures/**/*");
  const nativeJson = JSON.parse(readFileSync(join(root, vector.nativeProject.root, "📋️project.json"), "utf8"));
  const nativeNamedInputs = cacheInternals.projectInputs(nativeJson, vector.nativeProject.root, root, new Map());
  const nativeProject = {
    name: vector.nativeProject.name,
    type: "lib" as const,
    data: {
      root: vector.nativeProject.root,
      namedInputs: nativeNamedInputs,
      targets: {
        production: { inputs: ["nativeSources"] },
        test: { inputs: ["nativeTestSources"] },
      },
    },
  };
  const nativeFile = [{ file: vector.nativeProject.fixturePath, hash: "native-fixture" }];
  const nativeSelected = (target: "production" | "test") => {
    const patterns = getTargetInputs(nxJson, nativeProject, target).selfInputs;
    const nxPatterns = patterns.map((pattern) => pattern.replaceAll("{workspaceRoot}", "."));
    const actual = filterUsingGlobPatterns(vector.nativeProject.root, nativeFile, nxPatterns).length === 1;
    expect(actual).toBe(oracle(vector.nativeProject.fixturePath, patterns, vector.nativeProject.root));
    return actual;
  };
  expect(nativeSelected("production")).toBe(false);
  expect(nativeSelected("test")).toBe(true);
  expect(cachePlugin.name).toBe("@repo/emoji-project-json");
  console.log("[DEBUG] Cache boundary", JSON.stringify({ production: [...production], test: [...tests] }));
});
