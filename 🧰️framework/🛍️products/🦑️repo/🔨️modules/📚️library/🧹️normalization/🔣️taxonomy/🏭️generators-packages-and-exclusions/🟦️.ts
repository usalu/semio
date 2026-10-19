/** 🏭️ The generator contracts, ecosystems, package rules, source dispositions, path exclusions and the assembled schema. */
import { requireExactKeys, requireLiteral, requireRecord, requireString, requireStringArray, type UnknownRecord } from "@semio-tech/schema-validator-ts";
import type { TaxonomyV7 } from "../📚️loaded/🟦️.ts";
import type { Taxonomy as DiscoveryTaxonomy } from "../../../🔍️discovery/📑️taxonomy-document/🟦️.ts";
import { createTaxonomyPathMatcher } from "../../../🔍️discovery/🎯️taxonomy-path-matching/🟦️.ts";
import { canonicalJson } from "../../../🧾️serialization/🔣️json/🟦️.ts";
import type { ConfigurableEntryContract, FileKindResolutionRuleSpec, FileKindSpec, FixedDirectoryContract, FixedFilenameContract, FixedFilenameRejectionContract, ScopedFileKindSpec, SemanticDirectoryKindSpec, SemanticDirectoryMemberKindSpec, SemanticProjectedMemberKindSpec } from "../📐️kind-specs/🟦️.ts";
import { normalizeRelative } from "../../🛣️path/🟦️.ts";
import type { MutationCatalogProjectionContractIds, SemanticPathProjectionCatalogContract, SemanticPathProjectionContract, SemanticPathProjectionProfileRenderer, SemanticPathProjectionReferenceConsumerContract } from "../🧭️path-projection/🟦️.ts";
import type { SemanticDescendantContract } from "../🌳️descendants/🟦️.ts";
import type { SemanticOwnedFileProjectionContract } from "../🗃️owned-file-projection/🟦️.ts";
import type { GeneratorProjectionActivation, RegistryCatalogInputDiscovery } from "../../../🔍️discovery/📜️taxonomy-contracts/🟦️.ts";
import type { EcosystemSpec, GeneratorContractSpec, GeneratorOutputRootSpec, GeneratorOwnership, PackageBoundaryProfile, PackageBoundaryRule, PackageGlueGrammar, PackageSourceDisposition } from "../🏭️generators-packages/🟦️.ts";
import { generatorPreviewResourceLimits, generatorPreviewScriptArguments } from "../../../🔍️discovery/🛠️generator-commands/🟦️.ts";
import { parseGeneratorPreviewProgressPolicyV1 } from "../../../🏭️generator/👁️preview/📈️progress/🟦️.ts";
import type { SemanticPackageGeneration } from "../../../🔍️discovery/📦️package-projection-contracts/🟦️.ts";
import type { PackageImplementationAdmission } from "../../../🔍️discovery/🧱️package-implementation-admission/🟦️.ts";
import { TAXONOMY_OPAQUE_PATH_EXCLUSIONS } from "../🧰️taxonomy-parse-helpers/🟦️.ts";

function parseImplementationAdmission(value: unknown, name: string): PackageImplementationAdmission | null {
  if (value === undefined || value === null) return null;
  const row = requireRecord(value, name);
  if (row.form === "nested-source-unit") {
    requireExactKeys(row, ["form", "unitSourceFileKindId"], name);
    return { form: "nested-source-unit", unitSourceFileKindId: requireString(row.unitSourceFileKindId, `${name}.unitSourceFileKindId`) };
  }
  if (row.form === "nested-manifest-unit") {
    requireExactKeys(row, ["form", "unitManifestContractId"], name);
    return { form: "nested-manifest-unit", unitManifestContractId: requireString(row.unitManifestContractId, `${name}.unitManifestContractId`) };
  }
  throw new Error(`Taxonomy v7 ${name}.form must be nested-source-unit or nested-manifest-unit`);
}

/** ⚙ Parses the generator contracts and the ecosystems. */
export function parseGeneratorsAndEcosystems({ generatorRows, validatedContractPattern, ecosystemRows }: {
  readonly generatorRows: UnknownRecord;
  readonly validatedContractPattern: (value: unknown, name: string, exactBasename: boolean) => string;
  readonly ecosystemRows: UnknownRecord;
}) {
  const generatorContracts: Record<string, GeneratorContractSpec> = {};
  const generatorRoots: { readonly id: string; readonly path: string }[] = [];
  for (const [id, value] of Object.entries(generatorRows)) {
    if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(id)) throw new Error(`Taxonomy v7 generatorContracts.${id} has an invalid identifier`);
    const spec = requireRecord(value, `generatorContracts.${id}`);
    if (spec.ownership !== "owned" && spec.ownership !== "external") throw new Error(`Taxonomy v7 generatorContracts.${id}.ownership is invalid`);
    const ownership = spec.ownership as GeneratorOwnership;
    const ownerPath = spec.ownerPath === null ? null : normalizeRelative(requireString(spec.ownerPath, `generatorContracts.${id}.ownerPath`));
    const target = spec.target === null ? null : requireString(spec.target, `generatorContracts.${id}.target`);
    const previewTarget = spec.previewTarget === undefined ? undefined : requireString(spec.previewTarget, `generatorContracts.${id}.previewTarget`);
    const checkTarget = spec.checkTarget === undefined ? undefined : requireString(spec.checkTarget, `generatorContracts.${id}.checkTarget`);
    if ((ownership === "owned") !== (ownerPath !== null && target !== null)) throw new Error(`Taxonomy v7 generatorContracts.${id} owner and target do not match ownership`);
    if (target && !/^@?[a-z0-9][a-z0-9@._/-]*:[a-z0-9][a-z0-9._-]*$/u.test(target)) throw new Error(`Taxonomy v7 generatorContracts.${id}.target must be one exact Nx target`);
    if (ownership === "owned" ? !previewTarget : previewTarget !== undefined) throw new Error(`Taxonomy v7 generatorContracts.${id}.previewTarget does not match ownership`);
    if (previewTarget && !/^@?[a-z0-9][a-z0-9@._/-]*:[a-z0-9][a-z0-9._-]*$/u.test(previewTarget)) throw new Error(`Taxonomy v7 generatorContracts.${id}.previewTarget must be one exact Nx target`);
    const previewArguments = spec.previewArguments === undefined ? undefined : requireStringArray(spec.previewArguments, `generatorContracts.${id}.previewArguments`);
    if (target) generatorPreviewScriptArguments({ ownership, target, previewTarget, previewArguments });
    else if (previewArguments !== undefined) throw new Error(`Taxonomy v7 generatorContracts.${id}.previewArguments requires owned output authority`);
    const previewProgress = spec.previewProgress === undefined ? undefined : parseGeneratorPreviewProgressPolicyV1(spec.previewProgress);
    if (previewProgress && (!target || !previewTarget || ownership !== "owned")) throw new Error(`Taxonomy v7 generatorContracts.${id}.previewProgress requires owned preview authority`);
    const previewLimits = spec.previewLimits === undefined ? undefined : generatorPreviewResourceLimits({ ownership, previewTarget, previewLimits: spec.previewLimits as GeneratorContractSpec["previewLimits"] });
    const compilerInputManifest = spec.compilerInputManifest === undefined ? undefined : requireRecord(spec.compilerInputManifest, `generatorContracts.${id}.compilerInputManifest`) as unknown as GeneratorContractSpec["compilerInputManifest"];
    if (checkTarget && !/^@?[a-z0-9][a-z0-9@._/-]*:[a-z0-9][a-z0-9._-]*$/u.test(checkTarget)) throw new Error(`Taxonomy v7 generatorContracts.${id}.checkTarget must be one exact Nx target`);
    const inputPatterns = requireStringArray(spec.inputPatterns, `generatorContracts.${id}.inputPatterns`).map((pattern, index) => validatedContractPattern(pattern, `generatorContracts.${id}.inputPatterns[${index}]`, false));
    if (ownership === "owned" ? inputPatterns.length === 0 : inputPatterns.length !== 0) throw new Error(`Taxonomy v7 generatorContracts.${id}.inputPatterns do not match ownership`);
    const outputRows = spec.outputRoots;
    if (!Array.isArray(outputRows) || outputRows.length === 0) throw new Error(`Taxonomy v7 generatorContracts.${id}.outputRoots must not be empty`);
    const outputRoots = outputRows.map((value, index) => {
      const output = requireRecord(value, `generatorContracts.${id}.outputRoots[${index}]`);
      const outputPath = requireString(output.path, `generatorContracts.${id}.outputRoots[${index}].path`);
      if (outputPath !== normalizeRelative(outputPath) || /[*?\[\]]/u.test(outputPath)) throw new Error(`Taxonomy v7 generatorContracts.${id} output path must be one literal NFC repository path`);
      if (output.inclusion !== "tracked" && output.inclusion !== "ignored") throw new Error(`Taxonomy v7 generatorContracts.${id} output inclusion is invalid`);
      generatorRoots.push({ id, path: outputPath });
      return { path: outputPath, inclusion: output.inclusion } as GeneratorOutputRootSpec;
    }).sort((left, right) => left.path.localeCompare(right.path));
    if (new Set(outputRoots.map((output) => output.path)).size !== outputRoots.length) throw new Error(`Taxonomy v7 generatorContracts.${id} repeats an output root`);
    const inputDiscovery = spec.inputDiscovery === undefined ? undefined : requireRecord(spec.inputDiscovery, `generatorContracts.${id}.inputDiscovery`) as unknown as RegistryCatalogInputDiscovery;
    if (inputDiscovery && (id !== "plugin-registry" || ownership !== "owned" || inputDiscovery.kind !== "registry-catalog")) throw new Error(`Taxonomy v7 generatorContracts.${id}.inputDiscovery has no exact catalog authority`);
    const projectionActivation = spec.projectionActivation === undefined ? undefined : requireRecord(spec.projectionActivation, `generatorContracts.${id}.projectionActivation`) as unknown as GeneratorProjectionActivation;
    const packageGeneration = spec.packageGeneration === undefined ? undefined : requireRecord(spec.packageGeneration, `generatorContracts.${id}.packageGeneration`) as unknown as SemanticPackageGeneration;
    generatorContracts[id] = { ownership, ownerPath, target, previewTarget, previewArguments, previewProgress, previewLimits, checkTarget, inputPatterns: [...new Set(inputPatterns)].sort(), inputDiscovery, compilerInputManifest, packageGeneration, projectionActivation, outputRoots, reason: requireString(spec.reason, `generatorContracts.${id}.reason`) };
  }
  if (Object.keys(generatorContracts).length === 0) throw new Error("Taxonomy v7 generatorContracts must not be empty");
  for (let left = 0; left < generatorRoots.length; left++) for (let right = left + 1; right < generatorRoots.length; right++) {
    const a = generatorRoots[left];
    const b = generatorRoots[right];
    if(!a||!b)throw new Error("Required generator root pair is missing");
    if (a.path === b.path || a.path.startsWith(`${b.path}/`) || b.path.startsWith(`${a.path}/`)) throw new Error(`Taxonomy v7 generator output roots overlap: ${a.id}:${a.path} and ${b.id}:${b.path}`);
  }

  const ecosystems: Record<string, EcosystemSpec> = {};
  for (const [id, value] of Object.entries(ecosystemRows)) {
    const spec = requireRecord(value, `ecosystems.${id}`);
    if (spec.packageIdentity !== "manifest" && spec.packageIdentity !== "boundary-only") throw new Error(`Taxonomy v7 ecosystems.${id}.packageIdentity is invalid`);
    const manifestContractId = spec.manifestContractId === null ? null : requireString(spec.manifestContractId, `ecosystems.${id}.manifestContractId`);
    if ((spec.packageIdentity === "manifest") !== (manifestContractId !== null)) throw new Error(`Taxonomy v7 ecosystems.${id} manifest identity is incomplete`);
    ecosystems[id] = { packageIdentity: spec.packageIdentity, manifestContractId };
  }
  if (Object.keys(ecosystems).length === 0) throw new Error("Taxonomy v7 ecosystems must not be empty");
  return { generatorContracts, ecosystems };
}

/** 📦 Parses the package glue grammar, boundary rules and profiles and the source dispositions. */
export function parsePackageRulesAndDispositions({ grammarRows, boundaryRows, fixedFilenameContracts, configurableEntryContracts, fileKinds, semanticDirectoryKinds, boundaryProfileRows, sourceDispositionRows }: {
  readonly grammarRows: UnknownRecord;
  readonly boundaryRows: UnknownRecord;
  readonly fixedFilenameContracts: Record<string, FixedFilenameContract>;
  readonly configurableEntryContracts: Record<string, ConfigurableEntryContract>;
  readonly fileKinds: Record<string, FileKindSpec>;
  readonly semanticDirectoryKinds: Record<string, SemanticDirectoryKindSpec>;
  readonly boundaryProfileRows: UnknownRecord;
  readonly sourceDispositionRows: UnknownRecord;
}) {
  const packageGlueGrammar: Record<string, PackageGlueGrammar> = {};
  for (const [id, value] of Object.entries(grammarRows)) {
    const spec = requireRecord(value, `packageGlueGrammar.${id}`);
    if (!["rust", "typescript", "javascript", "go", "python", "dotnet", "c-cpp", "tex"].includes(String(spec.analyzer))) throw new Error(`Taxonomy v7 packageGlueGrammar.${id}.analyzer is invalid`);
    const allowedRoles = requireStringArray(spec.allowedRoles, `packageGlueGrammar.${id}.allowedRoles`) as PackageGlueGrammar["allowedRoles"];
    if (allowedRoles.some((role) => !["declaration", "registration", "bootstrap", "thin-delegation"].includes(role)) || new Set(allowedRoles).size !== allowedRoles.length) throw new Error(`Taxonomy v7 packageGlueGrammar.${id}.allowedRoles is invalid`);
    if (!Number.isSafeInteger(spec.maxDelegationStatements) || (spec.maxDelegationStatements as number) < 0) throw new Error(`Taxonomy v7 packageGlueGrammar.${id}.maxDelegationStatements is invalid`);
    packageGlueGrammar[id] = { analyzer: spec.analyzer as PackageGlueGrammar["analyzer"], allowedRoles, maxDelegationStatements: spec.maxDelegationStatements as number };
  }

  const packageBoundaryRules: Record<string, PackageBoundaryRule> = {};
  for (const [id, value] of Object.entries(boundaryRows)) {
    const spec = requireRecord(value, `packageBoundaryRules.${id}`);
    const glueGrammarId = requireString(spec.glueGrammarId, `packageBoundaryRules.${id}.glueGrammarId`);
    if (!packageGlueGrammar[glueGrammarId]) throw new Error(`Taxonomy v7 packageBoundaryRules.${id} references unknown grammar ${glueGrammarId}`);
    if (spec.recursive !== true || spec.uncertainRole !== "problem" || spec.implementationRole !== "problem") throw new Error(`Taxonomy v7 packageBoundaryRules.${id} must be recursive and fail closed`);
    packageBoundaryRules[id] = {
      manifestContractId: spec.manifestContractId === null ? null : requireString(spec.manifestContractId, `packageBoundaryRules.${id}.manifestContractId`),
      entryContractIds: requireStringArray(spec.entryContractIds, `packageBoundaryRules.${id}.entryContractIds`),
      allowedFixedContractIds: requireStringArray(spec.allowedFixedContractIds, `packageBoundaryRules.${id}.allowedFixedContractIds`),
      allowedFileKindIds: requireStringArray(spec.allowedFileKindIds, `packageBoundaryRules.${id}.allowedFileKindIds`),
      allowedDirectoryKindIds: requireStringArray(spec.allowedDirectoryKindIds, `packageBoundaryRules.${id}.allowedDirectoryKindIds`),
      glueGrammarId,
      recursive: true,
      uncertainRole: "problem",
      implementationRole: "problem",
      implementationAdmission: parseImplementationAdmission(spec.implementationAdmission, `packageBoundaryRules.${id}.implementationAdmission`),
    };
    const rule = packageBoundaryRules[id];
    if (rule.manifestContractId && !fixedFilenameContracts[rule.manifestContractId]) throw new Error(`Taxonomy v7 packageBoundaryRules.${id} references unknown manifest contract ${rule.manifestContractId}`);
    for (const contractId of rule.entryContractIds) if (!configurableEntryContracts[contractId]) throw new Error(`Taxonomy v7 packageBoundaryRules.${id} references unknown entry contract ${contractId}`);
    for (const contractId of rule.allowedFixedContractIds) if (!fixedFilenameContracts[contractId]) throw new Error(`Taxonomy v7 packageBoundaryRules.${id} references unknown fixed contract ${contractId}`);
    for (const kindId of rule.allowedFileKindIds) if (!fileKinds[kindId]) throw new Error(`Taxonomy v7 packageBoundaryRules.${id} references unknown file kind ${kindId}`);
    for (const kindId of rule.allowedDirectoryKindIds) if (!semanticDirectoryKinds[kindId]) throw new Error(`Taxonomy v7 packageBoundaryRules.${id} references unknown directory kind ${kindId}`);
  }
  const packageBoundaryProfiles: Record<string, PackageBoundaryProfile> = {};
  for (const [id, value] of Object.entries(boundaryProfileRows)) {
    const spec = requireRecord(value, `packageBoundaryProfiles.${id}`);
    requireExactKeys(spec, ["admission", "allowedFileKindIds", "allowedDirectoryKindIds", "allowedFixedContractIds", "glueGrammarId", "recursive", "uncertainRole", "implementationRole", "reason"], `packageBoundaryProfiles.${id}`);
    if (spec.admission !== "blocked-until-language-directory-registered" || spec.recursive !== true || spec.uncertainRole !== "problem" || spec.implementationRole !== "problem") throw new Error(`Taxonomy v7 packageBoundaryProfiles.${id} must remain fail-closed`);
    const glueGrammarId = requireString(spec.glueGrammarId, `packageBoundaryProfiles.${id}.glueGrammarId`);
    if (!packageGlueGrammar[glueGrammarId]) throw new Error(`Taxonomy v7 packageBoundaryProfiles.${id} references unknown grammar`);
    packageBoundaryProfiles[id] = { admission: "blocked-until-language-directory-registered", allowedFileKindIds: requireStringArray(spec.allowedFileKindIds, `packageBoundaryProfiles.${id}.allowedFileKindIds`), allowedDirectoryKindIds: requireStringArray(spec.allowedDirectoryKindIds, `packageBoundaryProfiles.${id}.allowedDirectoryKindIds`), allowedFixedContractIds: requireStringArray(spec.allowedFixedContractIds, `packageBoundaryProfiles.${id}.allowedFixedContractIds`), glueGrammarId, recursive: true, uncertainRole: "problem", implementationRole: "problem", reason: requireString(spec.reason, `packageBoundaryProfiles.${id}.reason`) };
  }
  if (Object.keys(packageBoundaryProfiles).length === 0) throw new Error("Taxonomy v7 packageBoundaryProfiles must not be empty");
  /** 🔖️ Each externally-mandated tool-config validator token is reserved for exactly one contract id, mirroring the pre-existing vitest-configuration/vitest-config-entry pinning. */
  const TOOL_CONFIG_VALIDATORS: Readonly<Record<string, string>> = { "vitest-configuration": "vitest-config-entry", "tool-config-tailwind": "tailwind-config", "tool-config-postcss": "postcss-config", "tool-config-eslint": "eslint-config", "tool-config-dependency-cruiser": "dependency-cruiser-config", "pytest-configuration": "root-pytest-config", "eslint-configuration": "root-eslint-config", "vscode-test-configuration": "vscode-test-cli-config" };
  const packageSourceDispositions: Record<string, PackageSourceDisposition> = {};
  for (const [id, value] of Object.entries(sourceDispositionRows)) {
    const spec = requireRecord(value, `packageSourceDispositions.${id}`);
    requireExactKeys(spec, ["contractKind", "disposition", "validator", ...(spec.grammarId === undefined ? [] : ["grammarId"]), "authority", "verification"], `packageSourceDispositions.${id}`);
    const configValidatorOwner = TOOL_CONFIG_VALIDATORS[spec.validator as string];
    if (spec.contractKind !== "fixed" && spec.contractKind !== "configurable" || spec.disposition !== "adapter-source" && spec.disposition !== "tool-metadata" || spec.validator !== "package-glue" && spec.validator !== "command-router" && configValidatorOwner === undefined || (configValidatorOwner !== undefined && id !== configValidatorOwner)) throw new Error(`Taxonomy v7 packageSourceDispositions.${id} is invalid`);
    const grammarId = spec.grammarId === undefined ? undefined : requireString(spec.grammarId, `packageSourceDispositions.${id}.grammarId`);
    if (grammarId !== undefined && !packageGlueGrammar[grammarId]) throw new Error(`Taxonomy v7 packageSourceDispositions.${id} references unknown grammar ${grammarId}`);
    packageSourceDispositions[id] = { contractKind: spec.contractKind, disposition: spec.disposition, validator: requireLiteral(spec.validator, `packageSourceDispositions.${id}.validator`, ["package-glue", "command-router", "vitest-configuration", "tool-config-vitest", "tool-config-tailwind", "tool-config-postcss", "tool-config-eslint", "tool-config-dependency-cruiser", "pytest-configuration", "eslint-configuration", "vscode-test-configuration"] as const), ...(grammarId === undefined ? {} : { grammarId }), authority: requireString(spec.authority, `packageSourceDispositions.${id}.authority`), verification: requireString(spec.verification, `packageSourceDispositions.${id}.verification`) };
  }
  if (Object.keys(packageSourceDispositions).length === 0) throw new Error("Taxonomy v7 packageSourceDispositions must not be empty");
  for (const [id, contract] of Object.entries(fixedFilenameContracts)) if (contract.scope.kind === "package-root" && !packageBoundaryRules[contract.scope.ecosystemId]) throw new Error(`Taxonomy v7 fixedFilenameContracts.${id} references unknown ecosystem ${contract.scope.ecosystemId}`);
  return { packageBoundaryRules, packageBoundaryProfiles, packageGlueGrammar, packageSourceDispositions };
}

/** 🚧 Parses the path exclusions and the opaque subtree set. */
export function parseExclusions({ exclusionRows, enforcement, semanticPathProjectionReferenceConsumerContracts, generatorContracts, pathMatcher }: {
  readonly exclusionRows: UnknownRecord;
  readonly enforcement: UnknownRecord;
  readonly semanticPathProjectionReferenceConsumerContracts: Record<string, SemanticPathProjectionReferenceConsumerContract>;
  readonly generatorContracts: Record<string, GeneratorContractSpec>;
  readonly pathMatcher: ReturnType<typeof createTaxonomyPathMatcher>;
}) {
  const pathExclusions: Record<string, { path: string; mode: "opaque"; reason: string }> = {};
  const exclusions: { id: string; path: string }[] = [];
  for (const [id, value] of Object.entries(exclusionRows)) {
    const spec = requireRecord(value, `pathExclusions.${id}`);
    if (spec.mode !== "opaque") throw new Error(`Taxonomy v7 pathExclusions.${id}.mode must be opaque`);
    const excludedPath = normalizeRelative(requireString(spec.path, `pathExclusions.${id}.path`));
    pathExclusions[id] = { path: excludedPath, mode: "opaque", reason: requireString(spec.reason, `pathExclusions.${id}.reason`) };
    exclusions.push({ id, path: excludedPath });
  }
  if (canonicalJson(Object.entries(pathExclusions).map(([id, spec]) => [id, spec.path])) !== canonicalJson(TAXONOMY_OPAQUE_PATH_EXCLUSIONS.map(([id, path]) => [id, normalizeRelative(path)]))) throw new Error(`Taxonomy v7 pathExclusions must contain exactly opaque ${TAXONOMY_OPAQUE_PATH_EXCLUSIONS.map(([, path]) => path).join(", ")}`);
  for (const id of requireStringArray(enforcement.opaquePathExclusionIds, "areaEnforcement.opaquePathExclusionIds")) {
    if (!pathExclusions[id]) throw new Error(`Taxonomy v7 areaEnforcement references unknown opaque exclusion ${id}`);
  }
  if (canonicalJson(enforcement.opaquePathExclusionIds) !== canonicalJson(TAXONOMY_OPAQUE_PATH_EXCLUSIONS.map(([id]) => id))) throw new Error(`Taxonomy v7 areaEnforcement must require ${TAXONOMY_OPAQUE_PATH_EXCLUSIONS.map(([id]) => id).join(", ")} in order`);
  const opaquePaths = Object.values(pathExclusions).map((entry) => entry.path);
  const crossesOpaque = (value: string): boolean => opaquePaths.some((opaque) => value === opaque || value.startsWith(`${opaque}/`) || opaque.startsWith(`${value}/`));
  for (const [id, contract] of Object.entries(semanticPathProjectionReferenceConsumerContracts)) {
    if (contract.sourcePathIdentities.some(crossesOpaque)) throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id} crosses an opaque path`);
    const pattern = new RegExp(contract.sourcePathPattern, "u");
    if (opaquePaths.some((opaque) => pattern.test(opaque) || pattern.test(`${opaque}/probe`))) throw new Error(`Taxonomy v7 semanticPathProjectionReferenceConsumerContracts.${id} admits an opaque path`);
  }
  for (const [id, contract] of Object.entries(generatorContracts)) {
    if (contract.ownerPath && crossesOpaque(contract.ownerPath)) throw new Error(`Taxonomy v7 generatorContracts.${id}.ownerPath crosses an opaque path`);
    for (const pattern of contract.inputPatterns) if (opaquePaths.some((opaque) => pathMatcher.matches(opaque, pattern) || pathMatcher.matches(`${opaque}/probe`, pattern))) throw new Error(`Taxonomy v7 generatorContracts.${id} input pattern admits an opaque path`);
    for (const output of contract.outputRoots) if (crossesOpaque(output.path)) throw new Error(`Taxonomy v7 generatorContracts.${id} output root crosses an opaque path`);
  }
  return { pathExclusions, exclusions };
}

/** 🧬 Assembles the taxonomy v7 schema object from the parsed rows. */
export function assembleTaxonomySchema({ root, fileKinds, semanticDirectoryKinds, fixedFilenameContracts, fixedFilenameRejectionContracts, fixedDirectoryContracts, configurableEntryContracts, fileKindResolutionRules, scopedFileKinds, semanticDirectoryMemberKinds, semanticProjectedMemberKinds, semanticPathProjectionProfileRenderers, semanticDescendantContracts, semanticPathProjectionCatalogContracts, semanticPathProjectionContracts, semanticOwnedFileProjectionContracts, semanticPathProjectionReferenceConsumerContracts, mutationCatalogProjection, generatorContracts, ecosystems, packageBoundaryRules, packageBoundaryProfiles, packageGlueGrammar, packageSourceDispositions, pathExclusions, collision, enforcement }: {
  readonly root: UnknownRecord;
  readonly fileKinds: Record<string, FileKindSpec>;
  readonly semanticDirectoryKinds: Record<string, SemanticDirectoryKindSpec>;
  readonly fixedFilenameContracts: Record<string, FixedFilenameContract>;
  readonly fixedFilenameRejectionContracts: Record<string, FixedFilenameRejectionContract>;
  readonly fixedDirectoryContracts: Record<string, FixedDirectoryContract>;
  readonly configurableEntryContracts: Record<string, ConfigurableEntryContract>;
  readonly fileKindResolutionRules: Record<string, FileKindResolutionRuleSpec>;
  readonly scopedFileKinds: Record<string, ScopedFileKindSpec>;
  readonly semanticDirectoryMemberKinds: Record<string, SemanticDirectoryMemberKindSpec>;
  readonly semanticProjectedMemberKinds: Record<string, SemanticProjectedMemberKindSpec>;
  readonly semanticPathProjectionProfileRenderers: Record<string, SemanticPathProjectionProfileRenderer>;
  readonly semanticDescendantContracts: Record<string, SemanticDescendantContract>;
  readonly semanticPathProjectionCatalogContracts: Record<string, SemanticPathProjectionCatalogContract>;
  readonly semanticPathProjectionContracts: Record<string, SemanticPathProjectionContract>;
  readonly semanticOwnedFileProjectionContracts: Record<string, SemanticOwnedFileProjectionContract>;
  readonly semanticPathProjectionReferenceConsumerContracts: Record<string, SemanticPathProjectionReferenceConsumerContract>;
  readonly mutationCatalogProjection: MutationCatalogProjectionContractIds;
  readonly generatorContracts: Record<string, GeneratorContractSpec>;
  readonly ecosystems: Record<string, EcosystemSpec>;
  readonly packageBoundaryRules: Record<string, PackageBoundaryRule>;
  readonly packageBoundaryProfiles: Record<string, PackageBoundaryProfile>;
  readonly packageGlueGrammar: Record<string, PackageGlueGrammar>;
  readonly packageSourceDispositions: Record<string, PackageSourceDisposition>;
  readonly pathExclusions: Record<string, { path: string; mode: "opaque"; reason: string; }>;
  readonly collision: UnknownRecord;
  readonly enforcement: UnknownRecord;
}) {
  const schema: TaxonomyV7 = {
    schemaVersion: 7,
    windowEmptyFacetFileKindId: requireString(root.windowEmptyFacetFileKindId, "windowEmptyFacetFileKindId"),
    fileKinds,
    semanticDirectoryKinds,
    fixedFilenameContracts,
    fixedFilenameRejectionContracts,
    fixedDirectoryContracts,
    configurableEntryContracts,
    fileKindResolutionRules,
    scopedFileKinds,
    semanticDirectoryMemberKinds,
    semanticProjectedMemberKinds,
    semanticPathProjectionProfileRenderers,
    semanticDescendantContracts,
    semanticPathProjectionCatalogContracts,
    semanticPathProjectionContracts,
    semanticOwnedFileProjectionContracts,
    semanticPackageProjectionContracts: root.semanticPackageProjectionContracts as DiscoveryTaxonomy["semanticPackageProjectionContracts"],
    semanticPathProjectionReferenceConsumerContracts,
    mutationCatalogProjection,
    generatorContracts,
    ecosystems,
    packageBoundaryRules,
    packageBoundaryProfiles,
    packageGlueGrammar,
    packageSourceDispositions,
    pathExclusions,
    unicodeNormalization: { form: "NFC", caseFold: "lower", locale: "und" },
    variationSelectorPolicy: { selector: "\uFE0F", requiredAfterEmoji: true, comparison: "ignore-selector" },
    collisionPolicy: {
      comparisons: collision.comparisons as TaxonomyV7["collisionPolicy"]["comparisons"],
      maxPathUtf16Units: collision.maxPathUtf16Units as number,
      maxComponentUtf8Bytes: collision.maxComponentUtf8Bytes as number,
      rejectWindowsReservedNames: collision.rejectWindowsReservedNames === true,
      rejectTrailingDotsAndSpaces: collision.rejectTrailingDotsAndSpaces === true,
    },
    areaEnforcement: { requiredState: "clean", undeclaredAreas: "enforce", opaquePathExclusionIds: [...(enforcement.opaquePathExclusionIds as string[])] },
  };
  return { schema };
}
