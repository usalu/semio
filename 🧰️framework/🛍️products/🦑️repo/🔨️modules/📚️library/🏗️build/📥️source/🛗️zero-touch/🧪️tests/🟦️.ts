import { expect, test } from "bun:test";
import { posix } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv/dist/2020";
import ts from "typescript";
import caseSchema from "../../../🧬️schema/🛗️zero-touch-closure.json" with { type: "json" };
import rosterSchema from "../🗂️closures/🧬️schema/🔣️.json" with { type: "json" };
import { checkZeroTouchClosuresV1, diskZeroTouchTreeV1, readZeroTouchRosterV1, scanModuleSpecifiersV1, workspaceSpecifierTargetV1, zeroTouchClosureFindingsV1, ZERO_TOUCH_CLOSURES_FILE, type ZeroTouchImportKindV1, type ZeroTouchTreeV1 } from "../🟦️.ts";
import fixture from "./🔣️.json" with { type: "json" };

const repoRoot = fileURLToPath(new URL("../../../../../../../../../", import.meta.url));
const treeOf = (files: Readonly<Record<string, string>>): ZeroTouchTreeV1 => ({ packages: fixture.packages, read: (path) => files[path] });
const subject = (finding: { closure: string; kind: string; file: string; specifier: string }): string => `${finding.closure}|${finding.kind}|${finding.file}|${finding.specifier}`;

test("the fixture cases and the roster satisfy their schemas", () => {
  const ajv = new Ajv({ strict: false }), cases = ajv.compile(caseSchema), roster = ajv.compile(rosterSchema);
  expect(cases(fixture), JSON.stringify(cases.errors)).toBe(true);
  for (const row of fixture.cases) expect(roster(row.roster), `${row.name}: ${JSON.stringify(roster.errors)}`).toBe(true);
  const real = JSON.parse(diskZeroTouchTreeV1(repoRoot).read(ZERO_TOUCH_CLOSURES_FILE)!);
  expect(roster(real), JSON.stringify(roster.errors)).toBe(true);
  expect(readZeroTouchRosterV1(JSON.stringify(real)).closures.length).toBe(real.closures.length);
});

test("every declared bootstrap source has a full zero-touch closure", () => {
 const tree=diskZeroTouchTreeV1(repoRoot),roster=readZeroTouchRosterV1(tree.read(ZERO_TOUCH_CLOSURES_FILE)!),declaration=JSON.parse(tree.read("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/🌱️sources/🔣️.json")!);
 for(const source of declaration.sources)expect(roster.closures.some(closure=>closure.entry===source.module&&closure.mode==="full"),source.id).toBe(true);
});

test("every native repository recovery owner has a full zero-touch closure", () => {
 const roster=readZeroTouchRosterV1(diskZeroTouchTreeV1(repoRoot).read(ZERO_TOUCH_CLOSURES_FILE)!);
 for(const entry of ["🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/📜️script.ts","🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🟦️.ts","🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🐹️go-plan/🟦️.ts"])expect(roster.closures.some(closure=>closure.entry===entry&&closure.mode==="full"),entry).toBe(true);
});

type CaseRow = Readonly<{ name: string; roster: unknown; files: Readonly<Record<string, string>>; closureFiles: Readonly<Record<string, number>>; findings: readonly string[]; details?: Readonly<Record<string, string>> }>;

for (const row of fixture.cases as unknown as readonly CaseRow[]) {
  test(`zero-touch closure: ${row.name}`, () => {
    const result = zeroTouchClosureFindingsV1(readZeroTouchRosterV1(JSON.stringify(row.roster)), treeOf(row.files));
    expect(result.findings.map(subject).sort()).toEqual([...row.findings].sort());
    expect(result.files).toEqual(row.closureFiles);
    for (const [key, detail] of Object.entries(row.details ?? {})) expect(result.findings.find((finding) => subject(finding) === key)?.detail).toBe(detail);
  });
}

for (const row of fixture.scans) {
  test(`module specifier scan: ${row.name}`, () => {
    expect(scanModuleSpecifiersV1(row.path, row.text).map((entry) => [entry.specifier, entry.kind]).sort()).toEqual([...row.imports].sort());
  });
}

test("a closed roster refuses unknown shapes", () => {
  expect(() => readZeroTouchRosterV1(JSON.stringify({ version: 1, closures: [], seams: [] }))).toThrow("Closed zero-touch closure roster required");
  expect(() => readZeroTouchRosterV1(JSON.stringify({ version: 1, closures: [{ id: "a", entry: "a.ts", mode: "other", reason: "x" }], seams: [] }))).toThrow("Closed zero-touch closure row required");
  expect(() => readZeroTouchRosterV1(JSON.stringify({ version: 1, closures: [{ id: "a", entry: "a.ts", mode: "full", reason: "x" }], seams: [{ module: "m.ts", reason: "x", extra: 1 }] }))).toThrow("Closed zero-touch seam row required");
});

test("workspace specifiers resolve like the Node package exports algorithm", () => {
  const nodeResolved = (specifier: string): string | undefined => {
    const name = specifier.split("/").slice(0, 2).join("/"), pkg = fixture.packages[name as keyof typeof fixture.packages];
    const key = `.${specifier.slice(name.length)}`, exports = pkg.exports as Record<string, unknown>;
    const direct = exports[key] ?? Object.entries(exports).find(([pattern]) => pattern.includes("*") && key.startsWith(pattern.split("*")[0]!))?.[1];
    const target = typeof direct === "string" ? direct.replace("*", key.slice(key.lastIndexOf("/") + 1)) : (direct as { bun?: string } | undefined)?.bun;
    return target === undefined ? undefined : `${pkg.directory}/${target.replace(/^\.\//u, "")}`;
  };
  for (const specifier of ["@scope/lib", "@scope/lib/sub", "@scope/lib/feature/alpha", "@scope/lib/package.json"]) expect(workspaceSpecifierTargetV1(fixture.packages, specifier), specifier).toBe(nodeResolved(specifier));
  expect(workspaceSpecifierTargetV1(fixture.packages, "@scope/lib/missing")).toBeUndefined();
  expect(workspaceSpecifierTargetV1(fixture.packages, "@other/lib")).toBeUndefined();
});

const compilerImports = (path: string, text: string): string[] => {
  const source = ts.createSourceFile(path, text, ts.ScriptTarget.Latest, true, /\.[cm]?tsx?$/u.test(path) ? ts.ScriptKind.TS : ts.ScriptKind.JS), rows = new Set<string>();
  const add = (specifier: ts.Node | undefined, kind: ZeroTouchImportKindV1): void => { if (specifier && ts.isStringLiteralLike(specifier)) rows.add(`${kind}\0${specifier.text}`); };
  const typeOnly = (clause: ts.ImportClause | undefined): boolean => clause !== undefined && (clause.isTypeOnly || (clause.name === undefined && clause.namedBindings !== undefined && ts.isNamedImports(clause.namedBindings) && clause.namedBindings.elements.length > 0 && clause.namedBindings.elements.every((element) => element.isTypeOnly)));
  const visit = (node: ts.Node): void => {
    if (ts.isImportDeclaration(node) && !typeOnly(node.importClause)) add(node.moduleSpecifier, "static");
    else if (ts.isExportDeclaration(node) && !node.isTypeOnly) add(node.moduleSpecifier, "static");
    else if (ts.isCallExpression(node) && node.arguments.length === 1) {
      if (node.expression.kind === ts.SyntaxKind.ImportKeyword) add(node.arguments[0], "dynamic");
      else if (ts.isIdentifier(node.expression) && node.expression.text === "require") add(node.arguments[0], "require");
    }
    ts.forEachChild(node, visit);
  };
  visit(source);
  return [...rows].sort();
};

test("the system compiler scan agrees with the TypeScript compiler AST on every file of the zero-touch closures", () => {
  const tree = diskZeroTouchTreeV1(repoRoot), roster = readZeroTouchRosterV1(tree.read(ZERO_TOUCH_CLOSURES_FILE)!);
  const files = new Set<string>(), pending = roster.closures.map((closure) => closure.entry);
  while (pending.length) {
    const file = pending.pop()!;
    if (files.has(file)) continue;
    files.add(file);
    const text = tree.read(file);
    if (text === undefined) continue;
    for (const { specifier } of scanModuleSpecifiersV1(file, text)) {
      if (!specifier.startsWith(".")) continue;
      const base = file.slice(0, file.lastIndexOf("/") + 1) + specifier.split("?")[0]!, normalized = posix.normalize(base);
      if (/\.(?:[cm]?[jt]s|tsx)$/u.test(normalized) && tree.read(normalized) !== undefined) pending.push(normalized);
    }
  }
  expect(files.size).toBeGreaterThan(300);
  const differences: string[] = [];
  for (const file of files) {
    const text = tree.read(file);
    if (text === undefined) continue;
    const ours = scanModuleSpecifiersV1(file, text).map((entry) => `${entry.kind}\0${entry.specifier}`).sort(), oracle = compilerImports(file, text);
    if (JSON.stringify(ours) !== JSON.stringify(oracle)) differences.push(`${file}: ours-only ${ours.filter((row) => !oracle.includes(row)).join(",")} oracle-only ${oracle.filter((row) => !ours.includes(row)).join(",")}`);
  }
  expect(differences).toEqual([]);
}, 120_000);

test("the real checkout has no import that leaves a zero-touch closure", () => {
  const result = checkZeroTouchClosuresV1(repoRoot);
  expect(result.findings.map(subject)).toEqual([]);
  expect(Object.keys(result.files).sort()).toEqual(readZeroTouchRosterV1(diskZeroTouchTreeV1(repoRoot).read(ZERO_TOUCH_CLOSURES_FILE)!).closures.map((closure) => closure.id).sort());
}, 120_000);
