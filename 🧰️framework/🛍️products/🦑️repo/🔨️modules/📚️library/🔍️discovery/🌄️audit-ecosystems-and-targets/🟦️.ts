import type { TaxonomyAudit } from "../🪣️taxonomy-audit-context/🟦️.ts";

/** 🌄️ Audits the ecosystem and target registries. */
export function auditEcosystemsAndTargets(audit: TaxonomyAudit): void {
  const { taxonomy, problems, ids } = audit;
  for (const [lang, ecosystem] of Object.entries(taxonomy.ecosystems ?? {})) {
    const oldShape = ecosystem as unknown as Record<string, unknown>;
    for (const key of ["manifestFilename", "moduleRootFilename", "entryFilenames", "leafFilename", "sourceExtension", "packagingDirNames"]) if (key in oldShape) problems.push(`ecosystems[${JSON.stringify(lang)}].${key} was removed.`);
    if (!['manifest', 'boundary-only'].includes(ecosystem.packageIdentity)) problems.push(`ecosystems[${JSON.stringify(lang)}].packageIdentity is invalid.`);
    if (ecosystem.packageIdentity === "manifest" && (!ecosystem.manifestContractId || !ecosystem.marker)) problems.push(`ecosystems[${JSON.stringify(lang)}] manifest identity requires a manifest contract and marker.`);
    if (ecosystem.packageIdentity === "boundary-only" && (ecosystem.manifestContractId !== null || ecosystem.marker !== null)) problems.push(`ecosystems[${JSON.stringify(lang)}] boundary-only identity cannot declare a manifest or marker.`);
    if (ecosystem.manifestContractId && !taxonomy.fixedFilenameContracts[ecosystem.manifestContractId]) problems.push(`ecosystems[${JSON.stringify(lang)}].manifestContractId is missing.`);
    if (ecosystem.moduleRootContractId && !taxonomy.fixedFilenameContracts[ecosystem.moduleRootContractId]) problems.push(`ecosystems[${JSON.stringify(lang)}].moduleRootContractId is missing.`);
    if (!taxonomy.fileKinds[ecosystem.componentFileKindId]) problems.push(`ecosystems[${JSON.stringify(lang)}].componentFileKindId is missing.`);
    ids(ecosystem.sourceFileKindIds, taxonomy.fileKinds, `ecosystems[${JSON.stringify(lang)}].sourceFileKindIds`);
    ids(ecosystem.entryContractIds, taxonomy.configurableEntryContracts, `ecosystems[${JSON.stringify(lang)}].entryContractIds`);
    ids(ecosystem.packagingDirectoryKindIds ?? [], taxonomy.semanticDirectoryKinds, `ecosystems[${JSON.stringify(lang)}].packagingDirectoryKindIds`);
    if (!taxonomy.packageBoundaryRules[lang]) problems.push(`ecosystems[${JSON.stringify(lang)}] has no packageBoundaryRules entry.`);
  }
  for (const [target, spec] of Object.entries(taxonomy.targets ?? {})) {
    const oldShape = spec as unknown as Record<string, unknown>;
    for (const key of ["leafFilename", "entryFilenames"]) if (key in oldShape) problems.push(`targets[${JSON.stringify(target)}].${key} was removed.`);
    if (!taxonomy.ecosystems[spec.lang]) problems.push(`targets[${JSON.stringify(target)}].lang is missing.`);
    if (!taxonomy.fileKinds[spec.componentFileKindId]) problems.push(`targets[${JSON.stringify(target)}].componentFileKindId is missing.`);
    ids(spec.entryContractIds, taxonomy.configurableEntryContracts, `targets[${JSON.stringify(target)}].entryContractIds`);
  }
}
