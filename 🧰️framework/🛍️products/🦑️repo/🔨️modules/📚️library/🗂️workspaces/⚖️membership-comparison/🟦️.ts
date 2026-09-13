import { diffWorkspaces, type WorkspaceDiscoveryOptions } from "../🟦️.ts";

export interface WorkspaceMembershipComparison {
  readonly expected: readonly string[];
  readonly fresh: boolean;
  readonly missing: readonly string[];
  readonly orderChanged: boolean;
  readonly stale: readonly string[];
}

/** ⚖️ Compares authored membership with the existing physical workspace authority. */
export function compareWorkspaceMembership(repoRoot: string, current: readonly string[], options: WorkspaceDiscoveryOptions = {}): WorkspaceMembershipComparison {
  const { expected, missing, stale } = diffWorkspaces(repoRoot, current, options);
  const fresh = current.length === expected.length && current.every((entry, index) => entry === expected[index]);
  return { expected, fresh, missing, orderChanged: !fresh && missing.length === 0 && stale.length === 0, stale };
}
