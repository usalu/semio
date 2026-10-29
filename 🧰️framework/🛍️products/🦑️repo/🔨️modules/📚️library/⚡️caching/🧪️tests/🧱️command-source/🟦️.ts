import { expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import Ajv from "ajv";
import ts from "typescript";
import schema from "./🧬️schema/🔣️.json";
import { getWorkspaceRoot } from "../../../🗂️workspaces/🟦️.ts";
import { loadTaxonomy, semanticDirectoryKindId } from "../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";

const repoRoot = getWorkspaceRoot();
const domainRoot = resolve(import.meta.dir, "../..");
const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "../🧫️fixtures/🧱️command-source/🔣️.json"), "utf8"));


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

test("validates the language-neutral cache command source contract", () => {
  
  const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);
  expect(fixture.owners).toHaveLength(13);
  expect(new Set(fixture.owners.map((owner: { path: string }) => owner.path)).size).toBe(13);
});

test("resolves every anonymous owner and cache semantic context", () => {
  const taxonomy = loadTaxonomy();
  for (const context of fixture.contexts) expect(semanticDirectoryKindId(context.directoryName, taxonomy, { parentKindId: context.parentKindId }), JSON.stringify(context)).toBe(context.kindId);
  for (const owner of fixture.owners) {
    expect(owner.path.split("/").at(-1)).toBe("🟦️.ts");
    const path = resolve(repoRoot, owner.path);
    expect(existsSync(path), owner.path).toBe(true);
    if (existsSync(path)) expect(namedDeclarations(path)).toEqual([...owner.declarations].sort());
  }
});

test("typechecks an acyclic cache owner graph with no command back edge", () => {
  const paths: string[] = fixture.owners.map((owner: { path: string }) => resolve(repoRoot, owner.path));
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
    if (active.has(owner)) throw new Error(`cache owner cycle at ${owner}`);
    if (seen.has(owner)) return;
    active.add(owner);
    for (const target of edges.get(owner)!) visit(target);
    active.delete(owner);
    seen.add(owner);
  };
  for (const owner of paths) visit(owner);
  expect(seen.size).toBe(paths.length);
}, { timeout: 30_000 });

test("moves semantic bodies out of the router and binds exact consumers", () => {
  const commandPath = resolve(domainRoot, "📜️script.ts"),
    moved = new Set(fixture.owners.flatMap((owner: { declarations: string[] }) => owner.declarations));
  expect(namedDeclarations(commandPath).filter((name) => moved.has(name))).toEqual([]);
  const command = readFileSync(commandPath, "utf8");
  for (const body of ["function sourceFiles", "function scanCacheAreas", "function artifactPackageInventory", "class CachePruneScript extends", "class CacheVerifyScript extends"]) expect(command).not.toContain(body);
  for (const consumer of fixture.consumers) {
    const source = readFileSync(resolve(repoRoot, consumer.path), "utf8");
    for (const owner of consumer.owners) expect(source.includes(relativeSpecifier(consumer.path, owner)) || source.includes(owner), `${consumer.path} -> ${owner}`).toBe(true);
  }
});

test("contains diagnostic output through an injected ticket authority", async () => {
  const { ticketOutput } = await import("../../🎫️output/🟦️.ts");
  const made: string[] = [];
  const operations = {
    realpath: (path: string) => resolve(path),
    exists: (path: string) => path.endsWith("/🎫️ticket.json"),
    makeDirectory: (path: string) => made.push(path),
  };
  const selected = ticketOutput("/repo", ["--ticket", ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/ACTIVE"], {}, operations);
  expect(selected).toBe("/repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/ACTIVE/🗑️generated/nx");
  expect(made).toEqual([selected]);
  expect(() => ticketOutput("/repo", ["--ticket", "../outside"], {}, operations)).toThrow(/existing repository ticket/);
  expect(() => ticketOutput("/repo", [], {}, operations)).toThrow(/Select the active ticket/);
});

test("walks admitted source entries once without following symlinks or generated stores", async () => {
  const { CACHE_POLICY, sourceFiles } = await import("../../🔍️discovery/📂️source/🟦️.ts");
  const generated = CACHE_POLICY.generatedDirectories[0];
  const tree: Record<string, any[]> = {
    "/repo": [
      { name: "b.ts", kind: "file" },
      { name: "src", kind: "directory" },
      { name: "linked", kind: "symlink" },
      { name: generated, kind: "directory" },
      { name: ".🧬semio", kind: "directory" },
    ],
    "/repo/src": [{ name: "a.ts", kind: "file" }],
  };
  const reads: string[] = [];
  const files = sourceFiles("/repo", {
    readDirectory(path: string) {
      reads.push(path);
      return (tree[path] ?? []).map((entry) => ({
        name: entry.name,
        isDirectory: () => entry.kind === "directory",
        isFile: () => entry.kind === "file",
        isSymbolicLink: () => entry.kind === "symlink",
      }));
    },
  });
  expect(files).toEqual(["b.ts", "src/a.ts"]);
  expect(reads).toEqual(["/repo", "/repo/src"]);
});

test("composes target inventory, artifact ownership and dependency consumers from portable facts", async () => {
  const { composeInventory } = await import("../../📇️inventory/🧮️composition/🟦️.ts");
  const result = composeInventory("/repo", fixture.inventory.files, { projects: fixture.inventory.projects, sources: fixture.inventory.sources }, { readText: (path: string) => fixture.inventory.texts[relative("/repo", path).replaceAll("\\", "/")] });
  expect(result.commands).toHaveLength(fixture.inventory.expected.commands);
  const produced = result.artifacts.filter((entry: { owner: string }) => entry.owner === "producer:build");
  expect(produced).toHaveLength(fixture.inventory.expected.artifacts);
  expect(produced[0].consumers).toContain(fixture.inventory.expected.consumer);
  expect([...new Set(result.violations.map((finding: { rule: string }) => finding.rule))].sort()).toEqual([...fixture.inventory.expected.violationRules].sort());
});

test("projects cache areas through injected scanners and preserves cancellation", async () => {
  const { scanCacheAreas, areaUnitRoot } = await import("../../🧹️pruning/🌐️workspace/🟦️.ts");
  const {phaseCompilerAdmissionControlV1}=await import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts");
  const control=(signal:AbortSignal)=>phaseCompilerAdmissionControlV1(signal,30000,()=>undefined,()=>new Promise(done=>setImmediate(done)));
  const unit = { path: "unit", bytes: 4, recencyMs: 1, lockHeld: false, kind: "directory" as const };
  const operations = {
    cargoDirectories: () => ({ build: "/cache/build", target: "/cache/target" }),
    cacheDirectory: (_root: string, area: string) => `/cache/${area}`,
    scanCargoBuild: async (_path: string, original:ReturnType<typeof control>) => (original.signal.throwIfAborted(), [unit]),
    scanCargoTarget: async (_path: string, original:ReturnType<typeof control>) => (original.signal.throwIfAborted(), [unit]),
    scanCargoIncremental: async (_path: string, original:ReturnType<typeof control>) => (original.signal.throwIfAborted(), { units: [unit], staleSessions: [unit] }),
    scanDirectories: async (_path: string, original:ReturnType<typeof control>) => (original.signal.throwIfAborted(), [unit]),
  };
  const areas = await scanCacheAreas("/repo", control(new AbortController().signal), undefined, operations);
  expect(areas.map((area: { name: string }) => area.name)).toEqual(["cargo", "cargo-incremental", "cargo-incremental-sessions", "vite", "agents"]);
  expect(areaUnitRoot("/repo", "cargo", { ...unit, kind: "cargo-build" }, operations)).toBe("/cache/build");
  expect(areaUnitRoot("/repo", "cargo", { ...unit, kind: "cargo-target-file" }, operations)).toBe("/cache/target");
  const cancelled = new AbortController();
  cancelled.abort(new Error("portable cancellation"));
  await expect(scanCacheAreas("/repo", control(cancelled.signal), undefined, operations)).rejects.toThrow(/portable cancellation/);
});

test("captures bounded child output and terminates a timed-out process tree", async () => {
  const { captureArtifactContract } = await import("../../📦️artifacts/🏃️contract-capture/🟦️.ts");
  expect(await captureArtifactContract(process.execPath, ["-e", "process.stdout.write('bounded')"], repoRoot, 5_000)).toBe("bounded");
  await expect(captureArtifactContract(process.execPath, ["-e", "setTimeout(() => {}, 10_000)"], repoRoot, 50)).rejects.toThrow(/timeout 50ms/);
  const parent = ["const {spawn}=require('node:child_process')", "const child=spawn(process.execPath,['-e','setTimeout(() => {}, 10000)'],{stdio:'ignore'})", "console.log('descendant='+child.pid)", "setTimeout(() => {}, 10000)"].join(";");
  let descendant = 0;
  try {
    await captureArtifactContract(process.execPath, ["-e", parent], repoRoot, 2_000);
  } catch (error) {
    descendant = Number(String(error).match(/descendant=(\d+)/)?.[1]);
  }
  expect(descendant).toBeGreaterThan(0);
  const alive = (): boolean => {
    try {
      process.kill(descendant, 0);
      return true;
    } catch {
      return false;
    }
  };
  for (let attempt = 0; attempt < 20 && alive(); attempt++) await new Promise((accept) => setTimeout(accept, 25));
  try {
    expect(alive()).toBe(false);
  } finally {
    if (alive()) process.kill(descendant, "SIGKILL");
  }
  expect(readFileSync(resolve(domainRoot, "📦️artifacts/🏃️contract-capture/🟦️.ts"), "utf8")).toContain('["taskkill", "/pid", String(child.pid), "/t", "/f"]');
}, { timeout: 10_000 });

test("keeps policy and bootstrap coordinates and invokes the direct prune owner", async () => {
  const { CACHE_BOOTSTRAP_EXECUTABLE } = await import("../../📇️inventory/🧮️composition/🟦️.ts");
  const source = await import("../../🔍️discovery/📂️source/🟦️.ts");
  expect(resolve(domainRoot, fixture.coordinates.policy)).toBe(source.CACHE_POLICY_PATH);
  expect(resolve(domainRoot, fixture.coordinates.bootstrap)).toBe(CACHE_BOOTSTRAP_EXECUTABLE);
  const path = resolve(repoRoot, fixture.coordinates.rootPruneConsumer), syntax = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true);
  const imports = syntax.statements.filter(ts.isImportDeclaration), owner = resolve(domainRoot, "🧹️pruning/📋️orchestration/🟦️.ts");
  const bindings = imports.flatMap(row => {
    const { importClause, moduleSpecifier } = row;
    if (!importClause || importClause.isTypeOnly || !importClause.namedBindings || !ts.isNamedImports(importClause.namedBindings) || !ts.isStringLiteral(moduleSpecifier)) return [];
    return importClause.namedBindings.elements.filter(binding => !binding.isTypeOnly && (binding.propertyName ?? binding.name).text === "CachePruneScript").map(binding => ({ name: binding.name.text, path: resolve(dirname(path), moduleSpecifier.text) }));
  });
  expect(bindings).toHaveLength(1);
  expect(bindings[0]!.path).toBe(owner);
  for (const row of imports) if (ts.isStringLiteral(row.moduleSpecifier) && row.moduleSpecifier.text.startsWith(".")) expect(resolve(dirname(path), row.moduleSpecifier.text)).not.toBe(resolve(domainRoot, "📜️script.ts"));
  const clean = syntax.statements.find((node): node is ts.ClassDeclaration => ts.isClassDeclaration(node) && node.name?.text === "CleanScript");
  const method = clean?.members.find((node): node is ts.MethodDeclaration => ts.isMethodDeclaration(node) && ts.isIdentifier(node.name) && node.name.text === "runCachePrune");
  expect(method?.body).toBeDefined();
  const body = method!.body!, nodes: ts.Node[] = [];
  const visit = (node: ts.Node): void => { nodes.push(node); ts.forEachChild(node, visit); };
  visit(body);
  const unwrap = (node: ts.Expression): ts.Expression => ts.isParenthesizedExpression(node) ? unwrap(node.expression) : node;
  const thisField = (node: ts.Expression | undefined, name: string): boolean => !!node && ts.isPropertyAccessExpression(node) && node.expression.kind === ts.SyntaxKind.ThisKeyword && node.name.text === name;
  const calls = nodes.filter((node): node is ts.CallExpression => ts.isCallExpression(node) && ts.isPropertyAccessExpression(node.expression) && node.expression.name.text === "run" && ts.isNewExpression(unwrap(node.expression.expression)));
  expect(calls).toHaveLength(1);
  const call = calls[0]!, construct = unwrap((call.expression as ts.PropertyAccessExpression).expression) as ts.NewExpression;
  expect(ts.isIdentifier(construct.expression) && construct.expression.text).toBe(bindings[0]!.name);
  expect(construct.arguments).toHaveLength(3);
  expect(ts.isIdentifier(construct.arguments![0]!) && (construct.arguments![0] as ts.Identifier).text).toBe("cachingRoot");
  expect(thisField(construct.arguments![1], "repoRoot")).toBe(true);
  expect(thisField(construct.arguments![2], "invocation")).toBe(true);
  expect(ts.isAwaitExpression(call.parent)).toBe(true);
  const coordinate = nodes.find((node): node is ts.VariableDeclaration => ts.isVariableDeclaration(node) && ts.isIdentifier(node.name) && node.name.text === "cachingRoot")?.initializer;
  expect(coordinate && ts.isCallExpression(coordinate)).toBe(true);
  const join = coordinate as ts.CallExpression;
  expect(ts.isIdentifier(join.expression) && join.expression.text).toBe("join");
  expect(join.arguments).toHaveLength(2);
  expect(thisField(join.arguments[0], "root")).toBe(true);
  expect(ts.isStringLiteralLike(join.arguments[1]!) && (join.arguments[1] as ts.StringLiteralLike).text).toBe(relative(repoRoot, domainRoot).replaceAll("\\", "/"));
  expect(method!.parameters).toHaveLength(1);
  expect(ts.isIdentifier(method!.parameters[0]!.name) && method!.parameters[0]!.name.text).toBe("dry");
  expect(method!.parameters[0]!.type?.kind).toBe(ts.SyntaxKind.BooleanKeyword);
  expect(call.arguments).toHaveLength(1);
  const dry = call.arguments[0]!;
  expect(ts.isConditionalExpression(dry)).toBe(true);
  if (!ts.isConditionalExpression(dry)) return;
  expect(ts.isIdentifier(dry.condition) && dry.condition.text).toBe("dry");
  expect(ts.isArrayLiteralExpression(dry.whenTrue) && dry.whenTrue.elements.map(node => ts.isStringLiteralLike(node) ? node.text : null)).toEqual(["--dry-run"]);
  expect(ts.isArrayLiteralExpression(dry.whenFalse) && dry.whenFalse.elements.length).toBe(0);
  const guards = body.statements.filter(ts.isIfStatement);
  const unreadable = guards.find(node => ts.isPrefixUnaryExpression(node.expression) && node.expression.operator === ts.SyntaxKind.ExclamationToken && ts.isIdentifier(node.expression.operand) && node.expression.operand.text === "processes");
  const active = guards.find(node => ts.isCallExpression(node.expression) && ts.isPropertyAccessExpression(node.expression.expression) && ts.isIdentifier(node.expression.expression.expression) && node.expression.expression.expression.text === "processes" && node.expression.expression.name.text === "some");
  for (const guard of [unreadable, active]) {
    expect(guard).toBeDefined();
    expect(guard!.end).toBeLessThan(call.getStart(syntax));
    expect(ts.isBlock(guard!.thenStatement) && guard!.thenStatement.statements.some(ts.isReturnStatement)).toBe(true);
  }
  expect(method!.getText(syntax)).toContain("cache-prune");
});

test("registers the exact Bun and Nx source closure", () => {
  const project = JSON.parse(readFileSync(resolve(domainRoot, "📋️project.json"), "utf8"));
  expect(project.targets[fixture.route.target]?.options.command).toBe(fixture.route.command);
  expect(project.targets[fixture.route.target]?.inputs).toEqual([fixture.route.namedInput]);
  const inputs = new Set(project.namedInputs?.[fixture.route.namedInput] ?? []);
  for(const input of inputs){const path=String(input).replace("{projectRoot}",domainRoot).replace("{workspaceRoot}",repoRoot);if(!/[?*\[\]{}]/u.test(path))expect(existsSync(path),String(input)).toBe(true);}
  expect(inputs.has(`{workspaceRoot}/${fixture.coordinates.rootPruneConsumer}`), fixture.coordinates.rootPruneConsumer).toBe(true);
  for (const owner of fixture.owners) expect(inputs.has(`{projectRoot}/${relative(domainRoot, resolve(repoRoot, owner.path)).replaceAll("\\", "/")}`), owner.path).toBe(true);
  for (const path of [
    "{projectRoot}/📋️project.json",
    "{projectRoot}/📜️script.ts",
    "{projectRoot}/🔣️policy.json",
    "{projectRoot}/🚀️bootstrap/📜️script.ts",
    "{projectRoot}/🧪️tests/🧱️command-source/🟦️.ts",
    "{projectRoot}/🧪️tests/🧱️command-source/🧬️schema/🔣️.json",
    "{projectRoot}/🧪️tests/🧫️fixtures/🧱️command-source/🔣️.json",
    "{projectRoot}/🧹️pruning/🧬️schema/🔣️.json",
  ])
    expect(inputs.has(path), path).toBe(true);
});
