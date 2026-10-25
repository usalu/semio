import type { TaxonomyAudit } from "../🪣️taxonomy-audit-context/🟦️.ts";

/** 📎️ Audits the physical leaf rendering, the reference closure and the implementation leaf policy declarations. */
export function auditLeafAndReferencePolicies(audit: TaxonomyAudit): void {
  const { taxonomy, problems, record, ids, pathPattern } = audit;
  if (record(taxonomy.physicalLeafRendering, "physicalLeafRendering")) {
    const expected = { direction: "forward-only", filename: "file-kind-emoji-and-extension-chain", sourceExtension: "longest-registered-chain", authoringExtension: "schema-ordered-primary", runtimeLookup: "canonical-only" };
    if (Object.keys(taxonomy.physicalLeafRendering).sort().join("\0") !== Object.keys(expected).sort().join("\0") || Object.entries(expected).some(([key, value]) => (taxonomy.physicalLeafRendering as unknown as Record<string, unknown>)[key] !== value)) problems.push("physicalLeafRendering must declare exact forward-only kind-only rendering, longest source extension, primary authoring extension, and canonical-only runtime lookup.");
  }
  if (record(taxonomy.referenceClosure, "referenceClosure")) {
    const expected = { scope: "repository-incoming-and-moved-outgoing", candidateSource: "git-tracked-and-untracked-plus-explicit-ticket", candidateAdmission: "opaque-first-no-follow", coordinateRoots: "verified-repository-ownership", unsupportedPathBearingForms: "error", frozenSourceCoordinates: "exact-digest-and-token-authority", frozenPlanCoordinates: "canonical-schema-v2-digest-and-typed-token-authority", preimageDrift: "reject", newIncomingReferences: "reject-or-rollback", ordering: "utf8-byte", historicalDocumentEvidence: "ticket-report-workspace-and-dev-prompt-log-whole-document-excluded" };
    if (Object.keys(taxonomy.referenceClosure).sort().join("\0") !== Object.keys(expected).sort().join("\0") || Object.entries(expected).some(([key, value]) => (taxonomy.referenceClosure as unknown as Record<string, unknown>)[key] !== value)) problems.push("referenceClosure must declare repository-wide incoming and moved outgoing closure with opaque-first no-follow candidates, exact frozen coordinate authority, drift rejection, and byte ordering.");
  }
  if (record(taxonomy.implementationLeafPolicy, "implementationLeafPolicy")) {
    const policy = taxonomy.implementationLeafPolicy;
    const ignoredPathPatterns = ["**/.cache", "**/.git", "**/.lake", "**/.next", "**/.nx", "**/.pytest_cache", "**/.venv", "**/.🧬semio", "**/__pycache__", "**/coverage", "**/dist", "**/node_modules", "**/storybook-static", "**/target", "**/📤️dist", "**/🗑️generated", "**/🔌️plugin-modules", "**/🕸️bindings", "**/📦️packages/🦀️rust/pkg", "**/📦️packages/🟦️typescript/out", "**/📦️packages/🔷️dotnet/obj", "**/📦️packages/🔷️dotnet/bin", "**/🎤️präsentation/📅️33.projektetage/🌐️public", "**/🪞️preview2-shim"];
    if (Object.keys(policy).sort().join("\0") !== "fileKindIds\0ignoredPathPatterns\0roles" || policy.roles.join("\0") !== "source" || policy.fileKindIds.join("\0") !== "css\0html" || policy.ignoredPathPatterns.join("\0") !== ignoredPathPatterns.join("\0")) problems.push("implementationLeafPolicy must cover every source role plus exact CSS/HTML authored kinds and only the declared generated, cache and tool-state paths.");
    ids(policy.fileKindIds, taxonomy.fileKinds, "implementationLeafPolicy.fileKindIds");
    for (const [index, value] of policy.ignoredPathPatterns.entries()) pathPattern(value, `implementationLeafPolicy.ignoredPathPatterns[${index}]`);
  }
}
