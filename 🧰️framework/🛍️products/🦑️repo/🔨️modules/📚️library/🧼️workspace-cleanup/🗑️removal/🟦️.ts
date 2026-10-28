import { generatedOutputHasLiveLease } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import { existsSync, rmSync } from "node:fs";
import { relative, resolve } from "node:path";
import { runProbe } from "../../🏃️process/🔍️probe/🟦️.ts";
import {
  cleanBuildArtifactRemovals,
  cleanCollectMisplaced,
  cleanCollectRootTransient,
  cleanCollectWindowsIllegal,
  cleanDedupePreferDeepest,
  cleanDiscoverTicketFolders,
  cleanDiscoverTicketRoots,
  cleanGitignoredMapForTicketRoots,
  cleanOversizedIgnoredRemovals,
  cleanPathBytes,
  cleanTicketGeneratedOutputRemovals,
  cleanCollectRustIncrementalRemovals,
} from "../🔍️candidate-discovery/🟦️.ts";
import {
  CLEAN_PROTECTION_VIEW,
  cleanManagedStoragePrefixes,
  type CleanRemoval,
  cleanIntersectsProtected,
  cleanIsInsideTicketGeneratedOutput,
  cleanIsNativeImagePath,
  cleanIsProtected,
  cleanOpenTicketGeneratedOutputProtection,
  cleanProjectRemovals,
  cleanProtectedPrefixes,
  cleanRemovalProtection,
  cleanTicketFolderForPath,
  CLEAN_RUST_INCREMENTAL_MAX_BYTES,
  cleanIsRustIncrementalRemovalPath,
  cleanRustIncrementalHasWorkingSession,
  cleanRustIncrementalSearchRoots,
} from "../🛡️protection/🟦️.ts";

export function cleanRemovePath(root: string, abs: string, dry: boolean, protectedPrefixes: readonly string[], allowWindowsIllegal = false, allowCargoIncremental = false): boolean {
  if(cleanIntersectsProtected(abs,cleanManagedStoragePrefixes(root)))return false;
  if (cleanIsNativeImagePath(abs) || (!allowCargoIncremental && cleanOpenTicketGeneratedOutputProtection(root, abs).length !== 0)) return false;
  if (allowCargoIncremental) {
    if (!cleanIsRustIncrementalRemovalPath(root, abs, cleanRustIncrementalSearchRoots(root)) || cleanRustIncrementalHasWorkingSession(abs)) return false;
    if (dry) return true;
    if (existsSync(abs)) rmSync(abs, { recursive: true, force: true });
    return true;
  }
  if (cleanIntersectsProtected(abs, protectedPrefixes) || cleanRemovalProtection(root, abs, CLEAN_PROTECTION_VIEW, cleanIsInsideTicketGeneratedOutput(root, abs) ? "unlink" : "protect").length !== 0) return false;
  if (cleanTicketFolderForPath(root, abs) !== undefined && generatedOutputHasLiveLease(abs)) return false;
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
export function runWorkspaceClean(root: string, dry: boolean, rustIncrementalMaxBytes = CLEAN_RUST_INCREMENTAL_MAX_BYTES): { removals: CleanRemoval[]; skippedProtected: string[] } {
  const protectedPrefixes = cleanProtectedPrefixes(root);
  const ticketRoots = cleanDiscoverTicketRoots(root);
  const ticketFolders = ticketRoots.flatMap(cleanDiscoverTicketFolders);
  const skippedProtected = protectedPrefixes.filter((p) => existsSync(p)).map((p) => relative(root, p) || p);
  const pending: CleanRemoval[] = [];
  pending.push(...cleanCollectMisplaced(root, protectedPrefixes));
  pending.push(...cleanCollectRootTransient(root, protectedPrefixes));
  pending.push(...cleanCollectWindowsIllegal(root, protectedPrefixes));
  pending.push(...cleanOversizedIgnoredRemovals(root, protectedPrefixes));
  const gitignoredMap = cleanGitignoredMapForTicketRoots(root, ticketRoots);
  for (const ticketFolder of ticketFolders) {
    pending.push(...cleanTicketGeneratedOutputRemovals(root, ticketFolder, protectedPrefixes));
    if (cleanIsProtected(ticketFolder, protectedPrefixes)) continue;
    const gitignored = gitignoredMap.get(resolve(ticketFolder)) ?? [];
    for (const abs of gitignored) {
      if (!existsSync(abs) || cleanIntersectsProtected(abs, protectedPrefixes)) continue;
      pending.push({ kind: "gitignore", path: relative(root, abs), bytes: cleanPathBytes(abs) });
    }
  }
  pending.push(...cleanBuildArtifactRemovals(root, protectedPrefixes));
  pending.push(...cleanCollectRustIncrementalRemovals(root, rustIncrementalMaxBytes));
  const candidates = cleanProjectRemovals(root, pending, protectedPrefixes, CLEAN_PROTECTION_VIEW, (path) => skippedProtected.push(relative(root, path) || path));
  const removals: CleanRemoval[] = [];
  for (const row of candidates) {
    if (cleanRemovePath(root, resolve(root, row.path), dry, protectedPrefixes, row.kind === "windows-illegal", row.kind === "cargo-incremental")) removals.push(row);
    else skippedProtected.push(row.path);
  }
  return { removals, skippedProtected };
}
