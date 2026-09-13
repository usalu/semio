import { POLICY_SOURCE_OPERATIONS, policySourceDirectory, type PolicySourceOperations } from "../../📖️source-access/🟦️.ts";

/** 🗿️ Lists plugin artifact roots without conflating them with standard/subset dialect owners. */
export function policyListPluginArtifactDirs(repoRoot: string, operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): string[] {
  const directories = (path: string) => {
    const source = policySourceDirectory(repoRoot, path, operations);
    if (source.state === "directory") return source.entries.filter((entry) => entry.isDirectory && !entry.isSymbolicLink);
    if (source.state === "missing") return [];
    throw new Error(`Artifact source directory ${path} is ${source.state}.`);
  };
  const out: string[] = [],
    pluginsRoot = "✏️s/🔌️plugins";
  for (const plugin of directories(pluginsRoot)) {
    const artifactsRel = `${pluginsRoot}/${plugin.name}/🗿️artifacts`;
    for (const artifact of directories(artifactsRel)) out.push(`${artifactsRel}/${artifact.name}`);
  }
  return out.sort();
}
