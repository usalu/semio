import {receiveScriptProcessInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {readNativeCallerAuthorityV1} from "../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import type { CargoAdmissionProviderV1, CompilerStoreViewV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { buildRepositoryCargoArtifacts } from "../../../⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";
import { runRepositoryCargoCommandV1, runRepositoryCargoLintV1, runRepositoryCargoTests, runRepositoryCargoTestsV1 } from "../🎟️admission/🟦️.ts";
import { configuredRepositoryCargoPreparationPolicyV1, runRepositoryExactCargoLaws } from "../🎯️exact/🟦️.ts";
import { withRepositoryCompilerStoreV1 } from "../../../⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts";
import { repositoryCargoCompilerInputsV1, repositoryCargoConsumptionV1, repositoryCargoNetworkV1 } from "../📥️dependencies/🟦️.ts";
import { repositoryCargoTestPolicyV1 } from "../🔣️policy/🟦️.ts";

/** 🔌️ Implements the framework Cargo admission port with the repository compiler stores, dependency custody and source admission. */
export const provider: CargoAdmissionProviderV1 = {
  runTests: runRepositoryCargoTestsV1,
  runLint: runRepositoryCargoLintV1,
  runCommand: runRepositoryCargoCommandV1,
  runPackageTests: runRepositoryCargoTests,
  testPolicy: repositoryCargoTestPolicyV1,
  runExactLaws: request => runRepositoryExactCargoLaws({ ...request, preparation: configuredRepositoryCargoPreparationPolicyV1() }),
  withCompilerStore: (workspace,mode,control,environment,body) => receiveScriptProcessInvocation(environment,original=>withRepositoryCompilerStoreV1(workspace,mode,control,environment,async(store,phase)=>{
    const root=readNativeCallerAuthorityV1(original.capabilities).repositoryRoot;
    const withCargoInputs:CompilerStoreViewV1["withCargoInputs"]=(request,consume)=>repositoryCargoCompilerInputsV1({workspace:root,invocation:original,control:phase,storage:store.storage,network:repositoryCargoNetworkV1(original,root),consumption:repositoryCargoConsumptionV1(request.arguments,"artifact")})(request,consume);
    return body(Object.freeze({...store,withCargoInputs}),phase);
  })),
  buildArtifacts: (manifest, args, root, request) => buildRepositoryCargoArtifacts(manifest, args, root, { ...request, network: repositoryCargoNetworkV1(request.invocation, root) }),
};
