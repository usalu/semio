/** 🧱️ The taxonomy v7 prelude, file and directory kinds, fixed filename and directory contracts, rejections, configurable entries and resolution rules. */
import { requireExactKeys, requireRecord, requireString, requireStringArray, type UnknownRecord } from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { validateTaxonomy } from "../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import type { Taxonomy as DiscoveryTaxonomy } from "../../../🔍️discovery/📑️taxonomy-document/🟦️.ts";
import { createTaxonomyPathMatcher, parseFixedDirectoryContractSetScope, parseNamedFixedDirectoryContractSetScope } from "../../../🔍️discovery/🎯️taxonomy-path-matching/🟦️.ts";
import { canonicalJson } from "../../../🧾️serialization/🔣️json/🟦️.ts";
import type { ConfigurableEntryContract, FileKindResolutionRuleSpec, FileKindSpec, FixedContractScope, FixedDirectoryContract, FixedFilenameContract, FixedFilenameRejectionContract, ScopedFileKindSpec, SemanticDirectoryKindSpec } from "../📐️kind-specs/🟦️.ts";
import { normalizeRelative } from "../../🛣️path/🟦️.ts";
import { fixedExpiry } from "../🧰️taxonomy-parse-helpers/🟦️.ts";

/** 🔰 Reads the root rows and validates the unicode, selector, collision and enforcement policies. */
export function parseTaxonomyPrelude({ raw, path }: {
  readonly raw: unknown;
  readonly path: string;
}) {
  const root = requireRecord(raw, "root");
  if (root.schemaVersion !== 7) throw new Error(`Taxonomy schemaVersion must be 7 at ${path}`);
  const discoveryProblems = validateTaxonomy(root as unknown as DiscoveryTaxonomy);
  if (discoveryProblems.length > 0) throw new Error(`Taxonomy v7 discovery contract validation failed at ${path}: ${discoveryProblems.join(" | ")}`);
  const pathMatcher = createTaxonomyPathMatcher();
  function validatedContractPattern(value: unknown, name: string, exactBasename: boolean): string {
    const pattern = requireString(value, name);
    if (pattern !== pattern.normalize("NFC") || pattern.startsWith("/") || pattern.endsWith("/") || pattern.includes("\\") || pattern.includes("//") || pattern.includes("\u0000")) throw new Error(`Taxonomy v7 ${name} must be one NFC workspace-relative POSIX pattern`);
    if (/[{}]/u.test(pattern) || /^!/u.test(pattern) || /[!@+?*]\(/u.test(pattern)) throw new Error(`Taxonomy v7 ${name} uses unsupported glob syntax`);
    for (const segment of pattern.split("/")) {
      if (segment.includes("**") && segment !== "**") throw new Error(`Taxonomy v7 ${name} may use ** only as a whole segment`);
      for (const match of segment.matchAll(/\[([^\]]*)\]/gu)) if (!/^[A-Za-z0-9-]+$/u.test(requireString(match[1],"character class")) || /^[!^]/u.test(requireString(match[1],"character class"))) throw new Error(`Taxonomy v7 ${name} has an invalid character class`);
      if ((segment.match(/\[/gu)?.length ?? 0) !== (segment.match(/\]/gu)?.length ?? 0)) throw new Error(`Taxonomy v7 ${name} has an unclosed character class`);
    }
    const filename = pattern.slice(pattern.lastIndexOf("/") + 1);
    if (exactBasename && /[*?\[\]{}]/u.test(filename)) throw new Error(`Taxonomy v7 ${name} must end in one exact literal basename`);
    pathMatcher.matches("", pattern);
    return pattern;
  }
  const fileKindRows = requireRecord(root.fileKinds, "fileKinds");
  const directoryKindRows = requireRecord(root.semanticDirectoryKinds, "semanticDirectoryKinds");
  const fixedRows = requireRecord(root.fixedFilenameContracts, "fixedFilenameContracts");
  const fixedRejectionRows = requireRecord(root.fixedFilenameRejectionContracts, "fixedFilenameRejectionContracts");
  const fixedDirectoryRows = requireRecord(root.fixedDirectoryContracts, "fixedDirectoryContracts");
  const configurableRows = requireRecord(root.configurableEntryContracts, "configurableEntryContracts");
  const fileResolutionRows = requireRecord(root.fileKindResolutionRules, "fileKindResolutionRules");
  const scopedFileRows = requireRecord(root.scopedFileKinds, "scopedFileKinds");
  const directoryMemberRows = requireRecord(root.semanticDirectoryMemberKinds, "semanticDirectoryMemberKinds");
  const projectedMemberRows = requireRecord(root.semanticProjectedMemberKinds, "semanticProjectedMemberKinds");
  const projectionRendererRows = requireRecord(root.semanticPathProjectionProfileRenderers, "semanticPathProjectionProfileRenderers");
  const descendantContractRows = requireRecord(root.semanticDescendantContracts, "semanticDescendantContracts");
  const projectionCatalogRows = requireRecord(root.semanticPathProjectionCatalogContracts, "semanticPathProjectionCatalogContracts");
  const projectionRows = requireRecord(root.semanticPathProjectionContracts, "semanticPathProjectionContracts");
  const ownedFileProjectionRows = requireRecord(root.semanticOwnedFileProjectionContracts, "semanticOwnedFileProjectionContracts");
  const projectionConsumerRows = requireRecord(root.semanticPathProjectionReferenceConsumerContracts, "semanticPathProjectionReferenceConsumerContracts");
  const mutationCatalogProjectionRow = requireRecord(root.mutationCatalogProjection, "mutationCatalogProjection");
  const generatorRows = requireRecord(root.generatorContracts, "generatorContracts");
  const ecosystemRows = requireRecord(root.ecosystems, "ecosystems");
  const boundaryRows = requireRecord(root.packageBoundaryRules, "packageBoundaryRules");
  const boundaryProfileRows = requireRecord(root.packageBoundaryProfiles, "packageBoundaryProfiles");
  const grammarRows = requireRecord(root.packageGlueGrammar, "packageGlueGrammar");
  const sourceDispositionRows = requireRecord(root.packageSourceDispositions, "packageSourceDispositions");
  const exclusionRows = requireRecord(root.pathExclusions, "pathExclusions");
  const unicode = requireRecord(root.unicodeNormalization, "unicodeNormalization");
  const selector = requireRecord(root.variationSelectorPolicy, "variationSelectorPolicy");
  const collision = requireRecord(root.collisionPolicy, "collisionPolicy");
  const enforcement = requireRecord(root.areaEnforcement, "areaEnforcement");
  if (unicode.form !== "NFC" || unicode.caseFold !== "lower" || unicode.locale !== "und") throw new Error("Taxonomy v7 unicodeNormalization must select NFC/lower/und");
  if (selector.selector !== "\uFE0F" || selector.requiredAfterEmoji !== true || selector.comparison !== "ignore-selector") throw new Error("Taxonomy v7 variationSelectorPolicy is not canonical");
  const requiredComparisons = ["byte", "nfc", "case-fold", "vs16-fold", "same-kind"];
  if (canonicalJson(collision.comparisons) !== canonicalJson(requiredComparisons) || !Number.isSafeInteger(collision.maxPathUtf16Units) || (collision.maxPathUtf16Units as number) < 1 || !Number.isSafeInteger(collision.maxComponentUtf8Bytes) || (collision.maxComponentUtf8Bytes as number) < 1 || collision.rejectWindowsReservedNames !== true || collision.rejectTrailingDotsAndSpaces !== true) throw new Error("Taxonomy v7 collisionPolicy is incomplete");
  if (enforcement.requiredState !== "clean" || enforcement.undeclaredAreas !== "enforce") throw new Error("Taxonomy v7 areaEnforcement must enforce clean undeclared areas");
  return { fileKindRows, directoryKindRows, fixedRows, root, validatedContractPattern, fixedDirectoryRows, fixedRejectionRows, configurableRows, fileResolutionRows, scopedFileRows, directoryMemberRows, projectedMemberRows, projectionRendererRows, descendantContractRows, projectionCatalogRows, projectionRows, ownedFileProjectionRows, generatorRows, projectionConsumerRows, mutationCatalogProjectionRow, ecosystemRows, grammarRows, boundaryRows, boundaryProfileRows, sourceDispositionRows, exclusionRows, enforcement, pathMatcher, collision };
}

/** 🗂 Parses the file kinds and the semantic directory kinds. */
export function parseFileAndDirectoryKinds({ fileKindRows, directoryKindRows }: {
  readonly fileKindRows: UnknownRecord;
  readonly directoryKindRows: UnknownRecord;
}) {
  const fileKinds: Record<string, FileKindSpec> = {};
  for (const [id, value] of Object.entries(fileKindRows)) {
    const spec = requireRecord(value, `fileKinds.${id}`);
    const emoji = requireString(spec.emoji, `fileKinds.${id}.emoji`).normalize("NFC");
    const extensionChains = requireStringArray(spec.extensionChains, `fileKinds.${id}.extensionChains`);
    if (extensionChains.length === 0 || extensionChains.some((chain) => !chain.startsWith("."))) throw new Error(`Taxonomy v7 fileKinds.${id}.extensionChains must contain dotted chains`);
    fileKinds[id] = { emoji, extensionChains: [...new Set(extensionChains)].sort((a, b) => b.length - a.length || a.localeCompare(b)), role: requireString(spec.role, `fileKinds.${id}.role`) };
  }
  if (Object.keys(fileKinds).length === 0) throw new Error("Taxonomy v7 fileKinds must not be empty");

  const semanticDirectoryKinds: Record<string, SemanticDirectoryKindSpec> = {};
  for (const [id, value] of Object.entries(directoryKindRows)) {
    const spec = requireRecord(value, `semanticDirectoryKinds.${id}`);
    const emoji = requireString(spec.emoji, `semanticDirectoryKinds.${id}.emoji`).normalize("NFC");
    const slugPattern = requireString(spec.slugPattern, `semanticDirectoryKinds.${id}.slugPattern`);
    new RegExp(slugPattern, "u");
    if (typeof spec.allowEmojiOnly !== "boolean") throw new Error(`Taxonomy v7 semanticDirectoryKinds.${id}.allowEmojiOnly must be boolean`);
    if (spec.inferWithoutEmoji !== undefined && typeof spec.inferWithoutEmoji !== "boolean") throw new Error(`Taxonomy v7 semanticDirectoryKinds.${id}.inferWithoutEmoji must be boolean when present`);
    if (spec.projectionOnly !== undefined && typeof spec.projectionOnly !== "boolean") throw new Error(`Taxonomy v7 semanticDirectoryKinds.${id}.projectionOnly must be boolean when present`);
    semanticDirectoryKinds[id] = { emoji, slugPattern, allowEmojiOnly: spec.allowEmojiOnly, ...(spec.inferWithoutEmoji === undefined ? {} : { inferWithoutEmoji: spec.inferWithoutEmoji }), ...(spec.projectionOnly === undefined ? {} : { projectionOnly: spec.projectionOnly }), ...(spec.parentKindIds === undefined ? {} : { parentKindIds: requireStringArray(spec.parentKindIds, `semanticDirectoryKinds.${id}.parentKindIds`) }) };
  }
  if (Object.keys(semanticDirectoryKinds).length === 0) throw new Error("Taxonomy v7 semanticDirectoryKinds must not be empty");
  return { semanticDirectoryKinds, fileKinds };
}

/** 📌 Parses the fixed filename and fixed directory contracts. */
export function parseFixedContracts({ fixedRows, root, semanticDirectoryKinds, validatedContractPattern, fixedDirectoryRows }: {
  readonly fixedRows: UnknownRecord;
  readonly root: UnknownRecord;
  readonly semanticDirectoryKinds: Record<string, SemanticDirectoryKindSpec>;
  readonly validatedContractPattern: (value: unknown, name: string, exactBasename: boolean) => string;
  readonly fixedDirectoryRows: UnknownRecord;
}) {
  const fixedFilenameContracts: Record<string, FixedFilenameContract> = {};
  for (const [id, value] of Object.entries(fixedRows)) {
    const spec = requireRecord(value, `fixedFilenameContracts.${id}`);
    if (spec.configurability !== "unconfigurable") throw new Error(`Taxonomy v7 fixedFilenameContracts.${id}.configurability must be unconfigurable`);
    const inputScopeRow = requireRecord(spec.scope, `fixedFilenameContracts.${id}.scope`);
    const namedScope = inputScopeRow.kind === "named-fixed-directory-contract-set" ? parseNamedFixedDirectoryContractSetScope(inputScopeRow, root.fixedDirectoryContracts as DiscoveryTaxonomy["fixedDirectoryContracts"], (root.fixedDirectoryContractSets ?? {}) as NonNullable<DiscoveryTaxonomy["fixedDirectoryContractSets"]>) : undefined;
    const scopeRow: UnknownRecord = namedScope ? { kind: namedScope.kind, fixedDirectoryContractIds: [...namedScope.fixedDirectoryContractIds] } : inputScopeRow;
    const scopeKind = requireString(scopeRow.kind, `fixedFilenameContracts.${id}.scope.kind`) as FixedContractScope["kind"];
    if (!["exact-path", "repository-root", "package-root", "directory-kind", "fixed-directory-contract", "fixed-directory-contract-set", "sibling-fixed-filename-contract", "path-pattern"].includes(scopeKind)) throw new Error(`Taxonomy v7 fixedFilenameContracts.${id}.scope.kind is invalid`);
    const scope: FixedContractScope = scopeKind === "exact-path"
      ? (requireExactKeys(scopeRow, ["kind", "path"], `fixedFilenameContracts.${id}.scope`), { kind: "exact-path", path: normalizeRelative(requireString(scopeRow.path, `fixedFilenameContracts.${id}.scope.path`)) })
      : scopeKind === "package-root"
        ? (requireExactKeys(scopeRow, ["kind", "ecosystemId"], `fixedFilenameContracts.${id}.scope`), { kind: "package-root", ecosystemId: requireString(scopeRow.ecosystemId, `fixedFilenameContracts.${id}.scope.ecosystemId`) })
        : scopeKind === "directory-kind"
          ? (requireExactKeys(scopeRow, ["kind", "directoryKindId"], `fixedFilenameContracts.${id}.scope`), { kind: "directory-kind", directoryKindId: requireString(scopeRow.directoryKindId, `fixedFilenameContracts.${id}.scope.directoryKindId`) })
          : scopeKind === "fixed-directory-contract"
            ? (requireExactKeys(scopeRow, ["kind", "fixedDirectoryContractId"], `fixedFilenameContracts.${id}.scope`), { kind: "fixed-directory-contract", fixedDirectoryContractId: requireString(scopeRow.fixedDirectoryContractId, `fixedFilenameContracts.${id}.scope.fixedDirectoryContractId`) })
            : scopeKind === "fixed-directory-contract-set"
              ? parseFixedDirectoryContractSetScope(scopeRow, root.fixedDirectoryContracts as DiscoveryTaxonomy["fixedDirectoryContracts"])
            : scopeKind === "sibling-fixed-filename-contract"
              ? (requireExactKeys(scopeRow, ["kind", "fixedFilenameContractId"], `fixedFilenameContracts.${id}.scope`), { kind: "sibling-fixed-filename-contract", fixedFilenameContractId: requireString(scopeRow.fixedFilenameContractId, `fixedFilenameContracts.${id}.scope.fixedFilenameContractId`) })
          : (requireExactKeys(scopeRow, ["kind"], `fixedFilenameContracts.${id}.scope`), { kind: scopeKind });
    if (scope.kind === "directory-kind" && !semanticDirectoryKinds[scope.directoryKindId]) throw new Error(`Taxonomy v7 fixedFilenameContracts.${id}.scope.directoryKindId is invalid`);
    fixedFilenameContracts[id] = {
      pathPattern: validatedContractPattern(spec.pathPattern, `fixedFilenameContracts.${id}.pathPattern`, true),
      authority: requireString(spec.authority, `fixedFilenameContracts.${id}.authority`),
      reason: requireString(spec.reason, `fixedFilenameContracts.${id}.reason`),
      configurability: "unconfigurable",
      scope,
      verification: requireString(spec.verification, `fixedFilenameContracts.${id}.verification`),
      expires: fixedExpiry(spec.expires, `fixedFilenameContracts.${id}.expires`),
    };
  }

  const fixedDirectoryContracts: Record<string, FixedDirectoryContract> = {};
  for (const [id, value] of Object.entries(fixedDirectoryRows)) {
    const spec = requireRecord(value, `fixedDirectoryContracts.${id}`);
    if (spec.configurability !== "unconfigurable") throw new Error(`Taxonomy v7 fixedDirectoryContracts.${id}.configurability must be unconfigurable`);
    const scopeRow = requireRecord(spec.scope, `fixedDirectoryContracts.${id}.scope`);
    const scopeKind = requireString(scopeRow.kind, `fixedDirectoryContracts.${id}.scope.kind`);
    if (!["exact-path", "repository-root", "directory-kind", "path-pattern"].includes(scopeKind)) throw new Error(`Taxonomy v7 fixedDirectoryContracts.${id}.scope.kind is invalid`);
    const scope: FixedDirectoryContract["scope"] = scopeKind === "exact-path"
      ? (requireExactKeys(scopeRow, ["kind", "path"], `fixedDirectoryContracts.${id}.scope`), { kind: "exact-path", path: normalizeRelative(requireString(scopeRow.path, `fixedDirectoryContracts.${id}.scope.path`)) })
      : scopeKind === "directory-kind"
        ? (requireExactKeys(scopeRow, ["kind", "directoryKindId"], `fixedDirectoryContracts.${id}.scope`), { kind: "directory-kind", directoryKindId: requireString(scopeRow.directoryKindId, `fixedDirectoryContracts.${id}.scope.directoryKindId`) })
        : (requireExactKeys(scopeRow, ["kind"], `fixedDirectoryContracts.${id}.scope`), { kind: scopeKind as "repository-root" | "path-pattern" });
    if (scope.kind === "directory-kind" && !semanticDirectoryKinds[scope.directoryKindId]) throw new Error(`Taxonomy v7 fixedDirectoryContracts.${id}.scope.directoryKindId is invalid`);
    fixedDirectoryContracts[id] = {
      pathPattern: validatedContractPattern(spec.pathPattern, `fixedDirectoryContracts.${id}.pathPattern`, false),
      authority: requireString(spec.authority, `fixedDirectoryContracts.${id}.authority`),
      reason: requireString(spec.reason, `fixedDirectoryContracts.${id}.reason`),
      configurability: "unconfigurable",
      scope,
      verification: requireString(spec.verification, `fixedDirectoryContracts.${id}.verification`),
      expires: fixedExpiry(spec.expires, `fixedDirectoryContracts.${id}.expires`),
    };
  }
  if (Object.keys(fixedDirectoryContracts).length === 0) throw new Error("Taxonomy v7 fixedDirectoryContracts must not be empty");
  for (const [id, contract] of Object.entries(fixedFilenameContracts)) {
    if (contract.scope.kind === "fixed-directory-contract" && !fixedDirectoryContracts[contract.scope.fixedDirectoryContractId]) throw new Error(`Taxonomy v7 fixedFilenameContracts.${id}.scope.fixedDirectoryContractId is invalid`);
    if (contract.scope.kind === "sibling-fixed-filename-contract" && !fixedFilenameContracts[contract.scope.fixedFilenameContractId]) throw new Error(`Taxonomy v7 fixedFilenameContracts.${id}.scope.fixedFilenameContractId is invalid`);
  }
  return { fixedDirectoryContracts, fixedFilenameContracts };
}

/** 🚫 Parses the fixed filename rejections, configurable entries, file kind resolution rules and scoped file kinds. */
export function parseRejectionsAndResolution({ fixedRejectionRows, configurableRows, fileKinds, fileResolutionRows, validatedContractPattern, semanticDirectoryKinds, scopedFileRows }: {
  readonly fixedRejectionRows: UnknownRecord;
  readonly configurableRows: UnknownRecord;
  readonly fileKinds: Record<string, FileKindSpec>;
  readonly fileResolutionRows: UnknownRecord;
  readonly validatedContractPattern: (value: unknown, name: string, exactBasename: boolean) => string;
  readonly semanticDirectoryKinds: Record<string, SemanticDirectoryKindSpec>;
  readonly scopedFileRows: UnknownRecord;
}) {
  const fixedFilenameRejectionContracts: Record<string, FixedFilenameRejectionContract> = {};
  const rejectedFixedPaths = new Set<string>();
  for (const [id, value] of Object.entries(fixedRejectionRows)) {
    const spec = requireRecord(value, `fixedFilenameRejectionContracts.${id}`);
    requireExactKeys(spec, ["sourcePathIdentities", "disposition", "reason"], `fixedFilenameRejectionContracts.${id}`);
    if (spec.disposition !== "normalize" && spec.disposition !== "relocate") throw new Error(`Taxonomy v7 fixedFilenameRejectionContracts.${id}.disposition is invalid`);
    const sourcePathIdentities = requireStringArray(spec.sourcePathIdentities, `fixedFilenameRejectionContracts.${id}.sourcePathIdentities`).map(normalizeRelative);
    if (sourcePathIdentities.length === 0 || sourcePathIdentities.some((path) => rejectedFixedPaths.has(path))) throw new Error(`Taxonomy v7 fixedFilenameRejectionContracts.${id}.sourcePathIdentities are empty or duplicated`);
    for (const path of sourcePathIdentities) rejectedFixedPaths.add(path);
    fixedFilenameRejectionContracts[id] = { sourcePathIdentities, disposition: spec.disposition, reason: requireString(spec.reason, `fixedFilenameRejectionContracts.${id}.reason`) };
  }
  if (Object.keys(fixedFilenameRejectionContracts).length === 0) throw new Error("Taxonomy v7 fixedFilenameRejectionContracts must not be empty");

  const configurableEntryContracts: Record<string, ConfigurableEntryContract> = {};
  for (const [id, value] of Object.entries(configurableRows)) {
    const spec = requireRecord(value, `configurableEntryContracts.${id}`);
    const fileKindId = requireString(spec.fileKindId, `configurableEntryContracts.${id}.fileKindId`);
    if (!fileKinds[fileKindId]) throw new Error(`Taxonomy v7 configurableEntryContracts.${id} references unknown file kind ${fileKindId}`);
    configurableEntryContracts[id] = {
      filename: requireString(spec.filename, `configurableEntryContracts.${id}.filename`),
      fileKindId,
      ecosystemId: requireString(spec.ecosystemId, `configurableEntryContracts.${id}.ecosystemId`),
      role: requireString(spec.role, `configurableEntryContracts.${id}.role`),
      configurationSources: requireStringArray(spec.configurationSources, `configurableEntryContracts.${id}.configurationSources`),
    };
  }

  const fileKindResolutionRules: Record<string, FileKindResolutionRuleSpec> = {};
  for (const [id, value] of Object.entries(fileResolutionRows)) {
    const spec = requireRecord(value, `fileKindResolutionRules.${id}`);
    const extensionChain = requireString(spec.extensionChain, `fileKindResolutionRules.${id}.extensionChain`);
    const fileKindId = requireString(spec.fileKindId, `fileKindResolutionRules.${id}.fileKindId`);
    if (!fileKinds[fileKindId]?.extensionChains.includes(extensionChain)) throw new Error(`Taxonomy v7 fileKindResolutionRules.${id} does not reference an owned extension chain`);
    if (!Number.isSafeInteger(spec.priority)) throw new Error(`Taxonomy v7 fileKindResolutionRules.${id}.priority must be an integer`);
    const filenamePattern = typeof spec.filenamePattern === "string" ? spec.filenamePattern : undefined;
    const pathPattern = typeof spec.pathPattern === "string" ? validatedContractPattern(spec.pathPattern, `fileKindResolutionRules.${id}.pathPattern`, false) : undefined;
    if (filenamePattern) new RegExp(filenamePattern, "u");
    const parentKindIds = spec.parentKindIds === undefined ? undefined : requireStringArray(spec.parentKindIds, `fileKindResolutionRules.${id}.parentKindIds`);
    const ancestorKindIds = spec.ancestorKindIds === undefined ? undefined : requireStringArray(spec.ancestorKindIds, `fileKindResolutionRules.${id}.ancestorKindIds`);
    for (const kindId of [...(parentKindIds ?? []), ...(ancestorKindIds ?? [])]) if (!semanticDirectoryKinds[kindId]) throw new Error(`Taxonomy v7 fileKindResolutionRules.${id} references unknown directory kind ${kindId}`);
    fileKindResolutionRules[id] = { extensionChain, fileKindId, priority: spec.priority as number, filenamePattern, pathPattern, parentKindIds, ancestorKindIds };
  }
  if (Object.keys(fileKindResolutionRules).length === 0) throw new Error("Taxonomy v7 fileKindResolutionRules must not be empty");

  const scopedFileKinds: Record<string, ScopedFileKindSpec> = {};
  for (const [id, value] of Object.entries(scopedFileRows)) {
    const spec = requireRecord(value, `scopedFileKinds.${id}`);
    const extensionChains = requireStringArray(spec.extensionChains, `scopedFileKinds.${id}.extensionChains`);
    if (extensionChains.length === 0 || extensionChains.some((chain) => !chain.startsWith("."))) throw new Error(`Taxonomy v7 scopedFileKinds.${id}.extensionChains must contain dotted chains`);
    const sourceFilenamePattern = requireString(spec.sourceFilenamePattern, `scopedFileKinds.${id}.sourceFilenamePattern`);
    new RegExp(sourceFilenamePattern, "u");
    const role = requireString(spec.role, `scopedFileKinds.${id}.role`);
    if (!["source", "schema", "specification", "configuration", "documentation", "test", "asset", "generated", "marker", "evidence"].includes(role)) throw new Error(`Taxonomy v7 scopedFileKinds.${id}.role is invalid`);
    const parentDirectoryKindId = spec.parentDirectoryKindId === undefined ? undefined : requireString(spec.parentDirectoryKindId, `scopedFileKinds.${id}.parentDirectoryKindId`);
    if (parentDirectoryKindId && !semanticDirectoryKinds[parentDirectoryKindId]) throw new Error(`Taxonomy v7 scopedFileKinds.${id} references unknown parent directory kind ${parentDirectoryKindId}`);
    scopedFileKinds[id] = {
      pathPattern: validatedContractPattern(spec.pathPattern, `scopedFileKinds.${id}.pathPattern`, false),
      parentDirectoryKindId,
      emoji: requireString(spec.emoji, `scopedFileKinds.${id}.emoji`).normalize("NFC"),
      extensionChains: [...new Set(extensionChains)].sort((left, right) => right.length - left.length || left.localeCompare(right)),
      role,
      sourceFilenamePattern,
      authority: requireString(spec.authority, `scopedFileKinds.${id}.authority`),
      reason: requireString(spec.reason, `scopedFileKinds.${id}.reason`),
      verification: requireString(spec.verification, `scopedFileKinds.${id}.verification`),
      expires: fixedExpiry(spec.expires, `scopedFileKinds.${id}.expires`),
    };
  }
  return { configurableEntryContracts, fixedFilenameRejectionContracts, fileKindResolutionRules, scopedFileKinds };
}
