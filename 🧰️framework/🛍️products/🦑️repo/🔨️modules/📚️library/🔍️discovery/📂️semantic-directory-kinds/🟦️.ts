import { leadingEmojiIdentity } from "../../../../../../📦️packages/🟦️typescript/🪪️identity/🧩️grapheme/🟦️.ts";
import { type SemanticDirectoryMemberKindSpec, canonicalTaxonomyEmoji } from "../📜️taxonomy-contracts/🟦️.ts";
import type { Taxonomy } from "../📑️taxonomy-document/🟦️.ts";

/** 🧭️ Structural ancestry supplied to owner-local exact semantic member resolution. */
export interface SemanticDirectoryMatchContext {
  readonly parentKindId?: string;
  readonly ancestorKindIds?: readonly string[];
}

export function canonicalSemanticDirectoryName(name: string, taxonomy: Taxonomy): string {
  const normalized = name.normalize(taxonomy.unicodeNormalization.form);
  const leading = leadingEmojiIdentity(normalized).first;
  if (!leading) return normalized;
  if (/^\p{Extended_Pictographic}$/u.test(leading)) return `${leading}\uFE0F${normalized.slice(leading.length)}`;
  return normalized;
}

export const semanticDirectoryKindIds = new WeakMap<Taxonomy, Map<string, string | null>>();

const memberSlugRegexes = new Map<string, RegExp>();

const memberPatternEmojiVerdicts = new Map<string, boolean>();

function memberSlugRegex(pattern: string): RegExp {
  let regex = memberSlugRegexes.get(pattern);
  if (!regex) memberSlugRegexes.set(pattern, regex = new RegExp(pattern, "u"));
  return regex;
}

/** 🧷️ Whether one leading emoji grapheme is an admitted RGI emoji (a lone pictograph counts with its explicit presentation selector), shared by every member slug pattern matcher. */
export function semanticMemberPatternEmojiAdmitted(emoji: string): boolean {
  let verdict = memberPatternEmojiVerdicts.get(emoji);
  if (verdict === undefined) memberPatternEmojiVerdicts.set(emoji, verdict = emoji.length > 0 && canonicalTaxonomyEmoji(/^\p{Extended_Pictographic}$/u.test(emoji) ? `${emoji}\uFE0F` : emoji));
  return verdict;
}

/** 🔏️ Whether one member row admits a canonical directory name exactly or through its slug pattern. */
export function semanticDirectoryMemberAdmitsName(spec: SemanticDirectoryMemberKindSpec, name: string): boolean {
  if (spec.memberNames.includes(name)) return true;
  if (spec.memberSlugPattern === undefined) return false;
  const identity = leadingEmojiIdentity(name.normalize("NFC"));
  return identity.first.length > 0 && semanticMemberPatternEmojiAdmitted(identity.first) && memberSlugRegex(spec.memberSlugPattern).test(identity.rest);
}

export function resolveSemanticDirectoryKindId(name: string, taxonomy: Taxonomy, context: SemanticDirectoryMatchContext): string | null {
  const normalized = canonicalSemanticDirectoryName(name, taxonomy);
  const identity = leadingEmojiIdentity(normalized);
  const roleName = identity.emoji ? `${identity.first}${identity.rest}` : normalized;
  const parentKindId = context.parentKindId;
  const matches = Object.entries(taxonomy.semanticDirectoryKinds).filter(([, spec]) => {
    if (spec.parentKindIds && !parentKindId) return false;
    if (spec.parentKindIds && !spec.parentKindIds.includes(parentKindId!)) return false;
    if (!roleName.startsWith(spec.emoji)) return false;
    const slug = roleName.slice(spec.emoji.length);
    return (slug.length === 0 && spec.allowEmojiOnly) || (slug.length > 0 && new RegExp(spec.slugPattern, "u").test(slug));
  });
  const contextual = parentKindId ? matches.filter(([, spec]) => spec.parentKindIds?.includes(parentKindId)) : [];
  const resolved = contextual.length > 0 ? contextual : matches.filter(([, spec]) => !spec.parentKindIds);
  if (resolved.length === 1) return resolved[0]![0];
  if (resolved.length > 1) return null;
  const owners = [context.parentKindId, ...(context.ancestorKindIds ?? [])].filter((id): id is string => Boolean(id));
  for (const ownerKindId of owners) {
    const memberMatches = Object.entries(taxonomy.semanticDirectoryMemberKinds).filter(([, spec]) => spec.ownerKindIds.includes(ownerKindId) && spec.memberNames.includes(roleName));
    if (memberMatches.length > 0) return memberMatches.length === 1 ? memberMatches[0]![0] : null;
  }
  if (!identity.emoji || !semanticMemberPatternEmojiAdmitted(identity.first)) return null;
  for (const ownerKindId of owners) {
    const patternMatches = Object.entries(taxonomy.semanticDirectoryMemberKinds).filter(([, spec]) => spec.memberSlugPattern !== undefined && spec.ownerKindIds.includes(ownerKindId) && memberSlugRegex(spec.memberSlugPattern).test(identity.rest));
    if (patternMatches.length > 0) return patternMatches.length === 1 ? patternMatches[0]![0] : null;
  }
  return null;
}
