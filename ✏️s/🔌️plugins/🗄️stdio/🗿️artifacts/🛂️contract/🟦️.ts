import assert from "node:assert/strict";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { CARGO_COMPOSITION_NAME, CARGO_CONTRACT_NAME, COMPOSITION_TYPESCRIPT_NAME, JsonMap, STDIO_RELATIVE_ROOT, StdioArtifactPackageRecord, canonicalStdioArtifactNames, readStdioJson, slashStdioPath } from "../../📇️inventory/🟦️.ts";

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

function assertDag(packages: readonly StdioArtifactPackageRecord[]): void {
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
  const unexpected = readdirSync(path, { withFileTypes: true }).filter((entry) => entry.name !== "dist" && !allowed.includes(entry.name)).map((entry) => entry.name);
  assert.deepEqual(unexpected, [], `${slashStdioPath(path)} contains implementation files: ${unexpected.join(", ")}`);
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

/** 🧬️ Validates the canonical package projection with the owned JSON Schema and hostile fixture. */
export async function assertStdioArtifactSchema(contract: { schemaVersion: 1; packages: StdioArtifactPackageRecord[] }, stdioRoot: string): Promise<void> {
  const { default: Ajv } = await import("ajv");
  const module = readStdioJson(join(stdioRoot, "🧬️schema/🔣️.json"));
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addKeyword({ keyword: "x-semio-formats", metaSchema: { type: "array", items: { type: "string" } } });
  ajv.addSchema(module);
  const validate = ajv.compile({ $ref: `${module.$id}#/$defs/StdioArtifactPackage` });
  const fixture = readStdioJson(join(stdioRoot, "🧫️fixtures/📦️artifact-package/🔣️.json"));
  for (const value of fixture.accepted) assert(validate(value), JSON.stringify(validate.errors));
  for (const row of fixture.rejected) assert(!validate(row.value), `negative package fixture ${row.id} was accepted`);
  assert(validate(contract), JSON.stringify(validate.errors));
  console.log(`[stdio-package-contract] AJV accepted=${fixture.accepted.length + 1} rejected=${fixture.rejected.length}`);
}

/** 🛂️ Proves package declarations, workspace membership, DAG laws, and composition isolation from the admitted contract. */
export async function assertStdioArtifactSourceContract(repoRoot: string, contract: { schemaVersion: 1; packages: StdioArtifactPackageRecord[] }, stdioRoot = join(repoRoot, STDIO_RELATIVE_ROOT)): Promise<void> {
  const cargoWorkspace = await parseToml(join(repoRoot, "Cargo.toml"));
  const catalogTypeScriptNames = new Set(contract.packages.map((entry) => entry.typescript.name));
  const identities = new Set<string>();
  const directories = new Set<string>();
  const cargoNames = new Set<string>();
  const typescriptNames = new Set<string>();
  for (const entry of contract.packages) {
    const names = canonicalStdioArtifactNames(entry.artifact);
    assert.equal(entry.identity, `s.stdio.${entry.artifact}`);
    assert(!identities.has(entry.identity), `duplicate identity ${entry.identity}`);
    assert(!directories.has(entry.directory), `duplicate directory ${entry.directory}`);
    assert(!cargoNames.has(names.cargo), `duplicate Cargo package ${names.cargo}`);
    assert(!typescriptNames.has(names.typescript), `duplicate TypeScript package ${names.typescript}`);
    identities.add(entry.identity);
    directories.add(entry.directory);
    cargoNames.add(names.cargo);
    typescriptNames.add(names.typescript);
    const definition = readStdioJson(join(repoRoot, entry.source));
    assert.equal(definition.directory, entry.directory);
    assert.equal(definition.id, entry.identity);
    assert.equal(definition.artifact, entry.artifact);
    const rustRoot = dirname(join(repoRoot, entry.rust.manifest));
    const typescriptRoot = dirname(join(repoRoot, entry.typescript.manifest));
    for (const path of [entry.rust.manifest, entry.typescript.manifest, slashStdioPath(relative(repoRoot, join(rustRoot, "📋️project.json"))), slashStdioPath(relative(repoRoot, join(rustRoot, "📜️script.ts"))), slashStdioPath(relative(repoRoot, join(typescriptRoot, "📋️project.json"))), slashStdioPath(relative(repoRoot, join(typescriptRoot, "📜️script.ts")))]) assert(existsSync(join(repoRoot, path)), `missing package declaration ${path}`);
    const cargo = await parseToml(join(repoRoot, entry.rust.manifest));
    assert.equal(cargo.package?.name, names.cargo);
    assert.equal(cargo.lib?.path, "../../🦀️.rs");
    const cargoDependencies = cargoDependencyPackageNames(cargo, cargoWorkspace.workspace ?? {});
    assert(!cargoDependencies.has(CARGO_COMPOSITION_NAME), `${names.cargo} depends on the stdio composition package`);
    assert(cargoDependencies.has(CARGO_CONTRACT_NAME), `${names.cargo} misses the shared artifact contract`);
    assertScriptTargets(readStdioJson(join(rustRoot, "📋️project.json")), names.rustNx);
    assert(readFileSync(join(rustRoot, "📜️script.ts"), "utf8").includes("runArtifactRustPackageMain"), `${names.cargo} does not use the domain-neutral Rust artifact router`);
    const typescript = readStdioJson(join(repoRoot, entry.typescript.manifest));
    assert.equal(typescript.name, names.typescript);
    assert.equal(typescript.type, "module");
    assert.equal(typescript.private, true);
    assert.deepEqual(typescript.exports?.["."], entry.typescript.entry);
    assert.equal(typescript.types, entry.typescript.entry.types);
    assert.equal(typescript.scripts?.build, `bun nx run ${names.typescript}:build`);
    assert.equal(typescript.scripts?.check, `bun nx run ${names.typescript}:check`);
    assert.equal(typescript.scripts?.test, `bun nx run ${names.typescript}:test`);
    for (const dependency of Object.keys(typescript.dependencies ?? {})) assert(catalogTypeScriptNames.has(dependency), `${names.typescript} has unknown artifact dependency ${dependency}`);
    assertScriptTargets(readStdioJson(join(typescriptRoot, "📋️project.json")), names.typescript);
    assert(readFileSync(join(typescriptRoot, "📜️script.ts"), "utf8").includes("runArtifactTypeScriptPackageMain"), `${names.typescript} does not use the domain-neutral TypeScript artifact router`);
    assertDeclarationOnly(rustRoot, ["Cargo.toml", "📋️project.json", "📜️script.ts"]);
    assertDeclarationOnly(typescriptRoot, ["package.json", "📋️project.json", "📜️script.ts"]);
  }
  assertDag(contract.packages);
  const workspacePackage = readStdioJson(join(repoRoot, "package.json"));
  const expectedTypeScriptRoots = contract.packages.map((entry) => slashStdioPath(dirname(entry.typescript.manifest)));
  const declaredTypeScriptRoots = (workspacePackage.workspaces as string[]).filter((path) => path.startsWith(`${STDIO_RELATIVE_ROOT}/🗿️artifacts/`) && path.endsWith("/📦️packages/🟦️typescript"));
  assert.deepEqual(declaredTypeScriptRoots.toSorted(), expectedTypeScriptRoots.toSorted(), "Bun stdio artifact workspaces differ from the registry");
  assert((workspacePackage.workspaces as string[]).includes(`${STDIO_RELATIVE_ROOT}/📦️packages/🟦️typescript`), "Bun workspace misses the stdio composition package");
  const expectedRustRoots = contract.packages.map((entry) => slashStdioPath(dirname(entry.rust.manifest)));
  const declaredRustRoots = (cargoWorkspace.workspace?.members as string[]).filter((path) => path.startsWith(`${STDIO_RELATIVE_ROOT}/🗿️artifacts/`) && path.endsWith("/📦️packages/🦀️rust"));
  assert.deepEqual(declaredRustRoots.toSorted(), expectedRustRoots.toSorted(), "Cargo stdio artifact workspaces differ from the registry");
  const composition = readStdioJson(join(stdioRoot, "📦️packages/🟦️typescript/package.json"));
  for (const entry of contract.packages) assert.equal(composition.dependencies?.[entry.typescript.name], "workspace:*");
  const barrel = readFileSync(join(stdioRoot, "🟦️.ts"), "utf8");
  for (const entry of contract.packages) assert(barrel.includes(`from "${entry.typescript.name}"`), `stdio TypeScript composition misses ${entry.typescript.name}`);
  const legacyConsumers = rustSources(repoRoot).filter((path) => readFileSync(path, "utf8").includes("semio_s_plugin_stdio::artifacts::")).map((path) => slashStdioPath(relative(repoRoot, path)));
  assert.deepEqual(legacyConsumers, [], `Rust consumers retain the stdio composition artifact namespace: ${legacyConsumers.join(", ")}`);
}
