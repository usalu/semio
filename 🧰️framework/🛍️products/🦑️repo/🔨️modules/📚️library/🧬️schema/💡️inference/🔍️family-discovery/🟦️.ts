import { POLICY_SKIP_DIRS, POLICY_SOURCE_OPERATIONS, policySourceDirectory, type PolicySourceOperations } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
import { policyArtifactRootOfMutationsDir } from "../../../🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts";
import { POLICY_INFERENCES_FACET, type PolicyInferenceDiscovery, type PolicyInferenceSourceIssue } from "../🧱️contract/🟦️.ts";

export type PolicyInferenceSlugListing = Readonly<{ slugs: readonly string[]; issues: readonly PolicyInferenceSourceIssue[] }>;

/** 🔎️ Lists concrete inference slugs and records linked or unreadable family entries. */
export function policyListInferenceDirs(repoRoot: string, inferencesRel: string, operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): PolicyInferenceSlugListing {
  const source = policySourceDirectory(repoRoot, inferencesRel, operations),
    issues: PolicyInferenceSourceIssue[] = [];
  if (source.state === "missing") return { slugs: [], issues };
  if (source.state !== "directory") return { slugs: [], issues: [{ path: inferencesRel, state: source.state }] };
  const slugs = source.entries.flatMap((entry) => {
    const path = `${inferencesRel}/${entry.name}`;
    if (entry.isSymbolicLink) {
      issues.push({ path, state: "symlink" });
      return [];
    }
    return entry.isDirectory && entry.name !== "📚️examples" && !entry.name.startsWith(".") ? [entry.name] : [];
  });
  return { slugs: slugs.sort(), issues };
}

/** 🔍️ Finds authored inference facets without requiring absent families or following linked subtrees. */
export function policyFindAllInferencesDirs(repoRoot: string, operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): Readonly<{ paths: readonly string[]; issues: readonly PolicyInferenceSourceIssue[] }> {
  const paths: string[] = [],
    issues: PolicyInferenceSourceIssue[] = [];
  const walk = (relDir: string): void => {
    const source = policySourceDirectory(repoRoot, relDir, operations);
    if (source.state === "missing") return;
    if (source.state !== "directory") {
      issues.push({ path: relDir, state: source.state });
      return;
    }
    for (const entry of source.entries) {
      const path = `${relDir}/${entry.name}`;
      if (entry.isSymbolicLink) {
        issues.push({ path, state: "symlink" });
        continue;
      }
      if (!entry.isDirectory || entry.name.startsWith(".") || POLICY_SKIP_DIRS.has(entry.name)) continue;
      if (entry.name === POLICY_INFERENCES_FACET) paths.push(path);
      else walk(path);
    }
  };
  walk("✏️s");
  return { paths: paths.sort(), issues };
}

/** 🗿️ Resolves an inference facet to its artifact boundary. */
export function policyArtifactRootOfInferencesDir(inferencesRel: string): string {
  return policyArtifactRootOfMutationsDir(inferencesRel);
}

/** 🧭️ Captures each inference family and its direct slug inventory in one admitted discovery epoch. */
export function policyDiscoverInferenceFamilies(repoRoot: string, operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): PolicyInferenceDiscovery {
  const discovered = policyFindAllInferencesDirs(repoRoot, operations),
    issues = [...discovered.issues],
    families = discovered.paths.map((inferencesRel) => {
      const listing = policyListInferenceDirs(repoRoot, inferencesRel, operations);
      issues.push(...listing.issues);
      return { inferencesRel, artifactRel: policyArtifactRootOfInferencesDir(inferencesRel), slugs: listing.slugs };
    });
  return { families, issues };
}
