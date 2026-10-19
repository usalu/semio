import type { MutationFixtureCoverageContract, SemanticPathProjectionReferenceConsumerForm } from "../../../🔍️discovery/📜️taxonomy-contracts/🟦️.ts";

export type SemanticProjectionCaptureField = "standardVersion" | "subsetId" | "mutationId" | "scenarioId" | "commandDirectoryName";

export type SemanticProjectionSourceSegment = Readonly<{ kindId: string; literal: string } | { kindId: string; capture: SemanticProjectionCaptureField } | { memberKindId: string; literal: string } | { projectedMemberKindId: string; capture: SemanticProjectionCaptureField }>;

export type SemanticProjectionDestinationSegment = Readonly<{ kindId: string; literal: string } | { kindId: string; render: "profile" } | { kindId: string; copy: SemanticProjectionCaptureField } | { projectedMemberKindId: string; copy: SemanticProjectionCaptureField }>;

export interface SemanticPathProjectionProfileRenderer {
  readonly direction: "forward-only";
  readonly captureFields: readonly ["standardVersion", "subsetId"];
  readonly directoryKindId: string;
  readonly template: "🪆️{standardVersion}-{subsetId}";
  readonly tupleCollisionFields: readonly ["artifactId", "standardVersion", "subsetId"];
}

export interface SemanticMutationPathProjectionCatalogContract {
  readonly registryField: "vectors";
  readonly required: true;
  readonly allowEmpty: true;
  readonly runtimeKindsField: "kinds";
  readonly runtimeKindsRelation: "independent";
  readonly mutationIdField: "mutationId";
  readonly sourceMutationDirectoryNameField: "sourceMutationDirectoryName";
  readonly mutationDirectoryNameField: "mutationDirectoryName";
  readonly scenariosField: "scenarios";
  readonly scenarioIdField: "id";
  readonly scenarioDirectoryNameField: "directoryName";
  readonly sourceBundleUniquenessFields: readonly ["mutationId", "sourceMutationDirectoryName", "scenarioId"];
  readonly canonicalBundleUniquenessFields: readonly ["mutationId", "mutationDirectoryName", "scenarioId"];
  readonly coverage: "every-physical-bundle-exactly-once";
}

export interface SemanticDistributedJsonManifestCatalogContract {
  readonly contractKind: "distributed-json-manifest-catalog";
  readonly ownerArtifactMemberName: string;
  readonly profileVectors: readonly Readonly<{ artifactId: string; standardVersion: string; subsetId: string }>[];
  readonly modelManifestSchema: string;
  readonly modelManifestSourceFilename: string;
  readonly modelIdentityField: "id";
  readonly memberIdentityField: "id";
  readonly memberVersionField: "version";
  readonly requiredMemberVersion: string;
  readonly requiredModelManifest: true;
  readonly categoryRules: readonly Readonly<{ sourceDirectoryName: string; directoryKindId: string; sourceShape: "direct-semantic-json"; manifestSchema: string; memberDirectoryEmoji: string } | { sourceDirectoryName: string; directoryKindId: string; sourceShape: "nested-fixed-json"; manifestSchema: string; fixedSourceFilename: string }>[];
  readonly coverage: "every-source-file-and-destination-node-exactly-once";
  readonly unknownCategoryPolicy: "problem";
  readonly unownedModelPolicy: "problem";
}

export interface SemanticExactOwnerVectorsCatalogContract {
  readonly contractKind: "exact-owner-vectors";
  readonly required: true;
  readonly allowEmpty: false;
  readonly identityFields: readonly ["artifactId", "standardVersion", "subsetId", "commandDirectoryName"];
  readonly coverage: "every-physical-command-bundle-exactly-once";
  readonly vectors: readonly Readonly<{ artifactId: string; standardVersion: string; subsetId: string; commandDirectoryName: string }>[];
}

export type SemanticPathProjectionCatalogContract = SemanticMutationPathProjectionCatalogContract | SemanticDistributedJsonManifestCatalogContract | SemanticExactOwnerVectorsCatalogContract;

export interface SemanticPathProjectionContract {
  readonly sourceOwnerKindId: string;
  readonly sourceArtifactMemberName?: string;
  readonly sourceSegments: readonly SemanticProjectionSourceSegment[];
  readonly profileRendererId: string;
  readonly destinationOwnerKindId: string;
  readonly destinationSegments: readonly SemanticProjectionDestinationSegment[];
  readonly descendantContractId: string;
  readonly catalogContractId: string;
  readonly rationaleRule: "artifact-example-model-catalog-projection-v1" | "artifact-editor-command-projection-v1";
}

export interface SemanticPathProjectionReferenceConsumerContract {
  readonly projectionContractId: string;
  readonly consumerIdentity: string;
  readonly ownership: "external";
  readonly sourcePathPattern: string;
  readonly sourcePathIdentities: readonly string[];
  readonly adapters: readonly ("rust" | "typescript" | "json" | "toml")[];
  readonly supportedForms: readonly SemanticPathProjectionReferenceConsumerForm[];
  readonly staleMarkers: readonly string[];
}

export interface MutationCatalogProjectionContractIds {
  readonly contractKind: "canonical-mutation-case-pair";
  readonly contractId: "canonical-mutation-case-pair-v1";
  readonly sourceOwnerKindId: string;
  readonly projectedMemberKindId: string;
  readonly implementationSegments: readonly SemanticProjectionSourceSegment[];
  readonly fixtureSegments: readonly SemanticProjectionSourceSegment[];
  readonly implementationDescendantContractId: string;
  readonly fixtureDescendantContractId: string;
  readonly catalogContractId: string;
  readonly coverage: "every-catalog-vector-has-one-fixture-bundle-with-declared-coverage";
  readonly fixtureCoverage: MutationFixtureCoverageContract;
}
