import { BUNDLE_DATE_SECTION_RE, BUNDLE_WIP_SUBJECT_RE, type CommitBundleDateSection, type CommitBundleSection } from "../🧾️bundle-model/🟦️.ts";
import { bulletEmojiValidationError, MICRO_COMMIT_BULLET_RE } from "../🔸️bullets/🟦️.ts";
import { normalizeBundleDateLine } from "../📅️dates/🟦️.ts";
import { bundleScopeLabelError, isBundleScopeLine, normalizeBundleScopeLabel } from "../🏷️labels/🟦️.ts";

/** 🚫️Validates stdin is bundle body only, not a full commit message. */
export function commitBundleBodyError(text: string): string | null {
  const first =
    text
      .trim()
      .split("\n")
      .find((l) => l.trim().length > 0)
      ?.trim() ?? "";
  if (BUNDLE_WIP_SUBJECT_RE.test(first)) {
    return "commit: stdin must not include the bundle subject (…🔀️) — script adds it";
  }
  if (/^🐙️|^🧑️/.test(first) && first.includes("🚩️")) {
    return "commit: stdin must not include micro-commit subject lines";
  }
  if (/^📊️metric/m.test(text)) {
    return "commit: stdin must not include the 📊️metric footer — script adds it";
  }
  if (/^🎆️\d{2}🌙️\d{2}☀️\d{2}📊️metric/m.test(text)) {
    return "commit: stdin must not include per-day 📊️metric — script adds it to each 🎆️ line";
  }
  return null;
}

function validateBulletLine(b: string): void {
  if (!MICRO_COMMIT_BULLET_RE.test(b)) {
    throw new Error(`commit: bullet must start with {emoji} then description (no space after emoji): ${b}`);
  }
  const err = bulletEmojiValidationError([b]);
  if (err) throw new Error(err.replace(/^micro-commit:/, "commit:"));
  if (BUNDLE_DATE_SECTION_RE.test(b.trim())) {
    throw new Error("commit: use a date section line `🎆️YY🌙️MM☀️DD` on its own, not as a bullet");
  }
}

/** 📦️Parses LLM bundle body (emoji-only scope lines, dates, bullets). */
export function parseCommitBundleBody(text: string): CommitBundleSection[] {
  const bundles: CommitBundleSection[] = [];
  let current: CommitBundleSection | null = null;
  let dateSection: CommitBundleDateSection | null = null;

  for (const raw of text.split("\n")) {
    const line = raw.trim();
    if (!line || line.startsWith("#")) continue;
    if (line.startsWith("📊️metric") || line.startsWith("Signed-off-by:")) continue;

    const dateCandidate = normalizeBundleDateLine(line);
    if (BUNDLE_DATE_SECTION_RE.test(dateCandidate)) {
      if (!current) throw new Error(`commit: date section before bundle scope: ${line}`);
      if (dateSection) current.dates.push(dateSection);
      dateSection = { dateLine: dateCandidate, bullets: [] };
      continue;
    }

    if (dateSection) {
      if (isBundleScopeLine(line)) {
        const scopeErr = bundleScopeLabelError(line);
        if (scopeErr) throw new Error(scopeErr);
        if (current) {
          current.dates.push(dateSection);
          bundles.push(current);
        }
        current = { label: normalizeBundleScopeLabel(line), dates: [] };
        dateSection = null;
        continue;
      }
      validateBulletLine(line);
      dateSection.bullets.push(line);
      continue;
    }

    if (isBundleScopeLine(line)) {
      const scopeErr = bundleScopeLabelError(line);
      if (scopeErr) throw new Error(scopeErr);
      if (current) bundles.push(current);
      current = { label: normalizeBundleScopeLabel(line), dates: [] };
      continue;
    }

    if (!current) throw new Error(`commit: expected emoji bundle scope (two+ emojis, no paths), got: ${line}`);
    throw new Error(`commit: expected 🎆️YY🌙️MM☀️DD date section before bullet: ${line}`);
  }

  if (current) {
    if (dateSection) current.dates.push(dateSection);
    bundles.push(current);
  }
  if (bundles.length === 0) throw new Error("commit: at least one emoji bundle scope is required");
  for (const b of bundles) {
    if (b.dates.length === 0) throw new Error(`commit: bundle ${b.label} needs at least one date section`);
    for (const d of b.dates) {
      if (d.bullets.length === 0) throw new Error(`commit: ${d.dateLine} in ${b.label} needs at least one bullet`);
    }
  }
  return bundles;
}
