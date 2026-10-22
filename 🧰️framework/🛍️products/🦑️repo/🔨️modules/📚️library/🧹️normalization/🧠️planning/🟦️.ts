import { report } from "../🏃️operation/🟦️.ts";
import { type LoadedTaxonomy } from "../🔣️taxonomy/📚️loaded/🟦️.ts";
import { loadTaxonomy } from "../🔣️taxonomy/🟦️.ts";
import { normalizeRelative, inScope } from "../🛣️path/🟦️.ts";
import { sha256 } from "../📁️input/🟦️.ts";
import { canonicalJson } from "../../🧾️serialization/🔣️json/🟦️.ts";
import { registryCatalogPathMayAffect } from "../../🔍️discovery/🧲️registry-catalog-projection/🟦️.ts";
import { registryCatalogInputView } from "../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { implementationLeafBasenameFinding } from "../../🔍️discovery/🗂️file-kind-resolution/🟦️.ts";
import { basename, dirname, resolve } from "node:path";
import { PLAN_COMMIT_ID, type ReferenceEdit, type TaxonomyEvidenceRemoval, type TaxonomyGeneratorNodeRecord, type TaxonomyInventory, type TaxonomyMove, type TaxonomyPlan, type TaxonomyPlanOptions, type TaxonomyRegeneration, type TaxonomyRemovalAuthority, type TaxonomyViolation, requireTaxonomyValue } from "../📜️plan-contracts/🟦️.ts";
import { checkCancellation, isProperScopeAncestor, stableViolations, violation } from "../🏷️file-kind-matching/🟦️.ts";
import { generatorPathCompare } from "../🧾️plan-parsing/🟦️.ts";
import { type GeneratorPlanningResult, compilerPreviewInputRecords, generatorInputInventory, generatorTreeInventory, previewNodeRecords, validatePreviewPreState } from "../🏭️generator-inputs/🟦️.ts";
import { inventoryEntriesByNormalizedPath, inventoryEntriesBySourcePath } from "../🧱️inventory-leaf-stamps/🟦️.ts";
import { generatorContractsForOutputPath } from "../📸️repository-capture/🟦️.ts";
import { sourceTreeDigest } from "../👥️mutation-projection/🟦️.ts";
import { nestedCargoCatalog } from "../🗺️artifact-projection-catalogs/🟦️.ts";
import { pathsOverlap } from "../🧐️preimage-assertions/🟦️.ts";
import { inventoryCaptures } from "../🔗️reference-inventory/🟦️.ts";
import { destinationAncestorPreimages, plannedAffectedStateDigests, taxonomyPlanDigest } from "../🗃️affected-state/🟦️.ts";
import { projectionStaleViolations } from "../♻️projection-staleness/🟦️.ts";
import { referenceEditCompare } from "../🪞️artifact-reference-projection/🟦️.ts";
import { mutationIndex, packageGeneratorActivated, planEmbeddedTicketRoots, planExactGeneratedOwnerRemovals, planMoveReferenceAuthority, planNestedCargoGeneratedSourceRemovals, planTicketImportantRemovals, planTrailingEvidenceRemovals, sortedSubtree } from "../🛫️embedded-roots/🟦️.ts";
import { generatorPreviewProjection, invokeGeneratorPreview } from "../🔍️inventory-planning/🟦️.ts";
import { planSymlinkTargetEdits } from "../🪢️symlink-target-edits/🟦️.ts";

function generatorPlanning(inventory: TaxonomyInventory, moves: readonly TaxonomyMove[], edits: readonly ReferenceEdit[], taxonomy: LoadedTaxonomy, options: TaxonomyPlanOptions, evidenceRemovals: readonly TaxonomyEvidenceRemoval[] = []): GeneratorPlanningResult {
  const jcoActivation = taxonomy.schema.generatorContracts["jco-package-adapter"]?.projectionActivation;
  const packageCatalog = jcoActivation && moves.some((move) => inScope(move.sourcePath, dirname(jcoActivation.sourceManifestPath))) ? nestedCargoCatalog(inventory.repoRoot, taxonomy.discoverySchema) : null;
  const jco = packageCatalog?.packages.find((row) => row.id === "jcoprobe-guest");
  const preservedLocks = new Set(jco?.mappings.filter((mapping) => basename(mapping.sourcePath) === "Cargo.lock" && moves.some((move) => move.sourcePath === mapping.sourcePath && move.destinationPath === mapping.destinationPath && move.sourcePreimage.contentHash === mapping.sourceHash && move.sourcePreimage.size === mapping.sourceSize) && !edits.some((edit) => edit.path === mapping.destinationPath)).map((mapping) => mapping.destinationPath) ?? []);
  const mutations = new Set<string>();
  const generatedRetirements = evidenceRemovals.filter((entry): entry is TaxonomyEvidenceRemoval & { authority: Extract<TaxonomyRemovalAuthority, { kind: "exact-owner-generated-source" | "nested-cargo-generated-source" }> } => entry.authority.kind === "exact-owner-generated-source" || entry.authority.kind === "nested-cargo-generated-source");
  for (const entry of generatedRetirements) mutations.add(entry.authority.destinationPath);
  for (const move of moves) {
    mutations.add(move.sourcePath);
    mutations.add(move.destinationPath);
  }
  const normalizedEntries = inventoryEntriesByNormalizedPath(inventory), entriesBySource = inventoryEntriesBySourcePath(inventory);
  for (const edit of edits) {
    mutations.add(edit.path);
    const source = normalizedEntries.get(edit.path)?.sourcePath;
    if (source) mutations.add(source);
  }
  const editedPaths = new Set(edits.map((edit) => edit.path)), mutated = mutationIndex(mutations), sortedEntries = [...new Set(inventory.entries.map((entry) => entry.sourcePath))].sort();
  const catalogMutation = [...mutations].some((path) => registryCatalogPathMayAffect(path, taxonomy.discoverySchema));
  const rows: TaxonomyViolation[] = [];
  const regenerations: TaxonomyRegeneration[] = [];
  const capture = inventoryCaptures.get(inventory);
  const catalogView = capture ? (capture.catalogView ??= registryCatalogInputView(inventory.repoRoot, taxonomy.discoverySchema)) : registryCatalogInputView(inventory.repoRoot, taxonomy.discoverySchema);
  const contracts = Object.entries(taxonomy.schema.generatorContracts).sort(([left], [right]) => left.localeCompare(right));
  for (const [index, [id, contract]] of contracts.entries()) {

    if (contract.ownerPath && catalogView.kind(contract.ownerPath) !== "directory") continue;
    const roots = contract.outputRoots.map((root) => root.path).sort(generatorPathCompare);
    const outputEntries = roots.flatMap((root) => [...sortedSubtree(sortedEntries, root)]).map((path) => entriesBySource.get(path)!);
    const outputProblem = outputEntries.some((entry) => !roots.includes(entry.sourcePath) && (entry.sourcePath !== entry.normalizedPath || entry.violations.some((entry) => entry.severity === "error")));
    const outputMutation = roots.some((root) => mutated.overlapping(root, (path) => !(id === "external-cargo-locks" && preservedLocks.has(path))));
    const catalogInputs = contract.inputDiscovery && catalogMutation ? generatorInputInventory(inventory, contract, taxonomy, options.cancelFile, catalogView) : undefined;
    const packageOwnerPath = contract.packageGeneration?.browserProfile.ownerPath;
    const packageOutputVerification = packageOwnerPath && (!inventory.scope || pathsOverlap(inventory.scope, packageOwnerPath));
    const compilerOutputVerification = contract.compilerInputManifest && (!inventory.scope || pathsOverlap(inventory.scope, contract.ownerPath!));
    const patternMutation = contract.inputPatterns.some((pattern) => mutated.matching(pattern, taxonomy.pathMatcher.matches));
    let compilerInputs: readonly TaxonomyGeneratorNodeRecord[] | undefined;
    if (contract.compilerInputManifest && (compilerOutputVerification || outputProblem || outputMutation || patternMutation)) {
      try {
        compilerInputs = generatorInputInventory(inventory, contract, taxonomy, options.cancelFile, catalogView);
      } catch (error) {
        rows.push(violation("generator-compiler-input-manifest-stale", contract.compilerInputManifest.manifestOutputPath, `Generator ${id}: ${error instanceof Error ? error.message : String(error)}`));
        continue;
      }
    }
    const discoveredInputs = catalogInputs ?? compilerInputs;
    const inputMutation = patternMutation || Boolean(discoveredInputs && (discoveredInputs.some((input) => editedPaths.has(input.path)) || moves.some((move) => discoveredInputs.some((input) => input.path === move.sourcePath || input.path === move.destinationPath || input.nodeKind === "directory" && (inScope(move.sourcePath, input.path) || inScope(move.destinationPath, input.path))))));
    if (!outputProblem && !outputMutation && !inputMutation && !packageOutputVerification && !compilerOutputVerification) continue;
    try { if (!packageGeneratorActivated(inventory.repoRoot, moves, contract, taxonomy, evidenceRemovals)) continue; }
    catch (error) { rows.push(violation("generator-activation-invalid", requireTaxonomyValue(roots[0],"roots[0]"), `Generator ${id}: ${error instanceof Error ? error.message : String(error)}`)); continue; }
    const inputs = discoveredInputs ?? generatorInputInventory(inventory, contract, taxonomy, options.cancelFile, catalogView);
    const preOutputs = generatorTreeInventory(inventory.repoRoot, roots, taxonomy);
    const inputDigest = sha256(canonicalJson(inputs));
    const preOutputDigest = sha256(canonicalJson(preOutputs));
    const path = requireTaxonomyValue(roots[0],"roots[0]");
    if (contract.ownership !== "owned") {
      rows.push(violation(`generator-ownership-${contract.ownership}`, path, `Generator contract ${id} is ${contract.ownership}; ${contract.reason}; input ${inputDigest}, output ${preOutputDigest}`));
      continue;
    }
    try {
      checkCancellation(inventory.repoRoot, options.cancelFile);
      const projection = contract.inputDiscovery ? generatorPreviewProjection("plugin-registry", inventory, moves, edits, evidenceRemovals, taxonomy, options.cancelFile) : contract.packageGeneration ? generatorPreviewProjection("wgpu-frame-worker", inventory, moves, edits, evidenceRemovals, taxonomy, options.cancelFile) : undefined;
      const preview = invokeGeneratorPreview(inventory, id, contract, taxonomy, projection, options.cancelFile);
      checkCancellation(inventory.repoRoot, options.cancelFile);
      validatePreviewPreState(preview.manifest, preOutputs);
      const outputs = previewNodeRecords(preview.manifest);
      const freshInputs = [...new Map([...inputs, ...compilerPreviewInputRecords(inventory.repoRoot, contract, taxonomy, preview.manifest)].map(row => [row.path, row])).values()].sort((left, right) => generatorPathCompare(left.path, right.path));
      const retirements = generatedRetirements.filter((entry) => entry.authority.generatorContractId === id);
      for (const retirement of retirements) {
        const generated = outputs.find((output) => output.path === retirement.authority.destinationPath);
        if (!generated || generated.nodeKind !== "file") throw new Error("Generated retirement preview lacks its exact canonical file: " + retirement.sourcePath);
        if (retirement.authority.kind === "exact-owner-generated-source" && (generated.contentHash !== retirement.authority.outputPreimage.contentHash || generated.mode !== retirement.authority.outputPreimage.mode || generated.size !== retirement.authority.outputPreimage.size)) throw new Error("Exact generated owner preview differs from frozen source: " + retirement.sourcePath);
        if (retirement.authority.kind === "nested-cargo-generated-source" && generated.mode !== retirement.authority.sourcePreimage.mode) throw new Error("Nested Cargo generated retirement changes output mode: " + retirement.sourcePath);
      }
      const changed = retirements.length > 0 || canonicalJson(preOutputs) !== canonicalJson(outputs) || preview.manifest.staleRemovals.length > 0;
      if (inputMutation || outputMutation || changed) {
        const command = ["repo", "run", contract.target!] as const;
        const verifyCommand = contract.checkTarget ? (["repo", "run", contract.checkTarget] as const) : undefined;
        const provisional = { contractId: id, cwd: contract.ownerPath!, command, verifyCommand, outputRoots: roots, inputs: freshInputs, preOutputs, outputs, preview: preview.manifest, previewManifestDigest: preview.digest, staleRemovals: preview.manifest.staleRemovals };
        regenerations.push({ id: sha256(canonicalJson(provisional)).slice(0, 24), ...provisional });
      }
      report(options.progress, "plan", "generator-preview", index + 1, contracts.length, id);
    } catch (error) {
      checkCancellation(inventory.repoRoot, options.cancelFile);
      const message = error instanceof Error ? error.message.replaceAll(resolve(inventory.repoRoot), "<repo>") : String(error);
      rows.push(violation("generator-preview-invalid", path, `Generator ${id} preview was rejected: ${message}`));
    }
  }
  return { regenerations: regenerations.sort((left, right) => left.contractId.localeCompare(right.contractId) || left.id.localeCompare(right.id)), violations: stableViolations(rows) };
}

/** 🧭️ Produces a deterministic, fail-closed move and structured-reference plan from an inventory snapshot. */
export function planTaxonomy(inventory: TaxonomyInventory, options: TaxonomyPlanOptions): TaxonomyPlan {
  if (inventory.taxonomySchemaVersion !== 7) throw new Error("Inventory taxonomy schemaVersion must be 7");
  if (inventory.sourceTreeDigest !== sourceTreeDigest(inventory.entries)) throw new Error("Inventory sourceTreeDigest does not match inventory entries");
  const taxonomy = loadTaxonomy({ repoRoot: inventory.repoRoot, taxonomyPath: inventory.taxonomyPath });
  const baselineCommit = options.baselineCommit.trim();
  if (!PLAN_COMMIT_ID.test(baselineCommit)) throw new Error("baselineCommit must be a full lowercase SHA-1 commit ID");
  checkCancellation(inventory.repoRoot, options.cancelFile);
  const embedded = planEmbeddedTicketRoots(inventory, taxonomy);
  const trailingRemovals = planTrailingEvidenceRemovals(inventory);
  const ticketImportantRemovals = planTicketImportantRemovals(inventory, taxonomy);
  const generatedOwnerRemovals = [...planExactGeneratedOwnerRemovals(inventory, taxonomy), ...planNestedCargoGeneratedSourceRemovals(inventory, taxonomy)];
  const ticketImportantSources = new Set(ticketImportantRemovals.removals.map((entry) => entry.sourcePath));
  const evidenceRemovals = [...embedded.removals, ...trailingRemovals.filter((entry) => !ticketImportantSources.has(entry.sourcePath)), ...ticketImportantRemovals.removals, ...generatedOwnerRemovals].sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath));
  const ownedRemovals = new Set(evidenceRemovals.map((entry) => entry.sourcePath));
  const embeddedPrefixes = embedded.roots.map((root) => root.sourceMetadataRoot);
  const isEmbedded = (path: string): boolean => embeddedPrefixes.some((root) => path === root || path.startsWith(`${root}/`));
  const references = planMoveReferenceAuthority(inventory, taxonomy, options, embedded.roots, evidenceRemovals);
  const moves = references.moves;
  const generators = generatorPlanning(inventory, moves, references.edits, taxonomy, options, evidenceRemovals);
  const symlinks = planSymlinkTargetEdits(inventory, taxonomy, options);
  const destinationAncestors = destinationAncestorPreimages(inventory.repoRoot, [...moves.map((entry) => entry.destinationPath), ...embedded.relocations.map((entry) => entry.destinationPath), ...symlinks.edits.map((entry) => entry.finalPath), ...generators.regenerations.flatMap((entry) => entry.outputRoots)]);
  const ownedSymlinks = new Set(symlinks.edits.map((entry) => entry.sourcePath));
  const kindOnlyMoves = new Set(moves.filter((move) => !implementationLeafBasenameFinding(move.destinationPath, taxonomy.discoverySchema)).map((move) => move.sourcePath));
  const unresolved: TaxonomyViolation[] = [
    ...inventory.violations.filter((entry) => entry.severity === "error" && !isProperScopeAncestor(entry.path, inventory.scope) && !isEmbedded(entry.path) && generatorContractsForOutputPath(entry.path, taxonomy).length === 0 && !(entry.code === "symlink-absolute-target" && ownedSymlinks.has(entry.path)) && !(entry.code === "trailing-dot-or-space" && ownedRemovals.has(entry.path)) && !(entry.code === "taxonomy/kind-only-basename" && kindOnlyMoves.has(entry.path))),
    ...references.unresolved,
    ...generators.violations,
    ...symlinks.violations,
    ...embedded.violations,
    ...ticketImportantRemovals.violations,
  ];
  for (const group of references.collisionGroups) if (group.sources.some((source) => !isEmbedded(source))) unresolved.push(violation(`collision-${group.comparison}`, requireTaxonomyValue(group.paths[0],"group.paths[0]") ?? group.sources[0], `Normalization collision ${group.id}: ${group.sources.join(", ")}`));
  for (const digest of options.excludedTreeDigests) {
    if (digest.algorithm !== "sha256-merkle-v1") unresolved.push(violation("opaque-digest-algorithm", digest.relativeRoot, `Unsupported opaque digest algorithm ${digest.algorithm}`));
    if (!inventory.pathExclusions.includes(normalizeRelative(digest.relativeRoot))) unresolved.push(violation("opaque-digest-unregistered", digest.relativeRoot, "Opaque digest is not registered by taxonomy pathExclusions"));
  }
  const affected = plannedAffectedStateDigests(inventory, { moves, embeddedTicketRoots: embedded.roots, embeddedTicketRootRelocations: embedded.relocations, symlinkTargetEdits: symlinks.edits, evidenceRemovals, destinationAncestorPreimages: destinationAncestors, edits: references.edits, regenerations: generators.regenerations }, references.resultHashes, references.resultSizes);
  for (const conflict of affected.conflicts) unresolved.push(violation("affected-path-state-conflict", conflict.path, `Planned operations disagree on the ${conflict.side}-state of this path: ${conflict.rows.join(" | ")}`));
  const provisionalBase: TaxonomyPlan = {
    schemaVersion: 2,
    taxonomySchemaVersion: 7,
    baselineCommit,
    scope: inventory.scope,
    sourceTreeDigest: inventory.sourceTreeDigest,
    excludedTreeDigests: [...options.excludedTreeDigests].sort((a, b) => a.relativeRoot.localeCompare(b.relativeRoot)),
    moves,
    embeddedTicketRoots: embedded.roots,
    embeddedTicketRootRelocations: embedded.relocations,
    symlinkTargetEdits: symlinks.edits,
    evidenceRemovals,
    destinationAncestorPreimages: destinationAncestors,
    edits: [...references.edits].sort(referenceEditCompare),
    regenerations: generators.regenerations,
    unresolved: stableViolations(unresolved),
    expectedAffectedPreStateDigest: affected.pre,
    expectedPostStateDigest: affected.post,
    planDigest: "",
  };
  const provisional: TaxonomyPlan = { ...provisionalBase, unresolved: stableViolations([...provisionalBase.unresolved, ...projectionStaleViolations(inventory.repoRoot, provisionalBase, taxonomy, inventory)]) };
  const plan = { ...provisional, planDigest: taxonomyPlanDigest(provisional) };
  const plannedOperations = moves.length + embedded.roots.length + embedded.relocations.length + symlinks.edits.length + evidenceRemovals.length + references.edits.length + generators.regenerations.length;
  report(options.progress, "plan", "complete", plannedOperations, plannedOperations);
  return plan;
}
