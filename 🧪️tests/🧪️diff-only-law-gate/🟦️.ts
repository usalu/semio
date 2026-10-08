/** 📐️ Diff-only mutation law gate: rules R8–R16 of `verify mutation-outcome-law` report exactly the planted violations of `🧫️fixtures/🧫️diff-only-law-gate` (file, line, rule, design code) and stay silent on its compliant controls — per file, through the real git-inventory rule entry points, and for R15 over planted leaf trees. The TypeScript scanner re-derives the token-level rules (R8, R9, R10, R16) independently of the gate's patterns. */
import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import ts from "typescript";
import {
  policyBaseCloneDiffBreaches,
  policyDiffDerivedInverseBreaches,
  policyDiffOnlyFileBreaches,
  policyInverseSumLawUntestedBreaches,
  policyInverseSumLawUntestedLeafBreaches,
  policyLeafBetweenBreaches,
  policyLeafCentralApplyBreaches,
  policyLeafMutableBaseBreaches,
  policyOutcomeApplyToBreaches,
  policyRestoreInverseBreaches,
  policyWholeStateDiffBreaches,
} from "../../📜️script.ts";

type Rule = "R8" | "R9" | "R10" | "R11" | "R12" | "R13" | "R14" | "R15" | "R16";
type Case = { id: string; description: string; segments: string[]; lines: string[]; breaches: { line: number; rule: Rule }[]; oracle: Rule[] };
type Tree = { id: string; description: string; files: Record<string, string[]>; breaches: string[] };
type Vectors = { contract: string; codes: Record<Rule, string>; cases: Case[]; leafTrees: Tree[] };

const root = resolve(import.meta.dir, "../..");
const vectors = JSON.parse(readFileSync(join(root, "🧫️fixtures/🧫️diff-only-law-gate/🔣️.json"), "utf8")) as Vectors;
const SUMMARY = /^(?<path>.+):(?<line>\d+) (?<rule>R\d+):(?<code>[A-Z0-9-]+) .+ \(`.*`\)$/u;
const FILE_RULES = [
  [policyLeafMutableBaseBreaches, "R8"],
  [policyLeafCentralApplyBreaches, "R9"],
  [policyLeafBetweenBreaches, "R10"],
  [policyDiffDerivedInverseBreaches, "R11"],
  [policyBaseCloneDiffBreaches, "R12"],
  [policyWholeStateDiffBreaches, "R13"],
  [policyRestoreInverseBreaches, "R14"],
  [policyOutcomeApplyToBreaches, "R16"],
] as const;

const pathOf = (row: Case): string => row.segments.join("/");
const contentOf = (row: Case): string => `${row.lines.join("\n")}\n`;
const keyOf = (line: number, rule: string): string => `${line}|${rule}`;

/** 🧭️ The gate's own report for one planted file, parsed back into `line|rule` keys with the fixture's design code checked on the way. */
function reported(row: Case): string[] {
  return policyDiffOnlyFileBreaches(pathOf(row), contentOf(row))
    .map((record) => {
      const match = SUMMARY.exec(record.summary)?.groups;
      if (!match || match.path !== pathOf(row)) throw new Error(`unparseable gate summary: ${record.summary}`);
      expect(match.code).toBe(vectors.codes[match.rule as Rule]);
      return keyOf(Number(match.line), match.rule!);
    })
    .sort();
}

type Token = { text: string; line: number };

/** 🌳️ TypeScript-scanner oracle: the lexical tokens of a planted file (comments and string contents are trivia, never tokens) with their lines. */
function tokensOf(source: string): Token[] {
  const scanner = ts.createScanner(ts.ScriptTarget.Latest, true, ts.LanguageVariant.Standard, source);
  const found: Token[] = [];
  for (let kind = scanner.scan(); kind !== ts.SyntaxKind.EndOfFileToken; kind = scanner.scan())
    found.push({ text: kind === ts.SyntaxKind.StringLiteral ? '""' : scanner.getTokenText(), line: source.slice(0, scanner.getTokenStart()).split("\n").length });
  return found;
}

/** 🧮️ The `line|rule` keys the token-level rules owe, derived from tokens alone. */
function oracleKeys(row: Case, rule: Rule): string[] {
  const tokens = tokensOf(contentOf(row));
  const keys: string[] = [];
  let inUse = false;
  tokens.forEach((token, index) => {
    const previous = tokens[index - 1]?.text;
    const next = tokens[index + 1]?.text;
    if (token.text === "use" && (previous === undefined || previous === ";" || previous === "}")) inUse = true;
    if (token.text === ";") inUse = false;
    const path = previous === "." || previous === ":";
    if (rule === "R8" && token.text === "&" && next === "mut") keys.push(keyOf(token.line, rule));
    if (rule === "R9" && !inUse && ((token.text === "apply" && path && next === "(") || (token.text === "apply_diff" && previous !== "fn" && next === "(") || token.text === "ApplyCapability")) keys.push(keyOf(token.line, rule));
    if (rule === "R10" && token.text === "between" && previous !== "fn" && next === "(") keys.push(keyOf(token.line, rule));
    if (rule === "R16" && token.text === "apply_to" && path && next === "(") keys.push(keyOf(token.line, rule));
  });
  return [...new Set(keys)].sort();
}

/** 🗂️ A scratch git repository holding the planted files, so the real inventory-driven rule entry points run over it. */
function scratchRepository(files: Record<string, string>): string {
  const scratch = mkdtempSync(join(tmpdir(), "diff-only-law-gate-"));
  for (const [path, content] of Object.entries(files)) {
    mkdirSync(dirname(join(scratch, path)), { recursive: true });
    writeFileSync(join(scratch, path), content);
  }
  const initialized = Bun.spawnSync(["git", "init", "-q"], { cwd: scratch });
  if (initialized.exitCode !== 0) throw new Error(`git init failed: ${initialized.stderr.toString()}`);
  return scratch;
}

test("the planted violations have unique authored source identities", () => {
  expect(new Set(vectors.cases.map((row) => row.id)).size).toBe(vectors.cases.length);
  expect(new Set(vectors.cases.map(pathOf)).size).toBe(vectors.cases.length);
  expect(new Set(vectors.leafTrees.map((tree) => tree.id)).size).toBe(vectors.leafTrees.length);
});

test("every rule R8–R16 has a negative sample and the compliant controls expect no breach", () => {
  const planted = new Set<string>([...vectors.cases.flatMap((row) => row.breaches.map((breach) => breach.rule)), ...vectors.leafTrees.filter((tree) => tree.breaches.length > 0).map(() => "R15")]);
  expect(planted).toEqual(new Set(Object.keys(vectors.codes)));
  const controls = vectors.cases.filter((row) => row.id.startsWith("compliant-"));
  expect(controls.length).toBeGreaterThanOrEqual(3);
  for (const row of controls) expect(row.breaches).toEqual([]);
});

for (const row of vectors.cases)
  test(`the gate reports exactly the planted breaches: ${row.id}`, () => {
    expect(reported(row)).toEqual(row.breaches.map((breach) => keyOf(breach.line, breach.rule)).sort());
  });

for (const row of vectors.cases.filter((candidate) => candidate.oracle.length > 0))
  for (const rule of row.oracle)
    test(`the TypeScript scanner oracle agrees with the gate on ${rule}: ${row.id}`, () => {
      expect(oracleKeys(row, rule)).toEqual(reported(row).filter((key) => key.endsWith(`|${rule}`)));
    });

test("the inventory-driven rule entry points report the planted breaches of a scratch repository", () => {
  const scratch = scratchRepository(Object.fromEntries(vectors.cases.map((row) => [pathOf(row), contentOf(row)])));
  try {
    for (const [rule, id] of FILE_RULES) {
      const expected = vectors.cases.flatMap((row) => row.breaches.filter((breach) => breach.rule === id).map((breach) => `${pathOf(row)}:${breach.line}`)).sort();
      expect(rule(scratch).map((record) => `${record.scope}:${record.line}`).sort()).toEqual(expected);
    }
  } finally {
    rmSync(scratch, { recursive: true, force: true });
  }
});

for (const tree of vectors.leafTrees) {
  test(`R15 reports exactly the leaves without a sum-law test: ${tree.id}`, () => {
    const files = Object.entries(tree.files).map(([path, lines]) => [path, `${lines.join("\n")}\n`] as const);
    const content = new Map(files);
    const records = policyInverseSumLawUntestedLeafBreaches(
      files.map(([path]) => path),
      (path) => content.get(path) ?? "",
    );
    expect(records.map((record) => record.scope).sort()).toEqual([...tree.breaches].sort());
    for (const record of records) expect(SUMMARY.exec(record.summary)?.groups?.code).toBe(vectors.codes.R15);
  });

  test(`R15 runs through the inventory entry point: ${tree.id}`, () => {
    const scratch = scratchRepository(Object.fromEntries(Object.entries(tree.files).map(([path, lines]) => [path, `${lines.join("\n")}\n`])));
    try {
      expect(
        policyInverseSumLawUntestedBreaches(scratch)
          .map((record) => record.scope)
          .sort(),
      ).toEqual([...tree.breaches].sort());
    } finally {
      rmSync(scratch, { recursive: true, force: true });
    }
  });
}
