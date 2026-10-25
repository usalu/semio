import { pathEmojiStatuteFindings } from "../../../../../../📦️packages/🟦️typescript/🪪️identity/🛣️path/🟦️.ts";
import { leadingEmojiIdentity } from "../../../../../../📦️packages/🟦️typescript/🪪️identity/🧩️grapheme/🟦️.ts";
import type { Taxonomy } from "../📑️taxonomy-document/🟦️.ts";

/** 🪜️ Level descriptor: fixed allowlist or wildcard (`*` = any emoji-prefixed slug dir). */
type ArtifactFacetLevel =
  | { readonly kind: "fixed"; readonly dirs: readonly string[] }
  | { readonly kind: "wildcard" }
  | { readonly kind: "none" };

/** 🏘️ Validates explicitly authored two-tier mutation owners without assigning names or traversing files. */
export function mutationDomainOwnersProblems(owners: Taxonomy["mutationDomainOwners"], genericEmojiIdentities: readonly string[] = []): string[] {
  const problems: string[] = [];
  if (!owners || typeof owners !== "object" || Array.isArray(owners)) return ["mutationDomainOwners must be an exact-owner object."];
  for (const [root, domains] of Object.entries(owners)) {
    if (!root.endsWith("/🧬️mutations") || root !== root.normalize("NFC") || /[*?{}]/u.test(root) || root.includes(String.fromCharCode(92)) || [...root].some((character) => character.charCodeAt(0) < 32) || root.split("/").some((part) => !part || part === "." || part === "..")) problems.push(`mutationDomainOwners[${JSON.stringify(root)}] must name one exact repository mutation root.`);
    if (!domains || typeof domains !== "object" || Array.isArray(domains) || Object.keys(domains).length === 0) {
      problems.push(`mutationDomainOwners[${JSON.stringify(root)}] must declare non-empty domains.`);
      continue;
    }
    const identities = new Set<string>();
    const entries: { path: string; nodeKind: "directory" }[] = [];
    for (const [domain, operations] of Object.entries(domains)) {
      const domainId = leadingEmojiIdentity(domain).rest;
      entries.push({ path: `${root}/${domain}`, nodeKind: "directory" });
      if (domain.includes("/") || domain.includes(String.fromCharCode(92)) || !/^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$/u.test(domainId)) problems.push(`${root}/${domain} is not one domain basename.`);
      if (!operations || typeof operations !== "object" || Array.isArray(operations) || Object.keys(operations).length === 0) {
        problems.push(`${root}/${domain} must declare non-empty operations.`);
        continue;
      }
      for (const [operation, identity] of Object.entries(operations)) {
        const operationId = leadingEmojiIdentity(operation).rest;
        entries.push({ path: `${root}/${domain}/${operation}`, nodeKind: "directory" });
        if (operation.includes("/") || operation.includes(String.fromCharCode(92)) || !/^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$/u.test(operationId) || typeof identity !== "string" || !/^[a-z][a-z0-9]*(?:-[a-z0-9]+)+$/u.test(identity)) problems.push(`${root}/${domain}/${operation} must declare one operation basename and its explicit full semantic identity.`);
        if (identities.has(identity)) problems.push(`${root} has duplicate semantic identity ${JSON.stringify(identity)}.`);
        identities.add(identity);
      }
    }
    for (const finding of pathEmojiStatuteFindings(entries, genericEmojiIdentities)) problems.push(`${finding.path} violates mutation-domain ${finding.kind} emoji identity.`);
  }
  return problems;
}

/** 🛂️ Validates direct source ownership without cross-artifact, cross-standard, chained, or inferred ownership. */
export function mutationCatalogSourceOwnersProblems(taxonomy: Pick<Taxonomy, "mutationCatalogSourceOwners" | "mutationDomainOwners">): string[] {
  const owners = taxonomy.mutationCatalogSourceOwners, problems: string[] = [];
  if (!owners || typeof owners !== "object" || Array.isArray(owners)) return ["mutationCatalogSourceOwners must be an exact-owner object."];
  const profile = (path: unknown): RegExpMatchArray | null => typeof path === "string" && path === path.normalize("NFC") && !/[\\\\:*?{}\u0000-\u001F]/u.test(path) && path.split("/").every((part) => part && part !== "." && part !== "..") ? path.match(/^(.+)\/🏅️standards\/(🔖️[^/]+)\/🪆️subsets\/([^/]+)$/u) : null;
  for (const [owner, source] of Object.entries(owners)) {
    const catalogProfile = profile(owner), sourceProfile = profile(source);
    if (!catalogProfile || !sourceProfile || catalogProfile[1] !== sourceProfile[1] || catalogProfile[2] !== sourceProfile[2] || owner === source || Object.hasOwn(owners, source) || !Object.hasOwn(taxonomy.mutationDomainOwners, `${source}/🧬️schema/🧬️mutations`)) problems.push(`mutationCatalogSourceOwners[${JSON.stringify(owner)}] must reference one distinct registered source subset in the same artifact and standard without ownership chains.`);
  }
  return problems;
}

/** 🏗️ Free top-level `fn <name>(` declarations (Rust's structural shape for an extracted diff/inverse body) keyed by the behavior facet they belong to. A trait-impl method (`async fn diff(&self, ...) { diff(self, base) }`, indented inside `impl … {}`) never matches — only a column-zero free function does, which is exactly the shape the collapsed leaves reduce to once split-out logic gets folded back in. */
export const MUTATION_DIRECT_LEAF_STRUCTURAL_PATTERNS: Readonly<Record<string, RegExp>> = {
  "🔺️diff": /^pub (?:async )?fn diff\(/mu,
  "↩️inverse": /^pub (?:async )?fn inverse\(/mu,
};

/** 🏠️ Declared child level of a path under an artifact root (parents are `/`-segments already accepted). */
export function artifactFacetChildLevel(parents: readonly string[], taxonomy: Taxonomy): ArtifactFacetLevel {
  if (parents.length === 0) return { kind: "fixed", dirs: taxonomy.artifactComponentDirs };
  const root = parents[0]!;
  const a = parents[1];
  const b = parents[2];
  const c = parents[3];
  if (parents.length === 1) {
    if (root === "🧬️schema") return { kind: "fixed", dirs: taxonomy.schemaChildDirs ?? [] };
    if (root === "🚪️io") return { kind: "fixed", dirs: [...(taxonomy.ioDirectionDirs ?? []), ...(taxonomy.representationDirs ?? [])] };
    return { kind: "none" };
  }
  if (root === "🧬️schema") {
    if (parents.length === 2 && (taxonomy.schemaChildDirs ?? []).includes(a!)) {
      if (a === "🧬️mutations") return { kind: "fixed", dirs: ["*"] };
      if (a === "💡️inferences") return { kind: "fixed", dirs: ["*"] };
      return { kind: "none" };
    }
    if (parents.length === 3 && a === "🧬️mutations") {
      if ((taxonomy.representationDirs ?? []).includes(b!)) return { kind: "none" };
      return { kind: "fixed", dirs: [...(taxonomy.mutationBehaviorFacetDirs ?? []), ...(taxonomy.mutationOrganizationalFacetDirs ?? [])] };
    }
    if (parents.length === 3 && a === "💡️inferences") return { kind: "none" };
    if (parents.length === 3 && (taxonomy.representationDirs ?? []).includes(b!)) return { kind: "none" };
    if (parents.length === 4 && a === "🧬️mutations") return { kind: "none" };
    return { kind: "none" };
  }
  if (root === "🚪️io") {
    const directions = taxonomy.ioDirectionDirs ?? [];
    const childMap = taxonomy.ioDirectionChildDirs ?? {};
    if (parents.length === 2 && (taxonomy.representationDirs ?? []).includes(a!)) return { kind: "fixed", dirs: taxonomy.ioSemanticCollectionDirNames ?? [] };
    if (parents.length === 3 && (taxonomy.representationDirs ?? []).includes(a!) && (taxonomy.ioSemanticCollectionDirNames ?? []).includes(b!)) return { kind: b === "🧬️mutations" || b === "💡️inferences" ? "wildcard" : "none" };
    if (parents.length === 4 && (taxonomy.representationDirs ?? []).includes(a!) && (b === "🧬️mutations" || b === "💡️inferences")) return { kind: "none" };
    if (parents.length === 2 && directions.includes(a!)) {
      const child = childMap[a!];
      return child ? { kind: "fixed", dirs: [child] } : { kind: "none" };
    }
    if (parents.length === 3 && directions.includes(a!) && childMap[a!] === b) {
      return { kind: "fixed", dirs: [taxonomy.artifactsDirName] };
    }
    if (parents.length === 4 && b === childMap[a!] && c === taxonomy.artifactsDirName) return { kind: "wildcard" };
    if (parents.length === 5) return { kind: "none" };
    return { kind: "none" };
  }
  return { kind: "none" };
}

/** 🌳️ Declared children of a nesting artifact facet path (`/`-joined parents), empty when leaves-only. */
function artifactFacetChildDirs(facetPath: string, taxonomy: Taxonomy): readonly string[] {
  const parents = facetPath ? facetPath.split("/") : [];
  const level = artifactFacetChildLevel(parents, taxonomy);
  if (level.kind !== "fixed") return [];
  return level.dirs.filter((d) => d !== "*");
}

/** 🚧️ The only opaque subtrees the taxonomy admits, in their declared order: two user-owned
 * scratch trees, and the one tracked nested-repository gitlink (`git ls-files -s` mode `160000`),
 * which must be filtered lexically before source admission or `inventoryTaxonomyWithSourceParentPruning`
 * refuses to classify anything at all. Every entry here is a whole opaque subtree, so nothing the
 * repository generates may live under one — which is why the gitlink itself is named, not its parent
 * `♻️mit-bestand/`, whose `📋️bericht`/`🖼️asset` subtrees are generator-contract outputs. */
export const OPAQUE_PATH_EXCLUSIONS: readonly (readonly [string, string])[] = [
  ["compose", "compose/"],
  ["temp-compose", "temp/compose/"],
  ["mit-bestand-recherche", "♻️mit-bestand/🔎️recherche/"],
];
