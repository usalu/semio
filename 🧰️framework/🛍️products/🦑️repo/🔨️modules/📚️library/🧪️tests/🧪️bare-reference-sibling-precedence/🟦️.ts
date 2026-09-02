import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { dirname, posix, resolve } from "node:path";
import ts from "typescript";

const root = resolve(import.meta.dir, "../../../../../../../");
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const text = readFileSync(resolve(root, library, "🧹️normalization/🟦️.ts"), "utf8");
const syntax = ts.createSourceFile("🟦️.ts", text, ts.ScriptTarget.Latest, true);
const names = ["resolveReferencePath", "referencePathIndex", "addUniqueIndex", "ancestorReferenceCoordinateRoot", "splitTokenSuffix", "normalizeRelative", "sourceRelative"];
const declarations = Object.fromEntries(names.map((name) => {
  const found = syntax.statements.filter((node) => ts.isFunctionDeclaration(node) && node.name?.text === name);
  expect(found, name).toHaveLength(1);
  return [name, found[0]!.getText(syntax)];
}));
const source = names.map((name) => declarations[name]).join("\n");
const compilers = [
  { name: "Bun", compile: (code: string) => new Bun.Transpiler({ loader: "ts" }).transformSync(code) },
  { name: "TypeScript", compile: (code: string) => ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText },
];

type PathIndex = Readonly<Record<string, unknown>>;
type Impl = { readonly resolveReferencePath: (referencePath: string, token: string, index: PathIndex) => string | null; readonly referencePathIndex: (paths: Iterable<string>) => PathIndex };

/** 🧪️ Executes the actual private resolver through independent compilers with real node:path. */
function implementation(compiler: typeof compilers[number]): Impl {
  return new Function("posix", "dirname", compiler.compile(source) + "\nreturn { resolveReferencePath, referencePathIndex };")(posix, dirname);
}
function buildIndexWith(impl: Impl, paths: readonly string[]): PathIndex {
  return impl.referencePathIndex(paths);
}

const FIXTURE_ROOT_CONFIG = "🧪️bare-reference-sibling-fixture.config.ts";
const FIXTURE_MODULE = "🧰️framework/🔨️modules/🧪️bare-reference-sibling-fixture";
const FIXTURE_SIBLING_CONFIG = `${FIXTURE_MODULE}/📦️packages/🟦️typescript/🧪️bare-reference-sibling-fixture.config.ts`;
const FIXTURE_SCRIPT = `${FIXTURE_MODULE}/📦️packages/🟦️typescript/📜️script.ts`;
const FIXTURE_ROOT_SUBDIR = `${FIXTURE_MODULE}/🎈️only-at-repo-root.config.ts`;

for (const compiler of compilers) test(compiler.name + " prefers a same-directory sibling over an unrelated repository-root file for a bare single-segment token", () => {
  const impl = implementation(compiler);
  const index = buildIndexWith(impl, [FIXTURE_ROOT_CONFIG, FIXTURE_SIBLING_CONFIG, FIXTURE_SCRIPT]);
  expect(impl.resolveReferencePath(FIXTURE_SCRIPT, FIXTURE_ROOT_CONFIG, index)).toBe(FIXTURE_SIBLING_CONFIG);
});

for (const compiler of compilers) test(compiler.name + " still falls back to the repository-root file when no sibling of that bare name exists", () => {
  const impl = implementation(compiler);
  const index = buildIndexWith(impl, [FIXTURE_ROOT_CONFIG, FIXTURE_SCRIPT]);
  expect(impl.resolveReferencePath(FIXTURE_SCRIPT, FIXTURE_ROOT_CONFIG, index)).toBe(FIXTURE_ROOT_CONFIG);
});

for (const compiler of compilers) test(compiler.name + " leaves an explicit relative token untouched by the sibling-first reordering", () => {
  const impl = implementation(compiler);
  const index = buildIndexWith(impl, [FIXTURE_ROOT_CONFIG, FIXTURE_SIBLING_CONFIG, FIXTURE_SCRIPT]);
  const relativeToken = "./🧪️bare-reference-sibling-fixture.config.ts";
  expect(impl.resolveReferencePath(FIXTURE_SCRIPT, relativeToken, index)).toBe(FIXTURE_SIBLING_CONFIG);
});

for (const compiler of compilers) test(compiler.name + " leaves a multi-segment bare token's root-first resolution unchanged even when a same-named sibling exists", () => {
  const impl = implementation(compiler);
  const multiSegmentToken = `${FIXTURE_MODULE}/🎈️only-at-repo-root.config.ts`;
  const decoySibling = `${FIXTURE_MODULE}/📦️packages/🟦️typescript/${multiSegmentToken}`;
  const index = buildIndexWith(impl, [FIXTURE_ROOT_SUBDIR, decoySibling, FIXTURE_SCRIPT]);
  expect(impl.resolveReferencePath(FIXTURE_SCRIPT, multiSegmentToken, index)).toBe(FIXTURE_ROOT_SUBDIR);
});

test("both independent compilers agree on the exact sibling-precedence resolution", () => {
  const [bun, typescript] = compilers.map((compiler) => {
    const impl = implementation(compiler);
    return impl.resolveReferencePath(FIXTURE_SCRIPT, FIXTURE_ROOT_CONFIG, buildIndexWith(impl, [FIXTURE_ROOT_CONFIG, FIXTURE_SIBLING_CONFIG, FIXTURE_SCRIPT]));
  });
  expect(bun).toEqual(typescript);
  expect(bun).toBe(FIXTURE_SIBLING_CONFIG);
});
