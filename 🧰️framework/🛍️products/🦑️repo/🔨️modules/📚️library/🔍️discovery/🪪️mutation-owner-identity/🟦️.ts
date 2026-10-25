import type { Taxonomy } from "../📑️taxonomy-document/🟦️.ts";
import { MUTATION_DIRECT_LEAF_STRUCTURAL_PATTERNS, artifactFacetChildLevel, mutationCatalogSourceOwnersProblems } from "../👥️mutation-owner-rules/🟦️.ts";
import { loadTaxonomy, semanticDirectoryKindId } from "../🔐️taxonomy-authority/🟦️.ts";

/** 📦️ Allowed semantic directory kinds inside one language package. */
export function packagingDirectoryKindIdsForLang(lang: string, taxonomy: Taxonomy = loadTaxonomy()): readonly string[] {
  const global = taxonomy.packagingDirectoryKindIds ?? [];
  const ecosystem = taxonomy.ecosystems[lang]?.packagingDirectoryKindIds ?? [];
  return [...new Set([...global, ...ecosystem])];
}

/**
 * 🂡 Whether a dir name is an emoji-prefixed slug — an Extended_Pictographic codepoint at the
 * start with the canonical selector and resolve to exactly one semantic-directory registry entry.
 */
function isEmojiPrefixedSlugDir(name: string, taxonomy: Taxonomy): boolean {
  return semanticDirectoryKindId(name, taxonomy) !== null;
}

/** 🧭️ Resolves only an exact declared catalog source; unmapped owners retain their own current source tree. */
export function mutationCatalogSourceOwner(owner: string, taxonomy: Pick<Taxonomy, "mutationCatalogSourceOwners" | "mutationDomainOwners"> = loadTaxonomy()): string | null {
  if (!Object.hasOwn(taxonomy.mutationCatalogSourceOwners, owner)) return owner;
  return mutationCatalogSourceOwnersProblems(taxonomy).length === 0 ? taxonomy.mutationCatalogSourceOwners[owner]! : null;
}

/** 🪪️ Resolves a physical mutation owner only through its declared exact layout. */
export function mutationOwnerIdentity(root: string, ownerPath: string, taxonomy: Taxonomy = loadTaxonomy()): string | null {
  if (ownerPath !== ownerPath.normalize("NFC") || ownerPath.includes(String.fromCharCode(92))) return null;
  const domains = taxonomy.mutationDomainOwners[root];
  if (domains) {
    const [domain, operation, extra] = ownerPath.split("/");
    return extra === undefined && domain && operation && Object.hasOwn(domains, domain) && Object.hasOwn(domains[domain]!, operation) ? domains[domain]![operation]! : null;
  }
  return !ownerPath.includes("/") && mutationDirectoryNameIsValid(ownerPath, taxonomy) ? ownerPath.match(/[a-z][a-z0-9]*(?:-[a-z0-9]+)+$/u)?.[0] ?? null : null;
}

/** 🗺️ Resolves one semantic ID to its explicitly registered domain and operation path. */
export function mutationOwnerRelativePath(root: string, identity: string, taxonomy: Pick<Taxonomy, "mutationDomainOwners"> = loadTaxonomy()): string | null {
  const paths = Object.entries(taxonomy.mutationDomainOwners[root] ?? {}).flatMap(([domain, operations]) => Object.entries(operations).filter(([, value]) => value === identity).map(([operation]) => `${domain}/${operation}`));
  return paths.length === 1 ? paths[0]! : null;
}

/** 🧬️ Whether one direct mutation owner has the configured emoji + semantic verb-noun identity. */
export function mutationDirectoryNameIsValid(name: string, taxonomy: Taxonomy = loadTaxonomy()): boolean {
  return name === name.normalize("NFC") && new RegExp(taxonomy.mutationDirectoryPattern, "u").test(name);
}

/**
 * 🔖️🏗️ Behavior facets (`🔺️diff`/`↩️inverse`) that a mutation's direct leaf re-inlines instead of placing
 * in their own facet directory — the exact regression the SEMANTIC-MUTATIONS-OVERHAUL contract forbids.
 *
 * STRUCTURAL detection is the actual contract: a free `pub (async)? fn diff(`/`fn inverse(` in the direct
 * leaf with no sibling facet directory present is a violation regardless of comments — an author who
 * simply omits a `//#region` marker cannot evade this (confirmed at 266-mutation scale in `🏛️architect`,
 * where every leaf was marker-free). `siblingFacetDirs` must be the actual child directory names observed
 * beside the direct leaf on disk; pass an empty set only when no sibling facets exist (e.g. an inline fixture).
 *
 * The `mutationDirectLeafForbiddenRegionMarkers` marker scan still runs, unioned in: it is the only signal
 * for a non-Rust direct leaf (e.g. a `🟦️.ts` mirror, where "free `pub fn diff(`" has no meaning) and for any
 * Rust shape that carries the marker without matching the bare free-function form. It adds coverage the
 * structural check cannot reach; it is not a substitute for it.
 *
 * Empty means the direct leaf is clean.
 */
export function mutationDirectLeafInlinedBehaviorFacets(directLeafSource: string, siblingFacetDirs: ReadonlySet<string> = new Set(), taxonomy: Taxonomy = loadTaxonomy()): readonly string[] {
  const found = new Set<string>();
  const markers = taxonomy.mutationDirectLeafForbiddenRegionMarkers ?? {};
  for (const [facet, marker] of Object.entries(markers)) {
    if (new RegExp(`//#region\\s+${marker.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&")}\\b`, "u").test(directLeafSource)) found.add(facet);
  }
  for (const [facet, pattern] of Object.entries(MUTATION_DIRECT_LEAF_STRUCTURAL_PATTERNS)) {
    if (pattern.test(directLeafSource) && !siblingFacetDirs.has(facet)) found.add(facet);
  }
  return [...found].sort();
}

/** 💠️ Whether a `/`-joined facet path walks only declared dirs from an artifact root (supports `*` wildcard levels). */
export function artifactFacetPathIsDeclared(facetPath: string, taxonomy: Taxonomy = loadTaxonomy()): boolean {
  const [root, ...rest] = facetPath.split("/");
  if (!root || !taxonomy.artifactComponentDirs.includes(root)) return false;
  const parents: string[] = [root];
  for (const segment of rest) {
    const level = artifactFacetChildLevel(parents, taxonomy);
    if (level.kind === "none") return false;
    if ((parents[0] === "🧬️schema" || parents[0] === "🚪️io" && parents.length >= 3) && (taxonomy.representationDirs ?? []).includes(segment) && !(level.kind === "fixed" && level.dirs.includes(segment))) return false;
    const directMutationOwner = parents.length === 2 && parents[0] === "🧬️schema" && parents[1] === "🧬️mutations" || parents.length === 3 && parents[0] === "🚪️io" && parents[2] === "🧬️mutations";
    const directInferenceOwner = parents.length === 2 && parents[0] === "🧬️schema" && parents[1] === "💡️inferences" || parents.length === 3 && parents[0] === "🚪️io" && parents[2] === "💡️inferences";
    const wildcardAccepted = directMutationOwner ? mutationDirectoryNameIsValid(segment, taxonomy) : directInferenceOwner ? semanticDirectoryKindId(segment, taxonomy, { parentKindId: "inferences" }) !== null : parents.at(-1) === taxonomy.artifactsDirName ? semanticDirectoryKindId(segment, taxonomy, { parentKindId: "artifacts" }) !== null : isEmojiPrefixedSlugDir(segment, taxonomy);
    if (level.kind === "wildcard") {
      if (!wildcardAccepted) return false;
    } else {
      const dirs = level.dirs;
      const allowWildcard = dirs.includes("*");
      const fixed = dirs.filter((d) => d !== "*");
      if (!(fixed.includes(segment) || (allowWildcard && wildcardAccepted))) return false;
    }
    parents.push(segment);
  }
  return true;
}
