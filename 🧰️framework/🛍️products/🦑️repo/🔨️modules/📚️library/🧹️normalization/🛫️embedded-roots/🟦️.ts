import { type LoadedTaxonomy } from "../🔣️taxonomy/📚️loaded/🟦️.ts";
import { type GeneratorContractSpec } from "../🔣️taxonomy/🏭️generators-packages/🟦️.ts";
import { normalizeRelative, splitLeadingEmoji } from "../🛣️path/🟦️.ts";
import { sha256, assertLexicalInputOutsideOpaque, lstatOrNull } from "../📁️input/🟦️.ts";
import { readFileSync, readlinkSync } from "node:fs";
import type { Stats } from "node:fs";
import { canonicalJson } from "../../🧾️serialization/🔣️json/🟦️.ts";
import { semanticPackageAdapterPreview } from "../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { semanticPackageGeneratedLeafPreview } from "../../🔍️discovery/🔭️repository-scan/🟦️.ts";
import { parseCanonicalWgpuPackageCatalog, parseSemanticPackageBrowserProfile } from "../../🔍️discovery/🎁️package-projection/🟦️.ts";
import { basename, dirname, posix } from "node:path";
import { semanticArtifactEmptyFacetProjectionAuthority, semanticOwnedFileProjectionAuthority } from "../../🔍️discovery/🗃️owned-file-projection/🟦️.ts";
import { type ReferenceEdit, type TaxonomyApplyOptions, type TaxonomyEmbeddedTicketRootDisposition, type TaxonomyEmbeddedTicketRootRelocation, type TaxonomyEvidenceMember, type TaxonomyEvidenceRemoval, type TaxonomyInventory, type TaxonomyInventoryEntry, type TaxonomyMove, type TaxonomyMoveSourceAuthority, type TaxonomyPlan, type TaxonomyPlanOptions, type TaxonomyRemovalAuthority, type TaxonomyViolation, requireTaxonomyValue } from "../📜️plan-contracts/🟦️.ts";
import { absolutePath, checkCancellation, isExcluded, stableViolations, violation } from "../🏷️file-kind-matching/🟦️.ts";
import { dispositionOperationId, generatorPathCompare } from "../🧾️plan-parsing/🟦️.ts";
import type { MutableJournalRecord } from "../💾️durable-journal/🟦️.ts";
import { type CollisionGroup, collisionGroups } from "../🏭️generator-inputs/🟦️.ts";
import { type EmbeddedDispositionPlanning, incomingEmbeddedReferences, repositoryLocalSymlinkTargetPath } from "../➰️symlink-targets/🟦️.ts";
import { referenceEditIdentity, taxonomyFixturePayloadPath } from "../🧱️inventory-leaf-stamps/🟦️.ts";
import { generatorContractsForOutputPath } from "../📸️repository-capture/🟦️.ts";
import { pathPolicyViolations } from "../👥️mutation-projection/🟦️.ts";
import { matchingFixedContracts } from "../📏️fixed-contracts/🟦️.ts";
import { exactOwnedCatalogSnapshot, exactOwnedCurrentRawPath, exactOwnedFileCatalog, nestedCargoCatalog, ticketImportantExactMutationAuthority, ticketImportantExactMutationCases } from "../🗺️artifact-projection-catalogs/🟦️.ts";
import { inventoryLeafPreimage, removalIncomingIgnoredSourceRoots } from "../🧐️preimage-assertions/🟦️.ts";
import { artifactCatalogProjectionRationale, exactOwnedFileResolution } from "../📂️owned-file-projection/🟦️.ts";
import { incomingReferenceLexicalAdmission, referenceCandidatesWithProgress, repositoryReferenceCandidatePaths } from "../🔗️reference-inventory/🟦️.ts";
import { noFollowTreeDigest, noFollowTreeDigestExcluding } from "../🌳️tree-digests/🟦️.ts";
import { frozenEvidenceCoordinateAuthority, isFrozenSourceCoordinateToken, validateObservedFrozenEvidenceNodes } from "../❄️frozen-evidence/🟦️.ts";
import { referencePathIndex, resolveReferenceTokenPath } from "../🧭️reference-resolution/🟦️.ts";
import { textualPath } from "../🧮️reference-digests/🟦️.ts";
import { planVerificationCandidatePaths } from "../♻️projection-staleness/🟦️.ts";
import { removalReferenceProjection } from "../🗑️removal-references/🟦️.ts";
import { referenceTokensIncludingUnsupported } from "../🔬️reference-analysis/🟦️.ts";
import { referenceCoordinateRoots } from "../📥️incoming-references/🟦️.ts";
import { buildReferenceEdits } from "../🪞️artifact-reference-projection/🟦️.ts";

export interface PreflightReferenceBasis {
  readonly repoRoot: string;
  readonly taxonomy: LoadedTaxonomy;
  readonly ticketDir?: string;
  readonly transactionRoot?: string;
  readonly authorityPlan?: TaxonomyPlan;
  readonly candidates: readonly string[];
  readonly knownPaths: ReadonlySet<string>;
  readonly coordinateRoots: readonly string[];
  readonly markers: ReadonlyMap<string, string>;
  readonly observed: Map<string, { readonly witness: string; readonly content: boolean }>;
  readonly changedNodes: Set<string>;
}

function preflightReferenceNodeWitness(stat: Stats | null, bytes?: string | Uint8Array, identity = false): string {
  if (!stat) return "absent";
  const kind = stat.isSymbolicLink() ? "symlink" : stat.isFile() ? "file" : stat.isDirectory() ? "directory" : "other";
  return canonicalJson({ kind, mode: stat.mode & 0o7777, ...(kind === "file" || kind === "symlink" ? { size: stat.size } : {}), ...(bytes === undefined ? {} : { hash: sha256(bytes) }), ...(identity ? { dev: stat.dev, ino: stat.ino } : {}) });
}

function observePreflightReferenceNode(basis: PreflightReferenceBasis | undefined, path: string, stat: Stats | null, bytes?: string | Uint8Array): void {
  if (!basis) return;
  const witness = preflightReferenceNodeWitness(stat, bytes), prior = basis.observed.get(path);
  if (!prior) basis.observed.set(path, { witness, content: bytes !== undefined });
  else if (prior.witness !== witness) basis.changedNodes.add(path);
}

export function capturePreflightReferenceBasis(repoRoot: string, taxonomy: LoadedTaxonomy, ticketDir?: string, transactionRoot?: string, authorityPlan?: TaxonomyPlan, cancelFile?: string, progress?: TaxonomyApplyOptions["progress"]): PreflightReferenceBasis {
  checkCancellation(repoRoot, cancelFile);
  const context = { ticketDir, transactionRoots: transactionRoot ? [transactionRoot] : [], exactEvidencePaths: [] };
  const candidates = [...new Set([...repositoryReferenceCandidatePaths(repoRoot, taxonomy, context, cancelFile), ...(authorityPlan ? planVerificationCandidatePaths(repoRoot, authorityPlan, taxonomy, ticketDir) : [])])].sort(generatorPathCompare);
  const knownPaths = new Set(candidates), markers = new Map<string, string>();
  for (const path of candidates) for (let parent = posix.dirname(path); parent && parent !== "."; parent = posix.dirname(parent)) knownPaths.add(parent);
  validateObservedFrozenEvidenceNodes(repoRoot, knownPaths, taxonomy);
  const coordinateRoots = referenceCoordinateRoots(repoRoot, candidates, taxonomy, cancelFile, (path, stat, bytes) => {
    const witness = preflightReferenceNodeWitness(stat, bytes, true), prior = markers.get(path);
    if (prior !== undefined && prior !== witness) throw new Error(`Preflight incoming reference marker witness changed: ${path}`);
    markers.set(path, witness);
  }, progress, "apply");
  return { repoRoot, taxonomy, ticketDir, transactionRoot, authorityPlan, candidates, knownPaths, coordinateRoots, markers, observed: new Map(), changedNodes: new Set() };
}

export function validatePreflightReferenceBasis(basis: PreflightReferenceBasis, cancelFile?: string, progress?: TaxonomyApplyOptions["progress"]): void {
  checkCancellation(basis.repoRoot, cancelFile);
  const fresh = capturePreflightReferenceBasis(basis.repoRoot, basis.taxonomy, basis.ticketDir, basis.transactionRoot, basis.authorityPlan, cancelFile, progress);
  const unowned = (path: string): boolean => !basis.transactionRoot || path !== basis.transactionRoot && !path.startsWith(`${basis.transactionRoot}/`);
  if (canonicalJson(basis.candidates.filter(unowned)) !== canonicalJson(fresh.candidates.filter(unowned))) throw new Error("Preflight incoming reference candidate membership changed");
  const markerRows = (value: PreflightReferenceBasis) => [...value.markers].filter(([path]) => unowned(path)).sort(([left], [right]) => generatorPathCompare(left, right));
  if (canonicalJson(markerRows(basis)) !== canonicalJson(markerRows(fresh)) || canonicalJson(basis.coordinateRoots.filter(unowned)) !== canonicalJson(fresh.coordinateRoots.filter(unowned))) throw new Error("Preflight incoming reference marker witness changed");
  for (const [, path] of referenceCandidatesWithProgress([...basis.observed.keys()], "apply", progress)) {
    checkCancellation(basis.repoRoot, cancelFile);
    const expected = basis.observed.get(path)!;
    const absolute = assertLexicalInputOutsideOpaque(basis.repoRoot, path, "Preflight incoming reference freshness"), stat = lstatOrNull(absolute);
    const bytes = stat?.isSymbolicLink() ? readlinkSync(absolute) : expected.content && stat?.isFile() ? readFileSync(absolute) : undefined;
    if (basis.changedNodes.has(path) || expected.witness !== preflightReferenceNodeWitness(stat, bytes)) throw new Error(`Preflight incoming reference node changed: ${path}`);
  }
}

export function lexicalTargetIncomingReferences(repoRoot: string, targetPaths: ReadonlySet<string>, ignoredSourceRoots: readonly string[], taxonomy: LoadedTaxonomy, ticketDir?: string, planAuthority?: Readonly<{ path: string; bytes: Uint8Array }>, transactionRoot?: string, authorityPlan?: TaxonomyPlan, cancelFile?: string, progress?: TaxonomyApplyOptions["progress"], projectReference?: ReturnType<typeof removalReferenceProjection>, basis?: PreflightReferenceBasis): readonly string[] {
  checkCancellation(repoRoot, cancelFile);
  if (targetPaths.size === 0) return [];
  if (basis && (basis.repoRoot !== repoRoot || basis.taxonomy !== taxonomy || basis.ticketDir !== ticketDir || basis.transactionRoot !== transactionRoot || basis.authorityPlan !== authorityPlan)) throw new Error("Preflight incoming reference basis context changed");
  const context = { ticketDir, transactionRoots: transactionRoot ? [transactionRoot] : [], exactEvidencePaths: [] };
  const candidates = basis ? new Set(basis.candidates) : new Set<string>([...repositoryReferenceCandidatePaths(repoRoot, taxonomy, context, cancelFile), ...(authorityPlan ? planVerificationCandidatePaths(repoRoot, authorityPlan, taxonomy, ticketDir) : [])]);
  const knownPaths = basis?.knownPaths ?? new Set(candidates);
  if (!basis) for (const path of candidates) for (let parent = posix.dirname(path); parent && parent !== "."; parent = posix.dirname(parent)) (knownPaths as Set<string>).add(parent);
  validateObservedFrozenEvidenceNodes(repoRoot, knownPaths, taxonomy);
  const targetIndex = referencePathIndex(targetPaths, repoRoot, basis?.coordinateRoots ?? referenceCoordinateRoots(repoRoot, candidates, taxonomy, cancelFile, undefined, progress, "apply"), knownPaths, cancelFile);
  const admitsText = incomingReferenceLexicalAdmission(targetPaths);
  const rows: string[] = [];
  for (const [, path] of referenceCandidatesWithProgress([...candidates].sort(generatorPathCompare), "apply", progress)) {
    checkCancellation(repoRoot, cancelFile);
    if (isExcluded(path, taxonomy) || taxonomyFixturePayloadPath(path, taxonomy.discoverySchema) || ignoredSourceRoots.some((root) => path === root || path.startsWith(`${root}/`)) || (transactionRoot && (path === transactionRoot || path.startsWith(`${transactionRoot}/`)))) continue;
    const absolute = assertLexicalInputOutsideOpaque(repoRoot, path, "Terminal incoming reference");
    const stat = lstatOrNull(absolute);
    if (!stat || stat.isDirectory()) { observePreflightReferenceNode(basis, path, stat); continue; }
    if (stat.isSymbolicLink()) {
      const raw = readlinkSync(absolute);
      observePreflightReferenceNode(basis, path, stat, raw);
      let target: string | null = null;
      if (raw.startsWith("/") || /^[A-Za-z]:[\\/]/u.test(raw) || /^(?:\\\\|\/\/)/u.test(raw)) target = repositoryLocalSymlinkTargetPath(repoRoot, raw);
      else try { target = normalizeRelative(posix.join(posix.dirname(path), raw.replaceAll("\\", "/"))); } catch { target = null; }
      if (target && targetPaths.has(target)) rows.push(`symlink\u0000${path}\u0000${target}`);
      continue;
    }
    if (!stat.isFile() || !textualPath(path)) { observePreflightReferenceNode(basis, path, stat); continue; }
    const physicalBytes = readFileSync(absolute);
    observePreflightReferenceNode(basis, path, stat, physicalBytes);
    if (planAuthority?.path === path && physicalBytes.equals(planAuthority.bytes)) continue;
    const projected = projectReference?.(path, physicalBytes, stat.mode & 0o7777) ?? { path, bytes: physicalBytes }, bytes = projected.bytes;
    frozenEvidenceCoordinateAuthority(projected.path, bytes, taxonomy);
    const content = bytes.toString("utf8");
    if (!admitsText(content)) continue;
    for (const token of referenceTokensIncludingUnsupported(projected.path, content, targetIndex)) {
      if (token.unsupportedReason && token.physicalTargets !== undefined) for (const target of token.physicalTargets) if (targetPaths.has(target)) rows.push(`text\u0000${path}\u0000${target}`);
      const target = resolveReferenceTokenPath(projected.path, token, targetIndex);
      if (target && targetPaths.has(target) && !isFrozenSourceCoordinateToken(projected.path, bytes, token, target, taxonomy, repoRoot)) rows.push(`text\u0000${path}\u0000${target}`);
    }
  }
  return [...new Set(rows)].sort(generatorPathCompare);
}

export function embeddedTargetPaths(plan: TaxonomyPlan, root: TaxonomyEmbeddedTicketRootDisposition): ReadonlySet<string> {
  const targetPaths = new Set<string>([root.sourceMetadataRoot, root.sourceTicketRoot]);
  for (const id of [...root.relocationOperationIds, ...root.removalOperationIds]) {
    const leaf = plan.embeddedTicketRootRelocations.find((entry) => entry.operationId === id)?.sourcePath ?? plan.evidenceRemovals.find((entry) => entry.operationId === id)?.sourcePath;
    if (!leaf) continue;
    targetPaths.add(leaf);
    for (let parent = posix.dirname(leaf); parent === root.sourceMetadataRoot || parent.startsWith(`${root.sourceMetadataRoot}/`); parent = posix.dirname(parent)) {
      targetPaths.add(parent);
      if (parent === root.sourceMetadataRoot) break;
    }
  }
  return targetPaths;
}

export function lexicalEmbeddedIncomingReferences(repoRoot: string, plan: TaxonomyPlan, root: TaxonomyEmbeddedTicketRootDisposition, taxonomy: LoadedTaxonomy, ticketDir?: string, planAuthority?: Readonly<{ path: string; bytes: Uint8Array }>, transactionRoot?: string, cancelFile?: string, progress?: TaxonomyApplyOptions["progress"]): readonly string[] {
  return lexicalTargetIncomingReferences(repoRoot, embeddedTargetPaths(plan, root), [root.sourceMetadataRoot], taxonomy, ticketDir, planAuthority, transactionRoot, plan, cancelFile, progress);
}

export function planEmbeddedTicketRoots(inventory: TaxonomyInventory, taxonomy: LoadedTaxonomy): EmbeddedDispositionPlanning {
  const violations: TaxonomyViolation[] = [];
  const roots: TaxonomyEmbeddedTicketRootDisposition[] = [];
  const relocations: TaxonomyEmbeddedTicketRootRelocation[] = [];
  const removals: TaxonomyEvidenceRemoval[] = [];
  const nested = new Map<string, { sourceTicketRoot: string; canonicalTicketRoot: string; ticketId: string }>();
  for (const entry of inventory.entries) {
    const parts = entry.sourcePath.split("/");
    for (let index = 1; index + 7 < parts.length; index++) {
      if (parts[index] !== ".🧬semio" || parts[index + 1] !== "🦑️repo" || parts[index + 2] !== "🎫️tickets") continue;
      const sourceMetadataRoot = parts.slice(0, index + 1).join("/");
      const suffix = parts.slice(index + 3, index + 7);
      if (!/^🎆️[0-9]{2}$/u.test(suffix[0] ?? "") || !/^🌙️[0-9]{2}$/u.test(suffix[1] ?? "") || !/^☀️[0-9]{2}$/u.test(suffix[2] ?? "") || !suffix[3]) { violations.push(violation("embedded-ticket-root-identity-invalid", entry.sourcePath, "Nested metadata path has no exact ticket identity")); continue; }
      const canonicalTicketRoot = [".🧬semio", "🦑️repo", "🎫️tickets", ...suffix].join("/");
      const rootContract = matchingFixedContracts(canonicalTicketRoot, taxonomy.schema.fixedDirectoryContracts, taxonomy, null).selected;
      const manifestPath = `${canonicalTicketRoot}/🎫️ticket.json`;
      const manifestContract = matchingFixedContracts(manifestPath, taxonomy.schema.fixedFilenameContracts, taxonomy, null).selected;
      const manifestStat = lstatOrNull(absolutePath(inventory.repoRoot, manifestPath));
      if (rootContract?.[0] !== "ticket-slug" || manifestContract?.[0] !== "ticket-manifest" || !manifestStat?.isFile() || manifestStat.isSymbolicLink()) { violations.push(violation("embedded-ticket-root-authority-missing", entry.sourcePath, "Canonical ticket root and exact ticket manifest authority must exist")); continue; }
      nested.set(sourceMetadataRoot, { sourceTicketRoot: parts.slice(0, index + 7).join("/"), canonicalTicketRoot, ticketId: `${splitLeadingEmoji(requireTaxonomyValue(suffix[0],"suffix[0]")).rest}/${splitLeadingEmoji(requireTaxonomyValue(suffix[1],"suffix[1]")).rest}/${splitLeadingEmoji(requireTaxonomyValue(suffix[2],"suffix[2]")).rest}/${suffix[3]}` });
    }
  }
  const occupancy = new Map(inventory.entries.map((entry) => [entry.sourcePath, entry]));
  const candidates: { root: string; rootId: string; sourcePath: string; destinationPath: string; entry: TaxonomyInventoryEntry }[] = [];
  for (const [sourceMetadataRoot, identity] of [...nested].sort(([left], [right]) => generatorPathCompare(left, right))) {
    const allLeaves = inventory.entries.filter((entry) => entry.nodeKind !== "directory" && entry.sourcePath.startsWith(`${sourceMetadataRoot}/`));
    const leaves = allLeaves.filter((entry) => entry.sourcePath.startsWith(`${identity.sourceTicketRoot}/`));
    if (allLeaves.length !== leaves.length) { violations.push(violation("embedded-ticket-root-residual-leaf", sourceMetadataRoot, "Nested metadata root contains a leaf outside its exact ticket root")); continue; }
    if (leaves.some((entry) => entry.nodeKind === "symlink" || !entry.fixedContractId)) { violations.push(violation("embedded-ticket-root-leaf-unresolved", sourceMetadataRoot, "Nested metadata root contains a symlink or a leaf without exact fixed-contract authority")); continue; }
    const sourceTreeDigest = noFollowTreeDigest(inventory.repoRoot, sourceMetadataRoot);
    const residualTreeDigest = noFollowTreeDigestExcluding(inventory.repoRoot, sourceMetadataRoot, leaves.map((entry) => entry.sourcePath));
    const incoming = incomingEmbeddedReferences(inventory, sourceMetadataRoot);
    const incomingReferenceDigest = sha256(`sha256-taxonomy-reference-set-v1\u0000${canonicalJson(incoming)}`);
    if (incoming.length > 0) { violations.push(violation("embedded-ticket-root-incoming-reference", sourceMetadataRoot, `Nested metadata root has ${incoming.length} incoming reference(s)`)); continue; }
    const authority = { sourceMetadataRoot, sourceTicketRoot: identity.sourceTicketRoot, canonicalTicketRoot: identity.canonicalTicketRoot, ticketId: identity.ticketId, sourceTreeDigest, residualTreeDigest, incomingReferenceDigest, rationaleRule: "embedded-ticket-root-relocation-v1" as const };
    const rootId = dispositionOperationId("embedded-ticket-root", authority);
    for (const entry of leaves) {
      const destinationPath = `${identity.canonicalTicketRoot}/${entry.sourcePath.slice(identity.sourceTicketRoot.length + 1)}`;
      if (destinationPath !== normalizeRelative(destinationPath) || pathPolicyViolations(destinationPath, taxonomy).length > 0 || !entry.fixedContractId) { violations.push(violation("embedded-ticket-root-destination-invalid", destinationPath, "Embedded evidence destination is noncanonical, over budget, or lacks exact fixed authority")); continue; }
      candidates.push({ root: sourceMetadataRoot, rootId, sourcePath: entry.sourcePath, destinationPath, entry });
    }
    roots.push({ operationId: rootId, ...authority, relocationOperationIds: [], removalOperationIds: [] });
  }
  const byDestination = new Map<string, typeof candidates>();
  for (const candidate of candidates) byDestination.set(candidate.destinationPath, [...(byDestination.get(candidate.destinationPath) ?? []), candidate]);
  for (const [destinationPath, group] of [...byDestination].sort(([left], [right]) => generatorPathCompare(left, right))) {
    const sorted = [...group].sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath));
    const occupied = occupancy.get(destinationPath);
    const evidenceIdentity = (entry: TaxonomyInventoryEntry): string => canonicalJson({ nodeKind: entry.nodeKind, contentHash: entry.contentHash, mode: entry.mode, size: entry.size, ownerId: entry.ownerId, fixedContractId: entry.fixedContractId, packageRole: entry.packageRole });
    const identity = evidenceIdentity(requireTaxonomyValue(sorted[0],"sorted[0]").entry);
    if (sorted.some((entry) => evidenceIdentity(entry.entry) !== identity) || (occupied && evidenceIdentity(occupied) !== identity)) { violations.push(violation("embedded-ticket-root-destination-conflict", destinationPath, "Many-to-one ticket evidence is not byte, mode, kind, size, owner, role and contract identical")); continue; }
    const installer = occupied ? null : sorted[0];
    if (installer) {
      const provisional = { embeddedTicketRootId: installer.rootId, sourcePath: installer.sourcePath, destinationPath, relativeEvidencePath: installer.sourcePath.slice(nested.get(installer.root)!.sourceTicketRoot.length + 1), preimage: inventoryLeafPreimage(installer.entry), fixedContractId: installer.entry.fixedContractId, ownerId: installer.entry.ownerId, rationaleRule: "embedded-ticket-root-relocation-v1" as const };
      relocations.push({ operationId: dispositionOperationId("embedded-ticket-root-relocation", provisional), ...provisional });
    }
    for (const candidate of sorted.filter((entry) => entry !== installer)) {
      const members: TaxonomyEvidenceMember[] = [
        ...sorted.map((entry) => ({ sourcePath: entry.sourcePath, finalPath: destinationPath, disposition: entry === installer ? "relocate" as const : "remove" as const, preimage: inventoryLeafPreimage(entry.entry) })),
        ...(occupied ? [{ sourcePath: destinationPath, finalPath: destinationPath, disposition: "retain" as const, preimage: inventoryLeafPreimage(occupied) }] : []),
      ].sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath));
      const retainedFinalPath = destinationPath;
      const evidenceSetDigest = sha256(canonicalJson({ algorithm: "sha256-byte-mode-evidence-set-v1", members, retainedFinalPath }));
      const authority: TaxonomyRemovalAuthority = { kind: "byte-and-mode-identical", evidenceSetDigest, retainedFinalPath, members };
      const provisional = { sourcePath: candidate.sourcePath, preimage: inventoryLeafPreimage(candidate.entry), authority, embeddedTicketRootId: candidate.rootId, rationaleRule: "redundant-ticket-evidence-v1" as const, ownerId: candidate.entry.ownerId };
      removals.push({ operationId: dispositionOperationId("evidence-removal", provisional), ...provisional });
    }
  }
  for (const root of roots) {
    const children = relocations.filter((entry) => entry.embeddedTicketRootId === root.operationId).length + removals.filter((entry) => entry.embeddedTicketRootId === root.operationId).length;
    if (children !== root.sourceTreeDigest.files + root.sourceTreeDigest.symlinks) violations.push(violation("embedded-ticket-root-closure-incomplete", root.sourceMetadataRoot, `Frozen tree has ${root.sourceTreeDigest.files + root.sourceTreeDigest.symlinks} leaves but ${children} dispositions`));
  }
  if (violations.length > 0) return { roots: [], relocations: [], removals: [], violations: stableViolations(violations) };
  const finalizedRoots = roots.map((root) => ({ ...root, relocationOperationIds: relocations.filter((entry) => entry.embeddedTicketRootId === root.operationId).map((entry) => entry.operationId).sort(generatorPathCompare), removalOperationIds: removals.filter((entry) => entry.embeddedTicketRootId === root.operationId).map((entry) => entry.operationId).sort(generatorPathCompare) }));
  return { roots: finalizedRoots, relocations: relocations.sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath)), removals: removals.sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath)), violations: stableViolations(violations) };
}

export function planTrailingEvidenceRemovals(inventory: TaxonomyInventory): readonly TaxonomyEvidenceRemoval[] {
  const rows: TaxonomyEvidenceRemoval[] = [];
  for (const entry of inventory.entries.filter((candidate) => candidate.nodeKind !== "directory" && candidate.sourcePath.startsWith(".🧬semio/🦑️repo/🎫️tickets") && /^[. ]+$/u.test(basename(candidate.sourcePath)))) {
    const parent = posix.dirname(entry.sourcePath);
    const identical = inventory.entries.filter((candidate) => candidate.nodeKind === entry.nodeKind && posix.dirname(candidate.sourcePath) === parent && candidate.contentHash === entry.contentHash && candidate.mode === entry.mode && candidate.size === entry.size && candidate.ownerId === entry.ownerId && candidate.fixedContractId === entry.fixedContractId && candidate.packageRole === entry.packageRole && candidate.sourcePath !== entry.sourcePath && !/^[. ]+$/u.test(basename(candidate.sourcePath))).sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath));
    if (identical.length === 0 || entry.referencesIn.length > 0) continue;
    const retainedFinalPath = requireTaxonomyValue(identical[0],"identical[0]").normalizedPath;
    const members: TaxonomyEvidenceMember[] = [{ sourcePath: entry.sourcePath, finalPath: retainedFinalPath, disposition: "remove" as const, preimage: inventoryLeafPreimage(entry) }, ...identical.map((candidate) => ({ sourcePath: candidate.sourcePath, finalPath: candidate.normalizedPath, disposition: "retain" as const, preimage: inventoryLeafPreimage(candidate) }))].sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath));
    const evidenceSetDigest = sha256(canonicalJson({ algorithm: "sha256-byte-mode-evidence-set-v1", members, retainedFinalPath }));
    const provisional = { sourcePath: entry.sourcePath, preimage: inventoryLeafPreimage(entry), authority: { kind: "byte-and-mode-identical" as const, evidenceSetDigest, retainedFinalPath, members }, rationaleRule: "redundant-ticket-evidence-v1" as const, ownerId: entry.ownerId };
    rows.push({ operationId: dispositionOperationId("evidence-removal", provisional), ...provisional });
  }
  return rows.sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath));
}

export function planExactGeneratedOwnerRemovals(inventory: TaxonomyInventory, taxonomy: LoadedTaxonomy): readonly TaxonomyEvidenceRemoval[] {
  const catalog = exactOwnedFileCatalog(inventory.repoRoot, taxonomy);
  const contract = taxonomy.schema.semanticOwnedFileProjectionContracts["readme-license-owner-leaves-v1"];
  if (!catalog || contract?.contractKind !== "exact-owner-path-catalog") return [];
  const rows: TaxonomyEvidenceRemoval[] = [];
  for (const owner of catalog.entries.filter((entry) => entry.generatorOwnerId !== null)) {
    const source = inventory.entries.find((entry) => entry.sourcePath === owner.sourcePath);
    if (!source || source.nodeKind !== "file" || source.violations.some((entry) => entry.severity === "error") || source.normalizedPath !== owner.destinationPath) continue;
    const outputPreimage = { nodeKind: "file" as const, contentHash: owner.preimage.sha256, mode: Number.parseInt(owner.preimage.mode, 8), size: owner.preimage.size };
    if (canonicalJson(inventoryLeafPreimage(source)) !== canonicalJson(outputPreimage)) continue;
    const authorityBase = { kind: "exact-owner-generated-source" as const, catalogPath: contract.authorityCatalogPath, catalogContentHash: contract.authorityCatalogSha256, generatorContractId: owner.generatorOwnerId!, destinationPath: owner.destinationPath, outputPreimage };
    const authority = { ...authorityBase, authorityDigest: sha256(canonicalJson(authorityBase)) };
    const provisional = { sourcePath: owner.sourcePath, preimage: outputPreimage, authority, rationaleRule: "exact-owner-generated-source-retirement-v1" as const, ownerId: source.ownerId };
    rows.push({ operationId: dispositionOperationId("evidence-removal", provisional), ...provisional });
  }
  return rows.sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath));
}

export function planNestedCargoGeneratedSourceRemovals(inventory: TaxonomyInventory, taxonomy: LoadedTaxonomy): readonly TaxonomyEvidenceRemoval[] {
  const catalog = nestedCargoCatalog(inventory.repoRoot, taxonomy.discoverySchema);
  const contract = taxonomy.discoverySchema.semanticPackageProjectionContracts["nested-cargo-packages-v1"];
  if (!catalog || !contract) return [];
  const entries = new Map(inventory.entries.map((entry) => [entry.sourcePath, entry])), removals: TaxonomyEvidenceRemoval[] = [];
  for (const row of catalog.packages) {
    if (row.id !== "wgpu-renderer" || !row.mappings.every((mapping) => { const source = entries.get(mapping.sourcePath); return source?.nodeKind === "file" && source.normalizedPath === mapping.destinationPath && source.contentHash === mapping.sourceHash && source.size === mapping.sourceSize && !source.violations.some((violation) => violation.severity === "error"); })) continue;
    for (const retirement of row.generatedSourceRetirements) {
      const source = entries.get(retirement.sourcePath)!, sourcePreimage = inventoryLeafPreimage(source);
      if (sourcePreimage.nodeKind !== "file" || sourcePreimage.mode !== retirement.sourceMode) continue;
      const digestible = { kind: "nested-cargo-generated-source" as const, catalogPath: contract.authorityCatalogPath, catalogContentHash: contract.authorityCatalogSha256, packageId: row.id, generatorContractId: retirement.generatorContractId, destinationPath: retirement.destinationPath, sourcePreimage };
      const authority = { ...digestible, authorityDigest: sha256(canonicalJson(digestible)) };
      const provisional = { sourcePath: retirement.sourcePath, preimage: sourcePreimage, authority, rationaleRule: "nested-cargo-generated-source-retirement-v1" as const, ownerId: source.ownerId };
      removals.push({ operationId: dispositionOperationId("evidence-removal", provisional), ...provisional });
    }
  }
  return removals.sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath));
}

export function planTicketImportantRemovals(inventory: TaxonomyInventory, taxonomy: LoadedTaxonomy): { readonly removals: readonly TaxonomyEvidenceRemoval[]; readonly violations: readonly TaxonomyViolation[] } {
  const removals: TaxonomyEvidenceRemoval[] = [];
  const violations: TaxonomyViolation[] = [];
  const entries = new Map(inventory.entries.map((entry) => [entry.sourcePath, entry]));
  for (const entry of inventory.entries.filter((candidate) => candidate.nodeKind === "file" && basename(candidate.sourcePath) === "📌️important.md" && candidate.size === 0)) {
    const ownerPath = dirname(entry.sourcePath);
    const owner = entries.get(ownerPath);
    const manifestPath = `${ownerPath}/🎫️ticket.json`;
    const manifest = entries.get(manifestPath);
    if (owner?.fixedContractId !== "ticket-slug" || manifest?.nodeKind !== "file" || manifest.fixedContractId !== "ticket-manifest") continue;
    const manifestContent = readFileSync(absolutePath(inventory.repoRoot, manifestPath), "utf8");
    const decision = semanticOwnedFileProjectionAuthority({ ownerPath, ownerFixedDirectoryContractIds: ["ticket-slug"], manifestPath, manifestFixedFilenameContractIds: ["ticket-manifest"], manifestContent, sourcePath: entry.sourcePath, sourceFileKindId: entry.fileKind ?? "", sourceByteLength: entry.size }, taxonomy.discoverySchema);
    if (decision.disposition !== "remove") continue;
    if (entry.referencesIn.length > 0) {
      violations.push(violation("ticket-important-removal-referenced", entry.sourcePath, `Closed empty ticket important document has incoming references: ${entry.referencesIn.join(", ")}`));
      continue;
    }
    const manifestPreimage = inventoryLeafPreimage(manifest);
    if (manifestPreimage.nodeKind !== "file") throw new Error(`Ticket manifest preimage is not a regular file: ${manifestPath}`);
    const authorityBase = { kind: "owner-manifest-status" as const, contractId: "ticket-important-markdown-v1" as const, ownerPath, manifestPath, manifestPreimage, status: "closed" as const, contentState: "zero-byte" as const };
    const authority = { ...authorityBase, authorityDigest: sha256(canonicalJson(authorityBase)) };
    const provisional = { sourcePath: entry.sourcePath, preimage: inventoryLeafPreimage(entry), authority, rationaleRule: "ticket-important-closed-empty-v1" as const, ownerId: entry.ownerId };
    removals.push({ operationId: dispositionOperationId("evidence-removal", provisional), ...provisional });
  }
  const exactCases = ticketImportantExactMutationCases(inventory.repoRoot).filter((entry) => entry.disposition === "remove");
  const exactSources = exactCases.filter((entry) => entries.has(entry.sourcePath));
  for (const exact of exactSources) {
    const entry = entries.get(exact.sourcePath)!;
    if (canonicalJson(inventoryLeafPreimage(entry)) !== canonicalJson(exact.sourcePreimage) || entry.referencesIn.length > 0) {
      violations.push(violation("ticket-important-exact-removal-invalid", exact.sourcePath, `Exact empty residue preimage or incoming references changed for ${exact.caseId}`));
      continue;
    }
    const authorityBase = { kind: "exact-path-mutation" as const, catalogPath: exact.catalogPath, catalogContentHash: exact.catalogContentHash, caseId: exact.caseId, sourcePath: exact.sourcePath, sourcePreimage: exact.sourcePreimage, disposition: "remove" as const };
    const authority = { ...authorityBase, authorityDigest: sha256(canonicalJson(authorityBase)) };
    const provisional = { sourcePath: entry.sourcePath, preimage: inventoryLeafPreimage(entry), authority, rationaleRule: "ticket-important-exact-empty-residue-v1" as const, ownerId: entry.ownerId };
    removals.push({ operationId: dispositionOperationId("evidence-removal", provisional), ...provisional });
  }
  return { removals: removals.sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath)), violations: stableViolations(violations) };
}

interface MoveReferenceAuthority {
  readonly moves: readonly TaxonomyMove[];
  readonly edits: readonly ReferenceEdit[];
  readonly editTargets: ReadonlyMap<string, string>;
  readonly resultHashes: ReadonlyMap<string, string>;
  readonly resultSizes: ReadonlyMap<string, number>;
  readonly unresolved: readonly TaxonomyViolation[];
  readonly collisionGroups: readonly CollisionGroup[];
}

export function planMoveReferenceAuthority(inventory: TaxonomyInventory, taxonomy: LoadedTaxonomy, options: TaxonomyPlanOptions, embeddedRoots: readonly TaxonomyEmbeddedTicketRootDisposition[], evidenceRemovals: readonly TaxonomyEvidenceRemoval[]): MoveReferenceAuthority {
  const ownerSnapshot = exactOwnedCatalogSnapshot(inventory.repoRoot, taxonomy);
  const ownerMappings = new Map(ownerSnapshot.catalog?.entries.filter((entry) => entry.disposition !== "fixed").map((entry) => [entry.sourcePath, entry.destinationPath]) ?? []);
  const packageMappings = new Map(nestedCargoCatalog(inventory.repoRoot, taxonomy.discoverySchema)?.packages.flatMap((row) => row.mappings.map((mapping) => [mapping.sourcePath, mapping.destinationPath] as const)) ?? []);
  const embeddedPrefixes = embeddedRoots.map((root) => root.sourceMetadataRoot);
  const removalSources = new Set(evidenceRemovals.map((entry) => entry.sourcePath));
  const isEmbedded = (path: string): boolean => embeddedPrefixes.some((root) => path === root || path.startsWith(`${root}/`));
  const groups = collisionGroups(inventory.entries.filter((entry) => !removalSources.has(entry.sourcePath)), taxonomy);
  const groupBySource = new Map<string, string>();
  for (const group of groups) for (const source of group.sources) if (!groupBySource.has(source)) groupBySource.set(source, group.id);
  const preliminaryMoves: TaxonomyMove[] = inventory.entries
    .filter((entry) => entry.nodeKind !== "directory" && entry.sourcePath !== entry.normalizedPath && !groupBySource.has(entry.sourcePath) && !isEmbedded(entry.sourcePath) && !removalSources.has(entry.sourcePath) && generatorContractsForOutputPath(entry.sourcePath, taxonomy).length === 0)
    .map((entry) => {
      const sourcePreimage = inventoryLeafPreimage(entry);
      const exactTicketImportant = ticketImportantExactMutationAuthority(inventory.repoRoot, entry.sourcePath, sourcePreimage);
      const emptyFacet = basename(entry.sourcePath) === "📌️.empty.md" ? semanticArtifactEmptyFacetProjectionAuthority({ sourcePath: entry.sourcePath, sourceFileKindId: entry.fileKind ?? "" }, taxonomy.discoverySchema) : undefined;
      let sourceAuthority: TaxonomyMoveSourceAuthority | undefined;
      if (exactOwnedCurrentRawPath(entry.sourcePath, ownerSnapshot, taxonomy)) {
        const fresh = exactOwnedFileResolution(inventory.repoRoot, entry, ownerSnapshot, taxonomy);
        if (!fresh.sourceAuthority || fresh.result.disposition !== "project" || fresh.result.problems.length || fresh.result.entry?.destinationPath !== entry.normalizedPath) throw new Error("owner-leaf-authority-invalid: " + entry.sourcePath + ": " + fresh.result.problems.join(" | "));
        sourceAuthority = fresh.sourceAuthority;
      }
      return ({
      operationId: dispositionOperationId("move-v2", { sourcePath: entry.sourcePath, destinationPath: entry.normalizedPath, sourcePreimage }),
      sourcePath: entry.sourcePath,
      destinationPath: entry.normalizedPath,
      sourcePreimage,
      rationaleRule: emptyFacet?.disposition === "project" && emptyFacet.destinationPath === entry.normalizedPath ? emptyFacet.contractId : ownerMappings.get(entry.sourcePath) === entry.normalizedPath ? "readme-license-owner-projection-v1" : packageMappings.get(entry.sourcePath) === entry.normalizedPath ? "nested-cargo-package-projection-v1" : basename(entry.sourcePath) === "ticket.md" && entry.normalizedPath === `${dirname(entry.sourcePath)}/📝️.md` ? "ticket-document-primary-markdown-v1" : exactTicketImportant?.disposition === "move" ? "ticket-important-presence-owned-markdown-v1" : basename(entry.sourcePath) === "📌️important.md" && entry.normalizedPath.endsWith("/📌️important/📝️.md") ? "ticket-important-markdown-projection-v1" : basename(entry.sourcePath) === "📌️important.md" && entry.normalizedPath.endsWith("/📓️important/📝️.md") ? "ticket-important-history-markdown-v1" : artifactCatalogProjectionRationale(entry.sourcePath, entry.normalizedPath, taxonomy) ?? (entry.semanticStem ? "semantic-stem-resolution" : entry.fixedContractId ? "fixed-contract-preservation" : "canonical-kind-name"),
      ownerId: entry.ownerId,
      collisionGroup: groupBySource.get(entry.sourcePath),
      referenceEdits: [],
      ...(sourceAuthority ? { sourceAuthority } : {}),
    }); })
    .sort((left, right) => generatorPathCompare(left.sourcePath, right.sourcePath) || generatorPathCompare(left.destinationPath, right.destinationPath));
  const generatedReferenceMoves = evidenceRemovals.flatMap((entry): TaxonomyMove[] => entry.authority.kind === "exact-owner-generated-source" || entry.authority.kind === "nested-cargo-generated-source" ? [{ operationId: entry.operationId, sourcePath: entry.sourcePath, destinationPath: entry.authority.destinationPath, sourcePreimage: entry.preimage, rationaleRule: entry.authority.kind === "nested-cargo-generated-source" ? "nested-cargo-package-projection-v1" : "readme-license-owner-projection-v1", ownerId: entry.ownerId, referenceEdits: [] }] : []);
  const references = buildReferenceEdits(inventory, [...preliminaryMoves, ...generatedReferenceMoves], taxonomy, options, referencePathIndex(inventory.entries.map((entry) => entry.sourcePath)), evidenceRemovals);
  const editsByTarget = new Map<string, ReferenceEdit[]>();
  for (const edit of references.edits) {
    const target = references.editTargets.get(referenceEditIdentity(edit));
    if (target === undefined) continue;
    const group = editsByTarget.get(target);
    if (group) group.push(edit);
    else editsByTarget.set(target, [edit]);
  }
  return {
    moves: preliminaryMoves.map((move) => ({ ...move, referenceEdits: editsByTarget.get(move.sourcePath) ?? [] })),
    edits: references.edits,
    editTargets: references.editTargets,
    resultHashes: references.resultHashes,
    resultSizes: references.resultSizes,
    unresolved: references.unresolved,
    collisionGroups: groups,
  };
}

export function packageGeneratorActivated(repoRoot: string, moves: readonly TaxonomyMove[], contract: GeneratorContractSpec, taxonomy: LoadedTaxonomy, removals: readonly TaxonomyEvidenceRemoval[]): boolean {
  if (contract.target === taxonomy.discoverySchema.generatorContracts["jco-package-adapter"]?.target) {
    semanticPackageAdapterPreview(repoRoot, "jcoprobe-guest", taxonomy.discoverySchema);
    return true;
  }
  if (contract.packageGeneration) {
    const generation = contract.packageGeneration, profile = parseSemanticPackageBrowserProfile(generation.browserProfile, taxonomy.discoverySchema.pathEmojiPolicy.genericEmojiIdentities);
    const catalogPath = assertLexicalInputOutsideOpaque(repoRoot, generation.catalogPath, "Current WGPU catalog activation", true), catalogState = lstatOrNull(catalogPath);
    if (!catalogState?.isFile() || catalogState.isSymbolicLink()) throw new Error("Current WGPU activation requires its no-follow catalog");
    const catalog = parseCanonicalWgpuPackageCatalog(readFileSync(catalogPath, "utf8"), generation.catalogSha256, profile, taxonomy.discoverySchema);
    const manifestPath = assertLexicalInputOutsideOpaque(repoRoot, catalog.ownerPath + "/" + catalog.packageRelativePath + "/Cargo.toml", "Current WGPU package activation", true), manifest = lstatOrNull(manifestPath);
    if (!manifest?.isFile() || manifest.isSymbolicLink()) throw new Error("Current WGPU activation requires its no-follow package manifest");
    return true;
  }
  const activation = contract.projectionActivation;
  if (!activation) return true;
  const source = lstatOrNull(assertLexicalInputOutsideOpaque(repoRoot, activation.sourceManifestPath, "Generator source activation", true));
  const destination = lstatOrNull(assertLexicalInputOutsideOpaque(repoRoot, activation.destinationManifestPath, "Generator canonical activation", true));
  if (!source && !destination) return false;
  if (source && (!source.isFile() || source.isSymbolicLink()) || destination && (!destination.isFile() || destination.isSymbolicLink()) || source && destination) throw new Error("Generator package activation requires one exact source or canonical manifest");
  const row = nestedCargoCatalog(repoRoot, taxonomy.discoverySchema)?.packages.find((row) => row.id === activation.packageId);
  if (!row || activation.sourceManifestPath !== `${row.sourceRoot}/Cargo.toml` || activation.destinationManifestPath !== `${row.destinationRoot}/Cargo.toml`) throw new Error("Generator package activation disagrees with its exact catalog");
  const relevant = moves.filter((move) => row.mappings.some((mapping) => mapping.sourcePath === move.sourcePath));
  const retired = removals.filter((removal): removal is TaxonomyEvidenceRemoval & { authority: Extract<TaxonomyRemovalAuthority, { kind: "nested-cargo-generated-source" }> } => removal.authority.kind === "nested-cargo-generated-source" && removal.authority.packageId === row.id);
  const projected = relevant.length + retired.length;
  if (projected > 0 && (projected !== row.mappings.length || !row.mappings.every((mapping) => mapping.disposition === "generated" ? retired.some((removal) => removal.sourcePath === mapping.sourcePath && removal.preimage.nodeKind === "file" && removal.preimage.contentHash === mapping.sourceHash && removal.preimage.size === mapping.sourceSize && removal.authority.kind === "nested-cargo-generated-source" && removal.authority.destinationPath === mapping.destinationPath && row.generatedSourceRetirements.some((entry) => entry.sourcePath === mapping.sourcePath && entry.generatorContractId === removal.authority.generatorContractId && entry.sourceMode === removal.preimage.mode)) : relevant.some((move) => move.sourcePath === mapping.sourcePath && move.destinationPath === mapping.destinationPath && move.sourcePreimage.contentHash === mapping.sourceHash && move.sourcePreimage.size === mapping.sourceSize)))) throw new Error("Generator package activation has an incomplete projection");
  if (source && projected === 0) return false;
  if (activation.packageId === "jcoprobe-guest") semanticPackageAdapterPreview(repoRoot, activation.packageId, taxonomy.discoverySchema);
  else semanticPackageGeneratedLeafPreview(repoRoot, activation.packageId, taxonomy.discoverySchema);
  return Boolean(destination) || projected === row.mappings.length;
}

/** 🧭️ A set of mutated paths indexed once for the two questions generator planning asks of every contract: does any mutation overlap a
 * root (equal, inside or containing it, under {@link pathsOverlap}'s normalization), and does any match a glob (under the path matcher's
 * normalization, where a glob's literal leading segments bound the candidates). */
interface MutationIndex {
  readonly overlapping: (root: string, admit?: (path: string) => boolean) => boolean;
  readonly matching: (pattern: string, matches: (path: string, pattern: string) => boolean) => boolean;
}

/** 🔎️ The members of code-unit-sorted `values` equal to `prefix` or below `prefix/`, found by binary search and one bounded scan. */
export function* sortedSubtree(values: readonly string[], prefix: string): Generator<string> {
  let low = 0, high = values.length;
  while (low < high) { const middle = (low + high) >>> 1; if (values[middle]! < prefix) low = middle + 1; else high = middle; }
  for (let index = low; index < values.length; index++) {
    const value = values[index]!;
    if (value === prefix || value.startsWith(`${prefix}/`)) yield value;
    else if (value.slice(0, prefix.length + 1) > `${prefix}/`) break;
  }
}

export function mutationIndex(paths: Iterable<string>): MutationIndex {
  const byNormalized = new Map<string, string[]>(), byMatcher = new Map<string, string[]>();
  for (const path of paths) {
    const normalized = normalizeRelative(path), matcherForm = path.replaceAll("\\", "/").replace(/^\.\//u, "").normalize("NFC");
    (byNormalized.get(normalized) ?? byNormalized.set(normalized, []).get(normalized)!).push(path);
    (byMatcher.get(matcherForm) ?? byMatcher.set(matcherForm, []).get(matcherForm)!).push(path);
  }
  const normalized = [...byNormalized.keys()].sort(), matcherForms = [...byMatcher.keys()].sort();
  const any = (values: readonly string[] | undefined, admit: (path: string) => boolean): boolean => values?.some(admit) ?? false;
  return {
    overlapping(root, admit = () => true) {
      const target = normalizeRelative(root);
      if (target === "") return [...byNormalized.values()].some((values) => any(values, admit));
      if (any(byNormalized.get(""), admit)) return true;
      for (let ancestor = target; ancestor; ancestor = ancestor.includes("/") ? ancestor.slice(0, ancestor.lastIndexOf("/")) : "") if (any(byNormalized.get(ancestor), admit)) return true;
      for (const value of sortedSubtree(normalized, target)) if (value !== target && any(byNormalized.get(value), admit)) return true;
      return false;
    },
    matching(pattern, matches) {
      const segments = pattern.normalize("NFC").split("/"), literal = segments.findIndex((segment) => /[*?[]/u.test(segment)), prefix = (literal < 0 ? segments : segments.slice(0, literal)).join("/");
      const candidates = prefix ? sortedSubtree(matcherForms, prefix) : matcherForms;
      for (const form of candidates) if (byMatcher.get(form)!.some((path) => matches(path, pattern))) return true;
      return false;
    },
  };
}

export function evidenceRemovalIncomingReferences(repoRoot: string, removal: TaxonomyEvidenceRemoval, plan: TaxonomyPlan, taxonomy: LoadedTaxonomy, ticketDir?: string, planAuthority?: Readonly<{ path: string; bytes: Uint8Array }>, transactionRoot?: string, journal?: MutableJournalRecord, cancelFile?: string, progress?: TaxonomyApplyOptions["progress"]): readonly string[] {
  const project = removal.authority.kind === "nested-cargo-generated-source" || removal.authority.kind === "exact-owner-generated-source" ? removalReferenceProjection(repoRoot, plan, journal) : undefined;
  return lexicalTargetIncomingReferences(repoRoot, new Set([removal.sourcePath]), removalIncomingIgnoredSourceRoots(removal), taxonomy, ticketDir, planAuthority, transactionRoot, plan, cancelFile, progress, project);
}
