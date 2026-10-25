import type { Contributor } from "../👤️contributor/🟦️.ts";
import { normalizeEmojiPresentation } from "../🔣️emoji-presentation/🟦️.ts";
import { git } from "../🌳️git/🟦️.ts";
import {refuseCommitV1} from "../📖️texts/🟦️.ts";

const COUNTER_RE = /^(.+🎆️[\uFE0E\uFE0F]?\d{2}🌙️[\uFE0E\uFE0F]?\d{2}☀️[\uFE0E\uFE0F]?\d{2})🚩️[\uFE0E\uFE0F]?(\d+)$/u;

const BUNDLE_TAG_RE = /^(.+🎆️[\uFE0E\uFE0F]?\d{2}🌙️[\uFE0E\uFE0F]?\d{2}☀️[\uFE0E\uFE0F]?\d{2})🚩️[\uFE0E\uFE0F]?$/u;

const NUMERIC_COUNTER_RE = /^(\d+)$/;

export function pad2(n: number): string {
  return String(n).padStart(2, "0");
}

function pad3(n: number): string {
  return String(n).padStart(3, "0");
}

const COUNTER_LOG_DEPTH = 40;

/** 🧭️Renders a historical WIP epoch with the contributor's current canonical emoji presentation. */
export function canonicalWipLine1Base(line1Base: string, contributor: Contributor): string {
  const match = /🎆️(\d{2})🌙️(\d{2})☀️(\d{2})$/u.exec(normalizeEmojiPresentation(line1Base));
  if (!match) return line1Base;
  return `${contributor.emoji}${contributor.alias}🎆️${match[1]}🌙️${match[2]}☀️${match[3]}`;
}

/** ⛏️Reads micro-commit counter from subject line `…🚩️NNN`. */
export function extractCounterFromSubject(subject: string): { nnn: number; line1Base: string } | null {
  const s = subject.trim();
  const formatted = COUNTER_RE.exec(s);
  if (!formatted) return null;
  const [, line1Base, counter] = formatted; if (line1Base === undefined || counter === undefined) return null;
  return { nnn: Number.parseInt(counter, 10), line1Base };
}

/** 🔢️Reads GitKraken numeric-only subjects (`152`, `299`, …). */
export function extractNumericCounterFromSubject(subject: string): number | null {
  const m = NUMERIC_COUNTER_RE.exec(subject.trim());
  if (!m) return null;
  const n = Number.parseInt(m[1]!, 10);
  return Number.isFinite(n) && n > 0 ? n : null;
}

/** 🏷️Reads WIP epoch base from a bundle squash tag (`…🎆️YY🌙️MM☀️DD🚩️`). */
export function line1BaseFromBundleTag(tag: string): string | null {
  const m = BUNDLE_TAG_RE.exec(tag.trim());
  return m ? m[1]! : null;
}

/** 🎆️Resolves the persisted WIP epoch for line 1 from formatted history or the latest bundle tag. */
export function contributorWipLine1Base(root: string, contributor: Contributor): string | null {
  const prefix = `${contributor.emoji}${contributor.alias}`;
  const normalizedPrefix = normalizeEmojiPresentation(prefix);
  const log = git(root, ["log", "--format=%s", "-1000"]).out;
  for (const subject of log ? log.split("\n") : []) {
    const hit = extractCounterFromSubject(subject);
    if (hit && normalizeEmojiPresentation(hit.line1Base).startsWith(normalizedPrefix)) return hit.line1Base;
  }
  const tags = git(root, ["tag", "-l", "--sort=-creatordate"]).out;
  for (const tag of tags ? tags.split("\n") : []) {
    const base = line1BaseFromBundleTag(tag);
    if (base && normalizeEmojiPresentation(base).startsWith(normalizedPrefix)) return base;
  }
  return null;
}

/** 🕰️Bumps counter from recent `…🚩️NNN` or numeric GitKraken subjects (newest first). */
export function bumpCounterFromHistory(subjectsNewestFirst: string[], contributor: Contributor, now = new Date(), wipLine1Base: string | null = null): { line1Base: string; nnn: string } {
  const yy = pad2(now.getFullYear() % 100);
  const mm = pad2(now.getMonth() + 1);
  const dd = pad2(now.getDate());
  const fresh = `${contributor.emoji}${contributor.alias}🎆️${yy}🌙️${mm}☀️${dd}`;
  const contributorPrefix = normalizeEmojiPresentation(`${contributor.emoji}${contributor.alias}`);
  let max = 0;
  let line1Base: string | null = null;
  for (const subject of subjectsNewestFirst) {
    const hit = extractCounterFromSubject(subject);
    if (hit) {
      if (!normalizeEmojiPresentation(hit.line1Base).startsWith(contributorPrefix)) continue;
      max = Math.max(max, hit.nnn);
      if (!line1Base) line1Base = hit.line1Base;
      continue;
    }
    const numeric = extractNumericCounterFromSubject(subject);
    if (numeric !== null) max = Math.max(max, numeric);
  }
  const epoch = canonicalWipLine1Base(line1Base ?? wipLine1Base ?? fresh, contributor);
  if (max > 0) return { line1Base: epoch, nnn: pad3(max + 1) };
  const unparsedPrior = subjectsNewestFirst.find((subject) => {
    const normalized = normalizeEmojiPresentation(subject.trim());
    const epochPrefix = normalizeEmojiPresentation(`${contributor.emoji}${contributor.alias}🎆️`);
    return normalized.startsWith(epochPrefix) && /🚩\d+$/u.test(normalized) && !extractCounterFromSubject(subject);
  });
  if (unparsedPrior) throw refuseCommitV1("failed","counter.refusing-to-reset-to-because",{subject:String(unparsedPrior)});
  return { line1Base: epoch, nnn: "001" };
}

export function bumpCounterFromSubject(subject: string, contributor: Contributor, now = new Date()): { line1Base: string; nnn: string } {
  return bumpCounterFromHistory([subject], contributor, now);
}

export function nextCounter(root: string, contributor: Contributor): { line1Base: string; nnn: string } {
  const log = git(root, ["log", "--format=%s", `-${COUNTER_LOG_DEPTH}`]).out;
  const subjects = log ? log.split("\n").filter(Boolean) : [];
  return bumpCounterFromHistory(subjects, contributor, new Date(), contributorWipLine1Base(root, contributor));
}

export function formatSecond(now: Date): string {
  const yy = pad2(now.getFullYear() % 100);
  const mm = pad2(now.getMonth() + 1);
  const dd = pad2(now.getDate());
  const hh = pad2(now.getHours());
  const min = pad2(now.getMinutes());
  const ss = pad2(now.getSeconds());
  return `🎆️${yy}🌙️${mm}☀️${dd}⏰️${hh}⌚️${min}⏱️${ss}`;
}
