/** 🚶️ Contract: a repository walk of the input precision (the crate index of the Cargo manifests, the TypeScript files that entry globs and tsconfig includes select) is a derived fact. A project that used it holds one row (the digest of the answer) instead of the directories the walk listed; the walk is asked again only when a row it read went stale, and an equal answer keeps every holder valid. Oracle: a Python walk of its own copy of the tree (standard library only) after every recorded and every seeded edit of directories and files anywhere; the answer of the derived fact equals the oracle's after every edit, and whenever every supporting row still holds the answer is the one the holders hold. */
import {afterAll, expect, test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, symlinkSync, writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname, join} from "node:path";
import Ajv from "ajv";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import fixture from "./🔣️.json";
import fixtureSchema from "./🧬️schema/🔣️.json";

const inference = await import("../../🟨️.mjs"), {createScriptInputCache, libraryBootstrap} = inference as any;
await libraryBootstrap;
const observation = await import("../../../../👁️observation/🟨️.mjs") as any, scriptInputs = await import("../../🧭️script-inputs/🟨️.mjs") as any, engine = await import("../../../../📥️inputs/🎯️precision/🟦️.ts") as any;
const here = import.meta.dir, roots: string[] = [];
afterAll(() => { for (const root of roots) rmSync(root, {recursive: true, force: true}); });

type Op = {op: "write"; path: string; text: string} | {op: "mkdir"; path: string} | {op: "remove"; path: string} | {op: "rename"; from: string; to: string} | {op: "symlink"; path: string; target: string};
type Question = {mode: "entries"; globs: string[]} | {mode: "include"; include: string[]; exclude: string[]};
const skip = fixture.skip, questions = fixture.questions as Record<string, Question>;

const put = (root: string, path: string, text: string) => { mkdirSync(dirname(join(root, path)), {recursive: true}); writeFileSync(join(root, path), text); };
const apply = (root: string, op: Op) => {
  if (op.op === "write") put(root, op.path, op.text);
  else if (op.op === "mkdir") mkdirSync(join(root, op.path), {recursive: true});
  else if (op.op === "remove") rmSync(join(root, op.path), {recursive: true, force: true});
  else if (op.op === "rename") { mkdirSync(dirname(join(root, op.to)), {recursive: true}); renameSync(join(root, op.from), join(root, op.to)); }
  else { mkdirSync(dirname(join(root, op.path)), {recursive: true}); symlinkSync(op.target, join(root, op.path)); }
};
const build = (files: Record<string, string>) => { const root = realpathSync(mkdtempSync(join(tmpdir(), "repository-walks-"))); roots.push(root); for (const [path, text] of Object.entries(files)) put(root, path, text); return root; };

/** 🔬️ The walks in place on the plain file system: the production pure functions, no observation. */
function inPlace(root: string) {
  const list = (directory: string) => { try { return readdirSync(join(root, directory), {withFileTypes: true}).map((entry) => ({name: entry.name, directory: entry.isDirectory(), file: entry.isFile(), link: entry.isSymbolicLink()})); } catch { return []; } };
  const readText = (path: string) => { try { return readFileSync(join(root, path), "utf8"); } catch { return undefined; } };
  const host = {exists: (path: string) => existsSync(join(root, path)), listDirectory: (directory: string) => list(directory).map(({name, directory}) => ({name, directory}))};
  return {crates: [...engine.crateNamesV1(engine.cargoManifestDirectoriesV1(list, skip), readText)], sources: Object.fromEntries(Object.entries(questions).map(([name, question]) => [name, question.mode === "entries" ? engine.entryFilesV1(question.globs, host) : engine.includeRootsV1(question.include, question.exclude, host)]))};
}

const recipeOf = (name: string) => name === "crates" ? {fn: "crate-index", skip} : {fn: "source-roots", question: questions[name]};
const askDerived = (root: string, name: string) => observation.observeInferenceSnapshotV1(async () => scriptInputs.deriveClosureFactV1({kind: "derived", path: JSON.stringify({derive: 1, workspaceRoot: root, recipe: recipeOf(name)}), fact: "none"}, createScriptInputCache({workspaceRoot: root}))) as Promise<{fact: string; rows: unknown[]}>;
const stillHolds = (rows: unknown[]) => observation.observeInferenceSnapshotV1(async () => observation.observationRowsHoldV1(rows)) as Promise<boolean>;
const digestOf = (expected: any, name: string) => scriptInputs.derivedWalkAnswerV1(name === "crates" ? expected.crates : expected.sources[name]);

function oracle(sequences: {files: Record<string, string>; steps: {ops: Op[]}[]}[]): any[][] {
  const input = join(realpathSync(mkdtempSync(join(tmpdir(), "repository-walks-oracle-"))), "input.json");
  roots.push(dirname(input));
  writeFileSync(input, JSON.stringify({skip, questions, sequences}));
  const run = spawnSync(testPythonV1() ?? "python3", ["-I", join(here, "🐍️oracle/🐍️.py"), input], {encoding: "utf8", maxBuffer: 256 * 1024 * 1024});
  expect(run.status, run.stderr).toBe(0);
  return JSON.parse(run.stdout);
}

/** 🎲️ Seeded edit sequences over a small random tree: adds, removals, renames and rewrites of files and directories anywhere, including into hidden, vendored and skipped places. */
function seeded(seed: number, count: number, length: number) {
  let state = seed >>> 0;
  const next = () => { state = (state + 0x6d2b79f5) >>> 0; let value = state; value = Math.imul(value ^ (value >>> 15), value | 1); value ^= value + Math.imul(value ^ (value >>> 7), value | 61); return ((value ^ (value >>> 14)) >>> 0) / 4294967296; };
  const pick = <T>(items: readonly T[]): T => items[Math.floor(next() * items.length)]!;
  const directoryNames = ["app", "src", "skip", "dist", "lib", "a", "b", "crates", "node_modules", ".hid", "target", "compose", "temp", "x"], fileNames = ["Cargo.toml", "Cargo.toml", "m.ts", "n.tsx", "o.mts", "p.d.ts", "readme.md", "lib.rs", "q.js"], crateNames = ["alpha", "beta", "gamma", "delta"];
  const text = (name: string) => name === "Cargo.toml" ? `[package]\nname = "${pick(crateNames)}"\n` : "export {};\n";
  return Array.from({length: count}, () => {
    const dirs = new Set<string>([""]), files = new Map<string, string>(), start: Record<string, string> = {}, steps: {ops: Op[]}[] = [];
    const addFile = (directory: string, name: string, body: string) => { const path = directory ? `${directory}/${name}` : name; files.set(path, body); let up = directory; while (up) { dirs.add(up); up = up.includes("/") ? up.slice(0, up.lastIndexOf("/")) : ""; } return path; };
    for (let index = 0; index < 12; index++) { const directory = Array.from({length: Math.floor(next() * 3)}, () => pick(directoryNames)).join("/"), name = pick(fileNames); if (![...files.keys()].some((path) => path === (directory ? `${directory}/${name}` : name) || path.startsWith(`${directory ? `${directory}/` : ""}${name}/`)) && !dirs.has(directory ? `${directory}/${name}` : name)) start[addFile(directory, name, text(name))] = text(name); }
    for (const [path, body] of files) start[path] = body;
    const exists = (path: string) => files.has(path) || dirs.has(path);
    const descendants = (path: string) => [...files.keys()].filter((file) => file.startsWith(`${path}/`)), subdirectories = (path: string) => [...dirs].filter((directory) => directory.startsWith(`${path}/`));
    for (let step = 0; step < length; step++) {
      const kind = pick(["file", "file", "file", "dir", "remove", "remove", "rename", "rewrite"]), ops: Op[] = [];
      if (kind === "file") { const directory = pick([...dirs]), name = pick(fileNames), path = directory ? `${directory}/${name}` : name; if (!exists(path)) { files.set(path, text(name)); ops.push({op: "write", path, text: text(name)}); } }
      else if (kind === "dir") { const parent = pick([...dirs]), name = pick(directoryNames), path = parent ? `${parent}/${name}` : name; if (!exists(path)) { dirs.add(path); ops.push({op: "mkdir", path}); } }
      else if (kind === "remove") { const target = pick([...files.keys(), ...[...dirs].filter(Boolean)]); if (target !== undefined) { for (const file of descendants(target)) files.delete(file); for (const directory of subdirectories(target)) dirs.delete(directory); files.delete(target); dirs.delete(target); ops.push({op: "remove", path: target}); } }
      else if (kind === "rename") { const source = pick([...files.keys(), ...[...dirs].filter(Boolean)]), parent = pick([...dirs]), name = pick([...directoryNames, ...fileNames]), path = parent ? `${parent}/${name}` : name; if (source !== undefined && !exists(path) && !path.startsWith(`${source}/`) && path !== source && dirs.has(parent)) { const moved = files.has(source); if (moved) { files.set(path, files.get(source)!); files.delete(source); } else { for (const file of descendants(source)) { files.set(path + file.slice(source.length), files.get(file)!); files.delete(file); } for (const directory of subdirectories(source)) { dirs.add(path + directory.slice(source.length)); dirs.delete(directory); } dirs.delete(source); dirs.add(path); } let up = parent; while (up) { dirs.add(up); up = up.includes("/") ? up.slice(0, up.lastIndexOf("/")) : ""; } ops.push({op: "rename", from: source, to: path}); } }
      else { const target = pick([...files.keys()].filter((path) => path.endsWith("Cargo.toml"))); if (target !== undefined) { const body = text("Cargo.toml"); files.set(target, body); ops.push({op: "write", path: target, text: body}); } }
      steps.push({ops});
    }
    return {files: start, steps};
  });
}

test("the fixture satisfies its closed schema", () => {
  const validate = new Ajv({strict: false});
  expect(validate.validate(fixtureSchema, fixture), JSON.stringify(validate.errors)).toBe(true);
  expect(JSON.parse(readFileSync(join(here, fixture.$schema), "utf8"))).toEqual(fixtureSchema);
});

test("the recipes of the walks are closed: a question or recipe the schema does not name is refused", () => {
  for (const bad of [{fn: "crate-index"}, {fn: "crate-index", skip: {paths: [], names: []}, extra: 1}, {fn: "source-roots", question: {mode: "entries"}}, {fn: "source-roots", question: {mode: "other", globs: []}}, {fn: "unknown"}, null, "crate-index"]) expect(() => engine.readDerivedWalkV1(bad), JSON.stringify(bad)).toThrow(/Derived walk refused/u);
  expect(engine.readDerivedWalkV1(recipeOf("crates"))).toEqual(recipeOf("crates"));
  for (const name of Object.keys(questions)) expect(engine.readDerivedWalkV1(recipeOf(name))).toEqual(recipeOf(name));
});

/** 🧪️ Replays one sequence against the three answers: the in-place walk, the derived fact asked again and the oracle. */
async function replay(sequence: {files: Record<string, string>; steps: {ops: Op[]}[]}, expected: any[], changes?: readonly Record<string, 1>[]) {
  const root = build(sequence.files), names = ["crates", ...Object.keys(questions)], held = new Map<string, {fact: string; rows: unknown[]}>();
  const tally = {steps: 0, asked: 0, kept: 0, moved: 0, retained: 0};
  for (let step = 0; step <= sequence.steps.length; step++) {
    if (step > 0) for (const op of sequence.steps[step - 1]!.ops) apply(root, op);
    const now = expected[step], place = inPlace(root);
    expect(place.crates, `step ${step}: crates in place`).toEqual(now.crates);
    expect(place.sources, `step ${step}: sources in place`).toEqual(now.sources);
    for (const name of names) {
      const wanted = digestOf(now, name), before = held.get(name);
      const holds = before === undefined ? false : await stillHolds(before.rows);
      if (before !== undefined && holds) { expect(wanted, `step ${step}: ${name} moved although every supporting row holds`).toBe(before.fact); tally.retained++; }
      const answer = await askDerived(root, name);
      expect(answer.fact, `step ${step}: ${name} asked again`).toBe(wanted);
      expect(answer.rows.length, `step ${step}: ${name} support`).toBeGreaterThan(0);
      if (before !== undefined) { tally.asked++; if (!holds && answer.fact === before.fact) tally.kept++; if (answer.fact !== before.fact) tally.moved++; if (changes) expect(answer.fact !== before.fact, `step ${step} (${name})`).toBe(changes[step - 1]![name] === 1); }
      held.set(name, answer);
    }
    tally.steps++;
  }
  return tally;
}

test("every recorded edit of the tree moves the derived answers exactly where the oracle's full walk moves, and an unchanged answer survives stale support", async () => {
  const sequence = {files: fixture.files as Record<string, string>, steps: fixture.steps as {ops: Op[]}[]};
  const tally = await replay(sequence, oracle([sequence])[0]!, fixture.steps.map((step) => step.changes as Record<string, 1>));
  expect(tally.moved).toBeGreaterThan(10);
  expect(tally.kept, "answers that stayed equal while their support went stale").toBeGreaterThan(5);
  expect(tally.retained, "answers asked while every supporting row held").toBeGreaterThan(20);
});

test("seeded random edits of directories and files anywhere agree with the oracle at every step", async () => {
  const sequences = seeded(fixture.random.seed, fixture.random.sequences, fixture.random.steps), expected = oracle(sequences), total = {steps: 0, asked: 0, kept: 0, moved: 0, retained: 0};
  for (const [index, sequence] of sequences.entries()) { const tally = await replay(sequence, expected[index]!); for (const key of Object.keys(total) as (keyof typeof total)[]) total[key] += tally[key]; }
  expect(total.steps).toBe(fixture.random.sequences * (fixture.random.steps + 1));
  expect(total.moved, "edits that moved an answer").toBeGreaterThan(50);
  expect(total.kept, "answers that stayed equal while their support went stale").toBeGreaterThan(50);
  expect(total.retained, "answers asked while every supporting row held").toBeGreaterThan(50);
});

test("the crate index of the real Cargo manifests equals the TOML parser's package names", async () => {
  const root = build(fixture.files as Record<string, string>), entries = await askDerived(root, "crates"), index = new Map<string, string>(inPlace(root).crates as [string, string][]);
  for (const [name, directory] of index) expect((Bun.TOML.parse(readFileSync(join(root, directory, "Cargo.toml"), "utf8")) as {package: {name: string}}).package.name).toBe(name);
  expect(entries.fact).toBe(scriptInputs.derivedWalkAnswerV1([...index]));
  expect([...index.keys()]).toEqual(["alpha", "beta", "other"]);
});

test("a project that uses a walk records one row, the walk is one derived fact with its support apart, and the direct path records every listing", async () => {
  const root = build(fixture.files as Record<string, string>), recipe = recipeOf("include1"), more: Record<string, string> = {};
  for (let index = 0; index < 120; index++) more[`app/gen/g${index}/f.ts`] = "export {};\n";
  for (const [path, text] of Object.entries(more)) put(root, path, text);
  const late = await observation.observeInferenceSnapshotV1(async () => {
    const scripts = createScriptInputCache({workspaceRoot: root, lateClosures: true}), api = scriptInputs.closureApiV1(scripts, root);
    const taken = await observation.captureObservedSourceInputsV1(async () => api.derive(recipe), undefined, false);
    const key = scriptInputs.closureRecipeKeyV1(recipe), resolved = await api.resolve([[key, recipe]]);
    return {taken, resolved, table: api.table(new Set())};
  });
  const direct = await observation.observeInferenceSnapshotV1(async () => {
    const scripts = createScriptInputCache({workspaceRoot: root}), api = scriptInputs.closureApiV1(scripts, root);
    return observation.captureObservedSourceInputsV1(async () => api.derive(recipe), undefined, false);
  });
  expect(late.taken.value).toEqual(direct.value);
  expect(direct.observations.length, "the direct path records every listing").toBeGreaterThan(120);
  expect(late.taken.observations).toHaveLength(1);
  expect(late.taken.observations[0].kind).toBe("closure");
  expect(Object.keys(late.table.closureRecipes)).toEqual([scriptInputs.closureRecipeKeyV1(recipe)]);
  expect(late.resolved.derived).toHaveLength(1);
  const derived = late.resolved.derived[0];
  expect(JSON.parse(derived.row.path).derive).toBe(1);
  expect(derived.row.fact).toBe(scriptInputs.derivedWalkAnswerV1(direct.value));
  expect(derived.support.length).toBeGreaterThan(120);
  expect(late.resolved.rows.every((row: any) => !derived.support.includes(row))).toBe(true);
  expect(late.resolved.rows.length).toBeLessThan(derived.support.length);
  expect(await stillHolds(late.taken.observations)).toBe(true);
});

test("a derived question that no walk answers is refused", async () => {
  const root = build(fixture.files as Record<string, string>);
  for (const recipe of [{fn: "unknown"}, {fn: "crate-index"}]) await expect(scriptInputs.deriveClosureFactV1({kind: "derived", path: JSON.stringify({derive: 1, workspaceRoot: root, recipe}), fact: "x"}, createScriptInputCache({workspaceRoot: root}))).rejects.toBeDefined();
  expect(lstatSync(root).isDirectory()).toBe(true);
});
