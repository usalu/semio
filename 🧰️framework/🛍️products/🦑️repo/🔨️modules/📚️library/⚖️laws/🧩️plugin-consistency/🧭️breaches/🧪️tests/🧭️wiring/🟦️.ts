/** 🧭️ The aggregate breach list calls exactly the laws and the loop-scoped crate laws its language-agnostic corpus pins, in order, each one a distinct exported function of its import source, as read by an independent Python parse and the TypeScript compiler API. */
import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { copyFileSync, mkdirSync, mkdtempSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import Ajv from "ajv";
import { testPythonV1 } from "../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import { gitSpawnEnv } from "../../../../../🏃️process/🌿️environment/🌳️git/🟦️.ts";
import { readFileSync } from "node:fs";
import ts from "typescript";
import corpus from "./🔣️.json" with { type: "json" };
import corpusSchema from "./🧬️schema/🔣️.json" with { type: "json" };

const unitSource = ts.createSourceFile("breaches.ts", readFileSync(resolve(import.meta.dir, "../../🟦️.ts"), "utf8"), ts.ScriptTarget.Latest, true);
const compilerWiring = () => {
  const rows: { name: string; args: string; loop: boolean }[] = [];
  const visit = (node: ts.Node, inLoop: boolean): void => {
    if (ts.isCallExpression(node) && ts.isPropertyAccessExpression(node.expression) && node.expression.name.text === "push" && node.expression.expression.getText(unitSource) === "breaches") {
      const arg = node.arguments[0];
      if (arg && ts.isSpreadElement(arg) && ts.isCallExpression(arg.expression)) rows.push({ name: arg.expression.expression.getText(unitSource), args: arg.expression.arguments.map((a) => a.getText(unitSource)).join(", "), loop: inLoop });
    }
    ts.forEachChild(node, (child) => visit(child, inLoop || ts.isForOfStatement(node)));
  };
  visit(unitSource, false);
  return rows;
};
const compilerImports = () => {
  const out: Record<string, string> = {};
  unitSource.forEachChild((node) => {
    if (!ts.isImportDeclaration(node) || !node.importClause?.namedBindings || !ts.isNamedImports(node.importClause.namedBindings) || node.importClause.isTypeOnly) return;
    for (const element of node.importClause.namedBindings.elements) if (!element.isTypeOnly) out[element.name.text] = (node.moduleSpecifier as ts.StringLiteral).text;
  });
  return out;
};
const rules: Record<string, (root: string, ...args: any[]) => unknown> = {
  wiring: () => compilerWiring(),
  imports: () => compilerImports(),
};
const artifacts = realpathSync(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR || tmpdir()));
const libraryRoot = resolve(import.meta.dir, "../../../../.."), libraryRel = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const canon = (value: unknown): unknown => (value === undefined ? null : JSON.parse(JSON.stringify(value)));
const admit = (root: string) => {
  for (const rel of ["🔣️taxonomy.json", "🧬️schema/🔣️.json"]) { mkdirSync(dirname(join(root, libraryRel, rel)), { recursive: true }); copyFileSync(join(libraryRoot, rel), join(root, libraryRel, rel)); }
  for (const args of [["init", "--quiet"], ["add", "-A"]]) expect(spawnSync("git", args, { cwd: root, env: gitSpawnEnv(), encoding: "utf8" }).status).toBe(0);
};
const run = (row: any) => {
  const root = mkdtempSync(join(artifacts, "lawcov-"));
  try {
    for (const [path, lines] of Object.entries((row.files ?? {}) as Record<string, string[]>)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), lines.length ? lines.join("\n") + "\n" : ""); }
    if (row.git) admit(root);
    return canon(rules[row.rule]!(root, ...(row.args ?? [])));
  } finally { rmSync(root, { recursive: true, force: true }); }
};
const modelled = (corpus.cases as any[]).filter((row) => row.oracle !== false);
const model = (() => {
  const run = spawnSync(testPythonV1() ?? "python3", ["-I", resolve(import.meta.dir, "./🐍️oracle.py"), resolve(import.meta.dir, "./🔣️.json")], { encoding: "utf8" });
  expect(run.status, run.stderr).toBe(0);
  return JSON.parse(run.stdout) as Record<string, unknown>;
})();

test("the corpus satisfies the strict independent validator and names every case once", () => {
  const ajv = new Ajv({ strict: true, allErrors: true });
  expect(ajv.validate(corpusSchema, corpus), JSON.stringify(ajv.errors)).toBe(true);
  expect(new Set(corpus.cases.map((row) => row.id)).size).toBe(corpus.cases.length);
  expect(Object.keys(model).sort()).toEqual(modelled.map((row) => row.id).sort());
});

for (const row of corpus.cases as any[]) {
  (row.defect ? test.failing : test)(`${row.rule}: ${row.id}`, () => {
    expect(run(row), "law versus corpus").toEqual(row.expect);
    if (row.oracle !== false) expect(model[row.id], "oracle versus corpus").toEqual(row.expect);
  }, row.git ? 120000 : 30000);
}

test("every wired law is a distinct exported function of the module it is imported from", async () => {
  const names = (corpus.cases[0] as any).expect.map((row: any) => row.name);
  expect(new Set(names).size).toBe(names.length);
  const sources = (corpus.cases[1] as any).expect as Record<string, string>;
  for (const name of names) {
    expect(sources[name], `${name} is imported`).toBeTruthy();
    const loaded = (await import(resolve(import.meta.dir, "../..", sources[name]!))) as Record<string, unknown>;
    expect(typeof loaded[name], `${name} is exported as a function`).toBe("function");
  }
});
