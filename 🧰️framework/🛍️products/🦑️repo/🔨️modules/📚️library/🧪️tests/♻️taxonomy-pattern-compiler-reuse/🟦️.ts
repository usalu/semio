import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { lstatSync, readFileSync } from "node:fs";
import { basename, isAbsolute, join, posix, relative, resolve, sep } from "node:path";
import Ajv from "ajv";
import picomatch from "picomatch";
import { parse, type ParseError } from "jsonc-parser";
import ts from "typescript";
import { taxonomyPathPatternMatches } from "../../🔍️discovery/🟦️.ts";
import * as discovery from "../../🔍️discovery/🟦️.ts";

type Matcher = Readonly<{ matches(path: string, pattern: string): boolean }>;
type Row = Readonly<{ id: string; path: string; pattern: string; expected: boolean }>;
type Vector = Readonly<{ schemaVersion: number; contractId: string; factory: string; rounds: number; uniqueNormalizedPatterns: number; cases: Row[]; invalidPatterns: string[]; oracle: { library: string; options: Record<string, string | boolean> }; integration: { loadedField: string; normalizerOwners: Record<string, number>; loadOwners: string[]; fixedContractCallCount: number; changedInput: { pointer: string; suffix: string }; freshProbe: { path: string; pattern: string } } }>;
const library = resolve(import.meta.dir, "../.."), sourcePath = join(library, "🔍️discovery/🟦️.ts");
const bytes = readFileSync(join(import.meta.dir, "../♻️taxonomy-pattern-compiler-reuse/🔣️.json"), "utf8"), vector: Vector = JSON.parse(bytes);
const source = ts.createSourceFile(sourcePath, readFileSync(sourcePath, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
const normalizerPath = join(library, "🧹️normalization/🟦️.ts"), normalizer = ts.createSourceFile(normalizerPath, readFileSync(normalizerPath, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
const compilers = [{ id: "bun", compile: (code: string): string => new Bun.Transpiler({ loader: "ts" }).transformSync(code) }, { id: "typescript", compile: (code: string): string => ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext } }).outputText }];
const pathForOracle = (path: string): string => path.replaceAll("\\", "/").replace(/^\.\//u, "").normalize("NFC");

function nodes<T extends ts.Node>(root: ts.Node, predicate: (node: ts.Node) => node is T): T[] {
  const result: T[] = [];
  const visit = (node: ts.Node): void => { if (predicate(node)) result.push(node); ts.forEachChild(node, visit); };
  visit(root);
  return result;
}

function owner(node: ts.Node): string {
  for (let parent = node.parent; parent; parent = parent.parent) if (ts.isFunctionDeclaration(parent) && parent.name) return parent.name.text;
  return "<module>";
}

function declaration(file: ts.SourceFile, name: string): ts.FunctionDeclaration {
  const found = file.statements.filter((node): node is ts.FunctionDeclaration => ts.isFunctionDeclaration(node) && node.name?.text === name);
  expect(found.length, `actual ${name} declaration`).toBe(1);
  return found[0]!;
}

/** 🔬️ Instruments only this exact compiled declaration closure, never the process-global RegExp. */
function compiled(compiler: typeof compilers[number], factory: boolean): { oneOff: typeof taxonomyPathPatternMatches; create: () => Matcher; expressions: RegExp[]; tests: () => number } {
  const names = ["taxonomyPatternExpression", "taxonomyPathPatternMatches", ...(factory ? [vector.factory] : [])];
  const declarations = names.map((name) => {
    const found = source.statements.filter((node) => ts.isFunctionDeclaration(node) && node.name?.text === name);
    expect(found.length, `${compiler.id}: actual ${name} declaration`).toBe(1);
    return found[0]!.getText(source).replace(/^export\s+/u, "");
  });
  const expressions: RegExp[] = [];
  let tests = 0;
  class ObservedExpression extends RegExp {
    constructor(pattern: string, flags?: string) { super(pattern, flags); expressions.push(this); }
    override test(value: string): boolean { tests++; return super.test(value); }
  }
  const javascript = compiler.compile(declarations.join("\n"));
  const result = new Function("RegExp", `${javascript}\nreturn { oneOff: taxonomyPathPatternMatches, create: ${factory ? vector.factory : "undefined"} };`)(ObservedExpression) as { oneOff: typeof taxonomyPathPatternMatches; create: () => Matcher };
  return { ...result, expressions, tests: () => tests };
}

test("pattern compilation reuse has a closed language-neutral schema and independent JSON parser", () => {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(import.meta.dir, "🧬️schema/🔣️.json"), "utf8")));
  expect(validate(vector), JSON.stringify(validate.errors)).toBe(true);
  for (const changed of [{ ...vector, extra: true }, { ...vector, rounds: 1 }, { ...vector, uniqueNormalizedPatterns: 1 }, { ...vector, factory: "processGlobalCache" }]) expect(validate(changed)).toBe(false);
  const errors: ParseError[] = [];
  expect(parse(bytes, errors, { disallowComments: true, allowTrailingComma: false })).toEqual(vector);
  expect(errors).toEqual([]);
  expect(new Set(vector.cases.map((row) => row.id)).size).toBe(vector.cases.length);
  expect(new Set(vector.cases.map((row) => row.pattern.normalize("NFC"))).size).toBe(vector.uniqueNormalizedPatterns);
});

test("actual current pattern semantics agree with both compilers and the independent glob oracle", () => {
  for (const compiler of compilers) {
    const actual = compiled(compiler, false);
    for (let round = 0; round < vector.rounds; round++) for (const row of vector.cases) {
      const oracle = picomatch(row.pattern.normalize("NFC"), vector.oracle.options)(pathForOracle(row.path));
      expect(oracle, row.id).toBe(row.expected);
      expect(taxonomyPathPatternMatches(row.path, row.pattern), row.id).toBe(row.expected);
      expect(actual.oneOff(row.path, row.pattern), row.id).toBe(row.expected);
    }
    expect(actual.expressions).toHaveLength(vector.cases.length * vector.rounds);
    expect(actual.tests()).toBe(vector.cases.length * vector.rounds);
    console.info(`[DEBUG] Pattern compiler baseline ${compiler.id}: ${actual.expressions.length} constructions for ${vector.uniqueNormalizedPatterns} distinct normalized patterns`);
  }
});

test("one query matcher compiles each normalized pattern once without memoizing path results", () => {
  for (const compiler of compilers) {
    const actual = compiled(compiler, true), matcher = actual.create();
    expect(actual.expressions).toHaveLength(0);
    for (let round = 0; round < vector.rounds; round++) for (const row of vector.cases) expect(matcher.matches(row.path, row.pattern), `${compiler.id}:${row.id}`).toBe(row.expected);
    expect(actual.expressions).toHaveLength(vector.uniqueNormalizedPatterns);
    expect(actual.tests()).toBe(vector.cases.length * vector.rounds);
  }
});

test("interleaved matcher sessions do not share compiled patterns or retained query state", () => {
  for (const compiler of compilers) {
    const actual = compiled(compiler, true), left = actual.create(), right = actual.create();
    for (const row of vector.cases) { expect(left.matches(row.path, row.pattern)).toBe(row.expected); expect(right.matches(row.path, row.pattern)).toBe(row.expected); }
    expect(actual.expressions).toHaveLength(vector.uniqueNormalizedPatterns * 2);
    const fresh = actual.create();
    expect(fresh.matches("changed.json", "changed.json")).toBe(true);
    expect(fresh.matches("original.json", "changed.json")).toBe(false);
    expect(actual.expressions).toHaveLength(vector.uniqueNormalizedPatterns * 2 + 1);
  }
});

test("invalid patterns retain lazy fresh errors without poisoning a matcher", () => {
  for (const compiler of compilers) {
    const actual = compiled(compiler, true), matcher = actual.create();
    expect(actual.expressions).toHaveLength(0);
    for (const pattern of vector.invalidPatterns) {
      let original: unknown;
      try { taxonomyPathPatternMatches("unmatched", pattern); } catch (error) { original = error; }
      expect(original).toBeInstanceOf(Error);
      const failures: unknown[] = [];
      for (let repeat = 0; repeat < 2; repeat++) { try { matcher.matches("unmatched", pattern); } catch (error) { failures.push(error); } }
      expect(failures).toHaveLength(2);
      expect(failures[0]).not.toBe(failures[1]);
      for (const error of failures) { expect((error as Error).name).toBe((original as Error).name); expect((error as Error).message).toBe((original as Error).message); }
    }
    expect(matcher.matches("valid", "valid")).toBe(true);
    expect(matcher.matches("other", "valid")).toBe(false);
    expect(actual.expressions).toHaveLength(1);
  }
});

test("the matcher exposes no mutable regex state and repeated tests remain stateless", () => {
  for (const compiler of compilers) {
    const actual = compiled(compiler, true), matcher = actual.create();
    expect(Object.keys(matcher)).toEqual(["matches"]);
    expect(Object.isFrozen(matcher)).toBe(true);
    expect(Reflect.set(matcher, "matches", () => false)).toBe(false);
    for (const value of ["a.ts", "b.js", "a.ts", "c.ts", "b.js", "a.ts"]) expect(matcher.matches(value, "*.ts")).toBe(value.endsWith(".ts"));
    expect(actual.expressions).toHaveLength(1);
    expect(actual.expressions[0]!.flags).toBe("u");
    expect(actual.expressions[0]!.lastIndex).toBe(0);
    Object.freeze(actual.expressions[0]);
    expect(matcher.matches("a.ts", "*.ts")).toBe(true);
    expect(actual.expressions[0]!.lastIndex).toBe(0);
  }
});

test("every normalizer pattern query has a required invocation-owned matcher and fresh load", () => {
  const loaded = normalizer.statements.find((node): node is ts.InterfaceDeclaration => ts.isInterfaceDeclaration(node) && node.name.text === "LoadedTaxonomy")!;
  const field = loaded.members.find((node): node is ts.PropertySignature => ts.isPropertySignature(node) && node.name.getText(normalizer) === vector.integration.loadedField);
  expect(field?.type?.getText(normalizer)).toBe("TaxonomyPathMatcher");
  expect(field?.questionToken).toBeUndefined();
  const calls = nodes(normalizer, ts.isCallExpression), owners: Record<string, number> = {};
  expect(calls.filter((node) => node.expression.getText(normalizer) === "taxonomyPathPatternMatches")).toEqual([]);
  for (const call of calls.filter((node) => /(?:^|\.)pathMatcher\.matches$/u.test(node.expression.getText(normalizer)))) {
    expect(call.questionDotToken).toBeUndefined();
    expect(call.expression.getText(normalizer)).not.toContain("?");
    const name = owner(call);
    expect(call.expression.getText(normalizer)).toBe(["validatedContractPattern", "parseTaxonomy", "renderCatalogGlob"].includes(name) ? "pathMatcher.matches" : "taxonomy.pathMatcher.matches");
    owners[name] = (owners[name] ?? 0) + 1;
  }
  expect(owners).toEqual(vector.integration.normalizerOwners);
  expect(calls.filter((node) => node.expression.getText(normalizer) === "createTaxonomyPathMatcher").map(owner).sort()).toEqual(["parseTaxonomy", "renderCatalogGlob"]);
  expect(calls.filter((node) => node.expression.getText(normalizer) === "loadTaxonomy").map(owner)).toEqual(vector.integration.loadOwners);
  const fixed = declaration(normalizer, "matchingFixedContracts"), parameter = fixed.parameters.find((entry) => entry.name.getText(normalizer) === "taxonomy");
  expect(parameter?.type?.getText(normalizer)).toBe("LoadedTaxonomy");
  expect(parameter?.questionToken).toBeUndefined();
  expect(parameter?.initializer).toBeUndefined();
  const fixedCalls = calls.filter((node) => node.expression.getText(normalizer) === "matchingFixedContracts"), position = fixed.parameters.indexOf(parameter!);
  expect(fixedCalls).toHaveLength(vector.integration.fixedContractCallCount);
  for (const call of fixedCalls) expect(call.arguments[position]?.getText(normalizer)).toBe("taxonomy");
  expect(normalizer.statements.some((node) => ts.isFunctionDeclaration(node) && node.name?.text === "validatedContractPattern")).toBe(false);
  const parser = declaration(normalizer, "parseTaxonomy"), factoryCall = nodes(parser, ts.isCallExpression).find((node) => node.expression.getText(normalizer) === "createTaxonomyPathMatcher")!;
  const validatorCall = nodes(parser, ts.isCallExpression).find((node) => node.expression.getText(normalizer) === "validateTaxonomy")!;
  expect(factoryCall.pos).toBeGreaterThan(validatorCall.end);
  expect(nodes(parser, ts.isShorthandPropertyAssignment).some((node) => node.name.text === "pathMatcher")).toBe(true);
  const loader = declaration(normalizer, "loadTaxonomy"), loaderCalls = nodes(loader, ts.isCallExpression).map((node) => node.expression.getText(normalizer));
  for (const name of ["assertLexicalInputOutsideOpaque", "semanticOwnedInputFileSnapshot", "parseTaxonomy", "JSON.parse"]) expect(loaderCalls.filter((entry) => entry === name)).toHaveLength(1);
  expect(loader.getText(normalizer)).toContain('if (!Buffer.from(text).equals(bytes)) throw new Error("Taxonomy schema has lossy UTF-8: " + path)');
  const validator = declaration(source, "validateTaxonomy"), validationCalls = nodes(validator, ts.isCallExpression);
  expect(validationCalls.filter((node) => node.expression.getText(source) === "createTaxonomyPathMatcher")).toHaveLength(1);
  expect(validationCalls.filter((node) => ["taxonomyPathPatternMatches", "taxonomyPatternExpression"].includes(node.expression.getText(source)))).toEqual([]);
  expect(validationCalls.filter((node) => node.expression.getText(source) === "pathMatcher.matches")).toHaveLength(2);
});

test("actual load and parse declarations reread and revalidate independent matcher sessions", () => {
  const repoRoot = resolve(library, "../../../../.."), schemaPath = join(library, "🔣️taxonomy.json");
  const names = ["loadTaxonomy", "assertLexicalInputOutsideOpaque", "assertNoFollowAncestors", "lstatOrNull", "parseTaxonomy", "record", "canonicalJson", "canonicalValue", "canonicalArrayKey", "requiredString", "stringArray", "requireExactKeys", "normalizeRelative", "sourceRelative", "fixedExpiry", "splitLeadingEmoji", "graphemes", "isEmojiGrapheme", "emojiFold"];
  const functions = names.map((name) => declaration(normalizer, name).getText(normalizer).replace(/^export\s+/u, ""));
  const constants = ["LEXICAL_OPAQUE_ROOTS", "TAXONOMY_RELATIVE_PATH", "SEGMENTER"].map((name) => {
    const rows = normalizer.statements.filter((node): node is ts.VariableStatement => ts.isVariableStatement(node) && node.declarationList.declarations.some((entry) => entry.name.getText(normalizer) === name));
    expect(rows).toHaveLength(1);
    return rows[0]!.getText(normalizer);
  });
  type Loaded = { pathMatcher: Matcher; schema: { fixedFilenameContracts: Record<string, { reason: string }> }; discoverySchema: discovery.Taxonomy; input: discovery.SemanticOwnedInputFileSnapshot };
  for (const compiler of compilers) {
    const actual = compiled(compiler, true);
    let reads = 0, parses = 0, validations = 0, variant: "original" | "changed" | "invalid" | "lossy" | "syntax" = "original";
    const captured: discovery.SemanticOwnedInputFileSnapshot[] = [];
    const snapshot = (root: string, path: string): discovery.SemanticOwnedInputFileSnapshot | null => {
      reads++;
      const input = discovery.semanticOwnedInputFileSnapshot(root, path);
      if (!input) return null;
      captured.push(input);
      const document = JSON.parse(Buffer.from(input.bytes).toString("utf8"));
      if (variant === "changed") document.fixedFilenameContracts["cargo-manifest"].reason += vector.integration.changedInput.suffix;
      if (variant === "invalid") document.windowEmptyFacetFileKindId = "not-registered";
      const bytes = variant === "lossy" ? Buffer.from([0xff]) : variant === "syntax" ? Buffer.from("{") : variant === "original" ? input.bytes : Buffer.from(JSON.stringify(document));
      return { ...input, bytes, size: bytes.byteLength, contentHash: createHash("sha256").update(bytes).digest("hex") };
    };
    const environment = { ...discovery, basename, isAbsolute, join, posix, relative, resolve, sep, lstatSync, Buffer, createTaxonomyPathMatcher: actual.create, semanticOwnedInputFileSnapshot: snapshot, validateTaxonomy: (document: discovery.Taxonomy): string[] => { validations++; return discovery.validateTaxonomy(document); }, JSON: { stringify: JSON.stringify, parse: (text: string): unknown => { parses++; return JSON.parse(text); } } };
    const javascript = compiler.compile([...constants, ...functions].join("\n"));
    const load = new Function(...Object.keys(environment), `${javascript}\nreturn loadTaxonomy;`)(...Object.values(environment)) as (options: { repoRoot: string; taxonomyPath: string }) => Loaded;
    const options = { repoRoot, taxonomyPath: schemaPath }, left = load(options), firstCount = actual.expressions.length, right = load(options);
    expect(firstCount).toBeGreaterThan(0);
    const document = left.discoverySchema;
    const patterns = [...Object.values(document.fixedFilenameContracts).map((row) => row.pathPattern), ...Object.values(document.fixedDirectoryContracts).map((row) => row.pathPattern), ...Object.values(document.fileKindResolutionRules).map((row) => "pathPattern" in row ? row.pathPattern : undefined), ...Object.values(document.scopedFileKinds).map((row) => row.pathPattern), ...Object.values(document.generatorContracts).flatMap((row) => row.inputPatterns)].filter((pattern): pattern is string => typeof pattern === "string");
    expect(firstCount).toBe(new Set(patterns.map((pattern) => pattern.normalize("NFC"))).size);
    expect(actual.expressions).toHaveLength(firstCount * 2);
    expect(left.pathMatcher).not.toBe(right.pathMatcher);
    expect(left.discoverySchema).not.toBe(right.discoverySchema);
    expect(left.input).not.toBe(right.input);
    expect(left.discoverySchema).toEqual(right.discoverySchema);
    expect([reads, parses, validations]).toEqual([2, 2, 2]);
    const probe = vector.integration.freshProbe;
    for (const session of [left, right]) for (let repeat = 0; repeat < 3; repeat++) expect(session.pathMatcher.matches(probe.path, probe.pattern)).toBe(picomatch(probe.pattern, vector.oracle.options)(probe.path));
    expect(actual.expressions).toHaveLength(firstCount * 2 + 2);
    variant = "changed";
    const changed = load(options);
    expect(changed.schema.fixedFilenameContracts["cargo-manifest"]!.reason).toBe(left.schema.fixedFilenameContracts["cargo-manifest"]!.reason + vector.integration.changedInput.suffix);
    expect(changed.input.contentHash).not.toBe(left.input.contentHash);
    expect(changed.pathMatcher).not.toBe(left.pathMatcher);
    expect(actual.expressions).toHaveLength(firstCount * 3 + 2);
    expect([reads, parses, validations]).toEqual([3, 3, 3]);
    variant = "invalid";
    expect(() => load(options)).toThrow("discovery contract validation failed");
    expect([reads, parses, validations]).toEqual([4, 4, 4]);
    variant = "lossy";
    expect(() => load(options)).toThrow("lossy UTF-8");
    expect([reads, parses, validations]).toEqual([5, 4, 4]);
    variant = "syntax";
    expect(() => load(options)).toThrow(SyntaxError);
    expect([reads, parses, validations]).toEqual([6, 5, 4]);
    expect(() => load({ ...options, taxonomyPath: "compose/never-read.json" })).toThrow("inside an opaque path");
    expect([reads, parses, validations]).toEqual([6, 5, 4]);
    expect(captured).toHaveLength(6);
    expect(new Set(captured.map((input) => input.contentHash)).size).toBe(1);
    console.info(`[DEBUG] Fresh matcher ${compiler.id}: ${firstCount} unique schema compilations per load, ${reads} actual reads, ${parses} parse attempts, ${validations} complete validations`);
  }
});

test("the owned factory interface and actual declarations satisfy strict TypeScript", () => {
  const ownedInterface = source.statements.find((node): node is ts.InterfaceDeclaration => ts.isInterfaceDeclaration(node) && node.name.text === "TaxonomyPathMatcher");
  expect(ownedInterface).toBeDefined();
  const factory = declaration(source, vector.factory);
  expect(factory.parameters).toHaveLength(0);
  const code = [ownedInterface!.getText(source), declaration(source, "taxonomyPatternExpression").getText(source), factory.getText(source), "const matcher: TaxonomyPathMatcher = createTaxonomyPathMatcher(); const matched: boolean = matcher.matches('x.ts', '*.ts'); void matched;"].join("\n");
  const path = join(import.meta.dir, "🟦️declarations.ts"), options: ts.CompilerOptions = { noEmit: true, strict: true, target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext, skipLibCheck: true, types: [] };
  const host = ts.createCompilerHost(options), originalRead = host.readFile.bind(host), originalExists = host.fileExists.bind(host);
  host.readFile = (file) => file === path ? code : originalRead(file);
  host.fileExists = (file) => file === path || originalExists(file);
  const program = ts.createProgram([path], options, host);
  expect(ts.getPreEmitDiagnostics(program).map((diagnostic) => ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n"))).toEqual([]);
});

test("changed runtime matcher declarations and call sites are strictly typed", () => {
  const program = ts.createProgram([sourcePath, normalizerPath], { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, strict: true, allowImportingTsExtensions: true, skipLibCheck: true, noEmit: true, types: ["node"] });
  for (const path of [sourcePath, normalizerPath]) {
    const file = program.getSourceFile(path)!;
    const diagnostics = [...program.getSyntacticDiagnostics(file), ...program.getSemanticDiagnostics(file)];
    const declarations = file.statements.filter((node) => ts.isFunctionDeclaration(node) && node.name?.text === vector.factory || ts.isInterfaceDeclaration(node) && node.name.text === "TaxonomyPathMatcher");
    const calls = nodes(file, ts.isCallExpression).filter((node) => /(?:^|\.)pathMatcher\.matches$/u.test(node.expression.getText(file)) || ["createTaxonomyPathMatcher", "matchingFixedContracts"].includes(node.expression.getText(file)));
    const spans = [...declarations, ...calls];
    const selected = diagnostics.filter((diagnostic) => diagnostic.start !== undefined && spans.some((node) => diagnostic.start! >= node.pos && diagnostic.start! < node.end));
    expect(selected.map((diagnostic) => `${relative(library, path)}:${file.getLineAndCharacterOfPosition(diagnostic.start ?? 0).line + 1} TS${diagnostic.code} ${ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n")}`)).toEqual([]);
  }
});

test("registers pattern compiler reuse through its closed canonical route", async () => {
  const directory = join(import.meta.dir, "../♻️taxonomy-pattern-compiler-reuse/🧪️registration"), bytes = readFileSync(join(directory, "../♻️taxonomy-pattern-compiler-reuse/🔣️.json"), "utf8"), registration = JSON.parse(bytes);
  const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(directory, "../♻️taxonomy-pattern-compiler-reuse/🧬️schema/🔣️.json"), "utf8")));
  expect(validate(registration), JSON.stringify(validate.errors)).toBe(true);
  for (const changed of [{ ...registration, source: "../♻️taxonomy-pattern-compiler-reuse/🟦️.ts" }, { ...registration, budget: 120000 }, { ...registration, budgetMs: 120000 }, { ...registration, runner: "other" }, { ...registration, launchOrder: 410.205 }]) expect(validate(changed)).toBe(false);
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
  expect(declarations.length).toBe(1);
  const code = `${declarations[0]!.getText(syntax)}\nreturn new TestScript();`;
  for (const compiler of compilers) {
    const invocations: { executable: string; args: string[]; options: { cwd: string } }[] = [];
    class FixtureBundle { root = packageRoot; repoRoot = repoRoot; }
    const router = new Function("BundleScript", "join", "runTestBudgeted", "resolveTestLevel", compiler.compile(code))(FixtureBundle, join, async (executable: string, args: string[], options: { cwd: string }) => { invocations.push({ executable, args, options }); }, () => { throw new Error("Pattern reuse fell through to generic routing"); });
    await router.run([registration.command]);
    expect(invocations).toEqual([{ executable: process.execPath, args: ["test", join(repoRoot, registration.source)], options: { cwd: repoRoot } }]);
  }
  for (const filename of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
    const parseErrors: ParseError[] = [], document = parse(readFileSync(join(repoRoot, filename), "utf8"), parseErrors);
    expect(parseErrors).toEqual([]);
    const entries = document.configurations.filter((row: { name: string }) => row.name === registration.launchName);
    expect(entries).toEqual([{ name: registration.launchName, type: "node-terminal", request: "launch", command: `bun nx run @semio-tech/repo-lib:${registration.target} --skip-nx-cache`, cwd: "${workspaceFolder}", presentation: { group: registration.launchGroup, order: registration.launchOrder } }]);
    expect(document.configurations.filter((row: { presentation?: { group: string; order: number } }) => row.presentation?.group === registration.launchGroup && row.presentation?.order === registration.launchOrder)).toHaveLength(1);
  }
});
