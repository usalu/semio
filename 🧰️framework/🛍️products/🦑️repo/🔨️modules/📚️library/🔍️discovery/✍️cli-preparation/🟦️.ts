import type { Taxonomy } from "../📑️taxonomy-document/🟦️.ts";
import type { TaxonomyCliWritePreparationFacts } from "../🛠️generator-commands/🟦️.ts";
import { loadTaxonomy, scopedFileKindIdForSourcePath, semanticDirectoryKindId } from "../🔐️taxonomy-authority/🟦️.ts";

function taxonomyCliWritePreparationProblems(facts: Readonly<{ parentKindId: string; directoryName: string; leafNames: readonly string[] }>, directoryKindId: "transaction-edit-write-preparation" | "transaction-backup-write-preparation", leafName: "🚧️.edit" | "🚧️.backup", scopedKindId: "transaction-edit-write-candidate" | "transaction-backup-write-candidate", pathPrefix: string, taxonomy: Taxonomy): string[] {
  const problems: string[] = [];
  const kindId = semanticDirectoryKindId(facts.directoryName, taxonomy, { parentKindId: facts.parentKindId });
  if (kindId !== directoryKindId) problems.push(`${directoryKindId} must resolve below its exact preparation owner.`);
  if (new Set(facts.leafNames).size !== facts.leafNames.length) problems.push(`${directoryKindId} leaves must be unique.`);
  for (const leaf of facts.leafNames) {
    if (leaf !== leafName) problems.push(`${directoryKindId} leaf ${JSON.stringify(leaf)} is not the unpublished kind-only candidate.`);
    else if (scopedFileKindIdForSourcePath(`${pathPrefix}/${facts.directoryName}/${leaf}`, taxonomy, { parentDirectoryKindId: kindId ?? undefined }) !== scopedKindId) problems.push(`${directoryKindId} leaf ${JSON.stringify(leaf)} has no scoped unpublished authority.`);
  }
  return problems;
}

/** 📝️ Validates one unpublished rendered-edit writer state. */
export function taxonomyCliEditWritePreparationProblems(facts: Readonly<{ parentKindId: string; directoryName: string; leafNames: readonly string[] }>, taxonomy: Taxonomy = loadTaxonomy()): string[] {
  return taxonomyCliWritePreparationProblems(facts, "transaction-edit-write-preparation", "🚧️.edit", "transaction-edit-write-candidate", "🚧️stage/🚧️edit-owner", taxonomy);
}

/** 💾️ Validates one unpublished backup writer state. */
export function taxonomyCliBackupWritePreparationProblems(facts: Readonly<{ parentKindId: string; directoryName: string; leafNames: readonly string[] }>, taxonomy: Taxonomy = loadTaxonomy()): string[] {
  return taxonomyCliWritePreparationProblems(facts, "transaction-backup-write-preparation", "🚧️.backup", "transaction-backup-write-candidate", "💾️backup/🚧️backup-owner", taxonomy);
}

/** 🚧️ Validates one transaction edit-preparation directory's exact Windows-safe exchange state. */
export function taxonomyCliEditPreparationProblems(facts: Readonly<{ parentKindId: string; directoryName: string; leafNames: readonly string[]; writePreparations: readonly Readonly<TaxonomyCliWritePreparationFacts>[] }>, taxonomy: Taxonomy = loadTaxonomy()): string[] {
  const problems: string[] = [];
  const kindId = semanticDirectoryKindId(facts.directoryName, taxonomy, { parentKindId: facts.parentKindId });
  if (kindId !== "transaction-edit-preparation") problems.push("Edit preparation directory must resolve below transaction-stage.");
  const operationHash = facts.directoryName.match(/^🚧️edit-([0-9a-f]{24})-/u)?.[1];
  if (!operationHash) problems.push("Edit preparation directory must carry one lowercase 24-hex operation hash.");
  const allowed = operationHash ? new Map([[`${operationHash}.edit`, "transaction-edit-candidate"], [`${operationHash}.pre`, "transaction-edit-preimage"]]) : new Map<string, string>();
  if (new Set(facts.leafNames).size !== facts.leafNames.length) problems.push("Edit preparation leaves must be unique.");
  for (const leaf of facts.leafNames) {
    const expectedKindId = allowed.get(leaf);
    if (!expectedKindId) problems.push(`Edit preparation leaf ${JSON.stringify(leaf)} is not hash-bound to its directory.`);
    else if (scopedFileKindIdForSourcePath(`🚧️stage/${facts.directoryName}/${leaf}`, taxonomy, { parentDirectoryKindId: kindId ?? undefined }) !== expectedKindId) problems.push(`Edit preparation leaf ${JSON.stringify(leaf)} has no scoped exchange authority.`);
  }
  if (facts.writePreparations.length > 1) problems.push("Edit preparation may contain at most one unpublished writer.");
  for (const writer of facts.writePreparations) problems.push(...taxonomyCliEditWritePreparationProblems({ parentKindId: kindId ?? "", directoryName: writer.directoryName, leafNames: writer.leafNames }, taxonomy));
  return problems;
}

/** 📦️ Validates one backup preparation's authoritative candidate and unpublished writer union. */
export function taxonomyCliBackupPreparationProblems(facts: Readonly<{ parentKindId: string; directoryName: string; leafNames: readonly string[]; writePreparations: readonly Readonly<TaxonomyCliWritePreparationFacts>[] }>, taxonomy: Taxonomy = loadTaxonomy()): string[] {
  const problems: string[] = [];
  const kindId = semanticDirectoryKindId(facts.directoryName, taxonomy, { parentKindId: facts.parentKindId });
  if (kindId !== "transaction-backup-preparation") problems.push("Backup preparation directory must resolve below transaction-backup.");
  const operationHash = facts.directoryName.match(/^🚧️backup-([0-9a-f]{24})-/u)?.[1];
  if (!operationHash) problems.push("Backup preparation directory must carry one lowercase 24-hex operation hash.");
  const allowedLeaf = operationHash ? `${operationHash}.backup` : "";
  if (new Set(facts.leafNames).size !== facts.leafNames.length) problems.push("Backup preparation leaves must be unique.");
  for (const leaf of facts.leafNames) if (leaf !== allowedLeaf) problems.push(`Backup preparation leaf ${JSON.stringify(leaf)} is not hash-bound to its directory.`);
  if (facts.writePreparations.length > 1) problems.push("Backup preparation may contain at most one unpublished writer.");
  for (const writer of facts.writePreparations) problems.push(...taxonomyCliBackupWritePreparationProblems({ parentKindId: kindId ?? "", directoryName: writer.directoryName, leafNames: writer.leafNames }, taxonomy));
  return problems;
}

/** ♻️ Validates one restore preparation's exact empty, preimage, exchange, or postimage state. */
export function taxonomyCliRestorePreparationProblems(facts: Readonly<{ parentKindId: string; directoryName: string; leafNames: readonly string[] }>, taxonomy: Taxonomy = loadTaxonomy()): string[] {
  const problems: string[] = [];
  const kindId = semanticDirectoryKindId(facts.directoryName, taxonomy, { parentKindId: facts.parentKindId });
  if (kindId !== "transaction-restore-preparation") problems.push("Restore preparation directory must resolve below transaction-backup.");
  const operationHash = facts.directoryName.match(/^🚧️restore-([0-9a-f]{24})-/u)?.[1];
  if (!operationHash) problems.push("Restore preparation directory must carry one lowercase 24-hex operation hash.");
  const allowed = operationHash ? new Set([`${operationHash}.backup`, `${operationHash}.post`]) : new Set<string>();
  if (new Set(facts.leafNames).size !== facts.leafNames.length) problems.push("Restore preparation leaves must be unique.");
  for (const leaf of facts.leafNames) {
    if (!allowed.has(leaf)) problems.push(`Restore preparation leaf ${JSON.stringify(leaf)} is not hash-bound to its directory.`);
    const scopedId = scopedFileKindIdForSourcePath(`💾️backup/${facts.directoryName}/${leaf}`, taxonomy, { parentDirectoryKindId: kindId ?? undefined });
    if (scopedId !== "transaction-backup-candidate" && scopedId !== "transaction-postimage-candidate") problems.push(`Restore preparation leaf ${JSON.stringify(leaf)} has no scoped candidate authority.`);
  }
  return problems;
}
