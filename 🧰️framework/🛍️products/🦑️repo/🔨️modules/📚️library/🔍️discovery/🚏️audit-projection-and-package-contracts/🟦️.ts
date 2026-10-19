import type { SemanticPathProjectionCatalogContract } from "../📜️taxonomy-contracts/🟦️.ts";
import { exampleAssetPath } from "../🎁️package-projection/🟦️.ts";
import type { TaxonomyAudit } from "../🪣️taxonomy-audit-context/🟦️.ts";

/** 🚏️ Audits the path projection catalog and projection contracts, the policy state coordinate contracts and the package projection contracts. */
export function auditProjectionAndPackageContracts(audit: TaxonomyAudit): void {
  const { taxonomy, problems, record, exactKeys, kebabId, semanticDirectoryKindId, artifactPathProjectionCatalogRoots } = audit;
  if (record(taxonomy.semanticPathProjectionCatalogContracts, "semanticPathProjectionCatalogContracts")) for (const [id, contract] of Object.entries(taxonomy.semanticPathProjectionCatalogContracts)) {
    kebabId(id, `semanticPathProjectionCatalogContracts id ${JSON.stringify(id)}`);
    if ("contractKind" in contract && contract.contractKind === "distributed-json-manifest-catalog") {
      exactKeys(contract, ["contractKind", "ownerArtifactMemberName", "profileVectors", "modelManifestSchema", "modelManifestSourceFilename", "modelIdentityField", "memberIdentityField", "memberVersionField", "requiredMemberVersion", "requiredModelManifest", "categoryRules", "coverage", "unknownCategoryPolicy", "unownedModelPolicy"], `semanticPathProjectionCatalogContracts[${JSON.stringify(id)}]`);
      if (!contract.ownerArtifactMemberName || !contract.modelManifestSchema || !contract.modelManifestSourceFilename || contract.modelIdentityField !== "id" || contract.memberIdentityField !== "id" || contract.memberVersionField !== "version" || !contract.requiredMemberVersion || contract.requiredModelManifest !== true) problems.push(`semanticPathProjectionCatalogContracts[${JSON.stringify(id)}] must declare exact non-empty manifest authority fields.`);
      if (!Array.isArray(contract.profileVectors) || contract.profileVectors.length === 0) problems.push(`semanticPathProjectionCatalogContracts[${JSON.stringify(id)}].profileVectors must be non-empty.`);
      const profiles = new Set<string>();
      for (const [index, vector] of (contract.profileVectors ?? []).entries()) {
        const scope = `semanticPathProjectionCatalogContracts[${JSON.stringify(id)}].profileVectors[${index}]`;
        exactKeys(vector, ["artifactId", "standardVersion", "subsetId"], scope);
        const key = [vector.artifactId, vector.standardVersion, vector.subsetId].join("\0");
        if (vector.artifactId !== contract.ownerArtifactMemberName || !vector.standardVersion || !vector.subsetId || profiles.has(key) || [vector.standardVersion, vector.subsetId].some((value) => value !== value.normalize("NFC") || /[\\/]/u.test(value))) problems.push(`${scope} must be one unique NFC owner profile tuple.`);
        profiles.add(key);
      }
      if (!Array.isArray(contract.categoryRules) || contract.categoryRules.length === 0) problems.push(`semanticPathProjectionCatalogContracts[${JSON.stringify(id)}].categoryRules must be non-empty.`);
      const categoryNames = new Set<string>();
      for (const [index, rule] of (contract.categoryRules ?? []).entries()) {
        const scope = `semanticPathProjectionCatalogContracts[${JSON.stringify(id)}].categoryRules[${index}]`;
        exactKeys(rule, rule.sourceShape === "direct-semantic-json" ? ["sourceDirectoryName", "directoryKindId", "sourceShape", "manifestSchema", "memberDirectoryEmoji"] : ["sourceDirectoryName", "directoryKindId", "sourceShape", "manifestSchema", "fixedSourceFilename"], scope);
        if (categoryNames.has(rule.sourceDirectoryName)) problems.push(`${scope}.sourceDirectoryName is duplicated.`);
        categoryNames.add(rule.sourceDirectoryName);
        if (!taxonomy.semanticDirectoryKinds[rule.directoryKindId]) problems.push(`${scope}.directoryKindId is missing.`);
        if (!rule.manifestSchema) problems.push(`${scope}.manifestSchema must be non-empty.`);
        if (rule.sourceShape === "direct-semantic-json") {
          if (!/^\p{Extended_Pictographic}\uFE0F$/u.test(rule.memberDirectoryEmoji)) problems.push(`${scope}.memberDirectoryEmoji must be one emoji plus U+FE0F.`);
        } else if (rule.sourceShape === "nested-fixed-json") {
          if (!rule.fixedSourceFilename) problems.push(`${scope}.fixedSourceFilename must be non-empty.`);
        } else problems.push(`${scope}.sourceShape is invalid.`);
      }
      if (contract.coverage !== "every-source-file-and-destination-node-exactly-once" || contract.unknownCategoryPolicy !== "problem" || contract.unownedModelPolicy !== "problem") problems.push(`semanticPathProjectionCatalogContracts[${JSON.stringify(id)}] must fail closed over exact source and destination ownership.`);
      continue;
    }
    if ("contractKind" in contract && contract.contractKind === "exact-owner-vectors") {
      exactKeys(contract, ["contractKind", "required", "allowEmpty", "identityFields", "coverage", "vectors"], `semanticPathProjectionCatalogContracts[${JSON.stringify(id)}]`);
      if (contract.required !== true || contract.allowEmpty !== false || contract.identityFields.join("\0") !== "artifactId\0standardVersion\0subsetId\0commandDirectoryName" || contract.coverage !== "every-physical-command-bundle-exactly-once" || !Array.isArray(contract.vectors) || contract.vectors.length === 0) problems.push(`semanticPathProjectionCatalogContracts[${JSON.stringify(id)}] must be the exact non-empty command owner-vector contract.`);
      const owners = new Set<string>();
      for (const [index, vector] of (contract.vectors ?? []).entries()) {
        exactKeys(vector, ["artifactId", "standardVersion", "subsetId", "commandDirectoryName"], `semanticPathProjectionCatalogContracts[${JSON.stringify(id)}].vectors[${index}]`);
        const owner = [vector.artifactId, vector.standardVersion, vector.subsetId, vector.commandDirectoryName].join("\0");
        if (!vector.artifactId || !vector.standardVersion || !vector.subsetId || !vector.commandDirectoryName || owners.has(owner)) problems.push(`semanticPathProjectionCatalogContracts[${JSON.stringify(id)}].vectors[${index}] must be one unique non-empty owner tuple.`);
        owners.add(owner);
      }
      continue;
    }
    exactKeys(contract, ["registryField", "required", "allowEmpty", "runtimeKindsField", "runtimeKindsRelation", "mutationIdField", "sourceMutationDirectoryNameField", "mutationDirectoryNameField", "scenariosField", "scenarioIdField", "scenarioDirectoryNameField", "sourceBundleUniquenessFields", "canonicalBundleUniquenessFields", "coverage"], `semanticPathProjectionCatalogContracts[${JSON.stringify(id)}]`);
    const expected: SemanticPathProjectionCatalogContract = { registryField: "vectors", required: true, allowEmpty: true, runtimeKindsField: "kinds", runtimeKindsRelation: "independent", mutationIdField: "mutationId", sourceMutationDirectoryNameField: "sourceMutationDirectoryName", mutationDirectoryNameField: "mutationDirectoryName", scenariosField: "scenarios", scenarioIdField: "id", scenarioDirectoryNameField: "directoryName", sourceBundleUniquenessFields: ["mutationId", "sourceMutationDirectoryName", "scenarioId"], canonicalBundleUniquenessFields: ["mutationId", "mutationDirectoryName", "scenarioId"], coverage: "every-physical-bundle-exactly-once" };
    if (JSON.stringify(contract) !== JSON.stringify(expected)) problems.push(`semanticPathProjectionCatalogContracts[${JSON.stringify(id)}] must be the exact independent physical vectors contract.`);
  }

  if (record(taxonomy.semanticPathProjectionContracts, "semanticPathProjectionContracts")) for (const [id, contract] of Object.entries(taxonomy.semanticPathProjectionContracts)) {
    kebabId(id, `semanticPathProjectionContracts id ${JSON.stringify(id)}`);
    exactKeys(contract, ["sourceOwnerKindId", ...(contract.sourceArtifactMemberName === undefined ? [] : ["sourceArtifactMemberName"]), "sourceSegments", "profileRendererId", "destinationOwnerKindId", "destinationSegments", "descendantContractId", "catalogContractId", "rationaleRule"], `semanticPathProjectionContracts[${JSON.stringify(id)}]`);
    if (typeof contract.sourceArtifactMemberName !== "string" || !taxonomy.semanticDirectoryMemberKinds[contract.sourceOwnerKindId]?.memberNames.includes(contract.sourceArtifactMemberName)) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].sourceArtifactMemberName must be one exact source-owner member.`);
    if (!taxonomy.semanticDirectoryMemberKinds[contract.sourceOwnerKindId]) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].sourceOwnerKindId is missing.`);
    if (!taxonomy.semanticDirectoryMemberKinds[contract.destinationOwnerKindId]) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].destinationOwnerKindId is missing.`);
    if (!taxonomy.semanticPathProjectionProfileRenderers[contract.profileRendererId]) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].profileRendererId is missing.`);
    if (!taxonomy.semanticDescendantContracts[contract.descendantContractId]) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].descendantContractId is missing.`);
    if (!taxonomy.semanticPathProjectionCatalogContracts[contract.catalogContractId]) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].catalogContractId is missing.`);
    if (!["artifact-example-model-catalog-projection-v1", "artifact-editor-command-projection-v1"].includes(contract.rationaleRule)) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].rationaleRule is invalid.`);
    const captures = new Set<string>();
    let sourceParentKindId: string | undefined;
    for (const [index, segment] of contract.sourceSegments.entries()) {
      const value = segment as unknown as Record<string, unknown>;
      const hasProjected = typeof value.projectedMemberKindId === "string";
      const hasMember = typeof value.memberKindId === "string";
      exactKeys(value, hasProjected ? ["projectedMemberKindId", "capture"] : hasMember ? ["memberKindId", "literal"] : ["kindId", "literal" in value ? "literal" : "capture"], `semanticPathProjectionContracts[${JSON.stringify(id)}].sourceSegments[${index}]`);
      const kindId = (hasProjected ? value.projectedMemberKindId : hasMember ? value.memberKindId : value.kindId) as string;
      const kind = hasProjected ? taxonomy.semanticProjectedMemberKinds[kindId] : hasMember ? taxonomy.semanticDirectoryMemberKinds[kindId] : taxonomy.semanticDirectoryKinds[kindId];
      if (!kind) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].sourceSegments[${index}] references a missing kind.`);
      if (typeof value.literal === "string") {
        const validLiteral = hasMember
          ? Boolean(taxonomy.semanticDirectoryMemberKinds[kindId]?.ownerKindIds.includes(sourceParentKindId ?? "") && taxonomy.semanticDirectoryMemberKinds[kindId]?.memberNames.includes(value.literal))
          : semanticDirectoryKindId(value.literal, taxonomy, { parentKindId: sourceParentKindId }) === kindId;
        if (!validLiteral) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].sourceSegments[${index}].literal does not resolve to its kind.`);
      }
      if (typeof value.capture === "string") {
        if (!["standardVersion", "subsetId", "mutationId", "scenarioId", "commandDirectoryName"].includes(value.capture)) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].sourceSegments[${index}].capture is invalid.`);
        if (captures.has(value.capture)) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}] captures ${JSON.stringify(value.capture)} more than once.`);
        captures.add(value.capture);
      }
      sourceParentKindId = kindId;
    }
    const requiredCaptures = contract.rationaleRule === "artifact-example-model-catalog-projection-v1" ? ["standardVersion", "subsetId"] : ["standardVersion", "subsetId", "commandDirectoryName"];
    if (captures.size !== requiredCaptures.length || requiredCaptures.some((field) => !captures.has(field))) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}] must capture exactly ${requiredCaptures.join(", ")}.`);
    let renderedProfiles = 0;
    let destinationParentKindId: string | undefined;
    for (const [index, segment] of contract.destinationSegments.entries()) {
      const value = segment as unknown as Record<string, unknown>;
      const hasProjected = typeof value.projectedMemberKindId === "string";
      const operation = "literal" in value ? "literal" : "render" in value ? "render" : "copy";
      exactKeys(value, [hasProjected ? "projectedMemberKindId" : "kindId", operation], `semanticPathProjectionContracts[${JSON.stringify(id)}].destinationSegments[${index}]`);
      const kindId = (hasProjected ? value.projectedMemberKindId : value.kindId) as string;
      if (!(hasProjected ? taxonomy.semanticProjectedMemberKinds[kindId] : taxonomy.semanticDirectoryKinds[kindId])) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].destinationSegments[${index}] references a missing kind.`);
      if (typeof value.literal === "string" && semanticDirectoryKindId(value.literal, taxonomy, { parentKindId: destinationParentKindId }) !== kindId) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].destinationSegments[${index}].literal does not resolve to its kind.`);
      if (value.render !== undefined) {
        if (value.render !== "profile") problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].destinationSegments[${index}].render must be profile.`);
        renderedProfiles += 1;
      }
      if (typeof value.copy === "string" && !captures.has(value.copy)) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}].destinationSegments[${index}].copy must reference a source capture.`);
      destinationParentKindId = kindId;
    }
    if (renderedProfiles !== 1) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}] must render exactly one profile segment.`);
    const projectedReferences = [...contract.sourceSegments, ...contract.destinationSegments].flatMap((segment) => "projectedMemberKindId" in segment ? [segment.projectedMemberKindId] : []);
    for (const [projectedId, projected] of Object.entries(taxonomy.semanticProjectedMemberKinds)) {
      const references = projectedReferences.filter((candidate) => candidate === projectedId).length;
      if (projected.projectionContractId === id && references !== 2) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}] must reference projected member ${JSON.stringify(projectedId)} exactly once in source and destination.`);
    }
    if (contract.sourceArtifactMemberName) {
      try {
        if (artifactPathProjectionCatalogRoots(`🗿️artifacts/${contract.sourceArtifactMemberName}`, id, taxonomy).length === 0) problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}] must render at least one exact forward owner profile.`);
      } catch (error) { problems.push(`semanticPathProjectionContracts[${JSON.stringify(id)}] has invalid forward profile authority: ${error instanceof Error ? error.message : String(error)}`); }
    }
  }

  if (record(taxonomy.semanticPolicyStateCoordinateContracts, "semanticPolicyStateCoordinateContracts")) {
    if (Object.keys(taxonomy.semanticPolicyStateCoordinateContracts).join("\0") !== "dependency-freeze-users-v1") problems.push("semanticPolicyStateCoordinateContracts requires the exact dependency policy-state contract.");
    for (const [id, contract] of Object.entries(taxonomy.semanticPolicyStateCoordinateContracts)) {
      exactKeys(contract, ["contractKind", "statePath", "stateSchemaVersion", "sourceDisposition", "ownerProjectionContractId", "packageIds", "manifestFilename", "dependencyEvidenceField", "coordinatePointer", "preserveNonCoordinateBytes"], `semanticPolicyStateCoordinateContracts.${id}`);
      if (contract.contractKind !== "dependency-freeze-user-coordinates" || contract.statePath !== "🔒️dependencies.json" || contract.stateSchemaVersion !== 2 || contract.sourceDisposition !== "authored-policy-state" || contract.ownerProjectionContractId !== "nested-cargo-packages-v1" || !taxonomy.semanticPackageProjectionContracts[contract.ownerProjectionContractId] || JSON.stringify(contract.packageIds) !== JSON.stringify(["jcoprobe-guest"]) || contract.manifestFilename !== "Cargo.toml" || contract.dependencyEvidenceField !== "witDependency" || contract.coordinatePointer !== "/entries/*/users/*" || contract.preserveNonCoordinateBytes !== true) problems.push(`semanticPolicyStateCoordinateContracts.${id} has invalid exact user-coordinate authority.`);
    }
  }
  if (record(taxonomy.semanticPackageProjectionContracts, "semanticPackageProjectionContracts")) {
    if (Object.keys(taxonomy.semanticPackageProjectionContracts).join("\0") !== "nested-cargo-packages-v1") problems.push("semanticPackageProjectionContracts requires the exact nested Cargo contract.");
    for (const [id, contract] of Object.entries(taxonomy.semanticPackageProjectionContracts)) {
      exactKeys(contract, ["contractKind", "authorityCatalogPath", "authorityCatalogSha256", "packageIds", "sourceLeafCounts", "purityCount", "adapterCount", "derivedLeafCount", "joinedPathBindingCounts", "generatedSourceRetirementCounts", "authoredFragmentCounts", "rationaleRule"], `semanticPackageProjectionContracts.${id}`);
      if (contract.contractKind !== "exact-nested-cargo-package-catalog" || !exampleAssetPath(contract.authorityCatalogPath, taxonomy) || !/^[0-9a-f]{64}$/u.test(contract.authorityCatalogSha256) || JSON.stringify(contract.packageIds) !== JSON.stringify(["wgpu-renderer", "jcoprobe-guest"]) || JSON.stringify(contract.sourceLeafCounts) !== JSON.stringify([32, 4]) || contract.purityCount !== 27 || contract.adapterCount !== 5 || contract.derivedLeafCount !== 1 || JSON.stringify(contract.joinedPathBindingCounts) !== JSON.stringify([1, 0]) || JSON.stringify(contract.generatedSourceRetirementCounts) !== JSON.stringify([1, 0]) || JSON.stringify(contract.authoredFragmentCounts) !== JSON.stringify([31, 0]) || contract.rationaleRule !== "nested-cargo-package-projection-v1") problems.push(`semanticPackageProjectionContracts.${id} has invalid exact authority.`);
    }
  }
}
