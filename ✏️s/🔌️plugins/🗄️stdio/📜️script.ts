#!/usr/bin/env bun
/** 📦️ Stdio artifact package build, validation and graph contract. */
import assert from "node:assert/strict";
import { copyFileSync, existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve, sep } from "node:path";
import { spawn } from "node:child_process";
import { getWorkspaceRoot } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts";
import { BundleScript, ScriptRouter, runBundleScriptMain, runCmd } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const STDIO_ROOT = import.meta.dir;
const REGISTRY_PATH = join(STDIO_ROOT, "📇️registry/🔣️.json");
const PACKAGE_SCHEMA_PATH = join(STDIO_ROOT, "🧬️schema/📦️artifact-package/🧬️.schema.json");
const PACKAGE_FIXTURE_PATH = join(STDIO_ROOT, "🧪️fixtures/📦️artifact-package/🔣️.json");
const CARGO_CONTRACT_NAME = "semio-s-artifact-stdio-contract";
const CARGO_COMPOSITION_NAME = "semio-s-plugin-stdio";
const NX_CONTRACT_NAME = "@semio-tech/stdio-artifact-contract-rs";
const COMPOSITION_TYPESCRIPT_NAME = "@semio-tech/stdio-js";
const COMPOSITION_RUST_NAME = "@semio-tech/stdio-plugin";

type JsonMap = Record<string, any>;
type PackageRecord = {
  artifact: string;
  directory: string;
  identity: string;
  source: string;
  dependencies: string[];
  rust: { cargoName: string; nxName: string; manifest: string };
  typescript: { name: string; nxName: string; manifest: string; entry: { types: "./dist/🟦️.d.ts"; import: "./dist/🟦️.js" } };
};

function slash(path: string): string {
  return path.split(sep).join("/");
}

function readJson(path: string): JsonMap {
  return JSON.parse(readFileSync(path, "utf8")) as JsonMap;
}

function canonicalNames(artifact: string): { cargo: string; rustNx: string; typescript: string } {
  return {
    cargo: `semio-s-artifact-stdio-${artifact}`,
    rustNx: `@semio-tech/stdio-${artifact}-rs`,
    typescript: `@semio-tech/stdio-${artifact}`,
  };
}

function packageRecord(repoRoot: string, definitionPath: string): PackageRecord {
  const definition = readJson(definitionPath);
  const artifact = String(definition.artifact ?? "");
  const directory = String(definition.directory ?? "");
  const identity = String(definition.id ?? "");
  const artifactRoot = dirname(dirname(definitionPath));
  const names = canonicalNames(artifact);
  return {
    artifact,
    directory,
    identity,
    source: slash(relative(repoRoot, definitionPath)),
    dependencies: Array.isArray(definition.dependencies) ? definition.dependencies.map(String) : [],
    rust: {
      cargoName: names.cargo,
      nxName: names.rustNx,
      manifest: slash(relative(repoRoot, join(artifactRoot, "📦️packages/🦀️rust/Cargo.toml"))),
    },
    typescript: {
      name: names.typescript,
      nxName: names.typescript,
      manifest: slash(relative(repoRoot, join(artifactRoot, "📦️packages/🟦️typescript/package.json"))),
      entry: { types: "./dist/🟦️.d.ts", import: "./dist/🟦️.js" },
    },
  };
}

/** 🧭️ Derives the complete package contract from the schema-owned registry. */
export function stdioArtifactPackageContract(repoRoot: string): { schemaVersion: 1; packages: PackageRecord[] } {
  const registry = readJson(REGISTRY_PATH);
  assert(Array.isArray(registry.artifact_definition_paths), "stdio artifact registry needs artifact_definition_paths");
  const paths = registry.artifact_definition_paths.map((path: unknown) => resolve(dirname(REGISTRY_PATH), String(path)));
  assert.equal(paths.length, 36, "stdio package registry must contain 36 definitions");
  assert.equal(new Set(paths).size, paths.length, "stdio package registry repeats a definition");
  return { schemaVersion: 1, packages: paths.map((path: string) => packageRecord(repoRoot, path)) };
}

function assertDag(packages: readonly PackageRecord[]): void {
  const byIdentity = new Map(packages.map((entry) => [entry.identity, entry]));
  const active = new Set<string>();
  const complete = new Set<string>();
  const visit = (identity: string): void => {
    if (active.has(identity)) throw new Error(`stdio artifact dependency cycle reaches ${identity}`);
    if (complete.has(identity)) return;
    const entry = byIdentity.get(identity);
    assert(entry, `unknown stdio artifact dependency ${identity}`);
    active.add(identity);
    for (const dependency of entry.dependencies) visit(dependency);
    active.delete(identity);
    complete.add(identity);
  };
  for (const entry of packages) visit(entry.identity);
}

function assertScriptTargets(project: JsonMap, name: string): void {
  assert.equal(project.name, name);
  for (const target of ["build", "check", "test"]) {
    const declaration = project.targets?.[target];
    assert.equal(declaration?.executor, "nx:run-commands", `${name}:${target} must use nx:run-commands`);
    assert.equal(declaration?.options?.command, `bun ./📜️script.ts ${target}`, `${name}:${target} must route through 📜️script.ts`);
  }
}

function assertDeclarationOnly(path: string, allowed: readonly string[]): void {
  const unexpected = readdirSync(path, { withFileTypes: true })
    .filter((entry) => entry.name !== "dist" && !allowed.includes(entry.name))
    .map((entry) => entry.name);
  assert.deepEqual(unexpected, [], `${slash(path)} contains implementation files: ${unexpected.join(", ")}`);
}

function rustSources(directory: string, files: string[] = []): string[] {
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    if (entry.isDirectory() && [".git", ".nx", ".🧬semio", "node_modules", "target", "dist", "🗑️generated"].includes(entry.name)) continue;
    const path = join(directory, entry.name);
    if (entry.isDirectory()) rustSources(path, files);
    else if (entry.isFile() && entry.name.endsWith(".rs")) files.push(path);
  }
  return files;
}

async function parseToml(path: string): Promise<JsonMap> {
  const { parse } = await import("@iarna/toml");
  return parse(readFileSync(path, "utf8")) as JsonMap;
}

function cargoDependencyPackageNames(manifest: JsonMap, workspace: JsonMap): Set<string> {
  const result = new Set<string>();
  for (const table of [manifest, ...Object.values(manifest.target ?? {})] as JsonMap[]) {
    for (const kind of ["dependencies", "build-dependencies", "dev-dependencies"]) {
      for (const [alias, value] of Object.entries(table[kind] ?? {})) {
        const declaration = value && typeof value === "object" ? value as JsonMap : {};
        const inherited = declaration.workspace === true ? workspace.dependencies?.[alias] as JsonMap | string | undefined : undefined;
        const resolved = inherited && typeof inherited === "object" ? inherited : declaration;
        result.add(String(resolved.package ?? declaration.package ?? alias));
      }
    }
  }
  return result;
}

async function runCaptured(command: string, args: string[], cwd: string, timeoutMs: number): Promise<string> {
  console.log(`[stdio-package-contract] running ${command} ${args.join(" ")}`);
  const child = spawn(command, args, { cwd, env: { ...process.env, NX_DAEMON: "false" }, stdio: ["ignore", "pipe", "pipe"], windowsHide: true });
  let stdout = "";
  let stderr = "";
  let cancelled = "";
  const maximum = 64 * 1024 * 1024;
  const append = (current: string, chunk: Buffer): string => {
    const next = current + chunk.toString("utf8");
    if (Buffer.byteLength(next) > maximum) {
      cancelled = "output limit";
      child.kill("SIGTERM");
    }
    return next;
  };
  child.stdout?.on("data", (chunk: Buffer) => stdout = append(stdout, chunk));
  child.stderr?.on("data", (chunk: Buffer) => stderr = append(stderr, chunk));
  const cancel = (signal: NodeJS.Signals): void => {
    cancelled = signal;
    child.kill("SIGTERM");
  };
  const interrupt = (): void => cancel("SIGINT");
  const terminate = (): void => cancel("SIGTERM");
  process.once("SIGINT", interrupt);
  process.once("SIGTERM", terminate);
  const started = Date.now();
  const progress = setInterval(() => console.log(`[stdio-package-contract] ${command} running elapsedMs=${Date.now() - started}`), 10_000);
  const timeout = setTimeout(() => {
    cancelled = `timeout ${timeoutMs}ms`;
    child.kill("SIGTERM");
  }, timeoutMs);
  try {
    const result = await new Promise<{ code: number | null; signal: NodeJS.Signals | null }>((resolveExit, rejectExit) => {
      child.once("error", rejectExit);
      child.once("exit", (code, signal) => resolveExit({ code, signal }));
    });
    assert(!cancelled, `${command} ${cancelled}\n${stderr}`);
    assert.equal(result.code, 0, stderr || `${command} exited with ${result.signal ?? result.code}`);
    console.log(`[stdio-package-contract] completed ${command} elapsedMs=${Date.now() - started}`);
    return stdout;
  } finally {
    clearInterval(progress);
    clearTimeout(timeout);
    process.off("SIGINT", interrupt);
    process.off("SIGTERM", terminate);
  }
}

async function cargoMetadata(repoRoot: string): Promise<JsonMap> {
  return JSON.parse(await runCaptured("cargo", ["metadata", "--locked", "--no-deps", "--format-version", "1"], repoRoot, 120_000)) as JsonMap;
}

function assertActualCargoDag(metadata: JsonMap, artifactCargoNames: ReadonlySet<string>): void {
  const packages = new Map((metadata.packages as JsonMap[]).map((entry) => [String(entry.name), entry]));
  const artifactGraph = new Map<string, string[]>();
  for (const name of artifactCargoNames) {
    const cargo = packages.get(name);
    assert(cargo, `Cargo metadata misses ${name}`);
    artifactGraph.set(name, (cargo.dependencies as JsonMap[]).map((dependency) => String(dependency.name)).filter((dependency) => artifactCargoNames.has(dependency)));
  }
  const complete = new Set<string>();
  const visitArtifacts = (name: string, route: string[]): void => {
    const repeated = route.indexOf(name);
    if (repeated >= 0) throw new Error(`Cargo artifact dependency cycle: ${[...route.slice(repeated), name].join(" -> ")}`);
    if (complete.has(name)) return;
    for (const dependency of artifactGraph.get(name) ?? []) visitArtifacts(dependency, [...route, name]);
    complete.add(name);
  };
  const visitComposition = (name: string, route: string[], visited: Set<string>): void => {
    if (name === CARGO_COMPOSITION_NAME) throw new Error(`Cargo artifact reaches stdio composition: ${route.join(" -> ")}`);
    if (visited.has(name)) return;
    visited.add(name);
    const cargo = packages.get(name);
    if (!cargo) return;
    for (const dependency of cargo.dependencies as JsonMap[]) visitComposition(String(dependency.name), [...route, String(dependency.name)], visited);
  };
  for (const name of artifactCargoNames) {
    visitArtifacts(name, []);
    visitComposition(name, [name], new Set());
  }
}

async function assertSourceContract(repoRoot: string, contract: { schemaVersion: 1; packages: PackageRecord[] }): Promise<void> {
  const cargoWorkspace = await parseToml(join(repoRoot, "Cargo.toml"));
  const catalogTypeScriptNames = new Set(contract.packages.map((entry) => entry.typescript.name));
  const identities = new Set<string>();
  const directories = new Set<string>();
  const cargoNames = new Set<string>();
  const typescriptNames = new Set<string>();
  for (const entry of contract.packages) {
    const names = canonicalNames(entry.artifact);
    assert.equal(entry.identity, `s.stdio.${entry.artifact}`);
    assert(!identities.has(entry.identity), `duplicate identity ${entry.identity}`);
    assert(!directories.has(entry.directory), `duplicate directory ${entry.directory}`);
    assert(!cargoNames.has(names.cargo), `duplicate Cargo package ${names.cargo}`);
    assert(!typescriptNames.has(names.typescript), `duplicate TypeScript package ${names.typescript}`);
    identities.add(entry.identity);
    directories.add(entry.directory);
    cargoNames.add(names.cargo);
    typescriptNames.add(names.typescript);
    const definition = readJson(join(repoRoot, entry.source));
    assert.equal(definition.directory, entry.directory);
    assert.equal(definition.id, entry.identity);
    assert.equal(definition.artifact, entry.artifact);
    const rustRoot = dirname(join(repoRoot, entry.rust.manifest));
    const typescriptRoot = dirname(join(repoRoot, entry.typescript.manifest));
    for (const path of [entry.rust.manifest, entry.typescript.manifest, slash(relative(repoRoot, join(rustRoot, "📋️project.json"))), slash(relative(repoRoot, join(rustRoot, "📜️script.ts"))), slash(relative(repoRoot, join(typescriptRoot, "📋️project.json"))), slash(relative(repoRoot, join(typescriptRoot, "📜️script.ts")))]) assert(existsSync(join(repoRoot, path)), `missing package declaration ${path}`);
    const cargo = await parseToml(join(repoRoot, entry.rust.manifest));
    assert.equal(cargo.package?.name, names.cargo);
    assert.equal(cargo.lib?.path, "../../🦀️.rs");
    const cargoDependencies = cargoDependencyPackageNames(cargo, cargoWorkspace.workspace ?? {});
    assert(!cargoDependencies.has("semio-s-plugin-stdio"), `${names.cargo} depends on the stdio composition package`);
    assert(cargoDependencies.has(CARGO_CONTRACT_NAME), `${names.cargo} misses the shared artifact contract`);
    assertScriptTargets(readJson(join(rustRoot, "📋️project.json")), names.rustNx);
    assert(readFileSync(join(rustRoot, "📜️script.ts"), "utf8").includes("runArtifactRustPackageMain"), `${names.cargo} does not use the domain-neutral Rust artifact router`);
    const typescript = readJson(join(repoRoot, entry.typescript.manifest));
    assert.equal(typescript.name, names.typescript);
    assert.equal(typescript.type, "module");
    assert.equal(typescript.private, true);
    assert.deepEqual(typescript.exports?.["."], entry.typescript.entry);
    assert.equal(typescript.types, entry.typescript.entry.types);
    assert.equal(typescript.scripts?.build, `bun nx run ${names.typescript}:build`);
    assert.equal(typescript.scripts?.check, `bun nx run ${names.typescript}:check`);
    assert.equal(typescript.scripts?.test, `bun nx run ${names.typescript}:test`);
    for (const dependency of Object.keys(typescript.dependencies ?? {})) assert(catalogTypeScriptNames.has(dependency), `${names.typescript} has unknown artifact dependency ${dependency}`);
    assertScriptTargets(readJson(join(typescriptRoot, "📋️project.json")), names.typescript);
    assert(readFileSync(join(typescriptRoot, "📜️script.ts"), "utf8").includes("runStdioTypeScriptArtifactPackageMain"), `${names.typescript} does not use its TypeScript artifact router`);
    assertDeclarationOnly(rustRoot, ["Cargo.toml", "📋️project.json", "📜️script.ts"]);
    assertDeclarationOnly(typescriptRoot, ["package.json", "📋️project.json", "📜️script.ts"]);
  }
  assertDag(contract.packages);
  const workspacePackage = readJson(join(repoRoot, "package.json"));
  const expectedTypeScriptRoots = contract.packages.map((entry) => slash(dirname(entry.typescript.manifest)));
  const declaredTypeScriptRoots = (workspacePackage.workspaces as string[]).filter((path) => path.startsWith("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/") && path.endsWith("/📦️packages/🟦️typescript"));
  assert.deepEqual(declaredTypeScriptRoots.toSorted(), expectedTypeScriptRoots.toSorted(), "Bun stdio artifact workspaces differ from the registry");
  assert((workspacePackage.workspaces as string[]).includes("✏️s/🔌️plugins/🗄️stdio/📦️packages/🟦️typescript"), "Bun workspace misses the stdio composition package");
  const expectedRustRoots = contract.packages.map((entry) => slash(dirname(entry.rust.manifest)));
  const declaredRustRoots = (cargoWorkspace.workspace?.members as string[]).filter((path) => path.startsWith("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/") && path.endsWith("/📦️packages/🦀️rust"));
  assert.deepEqual(declaredRustRoots.toSorted(), expectedRustRoots.toSorted(), "Cargo stdio artifact workspaces differ from the registry");
  const composition = readJson(join(STDIO_ROOT, "📦️packages/🟦️typescript/package.json"));
  for (const entry of contract.packages) assert.equal(composition.dependencies?.[entry.typescript.name], "workspace:*");
  const barrel = readFileSync(join(STDIO_ROOT, "🟦️.ts"), "utf8");
  for (const entry of contract.packages) assert(barrel.includes(`from "${entry.typescript.name}"`), `stdio TypeScript composition misses ${entry.typescript.name}`);
  const legacyConsumers = rustSources(repoRoot).filter((path) => readFileSync(path, "utf8").includes("semio_s_plugin_stdio::artifacts::")).map((path) => slash(relative(repoRoot, path)));
  assert.deepEqual(legacyConsumers, [], `Rust consumers retain the stdio composition artifact namespace: ${legacyConsumers.join(", ")}`);
}

async function assertSchemaOracle(contract: { schemaVersion: 1; packages: PackageRecord[] }): Promise<void> {
  const { default: Ajv2020 } = await import("ajv/dist/2020.js");
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(readJson(PACKAGE_SCHEMA_PATH));
  const fixture = readJson(PACKAGE_FIXTURE_PATH);
  for (const value of fixture.accepted) assert(validate(value), JSON.stringify(validate.errors));
  for (const row of fixture.rejected) assert(!validate(row.value), `negative package fixture ${row.id} was accepted`);
  assert(validate(contract), JSON.stringify(validate.errors));
  console.log(`[stdio-package-contract] AJV accepted=${fixture.accepted.length + 1} rejected=${fixture.rejected.length}`);
}

async function assertCargoMetadata(repoRoot: string, contract: { packages: PackageRecord[] }): Promise<void> {
  const metadata = await cargoMetadata(repoRoot);
  const byName = new Map((metadata.packages as JsonMap[]).map((entry) => [entry.name, entry]));
  for (const entry of contract.packages) {
    const cargo = byName.get(entry.rust.cargoName);
    assert(cargo, `Cargo metadata misses ${entry.rust.cargoName}`);
    assert.equal(slash(cargo.manifest_path), slash(join(repoRoot, entry.rust.manifest)));
    const dependencies = new Set((cargo.dependencies as JsonMap[]).map((dependency) => dependency.name));
    assert(!dependencies.has("semio-s-plugin-stdio"));
    assert(dependencies.has(CARGO_CONTRACT_NAME), `${entry.rust.cargoName} misses ${CARGO_CONTRACT_NAME}`);
  }
  assertActualCargoDag(metadata, new Set(contract.packages.map((entry) => entry.rust.cargoName)));
  console.log(`[stdio-package-contract] Cargo metadata packages=${contract.packages.length}`);
}

/** 🔍️ Validates schema fixtures, declarations, workspaces and Cargo's own metadata projection. */
export async function testStdioArtifactPackageContract(repoRoot: string): Promise<void> {
  const contract = stdioArtifactPackageContract(repoRoot);
  await assertSchemaOracle(contract);
  await assertSourceContract(repoRoot, contract);
  await assertCargoMetadata(repoRoot, contract);
  console.log(`[stdio-package-contract] source packages=${contract.packages.length} dag=valid`);
}

/** 🕸️ Compares declared artifact dependencies with the Nx project graph. */
export async function testStdioArtifactPackageGraph(repoRoot: string): Promise<void> {
  const contract = stdioArtifactPackageContract(repoRoot);
  const metadata = await cargoMetadata(repoRoot);
  const cargoPackages = new Map((metadata.packages as JsonMap[]).map((entry) => [String(entry.name), entry]));
  assertActualCargoDag(metadata, new Set(contract.packages.map((entry) => entry.rust.cargoName)));
  const output = await runCaptured(process.execPath, ["run", "nx", "graph", "--print"], repoRoot, 180_000);
  const start = output.indexOf("{");
  assert(start >= 0, "Nx graph emitted no JSON");
  const graph = JSON.parse(output.slice(start)) as JsonMap;
  const dependencies = graph.graph?.dependencies ?? graph.dependencies;
  const nodes = graph.graph?.nodes ?? graph.nodes;
  for (const entry of contract.packages) {
    assert(nodes?.[entry.rust.nxName], `Nx graph misses ${entry.rust.nxName}`);
    assert(nodes?.[entry.typescript.nxName], `Nx graph misses ${entry.typescript.nxName}`);
    const rustEdges = new Set((dependencies?.[entry.rust.nxName] ?? []).map((edge: JsonMap) => edge.target));
    const typescriptEdges = new Set((dependencies?.[entry.typescript.nxName] ?? []).map((edge: JsonMap) => edge.target));
    assert(!rustEdges.has(COMPOSITION_RUST_NAME), `${entry.rust.nxName} has a composition back-edge`);
    assert(rustEdges.has(NX_CONTRACT_NAME), `${entry.rust.nxName} misses Nx edge ${NX_CONTRACT_NAME}`);
    const cargo = cargoPackages.get(entry.rust.cargoName);
    assert(cargo, `Cargo metadata misses ${entry.rust.cargoName}`);
    const expectedRustEdges = (cargo.dependencies as JsonMap[]).map((dependency) => String(dependency.name))
      .filter((name) => name.startsWith("semio-s-artifact-stdio-") && name !== CARGO_CONTRACT_NAME)
      .map((name) => canonicalNames(name.slice("semio-s-artifact-stdio-".length)).rustNx);
    for (const target of expectedRustEdges) assert(rustEdges.has(target), `${entry.rust.nxName} misses Nx edge ${target}`);
    const typescript = readJson(join(repoRoot, entry.typescript.manifest));
    for (const target of Object.keys(typescript.dependencies ?? {})) assert(typescriptEdges.has(target), `${entry.typescript.nxName} misses Nx edge ${target}`);
    const ownerRoot = slash(dirname(dirname(entry.source)));
    const otherOwnerRoots = contract.packages.filter((candidate) => candidate.identity !== entry.identity).map((candidate) => slash(dirname(dirname(candidate.source))));
    for (const node of [nodes[entry.rust.nxName], nodes[entry.typescript.nxName]]) {
      const defaultInputs = node.data?.namedInputs?.default ?? [];
      assert(defaultInputs.some((input: unknown) => typeof input === "string" && input.startsWith(`{workspaceRoot}/${ownerRoot}/`)), `${node.name} does not hash ${ownerRoot}`);
      for (const input of defaultInputs) if (typeof input === "string" && !input.startsWith("!")) for (const other of otherOwnerRoots) {
        if (input.startsWith(`{workspaceRoot}/${other}/`)) assert(!input.includes("*"), `${node.name} broadly hashes another artifact owner ${other}`);
      }
    }
  }
  assert(nodes?.[NX_CONTRACT_NAME], `Nx graph misses ${NX_CONTRACT_NAME}`);
  assert(nodes?.[COMPOSITION_TYPESCRIPT_NAME], `Nx graph misses ${COMPOSITION_TYPESCRIPT_NAME}`);
  console.log(`[stdio-package-contract] Nx graph rust=${contract.packages.length} typescript=${contract.packages.length}`);
}

function artifactFromPackageRoot(packageRoot: string): PackageRecord {
  const artifactRoot = resolve(packageRoot, "../..");
  const definitionPath = join(artifactRoot, "🧬️schema/📜️artifact-definition.json");
  return packageRecord(getWorkspaceRoot(), definitionPath);
}

async function buildTypeScriptPackage(root: string, entry: PackageRecord): Promise<void> {
  const outdir = join(root, "dist");
  const source = resolve(root, "../../🟦️.ts");
  rmSync(outdir, { recursive: true, force: true });
  const result = await Bun.build({ entrypoints: [source], outdir, naming: "🟦️.js", target: "bun", format: "esm", minify: false });
  if (!result.success) throw new AggregateError(result.logs, `failed to build ${entry.typescript.name}`);
  runTypeScriptCompiler(source, ["--declaration", "--emitDeclarationOnly", "--outDir", outdir]);
  const schemaRoot = join(outdir, "🧬️schema");
  mkdirSync(schemaRoot, { recursive: true });
  copyFileSync(resolve(root, "../../🧬️schema/📜️artifact-definition.json"), join(schemaRoot, "📜️artifact-definition.json"));
  console.log(`[stdio-package] built ${entry.typescript.name} outputs=${result.outputs.length}`);
}

async function checkTypeScriptPackage(root: string, entry: PackageRecord): Promise<void> {
  const source = resolve(root, "../../🟦️.ts");
  const result = await Bun.build({ entrypoints: [source], write: false, target: "bun", format: "esm" });
  if (!result.success) throw new AggregateError(result.logs, `failed to check ${entry.typescript.name}`);
  runTypeScriptCompiler(source, ["--noEmit"]);
  console.log(`[stdio-package] checked ${entry.typescript.name}`);
}

async function testTypeScriptPackage(root: string, entry: PackageRecord): Promise<void> {
  await buildTypeScriptPackage(root, entry);
  checkTypeScriptPackageDeclaration(root, entry.typescript.name, "definition, type ArtifactDefinition");
  const module = await import(entry.typescript.name);
  assert.equal(module.definition?.id, entry.identity);
  assert.equal(module.definition?.artifact, entry.artifact);
  console.log(`[stdio-package] tested ${entry.typescript.name} identity=${entry.identity}`);
}

function checkTypeScriptPackageDeclaration(root: string, packageName: string, bindings: string): void {
  const probe = join(root, "dist/🧪️consumer.ts");
  writeFileSync(probe, `import { ${bindings} } from ${JSON.stringify(packageName)};\nvoid definition;\n`);
  try {
    runTypeScriptCompiler(probe, ["--noEmit"]);
  } finally {
    rmSync(probe, { force: true });
  }
}

function runTypeScriptCompiler(source: string, mode: string[]): void {
  runCmd(process.execPath, ["x", "tsc", source, ...mode, "--module", "ESNext", "--moduleResolution", "Bundler", "--resolveJsonModule", "--allowSyntheticDefaultImports", "--strict", "--skipLibCheck", "--target", "ES2022"], { cwd: getWorkspaceRoot() });
}

async function buildTypeScriptComposition(root: string): Promise<void> {
  const outdir = join(root, "dist");
  const source = resolve(root, "../../🟦️.ts");
  rmSync(outdir, { recursive: true, force: true });
  const result = await Bun.build({ entrypoints: [source], outdir, naming: "🟦️.js", target: "bun", format: "esm", minify: false });
  if (!result.success) throw new AggregateError(result.logs, `failed to build ${COMPOSITION_TYPESCRIPT_NAME}`);
  runTypeScriptCompiler(source, ["--declaration", "--emitDeclarationOnly", "--outDir", outdir]);
  console.log(`[stdio-package] built ${COMPOSITION_TYPESCRIPT_NAME} outputs=${result.outputs.length}`);
}

/** 🗄️ Runs the declaration-only TypeScript composition package. */
export async function runStdioCompositionPackageMain(packageRoot: string, scriptUrl: string): Promise<void> {
  class BuildScript extends BundleScript {
    async run(): Promise<void> {
      await buildTypeScriptComposition(this.root);
    }
  }
  class CheckScript extends BundleScript {
    async run(): Promise<void> {
      const result = await Bun.build({ entrypoints: [resolve(this.root, "../../🟦️.ts")], write: false, target: "bun", format: "esm" });
      if (!result.success) throw new AggregateError(result.logs, `failed to check ${COMPOSITION_TYPESCRIPT_NAME}`);
      runTypeScriptCompiler(resolve(this.root, "../../🟦️.ts"), ["--noEmit"]);
      console.log(`[stdio-package] checked ${COMPOSITION_TYPESCRIPT_NAME}`);
    }
  }
  class TestScript extends BundleScript {
    async run(): Promise<void> {
      await buildTypeScriptComposition(this.root);
      checkTypeScriptPackageDeclaration(this.root, COMPOSITION_TYPESCRIPT_NAME, "binary as definition");
      const facade = await import(COMPOSITION_TYPESCRIPT_NAME);
      const entries = Object.entries(facade);
      assert.equal(entries.length, 36);
      for (const [artifact, namespace] of entries) assert.equal((namespace as JsonMap).definition?.id, `s.stdio.${artifact}`);
      console.log(`[stdio-package] tested ${COMPOSITION_TYPESCRIPT_NAME} artifacts=${entries.length}`);
    }
  }
  class ContractScript extends BundleScript {
    async run(): Promise<void> {
      await testStdioArtifactPackageContract(this.repoRoot);
    }
  }
  class GraphScript extends BundleScript {
    async run(): Promise<void> {
      await testStdioArtifactPackageGraph(this.repoRoot);
    }
  }
  const router = new ScriptRouter(packageRoot).register("build", BuildScript).register("check", CheckScript).register("test", TestScript).register("package-contract", ContractScript).register("package-graph", GraphScript);
  await runBundleScriptMain(router, scriptUrl, { defaultCommand: "test" });
}

/** ⚙️ Runs one stdio artifact declaration through its TypeScript package router. */
export async function runStdioTypeScriptArtifactPackageMain(packageRoot: string, scriptUrl: string): Promise<void> {
  const entry = artifactFromPackageRoot(packageRoot);
  class BuildScript extends BundleScript {
    async run(): Promise<void> {
      await buildTypeScriptPackage(this.root, entry);
    }
  }
  class CheckScript extends BundleScript {
    async run(): Promise<void> {
      await checkTypeScriptPackage(this.root, entry);
    }
  }
  class TestScript extends BundleScript {
    async run(): Promise<void> {
      await testTypeScriptPackage(this.root, entry);
    }
  }
  const router = new ScriptRouter(packageRoot).register("build", BuildScript).register("check", CheckScript).register("test", TestScript);
  await runBundleScriptMain(router, scriptUrl, { defaultCommand: "test" });
}

if (import.meta.main) {
  class ContractScript extends BundleScript {
    async run(): Promise<void> {
      await testStdioArtifactPackageContract(this.repoRoot);
    }
  }
  class GraphScript extends BundleScript {
    async run(): Promise<void> {
      await testStdioArtifactPackageGraph(this.repoRoot);
    }
  }
  const router = new ScriptRouter(import.meta.dir).register("package-contract", ContractScript).register("package-graph", GraphScript);
  await runBundleScriptMain(router, import.meta.url, { defaultCommand: "package-contract" });
}
