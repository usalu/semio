import { loadTaxonomy } from "../../🟦️.ts";
import { POLICY_SOURCE_OPERATIONS, policySourceDirectory, type PolicySourceOperations } from "../📖️source-access/🟦️.ts";

/** 🗺️ Discovers existing viewer and editor roots through the standard/subset surface axis without following links. */
export function policySurfaceRoots(repoRoot: string, ownerRoot: string, taxonomy: ReturnType<typeof loadTaxonomy>, operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): string[] {
  const roots: string[] = [];
  const directories = (path: string) => {
    const source = policySourceDirectory(repoRoot, path, operations);
    if (source.state === "directory") return source.entries.filter((entry) => entry.isDirectory && !entry.isSymbolicLink);
    if (source.state === "missing") return [];
    throw new Error(`Surface source directory ${path} is ${source.state}.`);
  };
  const artifactsRoot = `${ownerRoot}/${taxonomy.artifactsDirName}`;
  for (const artifact of directories(artifactsRoot)) {
    const standardsRoot = `${artifactsRoot}/${artifact.name}/${taxonomy.standardsDirName}`;
    for (const standard of directories(standardsRoot)) {
      const subsetsRoot = `${standardsRoot}/${standard.name}/${taxonomy.subsetsDirName}`;
      for (const subset of directories(subsetsRoot)) {
        for (const role of taxonomy.surfaceRoles) {
          const surfaceRoot = `${subsetsRoot}/${subset.name}/${taxonomy.surfaceDirNames[role]}`,
            source = policySourceDirectory(repoRoot, surfaceRoot, operations);
          if (source.state === "directory") roots.push(surfaceRoot);
          else if (source.state !== "missing") throw new Error(`Surface source directory ${surfaceRoot} is ${source.state}.`);
        }
      }
    }
  }
  return roots;
}
