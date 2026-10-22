import { receiveGeneratorPreviewProgressV1 } from "../../🏭️generator/👁️preview/📈️progress/🟦️.ts";
import { report } from "../🏃️operation/🟦️.ts";
import { type LoadedTaxonomy } from "../🔣️taxonomy/📚️loaded/🟦️.ts";
import { type GeneratorContractSpec } from "../🔣️taxonomy/🏭️generators-packages/🟦️.ts";
import { loadTaxonomy } from "../🔣️taxonomy/🟦️.ts";
import { normalizeRelative, splitLeadingEmoji, inScope, utf8Compare } from "../🛣️path/🟦️.ts";
import { sha256, assertLexicalInputOutsideOpaque, lstatOrNull } from "../📁️input/🟦️.ts";
import { sourceAdmissionPrepareOptions, sourceAdmissionCheckCancellation, collectTaxonomySourceAdmission, sourceAdmissionScopedOptions } from "../🚪️source-admission/📁️io/🟦️.ts";
import { spawnSync } from "node:child_process";
import { availableParallelism } from "node:os";
import { readFileSync, readlinkSync } from "node:fs";
import { canonicalJson } from "../../🧾️serialization/🔣️json/🟦️.ts";
import { generatorPreviewResourceLimits } from "../../🔍️discovery/🛠️generator-commands/🟦️.ts";
import { fixedSourceDispositionDecision } from "../../🔍️discovery/🔭️repository-scan/🟦️.ts";
import { parseGeneratorInputProjection, type GeneratorInputProjection } from "../../🔍️discovery/📇️registry-catalog-inputs/🟦️.ts";
import { cargoPackageRootBuildScriptPath } from "../../🔍️discovery/📄️implementation-findings/🟦️.ts";
import { basename, dirname, isAbsolute, posix } from "node:path";
import { recordKey } from "../../⚡️caching/🗃️records/🟦️.ts";
import { type ReferenceEdit, type TaxonomyCapturedSourceRead, type TaxonomyEvidenceRemoval, type TaxonomyGeneratorPreviewManifest, type TaxonomyInventory, type TaxonomyInventoryEntry, type TaxonomyInventoryOptions, type TaxonomyMove, requireTaxonomyValue } from "../📜️plan-contracts/🟦️.ts";
import { type ContentKindHint, absolutePath, areaId, checkCancellation, extensionlessContentKind, isExcluded, ownerId, stableViolations, violation } from "../🏷️file-kind-matching/🟦️.ts";
import { generatorPathCompare, parseGeneratorPreviewManifest } from "../🧾️plan-parsing/🟦️.ts";
import { applyEditsToContent } from "../✂️reference-edits/🟦️.ts";
import { type LeafStamp, inventoryEntriesByNormalizedPath, inventoryLeafStamps, referenceEditsByPath } from "../🧱️inventory-leaf-stamps/🟦️.ts";
import { type CandidatePath, type MutableInventoryEntry, TEXT_LIMIT_BYTES, contentOf, leafContentOf, packageLocation, storedClassification } from "../📸️repository-capture/🟦️.ts";
import { ancestorDirectoryKindIds, normalizeMutationCasePairs, pathPolicyViolations, provenMutationLeafOwners, sourceTreeDigest, validateMutationCasePairs } from "../👥️mutation-projection/🟦️.ts";
import { canonicalDirectory, canonicalFile, classifyPackageRole, packageImplementationAdmittedAt, directoryHash, inventoryDigestOf, matchingFixedContracts, packageImplementationDestination } from "../📏️fixed-contracts/🟦️.ts";
import { projectNestedCargoPackages, validateMutationPayloadSchemas, validateTicketImportantDirectories } from "../🗺️artifact-projection-catalogs/🟦️.ts";
import { assertGeneratorPreviewTarget } from "../🧐️preimage-assertions/🟦️.ts";
import { projectArtifactCatalogs, projectArtifactEmptyFacetFiles, projectExactOwnedFiles, projectTicketDocumentFiles, projectTicketImportantFiles } from "../📂️owned-file-projection/🟦️.ts";
import { type TaxonomyRepositoryCapture, inheritReferenceInventoryContext, inventoryCaptures, referenceInventoryContexts } from "../🔗️reference-inventory/🟦️.ts";
import { normalizationRevision, referenceTokenStore, textualPath } from "../🧮️reference-digests/🟦️.ts";
import { referenceGraph } from "../🕸️reference-graph/🟦️.ts";

export function inventoryWithoutTransactionEvidence(inventory: TaxonomyInventory, transactionRoot: string, exactPlanArtifactPath?: string): TaxonomyInventory {
  const suppressed = (path: string): boolean => path === exactPlanArtifactPath || path === transactionRoot || path.startsWith(`${transactionRoot}/`);
  const entries = inventory.entries
    .filter((entry) => !suppressed(entry.sourcePath))
    .map((entry) => ({ ...entry, referencesIn: entry.referencesIn.filter((path) => !suppressed(path)), referencesOut: entry.referencesOut.filter((path) => !suppressed(path)) }));
  return inheritReferenceInventoryContext(inventory, { ...inventory, entries, violations: inventory.violations.filter((entry) => !suppressed(entry.path)), sourceTreeDigest: sourceTreeDigest(entries) }, transactionRoot, exactPlanArtifactPath);
}

/** 🧱️ Inventories Git-index paths and explicitly admitted ticket paths without traversing opaque exclusions or following symlinks. */
export function inventoryTaxonomy(options: TaxonomyInventoryOptions): TaxonomyInventory {
  return inventoryTaxonomyWithSourceParentPruning(options, new Set());
}

/** 📥️ Inventories admitted physical paths using an explicit captured leaf-byte provider. */
export function inventoryTaxonomyWithCapturedSourceRead(options: TaxonomyInventoryOptions, sourceRead: TaxonomyCapturedSourceRead): TaxonomyInventory {
  return inventoryTaxonomyWithSourceParentPruning(options, new Set(), sourceRead);
}

/** 🪵️ Projects only transaction-proven empty source parents into package authority before final classification. */
export function inventoryTaxonomyWithSourceParentPruning(options: TaxonomyInventoryOptions, prunableSourceParents: ReadonlySet<string>, capturedRead?: TaxonomyCapturedSourceRead, capture?: TaxonomyRepositoryCapture): TaxonomyInventory {
  const sourceRead = capturedRead ?? ((path: string) => readFileSync(path)), stamps = capturedRead ? undefined : new Map<string, LeafStamp>();
  const request = { ...options };
  options = Object.freeze({ ...request, structuralDirectoryNames: request.structuralDirectoryNames && Object.freeze([...request.structuralDirectoryNames]) });
  const prepared = capture && options.scope !== undefined ? sourceAdmissionScopedOptions(capture.source, options.scope) : sourceAdmissionPrepareOptions(options), { repoRoot, scope } = prepared;
  report(options.progress, "inventory", "setup", 0, 1, scope);
  const leafStore = capturedRead ? null : referenceTokenStore(repoRoot);
  if (options.workers !== undefined && (!Number.isSafeInteger(options.workers) || options.workers < 1)) throw new Error("workers must be a positive integer");
  const taxonomy = capture?.taxonomy ?? loadTaxonomy({ repoRoot, taxonomyPath: prepared.taxonomyPath });
  if (scope && isExcluded(scope, taxonomy)) throw new Error(`Inventory scope is opaque: ${scope}`);
  sourceAdmissionCheckCancellation(repoRoot, prepared.cancelFile, prepared.repositoryFences);
  report(options.progress, "inventory", "setup", 1, 1, scope);
  const activeExclusions: string[] = [];
  const collectedSourceAdmission = collectTaxonomySourceAdmission(taxonomy, prepared);
  const sourceAdmission = collectedSourceAdmission.inventory;
  const blockingAdmission = sourceAdmission.diagnostics.filter((row) => row.code !== "tracked-path-absent");
  if (blockingAdmission.length > 0) throw new Error(`Source admission rejected: ${blockingAdmission.map((row) => `${row.code}:${row.path}`).join(", ")}`);
  const repositoryBoundary = sourceAdmission.observations.find((row) => row.repositoryBoundary === "gitlink");
  if (repositoryBoundary) throw new Error(`Normalization requires an explicit repository-boundary decision before authored classification: ${repositoryBoundary.sourcePath}`);
  const admitted = new Map<string, CandidatePath>();
  for (const row of sourceAdmission.observations) {
    if (!["file", "directory", "symlink"].includes(row.observedKind)) continue;
    const stageZero = row.indexEntries.find((entry) => entry.stage === 0);
    admitted.set(row.sourcePath, { path: row.sourcePath, mode: row.worktreeMode ?? (row.observedKind === "directory" ? "040000" : row.observedKind === "symlink" ? "120000" : "100644"), objectId: stageZero?.objectId, explicitDirectory: row.explicitDirectory });
  }
  const directoryPaths = new Set<string>();
  for (const row of admitted.values()) {
    if (row.explicitDirectory || row.mode === "040000") directoryPaths.add(row.path);
    let parent = dirname(row.path);
    while (parent && parent !== ".") {
      if (inScope(parent, scope)) directoryPaths.add(parent);
      parent = dirname(parent);
    }
  }
  const entries = new Map<string, MutableInventoryEntry>();
  const canonicalDirectoryByPath = new Map<string, string>();
  const directoryKindByPath = new Map<string, string>();
  const fixedDirectoryContractByPath = new Map<string, string>();
  const directories = [...directoryPaths].sort((a, b) => a.split("/").length - b.split("/").length || utf8Compare(a, b));
  const mutationLeaves = provenMutationLeafOwners(repoRoot, directories, admitted, sourceRead, taxonomy);
  report(options.progress, "inventory", "directories", 0, directories.length);
  for (let index = 0; index < directories.length; index++) {
    const path = requireTaxonomyValue(directories[index],"directories[index]");
    checkCancellation(repoRoot, options.cancelFile);

    const parentCanonical = canonicalDirectoryByPath.get(dirname(path)) ?? "";
    const parentContextId = directoryKindByPath.get(dirname(path)) ?? fixedDirectoryContractByPath.get(dirname(path));
    const canonical = canonicalDirectory(path, parentCanonical, parentContextId, ancestorDirectoryKindIds(path, directoryKindByPath), taxonomy, mutationLeaves);
    canonicalDirectoryByPath.set(path, canonical.path);
    if (canonical.kindId) directoryKindByPath.set(path, canonical.kindId);
    if (canonical.fixedId) fixedDirectoryContractByPath.set(path, canonical.fixedId);
    entries.set(path, {
      sourcePath: path,
      normalizedPath: canonical.path,
      nodeKind: "directory",
      ownerId: ownerId(path),
      areaId: areaId(path),
      fileKind: null,
      semanticStem: splitLeadingEmoji(basename(path)).rest || null,
      fixedContractId: canonical.fixedId,
      contentHash: "",
      referencesIn: [],
      referencesOut: [],
      violations: [...canonical.violations, ...pathPolicyViolations(canonical.path, taxonomy)],
      mode: (lstatOrNull(absolutePath(repoRoot, path))?.mode ?? 0) & 0o7777,
      size: 0,
    });
    report(options.progress, "inventory", "directories", index + 1, directories.length, path);
  }
  checkCancellation(repoRoot, options.cancelFile);
  const leaves = [...admitted.values()].filter((row) => row.mode !== "040000" && !row.explicitDirectory).sort((a, b) => utf8Compare(a.path, b.path));
  report(options.progress, "inventory", "files", 0, leaves.length);
  const leafPaths = new Set(leaves.map((row) => row.path));
  const authorizedCargoBuildScriptPaths = new Set<string>();
  for (const manifest of leaves.filter((row) => basename(row.path) === "Cargo.toml")) {
    const location = packageLocation(manifest.path, taxonomy);
    if (!location || location.ecosystemId !== "🦀️rust" || dirname(manifest.path) !== location.packageRoot) continue;
    const content = contentOf(repoRoot, manifest, sourceRead);
    if (content.kind !== "file" || !content.bytes || cargoPackageRootBuildScriptPath(new TextDecoder("utf-8", { fatal: false }).decode(content.bytes)) !== "build.rs") continue;
    const buildScript = `${location.packageRoot}/build.rs`;
    if (leafPaths.has(buildScript)) authorizedCargoBuildScriptPaths.add(buildScript);
  }
  const siblingFixedFilenameContractIdsByParent = new Map<string, readonly string[]>();
  const siblingIds = new Map<string, Set<string>>();
  for (const row of leaves) {
    const parent = dirname(row.path);
    const fixed = matchingFixedContracts(row.path, taxonomy.schema.fixedFilenameContracts, taxonomy, packageLocation(row.path, taxonomy), directoryKindByPath.get(parent), fixedDirectoryContractByPath.get(parent));
    if (!fixed.selected) continue;
    const ids = siblingIds.get(parent) ?? new Set<string>();
    ids.add(fixed.selected[0]);
    siblingIds.set(parent, ids);
  }
  for (const [parent, ids] of siblingIds) siblingFixedFilenameContractIdsByParent.set(parent, [...ids].sort(generatorPathCompare));
  for (let index = 0; index < leaves.length; index++) {
    const row = requireTaxonomyValue(leaves[index],"leaves[index]");
    checkCancellation(repoRoot, options.cancelFile);

    const content = leafContentOf(repoRoot, row, sourceRead, leafStore);
    const parent = dirname(row.path) === "." ? "" : dirname(row.path);
    const contentKind: ContentKindHint = content.kind === "file" ? extensionlessContentKind(row.path, basename(row.path).includes(".") ? undefined : content.bytes(), taxonomy) : { kindId: null };
    const parentContextId = directoryKindByPath.get(parent) ?? fixedDirectoryContractByPath.get(parent);
    const canonical = canonicalFile(row.path, canonicalDirectoryByPath.get(parent) ?? "", parentContextId, ancestorDirectoryKindIds(row.path, directoryKindByPath), directoryKindByPath, fixedDirectoryContractByPath, siblingFixedFilenameContractIdsByParent, authorizedCargoBuildScriptPaths, taxonomy, contentKind.kindId ?? undefined);
    const violations = [...canonical.violations];
    if (content.violation) violations.push(content.violation);
    if (contentKind.violation && !canonical.fixedId) violations.push(contentKind.violation);
    const textual = content.kind === "file" && content.violation === undefined && content.size <= TEXT_LIMIT_BYTES && (textualPath(row.path) || (contentKind.kindId !== null && contentKind.kindId !== "binary")) && content.utf8() === true;
    let decoded: string | undefined;
    const text = (): string | null => textual ? decoded ??= new TextDecoder("utf-8").decode(content.bytes()) : null;
    if (stamps && content.stat && content.observedAt !== undefined) stamps.set(row.path, { dev: content.stat.dev, ino: content.stat.ino, size: content.stat.size, mtimeMs: content.stat.mtimeMs, ctimeMs: content.stat.ctimeMs, mode: content.mode, observedAt: content.observedAt, utf8: textual });
    const fixedSource = canonical.fixedId ? storedClassification(leafStore, () => recordKey("taxonomy-fixed-source-v1", normalizationRevision(), taxonomy.input.contentHash, canonical.fixedId!, content.hash, String(textual)), () => fixedSourceDispositionDecision(canonical.fixedId, text(), taxonomy.discoverySchema)) : fixedSourceDispositionDecision(canonical.fixedId, null, taxonomy.discoverySchema);
    if (fixedSource?.finding === "fixed-source-content-unreadable") violations.push(violation(fixedSource.finding, row.path, `Fixed source contract ${fixedSource.contractId} could not read a UTF-8 body for its declared semantic validator`));
    if (fixedSource?.finding === "fixed-source-disposition-unresolved") violations.push(violation(fixedSource.finding, row.path, `Fixed source contract ${fixedSource.contractId} rejected the body with its declared semantic validator`));
    const admitted = packageImplementationAdmittedAt(row.path, canonical.fileKind, taxonomy, (candidate) => leafPaths.has(candidate));
    const role = packageLocation(row.path, taxonomy) ? storedClassification(leafStore, () => recordKey("taxonomy-package-role-v2", normalizationRevision(), taxonomy.input.contentHash, row.path, canonical.fileKind ?? "", canonical.fixedId ?? "", content.hash, String(textual), String(admitted)), () => classifyPackageRole(row.path, canonical.fileKind, canonical.fixedId, text(), taxonomy, admitted)) : classifyPackageRole(row.path, canonical.fileKind, canonical.fixedId, null, taxonomy);
    let normalizedPath = canonical.path;
    if (role === "implementation") {
      const extracted = packageImplementationDestination(row.path, canonical, canonicalDirectoryByPath, directoryKindByPath, taxonomy);
      if (extracted) {
        normalizedPath = extracted;
        violations.push(violation("package-implementation-file", row.path, `Package implementation must move to ${extracted}`, "warning"));
      } else violations.push(violation("package-implementation-destination-unresolved", row.path, "Package implementation has no deterministic semantic owner"));
    }
    if (role === "unresolved") violations.push(violation("package-role-unresolved", row.path, "Package role cannot be proven by the configured glue grammar"));
    violations.push(...pathPolicyViolations(normalizedPath, taxonomy));
    if (content.kind === "symlink") {
      try {
        const target = readlinkSync(absolutePath(repoRoot, row.path));
        if (isAbsolute(target)) violations.push(violation("symlink-absolute-target", row.path, "Absolute symlink target cannot be proven repository-local"));
        else {
          const lexicalTarget = normalizeRelative(posix.join(dirname(row.path), target.replaceAll("\\", "/")));
          if (isExcluded(lexicalTarget, taxonomy)) violations.push(violation("symlink-opaque-boundary", row.path, `Symlink lexically targets opaque path ${lexicalTarget}`));
        }
      } catch (error) {
        violations.push(violation("symlink-target-unreadable", row.path, error instanceof Error ? error.message : String(error)));
      }
    }
    entries.set(row.path, {
      sourcePath: row.path,
      normalizedPath,
      nodeKind: content.kind,
      ownerId: ownerId(row.path),
      areaId: areaId(row.path),
      fileKind: canonical.fileKind,
      semanticStem: canonical.stem,
      fixedContractId: canonical.fixedId,
      packageRole: role,
      contentHash: content.hash,
      referencesIn: [],
      referencesOut: [],
      violations,
      mode: content.mode,
      size: content.size,
      symlinkTarget: content.symlinkTarget,
    });
    report(options.progress, "inventory", "files", index + 1, leaves.length, row.path);
  }
  checkCancellation(repoRoot, options.cancelFile);
  projectExactOwnedFiles(repoRoot, entries, taxonomy);
  projectArtifactEmptyFacetFiles(repoRoot, entries, taxonomy);
  projectTicketDocumentFiles(repoRoot, entries, fixedDirectoryContractByPath, taxonomy);
  projectTicketImportantFiles(repoRoot, entries, fixedDirectoryContractByPath, taxonomy);
  validateTicketImportantDirectories(repoRoot, entries, directoryKindByPath, fixedDirectoryContractByPath);
  normalizeMutationCasePairs(repoRoot, scope, entries, directoryKindByPath, taxonomy);
  validateMutationPayloadSchemas(repoRoot, entries, taxonomy);
  validateMutationCasePairs(repoRoot, scope, entries, directoryKindByPath, taxonomy);
  projectArtifactCatalogs(repoRoot, entries, taxonomy);
  projectNestedCargoPackages(repoRoot, entries, taxonomy, prunableSourceParents);
  const childrenByParent = new Map<string, MutableInventoryEntry[]>();
  for (const entry of entries.values()) {
    const parent = dirname(entry.sourcePath);
    const children = childrenByParent.get(parent) ?? [];
    children.push(entry);
    childrenByParent.set(parent, children);
  }
  for (const path of [...directoryPaths].sort((a, b) => b.split("/").length - a.split("/").length || b.localeCompare(a))) {
    const entry = entries.get(path);
    if (entry) entry.contentHash = directoryHash(path, childrenByParent.get(path) ?? []);
  }
  referenceGraph(repoRoot, entries, taxonomy, options.progress, options.cancelFile, leafStore, options.workers ?? Math.max(1, Math.min(8, availableParallelism() - 1)));
  const frozenEntries: TaxonomyInventoryEntry[] = [...entries.values()]
    .sort((a, b) => utf8Compare(a.sourcePath, b.sourcePath))
    .map((entry) => ({
      sourcePath: entry.sourcePath,
      normalizedPath: entry.normalizedPath,
      nodeKind: entry.nodeKind,
      ownerId: entry.ownerId,
      areaId: entry.areaId,
      fileKind: entry.fileKind,
      semanticStem: entry.semanticStem,
      fixedContractId: entry.fixedContractId,
      packageRole: entry.packageRole,
      contentHash: entry.contentHash,
      mode: entry.mode,
      size: entry.size,
      symlinkTarget: entry.symlinkTarget,
      referencesIn: [...entry.referencesIn],
      referencesOut: [...entry.referencesOut],
      violations: stableViolations(entry.violations),
    }));
  const violations = stableViolations(frozenEntries.flatMap((entry) => entry.violations));
  const sourceDigest = sourceTreeDigest(frozenEntries);
  const partial = {
    schemaVersion: 1 as const,
    taxonomySchemaVersion: 7 as const,
    scope,
    pathExclusions: taxonomy.exclusions.map((entry) => entry.path),
    activePathExclusions: activeExclusions,
    entries: frozenEntries,
    violations,
    sourceTreeDigest: sourceDigest,
  };
  const inventory: TaxonomyInventory = {
    ...partial,
    repoRoot,
    taxonomyPath: taxonomy.path,
    inventoryDigest: inventoryDigestOf(partial),
  };
  referenceInventoryContexts.set(inventory, {
    ticketDir: options.ticketDir,
    transactionRoots: [],
    exactEvidencePaths: [],
    sourceAdmission: Object.freeze({
      state: "captured" as const,
      retained: Object.freeze({
        originInventory: inventory,
        originalInputText: collectedSourceAdmission.inputText,
        sourceInventoryText: JSON.stringify(sourceAdmission),
        originSourceTreeDigest: inventory.sourceTreeDigest,
        originInventoryDigest: inventory.inventoryDigest,
      }),
    }),
  });
  if (capture) inventoryCaptures.set(inventory, capture);
  if (stamps) inventoryLeafStamps.set(inventory, stamps);
  report(options.progress, "inventory", "complete", frozenEntries.length, frozenEntries.length);
  return inventory;
}

export function generatorPreviewProjection(contractId: GeneratorInputProjection["contractId"], inventory: TaxonomyInventory, moves: readonly TaxonomyMove[], edits: readonly ReferenceEdit[], removals: readonly TaxonomyEvidenceRemoval[], taxonomy: LoadedTaxonomy, cancelFile?: string): GeneratorInputProjection {
  const normalizedEntries = inventoryEntriesByNormalizedPath(inventory), editsAt = referenceEditsByPath(edits), moveByDestination = new Map<string, TaxonomyMove>();
  for (const move of moves) if (!moveByDestination.has(move.destinationPath)) moveByDestination.set(move.destinationPath, move);
  const projectedEdits = [...editsAt.keys()].sort(generatorPathCompare).map((path) => {
    checkCancellation(inventory.repoRoot, cancelFile);
    const sourcePath = moveByDestination.get(path)?.sourcePath ?? normalizedEntries.get(path)?.sourcePath ?? path;
    const absolute = assertLexicalInputOutsideOpaque(inventory.repoRoot, sourcePath, "Registry projected edit", true);
    const bytes = applyEditsToContent(readFileSync(absolute, "utf8"), editsAt.get(path)!);
    return { path, bytesBase64: Buffer.from(bytes).toString("base64") };
  });
  return parseGeneratorInputProjection(canonicalJson({ contractId, schemaVersion: 1, moves: moves.map((move) => ({ sourcePath: move.sourcePath, destinationPath: move.destinationPath, nodeKind: move.sourcePreimage.nodeKind })), edits: projectedEdits, removals: removals.map((removal) => removal.sourcePath) }), taxonomy.discoverySchema, contractId);
}

export function invokeGeneratorPreview(inventory: TaxonomyInventory, id: string, contract: GeneratorContractSpec, taxonomy: LoadedTaxonomy, projection?: GeneratorInputProjection, cancelFile?: string): { readonly manifest: TaxonomyGeneratorPreviewManifest; readonly digest: string } {
  if (!contract.ownerPath || !contract.previewTarget) throw new Error(`Owned generator ${id} has no preview target`);
  const execution = assertGeneratorPreviewTarget(inventory.repoRoot, contract);
  checkCancellation(inventory.repoRoot, cancelFile);
  const protocol = contract.inputDiscovery?.previewInput ?? contract.packageGeneration?.previewInput;
  const input = projection ? canonicalJson(projection) + "\n" : undefined;
  if (input && (!protocol || Buffer.byteLength(input) > protocol.maxBytes)) throw new Error(`Generator ${id} projected input exceeds its declared byte limit`);
  const cancellationPath = cancelFile ? assertLexicalInputOutsideOpaque(inventory.repoRoot, cancelFile, "Generator preview cancellation", true) : "";
  const limits = generatorPreviewResourceLimits(contract), started = performance.now(), duration = contract.previewProgress ? Math.min(limits.timeoutMs, contract.previewProgress.maxDurationMs) : limits.timeoutMs;
  const result = spawnSync(execution.command, execution.args, { cwd: absolutePath(inventory.repoRoot, execution.cwd), encoding: "utf8", input, maxBuffer: limits.maxOutputBytes, timeout: duration, env: { ...process.env, REPO_ROOT: inventory.repoRoot, SEMIO_GENERATOR_PREVIEW_PROGRESS: contract.previewProgress ? JSON.stringify(contract.previewProgress) : "", SEMIO_GENERATOR_PREVIEW: "1", SEMIO_GENERATOR_PREVIEW_PROTOCOL: projection ? protocol!.protocol : "", SEMIO_GENERATOR_PREVIEW_CANCEL_FILE: cancellationPath } });
  checkCancellation(inventory.repoRoot, cancelFile);
  const stdout = result.stdout ?? "", stderr = result.stderr ?? "";
  if (result.error || result.status !== 0 || result.signal !== null) throw new Error(`Generator preview command failed for ${id}: status=${result.status ?? -1}${result.error ? `, reason=${result.error.message}` : ""}, stdout=${sha256(stdout)}, stderr=${sha256(stderr)}`, { cause: result.error });
  receiveGeneratorPreviewProgressV1(stderr, contract.previewProgress, { cancelled: () => { try { checkCancellation(inventory.repoRoot, cancelFile); return false; } catch { return true; } }, remainingMs: () => duration - (performance.now() - started) });
  const roots = contract.outputRoots.map((root) => root.path).sort(generatorPathCompare);
  const manifest = parseGeneratorPreviewManifest(stdout, id, roots, taxonomy.exclusions.map((entry) => entry.path));
  return { manifest, digest: sha256(stdout) };
}
