import { expect, test } from "bun:test";
import { existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, rmSync, symlinkSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import Ajv from "ajv";
import ts from "typescript";
import { getWorkspaceRoot, loadTaxonomy, semanticDirectoryKindId } from "../../📦️packages/🟦️typescript/🟦️.ts";

const repoRoot = getWorkspaceRoot(import.meta.url);
const libraryRoot = resolve(import.meta.dir, "../..");
const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧫️fixtures/🧱️cargo-transaction-command-source/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧬️schema/🧱️cargo-transaction-command-source/🔣️.json"), "utf8"));

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
  const path = relative(resolve(repoRoot, dirname(consumer)), resolve(libraryRoot, owner)).replaceAll("\\", "/");
  return path.startsWith(".") ? path : `./${path}`;
}

test("validates the language-neutral Cargo, transaction, and Go source contract", () => {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  expect(fixture.owners).toHaveLength(9);
  expect(fixture.contexts).toHaveLength(12);
  expect(fixture.consumers).toHaveLength(18);
});

test("resolves every anonymous owner and semantic context", () => {
  const taxonomy = loadTaxonomy();
  const contexts = new Map(fixture.contexts.map((context: { kindId: string }) => [context.kindId, context]));
  for (const context of fixture.contexts) expect(semanticDirectoryKindId(context.directoryName, taxonomy, { parentKindId: context.parentKindId }), JSON.stringify(context)).toBe(context.kindId);
  for (const owner of fixture.owners) {
    expect(owner.path.split("/").at(-1)).toBe("🟦️.ts");
    expect(owner.path).not.toContain("/📦️packages/");
    const rows = owner.contextChain.map((kindId: string) => contexts.get(kindId));
    expect(rows.every(Boolean), owner.path).toBe(true);
    expect(owner.path.split("/").slice(-1 - rows.length, -1)).toEqual(rows.map((row: { directoryName: string }) => row.directoryName));
    for (let index = 1; index < rows.length; index++) expect(rows[index].parentKindId, owner.path).toBe(rows[index - 1].kindId);
    const path = resolve(repoRoot, owner.path);
    expect(existsSync(path), owner.path).toBe(true);
    if (existsSync(path)) expect(namedDeclarations(path)).toEqual([...owner.declarations].sort());
  }
  expect(new Set(fixture.owners.flatMap((owner: { contextChain: string[] }) => owner.contextChain))).toEqual(new Set(fixture.contexts.map((context: { kindId: string }) => context.kindId)));
});

test("typechecks an acyclic owner graph with no command-module back edge", { timeout: 30_000 }, () => {
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
    paths
      .flatMap((path: string) => [...program.getSyntacticDiagnostics(program.getSourceFile(path)), ...program.getSemanticDiagnostics(program.getSourceFile(path))])
      .map((diagnostic) => {
        const position = diagnostic.file && diagnostic.start !== undefined ? diagnostic.file.getLineAndCharacterOfPosition(diagnostic.start) : undefined;
        return `${diagnostic.file ? relative(repoRoot, diagnostic.file.fileName) : "<owner>"}${position ? `:${position.line + 1}:${position.character + 1}` : ""}: ${ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n")}`;
      }),
  ).toEqual([]);
  const owners = new Set(paths),
    commandModules = new Set([resolve(libraryRoot, "⚡️caching/🦀️cargo/📜️script.ts"), resolve(libraryRoot, "📦️packages/🟦️typescript/📜️script.ts")]),
    edges = new Map<string, string[]>(paths.map((path: string) => [path, []]));
  for (const owner of paths) {
    const source = ts.createSourceFile(owner, readFileSync(owner, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    for (const statement of source.statements) {
      const specifier = (ts.isImportDeclaration(statement) || ts.isExportDeclaration(statement)) && statement.moduleSpecifier && ts.isStringLiteral(statement.moduleSpecifier) ? statement.moduleSpecifier.text : null;
      if (!specifier?.startsWith(".")) continue;
      const target = resolve(dirname(owner), specifier);
      expect(commandModules.has(target), `${owner} -> ${target}`).toBe(false);
      if (owners.has(target)) edges.get(owner)!.push(target);
    }
  }
  const seen = new Set<string>(),
    active = new Set<string>();
  const visit = (owner: string): void => {
    if (active.has(owner)) throw new Error(`owner cycle at ${owner}`);
    if (seen.has(owner)) return;
    active.add(owner);
    for (const target of edges.get(owner)!) visit(target);
    active.delete(owner);
    seen.add(owner);
  };
  for (const owner of paths) visit(owner);
  expect(seen.size).toBe(paths.length);
});

test("moves behavior out of command modules and rebinds every live API consumer", () => {
  const moved = new Set(fixture.owners.flatMap((owner: { declarations: string[] }) => owner.declarations));
  for (const command of ["⚡️caching/🦀️cargo/📜️script.ts", "📦️packages/🟦️typescript/📜️script.ts"]) {
    const path = resolve(libraryRoot, command);
    expect(namedDeclarations(path).filter((name) => moved.has(name))).toEqual([]);
  }
  for (const consumer of fixture.consumers) {
    const path = resolve(repoRoot, consumer.path);
    const source = readFileSync(path, "utf8");
    const syntax = ts.createSourceFile(path, source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    for (const owner of consumer.owners) {
      const specifier = relativeSpecifier(consumer.path, owner);
      const statement = syntax.statements.find((candidate) => ts.isImportDeclaration(candidate) && ts.isStringLiteral(candidate.moduleSpecifier) && candidate.moduleSpecifier.text === specifier);
      if (!statement || !ts.isImportDeclaration(statement)) {
        expect(source.includes(owner), `${consumer.path} -> ${owner}`).toBe(true);
        continue;
      }
      const bindings = statement.importClause?.namedBindings;
      expect(bindings && ts.isNamedImports(bindings), `${consumer.path} -> ${owner}`).toBeTruthy();
      const declarations = new Set(fixture.owners.find((candidate: { path: string }) => candidate.path.endsWith(owner))!.declarations);
      if (bindings && ts.isNamedImports(bindings)) for (const binding of bindings.elements) expect(declarations.has((binding.propertyName ?? binding.name).text), `${consumer.path}: ${binding.name.text}`).toBe(true);
    }
    expect(source).not.toMatch(/from ["'][^"']*⚡️caching\/🦀️cargo\/📜️script\.ts["']/);
  }
});

test("preserves native selector admission through portable vectors", async () => {
  const owner = resolve(libraryRoot, "⚡️caching/📦️artifacts/🎛️native-input/🟦️.ts");
  expect(existsSync(owner)).toBe(true);
  if (!existsSync(owner)) return;
  const native = await import(owner);
  for (const row of fixture.nativeArguments) {
    const invoke = () => native.validateNativeCargoArguments(row.operation, row.args);
    if (row.valid) expect(invoke, JSON.stringify(row)).not.toThrow();
    else expect(invoke, JSON.stringify(row)).toThrow(/input contract/);
  }
  expect(native.artifactRustCargoArguments("test", ["quick", "--", "--nocapture"])).toEqual({ cargoArgs: ["--", "--nocapture"], testLevel: "quick" });
});

test("projects requested Go inputs without copying the canonical planner", async () => {
  const owner = resolve(libraryRoot, "🧪️execution/🐹️go/🟦️.ts");
  expect(existsSync(owner)).toBe(true);
  if (!existsSync(owner)) return;
  const { projectGoInputPackages } = await import(owner);
  const plan = { packages: fixture.goSelection.packages, replacements: fixture.goSelection.replacements };
  for (const row of fixture.goSelection.cases) {
    const invoke = () =>
      projectGoInputPackages("/repo", row.input, {
        realpath: (path: string) => path,
        isDirectory: (path: string) => row.directories.includes(path),
        plan: () => plan,
      });
    if (row.accepted) expect(invoke(), row.input).toEqual({ moduleRoot: "/repo", packages: row.expected });
    else expect(invoke, row.input).toThrow(/compiler package owner/);
  }
  expect(readFileSync(owner, "utf8")).toContain("canonicalGoPlan");
  expect(readFileSync(owner, "utf8")).toContain("runCanonicalGoTests");
});

test("owns transaction allocation and exact source identities in the current ticket", async () => {
  const allocationOwner = resolve(libraryRoot, "🔄️transactions/🧪️verification/📁️run-allocation/🟦️.ts");
  const provenanceOwner = resolve(libraryRoot, "🔄️transactions/🧪️verification/🧾️provenance/🟦️.ts");
  expect(existsSync(allocationOwner)).toBe(true);
  expect(existsSync(provenanceOwner)).toBe(true);
  if (!existsSync(allocationOwner) || !existsSync(provenanceOwner)) return;
  const allocation = await import(allocationOwner);
  const provenance = await import(provenanceOwner);
  expect(allocation.TRANSACTION_V2_RUN_OWNER_RELATIVE).toBe(fixture.transaction.runOwnerPath);
  const artifactParent = process.env.SEMIO_TEST_ARTIFACT_DIR;
  expect(artifactParent).toBeTruthy();
  if (!artifactParent) return;
  const sandbox = mkdtempSync(resolve(artifactParent, "transaction-allocation-"));
  try {
    mkdirSync(resolve(sandbox, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE"), { recursive: true });
    const id = `${process.pid}-${crypto.randomUUID()}`;
    const bundle = allocation.transactionV2BundleRoot(sandbox, id);
    expect(relative(sandbox, dirname(bundle)).replaceAll("\\", "/")).toBe(`${fixture.transaction.runOwnerPath}/🔖️${id.slice(id.indexOf("-") + 1)}`);
    expect(lstatSync(bundle).isDirectory()).toBe(true);
    expect(() => allocation.transactionV2BundleRoot(sandbox, id)).toThrow();
    expect(() => allocation.transactionV2BundleRoot(sandbox, "1-../escape")).toThrow(/Invalid transaction/);
    const linked = mkdtempSync(resolve(artifactParent, "transaction-linked-"));
    const linkedRepo = resolve(sandbox, "linked-repo");
    mkdirSync(resolve(linkedRepo, dirname(fixture.transaction.runOwnerPath)), { recursive: true });
    rmSync(resolve(linkedRepo, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE/🗑️generated"), { recursive: true, force: true });
    symlinkSync(linked, resolve(linkedRepo, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE/🗑️generated"));
    expect(() => allocation.transactionV2BundleRoot(linkedRepo, `${process.pid}-${crypto.randomUUID()}`)).toThrow(/no-follow/);
  } finally {
    rmSync(sandbox, { recursive: true, force: true });
  }
  const paths = provenance.transactionV2IdentityPaths(repoRoot);
  expect(Object.keys(paths).sort()).toEqual(["allocation", "discovery", "execution", "harness", "ledgerBoundaries", "normalization", "provenance", "shards", "suite", "taxonomy"]);
  for (const [key, expected] of Object.entries(fixture.transaction)) if (key !== "runOwnerPath") expect(paths[key]).toBe(resolve(repoRoot, expected as string));
  for (const key of ["allocation", "execution", "provenance", "shards"]) expect(fixture.owners.some((owner: { path: string }) => resolve(repoRoot, owner.path) === paths[key])).toBe(true);
  const identities = provenance.transactionV2Identities(paths);
  expect(Object.keys(identities).sort()).toEqual(Object.keys(paths).sort());
  for (const value of Object.values(identities) as { bytes: number; sha256: string }[]) {
    expect(value.bytes).toBeGreaterThan(0);
    expect(value.sha256).toMatch(/^[0-9a-f]{64}$/);
  }
});

test("keeps the 62-case shard selection and source-as-data consumers exact", async () => {
  const shardOwner = resolve(libraryRoot, "🔄️transactions/🧪️verification/🏃️shard-execution/🟦️.ts");
  expect(existsSync(shardOwner)).toBe(true);
  if (!existsSync(shardOwner)) return;
  const { TRANSACTION_V2_DEFAULT_FILTER_WAVES } = await import(shardOwner);
  const harness = JSON.parse(readFileSync(resolve(repoRoot, fixture.transaction.harness), "utf8"));
  expect(TRANSACTION_V2_DEFAULT_FILTER_WAVES).toEqual([harness.shards.map((row: { filter: string }) => row.filter)]);
  expect(harness.shards.reduce((sum: number, row: { tests: number }) => sum + row.tests, 0)).toBe(62);
  expect(harness.launcherPath).toBe(fixture.owners.find((owner: { declarations: string[] }) => owner.declarations.includes("TRANSACTION_V2_DEFAULT_FILTER_WAVES")).path);
  expect(harness.allocationPath).toBe(fixture.owners.find((owner: { declarations: string[] }) => owner.declarations.includes("transactionV2BundleRoot")).path);
  expect(harness.runOwnerPath).toBe(fixture.transaction.runOwnerPath);
  const suite = readFileSync(resolve(repoRoot, fixture.transaction.suite), "utf8");
  expect(suite).toContain("harness.filterWavesConstant");
  expect(suite).toContain("harness.bundleRootFunction");
  expect(suite).toContain("harness.allocationPath");
  expect(suite).not.toContain("📦️packages/🟦️typescript/📜️script.ts");
});

test("terminates a bounded owned descendant process", { timeout: 10_000 }, async () => {
  const owner = resolve(libraryRoot, "🏃️process/🎛️owned-execution/🟦️.ts");
  expect(existsSync(owner)).toBe(true);
  if (!existsSync(owner)) return;
  const artifactParent = process.env.SEMIO_TEST_ARTIFACT_DIR;
  expect(artifactParent).toBeTruthy();
  if (!artifactParent) return;
  const root = mkdtempSync(resolve(artifactParent, "owned-execution-"));
  const marker = resolve(root, "descendant.txt");
  const source = [
    "const {spawn}=require('node:child_process')",
    "const {writeFileSync}=require('node:fs')",
    "const child=spawn(process.execPath,['-e','setTimeout(() => {}, 10000)'],{stdio:'ignore'})",
    `writeFileSync(${JSON.stringify(marker)},String(child.pid))`,
    "setTimeout(() => {}, 10000)",
  ].join(";");
  const { runOwnedCommand } = await import(owner);
  try {
    await expect(runOwnedCommand(process.execPath, ["-e", source], repoRoot, "owned-descendant", 2_000, { stdout: "ignore" })).rejects.toThrow(/timeout 2000ms/);
    const descendant = Number(readFileSync(marker, "utf8"));
    const alive = (): boolean => {
      try {
        process.kill(descendant, 0);
        return true;
      } catch {
        return false;
      }
    };
    for (let attempt = 0; attempt < 40 && alive(); attempt++) await Bun.sleep(25);
    try {
      expect(alive()).toBe(false);
    } finally {
      if (alive()) process.kill(descendant, "SIGKILL");
    }
    expect(readFileSync(owner, "utf8")).toContain('["taskkill", "/pid", String(child.pid), "/t", "/f"]');
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("registers exact Bun, Nx, cache, package, and launch closure", () => {
  const projectPath = resolve(libraryRoot, "📦️packages/🟦️typescript/📋️project.json");
  const project = JSON.parse(readFileSync(projectPath, "utf8"));
  const source = fixture.routes.source;
  expect(project.targets[source.target]?.options.command).toBe(source.command);
  expect(project.targets[source.target]?.inputs).toEqual(["cargoTransactionCommandSources"]);
  expect(project.targets[fixture.routes.go.target]?.options.command).toBe(fixture.routes.go.command);
  expect(project.targets[fixture.routes.goProjection.target]?.options.command).toBe(fixture.routes.goProjection.command);
  expect(project.targets[fixture.routes.goDispatch.target]?.options.command).toBe(fixture.routes.goDispatch.command);
  expect(project.targets[fixture.routes.transaction.target]?.options.command).toBe(fixture.routes.transaction.command);
  const inputs = new Set(project.namedInputs?.cargoTransactionCommandSources ?? []);
  for (const owner of fixture.owners) expect(inputs.has(`{workspaceRoot}/${owner.path}`), owner.path).toBe(true);
  for (const path of [fixture.transaction.ledgerBoundaries, fixture.transaction.harness, fixture.transaction.suite, fixture.transaction.normalization, fixture.transaction.taxonomy, fixture.transaction.discovery])
    expect(inputs.has(`{workspaceRoot}/${path}`), path).toBe(true);
  const packageJson = JSON.parse(readFileSync(resolve(libraryRoot, "📦️packages/🟦️typescript/package.json"), "utf8"));
  expect(packageJson.scripts[source.target]).toBe(`nx run @semio-tech/repo-lib:${source.target}`);
  for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
    const launch = readFileSync(resolve(repoRoot, path), "utf8");
    for (const route of [source, fixture.routes.goProjection, fixture.routes.goDispatch, fixture.routes.transaction]) {
      expect(launch.split(route.launchName).length - 1, `${path}: ${route.launchName}`).toBe(1);
      expect(launch).toContain(route.launchCommand);
    }
  }
});
