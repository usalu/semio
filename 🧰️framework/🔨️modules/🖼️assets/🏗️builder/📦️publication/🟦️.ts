import { existsSync, lstatSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { renderCatalogArtifacts, type AssetArtifact } from "../../🔣️icons/🏗️builder/📽️projection/🟦️.ts";
import { renderMetabolismArtifacts } from "../../🌱️metabolism/🏗️builder/📽️projection/🟦️.ts";

const assetsRoot = (): string => join(import.meta.dir, "..", "..");

function assetOutputRoots(): readonly string[] {
  const root = assetsRoot();
  const generated = join(root, "🔣️icons/🤖️generated");
  return [
    join(root, "README.md"),
    join(root, "🌱️metabolism/🔣️icons/🤖️generated"),
    join(generated, "🖼️icons"),
    join(generated, "🖼️icon_svgs"),
    join(generated, "🔤️shortcodes"),
    join(generated, "🪪️icon-name", "🦀️.rs"),
  ];
}

function allAssetArtifacts(): readonly AssetArtifact[] {
  return [...renderCatalogArtifacts("all"), ...renderMetabolismArtifacts()];
}

/** @emoji 📋️ Exposes every deterministic asset output as a stable owner-relative manifest. */
export function assetOutputManifest(): readonly string[] {
  return allAssetArtifacts().map((artifact) => relative(assetsRoot(), artifact.path)).sort();
}

function validateAssetOutputManifest(artifacts: readonly AssetArtifact[]): void {
  const paths = artifacts.map((artifact) => artifact.path);
  if (new Set(paths).size !== paths.length) throw new Error("asset output manifest contains duplicate paths");
  const roots = assetOutputRoots();
  for (const path of paths) if (!roots.some((root) => path === root || path.startsWith(`${root}/`))) throw new Error(`asset output escapes owned roots: ${path}`);
  const uiDuplicate = join(assetsRoot(), "..", "🖱️ui", "🖼️assets", "🔣️icons", "🤖️generated");
  if (paths.some((path) => path === uiDuplicate || path.startsWith(`${uiDuplicate}/`))) throw new Error("asset owner must not adopt the unresolved UI icon duplicate");
}

function listAssetFiles(root: string): string[] {
  if (!existsSync(root)) return [];
  const files: string[] = [];
  const walk = (dir: string): void => {
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      const path = join(dir, entry.name);
      if (entry.isDirectory()) walk(path);
      else if (entry.isFile()) files.push(path);
    }
  };
  walk(root);
  return files.sort();
}

function ownedAssetMembership(artifacts: readonly AssetArtifact[]): { actual: string[]; expected: string[] } {
  const roots = assetOutputRoots();
  const actual = roots.flatMap((root) => !existsSync(root) ? [] : lstatSync(root).isDirectory() ? listAssetFiles(root) : [root]).sort();
  return { actual, expected: artifacts.map((artifact) => artifact.path).sort() };
}

function assetExpectedNodePaths(artifacts: readonly AssetArtifact[]): Set<string> {
  const roots = assetOutputRoots();
  const expected = new Set(artifacts.map((artifact) => artifact.path));
  for (const root of roots) if (artifacts.some((artifact) => artifact.path.startsWith(`${root}/`))) expected.add(root);
  for (const artifact of artifacts) {
    const root = roots.find((candidate) => artifact.path.startsWith(`${candidate}/`));
    if (!root) continue;
    for (let dir = dirname(artifact.path); dir !== dirname(root); dir = dirname(dir)) expected.add(dir);
  }
  return expected;
}

function listAssetNodes(root: string): string[] {
  if (!existsSync(root)) return [];
  const paths = [root];
  const walk = (dir: string): void => {
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      const path = join(dir, entry.name);
      paths.push(path);
      if (entry.isDirectory() && !entry.isSymbolicLink()) walk(path);
    }
  };
  if (lstatSync(root).isDirectory()) walk(root);
  return paths;
}

function pruneOwnedAssetOutputs(artifacts: readonly AssetArtifact[]): void {
  const expected = assetExpectedNodePaths(artifacts);
  const stale = assetOutputRoots().flatMap(listAssetNodes).filter((path) => !expected.has(path)).sort((left, right) => right.length - left.length);
  for (const path of stale) rmSync(path, { recursive: true, force: true });
}

export function writeAssetArtifacts(artifacts: readonly AssetArtifact[]): void {
  for (const artifact of artifacts) {
    mkdirSync(dirname(artifact.path), { recursive: true });
    writeFileSync(artifact.path, artifact.content, "utf8");
  }
}

export function publishAssetArtifacts(): number {
  const artifacts = allAssetArtifacts();
  validateAssetOutputManifest(artifacts);
  writeAssetArtifacts(artifacts);
  pruneOwnedAssetOutputs(artifacts);
  return artifacts.length;
}

/** @emoji ✅️ Checks all language renderers, output membership, and external shortcode parity without writes. */
export function checkAssetArtifacts(): void {
  const artifacts = allAssetArtifacts();
  validateAssetOutputManifest(artifacts);
  const membership = ownedAssetMembership(artifacts);
  const stale = artifacts.filter((artifact) => !existsSync(artifact.path) || readFileSync(artifact.path, "utf8") !== artifact.content).map((artifact) => relative(assetsRoot(), artifact.path));
  if (JSON.stringify(membership.actual) !== JSON.stringify(membership.expected) || stale.length > 0) throw new Error(`asset outputs are stale: membership=${JSON.stringify(membership.actual) !== JSON.stringify(membership.expected)}, files=${JSON.stringify(stale)}`);
}

/** 🧾️ Emits the canonical asset byte tree and exact stale removals without writing owned roots. */
export function previewAssetArtifacts(repoRoot: string): string {
  const artifacts = allAssetArtifacts();
  validateAssetOutputManifest(artifacts);
  const expected = assetExpectedNodePaths(artifacts);
  const directories = [...expected].filter((path) => !artifacts.some((artifact) => artifact.path === path));
  const nodes = [
    ...directories.map((path) => ({ bytesBase64: "", mode: 0o755, nodeKind: "directory" as const, path: relative(repoRoot, path).replaceAll("\\", "/").normalize("NFC") })),
    ...artifacts.map((artifact) => ({ bytesBase64: Buffer.from(artifact.content).toString("base64"), mode: 0o644, nodeKind: "file" as const, path: relative(repoRoot, artifact.path).replaceAll("\\", "/").normalize("NFC") })),
  ].sort((left, right) => Buffer.from(left.path).compare(Buffer.from(right.path)));
  const staleRemovals = assetOutputRoots().flatMap(listAssetNodes).filter((path) => !expected.has(path)).map((path) => relative(repoRoot, path).replaceAll("\\", "/").normalize("NFC")).sort((left, right) => Buffer.from(left).compare(Buffer.from(right)));
  return `${JSON.stringify({ contractId: "assets-build", nodes, schemaVersion: 1, staleRemovals })}\n`;
}
