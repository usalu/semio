import { fixedContractFilename } from "../🎯️taxonomy-path-matching/🟦️.ts";
import type { TaxonomyAudit } from "../🪣️taxonomy-audit-context/🟦️.ts";

/** 🚛️ Audits the configurable entry contracts, the package glue grammar, the package boundary rules and profiles and the package source dispositions. */
export function auditPackageBoundaryContracts(audit: TaxonomyAudit): void {
  const { taxonomy, problems, record, ids, exactKeys, fileKindIdForSourcePath, canonicalFilenamesForKind } = audit;
  if (record(taxonomy.configurableEntryContracts, "configurableEntryContracts")) for (const [id, contract] of Object.entries(taxonomy.configurableEntryContracts)) {
    if (!taxonomy.fileKinds[contract.fileKindId]) problems.push(`configurableEntryContracts[${JSON.stringify(id)}].fileKindId is missing.`);
    else if (!canonicalFilenamesForKind(contract.fileKindId, taxonomy).includes(contract.filename)) problems.push(`configurableEntryContracts[${JSON.stringify(id)}].filename is not canonical for its file kind.`);
    if (!taxonomy.ecosystems[contract.ecosystemId]) problems.push(`configurableEntryContracts[${JSON.stringify(id)}].ecosystemId is missing.`);
    if (!Array.isArray(contract.configurationSources) || contract.configurationSources.length === 0) problems.push(`configurableEntryContracts[${JSON.stringify(id)}].configurationSources must be non-empty.`);
  }

  if (record(taxonomy.packageGlueGrammar, "packageGlueGrammar")) for (const [id, grammar] of Object.entries(taxonomy.packageGlueGrammar)) {
    if (!["rust", "typescript", "javascript", "go", "python", "dotnet", "c-cpp", "tex"].includes(grammar.analyzer)) problems.push(`packageGlueGrammar[${JSON.stringify(id)}].analyzer is invalid.`);
    if (!Number.isSafeInteger(grammar.maxDelegationStatements) || grammar.maxDelegationStatements < 0) problems.push(`packageGlueGrammar[${JSON.stringify(id)}].maxDelegationStatements is invalid.`);
    if (!Array.isArray(grammar.allowedRoles) || grammar.allowedRoles.some((role) => !["declaration", "registration", "bootstrap", "thin-delegation"].includes(role))) problems.push(`packageGlueGrammar[${JSON.stringify(id)}].allowedRoles is invalid.`);
  }

  if (record(taxonomy.packageBoundaryRules, "packageBoundaryRules")) for (const [ecosystemId, rule] of Object.entries(taxonomy.packageBoundaryRules)) {
    if (!taxonomy.ecosystems[ecosystemId]) problems.push(`packageBoundaryRules[${JSON.stringify(ecosystemId)}] has no ecosystem.`);
    if (rule.manifestContractId && !taxonomy.fixedFilenameContracts[rule.manifestContractId]) problems.push(`packageBoundaryRules[${JSON.stringify(ecosystemId)}].manifestContractId is missing.`);
    ids(rule.entryContractIds, taxonomy.configurableEntryContracts, `packageBoundaryRules[${JSON.stringify(ecosystemId)}].entryContractIds`);
    ids(rule.allowedFixedContractIds, taxonomy.fixedFilenameContracts, `packageBoundaryRules[${JSON.stringify(ecosystemId)}].allowedFixedContractIds`);
    ids(rule.allowedFileKindIds, taxonomy.fileKinds, `packageBoundaryRules[${JSON.stringify(ecosystemId)}].allowedFileKindIds`);
    ids(rule.allowedDirectoryKindIds, taxonomy.semanticDirectoryKinds, `packageBoundaryRules[${JSON.stringify(ecosystemId)}].allowedDirectoryKindIds`);
    if (!taxonomy.packageGlueGrammar[rule.glueGrammarId]) problems.push(`packageBoundaryRules[${JSON.stringify(ecosystemId)}].glueGrammarId is missing.`);
    if (rule.recursive !== true || rule.uncertainRole !== "problem" || rule.implementationRole !== "problem") problems.push(`packageBoundaryRules[${JSON.stringify(ecosystemId)}] must recursively block uncertain and implementation roles.`);
    const admission = rule.implementationAdmission;
    if (admission !== null && admission !== undefined) {
      if (admission.form === "nested-source-unit") {
        exactKeys(admission, ["form", "unitSourceFileKindId"], `packageBoundaryRules[${JSON.stringify(ecosystemId)}].implementationAdmission`);
        if (!rule.allowedFileKindIds.includes(admission.unitSourceFileKindId) || taxonomy.fileKinds[admission.unitSourceFileKindId]?.role !== "source") problems.push(`packageBoundaryRules[${JSON.stringify(ecosystemId)}].implementationAdmission.unitSourceFileKindId must be an allowed source file kind.`);
      } else if (admission.form === "nested-manifest-unit") {
        exactKeys(admission, ["form", "unitManifestContractId"], `packageBoundaryRules[${JSON.stringify(ecosystemId)}].implementationAdmission`);
        if (admission.unitManifestContractId !== rule.manifestContractId) problems.push(`packageBoundaryRules[${JSON.stringify(ecosystemId)}].implementationAdmission.unitManifestContractId must be the package manifest contract.`);
      } else problems.push(`packageBoundaryRules[${JSON.stringify(ecosystemId)}].implementationAdmission.form is invalid.`);
    }
  }

  if (record(taxonomy.packageBoundaryProfiles, "packageBoundaryProfiles")) for (const [id, profile] of Object.entries(taxonomy.packageBoundaryProfiles)) {
    exactKeys(profile, ["admission", "allowedFileKindIds", "allowedDirectoryKindIds", "allowedFixedContractIds", "glueGrammarId", "recursive", "uncertainRole", "implementationRole", "reason"], `packageBoundaryProfiles[${JSON.stringify(id)}]`);
    if (profile.admission !== "blocked-until-language-directory-registered") problems.push(`packageBoundaryProfiles[${JSON.stringify(id)}].admission is invalid.`);
    ids(profile.allowedFileKindIds, taxonomy.fileKinds, `packageBoundaryProfiles[${JSON.stringify(id)}].allowedFileKindIds`);
    ids(profile.allowedDirectoryKindIds, taxonomy.semanticDirectoryKinds, `packageBoundaryProfiles[${JSON.stringify(id)}].allowedDirectoryKindIds`);
    ids(profile.allowedFixedContractIds, taxonomy.fixedFilenameContracts, `packageBoundaryProfiles[${JSON.stringify(id)}].allowedFixedContractIds`);
    if (!taxonomy.packageGlueGrammar[profile.glueGrammarId]) problems.push(`packageBoundaryProfiles[${JSON.stringify(id)}].glueGrammarId is missing.`);
    if (profile.recursive !== true || profile.uncertainRole !== "problem" || profile.implementationRole !== "problem" || !profile.reason) problems.push(`packageBoundaryProfiles[${JSON.stringify(id)}] must fail closed with a reason.`);
  }

  if (record(taxonomy.packageSourceDispositions, "packageSourceDispositions")) {
    const expected = new Map<string, "fixed" | "configurable">();
    for (const [id, contract] of Object.entries(taxonomy.fixedFilenameContracts)) {
      const kindId = fileKindIdForSourcePath(fixedContractFilename(contract), taxonomy);
      if (kindId && taxonomy.fileKinds[kindId]?.role === "source") expected.set(id, "fixed");
    }
    for (const [id, contract] of Object.entries(taxonomy.configurableEntryContracts)) if (taxonomy.fileKinds[contract.fileKindId]?.role === "source") expected.set(id, "configurable");
    for (const [id, disposition] of Object.entries(taxonomy.packageSourceDispositions)) if (disposition.contractKind === "fixed" && disposition.grammarId && taxonomy.fixedFilenameContracts[id]) expected.set(id, "fixed");
    for (const missing of [...expected.keys()].filter((id) => !taxonomy.packageSourceDispositions[id])) problems.push(`packageSourceDispositions is missing source-format contract ${JSON.stringify(missing)}.`);
    for (const [id, disposition] of Object.entries(taxonomy.packageSourceDispositions)) {
      exactKeys(disposition, ["contractKind", "disposition", "validator", ...(disposition.grammarId === undefined ? [] : ["grammarId"]), "authority", "verification"], `packageSourceDispositions[${JSON.stringify(id)}]`);
      if (!expected.has(id)) problems.push(`packageSourceDispositions[${JSON.stringify(id)}] does not name a source-format fixed/configurable contract.`);
      else if (expected.get(id) !== disposition.contractKind) problems.push(`packageSourceDispositions[${JSON.stringify(id)}].contractKind does not match its registry.`);
      if (!["adapter-source", "tool-metadata"].includes(disposition.disposition)) problems.push(`packageSourceDispositions[${JSON.stringify(id)}].disposition is invalid.`);
      const TOOL_CONFIG_VALIDATORS: Readonly<Record<string, string>> = { "vitest-configuration": "vitest-config-entry", "tool-config-tailwind": "tailwind-config", "tool-config-postcss": "postcss-config", "tool-config-eslint": "eslint-config", "tool-config-dependency-cruiser": "dependency-cruiser-config", "pytest-configuration": "root-pytest-config", "eslint-configuration": "root-eslint-config", "vscode-test-configuration": "vscode-test-cli-config" };
      const configValidatorOwner = TOOL_CONFIG_VALIDATORS[disposition.validator];
      if (!["package-glue", "command-router", ...Object.keys(TOOL_CONFIG_VALIDATORS)].includes(disposition.validator) || (disposition.disposition === "adapter-source") !== (disposition.validator === "package-glue") || (configValidatorOwner !== undefined && id !== configValidatorOwner)) problems.push(`packageSourceDispositions[${JSON.stringify(id)}] disposition/validator pair is invalid.`);
      if (disposition.grammarId !== undefined && !taxonomy.packageGlueGrammar[disposition.grammarId]) problems.push(`packageSourceDispositions[${JSON.stringify(id)}].grammarId is missing.`);
      if (!disposition.authority || !disposition.verification) problems.push(`packageSourceDispositions[${JSON.stringify(id)}] must declare authority and verification.`);
    }
  }
}
