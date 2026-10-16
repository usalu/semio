/** 🔖️ What a plugin contributes to an artifact kind: mutation semantics and metadata, inference metadata and the contribution descriptor.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 */

// #region 🎠️Kernel
//#region 🔖️ArtifactContribution
/** 🗂️ The `verb`/`entity`/`kind`/`record` semantic identity of one contributed mutation — mirrors
 * Rust `ContributedMutationSemantics`. */
export type ContributedMutationSemantics = {
  readonly verb: string;
  readonly entity: string;
  readonly kind: string;
  readonly record: string;
};

/** 🗂️ One mutation a plugin contributes onto an artifact kind it depends on — mirrors Rust
 * `ContributedMutationMetadata`. `mutationId` follows the contract freeze §3 contributed-id
 * grammar: `"<target-document-schema>#<contributor-plugin-id>:<kebab-kind>"`. */
export type ContributedMutationMetadata = {
  readonly mutationId: string;
  readonly semantics: ContributedMutationSemantics;
  readonly schemaVersion: number;
  readonly algorithmVersion: number;
};

/** 💡️ One inference a plugin contributes onto an artifact kind it depends on — mirrors Rust
 * `ContributedInferenceMetadata` (the native `ArtifactInferenceServiceMetadata` fields plus
 * `contributor`/`dependsOn`). Registration gate (contract freeze §4): `owner === contributor`,
 * `artifactKind` equals the target artifact kind. */
export type ContributedInferenceMetadata = {
  readonly owner: string;
  readonly artifactKind: string;
  readonly artifactSchema: string;
  readonly artifactSchemaVersion: number;
  readonly documentSchema: string;
  readonly documentSchemaVersion: number;
  readonly inferenceSchema: string;
  readonly inferenceSchemaVersion: number;
  readonly algorithmVersion: number;
  readonly policyVersion: number;
  readonly contributor: string;
  readonly dependsOn?: readonly string[];
};

/** 🗂️ Everything one plugin contributes onto one artifact kind it depends on — mirrors Rust
 * `ArtifactContributionDescriptor`. Accepted only when `artifactKind`'s owning plugin is a direct
 * entry in the contributor's declared `PluginManifest.dependencies` (contract freeze §4). */
export type ArtifactContributionDescriptor = {
  readonly artifactKind: string;
  readonly mutations?: readonly ContributedMutationMetadata[];
  readonly inferences?: readonly ContributedInferenceMetadata[];
};
//#endregion 🔖️ArtifactContribution
// #endregion 🎠️Kernel
