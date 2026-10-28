import {afterAll, expect, test} from "bun:test";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname, join} from "node:path";
import Ajv from "ajv";
import {minimatch} from "minimatch";
import {commandWordsV1, crateNamesV1, deriveInputsV1, dispatchBranchesV1, entryFilesV1, includeRootsV1, excludesV1, globMatchesV1, inputPrecisionLanguageV1, inputPrecisionTextV1, packageNameV1, parseJsoncV1, programInputsV1, readInputPrecisionV1, selectRuleV1, type InputPrecisionHostV1, type ProgramEventV1, type ProgramFactV1} from "../🟦️.ts";
import fixtureSchema from "./🧬️schema/🔣️.json" with {type: "json"};
import fixture from "./🔣️.json" with {type: "json"};
import schema from "../🧬️schema/🔣️.json" with {type: "json"};
import rules from "../🔣️.json" with {type: "json"};

const here = import.meta.dir;
const findRoot = (directory: string): string => (existsSync(join(directory, "nx.json")) ? directory : dirname(directory) === directory ? process.env.SEMIO_WORKSPACE_ROOT ?? "/Users/ueli/Documents/semio" : findRoot(dirname(directory)));
const workspace = findRoot(here);
const files = fixture.workspace.files as Record<string, string>;
const SPEC = /(?:from\s*|import\s*\(\s*|import\s*)["'](\.{1,2}\/[^"']+)["']/gu;
const normalize = (path: string) => path.split("/").reduce<string[]>((parts, part) => (part === ".." ? (parts.pop(), parts) : part === "." || part === "" ? parts : [...parts, part]), []).join("/");

/** 🔬️ What a scenario step observed of one program analysis. */
type ProgramRun = {derived: number; wrote: number; events: string[]; files: string[]; roots: string[]; error?: string};
const digest = (text: string | undefined) => (text === undefined ? null : createHash("sha256").update(text).digest("hex"));

/** 🧰️ The fixture workspace as an engine host: an in-memory tree, a regex relative-import closure with explicit resolution probes, a receipt store and the declared Cargo graph. */
function fixtureHost(tree: Record<string, string> = files, options: {receipts?: Map<string, string>; cancelAfter?: number; run?: ProgramRun} = {}): InputPrecisionHostV1 {
  const closure = (entries: readonly string[]) => {
    const seen = new Set<string>(), pending = [...entries];
    while (pending.length) {
      const file = pending.pop()!;
      if (seen.has(file) || !(file in tree)) continue;
      seen.add(file);
      for (const match of (tree[file] ?? "").matchAll(SPEC)) {
        const base = normalize(`${dirname(file)}/${match[1]}`), hit = [base, `${base}.ts`, `${base}/🟦️.ts`].find((candidate) => candidate in tree);
        if (hit) pending.push(hit);
      }
    }
    return [...seen].sort();
  };
  const directories = new Set(Object.keys(tree).flatMap((path) => path.split("/").slice(0, -1).map((_, index, parts) => parts.slice(0, index + 1).join("/"))));
  const run = options.run ?? {derived: 0, wrote: 0, events: [], files: [], roots: []};
  let checks = 0;
  return {
    exists: (path) => path in tree || directories.has(path),
    readText: (path) => tree[path],
    listDirectory: (path) => {
      const prefix = path ? `${path}/` : "", names = new Map<string, boolean>();
      for (const file of Object.keys(tree)) if (file.startsWith(prefix)) { const rest = file.slice(prefix.length), [name] = rest.split("/"); names.set(name!, rest.includes("/")); }
      return [...names].map(([name, directory]) => ({name, directory}));
    },
    kinds: fixture.workspace.kinds as InputPrecisionHostV1["kinds"],
    importClosure: async (entries) => closure(entries),
    program: {
      compiler: "fixture-compiler-1",
      imports: async (file) => {
        run.derived++;
        const facts: ProgramFactV1[] = [], resolved: string[] = [];
        for (const match of (tree[file] ?? "").matchAll(SPEC)) {
          const base = normalize(`${dirname(file)}/${match[1]}`);
          for (const candidate of [base, `${base}.ts`, `${base}/🟦️.ts`]) {
            facts.push({kind: "file", path: candidate, value: candidate in tree});
            if (candidate in tree) { resolved.push(candidate); break; }
          }
        }
        return {resolved: [...new Set(resolved)].sort(), facts};
      },
      fact: (kind, path) => (kind === "digest" ? digest(tree[path]) : kind === "file" ? path in tree : null),
      receipts: options.receipts && {read: (key) => options.receipts!.get(key), write: (key, image) => { run.wrote++; options.receipts!.set(key, image); }},
      check: () => { if (options.cancelAfter !== undefined && ++checks > options.cancelAfter) throw new Error("cancelled"); },
      report: (event: ProgramEventV1) => { run.events.push(event.phase); },
    },
    cargoManifests: () => Object.keys(tree).filter((path) => path.endsWith("Cargo.toml")).map((path) => path.slice(0, -"Cargo.toml".length).replace(/\/$/u, "")),
    cargoRoots: (crate) => (fixture.workspace.cargoRoots as Record<string, string[]>)[crate] ?? [crate],
    cargoSources: (root, named) => ((named ? undefined : (fixture.workspace.cargoDependencySources as Record<string, string[]>)[root]) ?? (fixture.workspace.cargoSources as Record<string, string[]>)[root]),
    cargoToolchain: fixture.workspace.cargoToolchain,
  };
}

const oracle = JSON.parse(spawnSync(testPythonV1()??"python3", ["-I", join(here, "🐍️oracle/🐍️.py"), join(here, "🔣️.json")], {encoding: "utf8", maxBuffer: 64 * 1024 * 1024}).stdout);
const frozen = fixture.expect as Record<string, any>, freezing = process.env.SEMIO_FREEZE === "1", next: Record<string, unknown> = {};
afterAll(() => { if (freezing) writeFileSync(join(here, "🔣️.json"), JSON.stringify({...fixture, expect: next}, null, 1) + "\n"); });
const document = readInputPrecisionV1(fixture.document);

test("the fixture satisfies its closed schema and the document its derivation schema", () => {
  const validate = new Ajv({strict: false});
  expect(validate.validate(fixtureSchema, fixture), JSON.stringify(validate.errors)).toBe(true);
  expect(validate.validate(schema, fixture.document), JSON.stringify(validate.errors)).toBe(true);
  expect(validate.validate(schema, rules), JSON.stringify(validate.errors)).toBe(true);
});

test("input globs agree with the fixture, with minimatch and with the Python oracle", () => {
  fixture.globs.forEach((row, index) => {
    expect(globMatchesV1(row.pattern, row.path), `${row.pattern} ~ ${row.path}`).toBe(row.match);
    expect(minimatch(row.path, row.pattern), `minimatch ${row.pattern} ~ ${row.path}`).toBe(row.match);
    expect(oracle.globs[index]).toBe(row.match);
  });
});

test("script command words, JSONC documents and Cargo package names agree with the fixture and the Python oracle", () => {
  fixture.commands.forEach((row, index) => {
    expect(commandWordsV1(row.command) ?? null).toEqual(row.words as any);
    expect(oracle.commands[index]).toEqual(row.words);
  });
  fixture.jsonc.forEach((row, index) => {
    expect(parseJsoncV1(row.text)).toEqual(row.value);
    expect(oracle.jsonc[index]).toEqual(row.value);
  });
  fixture.packageNames.forEach((row, index) => {
    expect(packageNameV1(row.manifest) ?? null).toBe(row.name);
    expect(oracle.packageNames[index]).toBe(row.name);
    const parsed = Bun.TOML.parse(row.manifest) as {package?: {name?: string}};
    expect(parsed.package?.name ?? null).toBe(row.name);
  });
});

test("dispatcher branches carry the same facts in TypeScript and in the Python oracle", () => {
  fixture.dispatch.forEach((row, index) => {
    const branches = dispatchBranchesV1(row.source);
    expect(JSON.parse(JSON.stringify(branches))).toEqual(oracle.dispatch[index]);
    expect(branches.map((branch) => branch.key)).toEqual(["alpha", "beta", "gamma", "delta"]);
    expect(branches[0]!.crates).toEqual(["crate-a"]);
    expect(branches[3]!.cargo).toBe(false);
  });
});

test("every fixture target derives the frozen narrowest inputs, and unselected targets derive nothing", async () => {
  const host = fixtureHost(), derived: unknown[] = [];
  for (const row of fixture.derive) {
    const rule = selectRuleV1(document, row.identity as any);
    expect(rule, JSON.stringify(row.identity)).toBeDefined();
    derived.push(rule!.evidence === "assumed" ? {rule: rule!.id, assumed: true} : await deriveInputsV1(document, rule!, row.identity as any, host));
  }
  const unselected = fixture.unselected.map((identity) => selectRuleV1(document, identity as any)?.id ?? null);
  if (freezing) {
    next.derive = derived;
    next.unselected = unselected;
    return;
  }
  expect(derived).toEqual(frozen.derive);
  expect(unselected).toEqual(frozen.unselected);
});

test("a Cargo exporter reads the named crate with its test targets and only the library of its dependencies, and never the output it writes", async () => {
  const host = fixtureHost(), rule = selectRuleV1(document, fixture.cargoExport.identity as any)!, strip = (files: readonly string[]) => files.map((file) => file.replace("{workspaceRoot}/", ""));
  const derived = await deriveInputsV1(document, rule, fixture.cargoExport.identity as any, host);
  expect(strip(derived.files)).toEqual(fixture.cargoExport.files);
  for (const path of fixture.cargoExport.absent) expect(strip(derived.files), path).not.toContain(path);
  const unwritten = await deriveInputsV1(document, rule, {...fixture.cargoExport.identity, outputs: []} as any, host);
  expect(strip(unwritten.files)).toEqual(fixture.cargoExport.withoutOutputs);
  expect(derived.drops).toEqual(["default", "^default"]);
  expect(derived.patterns).toContain("{workspaceRoot}/crates/d/Cargo.toml");
  expect(derived.patterns).toContain("{workspaceRoot}/crates/e/Cargo.toml");
});

test("every narrowed target keeps the retained toolchain identity its replaced groups carried", () => {
  expect(document.retained).toEqual(["sharedGlobals", "{projectRoot}/📋️project.json"]);
  for (const row of frozen.derive as any[]) {
    const rule = document.rules.find((candidate) => candidate.id === row.rule)!;
    if (row.assumed || !rule.replaces.length) continue;
    for (const name of document.retained) expect(row.groups, `${row.rule} keeps ${name}`).toContain(name);
  }
});

test("a document or derivation that cannot be bounded is refused with the stated reason", async () => {
  const host = fixtureHost();
  for (const row of fixture.refusals as any[]) {
    if (row.kind === "document") {
      const mutated = JSON.parse(JSON.stringify(fixture.document));
      if (row.mutate === "wholeRepository") mutated.rules[2].reads.push({globs: ["{workspaceRoot}/**/*"]});
      if (row.mutate === "wholeRepositoryDot") mutated.rules[2].reads.push({globs: ["{workspaceRoot}/./**/*"]});
      if (row.mutate === "unknownProfile") mutated.rules[0].reads = [{profile: "no-such-profile"}];
      if (row.mutate === "duplicateRule") mutated.rules[1].id = mutated.rules[0].id;
      if (row.mutate === "unknownKey") mutated.surprise = true;
      expect(() => readInputPrecisionV1(mutated), row.mutate).toThrow(new RegExp(row.message, "iu"));
    } else {
      const derived = {...document, rules: [{id: row.rule, evidence: "static", description: "refusal", select: {project: row.identity.project}, replaces: [], reads: row.reads}]};
      await expect(deriveInputsV1(derived as any, derived.rules[0] as any, row.identity, host)).rejects.toThrow(new RegExp(row.message, "iu"));
    }
  }
});

/** 📋️ The files `tsc -p <tsconfig> --listFilesOnly` lists for a tree: the third-party oracle of the program closure. */
function tscListed(tree: Record<string, string>, entries?: readonly string[]): string[] {
  const root = mkdtempSync(join(tmpdir(), "input-precision-"));
  try {
    for (const [path, text] of Object.entries(tree)) { mkdirSync(dirname(join(root, path)), {recursive: true}); writeFileSync(join(root, path), text); }
    const tsc = join(workspace, "node_modules/typescript/bin/tsc"), run = spawnSync(process.execPath, [tsc, ...(entries ? [...entries.map((entry) => join(root, entry)), "--module", "ESNext", "--moduleResolution", "Bundler", "--allowImportingTsExtensions", "--noEmit", "--skipLibCheck", "--target", "ESNext", "--listFilesOnly"] : ["-p", join(root, "app/tsconfig.json"), "--listFilesOnly"])], {encoding: "utf8", cwd: root});
    expect(run.status, run.stderr + run.stdout).toBe(0);
    return run.stdout.split("\n").filter((line) => line.startsWith(root + "/") && !line.includes("/node_modules/")).map((line) => line.slice(root.length + 1)).sort();
  } finally {
    rmSync(root, {recursive: true, force: true});
  }
}

test("a tsconfig program equals what tsc --listFilesOnly lists, and the fixture rule never selects an excluded file", async () => {
  const listed = tscListed(files), program = await programInputsV1("app/tsconfig.json", fixtureHost()), declared = new Set([...program.roots, ...program.files.filter((file) => file.endsWith(".ts"))]);
  expect(listed.length).toBeGreaterThan(0);
  for (const file of listed) expect(declared.has(file), `tsc lists ${file}`).toBe(true);
  for (const file of program.roots) expect(listed).toContain(file);
  expect(program.roots).not.toContain("app/src/skip/x.ts");
  expect(program.files).toContain("shared/package.json");
  expect(program.files).toContain("tsconfig.base.json");
});

/** 🔭️ A program rooted at entry globs is what `tsc <files> --listFilesOnly` lists for the files they select (type-only imports included), a negation removes a root, a glob that selects no TypeScript file is refused. */
test("a program rooted at entries equals what tsc lists for those files", async () => {
  for (const entries of [["{workspaceRoot}/app/src/main.ts"], ["{workspaceRoot}/app/src/**/*.ts", "!{workspaceRoot}/app/src/skip/**/*"]]) {
    const roots = entryFilesV1(entries, fixtureHost()), listed = tscListed(files, roots), program = await programInputsV1("app/tsconfig.json", fixtureHost(), entries), declared = new Set([...program.roots, ...program.files.filter((file) => file.endsWith(".ts"))]);
    expect(program.roots).toEqual(roots);
    expect([...declared].sort(), entries.join(" ")).toEqual(listed);
    expect(program.patterns).toEqual([]);
    expect(program.files).toContain("shared/package.json");
  }
  expect(entryFilesV1(["{workspaceRoot}/app/src/**/*.ts", "!{workspaceRoot}/app/src/skip/**/*"], fixtureHost())).not.toContain("app/src/skip/x.ts");
  await expect(programInputsV1("app/tsconfig.json", fixtureHost(), ["{workspaceRoot}/app/src/**/*.rs"])).rejects.toThrow(/no TypeScript file/u);
});

test("the program receipt makes a warm run derive nothing, an edit derive only what it touches, a cancelled run resume and every step equal a cold derivation", async () => {
  const tree: Record<string, string> = {...files}, receipts = new Map<string, string>(), observed: Record<string, {derived: number; wrote: number; events: string[]; error: string | null}> = {};
  for (const step of fixture.program.steps as {name: string; edits: {path: string; text: string | null}[]; corruptReceipt?: boolean; cancelAfter?: number}[]) {
    for (const edit of step.edits) { if (edit.text === null) delete tree[edit.path]; else tree[edit.path] = edit.text; }
    if (step.corruptReceipt) for (const key of receipts.keys()) receipts.set(key, '{"version":1}');
    const run: ProgramRun = {derived: 0, wrote: 0, events: [], files: [], roots: []};
    let error: string | null = null;
    try {
      const result = await programInputsV1(fixture.program.tsconfig, fixtureHost(tree, {receipts, cancelAfter: step.cancelAfter, run}));
      expect(result, step.name).toEqual(await programInputsV1(fixture.program.tsconfig, fixtureHost(tree)));
    } catch (failure) {
      error = (failure as Error).message;
    }
    observed[step.name] = {derived: run.derived, wrote: run.wrote, events: run.events, error};
  }
  const step = (name: string) => observed[name]!;
  expect(step("cold").events).toEqual(["derive", "done"]);
  expect(step("warm")).toEqual({derived: 0, wrote: 0, events: [], error: null});
  expect(step("cancel").error).toBe("cancelled");
  expect(step("cancel").events).toEqual(["derive"]);
  expect(step("cancel").derived + step("resume").derived).toBe(step("corrupt-receipt").derived);
  expect(step("tsconfig-change").derived).toBe(step("corrupt-receipt").derived);
  const final = await programInputsV1(fixture.program.tsconfig, fixtureHost(tree)), declared = new Set([...final.roots, ...final.files.filter((file) => file.endsWith(".ts"))]);
  for (const file of tscListed(tree)) expect(declared.has(file), `tsc lists ${file}`).toBe(true);
  if (freezing) next.program = observed;
  else expect(observed).toEqual(frozen.program);
});

test("the exclusion predicate answers like matching every glob and its /** extension on every short path", () => {
  const patterns = ["✏️s/**/node_modules/**", "✏️s/**/dist/**", "✏️s/**/next-env.d.ts", "**/target/**", "**/out", "js/temp", "temp", "✏️s/**/🧫️fixtures/📁️some/**", "src/skip/**", "a/**/b", "**/c/**", "x/*.ts"];
  const names = ["✏️s", "a", "b", "c", "dist", "node_modules", "next-env.d.ts", "js", "temp", "target", "out", "🧫️fixtures", "📁️some", "skip", "src", "x", "y.ts"], paths: string[] = [];
  const grow = (prefix: string[], depth: number): void => { for (const name of names) { const next = [...prefix, name]; paths.push(next.join("/")); if (depth > 1) grow(next, depth - 1); } };
  grow([], 3);
  const predicate = excludesV1(patterns);
  for (const path of paths) expect(predicate(path), path).toBe(patterns.some((pattern) => globMatchesV1(pattern, path) || globMatchesV1(`${pattern}/**`, path)));
});

test("progress texts exist in every language, the language follows the CLI help rules and cancellation is a refusal not a partial result", () => {
  for (const row of fixture.messages as {environment: Record<string, string>; language?: string; refusal?: string}[]) {
    if (row.refusal === undefined) expect(inputPrecisionLanguageV1(document, row.environment, () => []), JSON.stringify(row.environment)).toBe(row.language as string);
    else expect(() => inputPrecisionLanguageV1(document, row.environment, () => []), JSON.stringify(row.environment)).toThrow(row.refusal);
  }
  const event: ProgramEventV1 = {phase: "progress", tsconfig: "app/tsconfig.json", files: 10, derived: 4, reused: 6, seconds: 1.5};
  expect(inputPrecisionTextV1(document, event, "en")).toBe("app/tsconfig.json: 10 visited, 4 analysed, 6 reused");
  expect(inputPrecisionTextV1(document, event, "de")).toBe("app/tsconfig.json: 10 besucht, 4 analysiert, 6 wiederverwendet");
  expect(inputPrecisionTextV1(document, {...event, phase: "done"}, "en")).toBe("app/tsconfig.json done in 1.5 s");
  expect(() => inputPrecisionTextV1(document, event, "fr")).toThrow(/language missing/u);
});

test.skipIf(process.env.SEMIO_INPUT_PRECISION_REPO !== "1")("the repository rules are valid, every owner route matches an owner declaration and no rule selects the whole repository", () => {
  const document = readInputPrecisionV1(rules);
  const owners = spawnSync("git", ["-c", "core.quotepath=false", "grep", "-l", "\"workspaceCommand\"", "--", ":(glob)**/📋️project.json"], {cwd: workspace, encoding: "utf8"}).stdout.split("\n").filter((path) => path && !path.startsWith(".🧬semio/"));
  const declared: string[] = [];
  for (const path of owners) {
    const project = JSON.parse(readFileSync(join(workspace, path), "utf8"));
    for (const [target, value] of Object.entries<any>(project.targets ?? {})) if (value.metadata?.semio?.workspaceCommand) declared.push(JSON.stringify([value.metadata.semio.workspaceCommand, project.name, target, dirname(path)]));
  }
  expect(document.routes.map((route) => JSON.stringify([route.words, route.project, route.target, route.root])).sort()).toEqual(declared.sort());
  expect(document.retained).toEqual(["sharedGlobals", "{projectRoot}/📋️project.json"]);
  expect(document.rules.filter((rule) => rule.evidence === "assumed").every((rule) => rule.description.length >= 40 && rule.reads.length > 0)).toBe(true);
  expect(document.rules.filter((rule) => rule.evidence !== "assumed").every((rule) => rule.reads.length === 0 || rule.reads.some((read) => !("input" in read)))).toBe(true);
});


test("a host that carries the resolution identity adds one marker per named crate and keeps every other derived input", async () => {
  const host = {...fixtureHost(), cargoToolchain: [] as string[], cargoIdentity: (directory: string) => ({env: `SEMIO_CARGO_LOCK_${directory.replace(/\W/gu, "_")}`})};
  let withMarkers = 0;
  for (const [index, row] of fixture.derive.entries()) {
    const rule = selectRuleV1(document, row.identity as any)!;
    if (rule.evidence === "assumed") continue;
    const derived = await deriveInputsV1(document, rule, row.identity as any, host), expected = frozen.derive[index];
    expect(derived.patterns, JSON.stringify(row.identity)).toEqual(expected.patterns.filter((pattern: string) => pattern !== "{workspaceRoot}/rust-toolchain.toml"));
    expect(derived.files).toEqual(expected.files);
    if (derived.markers?.length) {
      withMarkers++;
      expect(new Set(derived.markers.map((marker) => JSON.stringify(marker))).size).toBe(derived.markers.length);
    }
  }
  expect(withMarkers).toBeGreaterThan(0);
});

test("the rules and the fixture point at their sibling schemas through $schema and validate against the pointed schema", () => {
  const pointed = (base: string, data: {readonly $schema: string}) => JSON.parse(readFileSync(join(base, data.$schema), "utf8"));
  expect(pointed(join(here, ".."), rules)).toEqual(schema);
  expect(pointed(here, fixture)).toEqual(fixtureSchema);
  const validate = new Ajv({strict: false});
  expect(validate.validate(pointed(join(here, ".."), rules), rules), JSON.stringify(validate.errors)).toBe(true);
  expect(validate.validate(pointed(here, fixture), fixture), JSON.stringify(validate.errors)).toBe(true);
  expect(validate.validate(pointed(join(here, ".."), rules), {...rules, $schema: 1})).toBe(false);
});

test("answering the repository walks through the derived-walk ports derives exactly what walking in place derives", async () => {
  const asked: string[] = [], ports = (base: InputPrecisionHostV1): InputPrecisionHostV1 => ({...base, crateNames: async () => { asked.push("crates"); return crateNamesV1(base.cargoManifests(), base.readText); }, sourceRoots: async (question) => { asked.push(question.mode); return question.mode === "entries" ? entryFilesV1(question.globs, base) : includeRootsV1(question.include, question.exclude, base); }});
  for (const row of fixture.derive) {
    const rule = selectRuleV1(document, row.identity as any)!;
    if (rule.evidence === "assumed") continue;
    expect(await deriveInputsV1(document, rule, row.identity as any, ports(fixtureHost())), JSON.stringify(row.identity)).toEqual(await deriveInputsV1(document, rule, row.identity as any, fixtureHost()));
  }
  expect(asked).toContain("crates");
  const entries = ["{workspaceRoot}/app/src/**/*.ts", "!{workspaceRoot}/app/src/skip/**/*"];
  expect(await programInputsV1("app/tsconfig.json", ports(fixtureHost()), entries)).toEqual(await programInputsV1("app/tsconfig.json", fixtureHost(), entries));
  expect(await programInputsV1("app/tsconfig.json", ports(fixtureHost()))).toEqual(await programInputsV1("app/tsconfig.json", fixtureHost()));
  expect(asked).toContain("entries");
  expect(asked).toContain("include");
});
