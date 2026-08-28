import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { chmodSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, posix, resolve } from "node:path";
import Ajv from "ajv";
import { fromMarkdown } from "mdast-util-from-markdown";
import ts from "typescript";
import * as discovery from "../../🔍️discovery/🟦️component.ts";
import * as normalization from "../../🧹️normalization/🟦️.ts";

const vector = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8"));
const historical = JSON.parse(readFileSync(join(import.meta.dir, "🧬️energy-source-coordinates/🔣️.json"), "utf8"));
const libraryRoot = resolve(import.meta.dir, "../.."), root = resolve(libraryRoot, "../../../../..");
const sha = (bytes: Uint8Array | string): string => createHash("sha256").update(bytes).digest("hex");
const functions = () => {
  const validate = Reflect.get(discovery, "validateFrozenMarkdownCoordinateEvidenceContracts");
  const coordinates = Reflect.get(normalization, "frozenMarkdownCoordinateEvidenceCoordinates");
  expect(typeof validate).toBe("function");
  expect(typeof coordinates).toBe("function");
  return { validate, coordinates };
};
const contractFor = (row: any) => ({ path: "🧪️tests/🧪️history/📝️.md", grammar: vector.contract, sha256: sha(row.content), coordinates: [{ start: row.start ?? row.content.indexOf(row.value), end: row.end ?? row.content.indexOf(row.value) + row.value.length, kind: "source", form: row.form, valueSha256: sha(row.value) }] });

/** 📦️ Reuses the executed Draw collector without importing or running its test suite. */
function producerInputs(schema: discovery.Taxonomy) {
  const path = join(libraryRoot, "📦️packages/🟦️typescript/🧪️index.test.ts"), content = readFileSync(path, "utf8"), tree = ts.createSourceFile(path, content, ts.ScriptTarget.Latest, true);
  const names = new Set(["artifactProjectionProducerInput", "artifactProjectionProducerInputs", "projectionByteSort"]);
  const declarations = tree.statements.filter((node) => ts.isFunctionDeclaration(node) && names.has(node.name?.text ?? ""));
  expect(declarations).toHaveLength(3);
  const source = declarations.map((node) => node.getText(tree)).join("\n"), compiled = new Bun.Transpiler({ loader: "ts" }).transformSync(source);
  const scenario = JSON.parse(readFileSync(join(libraryRoot, "🧪️tests/🧪️draw-source-scenario/🔣️.json"), "utf8"));
  const build = new Function("getWorkspaceRoot", "lstatSync", "readFileSync", "createHash", "join", "posix", "registryCompilerInputDependencies", "DRAW_SOURCE_SCENARIO", compiled + "\nreturn { read: artifactProjectionProducerInput, collect: artifactProjectionProducerInputs };")(() => root, lstatSync, readFileSync, createHash, join, posix, discovery.registryCompilerInputDependencies, scenario);
  return { ...build.collect(schema), read: build.read, sourceSha256: sha(source) } as { files: Record<string, { content: string; mode: number; sha256: string; origin: string }>; modules: unknown[]; read: (path: string) => { sha256: string; mode: number }; sourceSha256: string };
}

/** 📜️ Uses CommonMark's independent block and inline AST without reusing production parsing. */
function oracle(row: any): boolean {
  const declaration = contractFor(row).coordinates[0], parsed = fromMarkdown(row.content);
  let found = false;
  const visit = (node: any, parents: any[]): void => {
    const allowed = parents.every((parent) => ["root", "paragraph", "list", "listItem"].includes(parent.type) && (parent.type !== "paragraph" || !parent.children.some((child: any) => child.type === "html")));
    if (allowed && row.form === "inline-code" && node.type === "inlineCode" && node.value === row.value && node.position?.start.offset === declaration.start - 1 && node.position?.end.offset === declaration.end + 1) found = true;
    if (allowed && row.form === "path-list-item" && node.type === "text" && node.value === row.value && parents.at(-1)?.type === "paragraph" && parents.at(-2)?.type === "listItem" && parents.at(-1).children.length === 1 && node.position?.start.offset === declaration.start && node.position?.end.offset === declaration.end) found = true;
    for (const child of node.children ?? []) visit(child, [...parents, node]);
  };
  visit(parsed, []);
  return found && Buffer.from(row.content).toString("utf8") === row.content && !/^(?:compose|temp\/compose)(?:\/|$)/u.test(row.value) && !/[\\:*?"<>|\u0000-\u0020]/u.test(row.value) && row.value.split("/").every((part: string) => part && part !== "." && part !== "..");
}

test("frozen Markdown neutral forms agree with an independent CommonMark AST", () => {
  const validate = new Ajv().compile({ type: "object", required: ["schemaVersion", "contract", "semantics", "cases"], properties: { schemaVersion: { const: 1 }, contract: { const: "frozen-markdown-source-coordinates-v1" }, cases: { type: "array", minItems: 28, items: { type: "object", required: ["id", "content", "form", "accepted", "value"], properties: { form: { enum: ["inline-code", "path-list-item"] }, accepted: { type: "boolean" } } } } } });
  expect(validate(vector), JSON.stringify(validate.errors)).toBe(true);
  for (const row of vector.cases) expect(oracle(row), row.id).toBe(row.accepted);
});

for (const row of vector.cases) test("frozen Markdown exact source span: " + row.id, () => {
  const actual = functions(), contract = contractFor(row), bytes = Buffer.from(row.content);
  expect(actual.validate({ history: contract })).toEqual([]);
  const run = () => actual.coordinates(contract.path, bytes, { history: contract });
  if (!row.accepted) expect(run).toThrow(/frozen-coordinate-evidence-invalid/u);
  else expect(run()).toEqual([{ pointer: "markdown:" + row.form + "@" + contract.coordinates[0].start, ...contract.coordinates[0], value: row.value }].map(({ form, valueSha256, ...entry }) => entry));
});

test("frozen Markdown authority rejects wrong document value and shifted overlapping or extra fields", () => {
  const actual = functions(), row = vector.cases[0], contract = contractFor(row), bytes = Buffer.from(row.content);
  expect(actual.coordinates("🧪️tests/🧪️unowned/📝️.md", bytes, { history: contract })).toBeNull();
  expect(() => actual.coordinates(contract.path, Buffer.concat([bytes, Buffer.from("\n")]), { history: contract })).toThrow(/digest/u);
  for (const alter of [
    (value: any) => { value.coordinates[0].start++; },
    (value: any) => { value.coordinates[0].end--; },
    (value: any) => { value.coordinates[0].valueSha256 = "0".repeat(64); },
    (value: any) => { value.coordinates.push({ ...value.coordinates[0] }); },
    (value: any) => { value.coordinates[0].kind = "destination"; },
    (value: any) => { value.coordinates[0].start = -1; },
    (value: any) => { value.coordinates[0].end = 1.5; },
    (value: any) => { value.extra = true; },
    (value: any) => { value.grammar = "markdown"; },
    (value: any) => { value.path = "compose/📝️.md"; },
    (value: any) => { value.coordinates[0].pointer = "/*"; },
  ]) {
    const changed = structuredClone(contract);
    alter(changed);
    expect(() => actual.coordinates(changed.path, bytes, { history: changed })).toThrow(/frozen-coordinate-evidence-invalid/u);
  }
  expect(actual.validate({ history: contract, duplicate: structuredClone(contract) }).length).toBeGreaterThan(0);
  const invalid = Buffer.concat([bytes, Buffer.from([0xff])]);
  expect(() => actual.coordinates(contract.path, invalid, { history: { ...contract, sha256: sha(invalid) } })).toThrow(/UTF-8/u);
});

test("the ten exact Markdown contracts are registered without changing existing JSON evidence authority", () => {
  const schema = discovery.loadCatalogTaxonomy();
  expect(schema.frozenMarkdownCoordinateEvidenceContracts).toEqual(Object.fromEntries(historical.documents.map((row: any) => [row.id, row.contract])));
  expect(Object.keys(schema.frozenCoordinateEvidenceContracts).filter((id) => id !== "energy-history-fixture-audit")).toHaveLength(38);
  expect(discovery.validateTaxonomy(schema)).toEqual([]);
});

test("all 21 reviewed historical spans retain exact physical bytes and independent CommonMark ownership", () => {
  const actual = functions();
  let count = 0;
  for (const row of historical.documents) {
    let path = root;
    const parts = row.contract.path.split("/");
    expect(/^(?:compose|temp\/compose)(?:\/|$)/u.test(row.contract.path)).toBe(false);
    for (const [index, part] of parts.entries()) {
      expect(part !== "" && part !== "." && part !== "..").toBe(true);
      path = join(path, part);
      const stat = lstatSync(path);
      expect(stat.isSymbolicLink()).toBe(false);
      expect(index === parts.length - 1 ? stat.isFile() : stat.isDirectory()).toBe(true);
    }
    const before = lstatSync(path), bytes = readFileSync(path), content = bytes.toString("utf8"), coordinates = actual.coordinates(row.contract.path, bytes, { [row.id]: row.contract });
    expect(sha(bytes)).toBe(row.contract.sha256);
    expect(bytes.length).toBe(row.size);
    expect(before.mode & 0o7777).toBe(row.mode);
    expect(coordinates).toHaveLength(row.contract.coordinates.length);
    for (const coordinate of coordinates) {
      expect(oracle({ content, ...coordinate, form: coordinate.pointer.split(":")[1].split("@")[0] })).toBe(true);
      expect(sha(coordinate.value)).toBe(row.contract.coordinates.find((declaration: any) => declaration.start === coordinate.start).valueSha256);
      count++;
    }
    expect(readFileSync(path)).toEqual(bytes);
    expect(lstatSync(path).mode).toBe(before.mode);
  }
  expect(count).toBe(21);
});

test("a scoped transaction preserves Markdown and escaped JSON history while rewriting its live neighboring reference", () => {
  const started = performance.now(), checkpoint = (phase: string) => console.log("[DEBUG] Historical coordinate fixture phase", JSON.stringify({ phase, milliseconds: performance.now() - started }));
  const parent = join(root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/📓️frozen-markdown-coordinates/🧾️runs");
  expect(lstatSync(parent).isDirectory() && !lstatSync(parent).isSymbolicLink()).toBe(true);
  const owner = mkdtempSync(join(parent, "🔖️transaction-")), fixture = join(owner, "🧪️fixture");
  mkdirSync(fixture);
  const put = (path: string, bytes: string) => { mkdirSync(dirname(join(fixture, path)), { recursive: true }); writeFileSync(join(fixture, path), bytes); };
  const git = (args: string[]) => { const run = Bun.spawnSync(["git", ...args], { cwd: fixture, stdout: "pipe", stderr: "pipe" }); expect(run.exitCode, run.stderr.toString()).toBe(0); return run.stdout.toString().trim(); };
  const scope = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/MARKDOWN-SOURCE/🧪️tests/🧪️fixture", source = scope + "/🦀️component.rs", final = scope + "/🦀️.rs", historyPath = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/HISTORICAL-SOURCE/📝️.md", livePath = "🧪️tests/🧪️consumer/🔣️.json";
  const history = "Recorded `" + source + "` before normalization.\n", live = JSON.stringify({ sourcePath: source }) + "\n";
  const encodedHistoryPath = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/HISTORICAL-SOURCE/🔣️.json", escaped = JSON.stringify(source).replaceAll("/", "\\/"), encodedHistory = '[{"path":' + escaped + '}]\n';
  const schemaPath = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";
  const declaration = { start: history.indexOf(source), end: history.indexOf(source) + source.length, kind: "source" as const, form: "inline-code" as const, valueSha256: sha(source) };
  const schema = { ...structuredClone(discovery.loadCatalogTaxonomy()), frozenCoordinateEvidenceContracts: { history: { path: encodedHistoryPath, sha256: sha(encodedHistory), schemaVersion: null, rootKind: "array" as const, coordinates: [{ pointer: "/0/path", kind: "source" as const, representation: "json-escaped-source-path" as const }] } }, frozenMarkdownCoordinateEvidenceContracts: { history: { path: historyPath, grammar: "frozen-markdown-source-coordinates-v1" as const, sha256: sha(history), coordinates: [declaration] } } };
  const producer = producerInputs(schema);
  checkpoint("producer-inputs");
  for (const [path, input] of Object.entries(producer.files)) { put(path, input.content); chmodSync(join(fixture, path), input.mode); }
  expect(discovery.registryCatalogPathMayAffect(source, schema)).toBe(false);
  const schemaBytes = JSON.stringify(schema, null, 2) + "\n";
  put(schemaPath, schemaBytes); put(source, "pub fn value() -> u32 { 7 }\n"); put(historyPath, history); put(encodedHistoryPath, encodedHistory); put(livePath, live);
  git(["init", "-q"]); put(".git/info/exclude", schemaPath + "\n");
  git(["add", "--all"]);
  git(["-c", "user.name=Markdown Source Fixture", "-c", "user.email=fixture@invalid.example", "-c", "commit.gpgsign=false", "commit", "-qm", "Markdown source fixture"]);
  const baselineCommit = git(["rev-parse", "HEAD"]), ticketDir = join(fixture, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/MARKDOWN-TRANSACTION");
  mkdirSync(ticketDir, { recursive: true });
  const plan = () => normalization.planTaxonomy(normalization.inventoryTaxonomy({ repoRoot: fixture, scope, workers: 1 }), { baselineCommit, excludedTreeDigests: [] });
  const current = plan();
  checkpoint("initial-plan");
  writeFileSync(join(owner, "📝️.md"), "# Historical Markdown Transaction\n\nThis isolated fixture retains all source and recovery evidence; no cleanup is performed.\n", { flag: "wx" });
  writeFileSync(join(owner, "🔣️.json"), normalization.canonicalJson(current) + "\n", { flag: "wx" });
  expect(current.unresolved).toEqual([]);
  expect(current.moves).toHaveLength(1);
  expect(current.regenerations).toHaveLength(0);
  expect(current.edits.filter((edit) => edit.path === historyPath)).toEqual([]);
  expect(current.edits.filter((edit) => edit.path === encodedHistoryPath)).toEqual([]);
  expect(current.edits.filter((edit) => edit.path === livePath)).toHaveLength(1);
  const extraHistory = history + "\nRecorded `" + source + "` in another paragraph.\n";
  const extraEncoded = '[{"path":' + escaped + '},{"path":' + escaped + '}]\n';
  put(historyPath, extraHistory);
  put(encodedHistoryPath, extraEncoded);
  put(schemaPath, JSON.stringify({ ...schema, frozenMarkdownCoordinateEvidenceContracts: { history: { ...schema.frozenMarkdownCoordinateEvidenceContracts.history, sha256: sha(extraHistory) } }, frozenCoordinateEvidenceContracts: { history: { ...schema.frozenCoordinateEvidenceContracts.history, sha256: sha(extraEncoded) } } }));
  const unowned = plan();
  checkpoint("unowned-plan");
  expect(unowned.unresolved.some((row) => row.code === "frozen-coordinate-evidence-unowned" && row.path === historyPath)).toBe(true);
  expect(unowned.edits.filter((edit) => edit.path === historyPath)).toEqual([]);
  expect(unowned.unresolved.some((row) => row.code === "frozen-coordinate-evidence-unowned" && row.path === encodedHistoryPath)).toBe(true);
  expect(unowned.edits.filter((edit) => edit.path === encodedHistoryPath)).toEqual([]);
  put(historyPath, history); put(encodedHistoryPath, encodedHistory); put(schemaPath, schemaBytes);
  const options = { repoRoot: fixture, ticketDir, expectedBaselineCommit: baselineCommit, expectedPlanDigest: current.planDigest };
  expect(normalization.applyTaxonomyPlan(current, { ...options, injectFailureAt: "after-edits" }).state).toBe("rolled-back");
  checkpoint("rollback");
  expect(readFileSync(join(fixture, source), "utf8")).toBe("pub fn value() -> u32 { 7 }\n");
  expect(readFileSync(join(fixture, livePath), "utf8")).toBe(live);
  expect(readFileSync(join(fixture, historyPath), "utf8")).toBe(history);
  expect(readFileSync(join(fixture, encodedHistoryPath), "utf8")).toBe(encodedHistory);
  expect(normalization.applyTaxonomyPlan(current, options).state).toBe("committed");
  checkpoint("retry-commit");
  expect(existsSync(join(fixture, source))).toBe(false);
  expect(readFileSync(join(fixture, final), "utf8")).toBe("pub fn value() -> u32 { 7 }\n");
  expect(JSON.parse(readFileSync(join(fixture, livePath), "utf8")).sourcePath).toBe(final);
  expect(readFileSync(join(fixture, historyPath), "utf8")).toBe(history);
  expect(readFileSync(join(fixture, encodedHistoryPath), "utf8")).toBe(encodedHistory);
  const after = plan();
  checkpoint("empty-replan");
  expect({ moves: after.moves.length, edits: after.edits.length, unresolved: after.unresolved }).toEqual({ moves: 0, edits: 0, unresolved: [] });
  for (const [path, input] of Object.entries(producer.files)) if (input.origin === "current-compiler-context") expect(producer.read(path)).toMatchObject({ sha256: input.sha256, mode: input.mode });
  mkdirSync(join(owner, "📦️producer"));
  writeFileSync(join(owner, "📦️producer/🔣️.json"), JSON.stringify({ collectorSha256: producer.sourceSha256, modules: producer.modules, inputs: Object.entries(producer.files).map(([path, input]) => ({ path, sha256: input.sha256, mode: input.mode, origin: input.origin })) }, null, 2) + "\n", { flag: "wx" });
  console.log("[DEBUG] Markdown and escaped JSON source transaction", JSON.stringify({ owner, moves: current.moves.length, historyEdits: 0, liveEdits: 1, rollback: "verified", retry: "committed", secondPlan: "empty" }));
}, 15_000);
