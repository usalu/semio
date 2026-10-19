import type { ConfigurableEntryContract, FileKindResolutionRuleSpec, FileKindSpec, FixedDirectoryContract, FixedFilenameContract, FixedFilenameRejectionContract, ScopedFileKindSpec, SemanticDirectoryKindSpec, SemanticDirectoryMemberKindSpec, SemanticProjectedMemberKindSpec } from "../📐️kind-specs/🟦️.ts";
import type { MutationCatalogProjectionContractIds, SemanticPathProjectionCatalogContract, SemanticPathProjectionContract, SemanticPathProjectionProfileRenderer, SemanticPathProjectionReferenceConsumerContract } from "../🧭️path-projection/🟦️.ts";
import type { SemanticDescendantContract } from "../🌳️descendants/🟦️.ts";
import type { SemanticOwnedFileProjectionContract } from "../🗃️owned-file-projection/🟦️.ts";
import type { Taxonomy as DiscoveryTaxonomy } from "../../../🔍️discovery/📑️taxonomy-document/🟦️.ts";
import type { EcosystemSpec, GeneratorContractSpec, PackageBoundaryProfile, PackageBoundaryRule, PackageGlueGrammar, PackageSourceDisposition } from "../🏭️generators-packages/🟦️.ts";
import type { TaxonomyPathMatcher } from "../../../🔍️discovery/🎯️taxonomy-path-matching/🟦️.ts";
import type { SemanticOwnedInputFileSnapshot } from "../../../🔍️discovery/📜️taxonomy-contracts/🟦️.ts";

export interface TaxonomyV7 {
  readonly schemaVersion: 7;
  readonly windowEmptyFacetFileKindId: string;
  readonly fileKinds: Readonly<Record<string, FileKindSpec>>;
  readonly semanticDirectoryKinds: Readonly<Record<string, SemanticDirectoryKindSpec>>;
  readonly fixedFilenameContracts: Readonly<Record<string, FixedFilenameContract>>;
  readonly fixedFilenameRejectionContracts: Readonly<Record<string, FixedFilenameRejectionContract>>;
  readonly fixedDirectoryContracts: Readonly<Record<string, FixedDirectoryContract>>;
  readonly configurableEntryContracts: Readonly<Record<string, ConfigurableEntryContract>>;
  readonly fileKindResolutionRules: Readonly<Record<string, FileKindResolutionRuleSpec>>;
  readonly scopedFileKinds: Readonly<Record<string, ScopedFileKindSpec>>;
  readonly semanticDirectoryMemberKinds: Readonly<Record<string, SemanticDirectoryMemberKindSpec>>;
  readonly semanticProjectedMemberKinds: Readonly<Record<string, SemanticProjectedMemberKindSpec>>;
  readonly semanticPathProjectionProfileRenderers: Readonly<Record<string, SemanticPathProjectionProfileRenderer>>;
  readonly semanticDescendantContracts: Readonly<Record<string, SemanticDescendantContract>>;
  readonly semanticPathProjectionCatalogContracts: Readonly<Record<string, SemanticPathProjectionCatalogContract>>;
  readonly semanticPathProjectionContracts: Readonly<Record<string, SemanticPathProjectionContract>>;
  readonly semanticOwnedFileProjectionContracts: Readonly<Record<string, SemanticOwnedFileProjectionContract>>;
  readonly semanticPackageProjectionContracts: DiscoveryTaxonomy["semanticPackageProjectionContracts"];
  readonly semanticPathProjectionReferenceConsumerContracts: Readonly<Record<string, SemanticPathProjectionReferenceConsumerContract>>;
  readonly mutationCatalogProjection: MutationCatalogProjectionContractIds;
  readonly generatorContracts: Readonly<Record<string, GeneratorContractSpec>>;
  readonly ecosystems: Readonly<Record<string, EcosystemSpec>>;
  readonly packageBoundaryRules: Readonly<Record<string, PackageBoundaryRule>>;
  readonly packageBoundaryProfiles: Readonly<Record<string, PackageBoundaryProfile>>;
  readonly packageGlueGrammar: Readonly<Record<string, PackageGlueGrammar>>;
  readonly packageSourceDispositions: Readonly<Record<string, PackageSourceDisposition>>;
  readonly pathExclusions: Readonly<Record<string, { readonly path: string; readonly mode: "opaque"; readonly reason: string }>>;
  readonly unicodeNormalization: { readonly form: "NFC"; readonly caseFold: "lower"; readonly locale: "und" };
  readonly variationSelectorPolicy: { readonly selector: "\uFE0F"; readonly requiredAfterEmoji: true; readonly comparison: "ignore-selector" };
  readonly collisionPolicy: {
    readonly comparisons: readonly ("byte" | "nfc" | "case-fold" | "vs16-fold" | "same-kind")[];
    readonly maxPathUtf16Units: number;
    readonly maxComponentUtf8Bytes: number;
    readonly rejectWindowsReservedNames: boolean;
    readonly rejectTrailingDotsAndSpaces: boolean;
  };
  readonly areaEnforcement: {
    readonly requiredState: "clean";
    readonly undeclaredAreas: "enforce";
    readonly opaquePathExclusionIds: readonly string[];
  };
}

export interface TaxonomyContentFacts {
  readonly schema: TaxonomyV7;
  readonly discoverySchema: DiscoveryTaxonomy;
  readonly exclusions: readonly { readonly id: string; readonly path: string }[];
  readonly fileKinds: readonly (FileKindSpec & { readonly id: string })[];
  readonly directoryKinds: readonly (SemanticDirectoryKindSpec & { readonly id: string; readonly slugRegex: RegExp })[];
}

export interface LoadedTaxonomy extends TaxonomyContentFacts {
  readonly path: string;
  readonly pathMatcher: TaxonomyPathMatcher;
  readonly input: SemanticOwnedInputFileSnapshot;
}

/** 🧭️ What a generator-input walk reads from a loaded taxonomy: the discovery vocabulary it classifies with and
 * the matcher it excludes with. Narrower than [[LoadedTaxonomy]] so a caller can build one without the walk's
 * private loading state. */
export type GeneratorInputTaxonomy = Pick<LoadedTaxonomy, "discoverySchema" | "pathMatcher" | "exclusions">;
