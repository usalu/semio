import assert from "node:assert/strict";
import { accessSync, constants, existsSync, lstatSync, readFileSync, readdirSync } from "node:fs";
import { dirname, isAbsolute, join, relative, sep } from "node:path";

export const STDIO_RELATIVE_ROOT = "✏️s/🔌️plugins/🗄️stdio";
export const CARGO_CONTRACT_NAME = "semio-s-artifact-stdio-contract";
export const CARGO_COMPOSITION_NAME = "semio-s-plugin-stdio";
export const NX_CONTRACT_NAME = "@semio-tech/stdio-artifact-contract-rs";
export const COMPOSITION_TYPESCRIPT_NAME = "@semio-tech/stdio-js";
export const COMPOSITION_RUST_NAME = "@semio-tech/stdio-plugin";

export type JsonMap = Record<string, any>;
export type StdioArtifactPackageRecord = {
  artifact: string;
  directory: string;
  identity: string;
  source: string;
  dependencies: string[];
  rust: { cargoName: string; nxName: string; manifest: string };
  typescript: { name: string; nxName: string; manifest: string; entry: { types: "./dist/🟦️.d.ts"; import: "./dist/🟦️.js" } };
};

export function slashStdioPath(path: string): string {
  return path.split(sep).join("/");
}

export function readStdioJson(path: string): JsonMap {
  return JSON.parse(readFileSync(path, "utf8")) as JsonMap;
}

export function canonicalStdioArtifactNames(artifact: string): { cargo: string; rustNx: string; typescript: string } {
  return { cargo: `semio-s-artifact-stdio-${artifact}`, rustNx: `@semio-tech/stdio-${artifact}-rs`, typescript: `@semio-tech/stdio-${artifact}` };
}

function admittedDirectory(path: string, label: string): void {
  let status;
  try {
    status = lstatSync(path);
  } catch (error) {
    throw new Error(`missing admitted stdio ${label}: ${path}`, { cause: error });
  }
  if (status.isSymbolicLink()) throw new Error(`admitted stdio ${label} is a symbolic link: ${path}`);
  if (!status.isDirectory()) throw new Error(`admitted stdio ${label} is not a directory: ${path}`);
  try {
    accessSync(path, constants.R_OK | constants.X_OK);
  } catch (error) {
    throw new Error(`cannot read admitted stdio ${label}: ${path}`, { cause: error });
  }
}

function admittedFile(path: string, label: string): void {
  let status;
  try {
    status = lstatSync(path);
  } catch (error) {
    throw new Error(`missing admitted stdio ${label}: ${path}`, { cause: error });
  }
  if (status.isSymbolicLink()) throw new Error(`admitted stdio ${label} is a symbolic link: ${path}`);
  if (!status.isFile()) throw new Error(`admitted stdio ${label} is not a file: ${path}`);
  try {
    accessSync(path, constants.R_OK);
  } catch (error) {
    throw new Error(`cannot read admitted stdio ${label}: ${path}`, { cause: error });
  }
}

function admittedDirectoryChain(root: string, path: string): void {
  admittedDirectory(root, "admission root");
  const route = relative(root, path);
  assert(!isAbsolute(route) && route !== ".." && !route.startsWith(`..${sep}`), `stdio root escapes admission root: ${path}`);
  let current = root;
  for (const segment of route.split(sep).filter(Boolean)) {
    current = join(current, segment);
    admittedDirectory(current, `ancestor ${segment}`);
  }
}

/** 📇️ Admits the exact catalogued artifact-definition leaves without following links or shrinking failures into absence. */
export function stdioArtifactDefinitionPaths(stdioRoot: string, admissionRoot = stdioRoot): string[] {
  admittedDirectoryChain(admissionRoot, stdioRoot);
  const artifactsRoot = join(stdioRoot, "🗿️artifacts");
  admittedDirectory(artifactsRoot, "artifact collection");
  const catalogPath = join(artifactsRoot, "🔣️.json");
  admittedFile(catalogPath, "artifact collection manifest");
  const catalog = readStdioJson(catalogPath);
  const members = catalog["x-semio"]?.members;
  assert(Array.isArray(members), "stdio artifact collection manifest must declare members");
  const directories = members.map((member: JsonMap) => String(member.directory ?? ""));
  assert.equal(directories.length, 36, "stdio must catalog exactly 36 artifact modules");
  assert.equal(new Set(directories).size, directories.length, "stdio artifact collection has duplicate directories");
  for (const directory of directories) assert(/^[^./\\][^/\\]*$/u.test(directory) && directory !== "..", `invalid stdio artifact directory ${directory}`);
  const paths = directories.map((directory) => {
    const artifactRoot = join(artifactsRoot, directory);
    admittedDirectory(artifactRoot, `artifact directory ${directory}`);
    const path = join(artifactRoot, "📜️artifact-definition.json");
    admittedFile(path, `artifact definition ${directory}`);
    return path;
  }).sort();
  const declared = new Set(paths);
  for (const entry of readdirSync(artifactsRoot, { withFileTypes: true })) {
    if (entry.isSymbolicLink()) throw new Error(`admitted stdio artifact collection child is a symbolic link: ${entry.name}`);
    if (!entry.isDirectory()) continue;
    const path = join(artifactsRoot, entry.name, "📜️artifact-definition.json");
    if (existsSync(path) && !declared.has(path)) throw new Error(`uncatalogued stdio artifact definition: ${path}`);
  }
  return paths;
}

function packageRecord(repoRoot: string, definitionPath: string): StdioArtifactPackageRecord {
  const definition = readStdioJson(definitionPath);
  const artifact = String(definition.artifact ?? "");
  const directory = String(definition.directory ?? "");
  const identity = String(definition.id ?? "");
  const artifactRoot = dirname(definitionPath);
  const names = canonicalStdioArtifactNames(artifact);
  return {
    artifact,
    directory,
    identity,
    source: slashStdioPath(relative(repoRoot, definitionPath)),
    dependencies: Array.isArray(definition.dependencies) ? definition.dependencies.map(String) : [],
    rust: { cargoName: names.cargo, nxName: names.rustNx, manifest: slashStdioPath(relative(repoRoot, join(artifactRoot, "📦️packages/🦀️rust/Cargo.toml"))) },
    typescript: { name: names.typescript, nxName: names.typescript, manifest: slashStdioPath(relative(repoRoot, join(artifactRoot, "📦️packages/🟦️typescript/package.json"))), entry: { types: "./dist/🟦️.d.ts", import: "./dist/🟦️.js" } },
  };
}

/** 🧭️ Projects the canonical package contract from the admitted artifact definitions. */
export function stdioArtifactPackageContract(repoRoot: string, stdioRoot = join(repoRoot, STDIO_RELATIVE_ROOT)): { schemaVersion: 1; packages: StdioArtifactPackageRecord[] } {
  return { schemaVersion: 1, packages: stdioArtifactDefinitionPaths(stdioRoot, repoRoot).map((path) => packageRecord(repoRoot, path)) };
}
