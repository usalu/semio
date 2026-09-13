import { expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import Ajv from "ajv";
import ts from "typescript";
import { loadTaxonomy, semanticDirectoryKindId } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";

const repoRoot = resolve(import.meta.dir, "../../../../../../../");
const domainRoot = resolve(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test");
const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧫️fixtures/🧱️command-composition-source/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧬️schema/🧱️command-composition-source/🔣️.json"), "utf8"));

function namedDeclarations(path: string): string[] {
  const source = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  return source.statements
    .flatMap((statement) => {
      if ((ts.isFunctionDeclaration(statement) || ts.isTypeAliasDeclaration(statement) || ts.isInterfaceDeclaration(statement) || ts.isClassDeclaration(statement)) && statement.name) return [statement.name.text];
      if (ts.isVariableStatement(statement)) return statement.declarationList.declarations.flatMap((declaration) => (ts.isIdentifier(declaration.name) ? [declaration.name.text] : []));
      return [];
    })
    .sort();
}

function relativeSpecifier(consumer: string, owner: string): string {
  const path = relative(resolve(repoRoot, dirname(consumer)), resolve(domainRoot, owner)).replaceAll("\\", "/");
  return path.startsWith(".") ? path : `./${path}`;
}

test("validates the portable command-composition ownership contract", () => {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  expect(fixture.owners).toHaveLength(16);
  expect(new Set(fixture.owners.map((owner: { path: string }) => owner.path)).size).toBe(16);
});

test("resolves every anonymous owner and semantic context", () => {
  const taxonomy = loadTaxonomy();
  for (const context of fixture.contexts) expect(semanticDirectoryKindId(context.directoryName, taxonomy, { parentKindId: context.parentKindId }), JSON.stringify(context)).toBe(context.kindId);
  for (const owner of fixture.owners) {
    expect(owner.path.split("/").at(-1)).toBe("🟦️.ts");
    const path = resolve(repoRoot, owner.path);
    expect(existsSync(path), owner.path).toBe(true);
    if (existsSync(path)) expect(namedDeclarations(path)).toEqual([...owner.declarations].sort());
  }
});

test("typechecks an acyclic owner graph with no command back edge", { timeout: 30_000 }, () => {
  const paths = fixture.owners.map((owner: { path: string }) => resolve(repoRoot, owner.path));
  const present = paths.filter(existsSync);
  expect(present).toHaveLength(paths.length);
  if (present.length !== paths.length) return;
  const program = ts.createProgram(paths, {
    target: ts.ScriptTarget.ESNext,
    module: ts.ModuleKind.ESNext,
    moduleResolution: ts.ModuleResolutionKind.Bundler,
    strict: true,
    noUncheckedIndexedAccess: true,
    allowImportingTsExtensions: true,
    allowJs: true,
    skipLibCheck: true,
    noEmit: true,
    types: ["node"],
  });
  expect(
    paths.flatMap((path: string) => [...program.getSyntacticDiagnostics(program.getSourceFile(path)), ...program.getSemanticDiagnostics(program.getSourceFile(path))]).map((diagnostic) => ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n")),
  ).toEqual([]);
  const owners = new Set(paths),
    edges = new Map<string, string[]>(paths.map((path: string) => [path, []]));
  for (const owner of paths) {
    const syntax = ts.createSourceFile(owner, readFileSync(owner, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    for (const statement of syntax.statements) {
      const specifier = (ts.isImportDeclaration(statement) || ts.isExportDeclaration(statement)) && statement.moduleSpecifier && ts.isStringLiteral(statement.moduleSpecifier) ? statement.moduleSpecifier.text : null;
      if (!specifier?.startsWith(".")) continue;
      const target = resolve(dirname(owner), specifier);
      expect(target).not.toBe(resolve(domainRoot, "📜️script.ts"));
      if (owners.has(target)) edges.get(owner)!.push(target);
    }
  }
  const seen = new Set<string>(),
    active = new Set<string>();
  const visit = (owner: string): void => {
    if (active.has(owner)) throw new Error(`repo-test owner cycle at ${owner}`);
    if (seen.has(owner)) return;
    active.add(owner);
    for (const target of edges.get(owner)!) visit(target);
    active.delete(owner);
    seen.add(owner);
  };
  for (const owner of paths) visit(owner);
  expect(seen.size).toBe(paths.length);
});

test("moves semantic bodies out of the command module and binds real consumers", () => {
  const command = resolve(domainRoot, "📜️script.ts"),
    moved = new Set(fixture.owners.flatMap((owner: { declarations: string[] }) => owner.declarations));
  expect(namedDeclarations(command).filter((name) => moved.has(name))).toEqual([]);
  const commandText = readFileSync(command, "utf8");
  expect(commandText).not.toContain("function executeOne");
  expect(commandText).not.toContain("function materializeRustHost");
  expect(commandText).not.toContain("class FixtureScript extends");
  expect(commandText).toContain('export { policy } from "./⚖️policy/🧹️domain/🟦️.ts"');
  for (const consumer of fixture.consumers) {
    const source = readFileSync(resolve(repoRoot, consumer.path), "utf8");
    for (const owner of consumer.owners) expect(source.includes(relativeSpecifier(consumer.path, owner)) || source.includes(owner), `${consumer.path} -> ${owner}`).toBe(true);
  }
});

test("the semantic policy owner admits every extracted concern", async () => {
  const owner = await import("../../⚖️policy/🧹️domain/🟦️.ts");
  const scopes = new Set(owner.policy().map((breach) => breach.scope));
  for (const directory of fixture.owners.map((entry: { path: string }) => relative(domainRoot, resolve(repoRoot, entry.path)).split("/")[0])) expect(scopes.has(`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/${directory}`), directory).toBe(false);
});

test("keeps selector behavior implementation-neutral", async () => {
  const api = await import("../../🔍️discovery/🎛️selection/🟦️.ts");
  const selectors = api.readSelectors(["--artifact", "jack", "--standard", "1", "--fixture-class", "canonical", "--status", "passed"]);
  expect(selectors).toEqual(expect.objectContaining({ artifact: "jack", standard: "1", fixtureClass: "canonical", status: "passed", implementation: null }));
  expect(api.matchesTarget({ artifact: "plugin.jack", standard: "1", subset: "any", mutations: [{ id: "move", subset: "any" }] }, selectors)).toBe(true);
  expect(api.matchesFixture({ target: { artifact: "plugin.jack", standard: "1", subset: "any" }, class: "canonical" }, selectors)).toBe(true);
  expect(api.matchesRow({ artifact: "plugin.jack", standard: "1", subset: "any", fixtureClass: "canonical", status: "passed" }, selectors)).toBe(true);
});

test("preserves native host parsing and separates build and scenario budgets", async () => {
  const tsApi = await import("typescript"),
    hostPath = resolve(domainRoot, "🖥️host/🏗️materialization/🟦️.ts"),
    executionPath = resolve(domainRoot, "🏃️execution/🎬️scenario/🟦️.ts");
  const hostSource = tsApi.createSourceFile(hostPath, readFileSync(hostPath, "utf8"), tsApi.ScriptTarget.Latest, true, tsApi.ScriptKind.TS);
  const parser = hostSource.statements.find((node) => tsApi.isFunctionDeclaration(node) && node.name?.text === "rustHostExecutableFromCargo");
  expect(parser).toBeDefined();
  const compiled = tsApi.transpileModule(parser!.getText(hostSource).replace(/^export\s+/u, ""), { compilerOptions: { target: tsApi.ScriptTarget.ES2022, module: tsApi.ModuleKind.None } }).outputText;
  const resolveExecutable = new Function(`${compiled}\nreturn rustHostExecutableFromCargo;`)() as (stdout: string) => string | null;
  expect(
    resolveExecutable(
      [JSON.stringify({ reason: "compiler-artifact", target: { name: "dependency", kind: ["lib"] } }), JSON.stringify({ reason: "compiler-artifact", target: { name: "host", kind: ["bin"] }, executable: "/portable/target/host" })].join("\n"),
    ),
  ).toBe("/portable/target/host");
  const execution = readFileSync(executionPath, "utf8");
  expect(execution).toContain("budgetMs: buildBudgetMs()");
  expect(execution).toContain("budgetMs: testLevelBudgetMs(level)");
  expect(execution.indexOf("budgetMs: buildBudgetMs()")).toBeLessThan(execution.indexOf("budgetMs: testLevelBudgetMs(level)"));
  expect(execution).toContain("host preparation emitted no executable");
  expect(execution).toContain("rmSync(plan.resultsPath");
  expect(execution.indexOf("rmSync(plan.resultsPath")).toBeLessThan(execution.indexOf("materializeHost(repoRoot"));
});

test("prioritizes each declared Python source over the anonymous host leaf", async () => {
  const control = fixture.nativePythonImport;
  const materialization = await import("../../🖥️host/🏗️materialization/🟦️.ts");
  const hostPath = resolve(domainRoot, "🖥️host/🐍️.py");
  const sourcePath = resolve(domainRoot, control.sourcePath);
  expect(materialization.pythonHostArguments(hostPath, "/plan.json", "/results.jsonl", "/adapter.py", [dirname(sourcePath)])).toEqual([
    hostPath,
    control.argument,
    dirname(sourcePath),
    "--plan",
    "/plan.json",
    "--out",
    "/results.jsonl",
    "--adapter",
    "/adapter.py",
  ]);
  const program = [
    "import importlib, importlib.util, os, sys",
    "host_path, source_dir, module_name, marker = sys.argv[1:]",
    "sys.path.insert(0, os.path.dirname(host_path))",
    "spec = importlib.util.spec_from_file_location('semio_test_host_collision', host_path)",
    "host = importlib.util.module_from_spec(spec)",
    "spec.loader.exec_module(host)",
    "host._prioritize_local_source_paths([source_dir])",
    "loaded = importlib.import_module(module_name)",
    "assert getattr(loaded, 'SOURCE_MARKER', None) == marker, getattr(loaded, '__file__', None)",
    "print(loaded.__file__)",
  ].join("\n");
  const probe = Bun.spawnSync([materialization.oracleHostPython(repoRoot), "-c", program, hostPath, dirname(sourcePath), control.module, control.marker], {
    cwd: repoRoot,
    env: { ...process.env, PYTHONDONTWRITEBYTECODE: "1" },
    stdout: "pipe",
    stderr: "pipe",
  });
  expect(probe.exitCode, Buffer.from(probe.stderr).toString()).toBe(0);
  expect(Buffer.from(probe.stdout).toString().trim()).toBe(sourcePath);
});

test("keeps fixture provenance output isolated from committed expectations", () => {
  const source = readFileSync(resolve(domainRoot, "🧾️provenance/📋️orchestration/🟦️.ts"), "utf8");
  expect(source).toContain('testCacheDir(this.repoRoot, "work")');
  expect(source).toContain("rmSync(outDir, { recursive: true, force: true })");
  expect(source).toContain("publishFixtureManifest");
  expect(source).toContain("installFixtureFile");
  expect(source).not.toMatch(/writeFileSync\([^\n]*fixture\.(?:before|after)/);
});

test("registers the ordinary Bun Nx launch route without losing HTML source inputs", () => {
  const project = JSON.parse(readFileSync(resolve(domainRoot, "📋️project.json"), "utf8"));
  expect(project.targets[fixture.route.target]?.options.command).toBe(fixture.route.command);
  expect(project.targets[fixture.route.target]?.inputs).toEqual([fixture.route.namedInput]);
  const inputs = new Set(project.namedInputs[fixture.route.namedInput]);
  for (const owner of fixture.owners) expect(inputs.has(`{projectRoot}/${relative(domainRoot, resolve(repoRoot, owner.path)).replaceAll("\\", "/")}`), owner.path).toBe(true);
  for (const path of [
    "{projectRoot}/📋️project.json",
    "{projectRoot}/📜️script.ts",
    "{projectRoot}/🧪️tests/🧱️command-composition-source/🟦️.ts",
    "{projectRoot}/🧬️schema/🧱️command-composition-source/🔣️.json",
    "{projectRoot}/🧫️fixtures/🧱️command-composition-source/🔣️.json",
    "{projectRoot}/🧫️fixtures/🧱️command-composition-source/🐍️.py",
    "{projectRoot}/🧪️tests/🧪️test-platform/🟦️.ts",
    "{projectRoot}/🖥️host/🐍️.py",
    "{workspaceRoot}/.vscode/🧩️launch.seed.jsonc",
    "{workspaceRoot}/.vscode/launch.json",
  ])
    expect(inputs.has(path), path).toBe(true);
  expect(project.targets["test-fixture-verify"].inputs).toContain("htmlSourcePairs");
  expect(project.namedInputs.htmlSourcePairs).toHaveLength(17);
  for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
    const source = readFileSync(resolve(repoRoot, path), "utf8");
    expect(source.split(fixture.route.launchName).length - 1).toBe(1);
    expect(source).toContain(fixture.route.launchCommand);
  }
});
