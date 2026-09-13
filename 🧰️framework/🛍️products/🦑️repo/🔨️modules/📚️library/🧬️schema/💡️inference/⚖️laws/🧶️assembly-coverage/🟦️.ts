import type { BreachRecord } from "../../../../🟦️.ts";
import { POLICY_SOURCE_OPERATIONS, policySourceText, type PolicySourceOperations } from "../../../../🔍️discovery/📖️source-access/🟦️.ts";
import { POLICY_RS_COMPONENT_LEAF_NAME, policyStripEmoji } from "../../../../🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts";
import { policyExtractRustSchemaFields } from "../../../🔍️field-discovery/🦀️rust/🟦️.ts";
import type { PolicyInferenceFamilySource } from "../../🧱️contract/🟦️.ts";
import { policyInferenceSourceBreaches } from "../🚧️source-admission/🟦️.ts";

/** 🧶️ Normalizes inference identity across kebab, snake, camel, Pascal, and scalar forms. */
export function policyInferenceNormalizeToken(raw: string): string {
  return raw.toLowerCase().replace(/[^a-z0-9]/g, "");
}

/** 🪢 Requires a bidirectional match between inference slug directories and assembled Rust fields. */
export function policyInferenceAssemblyCoverageBreaches(repoRoot: string, families: readonly PolicyInferenceFamilySource[], operations: PolicySourceOperations = POLICY_SOURCE_OPERATIONS): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const family of families) {
    const rel = `${family.inferencesRel}/${POLICY_RS_COMPONENT_LEAF_NAME}`,
      source = policySourceText(repoRoot, rel, operations);
    if (source.state === "missing") continue;
    if (source.state !== "file") {
      breaches.push(...policyInferenceSourceBreaches([{ path: rel, state: source.state }]));
      continue;
    }
    const match = /\bpub\s+struct\s+(\w+Inference)\b/.exec(source.text);
    if (!match) continue;
    const structName = match[1]!,
      extract = policyExtractRustSchemaFields(source.text, structName);
    if (extract.typeName !== structName) continue;
    const slugs = new Map(family.slugs.map((slug) => [slug, policyInferenceNormalizeToken(policyStripEmoji(slug))]));
    const fields = extract.fields.map((field) => ({ field, name: policyInferenceNormalizeToken(field.name), scalar: policyInferenceNormalizeToken(field.scalar) }));
    for (const [slug, token] of slugs)
      if (!fields.some((field) => field.name === token || field.scalar.endsWith(token)))
        breaches.push({
          id: `inference-orphan-slug-${family.inferencesRel}/${slug}`,
          summary: `"${family.inferencesRel}/${slug}" has no matching field on ${structName}`,
          kind: "inference-migration/assembly-coverage",
          scope: family.artifactRel,
          priority: "medium",
          reason: "Every inference slug must be assembled into a matching family-root field.",
          solution: `Add a ${structName} field named or typed after ${policyStripEmoji(slug)}, or remove the stale slug.`,
        });
    for (const field of fields)
      if (![...slugs.values()].some((token) => field.name === token || field.scalar.endsWith(token)))
        breaches.push({
          id: `inference-uncovered-field-${family.inferencesRel}-${field.field.name}`,
          summary: `"${structName}.${field.field.name}" has no matching inference slug`,
          kind: "inference-migration/assembly-coverage",
          scope: family.artifactRel,
          priority: "medium",
          reason: "Every assembled inference field must be backed by a concrete slug directory.",
          solution: `Add or rename an inference slug to match ${field.field.name}.`,
        });
  }
  return breaches;
}
