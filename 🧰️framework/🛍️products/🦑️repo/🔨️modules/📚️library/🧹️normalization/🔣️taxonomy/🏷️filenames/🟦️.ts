/** 🧭️ Taxonomy filename and option helpers shared by the generate, test and policy lanes.
 * @see 📜️script.ts
 */
import { canonicalPrimaryFilenameForKind, fixedContractFilename, loadTaxonomy } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { readFileSync } from "node:fs";
import { join } from "node:path";

export function taxonomyOption(args: readonly string[], name: string): string | undefined {
  const index = args.indexOf(name);
  return index >= 0 ? args[index + 1] : undefined;
}

function taxonomyContractFilename(taxonomy: ReturnType<typeof loadTaxonomy>, contractId: string): string {
  const fixed = taxonomy.fixedFilenameContracts[contractId];
  const filename = fixed ? fixedContractFilename(fixed) : taxonomy.configurableEntryContracts[contractId]?.filename;
  if (!filename) throw new Error(`[taxonomy] unknown filename contract ${JSON.stringify(contractId)}.`);
  return filename;
}

export function taxonomyContractFilenames(taxonomy: ReturnType<typeof loadTaxonomy>, contractIds: readonly string[]): string[] {
  return contractIds.map((contractId) => taxonomyContractFilename(taxonomy, contractId));
}

export function taxonomyMappedFilename(taxonomy: ReturnType<typeof loadTaxonomy>, mapping: Readonly<Record<string, string>>, key: string): string {
  const fileKindId = mapping[key];
  if (!fileKindId) throw new Error(`[taxonomy] no file-kind mapping for ${JSON.stringify(key)}.`);
  return canonicalPrimaryFilenameForKind(fileKindId, taxonomy);
}

export function taxonomyEcosystemEntryFilenames(taxonomy: ReturnType<typeof loadTaxonomy>, ecosystemId: string): string[] {
  return taxonomyContractFilenames(taxonomy, taxonomy.ecosystems[ecosystemId]?.entryContractIds ?? []);
}

/**
 * 🔣️The SSOT taxonomy, read once. The root script must not know WHERE a module lives or WHICH
 * subcommands it offers — both are declared vocabulary, so adding a phase or relocating the testing
 * domain is a taxonomy edit and never an edit here.
 */
export function repoTaxonomy(root: string): Record<string, unknown> {
  return JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8")) as Record<string, unknown>;
}
