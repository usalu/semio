import { expect, test } from "bun:test";
import { existsSync, readFileSync, mkdirSync, mkdtempSync, writeFileSync, realpathSync, symlinkSync, unlinkSync, renameSync } from "node:fs";
import { tmpdir } from "node:os";
import ignore from "ignore";
import * as bootstrap from "../../⚡️caching/🚀️bootstrap/📜️script.ts";
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
  collectionBoundaries: readonly Readonly<{ path: string; production: boolean }>[];
  nativeVocabulary: readonly { id: string; root: string; inputs: Record<string, string[]>; target: string; samples: readonly { path: string; selected: boolean }[]; refused: boolean }[];
  samples: readonly Readonly<{ id: string; role: "fixture" | "test" | "asset" | "code"; path: string; production: boolean; test: boolean }>[];
}>;

const root = resolve(import.meta.dir, "../../../../../../..");
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const vector = JSON.parse(readFileSync(join(root, library, "🧫️fixtures/⚡️production-cache-input-boundary/🔣️.json"), "utf8")) as Vector;

const nxJson = JSON.parse(readFileSync(join(root, "nx.json"), "utf8"));

/** 🧭️ Applies file-set precedence independently with minimatch. */
function oracle(path: string, patterns: readonly string[], projectRoot = vector.projectRoot): boolean {
  const normalized = patterns.map((pattern) => pattern.replaceAll("{workspaceRoot}", ".").replaceAll("{projectRoot}", projectRoot).replace(/^(!?)\.\//, "$1"));
  const positives = normalized.filter((pattern) => !pattern.startsWith("!"));
  const negatives = normalized.filter((pattern) => pattern.startsWith("!")).map((pattern) => pattern.slice(1));
  return (positives.length === 0 || positives.some((pattern) => minimatch(path, pattern))) && negatives.every((pattern) => !minimatch(path, pattern));
}

test("production cache inputs exclude fixtures while test inputs retain them", () => {
  
  expect(vector["version"]).toEqual(1);
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
  expect(namedInputs.production).toContain("!{workspaceRoot}/**/!(🔨️modules)/{🧫️fixtures,🧫️examples,🧪️fixtures,🧪️examples}/**/*");
  expect(nxJson.namedInputs.production).toContain("!{projectRoot}/**/!(🔨️modules)/{🧫️fixtures,🧫️examples,🧪️fixtures,🧪️examples}/**/*");
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
});


test("native input vocabulary is total while owner overrides and unknown refusals remain exact", () => {
  expect(vector["version"]).toEqual(1);
  for (const row of vector.nativeVocabulary) {
    const project = { name: row.id, type: "lib" as const, data: { root: row.root, namedInputs: row.inputs, targets: { proof: { inputs: [{ input: row.target }] } } } };
    if (row.refused) { expect(() => getTargetInputs(nxJson, project, "proof")).toThrow("not defined"); continue; }
    const patterns = getTargetInputs(nxJson, project, "proof").selfInputs;
    const selected = filterUsingGlobPatterns(row.root, row.samples.filter(sample => sample.path.startsWith(row.root + "/")).map(sample => ({ file: sample.path, hash: sample.path })), patterns).map(file => file.file);
    expect(selected).toEqual(row.samples.filter(sample => sample.selected).map(sample => sample.path));
    expect(selected).toEqual(row.samples.filter(sample => oracle(sample.path, patterns, row.root)).map(sample => sample.path));
  }
});


/** 🧫️ Original Nx hashing and independent minimatch agree on collection ancestry. */
test("exact example collection boundary preserves domain module names", () => {
  const json = JSON.parse(readFileSync(join(root, vector.projectRoot, "📋️project.json"), "utf8"));
  const namedInputs = cacheInternals.projectInputs(json, vector.projectRoot, root, new Map());
  const project = { name: vector.projectName, type: "lib" as const, data: { root: vector.projectRoot, namedInputs, targets: { production: { inputs: ["production"] }, test: { inputs: ["default"] } } } };
  const files = vector.collectionBoundaries.map((row) => ({ file: `${vector.projectRoot}/${row.path}`, hash: row.path }));
  for (const target of ["production", "test"] as const) {
    const patterns = getTargetInputs(nxJson, project, target).selfInputs;
    const actual = new Set(filterUsingGlobPatterns(vector.projectRoot, files, patterns.map((pattern) => pattern.replaceAll("{workspaceRoot}", "."))).map((file) => file.file));
    expect(actual).toEqual(new Set(files.filter((file) => oracle(file.file, patterns)).map((file) => file.file)));
    for (const row of vector.collectionBoundaries) expect(actual.has(`${vector.projectRoot}/${row.path}`), `${target}: ${row.path}`).toBe(target === "test" || row.production);
  }
  const productionProject = `${vector.projectRoot}/📋️project.json`;
  const example = vector.nativeProject.fixturePath;
  expect(cacheInternals.inferenceCandidates([example, productionProject], root)).toEqual([productionProject]);
  console.log("[DEBUG] actual Nx production/test input selection and minimatch agree on four collection roots, genuine module names and nested examples; physical inference refuses testing examples");
});


/** 🛡️ Current Nx core gitignore sees only genuine project custody before discovery. */
test("core project discovery excludes examples and preserves exact domain manifest files", async () => {
  const policy = JSON.parse(readFileSync(join(root, library, "⚡️caching/🔣️policy.json"), "utf8"));
  expect(policy.exampleCollections).toBeDefined();
  const schema = JSON.parse(readFileSync(join(root, library, "⚡️caching/🧬️schema/🔣️.json"), "utf8"));
  const validate = new Ajv({ strict: false }).compile(schema);
  expect(validate(policy)).toBe(true);
  for (const field of ["names", "moduleMember", "manifestBasenames"]) {
    const omitted = { ...policy.exampleCollections }; delete omitted[field];
    expect(validate({ ...policy, exampleCollections: omitted })).toBe(false);
  }
  expect(validate({ ...policy, exampleCollections: { ...policy.exampleCollections, names: ["fixtures"] } })).toBe(false);
  const publish = (bootstrap as Record<string, unknown>).publishNxCollectionBoundary as (root: string, signal?: AbortSignal, progress?: (directories: number) => void) => Promise<void>;
  expect(typeof publish).toBe("function");
  const parent = process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir(); mkdirSync(parent, { recursive: true });
  const workspace = mkdtempSync(join(realpathSync(parent), "nc-"));
  const files: { path: string; ignored: boolean }[] = [];
  for (const row of vector.collectionBoundaries) for (const basename of ["project.json", "📋️project.json", "package.json"]) {
    const path = row.path.slice(0, row.path.lastIndexOf("/") + 1) + basename;
    const absolute = join(workspace, path); expect(absolute.length).toBeLessThanOrEqual(256); expect(Array.from(absolute).length).toBeLessThanOrEqual(256);
    mkdirSync(join(absolute, ".."), { recursive: true }); writeFileSync(absolute, JSON.stringify({ name: "neutral", targets: {} }));
    files.push({ path, ignored: !row.production });
  }
  writeFileSync(join(workspace, ".nxignore"), "# retained unrelated user rule\n**/node_modules/**\n");
  const directories: number[] = [];
  await publish(workspace, undefined, (count) => directories.push(count));
  const first = readFileSync(join(workspace, ".nxignore"), "utf8");
  expect(first.startsWith("# retained unrelated user rule\n**/node_modules/**\n")).toBe(true);
  const independent = ignore().add(first);
  for (const file of files) expect(independent.ignores(file.path), file.path).toBe(file.ignored);
  expect(directories.length).toBeGreaterThan(0);
  await publish(workspace);
  expect(readFileSync(join(workspace, ".nxignore"), "utf8")).toBe(first);
  await expect(publish(workspace, AbortSignal.abort())).rejects.toBeDefined();
  expect(readFileSync(join(workspace, ".nxignore"), "utf8")).toBe(first);
  const stopped = new AbortController();
  await expect(publish(workspace, stopped.signal, (count) => { if (count === 2) stopped.abort(); })).rejects.toBeDefined();
  expect(readFileSync(join(workspace, ".nxignore"), "utf8")).toBe(first);
  const genuine = files.find(file => !file.ignored && file.path.includes("🧫️fixtures/"))!;
  unlinkSync(join(workspace, genuine.path));
  await publish(workspace);
  expect(ignore().add(readFileSync(join(workspace, ".nxignore"), "utf8")).ignores(genuine.path)).toBe(true);
  const original = readFileSync(join(workspace, ".nxignore"), "utf8"), peer = original + "# concurrent peer input\n";
  let edited = false;
  await expect(publish(workspace, undefined, (count) => { if (count === 2 && !edited) { edited = true; writeFileSync(join(workspace, ".nxignore"), peer); } })).rejects.toThrow("changed");
  expect(readFileSync(join(workspace, ".nxignore"), "utf8")).toBe(peer);
  const alias = join(workspace, "linked");
  symlinkSync(workspace, alias, process.platform === "win32" ? "junction" : "dir");
  await expect(publish(alias)).rejects.toThrow("real directory ancestry");
  expect(readFileSync(join(workspace, ".nxignore"), "utf8")).toBe(peer);
  await publish(workspace);
  expect(readFileSync(join(workspace, ".nxignore"), "utf8")).toContain("# concurrent peer input");
  console.log("[DEBUG] canonical core Nx gitignore rejects four example collections, retains exact genuine module manifests and nested collection refusal, preserves unrelated rules, is idempotent; pre/mid-scan abort and real-ancestor refusal preserve original bytes; stale exceptions disappear and current peer edits are retained");
});


/** 🧷️ A queued original directory cannot become an admitted foreign symlink path. */
test("publisher refuses when its queued module path becomes a symlink", async () => {
  const parent = process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir(); mkdirSync(parent, { recursive: true });
  const workspace = mkdtempSync(join(realpathSync(parent), "nr-")), foreign = mkdtempSync(join(realpathSync(parent), "nf-"));
  for (const candidate of [workspace, foreign]) {
    const directory = candidate === workspace ? join(candidate, "🔨️modules", "🧫️fixtures") : join(candidate, "🧫️fixtures");
    mkdirSync(directory, { recursive: true }); writeFileSync(join(directory, "project.json"), JSON.stringify({ name: "neutral", targets: {} }));
  }
  const original = "# original unrelated authority\n"; writeFileSync(join(workspace, ".nxignore"), original);
  const publish = (bootstrap as Record<string, unknown>).publishNxCollectionBoundary as (root: string, signal?: AbortSignal, progress?: (directories: number) => void) => Promise<void>;
  let replaced = false;
  await expect(publish(workspace, undefined, (count) => {
    if (count === 2 && !replaced) {
      replaced = true; renameSync(join(workspace, "🔨️modules"), join(workspace, "original-input"));
      symlinkSync(foreign, join(workspace, "🔨️modules"), process.platform === "win32" ? "junction" : "dir");
    }
  })).rejects.toThrow("real");
  expect(replaced).toBe(true);
  expect(readFileSync(join(workspace, ".nxignore"), "utf8")).toBe(original);
  console.log("[DEBUG] original queued module authority refuses symlink replacement before exact-file publication; original ignore bytes retained");
});
