import type { TaxonomyAudit } from "../🪣️taxonomy-audit-context/🟦️.ts";

/** 📔️ Audits the file kind registry, its resolution rules and the single owner of every extension chain. */
export function auditFileKinds(audit: TaxonomyAudit): void {
  const { taxonomy, problems, canonicalEmoji, record, extensionChain } = audit;
  if (record(taxonomy.fileKinds, "fileKinds")) {
    const canonical = new Map<string, string>();
    const extensionOwners = new Map<string, string>();
    for (const [id, spec] of Object.entries(taxonomy.fileKinds)) {
      if (!canonicalEmoji(spec.emoji)) problems.push(`fileKinds[${JSON.stringify(id)}].emoji must be one canonical NFC emoji sequence.`);
      if (!Array.isArray(spec.extensionChains) || spec.extensionChains.length === 0) problems.push(`fileKinds[${JSON.stringify(id)}].extensionChains must be non-empty.`);
      for (const extension of spec.extensionChains ?? []) {
        if (!/^\.[a-z0-9]+(?:[.-][a-z0-9]+)*$/u.test(extension)) problems.push(`fileKinds[${JSON.stringify(id)}] has invalid extension chain ${JSON.stringify(extension)}.`);
        const extensionOwner = extensionOwners.get(extension);
        if (extensionOwner) problems.push(`fileKinds ${JSON.stringify(extensionOwner)} and ${JSON.stringify(id)} both own physical extension ${JSON.stringify(extension)}.`);
        extensionOwners.set(extension, id);
        const filename = `${spec.emoji}${extension}`;
        const prior = canonical.get(filename);
        if (prior) problems.push(`fileKinds ${JSON.stringify(prior)} and ${JSON.stringify(id)} collide at ${JSON.stringify(filename)}.`);
        canonical.set(filename, id);
      }
    }
    for (const [id, extension] of [["png", ".png"], ["bmp", ".bmp"]] as const) {
      const kind = taxonomy.fileKinds[id];
      if (!kind || kind.role !== "asset" || kind.emoji !== "🖼️" || kind.extensionChains.join("\0") !== extension) problems.push(`fileKinds.${id} must be the canonical 🖼️${extension} asset kind.`);
    }
  }

  if (record(taxonomy.fileKindResolutionRules, "fileKindResolutionRules")) for (const [id, rule] of Object.entries(taxonomy.fileKindResolutionRules)) {
    extensionChain(rule.extensionChain, `fileKindResolutionRules[${JSON.stringify(id)}].extensionChain`);
    const kind = taxonomy.fileKinds[rule.fileKindId];
    if (!kind || !kind.extensionChains.includes(rule.extensionChain)) problems.push(`fileKindResolutionRules[${JSON.stringify(id)}] must reference a kind owning its extension chain.`);
    if (rule.priority !== 0) problems.push(`fileKindResolutionRules[${JSON.stringify(id)}].priority must be zero for physical resolution.`);
    for (const removed of ["filenamePattern", "pathPattern", "parentKindIds", "ancestorKindIds"]) if (removed in rule) problems.push(`fileKindResolutionRules[${JSON.stringify(id)}].${removed} is forbidden; directories own semantics.`);
  }
  for (const [fileKindId, kind] of Object.entries(taxonomy.fileKinds)) for (const extension of kind.extensionChains) {
    const rules = Object.values(taxonomy.fileKindResolutionRules).filter((rule) => rule.extensionChain === extension && rule.fileKindId === fileKindId);
    if (rules.length !== 1) problems.push(`fileKindResolutionRules must own ${JSON.stringify(extension)} exactly once for ${JSON.stringify(fileKindId)}.`);
  }
}
