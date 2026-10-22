import { join } from "node:path";
import { existsSync, readFileSync, writeFileSync } from "node:fs";

import { branchValidationError, type Contributor, currentBranch, findContributor } from "../👤️contributor/🟦️.ts";
import type { CommitBundleSection } from "../🧾️bundle-model/🟦️.ts";
import type { MetricsRunner } from "../📊️metrics/🏷️types/🟦️.ts";
import { git, gitDir, gitEmail, gitRepoRoot } from "../🌳️git/🟦️.ts";
import { assignChangedPathsToBundles, buildBundlePathPrefixSets } from "../🧭️path-prefixes/🟦️.ts";
import { buildBundleDateDeltasMap, buildBundleDateSizeDeltasMap, formatBundleHeaderLine, formatGitDeltaSumBrief, partitionRangeDeltasByBundle, partitionRangeSizeDeltasByBundle, rangeGitDeltaTotal, sortCommitBundlesByEditTotal, validateBundleCommitAttribution } from "../📈️attribution/🟦️.ts";
import { createDefaultMetricsRunner } from "../📊️metrics/💾️scan/🟦️.ts";
import { buildCommitMetricsForRange, countBytesUnderPathPrefixes, countUnifiedLocUnderPathPrefixes, formatCommitMetricsLines } from "../📊️metrics/🧮️summary/🟦️.ts";
import { findLastBundleWipCommit, formatBundleSubject, formatBundleTagName } from "../🔖️tags/🟦️.ts";
import { formatBundleDateLine } from "../📅️dates/🟦️.ts";
import { emitCommitAnalysis, emitCommitDiff, emitCommitLog, validateBundleBulletsFresh } from "../📤️emit/🟦️.ts";
import { commitBundleBodyError, parseCommitBundleBody } from "../📜️body/🟦️.ts";
import { isCommitPrepareOnly, loadCommitSteps } from "../🪜️steps/🟦️.ts";
import { formatCommitPrepareAgentReply } from "../📋️prepare-commands/🟦️.ts";
import { runToolSyncV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import {sayCommitV1,reportCommitErrorV1} from "../📖️texts/🟦️.ts";

export function buildCommitMessage(root: string, contributor: Contributor, bundles: CommitBundleSection[], wipSha: string, head = "HEAD", metricsRunner?: MetricsRunner, now = new Date()): string {
  root = gitRepoRoot(root);
  const pathAssignments = assignChangedPathsToBundles(root, wipSha, head, bundles);
  const sorted = sortCommitBundlesByEditTotal(root, wipSha, head, bundles, pathAssignments);
  bundles = sorted.bundles;
  const runner = metricsRunner ?? createDefaultMetricsRunner();
  validateBundleCommitAttribution(root, wipSha, head, bundles, runner);
  const prefixSets = buildBundlePathPrefixSets(root, wipSha, head, bundles);
  const { bundleTotals: ulocBundleTotals } = partitionRangeDeltasByBundle(root, wipSha, head, bundles, prefixSets);
  const { bundleTotals: sizeBundleTotals } = partitionRangeSizeDeltasByBundle(root, wipSha, head, bundles, prefixSets);
  const dateDeltas = buildBundleDateDeltasMap(root, wipSha, head, bundles);
  const dateSizeDeltas = buildBundleDateSizeDeltasMap(root, wipSha, head, bundles);
  const ulocBundleBlocs = prefixSets.map((prefixes) => countUnifiedLocUnderPathPrefixes(root, prefixes));
  const sizeBundleBlocs = prefixSets.map((prefixes) => countBytesUnderPathPrefixes(root, prefixes));
  const lines: string[] = [formatBundleSubject(contributor, now), ""];
  for (let bi = 0; bi < bundles.length; bi++) {
    const bundle = bundles[bi]!;
    const ulocBloc = ulocBundleBlocs[bi] ?? 0;
    const sizeBloc = sizeBundleBlocs[bi] ?? 0;
    lines.push(formatBundleHeaderLine(bundle.label, ulocBundleTotals[bi] ?? { added: 0, removed: 0, edited: 0 }, sizeBundleTotals[bi] ?? { added: 0, removed: 0, edited: 0 }, ulocBloc, sizeBloc));
    const perDay = dateDeltas.get(bi);
    const perSizeDay = dateSizeDeltas.get(bi);
    for (const section of bundle.dates) {
      const dayUloc = perDay?.get(section.dateLine) ?? { added: 0, removed: 0, edited: 0 };
      const daySize = perSizeDay?.get(section.dateLine) ?? { added: 0, removed: 0, edited: 0 };
      lines.push(formatBundleDateLine(section.dateLine, dayUloc, daySize, ulocBloc, sizeBloc));
      lines.push(...section.bullets);
    }
    if (bi < bundles.length - 1) lines.push("");
  }
  const metrics = formatCommitMetricsLines(buildCommitMetricsForRange(root, wipSha, head, undefined, runner));
  if (metrics.length > 0) lines.push(...metrics, "");
  lines.push(`Signed-off-by: ${contributor.name} <${contributor.email}>`);
  return `${lines.join("\n")}\n`;
}

function readBodyInput(root: string, file: string | null): string {
  if (file) {
    const path = file.startsWith("/") ? file : join(root, file);
    return readFileSync(path, "utf8");
  }
  if (process.stdin.isTTY) return "";
  return readFileSync(0, "utf8");
}

function assertCleanWorktree(root: string): void {
  const st = git(root, ["status", "--porcelain"]);
  if (!st.ok) return;
  if (st.out.trim()) {
    sayCommitV1("run.working-tree-must-be-clean",{},"error");
    process.exit(1);
  }
}

function emitStdout(message: string): void {
  process.stdout.write(message.endsWith("\n") ? message : `${message}\n`);
}

export function runCommit(root: string, segments: string[]): void {
  root = gitRepoRoot(root);
  const cmd = segments[0] ?? "prepare";
  const branchError = branchValidationError("commit", currentBranch(root));
  if (branchError) {
    console.error(branchError);
    process.exit(1);
  }
  const contributor = findContributor(root);
  if (!contributor) {
    sayCommitV1("run.no-contributor-for-git-user",{email:String(gitEmail(root) || "(unset)")},"error");
    process.exit(1);
  }

  if (cmd === "log") {
    emitCommitLog(root);
    process.exit(0);
  }

  if (cmd === "diff") {
    emitCommitDiff(root);
    process.exit(0);
  }

  if (cmd === "analyze") {
    emitCommitAnalysis(root);
    process.exit(0);
  }

  if (cmd === "check") {
    const dash = segments.indexOf("--");
    const bodyFile = dash >= 0 ? (segments[dash + 1] ?? null) : null;
    const body = readBodyInput(root, bodyFile);
    if (!body.trim()) {
      sayCommitV1("run.check-pass-bundle-body-on",{},"error");
      process.exit(1);
    }
    const bodyErr = commitBundleBodyError(body);
    if (bodyErr) {
      console.error(bodyErr);
      process.exit(1);
    }
    const wip = findLastBundleWipCommit(root);
    if (!wip) {
      sayCommitV1("run.no-prior-bundle-wip-found",{},"error");
      process.exit(1);
    }
    const ahead = git(root, ["rev-list", "--count", `${wip.sha}..HEAD`]);
    if (!ahead.ok || Number(ahead.out) === 0) {
      sayCommitV1("run.no-commits-after-last-bundle",{},"error");
      process.exit(1);
    }
    try {
      const bundles = parseCommitBundleBody(body);
      validateBundleBulletsFresh(root, wip.sha, "HEAD", bundles);
      validateBundleCommitAttribution(root, wip.sha, "HEAD", bundles);
    } catch (e) {
      reportCommitErrorV1(e);
      process.exit(1);
    }
    const range = rangeGitDeltaTotal(root, wip.sha, "HEAD");
    sayCommitV1("run.check-ok",{range:String(formatGitDeltaSumBrief(range))},"error");
    sayCommitV1("run.check-per-bundle-days-bundle",{},"error");
    process.exit(0);
  }

  if (cmd !== "prepare") {
    console.error("[commit] usage: repo ./📜️script.ts commit <log|diff|analyze|check|prepare> [ct|cs|cp|…] [-- body.txt]");
    process.exit(1);
  }

  const dash = segments.indexOf("--");
  const levelSegments = dash >= 0 ? segments.slice(1, dash) : segments.slice(1);
  const bodyFile = dash >= 0 ? (segments[dash + 1] ?? null) : null;
  const steps = loadCommitSteps(root, contributor, levelSegments);
  const prepareOnly = isCommitPrepareOnly(levelSegments);
  const messagePath = join(gitDir(root), "compose-commit-message");

  const wip = findLastBundleWipCommit(root);
  if (!wip) {
    sayCommitV1("run.no-prior-bundle-wip-found",{},"error");
    process.exit(1);
  }

  const ahead = git(root, ["rev-list", "--count", `${wip.sha}..HEAD`]);
  if (!ahead.ok || Number(ahead.out) === 0) {
    sayCommitV1("run.no-commits-after-last-bundle-2",{},"error");
    process.exit(1);
  }

  const body = readBodyInput(root, bodyFile);
  let message: string;
  if (body.trim()) {
    const bodyErr = commitBundleBodyError(body);
    if (bodyErr) {
      console.error(bodyErr);
      process.exit(1);
    }
    let bundles: CommitBundleSection[];
    try {
      bundles = parseCommitBundleBody(body);
      validateBundleBulletsFresh(root, wip.sha, "HEAD", bundles);
    } catch (e) {
      reportCommitErrorV1(e);
      process.exit(1);
    }
    try {
      message = buildCommitMessage(root, contributor, bundles, wip.sha);
    } catch (e) {
      reportCommitErrorV1(e);
      process.exit(1);
    }
    writeFileSync(messagePath, message);
  } else if (existsSync(messagePath)) {
    message = readFileSync(messagePath, "utf8");
    if (!message.trim()) {
      sayCommitV1("run.compose-message-is-empty-prepare",{},"error");
      process.exit(1);
    }
  } else {
    emitCommitAnalysis(root);
    process.exit(1);
  }

  if (prepareOnly) {
    if (!body.trim()) {
      emitCommitAnalysis(root);
      process.exit(1);
    }
    const tagName = formatBundleTagName(contributor);
    emitStdout(
      formatCommitPrepareAgentReply({
        tagName,
        wipSha: wip.sha,
        messageFile: ".git/compose-commit-message",
        commitMessage: message,
      }),
    );
    process.exit(0);
  }

  if (steps.tag || steps.squash || steps.push) assertCleanWorktree(root);

  if (steps.tag) {
    const tagName = formatBundleTagName(contributor);
    const tag = runToolSyncV1("git", ["tag", "-s", "-m", tagName, tagName, "HEAD"], { cwd: root, encoding: "utf8" });
    if (tag.status !== 0) {
      console.error((tag.stderr ?? tag.stdout ?? "git tag failed").trim());
      process.exit(tag.status ?? 1);
    }
  }

  if (steps.squash) {
    const reset = runToolSyncV1("git", ["reset", "--soft", wip.sha], { cwd: root, encoding: "utf8" });
    if (reset.status !== 0) {
      console.error((reset.stderr ?? reset.stdout ?? "git reset --soft failed").trim());
      process.exit(reset.status ?? 1);
    }
    writeFileSync(join(gitDir(root), "COMMIT_EDITMSG"), message);
    const commit = runToolSyncV1("git", ["commit", "-S", "-F", join(gitDir(root), "COMMIT_EDITMSG")], {
      cwd: root,
      encoding: "utf8",
    });
    if (commit.status !== 0) {
      console.error((commit.stderr ?? commit.stdout ?? "git commit failed").trim());
      process.exit(commit.status ?? 1);
    }
  }

  if (steps.push) {
    const push = runToolSyncV1("git", ["push", "--follow-tags"], { cwd: root, encoding: "utf8" });
    if (push.status !== 0) {
      console.error((push.stderr ?? push.stdout ?? "git push failed").trim());
      process.exit(push.status ?? 1);
    }
  }

  emitStdout(message);
  process.exit(0);
}
