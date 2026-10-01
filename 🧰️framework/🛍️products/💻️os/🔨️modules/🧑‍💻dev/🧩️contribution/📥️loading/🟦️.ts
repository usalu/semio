import { readFileSync, realpathSync } from "node:fs";
import { isAbsolute, relative, resolve } from "node:path";
import { ownedDevPath, parseDevContribution, type DevContribution } from "../🧬️schema/🟦️.ts";

/** 🔒️ Resolves real files within the workspace and their declared contribution owner. */
export function resolveDevContributionFile(workspace: string, path: string, ownerRoot?: string): string {
  const root = realpathSync(workspace), file = realpathSync(resolve(root, ownedDevPath(path)));
  const boundary = ownerRoot === undefined ? root : realpathSync(resolve(root, ownedDevPath(ownerRoot)));
  for (const base of [root, boundary]) {
    const tail = relative(base, file);
    if (isAbsolute(tail) || tail === ".." || tail.startsWith("..") || tail.startsWith("..\\")) throw new Error("Development contribution file escapes its owner");
  }
  return file;
}

/** 📇️ Loads a selected owner's schema data; absence means an unbranded framework entry. */
export function loadDevContribution(workspace: string, row: { readonly devContribution?: string; readonly brand?: string }): DevContribution | undefined {
  if (!row.devContribution) {
    if (row.brand) throw new Error("A branded playground requires an owned development contribution");
    return undefined;
  }
  const contribution = parseDevContribution(JSON.parse(readFileSync(resolveDevContributionFile(workspace, row.devContribution), "utf8")));
  resolveDevContributionFile(workspace, row.devContribution, contribution.ownerRoot);
  resolveDevContributionFile(workspace, contribution.viteConfig, contribution.ownerRoot);
  resolveDevContributionFile(workspace, contribution.browserEntry, contribution.ownerRoot);
  return contribution;
}
