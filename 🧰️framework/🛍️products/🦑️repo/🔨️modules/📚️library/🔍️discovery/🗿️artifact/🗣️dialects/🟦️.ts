import { loadTaxonomy, subsetIdForDirectoryName } from "../../../🟦️.ts";
import { POLICY_SOURCE_OPERATIONS, policySourceDirectory, type PolicySourceOperations } from "../../📖️source-access/🟦️.ts";
import { policyListPluginArtifactDirs } from "../🏠️roots/🟦️.ts";

export const POLICY_STANDARDS_DIR = "🏅️standards";
export const POLICY_SUBSETS_DIR = "🪆️subsets";

export type PolicyArtifactDialect = {
  artRel: string;
  standardRel: string;
  standardSlug: string;
  subsetsRel: string;
  subsetRel: string;
  subsetDirName: string;
  subsetId: string;
};

/** 🗣️ Lists standard/subset dialect rows separately from their enclosing plugin artifact roots. */
export function policyListArtifactDialectDirs(repoRoot: string, operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): PolicyArtifactDialect[] {
  const taxonomy = loadTaxonomy(),
    standardsDirName = taxonomy.standardsDirName ?? POLICY_STANDARDS_DIR,
    subsetsDirName = taxonomy.subsetsDirName ?? POLICY_SUBSETS_DIR,
    standardPrefix = taxonomy.standardDirPrefix ?? "🔖️",
    directories = (path: string) => {
      const source = policySourceDirectory(repoRoot, path, operations);
      if (source.state === "directory") return source.entries.filter((entry) => entry.isDirectory && !entry.isSymbolicLink);
      if (source.state === "missing") return [];
      throw new Error(`Artifact dialect source directory ${path} is ${source.state}.`);
    },
    out: PolicyArtifactDialect[] = [];
  for (const artRel of policyListPluginArtifactDirs(repoRoot, operations)) {
    const standardsRel = `${artRel}/${standardsDirName}`;
    for (const standard of directories(standardsRel)) {
      if (!standard.name.startsWith(standardPrefix)) continue;
      const standardRel = `${standardsRel}/${standard.name}`,
        standardSlug = standard.name.slice(standardPrefix.length),
        subsetsRel = `${standardRel}/${subsetsDirName}`;
      for (const subset of directories(subsetsRel)) {
        out.push({ artRel, standardRel, standardSlug, subsetsRel, subsetRel: `${subsetsRel}/${subset.name}`, subsetDirName: subset.name, subsetId: subsetIdForDirectoryName(subsetsRel, subset.name, taxonomy) ?? subset.name });
      }
    }
  }
  return out;
}
