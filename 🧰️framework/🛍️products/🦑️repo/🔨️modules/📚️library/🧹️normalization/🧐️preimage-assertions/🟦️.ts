import { requireRecord, type UnknownRecord } from "../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import { type LoadedTaxonomy } from "../🔣️taxonomy/📚️loaded/🟦️.ts";
import { type GeneratorContractSpec } from "../🔣️taxonomy/🏭️generators-packages/🟦️.ts";
import { normalizeRelative } from "../🛣️path/🟦️.ts";
import { sha256, lstatOrNull, assertNoFollowAncestors } from "../📁️input/🟦️.ts";
import { lstatSync, readFileSync, readdirSync, readlinkSync } from "node:fs";
import { canonicalJson } from "../../🧾️serialization/🔣️json/🟦️.ts";
import { generatorPreviewExecution } from "../../🔍️discovery/✅️owned-file-approvals/🟦️.ts";
import { semanticPackageProjectionCatalog } from "../../🔍️discovery/🎁️package-projection/🟦️.ts";
import { basename, dirname, join } from "node:path";
import { type TaxonomyEvidenceRemoval, type TaxonomyGeneratorNodeRecord, type TaxonomyInventoryEntry, type TaxonomyLeafPreimage, type TaxonomyMove, type TaxonomyPathPreimage, type TaxonomyPlan, type TaxonomyRegeneration, type TaxonomyRemovalAuthority, type TaxonomySymlinkTargetEdit, requireTaxonomyValue } from "../📜️plan-contracts/🟦️.ts";
import { absolutePath, isExcluded } from "../🏷️file-kind-matching/🟦️.ts";
import { generatorPathCompare, generatorPreviewJson, parseGeneratorPreviewManifest } from "../🧾️plan-parsing/🟦️.ts";
import { transactionBackupAuthorities } from "../🩹️recovery-authorities/🟦️.ts";
import { compilerPreviewInputRecords, previewNodeRecords, validatePreviewPreState } from "../🏭️generator-inputs/🟦️.ts";
import { exactOwnedFileCatalog, ticketImportantExactMutationCases, ticketManifestState } from "../🗺️artifact-projection-catalogs/🟦️.ts";

export function hashPath(path: string): string {
  const stat = lstatSync(path);
  if (stat.isSymbolicLink()) return sha256(readlinkSync(path));
  if (!stat.isFile()) throw new Error(`Expected file or symlink at ${path}`);
  return sha256(readFileSync(path));
}

export function inventoryLeafPreimage(entry: Pick<TaxonomyInventoryEntry, "nodeKind" | "contentHash" | "mode" | "size" | "symlinkTarget">): TaxonomyLeafPreimage {
  if (entry.nodeKind === "file") return { nodeKind: "file", contentHash: entry.contentHash, mode: entry.mode, size: entry.size };
  if (entry.nodeKind !== "symlink" || entry.symlinkTarget === undefined || sha256(entry.symlinkTarget) !== entry.contentHash || Buffer.byteLength(entry.symlinkTarget) !== entry.size) throw new Error("Inventory leaf lacks exact no-follow file/symlink authority");
  return { nodeKind: "symlink", contentHash: entry.contentHash, mode: entry.mode, size: entry.size, target: entry.symlinkTarget };
}

export function leafPreimage(path: string): TaxonomyLeafPreimage {
  const stat = lstatSync(path);
  if (!stat.isFile() && !stat.isSymbolicLink()) throw new Error(`Expected no-follow leaf at ${path}`);
  const nodeKind = stat.isSymbolicLink() ? "symlink" : "file";
  const bytes = nodeKind === "symlink" ? Buffer.from(readlinkSync(path)) : readFileSync(path);
  const core = { contentHash: sha256(bytes), mode: stat.mode & 0o7777, size: bytes.byteLength };
  return nodeKind === "symlink" ? { nodeKind, ...core, target: bytes.toString() } : { nodeKind, ...core };
}

export function leafPathPreimage(path: string): Extract<TaxonomyPathPreimage, { state: "file" | "symlink" }> {
  const leaf = leafPreimage(path);
  return leaf.nodeKind === "symlink" ? { state: "symlink", contentHash: leaf.contentHash, mode: leaf.mode, size: leaf.size, target: leaf.target } : { state: "file", contentHash: leaf.contentHash, mode: leaf.mode, size: leaf.size };
}

export function assertLeafPreimage(repoRoot: string, path: string, expected: TaxonomyLeafPreimage): void {
  const absolute = absolutePath(repoRoot, path);
  if (!lstatOrNull(absolute) || canonicalJson(leafPreimage(absolute)) !== canonicalJson(expected)) throw new Error(`Disposition preimage changed: ${path}`);
}

export function retargetedMovePreimage(move: TaxonomyMove, edit?: TaxonomySymlinkTargetEdit): TaxonomyLeafPreimage {
  if (!edit) return move.sourcePreimage;
  if (move.sourcePreimage.nodeKind !== "symlink" || edit.oldTarget !== move.sourcePreimage.target || edit.oldTargetHash !== move.sourcePreimage.contentHash) throw new Error(`Symlink edit is not bound to move preimage: ${move.sourcePath}`);
  return { nodeKind: "symlink", contentHash: edit.newTargetHash, mode: move.sourcePreimage.mode, size: Buffer.byteLength(edit.newTarget), target: edit.newTarget };
}

export function assertDirectoryOnlyTree(path: string): void {
  const stat = lstatSync(path);
  if (!stat.isDirectory() || stat.isSymbolicLink()) throw new Error(`Embedded root residual node is not a directory: ${path}`);
  for (const name of readdirSync(path).sort((left, right) => Buffer.from(left).compare(Buffer.from(right)))) assertDirectoryOnlyTree(join(path, name));
}

export function assertWritableAncestors(repoRoot: string, logicalPath: string): void {
  const target = absolutePath(repoRoot, logicalPath);
  for (let current = dirname(target); current !== repoRoot && current !== dirname(current); current = dirname(current)) {
    const stat = lstatOrNull(current);
    if (stat?.isSymbolicLink() || (stat && !stat.isDirectory())) throw new Error(`Destination ancestor is not a no-follow directory: ${logicalPath}`);
  }
}

export function canonicalDirectoryName(taxonomy: LoadedTaxonomy, kindId: string, slug: string, parentKindId?: string): string {
  const kind = taxonomy.directoryKinds.find((entry) => entry.id === kindId);
  if (!kind || !kind.slugRegex.test(slug) || ((kind.parentKindIds?.length ?? 0) > 0 && !kind.parentKindIds?.includes(parentKindId ?? ""))) throw new Error(`Taxonomy directory kind ${kindId} cannot own slug ${slug}`);
  return `${kind.emoji}${slug}`.normalize("NFC");
}

export function canonicalKindOnlyFilename(taxonomy: LoadedTaxonomy, kindId: string, extension: string): string {
  const kind = taxonomy.fileKinds.find((entry) => entry.id === kindId);
  if (!kind || !kind.extensionChains.includes(extension)) throw new Error(`Taxonomy file kind ${kindId} cannot own extension ${extension}`);
  return `${kind.emoji}${extension}`.normalize("NFC");
}

export function canonicalScopedKindOnlyFilename(taxonomy: LoadedTaxonomy, kindId: string, parentKindId: string, extension: string): string {
  const kind = taxonomy.schema.scopedFileKinds[kindId];
  const filename = `${kind?.emoji ?? ""}${extension}`.normalize("NFC");
  if (!kind || kind.parentDirectoryKindId !== parentKindId || !kind.extensionChains.includes(extension) || !new RegExp(kind.sourceFilenamePattern, "u").test(filename)) throw new Error(`Taxonomy scoped file kind ${kindId} cannot own ${filename} below ${parentKindId}`);
  return filename;
}

export function pathsOverlap(left: string, right: string): boolean {
  const a = normalizeRelative(left);
  const b = normalizeRelative(right);
  return a === b || a === "" || b === "" || a.startsWith(`${b}/`) || b.startsWith(`${a}/`);
}

function assertGeneratorNodeRecords(records: readonly TaxonomyGeneratorNodeRecord[], roots: readonly string[], label: string): void {
  const seen = new Set<string>();
  for (const record of records) {
    const path = normalizeRelative(record.path);
    if (path !== record.path || seen.has(path)) throw new Error(`${label} contains a duplicate or noncanonical path: ${record.path}`);
    if (!roots.some((root) => path === root || path.startsWith(`${root}/`))) throw new Error(`${label} path is outside registered roots: ${path}`);
    if (!["directory", "file", "symlink"].includes(record.nodeKind) || !/^[a-f0-9]{64}$/u.test(record.contentHash) || !Number.isSafeInteger(record.mode) || record.mode < 0 || record.mode > 0o7777) throw new Error(`${label} contains an invalid node record: ${path}`);
    if (record.nodeKind !== "directory" && (!Number.isSafeInteger(record.size) || record.size < 0) || record.nodeKind === "symlink" && (sha256(record.target) !== record.contentHash || Buffer.byteLength(record.target) !== record.size)) throw new Error(`${label} contains incomplete no-follow leaf evidence: ${path}`);
    seen.add(path);
  }
  if (records.some((record, index) => index > 0 && generatorPathCompare(requireTaxonomyValue(records[index - 1],"records[index - 1]").path, record.path) > 0)) throw new Error(`${label} must be path-sorted`);
}

function buildTargetRecord(repoRoot: string, ownerPath: string, target: string): UnknownRecord {
  const manifestPath = absolutePath(repoRoot, `${ownerPath}/📋️project.json`);
  const manifest = requireRecord(JSON.parse(readFileSync(manifestPath, "utf8")) as unknown, `Build manifest ${manifestPath}`);
  const separator = target.lastIndexOf(":");
  const project = target.slice(0, separator);
  const targetName = target.slice(separator + 1);
  const targets = requireRecord(manifest.targets, `Build manifest ${manifestPath}.targets`);
  if (manifest.name !== project || !Object.hasOwn(targets, targetName)) throw new Error(`Build manifest ${manifestPath} does not own target ${target}`);
  return requireRecord(targets[targetName], `Build target ${target}`);
}

function assertBuildTarget(repoRoot: string, ownerPath: string, target: string): void {
  buildTargetRecord(repoRoot, ownerPath, target);
}

export function assertGeneratorPreviewTarget(repoRoot: string, contract: GeneratorContractSpec): ReturnType<typeof generatorPreviewExecution> {
  const { ownerPath, previewTarget: target } = contract;
  if (!ownerPath || !target) throw new Error("Generator lacks an exact owner JSON preview command");
  const preview = buildTargetRecord(repoRoot, ownerPath, target);
  return generatorPreviewExecution(contract, { executor: preview.executor, options: requireRecord(preview.options, `Build target ${target}.options`) });
}

function assertRegenerationContract(regeneration: TaxonomyRegeneration, taxonomy: LoadedTaxonomy, repoRoot: string): GeneratorContractSpec {
  const contract = taxonomy.schema.generatorContracts[regeneration.contractId];
  if (!contract || contract.ownership !== "owned" || !contract.ownerPath || !contract.target || !contract.previewTarget) throw new Error(`Regeneration ${regeneration.id} does not reference an owned generator contract`);
  const roots = contract.outputRoots.map((output) => output.path).sort(generatorPathCompare);
  if (regeneration.cwd !== contract.ownerPath || canonicalJson(regeneration.command) !== canonicalJson(["bun", "repo", "run", contract.target])) throw new Error(`Regeneration ${regeneration.id} command is not schema-owned`);
  const expectedVerify = contract.checkTarget ? ["bun", "repo", "run", contract.checkTarget] : undefined;
  if (canonicalJson(regeneration.verifyCommand) !== canonicalJson(expectedVerify)) throw new Error(`Regeneration ${regeneration.id} verification command is not schema-owned`);
  if (canonicalJson([...regeneration.outputRoots].sort()) !== canonicalJson(roots)) throw new Error(`Regeneration ${regeneration.id} output roots do not match its contract`);
  assertGeneratorNodeRecords(regeneration.preOutputs, roots, `Regeneration ${regeneration.id} preOutputs`);
  assertGeneratorNodeRecords(regeneration.outputs, roots, `Regeneration ${regeneration.id} outputs`);
  assertGeneratorNodeRecords(regeneration.inputs, regeneration.inputs.map((input) => input.path), `Regeneration ${regeneration.id} inputs`);
  const preview = parseGeneratorPreviewManifest(`${generatorPreviewJson(regeneration.preview)}\n`, regeneration.contractId, roots, taxonomy.exclusions.map((entry) => entry.path));
  const compilerRecords = compilerPreviewInputRecords(repoRoot, contract, taxonomy, preview), compilerPaths = new Set(compilerRecords.map(row => row.path));
  for (const row of compilerRecords) if (canonicalJson(regeneration.inputs.find(input => input.path === row.path)) !== canonicalJson(row)) throw new Error(`Regeneration ${regeneration.id} omits its compiler input witness: ${row.path}`);
  for (const input of regeneration.inputs) if (!contract.inputDiscovery && !contract.inputPatterns.some((pattern) => taxonomy.pathMatcher.matches(input.path, pattern)) && !compilerPaths.has(input.path)) throw new Error(`Regeneration ${regeneration.id} input is not schema-owned: ${input.path}`);
  if (regeneration.previewManifestDigest !== sha256(`${generatorPreviewJson(preview)}\n`) || canonicalJson(regeneration.staleRemovals) !== canonicalJson(preview.staleRemovals) || canonicalJson(regeneration.outputs) !== canonicalJson(previewNodeRecords(preview))) throw new Error(`Regeneration ${regeneration.id} does not match its frozen preview manifest`);
  validatePreviewPreState(preview, regeneration.preOutputs);
  const identity = sha256(canonicalJson({ contractId: regeneration.contractId, cwd: regeneration.cwd, command: regeneration.command, verifyCommand: regeneration.verifyCommand, outputRoots: roots, inputs: regeneration.inputs, preOutputs: regeneration.preOutputs, outputs: regeneration.outputs, preview, previewManifestDigest: regeneration.previewManifestDigest, staleRemovals: regeneration.staleRemovals })).slice(0, 24);
  if (regeneration.id !== identity) throw new Error(`Regeneration ${regeneration.id} does not match canonical regeneration bytes`);
  assertBuildTarget(repoRoot, contract.ownerPath, contract.target);
  assertGeneratorPreviewTarget(repoRoot, contract);
  if (contract.checkTarget) assertBuildTarget(repoRoot, contract.ownerPath, contract.checkTarget);
  return contract;
}

export function removalAuthorityPaths(authority: TaxonomyRemovalAuthority): readonly string[] {
  if (authority.kind === "byte-and-mode-identical") return authority.members.flatMap((member) => [member.sourcePath, member.finalPath]);
  if (authority.kind === "owner-manifest-status") return [authority.manifestPath];
  if (authority.kind === "exact-path-mutation" || authority.kind === "exact-owner-generated-source" || authority.kind === "nested-cargo-generated-source") return [authority.catalogPath];
  throw Error("Closed taxonomy removal authority required");
}

export function removalIncomingIgnoredSourceRoots(removal: TaxonomyEvidenceRemoval): readonly string[] {
  if (removal.authority.kind === "exact-path-mutation" || removal.authority.kind === "exact-owner-generated-source" || removal.authority.kind === "nested-cargo-generated-source") return [removal.sourcePath, removal.authority.catalogPath];
  
  return [removal.sourcePath];
}

export function assertTicketImportantRemovalAuthority(repoRoot: string, removal: TaxonomyEvidenceRemoval, taxonomy: LoadedTaxonomy): void {
  if (removal.authority.kind === "nested-cargo-generated-source") {
    const contract = taxonomy.discoverySchema.semanticPackageProjectionContracts["nested-cargo-packages-v1"], authority = removal.authority;
    const row = semanticPackageProjectionCatalog(repoRoot, taxonomy.discoverySchema)?.packages.find((entry) => entry.id === authority.packageId);
    const declaration = row?.generatedSourceRetirements.find((entry) => entry.sourcePath === removal.sourcePath), mapping = row?.mappings.find((entry) => entry.sourcePath === removal.sourcePath);
    if (!contract || !declaration || !mapping || mapping.disposition !== "generated" || contract.authorityCatalogPath !== authority.catalogPath || contract.authorityCatalogSha256 !== authority.catalogContentHash || declaration.generatorContractId !== authority.generatorContractId || declaration.destinationPath !== authority.destinationPath || mapping.sourceHash !== authority.sourcePreimage.contentHash || mapping.sourceSize !== authority.sourcePreimage.size || declaration.sourceMode !== authority.sourcePreimage.mode || canonicalJson(removal.preimage) !== canonicalJson(authority.sourcePreimage)) throw new Error("Nested Cargo generated retirement authority changed: " + removal.sourcePath);
    return;
  }
  if (removal.authority.kind === "exact-owner-generated-source") {
    const contract = taxonomy.schema.semanticOwnedFileProjectionContracts["readme-license-owner-leaves-v1"];
    const owner = exactOwnedFileCatalog(repoRoot, taxonomy)?.entries.find((entry) => entry.sourcePath === removal.sourcePath);
    const authority = removal.authority;
    if (contract?.contractKind !== "exact-owner-path-catalog" || !owner || owner.generatorOwnerId !== authority.generatorContractId || owner.destinationPath !== authority.destinationPath || contract.authorityCatalogPath !== authority.catalogPath || contract.authorityCatalogSha256 !== authority.catalogContentHash || canonicalJson(removal.preimage) !== canonicalJson(authority.outputPreimage) || owner.preimage.sha256 !== authority.outputPreimage.contentHash || owner.preimage.size !== authority.outputPreimage.size || Number.parseInt(owner.preimage.mode, 8) !== authority.outputPreimage.mode) throw new Error("Exact generated owner retirement authority changed: " + removal.sourcePath);
    return;
  }
  if (removal.authority.kind !== "owner-manifest-status") return;
  const authority = removal.authority;
  if (dirname(removal.sourcePath) !== authority.ownerPath || basename(removal.sourcePath) !== "📌️important.md" || authority.manifestPath !== `${authority.ownerPath}/🎫️ticket.json`) throw new Error(`Ticket important removal escapes its owner: ${removal.sourcePath}`);
  if (!taxonomy.pathMatcher.matches(authority.ownerPath, requireTaxonomyValue(taxonomy.schema.fixedDirectoryContracts["ticket-slug"],"ticket directory contract").pathPattern) || !taxonomy.pathMatcher.matches(authority.manifestPath, requireTaxonomyValue(taxonomy.schema.fixedFilenameContracts["ticket-manifest"],"ticket manifest contract").pathPattern)) throw new Error(`Ticket important removal owner contracts changed: ${removal.sourcePath}`);
  assertLeafPreimage(repoRoot, authority.manifestPath, authority.manifestPreimage);
  if (ticketManifestState(readFileSync(absolutePath(repoRoot, authority.manifestPath), "utf8")) !== "closed") throw new Error(`Ticket important removal manifest status changed: ${authority.manifestPath}`);
  const { authorityDigest: _digest, ...digestible } = authority;
  if (authority.authorityDigest !== sha256(canonicalJson(digestible))) throw new Error(`Ticket important removal authority digest changed: ${removal.sourcePath}`);
}

export function assertTicketImportantExactRemovalAuthority(repoRoot: string, removal: TaxonomyEvidenceRemoval): void {
  if (removal.authority.kind !== "exact-path-mutation") return;
  const authority = removal.authority;
  const exact = ticketImportantExactMutationCases(repoRoot).find((entry) => entry.caseId === authority.caseId);
  if (!exact || exact.disposition !== "remove" || exact.catalogPath !== authority.catalogPath || exact.catalogContentHash !== authority.catalogContentHash || exact.sourcePath !== removal.sourcePath || canonicalJson(exact.sourcePreimage) !== canonicalJson(removal.preimage)) throw new Error(`Ticket important exact mutation authority changed: ${authority.caseId}`);
  const { authorityDigest: _digest, ...digestible } = authority;
  if (authority.authorityDigest !== sha256(canonicalJson(digestible))) throw new Error(`Ticket important exact mutation digest changed: ${authority.caseId}`);
}

export function assertPlanOutsideTransaction(plan: TaxonomyPlan, transactionRoot: string, taxonomy: LoadedTaxonomy, repoRoot: string): void {
  transactionBackupAuthorities(plan);
  for (const removal of plan.evidenceRemovals) if (removal.authority.kind === "nested-cargo-generated-source") {
    const authority = removal.authority;
    const owners = plan.regenerations.filter((regeneration) => regeneration.contractId === authority.generatorContractId && regeneration.outputs.some((output) => output.path === authority.destinationPath && output.nodeKind === "file" && output.mode === authority.sourcePreimage.mode));
    if (owners.length !== 1 || owners[0]!.outputRoots.some((root) => pathsOverlap(root, removal.sourcePath))) throw new Error("Nested Cargo generated retirement requires exactly one disjoint canonical generator: " + removal.sourcePath);
  }
  const paths = [
    ...(plan.scope ? [plan.scope] : []),
    ...plan.moves.flatMap((move) => [move.sourcePath, move.destinationPath]),
    ...plan.embeddedTicketRoots.flatMap((root) => [root.sourceMetadataRoot, root.sourceTicketRoot, root.canonicalTicketRoot]),
    ...plan.embeddedTicketRootRelocations.flatMap((entry) => [entry.sourcePath, entry.destinationPath]),
    ...plan.symlinkTargetEdits.flatMap((entry) => [entry.sourcePath, entry.finalPath, entry.logicalTargetSourcePath, entry.logicalTargetFinalPath]),
    ...plan.evidenceRemovals.flatMap((entry) => [entry.sourcePath, ...removalAuthorityPaths(entry.authority)]),
    ...plan.destinationAncestorPreimages.map((entry) => entry.path),
    ...plan.edits.map((edit) => edit.path),
    ...plan.regenerations.flatMap((regeneration) => [regeneration.cwd, ...regeneration.outputRoots, ...regeneration.inputs.map((input) => input.path), ...regeneration.preOutputs.map((output) => output.path), ...regeneration.outputs.map((output) => output.path), ...regeneration.staleRemovals]),
  ];
  const excludedPath = paths.find((path) => isExcluded(path, taxonomy));
  if (excludedPath) throw new Error(`Plan path crosses a lexical opaque exclusion: ${excludedPath}`);
  const ancestorAuthority = new Map(plan.destinationAncestorPreimages.map((entry) => [entry.path, entry]));
  const overlap = paths.find((path) => pathsOverlap(path, transactionRoot) && !(ancestorAuthority.get(path)?.state === "directory" && transactionRoot.startsWith(`${path}/`)));
  if (overlap) throw new Error(`Plan path overlaps taxonomy transaction root: ${overlap} <> ${transactionRoot}`);
  assertNoFollowAncestors(repoRoot, absolutePath(repoRoot, transactionRoot), "taxonomy transaction root", true);
  for (const path of paths) assertNoFollowAncestors(repoRoot, absolutePath(repoRoot, path), `plan path ${path}`);
  const directoryRoles = [
    ...(plan.scope ? [plan.scope] : []),
    ...plan.embeddedTicketRoots.flatMap((root) => [root.sourceMetadataRoot, root.sourceTicketRoot, root.canonicalTicketRoot]),
    ...plan.regenerations.flatMap((regeneration) => [regeneration.cwd, ...regeneration.outputRoots]),
  ];
  for (const path of directoryRoles) assertNoFollowAncestors(repoRoot, absolutePath(repoRoot, path), `plan directory ${path}`, true);
  const sourceRoles = new Map<string, string>();
  const ownSource = (path: string, role: string): void => { const prior = sourceRoles.get(path); if (prior) throw new Error(`Plan source has conflicting ${prior} and ${role} operations: ${path}`); sourceRoles.set(path, role); };
  for (const move of plan.moves) ownSource(move.sourcePath, "move");
  for (const relocation of plan.embeddedTicketRootRelocations) ownSource(relocation.sourcePath, "embedded relocation");
  for (const removal of plan.evidenceRemovals) ownSource(removal.sourcePath, "evidence removal");
  const destinations = [...plan.moves.map((entry) => entry.destinationPath), ...plan.embeddedTicketRootRelocations.map((entry) => entry.destinationPath)];
  if (new Set(destinations).size !== destinations.length || destinations.some((path, index) => destinations.some((candidate, other) => index !== other && pathsOverlap(path, candidate)))) throw new Error("Plan contains duplicate or overlapping move/relocation destinations");
  const removalSources = new Set(plan.evidenceRemovals.map((entry) => entry.sourcePath));
  if (destinations.some((path) => removalSources.has(path))) throw new Error("Plan destination overlaps an evidence-removal source");
  const relocationSources = new Set(plan.embeddedTicketRootRelocations.map((entry) => entry.sourcePath));
  if (plan.moves.some((entry) => relocationSources.has(entry.destinationPath)) || plan.embeddedTicketRootRelocations.some((entry) => sourceRoles.has(entry.destinationPath))) throw new Error("Move/relocation destination overlaps another mutable source");
  if (plan.embeddedTicketRoots.some((root, index) => plan.embeddedTicketRoots.some((candidate, other) => index !== other && pathsOverlap(root.sourceMetadataRoot, candidate.sourceMetadataRoot)))) throw new Error("Embedded metadata roots overlap");
  for (const root of plan.embeddedTicketRoots) {
    const forbidden = [
      ...plan.moves.flatMap((entry) => [entry.sourcePath, entry.destinationPath]),
      ...plan.symlinkTargetEdits.flatMap((entry) => [entry.sourcePath, entry.finalPath, entry.logicalTargetSourcePath, entry.logicalTargetFinalPath]),
      ...plan.edits.map((entry) => entry.path),
      ...plan.regenerations.flatMap((entry) => [entry.cwd, ...entry.outputRoots, ...entry.inputs.map((input) => input.path), ...entry.preOutputs.map((output) => output.path), ...entry.outputs.map((output) => output.path), ...entry.staleRemovals]),
    ].find((path) => pathsOverlap(path, root.sourceMetadataRoot));
    if (forbidden) throw new Error(`Operation conflicts with embedded metadata-root ownership: ${forbidden} <> ${root.sourceMetadataRoot}`);
  }
  if (new Set(plan.symlinkTargetEdits.map((entry) => entry.sourcePath)).size !== plan.symlinkTargetEdits.length || new Set(plan.symlinkTargetEdits.map((entry) => entry.finalPath)).size !== plan.symlinkTargetEdits.length) throw new Error("Symlink target edits do not have unique source/final paths");
  for (const edit of plan.symlinkTargetEdits) {
    const move = plan.moves.filter((candidate) => candidate.sourcePath === edit.sourcePath && candidate.destinationPath === edit.finalPath);
    if (edit.sourcePath !== edit.finalPath && move.length !== 1) throw new Error(`Symlink target edit is not bound to its exact move: ${edit.sourcePath}`);
    if (plan.evidenceRemovals.some((entry) => entry.sourcePath === edit.sourcePath) || plan.embeddedTicketRootRelocations.some((entry) => entry.sourcePath === edit.sourcePath) || plan.edits.some((entry) => entry.path === edit.finalPath)) throw new Error(`Symlink target edit conflicts with another mutation: ${edit.sourcePath}`);
  }
  for (const edit of plan.edits) {
    const sourceMove = plan.moves.find((move) => move.sourcePath === edit.path && move.destinationPath !== edit.path);
    if (sourceMove || plan.evidenceRemovals.some((entry) => entry.sourcePath === edit.path) || plan.embeddedTicketRootRelocations.some((entry) => entry.sourcePath === edit.path || entry.destinationPath === edit.path)) throw new Error(`Text edit conflicts with a mutable source: ${edit.path}`);
  }
  for (const regeneration of plan.regenerations) {
    assertRegenerationContract(regeneration, taxonomy, repoRoot);
    const conflict = [...plan.moves.flatMap((move) => [move.sourcePath, move.destinationPath]), ...plan.embeddedTicketRoots.flatMap((entry) => [entry.sourceMetadataRoot, entry.sourceTicketRoot, entry.canonicalTicketRoot]), ...plan.embeddedTicketRootRelocations.flatMap((entry) => [entry.sourcePath, entry.destinationPath]), ...plan.evidenceRemovals.flatMap((entry) => [entry.sourcePath, ...removalAuthorityPaths(entry.authority)]), ...plan.symlinkTargetEdits.flatMap((entry) => [entry.sourcePath, entry.finalPath]), ...plan.edits.map((edit) => edit.path)].find((path) => regeneration.outputRoots.some((root) => pathsOverlap(path, root)));
    if (conflict) throw new Error(`Generated output must be regenerated source-first, not moved or edited directly: ${conflict}`);
  }
  type MutablePathRole = Readonly<{ path: string; role: "move-source" | "move-destination" | "relocation-source" | "relocation-destination" | "removal-source" | "root-source" | "symlink-source" | "symlink-final" | "edit" | "generator-output"; owner: string }>;
  const roles: MutablePathRole[] = [
    ...plan.moves.flatMap((entry) => [{ path: entry.sourcePath, role: "move-source" as const, owner: entry.operationId }, { path: entry.destinationPath, role: "move-destination" as const, owner: entry.operationId }]),
    ...plan.embeddedTicketRootRelocations.flatMap((entry) => [{ path: entry.sourcePath, role: "relocation-source" as const, owner: entry.operationId }, { path: entry.destinationPath, role: "relocation-destination" as const, owner: entry.operationId }]),
    ...plan.evidenceRemovals.map((entry) => ({ path: entry.sourcePath, role: "removal-source" as const, owner: entry.operationId })),
    ...plan.embeddedTicketRoots.map((entry) => ({ path: entry.sourceMetadataRoot, role: "root-source" as const, owner: entry.operationId })),
    ...plan.symlinkTargetEdits.flatMap((entry) => [{ path: entry.sourcePath, role: "symlink-source" as const, owner: entry.operationId }, { path: entry.finalPath, role: "symlink-final" as const, owner: entry.operationId }]),
    ...[...new Set(plan.edits.map((entry) => entry.path))].map((path) => ({ path, role: "edit" as const, owner: path })),
    ...plan.regenerations.flatMap((entry) => entry.outputRoots.map((path) => ({ path, role: "generator-output" as const, owner: entry.id }))),
  ];
  const allowedOverlap = (left: MutablePathRole, right: MutablePathRole): boolean => {
    if (left.role === "edit" && right.role === "edit" && left.path === right.path) return true;
    if (left.owner === right.owner && left.path === right.path && new Set([left.role, right.role]).has("symlink-source") && new Set([left.role, right.role]).has("symlink-final")) return true;
    const pair = new Set([left.role, right.role]);
    if (pair.has("move-source") && pair.has("symlink-source") || pair.has("move-destination") && pair.has("symlink-final")) {
      const move = left.role.startsWith("move-") ? left : right;
      const symlink = left.role.startsWith("symlink-") ? left : right;
      return plan.symlinkTargetEdits.some((entry) => entry.operationId === symlink.owner && plan.moves.some((candidate) => candidate.operationId === move.owner && candidate.sourcePath === entry.sourcePath && candidate.destinationPath === entry.finalPath));
    }
    if (pair.has("move-destination") && pair.has("edit") && left.path === right.path) return true;
    if (pair.has("root-source") && (pair.has("relocation-source") || pair.has("removal-source"))) {
      const root = left.role === "root-source" ? left : right, child = left.role === "root-source" ? right : left;
      return child.path.startsWith(`${root.path}/`) && (child.role === "relocation-source" ? plan.embeddedTicketRootRelocations.some((entry) => entry.operationId === child.owner && entry.embeddedTicketRootId === root.owner) : plan.evidenceRemovals.some((entry) => entry.operationId === child.owner && entry.embeddedTicketRootId === root.owner));
    }
    return false;
  };
  for (let leftIndex = 0; leftIndex < roles.length; leftIndex++) for (let rightIndex = leftIndex + 1; rightIndex < roles.length; rightIndex++) {
    const left = requireTaxonomyValue(roles[leftIndex],"left mutable role"), right = requireTaxonomyValue(roles[rightIndex],"right mutable role");
    if (pathsOverlap(left.path, right.path) && !allowedOverlap(left, right)) throw new Error(`Plan mutable path roles overlap: ${left.role}:${left.path} <> ${right.role}:${right.path}`);
  }
}
