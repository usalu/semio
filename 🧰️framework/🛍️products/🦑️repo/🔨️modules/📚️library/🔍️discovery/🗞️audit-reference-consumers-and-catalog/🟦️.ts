import type { SemanticPathProjectionReferenceConsumerForm } from "../📜️taxonomy-contracts/🟦️.ts";
import type { TaxonomyAudit } from "../🪣️taxonomy-audit-context/🟦️.ts";

/** 🗞️ Audits the path projection reference consumer contracts and the mutation catalog projection with its canonical descendant contracts. */
export function auditReferenceConsumersAndMutationCatalog(audit: TaxonomyAudit): void {
  const { taxonomy, problems, record, exactKeys, kebabId } = audit;
  if (record(taxonomy.semanticPathProjectionReferenceConsumerContracts, "semanticPathProjectionReferenceConsumerContracts")) {
    const identities = new Set<string>();
    const patterns = new Set<string>();
    const adapters = ["rust", "typescript", "json", "toml"];
    const forms: readonly SemanticPathProjectionReferenceConsumerForm[] = ["path-reference", "artifact-catalog-glob", "artifact-catalog-prose:root-marker", "artifact-catalog-prose:relative-root", "artifact-catalog-prose:category-glob", "artifact-catalog-prose:catalog-grammar"];
    for (const [id, contract] of Object.entries(taxonomy.semanticPathProjectionReferenceConsumerContracts)) {
      const scope = `semanticPathProjectionReferenceConsumerContracts[${JSON.stringify(id)}]`;
      kebabId(id, `${scope} id`);
      exactKeys(contract, ["projectionContractId", "consumerIdentity", "ownership", "sourcePathPattern", "sourcePathIdentities", "adapters", "supportedForms", "staleMarkers"], scope);
      const projection = taxonomy.semanticPathProjectionContracts[contract.projectionContractId];
      if (!projection || !["artifact-example-model-catalog-projection-v1", "artifact-editor-command-projection-v1"].includes(projection.rationaleRule)) problems.push(`${scope}.projectionContractId must reference one CAD or Draw artifact projection.`);
      if (contract.consumerIdentity !== id || identities.has(contract.consumerIdentity)) problems.push(`${scope}.consumerIdentity must equal its unique registry id.`);
      identities.add(contract.consumerIdentity);
      if (contract.ownership !== "external") problems.push(`${scope}.ownership must be external.`);
      if (!contract.sourcePathPattern.startsWith("^") || !contract.sourcePathPattern.endsWith("$") || contract.sourcePathPattern !== contract.sourcePathPattern.normalize("NFC") || /\uFE0E/u.test(contract.sourcePathPattern)) problems.push(`${scope}.sourcePathPattern must be one anchored NFC regex without VS15.`);
      try {
        const expression = new RegExp(contract.sourcePathPattern, "u");
        if (!Array.isArray(contract.sourcePathIdentities) || contract.sourcePathIdentities.length === 0 || contract.sourcePathIdentities.some((path) => !path || path !== path.normalize("NFC") || /\uFE0E/u.test(path) || !expression.test(path)) || new Set(contract.sourcePathIdentities).size !== contract.sourcePathIdentities.length) problems.push(`${scope}.sourcePathIdentities must be unique exact NFC paths admitted by sourcePathPattern.`);
      } catch {
        problems.push(`${scope}.sourcePathPattern must compile with Unicode semantics.`);
      }
      const patternKey = `${contract.projectionContractId}\0${contract.sourcePathPattern}`;
      if (patterns.has(patternKey)) problems.push(`${scope}.sourcePathPattern is duplicated for its projection.`);
      patterns.add(patternKey);
      if (!Array.isArray(contract.adapters) || contract.adapters.length === 0 || contract.adapters.some((adapter) => !adapters.includes(adapter)) || new Set(contract.adapters).size !== contract.adapters.length) problems.push(`${scope}.adapters must be unique supported reference adapters.`);
      if (!Array.isArray(contract.supportedForms) || contract.supportedForms.length === 0 || contract.supportedForms.some((form) => !forms.includes(form)) || new Set(contract.supportedForms).size !== contract.supportedForms.length) problems.push(`${scope}.supportedForms must be unique supported structural forms.`);
      if (!Array.isArray(contract.staleMarkers) || contract.staleMarkers.length === 0 || contract.staleMarkers.some((marker) => !marker || marker !== marker.normalize("NFC") || /\uFE0E/u.test(marker)) || new Set(contract.staleMarkers).size !== contract.staleMarkers.length) problems.push(`${scope}.staleMarkers must be unique non-empty NFC markers without VS15.`);
      if (contract.supportedForms.some((form) => form.startsWith("artifact-catalog-")) && (projection?.rationaleRule !== "artifact-example-model-catalog-projection-v1" || !contract.adapters.some((adapter) => adapter === "rust" || adapter === "typescript"))) problems.push(`${scope} artifact-catalog forms require the CAD projection and a Rust or TypeScript adapter.`);
    }
    const required = ["cad-editor-interaction", "cad-editor-runtime", "cad-interaction-spec", "cad-spatial-kernel-geometry", "draw-dependency-registry", "draw-package-cargo", "draw-package-library", "draw-workspace-cargo", "draw-workspace-script"];
    if ([...identities].sort().join("\0") !== required.join("\0")) problems.push("semanticPathProjectionReferenceConsumerContracts must encode the nine exact current external CAD/Draw consumers.");
    const rows = Object.values(taxonomy.semanticPathProjectionReferenceConsumerContracts);
    for (const [index, left] of rows.entries()) for (const right of rows.slice(index + 1)) {
      if (left.projectionContractId !== right.projectionContractId || !left.supportedForms.some((form) => right.supportedForms.includes(form))) continue;
      const leftPattern = new RegExp(left.sourcePathPattern, "u");
      const rightPattern = new RegExp(right.sourcePathPattern, "u");
      if (left.sourcePathIdentities.some((path) => rightPattern.test(path)) || right.sourcePathIdentities.some((path) => leftPattern.test(path))) problems.push(`semanticPathProjectionReferenceConsumerContracts ${JSON.stringify(left.consumerIdentity)} and ${JSON.stringify(right.consumerIdentity)} overlap for one supported form.`);
    }
  }

  const projectionIds = taxonomy.mutationCatalogProjection;
  if (!projectionIds || typeof projectionIds !== "object") problems.push("mutationCatalogProjection must be an object.");
  else {
    exactKeys(projectionIds, ["contractKind", "contractId", "sourceOwnerKindId", "projectedMemberKindId", "implementationSegments", "fixtureSegments", "implementationDescendantContractId", "fixtureDescendantContractId", "catalogContractId", "coverage", "fixtureCoverage"], "mutationCatalogProjection");
    const projected = taxonomy.semanticProjectedMemberKinds[projectionIds.projectedMemberKindId];
    if (projectionIds.contractKind !== "canonical-mutation-case-pair" || projectionIds.contractId !== "canonical-mutation-case-pair-v1" || projectionIds.coverage !== "every-catalog-vector-has-one-fixture-bundle-with-declared-coverage") problems.push("mutationCatalogProjection must declare the canonical pair identity and closed coverage.");
    if (!projected || projected.projectionContractId !== projectionIds.contractId) problems.push("mutationCatalogProjection projected member and pair IDs do not agree.");
    const coverage = projectionIds.fixtureCoverage;
    if (!coverage || typeof coverage !== "object") problems.push("mutationCatalogProjection.fixtureCoverage must be an object.");
    else {
      exactKeys(coverage, ["implementationCase", "exhaustiveCase", "fixtureRunner", "leafCaseTable"], "mutationCatalogProjection.fixtureCoverage");
      exactKeys(coverage.exhaustiveCase, ["featureFileKindId", "stepUriPrefix", "bundleFacetDirectoryNames"], "mutationCatalogProjection.fixtureCoverage.exhaustiveCase");
      exactKeys(coverage.fixtureRunner, ["relativePath"], "mutationCatalogProjection.fixtureCoverage.fixtureRunner");
      exactKeys(coverage.leafCaseTable, ["fileKindId", "casesField"], "mutationCatalogProjection.fixtureCoverage.leafCaseTable");
      if (!taxonomy.fileKinds[coverage.leafCaseTable?.fileKindId] || typeof coverage.leafCaseTable?.casesField !== "string" || coverage.leafCaseTable.casesField === "") problems.push("mutationCatalogProjection.fixtureCoverage.leafCaseTable must name a registered file kind and the member that lists the cases.");
      if (coverage.implementationCase !== projectionIds.implementationDescendantContractId) problems.push("mutationCatalogProjection.fixtureCoverage.implementationCase must be the implementation descendant contract.");
      if (!taxonomy.fileKinds[coverage.exhaustiveCase?.featureFileKindId]) problems.push("mutationCatalogProjection.fixtureCoverage.exhaustiveCase.featureFileKindId is missing.");
      if (!String(coverage.exhaustiveCase?.stepUriPrefix).endsWith("/") || !Array.isArray(coverage.exhaustiveCase?.bundleFacetDirectoryNames) || coverage.exhaustiveCase.bundleFacetDirectoryNames.length === 0) problems.push("mutationCatalogProjection.fixtureCoverage.exhaustiveCase must name a slash-terminated step URI prefix and its bundle facets.");
      if (typeof coverage.fixtureRunner?.relativePath !== "string" || coverage.fixtureRunner.relativePath === "" || coverage.fixtureRunner.relativePath.startsWith("/") || coverage.fixtureRunner.relativePath.includes("..")) problems.push("mutationCatalogProjection.fixtureCoverage.fixtureRunner.relativePath must be a relative path.");
    }
    if (!taxonomy.semanticDirectoryMemberKinds[projectionIds.sourceOwnerKindId] || !taxonomy.semanticDescendantContracts[projectionIds.implementationDescendantContractId] || !taxonomy.semanticDescendantContracts[projectionIds.fixtureDescendantContractId] || !taxonomy.semanticPathProjectionCatalogContracts[projectionIds.catalogContractId]) problems.push("mutationCatalogProjection contract IDs do not agree with its pair registries.");
  }
  const exactImplementation = [{ kindId: "standards", literal: "🏅️standards" }, { kindId: "standard", capture: "standardVersion" }, { kindId: "subsets", literal: "🪆️subsets" }, { kindId: "subset", capture: "subsetId" }, { kindId: "schema", literal: "🧬️schema" }, { kindId: "schema", literal: "🧬️mutations" }, { projectedMemberKindId: "mutation-test-subject", capture: "mutationId" }, { kindId: "tests", literal: "🧪️tests" }, { kindId: "test-case", capture: "scenarioId" }];
  const exactFixture = [{ kindId: "standards", literal: "🏅️standards" }, { kindId: "standard", capture: "standardVersion" }, { kindId: "subsets", literal: "🪆️subsets" }, { kindId: "subset", capture: "subsetId" }, { kindId: "tests", literal: "🧪️tests" }, { kindId: "fixtures", literal: "🧫️fixtures" }, { kindId: "schema", literal: "🧬️mutations" }, { projectedMemberKindId: "mutation-test-subject", capture: "mutationId" }, { kindId: "test-case", capture: "scenarioId" }];
  if (JSON.stringify(projectionIds?.implementationSegments) !== JSON.stringify(exactImplementation) || JSON.stringify(projectionIds?.fixtureSegments) !== JSON.stringify(exactFixture)) problems.push("mutationCatalogProjection must encode the exact canonical implementation and fixture root grammars.");
  const implementation = taxonomy.semanticDescendantContracts["mutation-implementation-case-v1"], fixture = taxonomy.semanticDescendantContracts["mutation-fixture-bundle-v1"];
  if (!implementation || "contractKind" in implementation || implementation.realizedNodeCount !== 2 || implementation.exclusiveAlternatives.length !== 0 || implementation.pathBudgetReserve.utf16Units !== 7) problems.push("semanticDescendantContracts.mutation-implementation-case-v1 must encode two nodes and the derived 7-unit reserve.");
  if (!fixture || "contractKind" in fixture || fixture.realizedNodeCount !== 12 || fixture.exclusiveAlternatives.length !== 1 || fixture.exclusiveAlternatives[0]?.id !== "diff-leaf" || fixture.pathBudgetReserve.utf16Units !== 30) problems.push("semanticDescendantContracts.mutation-fixture-bundle-v1 must encode 12 nodes, one diff alternative, and the derived 30-unit reserve.");
}
