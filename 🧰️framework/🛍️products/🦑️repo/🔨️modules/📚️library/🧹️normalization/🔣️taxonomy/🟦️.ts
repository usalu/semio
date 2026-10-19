import { relative, resolve } from "node:path";
import type { LoadedTaxonomy, TaxonomyContentFacts } from "./📚️loaded/🟦️.ts";
import type { Taxonomy as DiscoveryTaxonomy } from "../../🔍️discovery/📑️taxonomy-document/🟦️.ts";
import { createTaxonomyPathMatcher } from "../../🔍️discovery/🎯️taxonomy-path-matching/🟦️.ts";
import type { TaxonomyLoadOptions } from "./📐️kind-specs/🟦️.ts";
import { semanticOwnedInputFileSnapshot } from "../../🔍️discovery/✅️owned-file-approvals/🟦️.ts";
import { assertLexicalInputOutsideOpaque, noFollowDirectoryChain, verifyNoFollowDirectoryChain } from "../📁️input/🟦️.ts";
import { parseFileAndDirectoryKinds, parseFixedContracts, parseRejectionsAndResolution, parseTaxonomyPrelude } from "./🧱️kinds-and-fixed-contracts/🟦️.ts";
import { parseDescendantsAndCatalogs, parseMemberKindsAndRenderers } from "./🌳️members-renderers-and-descendants/🟦️.ts";
import { parseConsumersAndMutationCatalog, parsePathProjections } from "./🪞️projections-and-consumers/🟦️.ts";
import { assembleTaxonomySchema, parseExclusions, parseGeneratorsAndEcosystems, parsePackageRulesAndDispositions } from "./🏭️generators-packages-and-exclusions/🟦️.ts";

export const TAXONOMY_RELATIVE_PATH = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";
function parseTaxonomy(raw: unknown, path: string): TaxonomyContentFacts {
  const { fileKindRows, directoryKindRows, fixedRows, root, validatedContractPattern, fixedDirectoryRows, fixedRejectionRows, configurableRows, fileResolutionRows, scopedFileRows, directoryMemberRows, projectedMemberRows, projectionRendererRows, descendantContractRows, projectionCatalogRows, projectionRows, ownedFileProjectionRows, generatorRows, projectionConsumerRows, mutationCatalogProjectionRow, ecosystemRows, grammarRows, boundaryRows, boundaryProfileRows, sourceDispositionRows, exclusionRows, enforcement, pathMatcher, collision } = parseTaxonomyPrelude({ raw, path });
  const { semanticDirectoryKinds, fileKinds } = parseFileAndDirectoryKinds({ fileKindRows, directoryKindRows });
  const { fixedDirectoryContracts, fixedFilenameContracts } = parseFixedContracts({ fixedRows, root, semanticDirectoryKinds, validatedContractPattern, fixedDirectoryRows });
  const { configurableEntryContracts, fixedFilenameRejectionContracts, fileKindResolutionRules, scopedFileKinds } = parseRejectionsAndResolution({ fixedRejectionRows, configurableRows, fileKinds, fileResolutionRows, validatedContractPattern, semanticDirectoryKinds, scopedFileRows });
  const { allDirectoryContextIds, semanticDirectoryMemberKinds, semanticProjectedMemberKinds, semanticPathProjectionProfileRenderers } = parseMemberKindsAndRenderers({ directoryMemberRows, semanticDirectoryKinds, projectedMemberRows, fixedDirectoryContracts, projectionRendererRows });
  const { captureFields, semanticDescendantContracts, semanticPathProjectionCatalogContracts } = parseDescendantsAndCatalogs({ semanticDirectoryKinds, allDirectoryContextIds, fileKinds, fixedFilenameContracts, descendantContractRows, root, projectionCatalogRows });
  const { semanticPathProjectionContracts, parseProjectionSegment, semanticOwnedFileProjectionContracts } = parsePathProjections({ allDirectoryContextIds, semanticDirectoryMemberKinds, semanticProjectedMemberKinds, captureFields, projectionRows, semanticPathProjectionProfileRenderers, semanticDescendantContracts, semanticPathProjectionCatalogContracts, ownedFileProjectionRows, generatorRows, fileKinds, semanticDirectoryKinds, fixedDirectoryRows, fixedFilenameContracts });
  const { semanticPathProjectionReferenceConsumerContracts, mutationCatalogProjection } = parseConsumersAndMutationCatalog({ projectionConsumerRows, semanticPathProjectionContracts, mutationCatalogProjectionRow, parseProjectionSegment, semanticDirectoryMemberKinds, semanticProjectedMemberKinds, semanticDescendantContracts, semanticPathProjectionCatalogContracts });
  const { generatorContracts, ecosystems } = parseGeneratorsAndEcosystems({ generatorRows, validatedContractPattern, ecosystemRows });
  const { packageBoundaryRules, packageBoundaryProfiles, packageGlueGrammar, packageSourceDispositions } = parsePackageRulesAndDispositions({ grammarRows, boundaryRows, fixedFilenameContracts, configurableEntryContracts, fileKinds, semanticDirectoryKinds, boundaryProfileRows, sourceDispositionRows });
  const { pathExclusions, exclusions } = parseExclusions({ exclusionRows, enforcement, semanticPathProjectionReferenceConsumerContracts, generatorContracts, pathMatcher });
  const { schema } = assembleTaxonomySchema({ root, fileKinds, semanticDirectoryKinds, fixedFilenameContracts, fixedFilenameRejectionContracts, fixedDirectoryContracts, configurableEntryContracts, fileKindResolutionRules, scopedFileKinds, semanticDirectoryMemberKinds, semanticProjectedMemberKinds, semanticPathProjectionProfileRenderers, semanticDescendantContracts, semanticPathProjectionCatalogContracts, semanticPathProjectionContracts, semanticOwnedFileProjectionContracts, semanticPathProjectionReferenceConsumerContracts, mutationCatalogProjection, generatorContracts, ecosystems, packageBoundaryRules, packageBoundaryProfiles, packageGlueGrammar, packageSourceDispositions, pathExclusions, collision, enforcement });
  return {
    schema,
    discoverySchema: root as unknown as DiscoveryTaxonomy,
    exclusions: exclusions.sort((a, b) => a.path.localeCompare(b.path)),
    fileKinds: Object.entries(fileKinds).map(([id, spec]) => ({ id, ...spec })).sort((a, b) => a.id.localeCompare(b.id)),
    directoryKinds: Object.entries(semanticDirectoryKinds).map(([id, spec]) => ({ id, ...spec, slugRegex: new RegExp(`^(?:${spec.slugPattern})$`, "u") })).sort((a, b) => a.id.localeCompare(b.id)),
  };
}

/** 🧠️ Keeps private content-derived facts while every load captures its own physical input and session. */
const PARSED_TAXONOMIES = new Map<string, TaxonomyContentFacts>();

const PARSED_TAXONOMY_CAPACITY = 8;

export function loadTaxonomy(options: TaxonomyLoadOptions): LoadedTaxonomy {
  const path = assertLexicalInputOutsideOpaque(options.repoRoot, options.taxonomyPath ?? TAXONOMY_RELATIVE_PATH, "taxonomyPath", true);
  const ancestors = noFollowDirectoryChain(resolve(options.repoRoot));
  const input = semanticOwnedInputFileSnapshot(options.repoRoot, relative(resolve(options.repoRoot), path).replaceAll("\\", "/"));
  verifyNoFollowDirectoryChain(ancestors);
  if (!input) throw new Error("Taxonomy schema is absent: " + path);
  let parsed = PARSED_TAXONOMIES.get(input.contentHash);
  if (!parsed) {
    const bytes = Buffer.from(input.bytes), text = bytes.toString("utf8");
    if (!Buffer.from(text).equals(bytes)) throw new Error("Taxonomy schema has lossy UTF-8: " + path);
    const values = parseTaxonomy(JSON.parse(text) as unknown, path);
    if (PARSED_TAXONOMIES.size >= PARSED_TAXONOMY_CAPACITY) PARSED_TAXONOMIES.delete(PARSED_TAXONOMIES.keys().next().value!);
    PARSED_TAXONOMIES.set(input.contentHash, values);
    parsed = values;
  }
  return { ...structuredClone(parsed), path, input, pathMatcher: createTaxonomyPathMatcher() };
}
