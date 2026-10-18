import { type LoadedTaxonomy } from "../🔣️taxonomy/📚️loaded/🟦️.ts";
import { type SemanticPathProjectionReferenceConsumerContract } from "../🔣️taxonomy/🧭️path-projection/🟦️.ts";
import { inScope } from "../🛣️path/🟦️.ts";
import { assertLexicalInputOutsideOpaque, lstatOrNull } from "../📁️input/🟦️.ts";
import { readFileSync, readdirSync } from "node:fs";
import { renderArtifactPathProjectionRoot } from "../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { dirname } from "node:path";
import { type TaxonomyInventory, type TaxonomyMove, type TaxonomyPlan, type TaxonomyViolation, requireTaxonomyValue } from "../📜️plan-contracts/🟦️.ts";
import { absolutePath, isExcluded, stableViolations, violation } from "../🏷️file-kind-matching/🟦️.ts";
import { generatorPathCompare } from "../🧾️plan-parsing/🟦️.ts";
import { applyEditsToContent } from "../✂️reference-edits/🟦️.ts";
import { referenceEditsByPath } from "../🧱️inventory-leaf-stamps/🟦️.ts";
import { explicitTicketRows, gitRows, scopedGitPathspec, untrackedGitPaths } from "../📸️repository-capture/🟦️.ts";
import { type ArtifactProjectionRationale, artifactProjectionContracts } from "../🗺️artifact-projection-catalogs/🟦️.ts";
import { canonicalDirectoryName, removalAuthorityPaths } from "../🧐️preimage-assertions/🟦️.ts";
import { artifactProjectionSourceLocation } from "../📂️owned-file-projection/🟦️.ts";
import { isTransactionRepositoryAuthorityError } from "../🏛️repository-authority/🟦️.ts";
import { textualPath } from "../🧮️reference-digests/🟦️.ts";

interface ArtifactStaleGroup {
  readonly id: string;
  readonly rationaleRule: ArtifactProjectionRationale;
  readonly ownerRoot: string;
  readonly markers: readonly string[];
}

function isProjectionConsumerPath(contract: SemanticPathProjectionReferenceConsumerContract, path: string): boolean {
  return contract.sourcePathIdentities.includes(path) && new RegExp(contract.sourcePathPattern, "u").test(path);
}

function artifactStaleGroups(paths: Iterable<string>, taxonomy: LoadedTaxonomy): readonly ArtifactStaleGroup[] {
  const values = [...new Set(paths)];
  const artifacts = canonicalDirectoryName(taxonomy, "artifacts", "artifacts");
  const consumers = Object.values(taxonomy.schema.semanticPathProjectionReferenceConsumerContracts);
  const rows = new Map<string, ArtifactStaleGroup>();
  for (const { id, contract } of artifactProjectionContracts(taxonomy)) for (const path of values) {
    const segments = path.split("/");
    for (let index = 1; index < segments.length; index++) {
      if (segments[index - 1] !== artifacts || segments[index] !== contract.sourceArtifactMemberName) continue;
      const ownerRoot = segments.slice(0, index - 1).join("/");
      if (!ownerRoot) continue;
      const markers = [...new Set(consumers.filter((consumer) => consumer.projectionContractId === id).flatMap((consumer) => consumer.staleMarkers))].sort(generatorPathCompare);
      if (markers.length === 0) continue;
      rows.set(`${id}\u0000${ownerRoot}`, { id, rationaleRule: contract.rationaleRule, ownerRoot, markers });
    }
  }
  return [...rows.values()].sort((left, right) => generatorPathCompare(left.ownerRoot, right.ownerRoot) || left.id.localeCompare(right.id));
}

/** 🎯️ The stale groups whose markers a file's content is checked for: groups it lies inside, or whose projection it consumes. */
function staleProjectionGroupsAt(path: string, groups: readonly ArtifactStaleGroup[], taxonomy: LoadedTaxonomy): readonly ArtifactStaleGroup[] {
  if (groups.length === 0) return groups;
  const consumers = Object.values(taxonomy.schema.semanticPathProjectionReferenceConsumerContracts);
  return groups.filter((group) => path === group.ownerRoot || path.startsWith(`${group.ownerRoot}/`) || consumers.some((contract) => contract.projectionContractId === group.id && isProjectionConsumerPath(contract, path)));
}

function staleProjectionContentViolations(path: string, content: string, groups: readonly ArtifactStaleGroup[], taxonomy: LoadedTaxonomy): readonly TaxonomyViolation[] {
  const rows: TaxonomyViolation[] = [];
  for (const group of staleProjectionGroupsAt(path, groups, taxonomy)) {
    for (const marker of group.markers) for (let index = content.indexOf(marker); index >= 0; index = content.indexOf(marker, index + marker.length)) rows.push(violation("projection-old-token-stale", path, `Old ${group.rationaleRule} token remains at raw offset ${index}`));
  }
  return rows;
}

function activeProjectionContractIds(plan: TaxonomyPlan, taxonomy: LoadedTaxonomy): ReadonlySet<string> {
  const ids = new Set<string>();
  for (const { id, contract } of artifactProjectionContracts(taxonomy)) if (plan.moves.some((move) => move.rationaleRule === contract.rationaleRule)) ids.add(id);
  return ids;
}

function declaredProjectionConsumerPaths(plan: TaxonomyPlan, taxonomy: LoadedTaxonomy, inventory?: TaxonomyInventory): readonly string[] {
  const active = new Set(activeProjectionContractIds(plan, taxonomy));
  if (inventory) for (const group of artifactStaleGroups(inventory.entries.map((entry) => entry.normalizedPath), taxonomy)) active.add(group.id);
  return [...new Set(Object.values(taxonomy.schema.semanticPathProjectionReferenceConsumerContracts)
    .filter((contract) => active.has(contract.projectionContractId))
    .flatMap((contract) => contract.sourcePathIdentities))]
    .sort(generatorPathCompare);
}

export function planVerificationCandidatePaths(repoRoot: string, plan: TaxonomyPlan, taxonomy: LoadedTaxonomy, ticketDir?: string): readonly string[] {
  const paths = new Set<string>();
  const pathspec = scopedGitPathspec(repoRoot, plan.scope, taxonomy);
  for (const row of gitRows(repoRoot, taxonomy, pathspec)) if (!isExcluded(row.path, taxonomy) && inScope(row.path, plan.scope) && lstatOrNull(absolutePath(repoRoot, row.path))) paths.add(row.path);
  for (const path of untrackedGitPaths(repoRoot, taxonomy, pathspec)) if (!isExcluded(path, taxonomy) && inScope(path, plan.scope)) paths.add(path);
  if (ticketDir) for (const row of explicitTicketRows(repoRoot, ticketDir, taxonomy, plan.scope)) if (!isExcluded(row.path, taxonomy) && inScope(row.path, plan.scope)) paths.add(row.path);
  for (const path of [
    ...plan.moves.flatMap((entry) => [entry.sourcePath, entry.destinationPath]),
    ...plan.embeddedTicketRoots.flatMap((entry) => [entry.sourceMetadataRoot, entry.sourceTicketRoot, entry.canonicalTicketRoot]),
    ...plan.embeddedTicketRootRelocations.flatMap((entry) => [entry.sourcePath, entry.destinationPath]),
    ...plan.symlinkTargetEdits.flatMap((entry) => [entry.sourcePath, entry.finalPath, entry.logicalTargetSourcePath, entry.logicalTargetFinalPath]),
    ...plan.evidenceRemovals.flatMap((entry) => [entry.sourcePath, ...removalAuthorityPaths(entry.authority)]),
    ...plan.edits.map((entry) => entry.path),
    ...plan.regenerations.flatMap((entry) => [entry.cwd, ...entry.outputRoots, ...entry.inputs.map((input) => input.path), ...entry.preOutputs.map((output) => output.path), ...entry.outputs.map((output) => output.path), ...entry.staleRemovals]),
    ...declaredProjectionConsumerPaths(plan, taxonomy),
  ]) if (!isExcluded(path, taxonomy)) paths.add(path);
  return [...paths].sort(generatorPathCompare);
}

export function projectionStaleViolations(repoRoot: string, plan: TaxonomyPlan, taxonomy: LoadedTaxonomy, inventory?: TaxonomyInventory, ticketDir?: string): readonly TaxonomyViolation[] {
  if (inventory) {
    const moveBySource = new Map(plan.moves.map((move) => [move.sourcePath, move.destinationPath]));
    const finalPaths = inventory.entries.map((entry) => moveBySource.get(entry.sourcePath) ?? entry.normalizedPath);
    const groups = artifactStaleGroups(finalPaths, taxonomy);
    const rows: TaxonomyViolation[] = [];
    const editsAt = referenceEditsByPath(plan.edits);
    for (const entry of inventory.entries.filter((candidate) => candidate.nodeKind === "file" && textualPath(candidate.sourcePath))) {
      const path = moveBySource.get(entry.sourcePath) ?? entry.normalizedPath;
      if (staleProjectionGroupsAt(path, groups, taxonomy).length === 0) continue;
      let content: string;
      try {
        content = new TextDecoder("utf-8", { fatal: true }).decode(readFileSync(absolutePath(repoRoot, entry.sourcePath)));
        const edits = editsAt.get(path) ?? [];
        if (edits.length > 0) content = applyEditsToContent(content, edits);
      } catch (error) {
        if (isTransactionRepositoryAuthorityError(error)) throw error;
        continue;
      }
      rows.push(...staleProjectionContentViolations(path, content, groups, taxonomy));
    }
    const admitted = new Set(inventory.entries.map((entry) => entry.sourcePath));
    for (const path of declaredProjectionConsumerPaths(plan, taxonomy, inventory).filter((candidate) => !admitted.has(candidate))) {
      const absolute = assertLexicalInputOutsideOpaque(repoRoot, path, "Declared projection consumer", true);
      const stat = lstatOrNull(absolute);
      if (!stat?.isFile() || stat.isSymbolicLink() || stat.size > 16 * 1024 * 1024 || !textualPath(path)) continue;
      try {
        let content = new TextDecoder("utf-8", { fatal: true }).decode(readFileSync(absolute));
        const edits = editsAt.get(path) ?? [];
        if (edits.length > 0) content = applyEditsToContent(content, edits);
        rows.push(...staleProjectionContentViolations(path, content, groups, taxonomy));
      } catch (error) { if (isTransactionRepositoryAuthorityError(error)) throw error; }
    }
    return stableViolations(rows);
  }
  const paths = new Set(planVerificationCandidatePaths(repoRoot, plan, taxonomy, ticketDir));
  const groups = artifactStaleGroups(paths, taxonomy);
  if (groups.length === 0) return [];
  const rows: TaxonomyViolation[] = [];
  for (const path of [...paths].filter(textualPath).sort(generatorPathCompare)) {
    if (isExcluded(path, taxonomy)) continue;
    const absolute = absolutePath(repoRoot, path);
    const stat = lstatOrNull(absolute);
    if (!stat?.isFile() || stat.size > 16 * 1024 * 1024) continue;
    try {
      const content = new TextDecoder("utf-8", { fatal: true }).decode(readFileSync(absolute));
      rows.push(...staleProjectionContentViolations(path, content, groups, taxonomy));
    } catch (error) { if (isTransactionRepositoryAuthorityError(error)) throw error; }
  }
  return stableViolations(rows);
}

export function artifactProjectionPostApplyViolations(repoRoot: string, plan: TaxonomyPlan, taxonomy: LoadedTaxonomy, prunableSourceParents: ReadonlySet<string>): readonly TaxonomyViolation[] {
  const rows: TaxonomyViolation[] = [];
  for (const { id, contract } of artifactProjectionContracts(taxonomy)) {
    const moves = plan.moves.filter((move) => move.rationaleRule === contract.rationaleRule);
    const groups = new Map<string, TaxonomyMove[]>();
    for (const move of moves) {
      const location = artifactProjectionSourceLocation(move.sourcePath, contract, taxonomy);
      if (location) groups.set(location.sourceRoot, [...(groups.get(location.sourceRoot) ?? []), move]);
    }
    for (const [sourceRoot, group] of [...groups].sort(([left], [right]) => generatorPathCompare(left, right))) {
      if (lstatOrNull(absolutePath(repoRoot, sourceRoot)) && !prunableSourceParents.has(sourceRoot)) rows.push(violation("projection-source-directory-stale", sourceRoot, `${id} source owner retains a nonempty or unplanned child outside exact move-source parent pruning`));
      const location = artifactProjectionSourceLocation(sourceRoot, contract, taxonomy);
      if (!location) {
        rows.push(violation("projection-apply-group-unresolved", sourceRoot, `${id} source root cannot be reconstructed from its frozen contract`));
        continue;
      }
      const rendered = renderArtifactPathProjectionRoot({ artifactRoot: location.artifactRoot, contractId: id, sourceRoot }, taxonomy.discoverySchema);
      if (rendered.problems.length > 0) {
        rows.push(violation("projection-apply-group-unresolved", sourceRoot, rendered.problems.join(" | ")));
        continue;
      }
      const expected = new Set<string>([`directory\u0000${rendered.destinationRoot}`]);
      for (const move of group) {
        if (lstatOrNull(absolutePath(repoRoot, move.sourcePath))) rows.push(violation("projection-source-file-stale", move.sourcePath, "Projected source file remains after staged move installation"));
        const destination = lstatOrNull(absolutePath(repoRoot, move.destinationPath));
        if (!destination?.isFile() || destination.isSymbolicLink()) rows.push(violation("projection-destination-file-invalid", move.destinationPath, "Projected destination is missing, non-file, or a symlink"));
        expected.add(`file\u0000${move.destinationPath}`);
        for (let path = dirname(move.destinationPath); path === rendered.destinationRoot || path.startsWith(`${rendered.destinationRoot}/`); path = dirname(path)) {
          expected.add(`directory\u0000${path}`);
          if (path === rendered.destinationRoot) break;
        }
      }
      const actual = new Set<string>();
      const walk = (path: string): void => {
        if (isExcluded(path, taxonomy)) throw new Error(`Artifact projection destination crosses opaque path ${path}`);
        const stat = lstatOrNull(absolutePath(repoRoot, path));
        if (!stat) return;
        if (stat.isSymbolicLink()) {
          rows.push(violation("projection-bundle-symlink", path, "Projected destination contains a symlink"));
          return;
        }
        actual.add(`${stat.isDirectory() ? "directory" : "file"}\u0000${path}`);
        if (stat.isDirectory()) for (const name of readdirSync(absolutePath(repoRoot, path)).sort((left, right) => Buffer.from(left).compare(Buffer.from(right)))) walk(`${path}/${name}`);
      };
      walk(rendered.destinationRoot);
      const missing = [...expected].filter((row) => !actual.has(row));
      const unexpected = [...actual].filter((row) => !expected.has(row));
      if (missing.length > 0 || unexpected.length > 0) rows.push(violation("projection-apply-descendants-invalid", rendered.destinationRoot, `${id} exact descendant mismatch: ${missing.length} missing, ${unexpected.length} unexpected`));
    }
    if ([...groups.values()].reduce((count, group) => count + group.length, 0) !== moves.length && moves.length > 0) rows.push(violation("projection-apply-group-unresolved", requireTaxonomyValue(moves[0],"moves[0]").sourcePath, `${id} has moves outside its exact source root`));
  }
  return stableViolations(rows);
}
