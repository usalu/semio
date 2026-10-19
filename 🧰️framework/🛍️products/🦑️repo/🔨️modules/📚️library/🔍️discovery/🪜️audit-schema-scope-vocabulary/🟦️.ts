import { taxonomyPathPatternMatches } from "../🎯️taxonomy-path-matching/🟦️.ts";
import type { TaxonomyAudit } from "../🪣️taxonomy-audit-context/🟦️.ts";

/** 🪜️ Validates the schema scope-owner, export-resolution and dialect vocabulary families. */
export function validateSchemaScopeVocabulary(audit: TaxonomyAudit): string[] {
  const { taxonomy, semanticDirectoryKindId } = audit;
  const problems: string[] = [];
  const dialect = taxonomy.mutationPayloadSchemaAuthority?.jsonSchemaDialect;
  if (typeof taxonomy.schemaJsonDialect !== "string" || taxonomy.schemaJsonDialect !== dialect) problems.push("schemaJsonDialect must be the single dialect mutationPayloadSchemaAuthority already declares.");
  const levels = taxonomy.schemaScopeOwnerLevels;
  if (!levels || typeof levels !== "object") problems.push("schemaScopeOwnerLevels must declare the eligible schema scope-owner levels.");
  else {
    if (levels.facetDirName !== "🧬️schema" || semanticDirectoryKindId(levels.facetDirName, taxonomy) !== levels.directoryKindId) problems.push("schemaScopeOwnerLevels must bind the registered 🧬️schema directory kind.");
    if (typeof taxonomy.schemaDefaultFacetKind !== "string" || !taxonomy.schemaFacetKinds?.[taxonomy.schemaDefaultFacetKind]) problems.push("schemaDefaultFacetKind must name a declared schemaFacetKinds entry.");
    else if ((taxonomy.schemaFacetKinds[taxonomy.schemaDefaultFacetKind]?.facetPathIdentities ?? []).length !== 0) problems.push("schemaDefaultFacetKind must not claim exact facet modules; it is the fallback kind.");
    if (!levels.levels || typeof levels.levels !== "object" || Object.keys(levels.levels).length === 0) problems.push("schemaScopeOwnerLevels.levels must be a non-empty object.");
    else for (const [levelId, level] of Object.entries(levels.levels)) {
      if (!/^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$/u.test(levelId)) problems.push(`schemaScopeOwnerLevels.levels[${JSON.stringify(levelId)}] must be a kebab identifier.`);
      if (!Array.isArray(level.pathPatterns) || level.pathPatterns.length === 0 || level.pathPatterns.some((value) => typeof value !== "string" || !value)) problems.push(`schemaScopeOwnerLevels.levels[${JSON.stringify(levelId)}].pathPatterns must be a non-empty string array.`);
      else for (const value of level.pathPatterns) try { void taxonomyPathPatternMatches("", value); } catch { problems.push(`schemaScopeOwnerLevels.levels[${JSON.stringify(levelId)}] has an invalid path pattern ${JSON.stringify(value)}.`); }
      if (typeof level.reason !== "string" || level.reason.length < 16) problems.push(`schemaScopeOwnerLevels.levels[${JSON.stringify(levelId)}].reason must state why the level owns contracts.`);
    }
    if (!Array.isArray(levels.excludedOwnerPathPatterns) || levels.excludedOwnerPathPatterns.length === 0) problems.push("schemaScopeOwnerLevels.excludedOwnerPathPatterns must list the never-eligible owner shapes.");
    else for (const value of levels.excludedOwnerPathPatterns) try { void taxonomyPathPatternMatches("", value); } catch { problems.push(`schemaScopeOwnerLevels.excludedOwnerPathPatterns has an invalid path pattern ${JSON.stringify(value)}.`); }
    if (!Array.isArray(levels.fixtureOwnerPathPatterns) || levels.fixtureOwnerPathPatterns.length === 0) problems.push("schemaScopeOwnerLevels.fixtureOwnerPathPatterns must list the test and fixture collection roots.");
    else for (const value of levels.fixtureOwnerPathPatterns) try { void taxonomyPathPatternMatches("", value); } catch { problems.push(`schemaScopeOwnerLevels.fixtureOwnerPathPatterns has an invalid path pattern ${JSON.stringify(value)}.`); }
    if (Array.isArray(levels.fixtureOwnerPathPatterns) && Array.isArray(levels.excludedOwnerPathPatterns) && levels.fixtureOwnerPathPatterns.some((value) => levels.excludedOwnerPathPatterns.includes(value))) problems.push("schemaScopeOwnerLevels.fixtureOwnerPathPatterns and excludedOwnerPathPatterns must be disjoint: a collection is recognized by its own ancestors with the module-member carve-out, an excluded owner shape by an exact pattern.");
    if (typeof levels.fixtureOwnerReason !== "string" || levels.fixtureOwnerReason.length < 16) problems.push("schemaScopeOwnerLevels.fixtureOwnerReason must state why a collection is never a scope owner and why the schema documents inside it are findings.");
    if (typeof levels.moduleMemberDirName !== "string" || !(taxonomy.pluginChildDirs ?? []).includes(levels.moduleMemberDirName)) problems.push("schemaScopeOwnerLevels.moduleMemberDirName must be the declared module-member directory a collection pattern never claims.");
    if (typeof levels.mutationsFacetDirName !== "string" || !(taxonomy.schemaChildDirs ?? []).includes(levels.mutationsFacetDirName)) problems.push("schemaScopeOwnerLevels.mutationsFacetDirName must be a declared schema child directory.");
    if (typeof levels.inertSchemaDataKeyword !== "string" || !/^[a-z][A-Za-z0-9]*$/u.test(levels.inertSchemaDataKeyword)) problems.push("schemaScopeOwnerLevels.inertSchemaDataKeyword must name the case-file key that declares schema-shaped test inputs.");
  }
  const resolution = taxonomy.schemaExportResolution;
  if (!resolution || typeof resolution !== "object") problems.push("schemaExportResolution must declare how a consumer names one schema export.");
  else {
    const expected = ["catalogContractId", "catalogDocumentPath", "catalogPath", "exportIdPattern", "exportsKeyword", "forbiddenPlacementPatterns", "formatIds", "generator", "idBase", "idFacetFilenamePattern", "mutationAggregateFacetFilename", "mutationLeafFacetFilename", "mutationScopeSegment", "placementExceptions", "rootExportKeyword", "rustEntriesContractId", "scopeIdPattern", "scopeIdSeparator", "uriPattern", "uriScheme"];
    if (Object.keys(resolution).sort().join("\0") !== expected.join("\0")) problems.push(`schemaExportResolution must declare exactly ${expected.join(", ")}.`);
    if (resolution.uriScheme !== "schema" || !resolution.uriPattern.includes(`${resolution.uriScheme}://`)) problems.push("schemaExportResolution.uriPattern must bind its own declared scheme.");
    for (const [key, value] of [["uriPattern", resolution.uriPattern], ["idFacetFilenamePattern", resolution.idFacetFilenamePattern], ["scopeIdPattern", resolution.scopeIdPattern], ["exportIdPattern", resolution.exportIdPattern]] as const) {
      try { void new RegExp(value, "u"); } catch { problems.push(`schemaExportResolution.${key} is not a valid Unicode regular expression.`); }
    }
    if (typeof resolution.idBase !== "string" || !resolution.idBase.startsWith("https://") || !resolution.idBase.endsWith("/")) problems.push("schemaExportResolution.idBase must be an absolute https prefix ending in a separator.");
    if (resolution.scopeIdSeparator !== ".") problems.push("schemaExportResolution.scopeIdSeparator must be the dotted separator the descriptor ids already use.");
    if (resolution.rootExportKeyword !== "title" || resolution.exportsKeyword !== "$defs") problems.push("schemaExportResolution must read exports from the JSON Schema title and $defs keywords.");
    if (typeof resolution.catalogPath !== "string" || !resolution.catalogPath.endsWith(".json") || typeof resolution.catalogDocumentPath !== "string" || !resolution.catalogDocumentPath.endsWith(".md")) problems.push("schemaExportResolution must point at one generated JSON catalog and one generated Markdown index.");
    if (typeof resolution.generator !== "string" || !resolution.generator.includes("📜️script.ts schema generate")) problems.push("schemaExportResolution.generator must name the exact generating command.");
    if (!Array.isArray(resolution.forbiddenPlacementPatterns) || resolution.forbiddenPlacementPatterns.length === 0) problems.push("schemaExportResolution.forbiddenPlacementPatterns must list the retired schema placements.");
    if (!resolution.placementExceptions || typeof resolution.placementExceptions !== "object" || Array.isArray(resolution.placementExceptions)) problems.push("schemaExportResolution.placementExceptions must be an object of exact path to reason.");
    else for (const [path, reason] of Object.entries(resolution.placementExceptions)) if (typeof reason !== "string" || reason.length < 16 || path.split("/").some((segment) => !segment || segment === "." || segment === "..")) problems.push(`schemaExportResolution.placementExceptions[${JSON.stringify(path)}] must name an exact path and a stated reason.`);
    const dataFormats = [...(taxonomy.schemaFacetKinds?.[taxonomy.schemaDefaultFacetKind]?.formats ?? [])].sort();
    if (!resolution.formatIds || typeof resolution.formatIds !== "object" || Array.isArray(resolution.formatIds)) problems.push("schemaExportResolution.formatIds must map every default-facet-kind format key to its ascii format id.");
    else {
      if (Object.keys(resolution.formatIds).sort().join("\0") !== dataFormats.join("\0")) problems.push(`schemaExportResolution.formatIds must name exactly the ${taxonomy.schemaDefaultFacetKind} formats ${dataFormats.join(", ")}.`);
      const ids = Object.values(resolution.formatIds);
      if (ids.some((value) => typeof value !== "string" || !/^[a-z][a-z0-9]*$/u.test(value)) || new Set(ids).size !== ids.length) problems.push("schemaExportResolution.formatIds values must be unique lowercase ascii format ids.");
    }
    if (typeof resolution.rustEntriesContractId !== "string" || !/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(resolution.rustEntriesContractId)) problems.push("schemaExportResolution.rustEntriesContractId must be the kebab contract id of the Rust registry entry dump.");
    if (typeof resolution.mutationScopeSegment !== "string" || !new RegExp(resolution.scopeIdPattern, "u").test(resolution.mutationScopeSegment)) problems.push("schemaExportResolution.mutationScopeSegment must be one scope-id segment.");
    if (typeof resolution.mutationLeafFacetFilename !== "string" || !new RegExp(resolution.idFacetFilenamePattern, "u").test(resolution.mutationLeafFacetFilename)) problems.push("schemaExportResolution.mutationLeafFacetFilename must be a declared facet filename.");
    if (typeof resolution.mutationAggregateFacetFilename !== "string" || !new RegExp(resolution.idFacetFilenamePattern, "u").test(resolution.mutationAggregateFacetFilename)) problems.push("schemaExportResolution.mutationAggregateFacetFilename must be a declared facet filename.");
  }
  return problems;
}
