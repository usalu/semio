import { POLICY_SKIP_DIRS, POLICY_SOURCE_OPERATIONS, policySourceDirectory, type PolicySourceOperations } from "../📖️source-access/🟦️.ts";

export type PolicySourceWalkIssue = Readonly<{ path: string; state: "unreadable" | "symlink" | "not-directory" }>;

export type PolicySourceWalk = Readonly<{ files: readonly string[]; issues: readonly PolicySourceWalkIssue[] }>;

/** 🚶️ Walks admitted directories for matching files without following links or collapsing unavailable evidence. */
export function policyWalkRelFileSources(
  repoRoot: string,
  relRoots: readonly string[],
  pred: (relPath: string, name: string) => boolean,
  skipDir?: (relDir: string, name: string) => boolean,
  operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS,
): PolicySourceWalk {
  const files = new Set<string>(),
    issues = new Map<string, PolicySourceWalkIssue>();
  const walk = (relDir: string): void => {
    const source = policySourceDirectory(repoRoot, relDir, operations);
    if (source.state === "missing") return;
    if (source.state !== "directory") {
      issues.set(relDir, { path: relDir, state: source.state });
      return;
    }
    for (const entry of source.entries) {
      const childRel = relDir ? `${relDir}/${entry.name}` : entry.name;
      if (entry.isSymbolicLink) {
        issues.set(childRel, { path: childRel, state: "symlink" });
        continue;
      }
      if (entry.isDirectory) {
        if (POLICY_SKIP_DIRS.has(entry.name) || entry.name.startsWith(".") || skipDir?.(childRel, entry.name)) continue;
        walk(childRel);
      } else if (entry.isFile && pred(childRel, entry.name)) files.add(childRel);
    }
  };
  for (const root of relRoots) walk(root);
  return { files: [...files].sort(), issues: [...issues.values()].sort((left, right) => left.path.localeCompare(right.path)) };
}

/** 🛑 Retains the established file-list API while failing closed for non-missing source evidence. */
export function policyWalkRelFiles(
  repoRoot: string,
  relRoots: readonly string[],
  pred: (relPath: string, name: string) => boolean,
  skipDir?: (relDir: string, name: string) => boolean,
  operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS,
): string[] {
  const result = policyWalkRelFileSources(repoRoot, relRoots, pred, skipDir, operations),
    issue = result.issues[0];
  if (issue) throw new Error(`Policy source path ${issue.path} is ${issue.state}.`);
  return [...result.files];
}
