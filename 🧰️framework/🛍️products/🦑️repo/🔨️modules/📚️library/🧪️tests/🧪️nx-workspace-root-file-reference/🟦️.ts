import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import ts from "typescript";

const root = resolve(import.meta.dir, "../../../../../../../");
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const text = readFileSync(resolve(root, library, "🧹️normalization/🟦️.ts"), "utf8");
const syntax = ts.createSourceFile("../🧪️🪐️nx-workspace-root-file-reference/🟦️.ts", text, ts.ScriptTarget.Latest, true);
type Token = { adapter: string; structuredLocation: string; start: number; end: number; value: string; targetValues?: string[]; rewriteKind?: string; rewriteData?: Record<string, unknown>; unsupportedReason?: string };
type Parser = (path: string, content: string, adapter: "json" | "jsonc") => Token[];
const helpers = new Set(["normalizeRelative", "sourceRelative", "emojiFold", "graphemes", "isEmojiGrapheme", "splitLeadingEmoji", "lineLocation", "embeddedArgumentTokens", "artifactRootForPath", "mutationStructuralPaths", "canonicalProjectionSuffix", "projectionKey", "projectedStructuralValue", "structuralProjectionToken", "structuralTokensInFragment"]);
const constants = new Set(["SEGMENTER", "indexedLineContent", "indexedLineStarts", "OLD_MUTATION_TEST_PREFIX_SOURCE", "OLD_MUTATION_STRUCTURE_SOURCE"]);
const support = syntax.statements.filter((node) => ts.isFunctionDeclaration(node) ? helpers.has(node.name?.text ?? "") : ts.isVariableStatement(node) && node.declarationList.declarations.some((declaration) => constants.has(declaration.name.getText(syntax)))).map((node) => node.getText(syntax)).join("\n");
const declaration = syntax.statements.filter((node) => ts.isFunctionDeclaration(node) && node.name?.text === "jsonTokens");
if (declaration.length !== 1) throw new Error("Expected one actual jsonTokens implementation");
const parserSource = declaration[0].getText(syntax);
const compilers = [
  { name: "Bun", compile: (source: string) => new Bun.Transpiler({ loader: "ts" }).transformSync(source) },
  { name: "TypeScript", compile: (source: string) => ts.transpileModule(source, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText },
];

/** 🧬️ Executes the actual private parser through independent compilers with a stub owner lookup. */
function implementation(compiler: typeof compilers[number]): Parser {
  const dependencies = new Function(compiler.compile(support) + "\nreturn { artifactRootForPath, structuralTokensInFragment, mutationStructuralPaths, embeddedArgumentTokens };")();
  return new Function("artifactRootForPath", "structuralTokensInFragment", "mutationStructuralPaths", "embeddedArgumentTokens", compiler.compile(parserSource) + "\nreturn jsonTokens;")(() => null, dependencies.structuralTokensInFragment, dependencies.mutationStructuralPaths, dependencies.embeddedArgumentTokens);
}

/** 🧪️ Fixture-only, non-real paths — a genuine repo path here would itself become a live physical
 * reference for the taxonomy plan scanner to rewrite once its target moves. */
const FIXTURE_COMPONENT = ["🧰️framework", "🔨️modules", "🧪️nx-workspace-root-fixture", "../🧪️🪐️nx-workspace-root-file-reference/🟦️.ts"].join("/");
const FIXTURE_GLOB = ["🧰️framework", "🔨️modules", "🧪️nx-workspace-root-fixture", "🧫️fixtures", "**/*"].join("/");

for (const compiler of compilers) test(compiler.name + " detects a non-glob {workspaceRoot} value as a rewritable file reference", () => {
  const parse = implementation(compiler);
  const content = JSON.stringify({ namedInputs: { default: [`{workspaceRoot}/${FIXTURE_COMPONENT}`] } });
  const rows = parse("📋️project.json", content, "json").filter((token) => token.structuredLocation.includes("/workspace-file@"));
  expect(rows).toHaveLength(1);
  expect(rows[0]).toMatchObject({ value: `{workspaceRoot}/${FIXTURE_COMPONENT}`, targetValues: [FIXTURE_COMPONENT], rewriteKind: "path-prefix", rewriteData: { prefix: "{workspaceRoot}/", suffix: "" } });
});

for (const compiler of compilers) test(compiler.name + " still detects a {workspaceRoot} directory glob, unaffected by the file-reference addition", () => {
  const parse = implementation(compiler);
  const content = JSON.stringify({ namedInputs: { default: [`{workspaceRoot}/${FIXTURE_GLOB}`] } });
  const rows = parse("📋️project.json", content, "json");
  const globRows = rows.filter((token) => token.structuredLocation.includes("/workspace-glob@"));
  const fileRows = rows.filter((token) => token.structuredLocation.includes("/workspace-file@"));
  expect(globRows).toHaveLength(1);
  expect(fileRows).toHaveLength(0);
  expect(globRows[0]).toMatchObject({ targetValues: ["🧰️framework/🔨️modules/🧪️nx-workspace-root-fixture/🧫️fixtures"], rewriteKind: "path-prefix" });
});

for (const compiler of compilers) test(compiler.name + " never invents a rewrite for a {projectRoot} glob", () => {
  const parse = implementation(compiler);
  const content = JSON.stringify({ namedInputs: { default: ["{projectRoot}/**/*"] } });
  const rows = parse("📋️project.json", content, "json");
  expect(rows.some((token) => token.structuredLocation.includes("/workspace-file@") || token.structuredLocation.includes("/workspace-glob@"))).toBe(false);
});

test("both independent compilers agree on the exact workspace-file token for the fixture component", () => {
  const content = JSON.stringify({ namedInputs: { default: [`{workspaceRoot}/${FIXTURE_COMPONENT}`] } });
  const [bun, typescript] = compilers.map((compiler) => implementation(compiler)("📋️project.json", content, "json").filter((token) => token.structuredLocation.includes("/workspace-file@")));
  expect(bun).toEqual(typescript);
});
