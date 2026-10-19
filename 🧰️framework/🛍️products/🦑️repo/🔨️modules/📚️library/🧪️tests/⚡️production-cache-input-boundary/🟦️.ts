import { expect, test } from "bun:test";
import { existsSync, readFileSync, mkdirSync, mkdtempSync, writeFileSync, realpathSync, symlinkSync, unlinkSync, renameSync } from "node:fs";
import { tmpdir } from "node:os";
import ignore from "ignore";
import * as bootstrap from "../../⚡️caching/🚀️bootstrap/📜️script.ts";
import { join, resolve } from "node:path";
import Ajv from "ajv";
import { minimatch } from "minimatch";
import { filterUsingGlobPatterns, getTargetInputs } from "nx/src/hasher/task-hasher";
import { createIssuedResourceControlV1, createScriptProcessEnvelope, withScriptInvocationSignalV1, withScriptProcessEnvelope } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { withBuildTestInvocationV1 } from "@semio-tech/process/testing/invocation";
import cachePlugin, { cacheInternals } from "../../🏗️build/🕸️graph/📥️inference/🟨️.mjs";

type Vector = Readonly<{
  version: 1;
  projectName: string;
  projectRoot: string;
  nativeProject: Readonly<{ name: string; root: string; fixturePath: string }>;
  collectionBoundaries: readonly Readonly<{ path: string; production: boolean }>[];
  nativeVocabulary: readonly { id: string; root: string; inputs: Record<string, string[]>; target: string; samples: readonly { path: string; selected: boolean }[]; refused: boolean }[];
  samples: readonly Readonly<{ id: string; role: "fixture" | "test" | "asset" | "code"; path: string; production: boolean; test: boolean }>[];
}>;

type NamedInputs = NonNullable<Parameters<typeof getTargetInputs>[1]["data"]["namedInputs"]>;
type InputRow = NamedInputs[string][number];
/** 🧭️ Admits one inferred named-input row as the Nx input definition it declares. */
function nxInputRow(row: unknown): InputRow {
  if (typeof row === "string") return row;
  if (typeof row === "object" && row !== null) {
    if ("fileset" in row && typeof row.fileset === "string") return { fileset: row.fileset };
    if ("input" in row && typeof row.input === "string") return "projects" in row && (typeof row.projects === "string" || (Array.isArray(row.projects) && row.projects.every((member) => typeof member === "string"))) ? { input: row.input, projects: row.projects } : { input: row.input };
    if ("env" in row && typeof row.env === "string") return { env: row.env };
    if ("runtime" in row && typeof row.runtime === "string") return { runtime: row.runtime };
  }
  throw Error("Inferred named input must be a string or a fileset, input, env or runtime definition");
}
function nxNamedInputs(inputs: Readonly<Record<string, readonly unknown[]>>): NamedInputs {
  return Object.fromEntries(Object.entries(inputs).map(([name, rows]) => [name, rows.map(nxInputRow)]));
}

/** 🔐️ Publishes through the caller-owned process authority the production Script entrypoint issues, with the test's own narrowed cancellation. */
function publish(workspace: string, signal: AbortSignal = new AbortController().signal, progress?: (directories: number) => void): Promise<void> {
  const envelope = createScriptProcessEnvelope({ version: 1, owner: "repository:nx:collection-boundary-test", maximumElapsedMilliseconds: 0 }, {}, Date.now());
  return withScriptProcessEnvelope(envelope, (original) => withScriptInvocationSignalV1(original, signal, (invocation) => bootstrap.publishNxCollectionBoundary(workspace, invocation, progress)));
}

const root = resolve(import.meta.dir, "../../../../../../..");
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const vector = JSON.parse(readFileSync(join(root, library, "🧪️tests/🧫️fixtures/⚡️production-cache-input-boundary/🔣️.json"), "utf8")) as Vector;

const nxJson = JSON.parse(readFileSync(join(root, "nx.json"), "utf8"));

/** 🧭️ Applies file-set precedence independently with minimatch. */
function oracle(path: string, patterns: readonly string[], projectRoot = vector.projectRoot): boolean {
  const normalized = patterns.map((pattern) => pattern.replaceAll("{workspaceRoot}", ".").replaceAll("{projectRoot}", projectRoot).replace(/^(!?)\.\//, "$1"));
  const positives = normalized.filter((pattern) => !pattern.startsWith("!"));
  const negatives = normalized.filter((pattern) => pattern.startsWith("!")).map((pattern) => pattern.slice(1));
  return (positives.length === 0 || positives.some((pattern) => minimatch(path, pattern))) && negatives.every((pattern) => !minimatch(path, pattern));
}

test("production cache inputs exclude fixtures while test inputs retain them", async() => withBuildTestInvocationV1(async (original) => {
  const storage = await import("../../🏗️build/📥️inputs/💾️storage/🟨️.mjs");
  await storage.withRepositoryBuildGraphFactStoreV1(root, original, createIssuedResourceControlV1(original, "graph:fixture-writer", 0), async () => {
  
  expect(vector["version"]).toEqual(1);
  expect(vector.samples.every((sample) => existsSync(join(root, sample.path)))).toBe(true);
  const projectJson = JSON.parse(readFileSync(join(root, vector.projectRoot, "📋️project.json"), "utf8"));
  const namedInputs = nxNamedInputs(await cacheInternals.projectInputs(projectJson, vector.projectRoot, root, new Map()));
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
  const nativeNamedInputs = nxNamedInputs(await cacheInternals.projectInputs(nativeJson, vector.nativeProject.root, root, new Map()));
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
  nativeSelected("test");
  const closure = (rows: readonly unknown[] | undefined): string | undefined => rows?.map((row) => (typeof row === "object" && row !== null && "env" in row && typeof row.env === "string" && row.env.startsWith("SEMIO_SOURCE_CLOSURE_") ? row.env : undefined)).find((env) => env !== undefined);
  expect(closure(nativeNamedInputs.nativeTestSources)).toBeDefined();
  expect(closure(nativeNamedInputs.nativeTestSources)).not.toBe(closure(nativeNamedInputs.nativeSources));
  expect(cachePlugin.name).toBe("@repo/emoji-project-json");
  });
}), 120_000);


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
test("exact example collection boundary preserves domain module names", async() => {
  const json = JSON.parse(readFileSync(join(root, vector.projectRoot, "📋️project.json"), "utf8"));
  const namedInputs = nxNamedInputs(await cacheInternals.projectInputs(json, vector.projectRoot, root, new Map()));
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
});


/** 🛡️ Current Nx core gitignore sees only genuine project custody before discovery. */
test("core project discovery excludes examples and preserves exact domain manifest files", async () => {
  const policy = JSON.parse(readFileSync(join(root, library, "⚡️caching/🔣️policy.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(root, library, "⚡️caching/🧬️schema/🔣️.json"), "utf8"));
  const validate = new Ajv({ strict: false }).compile(schema);
  expect(validate(policy)).toBe(true);
  expect(validate({ ...policy, exampleCollections: {} })).toBe(false);
  const ownership = JSON.parse(readFileSync(join(root, library, "🗂️workspaces/🔣️policy.json"), "utf8"));
  const ownershipSchema = JSON.parse(readFileSync(join(root, library, "🗂️workspaces/🧬️schema/🔣️.json"), "utf8"));
  const admitOwnership = new Ajv({ strict: true }).compile(ownershipSchema);
  expect(admitOwnership(ownership)).toBe(true);
  for (const field of ["names", "moduleMember"]) {
    const omitted = { ...ownership.collections }; delete omitted[field];
    expect(admitOwnership({ ...ownership, collections: omitted })).toBe(false);
  }
  expect(admitOwnership({ ...ownership, collections: { ...ownership.collections, names: ["fixtures"] } })).toBe(false);
  expect(admitOwnership({ ...ownership, collections: { ...ownership.collections, manifestBasenames: [] } })).toBe(false);
  const { manifestBasenames, ...omittedBasenames } = policy;
  expect(validate(omittedBasenames)).toBe(false);
  expect(validate({ ...policy, manifestBasenames: ["schema.json"] })).toBe(false);
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
  let replaced = false;
  await expect(publish(workspace, undefined, (count) => {
    if (count === 2 && !replaced) {
      replaced = true; renameSync(join(workspace, "🔨️modules"), join(workspace, "original-input"));
      symlinkSync(foreign, join(workspace, "🔨️modules"), process.platform === "win32" ? "junction" : "dir");
    }
  })).rejects.toThrow("real");
  expect(replaced).toBe(true);
  expect(readFileSync(join(workspace, ".nxignore"), "utf8")).toBe(original);
});
