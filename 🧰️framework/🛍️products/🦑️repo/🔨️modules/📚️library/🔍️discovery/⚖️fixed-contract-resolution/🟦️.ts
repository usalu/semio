import type { FixedDirectoryContract, FixedFilenameContract } from "../📜️taxonomy-contracts/🟦️.ts";
import type { Taxonomy } from "../📑️taxonomy-document/🟦️.ts";
import { type FixedContractMatchContext, createTaxonomyPathMatcher, fixedContractWinner, fixedScopeMatches, taxonomyPathPatternMatches } from "../🎯️taxonomy-path-matching/🟦️.ts";
import type { SchemaFormatSpec } from "../📄️implementation-findings/🟦️.ts";
import { canonicalFilenamesForKind, loadTaxonomy } from "../🔐️taxonomy-authority/🟦️.ts";

/** 🔒️ Resolves the single deterministic fixed filename winner. */
export function fixedFilenameContractIdsForPath(path: string, taxonomy: Taxonomy = loadTaxonomy(), context: FixedContractMatchContext = {}): string[] {
  const normalized = path.replaceAll("\\", "/").replace(/^\.\//u, "").normalize(taxonomy.unicodeNormalization.form);
  return fixedContractWinner("filename", Object.entries(taxonomy.fixedFilenameContracts)
    .filter(([, contract]) => (context.pathMatcher?.matches(normalized, contract.pathPattern) ?? taxonomyPathPatternMatches(normalized, contract.pathPattern)) && fixedScopeMatches(contract, normalized, context, taxonomy.fixedDirectoryContractSets)));
}

/** 📁️ Resolves the single deterministic fixed directory winner. */
export function fixedDirectoryContractIdsForPath(path: string, taxonomy: Taxonomy = loadTaxonomy(), context: FixedContractMatchContext = {}): string[] {
  const normalized = path.replaceAll("\\", "/").replace(/^\.\//u, "").replace(/\/$/u, "").normalize(taxonomy.unicodeNormalization.form);
  return fixedContractWinner("directory", Object.entries(taxonomy.fixedDirectoryContracts)
    .filter(([, contract]) => (context.pathMatcher?.matches(normalized, contract.pathPattern) ?? taxonomyPathPatternMatches(normalized, contract.pathPattern)) && fixedScopeMatches(contract, normalized, context, taxonomy.fixedDirectoryContractSets)));
}

/** 🧵️Invocation-owned fixed-contract resolver that indexes literal basenames and compiles each glob once. */
export function createFixedContractResolver(taxonomy: Taxonomy = loadTaxonomy()): Readonly<{
  filenameIdsForPath: (path: string, context?: FixedContractMatchContext) => string[];
  directoryIdsForPath: (path: string, context?: FixedContractMatchContext) => string[];
}> {
  const matcher = createTaxonomyPathMatcher();
  const index = <T extends FixedFilenameContract | FixedDirectoryContract>(contracts: Readonly<Record<string, T>>) => {
    const literal = new Map<string, (readonly [string, T])[]>(), wildcard: (readonly [string, T])[] = [];
    for (const entry of Object.entries(contracts)) {
      const name = entry[1].pathPattern.slice(entry[1].pathPattern.lastIndexOf("/") + 1);
      if (/[?*\[]/u.test(name)) wildcard.push(entry);
      else literal.set(name, [...(literal.get(name) ?? []), entry]);
    }
    return (path: string, context: FixedContractMatchContext, kind: "filename" | "directory"): string[] => {
      const normalized = path.replaceAll("\\", "/").replace(/^\.\//u, "").replace(kind === "directory" ? /\/$/u : /$^/u, "").normalize(taxonomy.unicodeNormalization.form);
      const name = normalized.slice(normalized.lastIndexOf("/") + 1);
      const candidates = [...(literal.get(name) ?? []), ...wildcard];
      return fixedContractWinner(kind, candidates.filter(([, contract]) => matcher.matches(normalized, contract.pathPattern) && fixedScopeMatches(contract, normalized, context, taxonomy.fixedDirectoryContractSets)));
    };
  };
  const filenames = index(taxonomy.fixedFilenameContracts), directories = index(taxonomy.fixedDirectoryContracts);
  return Object.freeze({
    filenameIdsForPath: (path: string, context: FixedContractMatchContext = {}) => filenames(path, context, "filename"),
    directoryIdsForPath: (path: string, context: FixedContractMatchContext = {}) => directories(path, context, "directory"),
  });
}

/** 🚫️ Resolves an exact fixed-looking path to its schema-owned normalize/relocate decision. */
export function fixedFilenameRejectionContractIdForPath(path: string, taxonomy: Taxonomy = loadTaxonomy()): string | null {
  const normalized = path.replaceAll("\\", "/").replace(/^\.\//u, "").normalize(taxonomy.unicodeNormalization.form);
  const matches = Object.entries(taxonomy.fixedFilenameRejectionContracts).filter(([, contract]) => contract.sourcePathIdentities.includes(normalized));
  if (matches.length > 1) throw new Error(`Path resolves to multiple fixed filename rejection contracts: ${matches.map(([id]) => id).join(", ")}.`);
  return matches[0]?.[0] ?? null;
}

export function componentFilenames(taxonomy: Taxonomy): string[] {
  return [...new Set(Object.values(taxonomy.componentFileKinds).flatMap((kindId) => canonicalFilenamesForKind(kindId, taxonomy)))];
}

/**
 * 🧬️ Resolves a schema facet's DECLARED kind: a kind claims a facet module by listing it in
 * `schemaFacetKinds[<kind>].facetPathIdentities`, everything else is `schemaDefaultFacetKind`. Nothing is
 * probed from disk, so a half-authored module never silently changes the formats it owes.
 */
export function resolveSchemaFacetKind(facetRel: string, taxonomy: Taxonomy = loadTaxonomy()): string {
  const normalized = facetRel.replaceAll("\\", "/").replace(/^\.\//u, "").replace(/\/+$/u, "").normalize("NFC");
  for (const [kindId, kind] of Object.entries(taxonomy.schemaFacetKinds ?? {})) if (kind.facetPathIdentities.includes(normalized)) return kindId;
  return taxonomy.schemaDefaultFacetKind;
}

/** 📜️ Returns the schemaFormats entries required for one facet path (declared-kind subset). */
export function schemaFacetFormatEntries(facetRel: string, taxonomy: Taxonomy = loadTaxonomy()): [string, SchemaFormatSpec][] {
  const kind = taxonomy.schemaFacetKinds?.[resolveSchemaFacetKind(facetRel, taxonomy)];
  if (!kind) return Object.entries(taxonomy.schemaFormats ?? {}) as [string, SchemaFormatSpec][];
  return kind.formats
    .map((formatId) => [formatId, taxonomy.schemaFormats[formatId]] as [string, SchemaFormatSpec | undefined])
    .filter((entry): entry is [string, SchemaFormatSpec] => entry[1] !== undefined);
}
