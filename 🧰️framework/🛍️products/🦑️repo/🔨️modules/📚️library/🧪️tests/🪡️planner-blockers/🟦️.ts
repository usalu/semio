/** 🪡️ Planner blocker laws over `../🧫️fixtures/🪡️planner-blockers/🔣️.json` (schema `../../🧬️schema/🪡️planner-blockers/🔣️.json`):
 * embedded argv candidates never span a JSON escape nor partially overlap each other, a file's conflicting edits are named instead of
 * thrown, and an invalid frozen evidence document becomes a reportable problem while the preflight form still aborts. Oracles: Ajv for
 * the fixture, string-argv (third-party shell word splitting) for candidate containment, sequential splicing for edit application. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";
import { parseArgsStringToArgv } from "string-argv";
import ts from "typescript";
import { loadCatalogTaxonomy } from "../../🔍️discovery/🟦️.ts";
import { frozenCoordinateEvidenceCoordinates, frozenMarkdownCoordinateEvidenceCoordinates } from "../../🧹️normalization/🟦️.ts";
import { utf8Compare } from "../../🧹️normalization/🛣️path/🟦️.ts";

type Fixture = { schemaVersion: number; argumentCases: { id: string; command: string; values: string[] }[]; frozenEvidenceContractId: string };
type Token = { start: number; end: number; value: string; structuredLocation: string };
type Edit = { path: string; structuredLocation: string; oldValue: string; newValue: string };
const library = resolve(import.meta.dir, "../..");
const fixture = JSON.parse(readFileSync(resolve(library, "🧫️fixtures/🪡️planner-blockers/🔣️.json"), "utf8")) as Fixture;
const syntax = ts.createSourceFile("normalization.ts", readFileSync(resolve(library, "🧹️normalization/🟦️.ts"), "utf8"), ts.ScriptTarget.Latest, true);

/** 🧬️ Compiles the named production declarations of the normalizer into one closure over the injected dependencies. */
function production<T>(names: readonly string[], entry: string, dependencies: Record<string, unknown> = {}): T {
  const wanted = new Set(names);
  const code = syntax.statements.filter((node) => ts.isFunctionDeclaration(node) ? wanted.has(node.name?.text ?? "") : ts.isVariableStatement(node) && node.declarationList.declarations.some((declaration) => wanted.has(declaration.name.getText(syntax)))).map((node) => node.getText(syntax).replace(/^export /u, "")).join("\n");
  return new Function(...Object.keys(dependencies), `${ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText}\nreturn ${entry};`)(...Object.values(dependencies)) as T;
}

/** 🎲️ A small deterministic generator so property laws are reproducible. */
function random(seed: number): () => number {
  let state = seed;
  return () => { state = (state * 1103515245 + 12345) & 0x7fffffff; return state / 0x80000000; };
}

const jsonTokens = production<(path: string, content: string, adapter: string) => Token[]>(["jsonTokens", "embeddedArgumentTokens", "lineLocation", "indexedLineContent", "indexedLineStarts", "EMBEDDED_ARGUMENT_WORD", "EMBEDDED_ESCAPED_ARGUMENT_WORD", "EMBEDDED_ARGUMENT_PATH"], "jsonTokens");
const argv = (command: string): Token[] => jsonTokens(".vscode/launch.json", JSON.stringify({ command }), "json").filter((token) => token.structuredLocation.startsWith("embedded-argv"));

test("fixture satisfies its schema", () => {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(resolve(library, "🧬️schema/🪡️planner-blockers/🔣️.json"), "utf8")));
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
});

test("embedded argv candidates are exact for every fixture command", () => {
  for (const row of fixture.argumentCases) expect(argv(row.command).map((token) => token.value), row.id).toEqual(row.values);
});

test("embedded argv candidates never span an escape, never partially overlap, and lie inside one shell word", () => {
  const next = random(23), pieces = [" ", "\"", "\\", "${workspaceFolder}/", "a/b", "./c", "../d", "/e", "{", "}", ":", ",", "=", "--x=", "file:", "q.toml", "😀/f", "\t"];
  let checked = 0;
  for (let round = 0; round < 5_000; round++) {
    const command = `bun ${Array.from({ length: 1 + Math.floor(next() * 14) }, () => pieces[Math.floor(next() * pieces.length)]!).join("")}`;
    const content = JSON.stringify({ command }), tokens = argv(command);
    let words: string[];
    try { words = parseArgsStringToArgv(command); } catch { words = [command]; }
    for (const token of tokens) {
      checked++;
      expect(token.value.includes("\\"), JSON.stringify([command, token.value])).toBe(false);
      expect(content.slice(token.start, token.end), JSON.stringify(command)).toBe(token.value);
      expect(words.some((word) => word.includes(token.value)), JSON.stringify([command, token.value, words])).toBe(true);
      for (const other of tokens) {
        const partial = other.start < token.start && token.start < other.end && other.end < token.end;
        expect(partial, JSON.stringify([command, other.value, token.value])).toBe(false);
      }
    }
  }
  expect(checked).toBeGreaterThan(1_000);
});

test("edit conflicts are named exactly when one-pass application could not reproduce sequential splicing", () => {
  const { apply, conflict } = production<{ apply: (content: string, edits: readonly Edit[]) => string; conflict: (content: string, edits: readonly Edit[]) => string | null }>(["applyEditsToContent", "referenceEditOffset", "referenceEditConflict"], "{ apply: applyEditsToContent, conflict: referenceEditConflict }", { utf8Compare });
  const offset = (edit: Edit): number => Number(edit.structuredLocation.match(/@(\d+)$/)![1]);
  const sequential = (content: string, edits: readonly Edit[]): string => {
    let result = content;
    for (const edit of [...edits].sort((a, b) => offset(b) - offset(a) || b.structuredLocation.localeCompare(a.structuredLocation))) {
      const start = offset(edit);
      if (result.slice(start, start + edit.oldValue.length) !== edit.oldValue) throw new Error("mismatch");
      result = result.slice(0, start) + edit.newValue + result.slice(start + edit.oldValue.length);
    }
    return result;
  };
  const next = random(29);
  let conflicts = 0;
  for (let round = 0; round < 10_000; round++) {
    const content = Array.from({ length: 4 + Math.floor(next() * 30) }, () => "ab😀c/"[Math.floor(next() * 6)]).join("");
    const edits = Array.from({ length: Math.floor(next() * 5) }, (_, index): Edit => {
      const start = Math.floor(next() * content.length), end = Math.min(content.length, start + Math.floor(next() * 4));
      return { path: "p", structuredLocation: `l${index}@${start}`, oldValue: next() < 0.05 ? "zz" : content.slice(start, end), newValue: `N${index}` };
    });
    const named = conflict(content, edits);
    let oracle: string | null;
    try { oracle = sequential(content, edits); } catch { oracle = null; }
    if (named === null) {
      expect(oracle, JSON.stringify([content, edits])).not.toBeNull();
      expect(apply(content, edits)).toBe(oracle!);
    } else conflicts++;
    if (oracle === null) expect(named, JSON.stringify([content, edits])).not.toBeNull();
  }
  expect(conflicts).toBeGreaterThan(0);
});

test("an invalid frozen evidence document is a reportable problem with no coordinates while the preflight form aborts", () => {
  const discoverySchema = loadCatalogTaxonomy(), taxonomy = { discoverySchema };
  const contract = discoverySchema.frozenCoordinateEvidenceContracts[fixture.frozenEvidenceContractId]!;
  const { record, authority } = production<{ record: (path: string, bytes: Uint8Array, taxonomy: unknown) => { coordinates: ReadonlySet<string>; problem: string | null } | null; authority: (path: string, bytes: Uint8Array, taxonomy: unknown) => ReadonlySet<string> | null }>(["frozenEvidenceRecord", "frozenEvidenceCoordinateAuthority", "frozenEvidenceContractIndex", "frozenEvidenceContractIndexes", "frozenEvidenceCoordinateCache"], "{ record: frozenEvidenceRecord, authority: frozenEvidenceCoordinateAuthority }", { frozenCoordinateEvidenceCoordinates, frozenMarkdownCoordinateEvidenceCoordinates });
  const registered = readFileSync(resolve(library, "../../../../..", contract.path)), tampered = Buffer.concat([registered, Buffer.from(" ")]);
  const valid = record(contract.path, registered, taxonomy)!;
  expect(valid.problem).toBeNull();
  expect(valid.coordinates.size).toBeGreaterThan(0);
  expect(authority(contract.path, registered, taxonomy)).toBe(valid.coordinates);
  const invalid = record(contract.path, tampered, taxonomy)!;
  expect(invalid.problem).toBe(`frozen-coordinate-evidence-invalid: ${contract.path}: document digest does not match registered bytes`);
  expect(invalid.coordinates.size).toBe(0);
  expect(() => authority(contract.path, tampered, taxonomy)).toThrow(invalid.problem!);
  expect(record("not/registered.json", registered, taxonomy)).toBeNull();
});
