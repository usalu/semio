import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import Ajv from "ajv";
import emojiRegex from "emoji-regex";
import { parse, type ParseError } from "jsonc-parser";
import toArray from "lodash/toArray";
import ts from "typescript";

type Split = Readonly<{ emoji: string; rest: string }>;
type Case = Readonly<{ id: string; input: string; first: string; emoji: string; rest: string; segmentationOracle: boolean; emojiOracle: boolean }>;
type ScalingCase = Readonly<{ id: string; prefix: string; suffixUnit: string; repetitions: number; emoji: string; rest: "suffix" | "whole-input" }>;
type Vector = Readonly<{ schemaVersion: number; contractId: string; helper: string; segmenter: { locale: string; granularity: "grapheme" }; semantics: { iteratorAdvancesPerRequest: number }; rounds: number; cases: Case[]; scalingCases: ScalingCase[]; oracleDivergences: string[] }>;
type Observation = { value: string; nextCalls: number; iteratorCalls: number };
const library = resolve(import.meta.dir, "../.."), sourcePath = join(library, "🧹️normalization/🟦️.ts");
const sourceBytes = readFileSync(sourcePath), source = ts.createSourceFile(sourcePath, sourceBytes.toString("utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
const vectorBytes = readFileSync(join(import.meta.dir, "🔣️.json"), "utf8"), vector: Vector = JSON.parse(vectorBytes);
const compilers = [{ id: "bun", compile: (code: string): string => new Bun.Transpiler({ loader: "ts" }).transformSync(code) }, { id: "typescript", compile: (code: string): string => ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext } }).outputText }];
const realSegmenter = (): Intl.Segmenter => new Intl.Segmenter(vector.segmenter.locale, { granularity: vector.segmenter.granularity });

function declaration(name: string): ts.FunctionDeclaration {
  const declarations = source.statements.filter((node): node is ts.FunctionDeclaration => ts.isFunctionDeclaration(node) && node.name?.text === name);
  expect(declarations).toHaveLength(1);
  return declarations[0]!;
}

function closure(): string {
  return ["graphemes", "isEmojiGrapheme", vector.helper].map((name) => declaration(name).getText(source)).join("\n");
}

/** 🔬️ Observes only the actual extracted helper's private real-Segmenter input, never a global prototype. */
function compiled(compiler: typeof compilers[number]): { split: (value: string) => Split; observations: Observation[] } {
  const observations: Observation[] = [], actual = realSegmenter();
  const segmenter = {
    segment(value: string): Iterable<Intl.SegmentData> {
      const segments = actual.segment(value), observation = { value, nextCalls: 0, iteratorCalls: 0 };
      observations.push(observation);
      return { [Symbol.iterator](): Iterator<Intl.SegmentData> { observation.iteratorCalls++; const iterator = segments[Symbol.iterator](); return { next(): IteratorResult<Intl.SegmentData> { observation.nextCalls++; return iterator.next(); } }; } };
    },
  };
  return { split: new Function("SEGMENTER", `${compiler.compile(closure())}\nreturn ${vector.helper};`)(segmenter) as (value: string) => Split, observations };
}

function eager(value: string): Split {
  const segments = [...realSegmenter().segment(value)].map((row) => row.segment);
  return segments.length === 0 || !/[\p{Extended_Pictographic}\p{Emoji_Presentation}\uFE0F\u20E3]/u.test(segments[0]!) ? { emoji: "", rest: value } : { emoji: segments[0]!, rest: segments.slice(1).join("") };
}

function scaling(row: ScalingCase): { input: string; expected: Split } {
  const suffix = row.suffixUnit.repeat(row.repetitions), input = row.prefix + suffix;
  return { input, expected: { emoji: row.emoji, rest: row.rest === "suffix" ? suffix : input } };
}

test("leading-grapheme vectors have closed language-neutral authority and independent JSON parsing", () => {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(import.meta.dir, "🧬️schema/🔣️.json"), "utf8")));
  expect(validate(vector), JSON.stringify(validate.errors)).toBe(true);
  for (const changed of [{ ...vector, extra: true }, { ...vector, helper: "cachedPrefix" }, { ...vector, semantics: { ...vector.semantics, iteratorAdvancesPerRequest: 2 } }, { ...vector, cases: vector.cases.map((row, index) => index ? row : { ...row, extra: true }) }]) expect(validate(changed)).toBe(false);
  const errors: ParseError[] = [];
  expect(parse(vectorBytes, errors, { disallowComments: true, allowTrailingComma: false })).toEqual(vector);
  expect(errors).toEqual([]);
  expect(new Set(vector.cases.map((row) => row.id)).size).toBe(vector.cases.length);
  expect(vector.cases.filter((row) => !row.segmentationOracle).map((row) => row.id)).toEqual(vector.oracleDivergences);
});

test("actual helper output preserves authored Unicode boundaries through Bun and TypeScript", () => {
  for (const compiler of compilers) {
    const actual = compiled(compiler);
    for (let round = 0; round < vector.rounds; round++) for (const row of vector.cases) {
      const expected = { emoji: row.emoji, rest: row.rest };
      expect([...realSegmenter().segment(row.input)][0]?.segment ?? "", row.id).toBe(row.first);
      expect(eager(row.input), row.id).toEqual(expected);
      expect(actual.split(row.input), `${compiler.id}:${row.id}`).toEqual(expected);
      expect(expected.emoji + expected.rest, row.id).toBe(row.input);
      if (expected.emoji) expect(row.input.slice(expected.emoji.length), row.id).toBe(expected.rest);
    }
    for (const row of vector.scalingCases) { const request = scaling(row); expect(eager(request.input), row.id).toEqual(request.expected); expect(actual.split(request.input), row.id).toEqual(request.expected); }
  }
});

test("installed independent Unicode oracles agree on their explicit overlap without redefining edge semantics", () => {
  const divergences: string[] = [];
  for (const row of vector.cases) {
    const first = toArray(row.input)[0] ?? "";
    if (row.segmentationOracle) expect(first, row.id).toBe(row.first);
    else { expect(first, row.id).not.toBe(row.first); divergences.push(row.id); }
    if (row.emojiOracle) {
      const match = emojiRegex().exec(row.input);
      expect(match?.index, row.id).toBe(0);
      expect(match?.[0], row.id).toBe(row.emoji);
      expect({ emoji: match![0], rest: row.input.slice(match![0].length) }, row.id).toEqual({ emoji: row.emoji, rest: row.rest });
    }
  }
  expect(divergences).toEqual(vector.oracleDivergences);
});

test("all current schema member prefixes retain native and independent segmentation parity", () => {
  const schema = JSON.parse(readFileSync(join(library, "🔣️taxonomy.json"), "utf8")) as { semanticDirectoryMemberKinds: Record<string, { memberNames: string[] }> };
  const names = Object.entries(schema.semanticDirectoryMemberKinds).flatMap(([id, row]) => row.memberNames.map((value) => ({ id, value })));
  expect(names.length).toBeGreaterThan(0);
  for (const compiler of compilers) {
    const actual = compiled(compiler);
    for (const { id, value } of names) {
      const expected = eager(value), first = toArray(value)[0] ?? "";
      expect(first, `${id}:${value}`).toBe(expected.emoji);
      expect({ emoji: first, rest: value.slice(first.length) }, `${id}:${value}`).toEqual(expected);
      expect(actual.split(value), `${compiler.id}:${id}:${value}`).toEqual(expected);
    }
  }
  console.info(`[DEBUG] Leading-grapheme corpus: ${names.length} current member occurrences; Bun/TypeScript and independent Lodash first-boundary parity`);
});

test("actual leading-grapheme declaration closure remains strictly typed without a replacement parser", () => {
  const constants = source.statements.filter((node): node is ts.VariableStatement => ts.isVariableStatement(node) && node.declarationList.declarations.some((row) => row.name.getText(source) === "SEGMENTER"));
  expect(constants).toHaveLength(1);
  const code = `${constants[0]!.getText(source)}\n${closure()}\nconst result: { emoji: string; rest: string } = splitLeadingEmoji('🧪️tests'); void result;`;
  const path = join(import.meta.dir, "🟦️declarations.ts"), options: ts.CompilerOptions = { noEmit: true, strict: true, target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext, skipLibCheck: true, types: [] };
  const host = ts.createCompilerHost(options), originalRead = host.readFile.bind(host), originalExists = host.fileExists.bind(host);
  host.readFile = (file) => file === path ? code : originalRead(file);
  host.fileExists = (file) => file === path || originalExists(file);
  expect(ts.getPreEmitDiagnostics(ts.createProgram([path], options, host)).map((row) => ts.flattenDiagnosticMessageText(row.messageText, "\n"))).toEqual([]);
});

for (const compiler of compilers) test(`${compiler.id} actual helper advances one grapheme iterator result per request`, () => {
  const actual = compiled(compiler), observations: { id: string; nextCalls: number; iteratorCalls: number }[] = [];
  for (let round = 0; round < vector.rounds; round++) for (const row of [...vector.cases, ...vector.scalingCases.map((item) => ({ id: item.id, input: scaling(item).input }))]) {
    const before = actual.observations.length;
    actual.split(row.input);
    expect(actual.observations).toHaveLength(before + 1);
    const observation = actual.observations[before]!;
    observations.push({ id: `${round}:${row.id}`, nextCalls: observation.nextCalls, iteratorCalls: observation.iteratorCalls });
  }
  const violations = observations.filter((row) => row.nextCalls !== vector.semantics.iteratorAdvancesPerRequest || row.iteratorCalls !== 1);
  console.info(`[DEBUG] Leading-grapheme ${compiler.id}: ${observations.length} requests, ${violations.length} iteration violations, maximum ${Math.max(...observations.map((row) => row.nextCalls))}; N ${createHash("sha256").update(sourceBytes).digest("hex")}`);
  expect(violations.length, JSON.stringify(violations.slice(0, 3))).toBe(0);
});

test("independent compiled sessions create fresh segment observations without retained request results", () => {
  for (const compiler of compilers) {
    const left = compiled(compiler), right = compiled(compiler);
    for (const row of vector.cases.slice(0, 7)) { expect(left.split(row.input)).toEqual({ emoji: row.emoji, rest: row.rest }); expect(right.split(row.input)).toEqual({ emoji: row.emoji, rest: row.rest }); }
    expect(left.observations).not.toBe(right.observations);
    expect(left.observations).toHaveLength(7);
    expect(right.observations).toHaveLength(7);
    const prior = right.observations.map((row) => ({ ...row }));
    left.split("🧪️fresh");
    expect(left.observations).toHaveLength(8);
    expect(right.observations).toEqual(prior);
    expect(readFileSync(sourcePath)).toEqual(sourceBytes);
  }
});

test("registers leading grapheme through its closed canonical route", async () => {
  const directory = join(import.meta.dir, "🧪️registration"), bytes = readFileSync(join(directory, "🔣️.json"), "utf8"), registration = JSON.parse(bytes);
  const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(directory, "🧬️schema/🔣️.json"), "utf8")));
  expect(validate(registration), JSON.stringify(validate.errors)).toBe(true);
  for (const changed of [{ ...registration, source: "../🧪️🌳️taxonomy-leading-grapheme/🟦️.ts" }, { ...registration, budget: 120000 }, { ...registration, budgetMs: 120000 }, { ...registration, filter: "selected" }, { ...registration, runner: "other" }, { ...registration, launchOrder: 410.208 }]) expect(validate(changed)).toBe(false);
  const errors: ParseError[] = [];
  expect(parse(bytes, errors, { disallowComments: true, allowTrailingComma: false })).toEqual(registration);
  expect(errors).toEqual([]);
  const repoRoot = resolve(library, "../../../../.."), packageRelative = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript", packageRoot = join(repoRoot, packageRelative);
  expect(join(repoRoot, registration.source)).toBe(import.meta.filename);
  const project = JSON.parse(readFileSync(join(packageRoot, "📋️project.json"), "utf8"));
  expect(project.targets[registration.target]).toBeDefined();
  expect(project.targets[registration.target]).toEqual({ executor: "nx:run-commands", options: { cwd: packageRelative, command: `bun ./📜️script.ts test ${registration.command}` } });
  const manifest = JSON.parse(readFileSync(join(packageRoot, "package.json"), "utf8"));
  expect(manifest.scripts[registration.target]).toBe(`nx run @semio-tech/repo-lib:${registration.target}`);
  const path = join(packageRoot, "📜️script.ts"), syntax = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const declarations = syntax.statements.filter((node) => ts.isClassDeclaration(node) && node.name?.text === "TestScript");
  expect(declarations).toHaveLength(1);
  for (const compiler of compilers) {
    const invocations: { executable: string; args: string[]; options: { cwd: string } }[] = [];
    class FixtureBundle { root = packageRoot; repoRoot = repoRoot; }
    const router = new Function("BundleScript", "join", "runTestBudgeted", "resolveTestLevel", compiler.compile(`${declarations[0]!.getText(syntax)}\nreturn new TestScript();`))(FixtureBundle, join, async (executable: string, args: string[], options: { cwd: string }) => { invocations.push({ executable, args, options }); }, () => { throw new Error("Leading grapheme fell through to generic routing"); });
    await router.run([registration.command]);
    expect(invocations).toEqual([{ executable: process.execPath, args: ["test", join(repoRoot, registration.source)], options: { cwd: repoRoot } }]);
  }
  for (const filename of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
    const parseErrors: ParseError[] = [], document = parse(readFileSync(join(repoRoot, filename), "utf8"), parseErrors);
    expect(parseErrors).toEqual([]);
    expect(document.configurations.filter((row: { name: string }) => row.name === registration.launchName)).toEqual([{ name: registration.launchName, type: "node-terminal", request: "launch", command: `bun nx run @semio-tech/repo-lib:${registration.target} --skip-nx-cache`, cwd: "${workspaceFolder}", presentation: { group: registration.launchGroup, order: registration.launchOrder } }]);
    expect(document.configurations.filter((row: { presentation?: { group: string; order: number } }) => row.presentation?.group === registration.launchGroup && row.presentation?.order === registration.launchOrder)).toHaveLength(1);
  }
});
