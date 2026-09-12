import { loadTaxonomy } from "../../../🟦️.ts";
import { POLICY_SKIP_DIRS, POLICY_SOURCE_OPERATIONS, policySourceDirectory, type PolicySourceOperations } from "../../../🔍️discovery/📖️source-access/🟦️.ts";

/** 🗿️ Finds standard/subset document owners without entering linked or generated artifact internals. */
export function policyDiscoverArtifactSchemaOwners(repoRoot: string, operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): string[] {
  const taxonomy = loadTaxonomy(),
    owners: string[] = [],
    pending = ["✏️s/🔌️plugins", "🧰️framework"];
  const directories = (path: string) => {
    const source = policySourceDirectory(repoRoot, path, operations);
    return source.state === "directory" ? source.entries.filter((entry) => entry.isDirectory && !entry.isSymbolicLink && !entry.name.startsWith(".") && !POLICY_SKIP_DIRS.has(entry.name) && entry.name !== "🗑️generated") : [];
  };
  while (pending.length) {
    const parent = pending.pop()!;
    for (const entry of directories(parent)) {
      const path = `${parent}/${entry.name}`;
      if (entry.name !== taxonomy.artifactsDirName) {
        pending.push(path);
        continue;
      }
      for (const artifact of directories(path)) {
        const standards = `${path}/${artifact.name}/${taxonomy.standardsDirName}`;
        for (const standard of directories(standards)) {
          const subsets = `${standards}/${standard.name}/${taxonomy.subsetsDirName}`;
          for (const subset of directories(subsets)) owners.push(`${subsets}/${subset.name}`);
        }
      }
    }
  }
  return owners.sort();
}
