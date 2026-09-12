import { existsSync, rmSync } from "node:fs";
import { relative, resolve } from "node:path";
import { exactCargoGeneratedOutputHasLiveLease, runProbe } from "../../🟦️.ts";
import { cleanBuildArtifactRemovals, cleanCollectMisplaced, cleanCollectWindowsIllegal, cleanDedupePreferDeepest, cleanDiscoverTicketFolders, cleanDiscoverTicketRoots, cleanGitignoredMapForTicketRoots, cleanPathBytes, cleanTicketGeneratedOutputRemovals, cleanTicketSizeRemovals } from "../🔍️candidate-discovery/🟦️.ts";
import { CLEAN_PROTECTION_VIEW, type CleanRemoval, cleanIntersectsProtected, cleanIsProtected, cleanProjectRemovals, cleanProtectedPrefixes, cleanRemovalProtection, cleanTicketFolderForPath, cleanTicketGeneratedOutputTicketRoot, cleanTicketManifestIsClosed } from "../🛡️protection/🟦️.ts";

export function cleanRemovePath(root: string, abs: string, dry: boolean, protectedPrefixes: readonly string[], allowTicketGeneratedOutput = false, allowWindowsIllegal = false): boolean {
  const allowedOpenTicket = (allowTicketGeneratedOutput ? cleanTicketGeneratedOutputTicketRoot(root, abs) : undefined)
    ?? (allowWindowsIllegal ? cleanTicketFolderForPath(root, abs) : undefined);
  const applicablePrefixes = allowedOpenTicket ? protectedPrefixes.filter((prefix) => resolve(prefix) !== allowedOpenTicket) : protectedPrefixes;
  if (cleanIntersectsProtected(abs, applicablePrefixes) || (!allowedOpenTicket && cleanRemovalProtection(root, abs, CLEAN_PROTECTION_VIEW, allowedOpenTicket).length !== 0)) return false;
  if (allowedOpenTicket && exactCargoGeneratedOutputHasLiveLease(abs)) return false;
  if (dry) return true;
  if (allowWindowsIllegal) {
    try {
      runProbe("git", ["rm", "-f", "--", relative(root, abs)], { cwd: root });
    } catch {}
  }
  if (existsSync(abs)) {
    rmSync(abs, { recursive: true, force: true });
  }
  return true;
}
export function runWorkspaceClean(root: string, dry: boolean): { removals: CleanRemoval[]; skippedProtected: string[] } {
  const protectedPrefixes = cleanProtectedPrefixes(root);
  const ticketRoots = cleanDiscoverTicketRoots(root);
  const ticketFolders = ticketRoots.flatMap(cleanDiscoverTicketFolders);
  for (const folder of ticketFolders) if (!cleanTicketManifestIsClosed(folder, CLEAN_PROTECTION_VIEW)) protectedPrefixes.push(resolve(folder));
  const skippedProtected = protectedPrefixes.filter((p) => existsSync(p)).map((p) => relative(root, p) || p);
  const pending: CleanRemoval[] = [];
  pending.push(...cleanCollectMisplaced(root, protectedPrefixes));
  pending.push(...cleanCollectWindowsIllegal(root, protectedPrefixes));
  const gitignoredMap = cleanGitignoredMapForTicketRoots(root, ticketRoots);
  for (const ticketFolder of ticketFolders) {
    pending.push(...cleanTicketGeneratedOutputRemovals(root, ticketFolder, protectedPrefixes));
    if (cleanIsProtected(ticketFolder, protectedPrefixes) || !cleanTicketManifestIsClosed(ticketFolder, CLEAN_PROTECTION_VIEW)) continue;
    const gitignored = gitignoredMap.get(resolve(ticketFolder)) ?? [];
    for (const abs of gitignored) {
      if (!existsSync(abs) || cleanIntersectsProtected(abs, protectedPrefixes)) continue;
      pending.push({ kind: "gitignore", path: relative(root, abs), bytes: cleanPathBytes(abs) });
    }
    pending.push(...cleanDedupePreferDeepest(cleanTicketSizeRemovals(root, ticketFolder, protectedPrefixes)));
  }
  pending.push(...cleanBuildArtifactRemovals(root, protectedPrefixes));
  const candidates = cleanProjectRemovals(root, pending, protectedPrefixes, CLEAN_PROTECTION_VIEW, (path) => skippedProtected.push(relative(root, path) || path));
  const removals: CleanRemoval[] = [];
  for (const row of candidates) {
    if (cleanRemovePath(root, resolve(root, row.path), dry, protectedPrefixes, row.kind === "ticket-generated", row.kind === "windows-illegal")) removals.push(row);
    else skippedProtected.push(row.path);
  }
  return { removals, skippedProtected };
}
