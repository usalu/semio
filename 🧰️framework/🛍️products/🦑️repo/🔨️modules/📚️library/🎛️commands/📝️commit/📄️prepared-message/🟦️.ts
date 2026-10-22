import { createHash } from "node:crypto";
import { join } from "node:path";
import { existsSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { git, gitDir, gitRepoRoot } from "../🌳️git/🟦️.ts";
import { branchAllowed, type Contributor, findContributor } from "../👤️contributor/🟦️.ts";
import type { MetricsRunner } from "../📊️metrics/🏷️types/🟦️.ts";
import { formatSecond, nextCounter } from "../🔢️counter/🟦️.ts";
import { listAddedTicketPaths, normalizeBulletLines, readPreparedBullets, ticketBullets } from "../🔸️bullets/🟦️.ts";
import { createDefaultMetricsRunner } from "../📊️metrics/💾️scan/🟦️.ts";
import { buildCommitMetrics, formatCommitMetricsLines, validateCommitMetricsDeltaSum } from "../📊️metrics/🧮️summary/🟦️.ts";
import {refuseCommitV1} from "../📖️texts/🟦️.ts";

export function digestMicroCommitMessage(message: string): string {
  return createHash("sha256").update(message.replace(/\r\n/g, "\n").trimEnd()).digest("hex");
}

function preparedDigestPath(root: string): string {
  return join(gitDir(root), "compose-micro-commit-digest");
}

function preparedActivePath(root: string): string {
  return join(gitDir(root), "compose-micro-commit-active");
}

function markPrepareActive(root: string): void {
  writeFileSync(preparedActivePath(root), "1\n");
}

function isPrepareActive(root: string): boolean {
  return existsSync(preparedActivePath(root));
}

const GK_TEMPLATE_BASENAME = "gkcommittemplate";

export const GK_COMMIT_TEMPLATE_FILE = `${GK_TEMPLATE_BASENAME}.txt`;

const GIT_COMMIT_DRAFT_FILES = ["COMMIT_EDITMSG", "MERGE_MSG", "SQUASH_MSG"] as const;

export function buildMicroCommitMessage(root: string, contributor: Contributor, diffBullets: string[] = [], metricsRunner?: MetricsRunner): string {
  root = gitRepoRoot(root);
  const { line1Base, nnn } = nextCounter(root, contributor);
  const now = new Date();
  const authored = diffBullets.length > 0 ? normalizeBulletLines(diffBullets.join("\n")) : readPreparedBullets(root);
  const tickets = ticketBullets(root);
  const authoredLower = new Set(authored.map((b) => b.toLowerCase()));
  const bullets = [...authored, ...tickets.filter((t) => !authoredLower.has(t.toLowerCase()))].slice(0, 8);
  if (bullets.length === 0) {
    throw refuseCommitV1("missing","message.at-least-one-description-bullet");
  }
  const runner = metricsRunner ?? createDefaultMetricsRunner();
  const metricBundle = buildCommitMetrics(root, runner);
  if (metricBundle.uloc.length === 0 && metricBundle.size.length === 0) {
    throw refuseCommitV1("missing","message.required-metric-footer-could-not");
  }
  validateCommitMetricsDeltaSum(metricBundle);
  const metrics = formatCommitMetricsLines(metricBundle);
  const lines = [`${line1Base}🚩️${nnn}`, formatSecond(now), ...bullets];
  lines.push("", ...metrics);
  lines.push("", `Signed-off-by: ${contributor.name} <${contributor.email}>`);
  return `${lines.join("\n")}\n`;
}

export function writeMicroCommitTemplates(root: string, message: string): void {
  const dir = gitDir(root);
  const gkCommitTemplate = join(dir, GK_COMMIT_TEMPLATE_FILE);
  removeGitKrakenTemplateFiles(root);
  writeFileSync(gkCommitTemplate, message);
  for (const name of GIT_COMMIT_DRAFT_FILES) {
    writeFileSync(join(dir, name), message);
  }
  git(root, ["config", "--local", "commit.template", gkCommitTemplate]);
  writeFileSync(preparedDigestPath(root), `${digestMicroCommitMessage(message)}\n`);
  markPrepareActive(root);
}

export function shouldRefreshPreparedCommitMessage(current: string, preparedDigest: string | null): boolean {
  const trimmed = current.trim();
  if (!trimmed) return true;
  if (!preparedDigest) return false;
  return digestMicroCommitMessage(current) === preparedDigest.trim();
}

function removeGitDirPrefixed(root: string, prefix: string): void {
  const dir = gitDir(root);
  for (const name of readdirSync(dir)) {
    if (name.startsWith(prefix)) {
      try {
        rmSync(join(dir, name), { force: true });
      } catch {
        /* ignore */
      }
    }
  }
}

function removeGitKrakenTemplateFiles(root: string): void {
  const dir = gitDir(root);
  for (const name of readdirSync(dir)) {
    if (!name.startsWith(GK_TEMPLATE_BASENAME)) continue;
    try {
      rmSync(join(dir, name), { force: true });
    } catch {
      /* ignore */
    }
  }
}

function resetGitCommitTemplateState(root: string): void {
  const dir = gitDir(root);
  removeGitKrakenTemplateFiles(root);
  const gkCommitTemplate = join(dir, GK_COMMIT_TEMPLATE_FILE);
  writeFileSync(gkCommitTemplate, "");
  git(root, ["config", "--local", "commit.template", gkCommitTemplate]);
}

/** 🧹️Clears GitKraken templates, git draft messages, and micro-commit prepare state. */
export function clearGitCommitDraftState(root: string): void {
  const dir = gitDir(root);
  resetGitCommitTemplateState(root);
  for (const name of GIT_COMMIT_DRAFT_FILES) {
    try {
      writeFileSync(join(dir, name), "");
    } catch {
      /* ignore */
    }
  }
  removeGitDirPrefixed(root, "compose-micro-commit");
}

/** 🧹️Resets GK/git commit-template state without wiping the active draft message file (COMMIT_EDITMSG/MERGE_MSG/SQUASH_MSG). */
export function clearMicroCommitTemplatesOnly(root: string): void {
  resetGitCommitTemplateState(root);
  removeGitDirPrefixed(root, "compose-micro-commit");
}

export function clearStaleTemplatesBeforePrepare(root: string): void {
  if (!isPrepareActive(root)) clearGitCommitDraftState(root);
}

/** 🧹️Removes prepare state and resets GK/git templates to empty after a commit. */
export function wipeAfterCommit(root: string): void {
  clearGitCommitDraftState(root);
}

export function handlePrepareCommitMsg(root: string, msgFile: string, source: string): void {
  if (!isPrepareActive(root)) {
    clearMicroCommitTemplatesOnly(root);
    return;
  }
  if (!branchAllowed(root)) return;
  const contributor = findContributor(root);
  if (!contributor) return;
  if (source === "merge" || source === "squash") return;
  const preparedBullets = readPreparedBullets(root);
  const newTickets = listAddedTicketPaths(root);
  if (preparedBullets.length === 0 && newTickets.length === 0) {
    const current = existsSync(msgFile) ? readFileSync(msgFile, "utf8") : "";
    if (current.trim()) return;
    return;
  }
  const digestPath = preparedDigestPath(root);
  const preparedDigest = existsSync(digestPath) ? readFileSync(digestPath, "utf8") : null;
  const current = existsSync(msgFile) ? readFileSync(msgFile, "utf8") : "";
  if (!shouldRefreshPreparedCommitMessage(current, preparedDigest)) return;
  const message = buildMicroCommitMessage(root, contributor, preparedBullets);
  writeFileSync(msgFile, message);
  writeMicroCommitTemplates(root, message);
}
