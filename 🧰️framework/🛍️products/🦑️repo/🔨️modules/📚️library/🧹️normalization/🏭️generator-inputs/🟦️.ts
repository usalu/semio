import { requireRecord } from "../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import { type TaxonomyNodeKind } from "../🚪️source-admission/🟦️.ts";
import { type LoadedTaxonomy, type GeneratorInputTaxonomy } from "../🔣️taxonomy/📚️loaded/🟦️.ts";
import { type GeneratorContractSpec } from "../🔣️taxonomy/🏭️generators-packages/🟦️.ts";
import { normalizeRelative, sourceRelative, emojiFold } from "../🛣️path/🟦️.ts";
import { sha256, lstatOrNull } from "../📁️input/🟦️.ts";
import { lstatSync, readFileSync, readdirSync, readlinkSync } from "node:fs";
import { registryCatalogInputPaths } from "../../🔍️discovery/🧲️registry-catalog-projection/🟦️.ts";
import { registryCatalogInputView } from "../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { type RegistryCatalogInputView } from "../../🔍️discovery/📇️registry-catalog-inputs/🟦️.ts";
import type { TaxonomyGeneratorNodeRecord, TaxonomyGeneratorPreviewManifest, TaxonomyInventory, TaxonomyInventoryEntry, TaxonomyRegeneration, TaxonomyViolation } from "../📜️plan-contracts/🟦️.ts";
import { absolutePath, checkCancellation, isExcluded } from "../🏷️file-kind-matching/🟦️.ts";
import { generatorPathCompare } from "../🧾️plan-parsing/🟦️.ts";

export interface CollisionGroup {
  readonly id: string;
  readonly comparison: string;
  readonly paths: readonly string[];
  readonly sources: readonly string[];
}

export function collisionKey(path: string, comparison: string): string {
  if (comparison === "byte" || comparison === "same-kind") return path;
  if (comparison === "nfc") return path.normalize("NFC");
  if (comparison === "case-fold") return path.normalize("NFC").toLocaleLowerCase("und");
  return emojiFold(path).toLocaleLowerCase("und");
}

export function collisionGroups(entries: readonly TaxonomyInventoryEntry[], taxonomy: LoadedTaxonomy): readonly CollisionGroup[] {
  const groups: CollisionGroup[] = [];
  for (const comparison of taxonomy.schema.collisionPolicy.comparisons) {
    const buckets = new Map<string, TaxonomyInventoryEntry[]>();
    for (const entry of entries) {
      const key = comparison === "same-kind" ? `${entry.nodeKind}\u0000${entry.fileKind ?? "fixed"}\u0000${collisionKey(entry.normalizedPath, comparison)}` : collisionKey(entry.normalizedPath, comparison);
      const rows = buckets.get(key) ?? [];
      rows.push(entry);
      buckets.set(key, rows);
    }
    for (const [key, rows] of buckets) {
      if (rows.length < 2) continue;
      const sources = rows.map((entry) => entry.sourcePath).sort();
      groups.push({ id: sha256(`${comparison}\u0000${key}\u0000${sources.join("\u0000")}`).slice(0, 24), comparison, paths: [...new Set(rows.map((entry) => entry.normalizedPath))].sort(), sources });
    }
  }
  return groups.sort((a, b) => a.comparison.localeCompare(b.comparison) || a.id.localeCompare(b.id));
}

export function generatorNodeRecord(repoRoot: string, path: string, taxonomy: Pick<LoadedTaxonomy, "exclusions">): TaxonomyGeneratorNodeRecord {
  if (isExcluded(path, taxonomy)) throw new Error(`Generator node is opaque: ${path}`);
  const absolute = absolutePath(repoRoot, path);
  const stat = lstatSync(absolute);
  const nodeKind: TaxonomyNodeKind = stat.isSymbolicLink() ? "symlink" : stat.isDirectory() ? "directory" : "file";
  const normalized = normalizeRelative(path), mode = stat.mode & 0o7777;
  if (nodeKind === "directory") return { path: normalized, nodeKind, contentHash: sha256("directory"), mode };
  if (nodeKind === "symlink") {
    const target = readlinkSync(absolute);
    return { path: normalized, nodeKind, contentHash: sha256(target), mode, size: Buffer.byteLength(target), target };
  }
  const bytes = readFileSync(absolute);
  return { path: normalized, nodeKind, contentHash: sha256(bytes), mode, size: bytes.byteLength };
}

export function generatorTreeInventory(repoRoot: string, roots: readonly string[], taxonomy: LoadedTaxonomy): readonly TaxonomyGeneratorNodeRecord[] {
  const rows = new Map<string, TaxonomyGeneratorNodeRecord>();
  const walk = (path: string): void => {
    if (isExcluded(path, taxonomy)) throw new Error(`Generator output root is opaque: ${path}`);
    const absolute = absolutePath(repoRoot, path);
    const stat = lstatOrNull(absolute);
    if (!stat) return;
    rows.set(path, generatorNodeRecord(repoRoot, path, taxonomy));
    if (!stat.isDirectory() || stat.isSymbolicLink()) return;
    for (const child of readdirSync(absolute).sort((a, b) => Buffer.from(a).compare(Buffer.from(b)))) walk(sourceRelative(`${path}/${child}`));
  };
  for (const root of [...new Set(roots.map(normalizeRelative))].sort(generatorPathCompare)) walk(root);
  return [...rows.values()].sort((left, right) => generatorPathCompare(left.path, right.path));
}

/** 📇️ Selects schema-owned generator inputs through physical or logical preimage membership. */
function compilerInputManifestRows(value: unknown, authority: NonNullable<GeneratorContractSpec["compilerInputManifest"]>, label: string): readonly Readonly<{ path: string; bytes: number; sha256: string }>[] {
  const manifest = requireRecord(value, label);
  if (Object.keys(manifest).sort().join("|") !== "contractId|inputs|layoutSha256|outputs|version" || manifest.version !== 1 || typeof manifest.contractId !== "string" || !/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(manifest.contractId) || typeof manifest.layoutSha256 !== "string" || !/^[a-f0-9]{64}$/u.test(manifest.layoutSha256) || !Array.isArray(manifest.outputs) || !Array.isArray(manifest.inputs) || manifest.inputs.length < 1 || manifest.inputs.length > authority.maxFiles) throw new Error(`${label} is not one bounded compiler input manifest`);
  const seen = new Set<string>();
  const rows = manifest.inputs.map((value, index) => {
    const row = requireRecord(value, `${label}.inputs[${index}]`);
    if (Object.keys(row).sort().join("|") !== "bytes|path|sha256" || typeof row.path !== "string" || row.path !== normalizeRelative(row.path) || typeof row.bytes !== "number" || !Number.isSafeInteger(row.bytes) || row.bytes < 0 || typeof row.sha256 !== "string" || !/^[a-f0-9]{64}$/u.test(row.sha256) || seen.has(row.path)) throw new Error(`${label} contains an invalid compiler input witness`);
    seen.add(row.path);
    return { path: row.path, bytes: row.bytes, sha256: row.sha256 };
  });
  if (rows.some((row, index) => index > 0 && generatorPathCompare(rows[index - 1]!.path, row.path) > 0)) throw new Error(`${label} compiler inputs are not path-sorted`);
  return rows;
}

function compilerInputRecords(repoRoot: string, contract: GeneratorContractSpec, taxonomy: Pick<LoadedTaxonomy, "exclusions">, manifest: unknown, label: string): readonly TaxonomyGeneratorNodeRecord[] {
  const authority = contract.compilerInputManifest;
  if (!authority) return [];
  return compilerInputManifestRows(manifest, authority, label).map(row => {
    if (isExcluded(row.path, taxonomy) || contract.outputRoots.some(root => row.path === root.path || row.path.startsWith(`${root.path}/`))) throw new Error(`${label} owns an opaque or self-produced compiler input: ${row.path}`);
    const node = generatorNodeRecord(repoRoot, row.path, taxonomy);
    if (node.nodeKind !== "file" || node.size !== row.bytes || node.contentHash !== row.sha256) throw new Error(`${label} compiler input bytes differ: ${row.path}`);
    return node;
  });
}

export function compilerPreviewInputRecords(repoRoot: string, contract: GeneratorContractSpec, taxonomy: LoadedTaxonomy, preview: TaxonomyGeneratorPreviewManifest): readonly TaxonomyGeneratorNodeRecord[] {
  const authority = contract.compilerInputManifest;
  if (!authority) return [];
  const node = preview.nodes.find(node => node.path === authority.manifestOutputPath);
  if (!node || node.nodeKind !== "file") throw new Error(`Compiler preview lacks its declared input manifest: ${authority.manifestOutputPath}`);
  return compilerInputRecords(repoRoot, contract, taxonomy, JSON.parse(Buffer.from(node.bytesBase64, "base64").toString("utf8")), "Compiler preview manifest");
}

const registryCatalogPathsByView = new WeakMap<RegistryCatalogInputView, WeakMap<object, readonly string[]>>();

const generatorPrefixCandidatesByView = new WeakMap<RegistryCatalogInputView, WeakMap<object, Map<string, readonly string[]>>>();

/** 🧭️ The current input paths of one generator contract. Over one shared catalog view (one repository snapshot), the registry catalog
 * input set and every literal-prefix walk are computed once and reused by every contract that names them. */
export function generatorInputPaths(inventory: Pick<TaxonomyInventory, "repoRoot">, contract: GeneratorContractSpec, taxonomy: GeneratorInputTaxonomy, cancelFile?: string, view?: RegistryCatalogInputView): readonly string[] {
  const paths = new Set<string>();
  const inputView = view ?? registryCatalogInputView(inventory.repoRoot, taxonomy.discoverySchema);
  if (contract.inputDiscovery) {
    const catalogTaxonomy = taxonomy.discoverySchema;
    const catalogView = inputView;
    const cancellableView: RegistryCatalogInputView = {
      entries(path) { checkCancellation(inventory.repoRoot, cancelFile); return catalogView.entries(path); },
      kind(path) { checkCancellation(inventory.repoRoot, cancelFile); return catalogView.kind(path); },
      readText(path) { checkCancellation(inventory.repoRoot, cancelFile); return catalogView.readText(path); },
      readBytes(path) { checkCancellation(inventory.repoRoot, cancelFile); return catalogView.readBytes(path); },
    };
    let catalogPaths = view ? registryCatalogPathsByView.get(view)?.get(catalogTaxonomy) : undefined;
    if (!catalogPaths) {
      catalogPaths = registryCatalogInputPaths(inventory.repoRoot, catalogTaxonomy, cancellableView);
      if (view) {
        let byTaxonomy = registryCatalogPathsByView.get(view);
        if (!byTaxonomy) registryCatalogPathsByView.set(view, byTaxonomy = new WeakMap());
        byTaxonomy.set(catalogTaxonomy, catalogPaths);
      }
    }
    for (const path of catalogPaths) paths.add(path);
  }
  let candidatesByPrefix = generatorPrefixCandidatesByView.get(inputView)?.get(taxonomy);
  if (!candidatesByPrefix) {
    let byTaxonomy = generatorPrefixCandidatesByView.get(inputView);
    if (!byTaxonomy) generatorPrefixCandidatesByView.set(inputView, byTaxonomy = new WeakMap());
    byTaxonomy.set(taxonomy, candidatesByPrefix = new Map());
  }
  for (const pattern of contract.inputPatterns) {
    const segments = pattern.split("/");
    const wildcard = segments.findIndex((segment) => /[*?[]/u.test(segment));
    const prefix = (wildcard < 0 ? segments : segments.slice(0, wildcard)).join("/");
    if (!prefix || isExcluded(prefix, taxonomy)) throw new Error("Generator input requires a nonopaque literal owner prefix: " + pattern);
    if (wildcard < 0) { paths.add(prefix); continue; }
    let candidates = candidatesByPrefix.get(prefix);
    if (!candidates) {
      const nodes = new Set<string>();
      const visit = (path: string): void => {
        checkCancellation(inventory.repoRoot, cancelFile);
        if (isExcluded(path, taxonomy)) return;
        const kind = inputView.kind(path);
        if (kind === null) return;
        if (kind === "symlink") throw new Error(`Generator input prefix is a symlink: ${path}`);
        nodes.add(path);
        if (kind === "directory") for (const entry of inputView.entries(path)) {
          const child = `${path}/${entry.name}`;
          if (isExcluded(child, taxonomy)) continue;
          if (entry.nodeKind === "directory") visit(child);
          else nodes.add(child);
        }
      };
      visit(prefix);
      candidates = [...nodes];
      candidatesByPrefix.set(prefix, candidates);
    }
    for (const candidate of candidates) {
      checkCancellation(inventory.repoRoot, cancelFile);
      if (taxonomy.pathMatcher.matches(candidate, pattern)) paths.add(candidate);
    }
  }
  if (contract.compilerInputManifest) {
    const manifestPath = absolutePath(inventory.repoRoot, contract.compilerInputManifest.manifestOutputPath), node = lstatOrNull(manifestPath);
    if (node?.isSymbolicLink()) throw new Error(`Compiler input manifest is a symlink: ${contract.compilerInputManifest.manifestOutputPath}`);
    if (node?.isFile()) for (const row of compilerInputRecords(inventory.repoRoot, contract, taxonomy, JSON.parse(readFileSync(manifestPath, "utf8")), "Current compiler input manifest")) paths.add(row.path);
    else if (node !== null) throw new Error(`Compiler input manifest is not a regular file: ${contract.compilerInputManifest.manifestOutputPath}`);
  }
  return [...paths].filter((path) => !isExcluded(path, taxonomy) && !contract.outputRoots.some((output) => path === output.path || path.startsWith(output.path + "/"))).sort(generatorPathCompare).filter((path) => {
    checkCancellation(inventory.repoRoot, cancelFile);
    const kind = inputView.kind(path);
    if (kind === "symlink") throw new Error(`Generator input is a symlink: ${path}`);
    return kind !== null;
  });
}

export function generatorInputInventory(inventory: TaxonomyInventory, contract: GeneratorContractSpec, taxonomy: LoadedTaxonomy, cancelFile?: string, view?: RegistryCatalogInputView): readonly TaxonomyGeneratorNodeRecord[] {
  return generatorInputPaths(inventory, contract, taxonomy, cancelFile, view).map((path) => generatorNodeRecord(inventory.repoRoot, path, taxonomy));
}

export function previewNodeRecords(manifest: TaxonomyGeneratorPreviewManifest): readonly TaxonomyGeneratorNodeRecord[] {
  return manifest.nodes.map((node) => {
    if (node.nodeKind === "directory") return { path: node.path, nodeKind: "directory" as const, contentHash: sha256("directory"), mode: node.mode };
    const bytes = Buffer.from(node.bytesBase64, "base64");
    return { path: node.path, nodeKind: "file" as const, contentHash: sha256(bytes), mode: node.mode, size: bytes.byteLength };
  });
}

export function validatePreviewPreState(manifest: TaxonomyGeneratorPreviewManifest, preOutputs: readonly TaxonomyGeneratorNodeRecord[]): void {
  const expected = new Set(manifest.nodes.map((node) => node.path));
  const prePaths = new Set(preOutputs.map((node) => node.path));
  for (const stale of manifest.staleRemovals) if (![...prePaths].some((path) => path === stale || path.startsWith(`${stale}/`))) throw new Error(`Generator preview stale removal does not exist in the output pre-state: ${stale}`);
  for (const path of prePaths) if (!expected.has(path) && !manifest.staleRemovals.some((stale) => path === stale || path.startsWith(`${stale}/`))) throw new Error(`Generator preview omits stale output from staleRemovals: ${path}`);
}

export interface GeneratorPlanningResult {
  readonly regenerations: readonly TaxonomyRegeneration[];
  readonly violations: readonly TaxonomyViolation[];
}
