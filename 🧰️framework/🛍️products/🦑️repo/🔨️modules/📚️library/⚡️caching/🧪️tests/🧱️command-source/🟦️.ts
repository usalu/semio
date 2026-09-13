import { expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import Ajv from "ajv";
import ts from "typescript";
import { getWorkspaceRoot, loadTaxonomy, semanticDirectoryKindId } from "../../../📦️packages/🟦️typescript/🟦️.ts";

const repoRoot = getWorkspaceRoot(import.meta.url);
const domainRoot = resolve(import.meta.dir, "../..");
const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧫️fixtures/🧱️command-source/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(resolve(import.meta.dir, "../../🧬️schema/🧱️command-source/🔣️.json"), "utf8"));

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
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
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

test("typechecks an acyclic cache owner graph with no command back edge", { timeout: 30_000 }, () => {
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
    if (active.has(owner)) throw new Error(`cache owner cycle at ${owner}`);
    if (seen.has(owner)) return;
    active.add(owner);
    for (const target of edges.get(owner)!) visit(target);
    active.delete(owner);
    seen.add(owner);
  };
  for (const owner of paths) visit(owner);
  expect(seen.size).toBe(paths.length);
});

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
  const unit = { path: "unit", bytes: 4, recencyMs: 1, lockHeld: false, kind: "directory" as const };
  const operations = {
    cargoDirectories: () => ({ build: "/cache/build", target: "/cache/target" }),
    cacheDirectory: (_root: string, area: string) => `/cache/${area}`,
    scanCargoBuild: (_path: string, signal: AbortSignal) => (signal.throwIfAborted(), [unit]),
    scanCargoTarget: (_path: string, signal: AbortSignal) => (signal.throwIfAborted(), [unit]),
    scanCargoIncremental: (_path: string, signal: AbortSignal) => (signal.throwIfAborted(), { units: [unit], staleSessions: [unit] }),
    scanDirectories: (_path: string, signal: AbortSignal) => (signal.throwIfAborted(), [unit]),
  };
  const areas = scanCacheAreas("/repo", new AbortController().signal, undefined, operations);
  expect(areas.map((area: { name: string }) => area.name)).toEqual(["cargo", "cargo-incremental", "cargo-incremental-sessions", "vite", "agents"]);
  expect(areaUnitRoot("/repo", "cargo", { ...unit, kind: "cargo-build" }, operations)).toBe("/cache/build");
  expect(areaUnitRoot("/repo", "cargo", { ...unit, kind: "cargo-target-file" }, operations)).toBe("/cache/target");
  const cancelled = new AbortController();
  cancelled.abort(new Error("portable cancellation"));
  expect(() => scanCacheAreas("/repo", cancelled.signal, undefined, operations)).toThrow(/portable cancellation/);
});

test("captures bounded child output and terminates a timed-out process tree", { timeout: 10_000 }, async () => {
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
});

test("keeps policy, bootstrap and prune executable coordinates source-relative", async () => {
  const { CACHE_BOOTSTRAP_EXECUTABLE } = await import("../../📇️inventory/🧮️composition/🟦️.ts");
  const source = await import("../../🔍️discovery/📂️source/🟦️.ts");
  expect(resolve(domainRoot, fixture.coordinates.policy)).toBe(source.CACHE_POLICY_PATH);
  expect(resolve(domainRoot, fixture.coordinates.bootstrap)).toBe(CACHE_BOOTSTRAP_EXECUTABLE);
  const rootConsumer = readFileSync(resolve(repoRoot, fixture.coordinates.rootPruneConsumer), "utf8");
  expect(rootConsumer).toContain("⚡️caching/📜️script.ts");
  expect(rootConsumer).toContain("cache-prune");
});

test("registers the exact Bun, Nx and launch source closure", () => {
  const project = JSON.parse(readFileSync(resolve(domainRoot, "📋️project.json"), "utf8"));
  expect(project.targets[fixture.route.target]?.options.command).toBe(fixture.route.command);
  expect(project.targets[fixture.route.target]?.inputs).toEqual([fixture.route.namedInput]);
  const inputs = new Set(project.namedInputs?.[fixture.route.namedInput] ?? []);
  for (const owner of fixture.owners) expect(inputs.has(`{projectRoot}/${relative(domainRoot, resolve(repoRoot, owner.path)).replaceAll("\\", "/")}`), owner.path).toBe(true);
  for (const path of [
    "{projectRoot}/📋️project.json",
    "{projectRoot}/📜️script.ts",
    "{projectRoot}/🔣️policy.json",
    "{projectRoot}/🚀️bootstrap/📜️script.ts",
    "{projectRoot}/🧪️tests/🧱️command-source/🟦️.ts",
    "{projectRoot}/🧬️schema/🧱️command-source/🔣️.json",
    "{projectRoot}/🧫️fixtures/🧱️command-source/🔣️.json",
    "{workspaceRoot}/.vscode/🧩️launch.seed.jsonc",
    "{workspaceRoot}/.vscode/launch.json",
  ])
    expect(inputs.has(path), path).toBe(true);
  for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
    const source = readFileSync(resolve(repoRoot, path), "utf8");
    expect(source.split(fixture.route.launchName).length - 1).toBe(1);
    expect(source).toContain(fixture.route.launchCommand);
  }
});
