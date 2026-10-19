import type { CargoAdmissionProviderV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { buildRepositoryCargoArtifacts } from "../../../⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";
import { runRepositoryCargoCommandV1, runRepositoryCargoLintV1, runRepositoryCargoTests, runRepositoryCargoTestsV1 } from "../🎟️admission/🟦️.ts";
import { configuredRepositoryCargoPreparationPolicyV1, runRepositoryExactCargoLaws } from "../🎯️exact/🟦️.ts";
import { withRepositoryCompilerStoreV1 } from "../../../⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts";
import { repositoryCargoNetworkV1 } from "../📥️dependencies/🟦️.ts";
import { repositoryCargoTestPolicyV1 } from "../🔣️policy/🟦️.ts";

/** 🔌️ Implements the framework Cargo admission port with the repository compiler stores, dependency custody and source admission. */
export const provider: CargoAdmissionProviderV1 = {
  runTests: runRepositoryCargoTestsV1,
  runLint: runRepositoryCargoLintV1,
  runCommand: runRepositoryCargoCommandV1,
  runPackageTests: runRepositoryCargoTests,
  testPolicy: repositoryCargoTestPolicyV1,
  runExactLaws: request => runRepositoryExactCargoLaws({ ...request, preparation: configuredRepositoryCargoPreparationPolicyV1() }),
  withCompilerStore: withRepositoryCompilerStoreV1,
  buildArtifacts: (manifest, args, root, request) => buildRepositoryCargoArtifacts(manifest, args, root, { ...request, network: repositoryCargoNetworkV1(request.invocation, root) }),
};
