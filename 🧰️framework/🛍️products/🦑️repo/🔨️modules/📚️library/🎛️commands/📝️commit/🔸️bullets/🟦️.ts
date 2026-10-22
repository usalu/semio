import { join } from "node:path";
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { gitCachedNames, gitDir } from "../🌳️git/🟦️.ts";
import {sayCommitV1} from "../📖️texts/🟦️.ts";

const TICKET_JSON_RE = /^\.🧬semio\/🦑️repo\/🎫️tickets\/.+\/🎫️ticket\.json$/;

function preparedBulletsPath(root: string): string {
  return join(gitDir(root), "compose-micro-commit-bullets");
}

const EMOJI_LEAD_RE = /^((?:\p{Extended_Pictographic}(?:\uFE0F|\u200D\p{Extended_Pictographic})*)+)/u;

export const MICRO_COMMIT_BULLET_RE = /^(?:\p{Extended_Pictographic}(?:\uFE0F|\u200D\p{Extended_Pictographic})*)+\S/u;

const TIMESTAMP_LINE_RE = /^🎆️\d{2}🌙️\d{2}☀️\d{2}/u;

const RESERVED_BULLET_LEAD_EMOJIS = new Set(["🎆️", "📊️", "🔢️", "🚩️"]);

/** 🏷️Leading emoji grapheme on a bullet line, or "" if none. */
export function bulletLeadEmoji(line: string): string {
  const m = EMOJI_LEAD_RE.exec(line.trim());
  return m?.[1] ?? "";
}

/** 📝️Formats one bullet as `{emoji}{description}` (line starts with emoji, no leading `-`). */
export function formatMicroCommitBulletLine(line: string): string {
  let body = line.trim().replace(/^-+\s*/, "");
  return body.replace(new RegExp(`^${EMOJI_LEAD_RE.source}\\s+`, "u"), "$1");
}

const MICRO_COMMIT_METRIC_ROW_RE = /^📊️metric/u;

function isCommitMetricLine(line: string): boolean {
  return MICRO_COMMIT_METRIC_ROW_RE.test(line.trim());
}

function isMicroCommitUlocLine(line: string): boolean {
  return isCommitMetricLine(line);
}

/** 📝️Normalizes LLM-authored bullet lines to `{emoji}{description}`. */
export function normalizeBulletLines(text: string): string[] {
  return text
    .split("\n")
    .map((l) => l.trim())
    .filter((l) => l.length > 0 && !l.startsWith("#") && !isMicroCommitUlocLine(l))
    .map(formatMicroCommitBulletLine)
    .filter((l) => l.length > 1)
    .slice(0, 8);
}

export function validateBulletSpacing(bullets: string[]): void {
  for (const b of bullets) {
    if (MICRO_COMMIT_BULLET_RE.test(b)) continue;
    sayCommitV1("bullets.bullet-must-start-with-then",{bullet:String(b)},"error");
    process.exit(1);
  }
}

/** 🚫️Returns an error when a bullet uses reserved or timestamp emojis. */
export function bulletEmojiValidationError(bullets: string[]): string | null {
  for (const b of bullets) {
    const lead = bulletLeadEmoji(b);
    if (RESERVED_BULLET_LEAD_EMOJIS.has(lead)) {
      return `micro-commit: ${lead} is reserved for subject/timestamp/uloc — start each bullet with the emoji that best matches that line's description`;
    }
    if (TIMESTAMP_LINE_RE.test(b.trim())) {
      return "micro-commit: bullet must not copy the 🎆️YY🌙️MM☀️DD timestamp pattern — use one leading emoji that fits the change, not the calendar line";
    }
  }
  return null;
}

export function validateBulletEmojis(bullets: string[]): void {
  const err = bulletEmojiValidationError(bullets);
  if (err) {
    console.error(err);
    process.exit(1);
  }
}

export function writePreparedBullets(root: string, bullets: string[]): void {
  writeFileSync(preparedBulletsPath(root), `${bullets.join("\n")}\n`);
}

export function readPreparedBullets(root: string): string[] {
  const path = preparedBulletsPath(root);
  if (!existsSync(path)) return [];
  return normalizeBulletLines(readFileSync(path, "utf8"));
}

const STAGED_CHANGE_AREAS = [
  { id: "ticket plans", match: (p: string) => p.includes("🎫️tickets/") && p.endsWith(".plan.md"), keywords: ["plan"] },
  { id: ".agents", match: (p: string) => p.includes(".agents") && !p.endsWith("SKILL.md"), keywords: ["skill", "agent"] },
  { id: "repo", match: (p: string) => p.includes("repo/"), keywords: ["hook", "micro-commit"] },
  { id: ".devcontainer", match: (p: string) => p.includes(".devcontainer"), keywords: ["devcontainer"] },
  {
    id: "product",
    match: (p: string) => /(^|\/)(framework|puzzle|compose|cad|ui|mathematical|infinite|elements|coda|reuse|s)\//.test(p.replace(/^[^\w./-]+/, "")),
    keywords: [],
  },
] as const;

function isInsignificantStagedPath(path: string): boolean {
  return /\/micro-commit\.ts$/.test(path) || /\/index\.test\.ts$/.test(path) || path.endsWith("SKILL.md");
}

/** 🔤️Path tokens used to check whether bullets mention a staged file. */
export function pathTokensForBulletCoverage(filePath: string): string[] {
  return [
    ...new Set(
      filePath
        .toLowerCase()
        .split(/[/._-]+/)
        .filter((s) => s.length >= 4),
    ),
  ];
}

function bulletsMentionPathTokens(text: string, paths: string[]): boolean {
  const tokens = paths.flatMap(pathTokensForBulletCoverage);
  return tokens.some((t) => text.includes(t));
}

function bulletsCoverArea(text: string, paths: string[], keywords: readonly string[]): boolean {
  if (keywords.some((k) => text.includes(k))) return true;
  return bulletsMentionPathTokens(text, paths);
}

/** 🧪️Returns staged area ids not reflected in bullets (empty = ok). */
export function uncoveredStagedAreas(bullets: string[], staged: string[]): string[] {
  const significant = staged.filter((p) => !isInsignificantStagedPath(p));
  if (significant.length === 0) return [];
  const text = bullets.join("\n").toLowerCase();
  const missed: string[] = [];
  const matched = new Set<string>();
  for (const area of STAGED_CHANGE_AREAS) {
    const files = significant.filter((p) => {
      if (!area.match(p)) return false;
      matched.add(p);
      return true;
    });
    if (files.length === 0) continue;
    if (!bulletsCoverArea(text, files, area.keywords)) missed.push(area.id);
  }
  const other = significant.filter((p) => !matched.has(p));
  if (other.length > 0 && !bulletsMentionPathTokens(text, other)) missed.push("other staged paths");
  return missed;
}

export function validateBulletsAgainstStaged(bullets: string[], staged: string[]): void {
  const missed = uncoveredStagedAreas(bullets, staged);
  if (missed.length === 0) return;
  sayCommitV1("bullets.must-cover-every-staged-area",{missing:missed.join(", ")},"error");
  sayCommitV1("bullets.read-diff-again",{},"error");
  for (const p of staged) console.error(`  ${p}`);
  process.exit(1);
}

export function readDiffBulletsInput(root: string, bulletsFile: string | null): string[] {
  if (bulletsFile) {
    const path = bulletsFile.startsWith("/") ? bulletsFile : join(root, bulletsFile);
    return normalizeBulletLines(readFileSync(path, "utf8"));
  }
  if (!process.stdin.isTTY) {
    try {
      return normalizeBulletLines(readFileSync(0, "utf8"));
    } catch {
      // ignore errors and fall through to empty
    }
  }
  return [];
}

export function listCachedPaths(root: string): string[] {
  return gitCachedNames(root);
}

export function listAddedTicketPaths(root: string): string[] {
  return gitCachedNames(root, ["--diff-filter=A"]).filter((p) => TICKET_JSON_RE.test(p));
}

export function ticketBullets(root: string): string[] {
  const bullets: string[] = [];
  for (const rel of listAddedTicketPaths(root)) {
    const path = join(root, rel);
    try {
      const t = JSON.parse(readFileSync(path, "utf8")) as { emoji?: string; title?: string };
      if (!t.emoji || !t.title) continue;
      bullets.push(`${t.emoji}${t.title}`);
    } catch {
      /* ignore invalid JSON ticket files */
    }
  }
  return bullets;
}
