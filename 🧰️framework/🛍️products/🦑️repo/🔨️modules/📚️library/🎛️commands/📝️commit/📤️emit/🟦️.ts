import { git, gitRepoRoot } from "../🌳️git/🟦️.ts";
import { findLastBundleWipCommit } from "../🔖️tags/🟦️.ts";
import { spawnCapturedSync } from "../../../🏃️process/🔍️probe/🟦️.ts";
import { gitSpawnEnv } from "../../../🏃️process/🌿️environment/🌳️git/🟦️.ts";
import { BUNDLE_DATE_SECTION_RE, BUNDLE_WIP_SUBJECT_RE, type CommitBundleSection } from "../🧾️bundle-model/🟦️.ts";
import { normalizeBundleDateLine } from "../📅️dates/🟦️.ts";

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

/** 🚫️Ensures bullets are newly written from diff analysis, not pasted from prior commits. */
export function validateBundleBulletsFresh(root: string, base: string, head: string, bundles: CommitBundleSection[]): void {
  const history = commitHistoryCompareLines(root, base, head);
  for (const bundle of bundles) {
    for (const section of bundle.dates) {
      for (const bullet of section.bullets) {
        if (bulletMatchesCommitHistory(bullet, history)) {
          throw new Error(`commit: bullet copies a prior commit message line — rewrite from git diff only: ${bullet}`);
        }
      }
    }
  }
}

function emitCommitBundleAttributionNote(): void {
  console.error("commit: bundles and file→bundle mapping are NOT automatic — folder layout and bundle boundaries change between WIPs");
  console.error("commit: you must (1) read log for last bundle/WIP state, (2) read diff --stat + full diff for every path, (3) decide scopes/dates/bullets, then prepare stdin");
  console.error("commit: script only adds subject, uloc suffixes, sort order, footer, Signed-off-by — never invents bundles or bullets");
  console.error("commit: prepare/check fail unless days→bundle, bundles→range, and languages→range (all ➕️✏️➖️🟰️); run: bun ./📜️script.ts commit check");
}

function emitCommitAnalysisHint(): void {
  console.error("commit: write NEW bundle scopes, dates, and bullets on prepare stdin after log + diff attribution");
}

/** 📜️Prior commit messages since last WIP (context for dates only — not for copying bullets). */
export function emitCommitLog(root: string): void {
  root = gitRepoRoot(root);
  const resolved = resolveCommitBundleRange(root);
  if (!resolved) {
    console.error("commit: no prior bundle WIP commit (subject …🔀️) found in recent history");
    process.exit(1);
  }
  const { wip, range } = resolved;
  console.error(`Last bundle WIP: ${wip.subject} (${wip.sha.slice(0, 7)})`);
  console.error(`Range: ${range}\n`);
  emitCommitBundleAttributionNote();
  console.error("\n=== prior commit messages (context only — do not copy bullets; old format may be wrong) ===\n");
  const log = git(root, ["log", `--format=commit %h%n%s%n%b%n---`, range]);
  if (log.ok && log.out) console.error(log.out);
  emitCommitAnalysisHint();
}

/** 📊️Git diff since last WIP (primary source for new bundle summaries). */
export function emitCommitDiff(root: string): void {
  root = gitRepoRoot(root);
  const resolved = resolveCommitBundleRange(root);
  if (!resolved) {
    console.error("commit: no prior bundle WIP commit (subject …🔀️) found in recent history");
    process.exit(1);
  }
  const { wip, range } = resolved;
  console.error(`Last bundle WIP: ${wip.subject} (${wip.sha.slice(0, 7)})`);
  console.error(`Range: ${range}\n`);
  const stat = git(root, ["diff", "--stat", range]);
  emitCommitBundleAttributionNote();
  console.error("\n=== git diff --stat (overview) ===\n");
  if (stat.ok && stat.out) console.error(`${stat.out}\n`);
  const patch = git(root, ["diff", range]);
  console.error("=== git diff (write bullets from this — not from prior commit text) ===\n");
  if (!patch.ok || !patch.out) {
    console.error(patch.out || "(empty diff)\n");
    emitCommitAnalysisHint();
    return;
  }
  const bytes = Buffer.byteLength(patch.out, "utf8");
  if (bytes > COMMIT_DIFF_MAX_BYTES) {
    console.error(patch.out.slice(0, COMMIT_DIFF_MAX_BYTES));
    console.error(`\n[commit diff truncated at ${COMMIT_DIFF_MAX_BYTES} bytes of ${bytes} — inspect locally: git diff ${range}]\n`);
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
