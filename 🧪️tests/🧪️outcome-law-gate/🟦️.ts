/** ⚖️ Outcome-law gate law: rule 2 of `verify mutation-outcome-law` reports exactly the planted violations of `🧫️fixtures/🧫️outcome-law-gate` (position, fault, code, level as spelled) and stays silent on its canonical controls. The frozen vocabulary document is the level oracle; Ajv (outcome documents) and the TypeScript compiler (twin positions) re-derive the expected breaches independently of the gate's patterns. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import Ajv from "ajv";
import ts from "typescript";
import { policyOutcomeCodeFileBreaches, policyOutcomeVocabulary } from "../../📜️script.ts";

type Level = "info" | "warning" | "error" | "fatal";
type Breach =
  | { line: number; fault: "outside-vocabulary" | "not-apply-code"; code: string }
  | { line: number; fault: "level-mismatch"; code: string; level: string; expected: Level }
  | { line: number; fault: "level-alias" }
  | { line: number; fault: "retired-builder"; builder: "warn" };
type Case = { id: string; description: string; segments: string[]; lines: string[]; breaches: Breach[] };
type Vectors = { contract: string; vocabulary: string; cases: Case[] };
type Vocabulary = { codes: { code: string; level: Level }[]; apply: { pattern: string; level: Level } };

const root = resolve(import.meta.dir, "../..");
const read = (path: string): unknown => JSON.parse(readFileSync(join(root, path), "utf8"));
const vectors = read("🧫️fixtures/🧫️outcome-law-gate/🔣️.json") as Vectors;
const document = read(vectors.vocabulary) as Vocabulary;
const fixed = new Map(document.codes.map((row) => [row.code, row.level]));
const applyPattern = new RegExp(document.apply.pattern);
const SUMMARY = /^"(?<path>.+):(?<line>\d+)" (?:outcome code "(?<code>[^"]*)"(?: at (?<level>\S+))? (?<fault>is not in the frozen outcome vocabulary|is fixed at (?<expected>\w+), not \S+|is not a mutation\.apply\.<detail> apply-rejection code)|(?<alias>outcome level "warn" is a retired spelling of "warning")|outcome builder "(?<builder>warn)" is a retired spelling of "warning")$/u;

const pathOf = (row: Case): string => row.segments.join("/");
const contentOf = (row: Case): string => `${row.lines.join("\n")}\n`;

/** 🔑️ One comparable key per breach; the level an out-of-vocabulary or apply code is spelled at does not change its fault. */
const key = (breach: Breach): string =>
  breach.fault === "level-mismatch" ? `${breach.line}|level-mismatch|${breach.code}|${breach.level}|${breach.expected}` : breach.fault === "level-alias" ? `${breach.line}|level-alias` : breach.fault === "retired-builder" ? `${breach.line}|retired-builder|${breach.builder}` : `${breach.line}|${breach.fault}|${breach.code}`;

/** 🧭️ The gate's own report for one planted file, parsed back into fixture breaches. */
function reported(row: Case): Breach[] {
  return policyOutcomeCodeFileBreaches(policyOutcomeVocabulary(root), pathOf(row), contentOf(row)).map((record) => {
    const match = SUMMARY.exec(record.summary)?.groups;
    if (!match || match.path !== pathOf(row)) throw new Error(`unparseable gate summary: ${record.summary}`);
    const line = Number(match.line);
    if (match.alias) return { line, fault: "level-alias" };
    if (match.builder) return { line, fault: "retired-builder", builder: "warn" };
    if (match.expected) return { line, fault: "level-mismatch", code: match.code!, level: match.level!, expected: match.expected as Level };
    return { line, fault: match.fault!.startsWith("is not a mutation.apply") ? "not-apply-code" : "outside-vocabulary", code: match.code! };
  });
}

/** 🧮️ Classifies one (code, level) position against the vocabulary document alone. */
function classify(line: number, code: string, level: string | null): Breach | null {
  const expected = fixed.get(code) ?? (applyPattern.test(code) ? document.apply.level : undefined);
  if (expected === undefined) return { line, fault: "outside-vocabulary", code };
  return level !== null && level !== expected ? { line, fault: "level-mismatch", code, level, expected } : null;
}

/** 🌳️ TypeScript-compiler oracle: `refuse(level, code)` calls, `{ level, code }` messages and, in mutation leaves, every `mutation.` string literal. */
function typescriptOracle(row: Case): Breach[] {
  const source = ts.createSourceFile(pathOf(row), contentOf(row), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const leaf = row.segments.includes("🧬️mutations") || row.segments.includes("🔺️diff");
  const found = new Map<string, Breach>();
  const add = (node: ts.Node, code: string, level: string | null) => {
    const line = source.getLineAndCharacterOfPosition(node.getStart(source)).line + 1;
    const breach = classify(line, code, level);
    if (breach && !found.has(`${line}|${code}`)) found.set(`${line}|${code}`, breach);
  };
  const text = (node: ts.Node | undefined): string | null => (node && ts.isStringLiteralLike(node) ? node.text : null);
  const visit = (node: ts.Node): void => {
    if (ts.isCallExpression(node) && ts.isIdentifier(node.expression) && node.expression.text === "refuse") {
      const [level, code] = [text(node.arguments[0]), text(node.arguments[1])];
      if (level !== null && code !== null) add(node, code, level);
    }
    if (ts.isObjectLiteralExpression(node)) {
      const property = (name: string) => text(node.properties.find((entry): entry is ts.PropertyAssignment => ts.isPropertyAssignment(entry) && ts.isIdentifier(entry.name) && entry.name.text === name)?.initializer);
      const [level, code] = [property("level"), property("code")];
      if (level !== null && code?.startsWith("mutation.")) add(node, code, level);
    }
    if (leaf && ts.isStringLiteralLike(node) && node.text.startsWith("mutation.")) add(node, node.text, null);
    ts.forEachChild(node, visit);
  };
  visit(source);
  return [...found.values()];
}

/** 🧾️ Ajv oracle for committed outcome documents: a message is admitted iff one vocabulary branch (code const + level const, or the apply pattern at its level) validates it. */
function outcomeDocumentOracle(row: Case): string[] {
  const ajv = new Ajv({ strict: true, allErrors: true });
  const known = ajv.compile({ type: "object", required: ["code"], properties: { code: { type: "string", anyOf: [{ type: "string", enum: [...fixed.keys()] }, { type: "string", pattern: document.apply.pattern }] } } });
  const admitted = ajv.compile({
    type: "object",
    required: ["code"],
    anyOf: [
      ...document.codes.map((entry) => ({ type: "object", properties: { code: { const: entry.code }, level: { const: entry.level } } })),
      { type: "object", properties: { code: { type: "string", pattern: document.apply.pattern }, level: { const: document.apply.level } } },
    ],
  });
  const parsed = JSON.parse(contentOf(row)) as { code?: unknown; messages?: unknown[] };
  return [parsed, ...(parsed.messages ?? [])]
    .filter((message): message is { code: string; level?: string } => typeof (message as { code?: unknown }).code === "string")
    .flatMap((message) => (admitted(message) ? [] : [`${known(message) ? "level-mismatch" : "outside-vocabulary"}|${message.code}`]))
    .sort();
}

test("the planted violations have unique authored source identities", () => {
  expect(new Set(vectors.cases.map((row) => row.id)).size).toBe(vectors.cases.length);
  expect(new Set(vectors.cases.map(pathOf)).size).toBe(vectors.cases.length);
});

test("the gate reads exactly the frozen vocabulary document the vectors name", () => {
  const vocabulary = policyOutcomeVocabulary(root);
  expect([...vocabulary.codes.entries()].sort()).toEqual([...fixed.entries()].sort());
  expect([vocabulary.applyPattern.source, vocabulary.applyLevel]).toEqual([applyPattern.source, document.apply.level]);
});

test("every expected level and code class agrees with the vocabulary document", () => {
  for (const row of vectors.cases)
    for (const breach of row.breaches) {
      if (breach.fault === "level-mismatch") expect([row.id, fixed.get(breach.code) ?? (applyPattern.test(breach.code) ? document.apply.level : null)]).toEqual([row.id, breach.expected]);
      if (breach.fault === "outside-vocabulary") expect([row.id, fixed.has(breach.code) || applyPattern.test(breach.code)]).toEqual([row.id, false]);
      if (breach.fault === "not-apply-code") expect([row.id, applyPattern.test(breach.code)]).toEqual([row.id, false]);
    }
  expect(vectors.cases.some((row) => row.breaches.length === 0)).toBe(true);
  expect(new Set(vectors.cases.flatMap((row) => row.breaches.map((breach) => breach.fault)))).toEqual(new Set(["outside-vocabulary", "level-mismatch", "not-apply-code", "level-alias", "retired-builder"]));
});

for (const row of vectors.cases)
  test(`the gate reports exactly the planted breaches: ${row.id}`, () => {
    expect(reported(row).map(key).sort()).toEqual(row.breaches.map(key).sort());
  });

for (const row of vectors.cases.filter((candidate) => candidate.segments.at(-1) === "🟦️.ts"))
  test(`the TypeScript compiler oracle agrees with the gate: ${row.id}`, () => {
    expect(typescriptOracle(row).map(key).sort()).toEqual(reported(row).map(key).sort());
  });

for (const row of vectors.cases.filter((candidate) => candidate.segments.at(-2) === "🎯️outcome"))
  test(`the Ajv outcome-document oracle agrees with the gate: ${row.id}`, () => {
    expect(outcomeDocumentOracle(row)).toEqual(reported(row).map((breach) => ("code" in breach ? `${breach.fault}|${breach.code}` : breach.fault)).sort());
  });
