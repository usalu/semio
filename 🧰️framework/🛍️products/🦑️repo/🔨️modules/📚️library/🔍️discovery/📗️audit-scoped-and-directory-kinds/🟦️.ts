import type { TaxonomyAudit } from "../🪣️taxonomy-audit-context/🟦️.ts";

/** 📗️ Audits the scoped file kinds and the semantic directory kind registry. */
export function auditScopedAndDirectoryKinds(audit: TaxonomyAudit): void {
  const { taxonomy, problems, canonicalEmoji, record, ids, pattern, fullPattern, pathPattern, extensionChain } = audit;
  if (!(typeof taxonomy.scopedFileKinds === "object" && taxonomy.scopedFileKinds !== null && !Array.isArray(taxonomy.scopedFileKinds))) problems.push("scopedFileKinds must be an object.");
  else for (const [id, spec] of Object.entries(taxonomy.scopedFileKinds)) {
    pathPattern(spec.pathPattern, `scopedFileKinds[${JSON.stringify(id)}].pathPattern`);
    if (spec.parentDirectoryKindId !== undefined && !taxonomy.semanticDirectoryKinds[spec.parentDirectoryKindId]) problems.push(`scopedFileKinds[${JSON.stringify(id)}].parentDirectoryKindId must reference a semantic directory kind.`);
    if (!canonicalEmoji(spec.emoji)) problems.push(`scopedFileKinds[${JSON.stringify(id)}].emoji must be one canonical NFC emoji sequence.`);
    if (!Array.isArray(spec.extensionChains) || spec.extensionChains.length === 0) problems.push(`scopedFileKinds[${JSON.stringify(id)}].extensionChains must be non-empty.`);
    for (const extension of spec.extensionChains ?? []) extensionChain(extension, `scopedFileKinds[${JSON.stringify(id)}].extensionChains`);
    if (!["source", "schema", "specification", "configuration", "documentation", "test", "asset", "generated", "marker", "evidence"].includes(spec.role)) problems.push(`scopedFileKinds[${JSON.stringify(id)}].role is invalid.`);
    fullPattern(spec.sourceFilenamePattern, `scopedFileKinds[${JSON.stringify(id)}].sourceFilenamePattern`);
    if (!spec.authority || !spec.reason || !spec.verification) problems.push(`scopedFileKinds[${JSON.stringify(id)}] must declare authority, reason, and verification.`);
    if (!(spec.expires === null || /^\d{4}-\d{2}-\d{2}$/u.test(spec.expires))) problems.push(`scopedFileKinds[${JSON.stringify(id)}].expires must be null or YYYY-MM-DD.`);
  }

  if (record(taxonomy.semanticDirectoryKinds, "semanticDirectoryKinds")) {
    const parentKinds = { ...taxonomy.semanticDirectoryKinds, ...taxonomy.semanticDirectoryMemberKinds, ...taxonomy.semanticProjectedMemberKinds, ...taxonomy.fixedDirectoryContracts };
    for (const [id, spec] of Object.entries(taxonomy.semanticDirectoryKinds)) {
      if (!canonicalEmoji(spec.emoji)) problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}].emoji must be one canonical NFC emoji sequence.`);
      if (typeof spec.slugPattern !== "string") problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}].slugPattern must be a string.`);
      else pattern(spec.slugPattern, `semanticDirectoryKinds[${JSON.stringify(id)}].slugPattern`);
      if (typeof spec.allowEmojiOnly !== "boolean") problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}].allowEmojiOnly must be boolean.`);
      if (spec.inferWithoutEmoji !== undefined && typeof spec.inferWithoutEmoji !== "boolean") problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}].inferWithoutEmoji must be boolean when present.`);
      if (spec.projectionOnly !== undefined && typeof spec.projectionOnly !== "boolean") problems.push(`semanticDirectoryKinds[${JSON.stringify(id)}].projectionOnly must be boolean when present.`);
      ids(spec.parentKindIds ?? [], parentKinds, `semanticDirectoryKinds[${JSON.stringify(id)}].parentKindIds`);
    }
  }
}
