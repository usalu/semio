import { join } from "node:path";
import type { ArtifactStagingProviderV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/📤️publication/🟦️.ts";
import { withRepositoryCacheStoreV1 } from "../../💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import { readNativeCallerAuthorityV1 } from "../📋️native-orchestration/📥️invocation/🟦️.ts";
import { stageRepositoryArtifacts, withRepositoryArtifactDestinationV1 } from "../🟦️.ts";

/** 🔌️ Implements the framework artifact staging port with the repository product store and its provenance-marked destinations. */
export const provider: ArtifactStagingProviderV1 = {
  async stageTrees({ workspace, invocation, phase, mode, trees }) {
    if (mode !== readNativeCallerAuthorityV1(invocation.capabilities).original.buildMode) throw Error("Original build mode required");
    await withRepositoryCacheStoreV1(workspace, "products", invocation, phase, async store => {
      for (const tree of trees(store.root)) {
        if (!tree.files.length) continue;
        await withRepositoryArtifactDestinationV1({ workspace, invocation, storage: store.storage, control: store.control, phaseControl: phase, directory: tree.root }, destination => stageRepositoryArtifacts(destination, tree.owner, new Map(tree.files.map(({ name, source }) => [name, join(workspace, source)]))));
      }
    });
  },
};
