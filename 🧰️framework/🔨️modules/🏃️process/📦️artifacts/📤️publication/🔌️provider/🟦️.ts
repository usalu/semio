import { defineScriptProviderV1, loadScriptProviderV1 } from "../../../🔌️providers/🟦️.ts";
import type { ScriptInvocation } from "../../../🧭️routing/📥️invocation/🟦️.ts";
import type { ResourceLeaseControl } from "../../../🔒️leases/🟦️.ts";
import type { BuildMode } from "../../../🏗️build/🎚️mode/🟦️.ts";

/** 🌳️ Names one managed output root, its owner and the repository-relative immutable inputs staged into it. */
export type ArtifactStageTreeV1 = Readonly<{ root: string; owner: string; files: readonly Readonly<{ name: string; source: string }>[] }>;
/** 📨️ Carries one staging request; the provider selects the managed product store and derives every tree root from it. */
export type ArtifactStageRequestV1 = Readonly<{ workspace: string; invocation: ScriptInvocation; phase: ResourceLeaseControl; mode: BuildMode; trees: (productsRoot: string) => readonly ArtifactStageTreeV1[] }>;

/** 🔌️ Publishes provenance-marked artifact trees into the managed product store an issuing parent selected for the whole script process. */
export interface ArtifactStagingProviderV1 {
  stageTrees(request: ArtifactStageRequestV1): Promise<void>;
}

/** 🧷️ Names the port and the exact functions its implementation must expose. */
export const ARTIFACT_STAGING_PROVIDER_V1 = defineScriptProviderV1<ArtifactStagingProviderV1>("artifact-staging", ["stageTrees"]);

/** 🚚️ Stages every non-empty tree under the provider's managed product store. */
export async function stageAdmittedArtifactTreesV1(request: ArtifactStageRequestV1): Promise<void> {
  return (await loadScriptProviderV1(ARTIFACT_STAGING_PROVIDER_V1)).stageTrees(request);
}
