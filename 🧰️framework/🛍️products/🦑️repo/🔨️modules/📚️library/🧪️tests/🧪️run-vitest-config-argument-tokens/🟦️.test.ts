import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import ts from "typescript";

const root = resolve(import.meta.dir, "../../../../../../../");
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const text = readFileSync(resolve(root, library, "🧹️normalization/🟦️.ts"), "utf8");
const syntax = ts.createSourceFile("🟦️.ts", text, ts.ScriptTarget.Latest, true);
const helpers = new Set(["lineLocation"]);
const constants = new Set(["indexedLineContent", "indexedLineStarts"]);
const support = syntax.statements.filter((node) => ts.isFunctionDeclaration(node) ? helpers.has(node.name?.text ?? "") : ts.isVariableStatement(node) && node.declarationList.declarations.some((declaration) => constants.has(declaration.name.getText(syntax)))).map((node) => node.getText(syntax)).join("\n");
const declaration = syntax.statements.filter((node) => ts.isFunctionDeclaration(node) && node.name?.text === "runVitestConfigArgumentTokens");
if (declaration.length !== 1) throw new Error("Expected one actual runVitestConfigArgumentTokens implementation");
const source = declaration[0]!.getText(syntax);
const compilers = [
  { name: "Bun", compile: (code: string) => new Bun.Transpiler({ loader: "ts" }).transformSync(code) },
  { name: "TypeScript", compile: (code: string) => ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText },
];

type Row = { readonly value: string; readonly start: number; readonly end: number };

/** 🧪️ Executes the actual private scanner through independent compilers. */
function implementation(compiler: typeof compilers[number]): (content: string) => Row[] {
  return new Function(compiler.compile(support) + "\n" + compiler.compile(source) + "\nreturn runVitestConfigArgumentTokens;")();
}

const CALL_WITH_CONFIG = 'runVitest(this.root, rest, "🧪️vitest.config.ts");';
const CALL_WITH_QUOTED_SEGMENTS_AND_CONFIG = 'await runVitest(this.root, ["🧪️browser-frame-transport.test.ts", "🧪️browser-interactive-job-port.test.ts", ...segments], "🧪️vitest.config.ts");';
const CALL_WITHOUT_CONFIG = "runVitest(this.root, rest);";
const DECLARATION_DEFAULT = 'export async function runVitest(bundleRoot: string, segments: string[], config = "🧪️vitest.config.ts"): Promise<void> {';

for (const compiler of compilers) test(compiler.name + " captures the config argument of a plain runVitest call", () => {
  const scan = implementation(compiler);
  const rows = scan(CALL_WITH_CONFIG);
  expect(rows).toHaveLength(1);
  expect(rows[0]).toMatchObject({ value: "🧪️vitest.config.ts" });
  expect(CALL_WITH_CONFIG.slice(rows[0]!.start, rows[0]!.end)).toBe("🧪️vitest.config.ts");
});

for (const compiler of compilers) test(compiler.name + " picks the trailing config string, not an earlier quoted segments-array entry", () => {
  const scan = implementation(compiler);
  const rows = scan(CALL_WITH_QUOTED_SEGMENTS_AND_CONFIG);
  expect(rows).toHaveLength(1);
  expect(rows[0]).toMatchObject({ value: "🧪️vitest.config.ts" });
  expect(CALL_WITH_QUOTED_SEGMENTS_AND_CONFIG.slice(rows[0]!.start, rows[0]!.end)).toBe("🧪️vitest.config.ts");
});

for (const compiler of compilers) test(compiler.name + " yields no token for a call that omits the config argument", () => {
  const scan = implementation(compiler);
  expect(scan(CALL_WITHOUT_CONFIG)).toHaveLength(0);
});

for (const compiler of compilers) test(compiler.name + " also captures the function declaration's own default value", () => {
  const scan = implementation(compiler);
  const rows = scan(DECLARATION_DEFAULT);
  expect(rows).toHaveLength(1);
  expect(rows[0]).toMatchObject({ value: "🧪️vitest.config.ts" });
});

test("both independent compilers agree on the exact captured span", () => {
  const [bun, typescript] = compilers.map((compiler) => implementation(compiler)(CALL_WITH_QUOTED_SEGMENTS_AND_CONFIG));
  expect(bun).toEqual(typescript);
});
