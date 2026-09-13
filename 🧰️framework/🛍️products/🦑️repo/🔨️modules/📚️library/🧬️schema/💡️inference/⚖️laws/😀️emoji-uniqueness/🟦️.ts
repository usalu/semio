import type { BreachRecord } from "../../../../🟦️.ts";
import { policyLeadingEmojiPrefix } from "../../../../🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts";
import type { PolicyInferenceFamilySource } from "../../🧱️contract/🟦️.ts";

/** 😀️ Enforces bare and unique emoji prefixes within each inference family independently. */
export function policyInferenceEmojiUniquenessBreaches(families: readonly PolicyInferenceFamilySource[]): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const family of families) {
    const seen = new Map<string, string>();
    for (const slug of family.slugs) {
      const emoji = policyLeadingEmojiPrefix(slug);
      if (!emoji) {
        breaches.push({
          id: `inference-emoji-missing-${family.inferencesRel}/${slug}`,
          summary: `"${family.inferencesRel}/${slug}" has no leading emoji prefix`,
          kind: "inference-migration/emoji-uniqueness",
          scope: family.artifactRel,
          priority: "medium",
          reason: "Each inference slug needs an emoji prefix unique within its family.",
          solution: `Rename ${slug} with a leading emoji prefix.`,
        });
        continue;
      }
      if (emoji.includes("️"))
        breaches.push({
          id: `inference-emoji-vs16-${family.inferencesRel}/${slug}`,
          summary: `"${family.inferencesRel}/${slug}" carries U+FE0F on its inference emoji`,
          kind: "inference-migration/emoji-uniqueness",
          scope: family.artifactRel,
          priority: "low",
          reason: "Inference slug emoji prefixes use the established bare form.",
          solution: `Drop U+FE0F from the leading emoji in ${slug}.`,
        });
      const previous = seen.get(emoji);
      if (previous)
        breaches.push({
          id: `inference-emoji-dup-${family.artifactRel}-${emoji}-${slug}`,
          summary: `"${family.inferencesRel}/${slug}" reuses emoji "${emoji}" already used by "${previous}" within the same family`,
          kind: "inference-migration/emoji-uniqueness",
          scope: family.artifactRel,
          priority: "medium",
          reason: "Inference emoji identity is unique within a family; reuse across different families is valid.",
          solution: `Give ${slug} a different emoji than ${previous}.`,
        });
      else seen.set(emoji, slug);
    }
  }
  return breaches;
}
