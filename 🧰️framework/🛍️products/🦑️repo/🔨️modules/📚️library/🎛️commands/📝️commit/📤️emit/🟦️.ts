import { git, gitRepoRoot } from "../🌳️git/🟦️.ts";
import { findLastBundleWipCommit } from "../🔖️tags/🟦️.ts";
import { spawnCapturedSync } from "../../../🏃️process/🔍️probe/🟦️.ts";
import { gitSpawnEnv } from "../../../🏃️process/🌿️environment/🌳️git/🟦️.ts";
import { BUNDLE_DATE_SECTION_RE, BUNDLE_WIP_SUBJECT_RE, type CommitBundleSection } from "../🧾️bundle-model/🟦️.ts";
import { normalizeBundleDateLine } from "../📅️dates/🟦️.ts";
import {sayCommitV1,refuseCommitV1} from "../📖️texts/🟦️.ts";

const COMMIT_DIFF_MAX_BYTES = 1_500_000;

export type CommitBundleRange = { wip: { sha: string; subject: string }; range: string };

/** 🔀️Resolves last bundle WIP and revision range for analysis. */
export function resolveCommitBundleRange(root: string): CommitBundleRange | null {
  root = gitRepoRoot(root);
  const wip = findLastBundleWipCommit(root);
  if (!wip) return null;
  return { wip, range: `${wip.sha}..HEAD` };
}

function normalizeCompareLine(s: string): string {
  return s.trim().replace(/\s+/g, " ").toLowerCase();
}

/** 📜️Collects comparable lines from commit bodies in range (for copy detection). */
export function commitHistoryCompareLines(root: string, base: string, head: string): Set<string> {
  const r = spawnCapturedSync("git", ["log", `--format=%B%x00`, `${base}..${head}`], {
    cwd: gitRepoRoot(root),
    env: gitSpawnEnv(),
  });
  const lines = new Set<string>();
  if (r.status !== 0) return lines;
  for (const body of r.stdout.toString("utf8").split("\0")) {
    if (!body.trim()) continue;
    for (const raw of body.split("\n")) {
      const line = raw.trim();
      if (line.length < 12) continue;
      if (BUNDLE_WIP_SUBJECT_RE.test(line)) continue;
      if (BUNDLE_DATE_SECTION_RE.test(normalizeBundleDateLine(line))) continue;
      if (line.startsWith("📊️metric") || line.startsWith("Signed-off-by:")) continue;
      if (line.startsWith("Signed-off-by:")) continue;
      if (/^🎆️\d{2}🌙️\d{2}☀️\d{2}⏰️/u.test(line)) continue;
      lines.add(normalizeCompareLine(line));
    }
  }
  return lines;
}

/** 🚫️Whether a bullet verbatim-matches a line from a prior commit body in the range. */
export function bulletMatchesCommitHistory(bullet: string, history: Set<string>): boolean {
  return history.has(normalizeCompareLine(bullet));
}

/** 🆕️Ensures bullets are newly written from diff analysis, not pasted from prior commits. */
export function validateBundleBulletsFresh(root: string, base: string, head: string, bundles: CommitBundleSection[]): void {
  const history = commitHistoryCompareLines(root, base, head);
  for (const bundle of bundles) {
    for (const section of bundle.dates) {
      for (const bullet of section.bullets) {
        if (bulletMatchesCommitHistory(bullet, history)) {
          throw refuseCommitV1("failed","emit.bullet-copies-a-prior-message",{bullet:String(bullet)});
        }
      }
    }
  }
}

function emitCommitBundleAttributionNote(): void {
  sayCommitV1("emit.bundles-and-file-bundle-mapping",{},"error");
  sayCommitV1("emit.you-must-read-log-for",{},"error");
  sayCommitV1("emit.script-only-adds-subject-uloc",{},"error");
  sayCommitV1("emit.prepare-check-fail-unless-days",{},"error");
}

function emitCommitAnalysisHint(): void {
  sayCommitV1("emit.write-new-bundle-scopes-dates",{},"error");
}

/** 🪵️Prior commit messages since last WIP (context for dates only — not for copying bullets). */
export function emitCommitLog(root: string): void {
  root = gitRepoRoot(root);
  const resolved = resolveCommitBundleRange(root);
  if (!resolved) {
    sayCommitV1("emit.no-prior-bundle-wip-found",{},"error");
    process.exit(1);
  }
  const { wip, range } = resolved;
  sayCommitV1("emit.last-bundle-wip",{subject:String(wip.subject),sha:String(wip.sha.slice(0, 7))},"error");
  console.error(`Range: ${range}\n`);
  emitCommitBundleAttributionNote();
  sayCommitV1("emit.prior-messages",{},"error");
  const log = git(root, ["log", `--format=commit %h%n%s%n%b%n---`, range]);
  if (log.ok && log.out) console.error(log.out);
  emitCommitAnalysisHint();
}

/** 📊️Git diff since last WIP (primary source for new bundle summaries). */
export function emitCommitDiff(root: string): void {
  root = gitRepoRoot(root);
  const resolved = resolveCommitBundleRange(root);
  if (!resolved) {
    sayCommitV1("emit.no-prior-bundle-wip-found",{},"error");
    process.exit(1);
  }
  const { wip, range } = resolved;
  sayCommitV1("emit.last-bundle-wip",{subject:String(wip.subject),sha:String(wip.sha.slice(0, 7))},"error");
  console.error(`Range: ${range}\n`);
  const stat = git(root, ["diff", "--stat", range]);
  emitCommitBundleAttributionNote();
  sayCommitV1("emit.git-diff-stat",{},"error");
  if (stat.ok && stat.out) console.error(`${stat.out}\n`);
  const patch = git(root, ["diff", range]);
  sayCommitV1("emit.git-diff",{},"error");
  if (!patch.ok || !patch.out) {
    console.error(patch.out || "(empty diff)\n");
    emitCommitAnalysisHint();
    return;
  }
  const bytes = Buffer.byteLength(patch.out, "utf8");
  if (bytes > COMMIT_DIFF_MAX_BYTES) {
    console.error(patch.out.slice(0, COMMIT_DIFF_MAX_BYTES));
    sayCommitV1("emit.diff-truncated-at-maximumbytes-bytes",{maximumBytes:String(COMMIT_DIFF_MAX_BYTES),bytes:String(bytes),range:String(range)},"error");
  } else {
    console.error(`${patch.out}\n`);
  }
  emitCommitAnalysisHint();
}

export function emitCommitAnalysis(root: string): void {
  emitCommitLog(root);
  console.error("\n");
  emitCommitDiff(root);
}
