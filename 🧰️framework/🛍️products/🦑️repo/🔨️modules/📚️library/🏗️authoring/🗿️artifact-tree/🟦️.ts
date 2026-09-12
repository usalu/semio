import { existsSync, mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { authorArtifactScaffold, type ArtifactScaffoldLeaf, type ArtifactScaffoldOptions } from "../../🏗️builder/🟦️.ts";
import { canonicalFilenameForKind, canonicalPrimaryFilenameForKind, loadCatalogTaxonomy as loadTaxonomy } from "../../🔍️discovery/🟦️.ts";
import { policyStripEmoji } from "../../🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts";
import { newScaffoldEmptyFacetMarkdown, newScaffoldRustLeaf, newScaffoldTsLeaf } from "../🧱️contract/🟦️.ts";

export function newScaffoldWriteIfAbsent(repoRoot: string, relPath: string, content: string, created: string[], skipped: string[], dryRun: boolean): void {
  const abs = join(repoRoot, relPath);
  if (existsSync(abs)) {
    skipped.push(relPath);
    return;
  }
  if (dryRun) {
    created.push(relPath);
    return;
  }
  mkdirSync(dirname(abs), { recursive: true });
  try {
    writeFileSync(abs, content, { flag: "wx" });
    created.push(relPath);
  } catch (error) {
    if ((error as { code?: string }).code !== "EEXIST") throw error;
    skipped.push(relPath);
  }
}

/** 🔎️ Resolves a bare CLI id to the real emoji-prefixed child directory name of `parentAbs`. */
export function newScaffoldIoTree(ioRel: string, taxonomy: ReturnType<typeof loadTaxonomy>, leaves: ArtifactScaffoldLeaf[]): void {
  const rustLeaf = canonicalPrimaryFilenameForKind(taxonomy.componentFileKinds["🦀️rust"]!, taxonomy);
  const typescriptLeaf = canonicalPrimaryFilenameForKind(taxonomy.componentFileKinds["🟦️typescript"]!, taxonomy);
  const emptyMarker = canonicalFilenameForKind(taxonomy.windowEmptyFacetFileKindId, taxonomy);
  leaves.push({ path: `${ioRel}/${rustLeaf}`, content: newScaffoldRustLeaf("io root (io() -> IoDeclaration stub)") });
  leaves.push({ path: `${ioRel}/${typescriptLeaf}`, content: newScaffoldTsLeaf("io root (IoEntryDescriptor[] mirror)") });
  for (const kind of taxonomy.ioSemanticCollectionDirNames ?? []) {
    const kindRel = `${ioRel}/${kind}`;
    if (kind === "🧬️mutations" || kind === "💡️inferences") {
      leaves.push({ path: `${kindRel}/${emptyMarker}`, content: newScaffoldEmptyFacetMarkdown(kind) });
      continue;
    }
    for (const rep of taxonomy.representationDirs ?? []) {
      leaves.push({ path: `${kindRel}/${rep}/${rustLeaf}`, content: newScaffoldRustLeaf(`${kind}/${rep} native codec`) });
    }
  }
}

export function newScaffoldSubsetTree(repoRoot: string, subsetRel: string, taxonomy: ReturnType<typeof loadTaxonomy>, dryRun: boolean, options: ArtifactScaffoldOptions = {}): { created: string[]; skipped: string[] } {
  const leaves: ArtifactScaffoldLeaf[] = [];
  const rustLeaf = canonicalPrimaryFilenameForKind(taxonomy.componentFileKinds["🦀️rust"]!, taxonomy);
  const typescriptLeaf = canonicalPrimaryFilenameForKind(taxonomy.componentFileKinds["🟦️typescript"]!, taxonomy);
  const emptyMarker = canonicalFilenameForKind(taxonomy.windowEmptyFacetFileKindId, taxonomy);
  leaves.push({ path: `${subsetRel}/${rustLeaf}`, content: newScaffoldRustLeaf("subset root (subset() -> SubsetDeclaration stub; mounts schema/io/viewer/editor/examples)") });
  leaves.push({ path: `${subsetRel}/${typescriptLeaf}`, content: newScaffoldTsLeaf("subset root") });
  leaves.push({ path: `${subsetRel}/🧬️schema/${rustLeaf}`, content: newScaffoldRustLeaf("schema root — own Snapshot/Diff/Mutation types, no codecs") });
  leaves.push({ path: `${subsetRel}/🧬️schema/${typescriptLeaf}`, content: newScaffoldTsLeaf("schema root") });
  newScaffoldIoTree(`${subsetRel}/🚪️io`, taxonomy, leaves);
  for (const role of taxonomy.surfaceRoles) {
    const surfaceRel = `${subsetRel}/${taxonomy.surfaceDirNames[role]}`;
    leaves.push({ path: `${surfaceRel}/${rustLeaf}`, content: newScaffoldRustLeaf(`${role} surface`) });
    leaves.push({ path: `${surfaceRel}/${typescriptLeaf}`, content: newScaffoldTsLeaf(`${role} surface`) });
  }
  leaves.push({ path: `${subsetRel}/📚️examples/${emptyMarker}`, content: newScaffoldEmptyFacetMarkdown("examples") });
  return authorArtifactScaffold(repoRoot, { kind: "subset", subsetPath: subsetRel }, leaves, taxonomy, { ...options, dryRun });
}

export function newScaffoldStandardTree(repoRoot: string, standardRel: string, dryRun: boolean): { created: string[]; skipped: string[] } {
  const created: string[] = [];
  const skipped: string[] = [];
  const taxonomy = loadTaxonomy();
  const rustLeaf = canonicalPrimaryFilenameForKind(taxonomy.componentFileKinds["🦀️rust"]!, taxonomy);
  const typescriptLeaf = canonicalPrimaryFilenameForKind(taxonomy.componentFileKinds["🟦️typescript"]!, taxonomy);
  const subsetsManifest = canonicalFilenameForKind(taxonomy.subsetsManifestFileKindId, taxonomy);
  newScaffoldWriteIfAbsent(repoRoot, `${standardRel}/${rustLeaf}`, newScaffoldRustLeaf("standard root (standard() -> StandardDeclaration stub; mounts subsets)"), created, skipped, dryRun);
  newScaffoldWriteIfAbsent(repoRoot, `${standardRel}/${typescriptLeaf}`, newScaffoldTsLeaf("standard root"), created, skipped, dryRun);
  const manifest = { standard: policyStripEmoji(standardRel.split("/").pop() ?? ""), subsets: { "*": {} } };
  newScaffoldWriteIfAbsent(repoRoot, `${standardRel}/🪆️subsets/${subsetsManifest}`, `${JSON.stringify(manifest, null, 2)}\n`, created, skipped, dryRun);
  return { created, skipped };
}

export function newScaffoldArtifactTree(repoRoot: string, artRel: string, dryRun: boolean): { created: string[]; skipped: string[] } {
  const created: string[] = [];
  const skipped: string[] = [];
  const taxonomy = loadTaxonomy();
  const rustLeaf = canonicalPrimaryFilenameForKind(taxonomy.componentFileKinds["🦀️rust"]!, taxonomy);
  const typescriptLeaf = canonicalPrimaryFilenameForKind(taxonomy.componentFileKinds["🟦️typescript"]!, taxonomy);
  newScaffoldWriteIfAbsent(repoRoot, `${artRel}/${rustLeaf}`, newScaffoldRustLeaf("artifact root (artifact() -> ArtifactDeclaration stub; mounts standards)"), created, skipped, dryRun);
  newScaffoldWriteIfAbsent(repoRoot, `${artRel}/${typescriptLeaf}`, newScaffoldTsLeaf("artifact root"), created, skipped, dryRun);
  return { created, skipped };
}
