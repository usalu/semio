/** 🔌 Registers every PluginRuntime topic suite into the in-source vitest run of the PluginRuntime module. */
import { type PluginRuntimeVitest, type PluginRuntimeDependencies, type PluginRuntimeSource } from "../🛠️plugin-runtime-helpers/🟦️.tsx";
import { registerUiCloseAndJobAdmissionTests } from "../🏁️ui-close-and-job-admission/🟦️.tsx";
import { registerViewModelSurfacesTests } from "../🖥️view-model-surfaces/🟦️.tsx";
import { registerExtensionInvocationWireTests } from "../📬️extension-invocation-wire/🟦️.tsx";
import { registerInstanceOpenLifecycleTests } from "../🚶️instance-open-lifecycle/🟦️.tsx";
import { registerTransactionCoordinatorTests } from "../🏗️transaction-coordinator/🟦️.tsx";
import { registerDocumentPackWireAdapterTests } from "../📦️document-pack-wire-adapter/🟦️.tsx";
import { registerTurnDispatchLifecycleTests } from "../🧵️turn-dispatch-lifecycle/🟦️.tsx";
import { registerTypedOperationPagesTests } from "../📃️typed-operation-pages/🟦️.tsx";
import { registerSettleContinuationsTests } from "../🔂️settle-continuations/🟦️.tsx";
import { registerTurnSchedulerLanesTests } from "../⚖️turn-scheduler-lanes/🟦️.tsx";
import { registerShardLossAndBootTests } from "../🌋️shard-loss-and-boot/🟦️.tsx";
import { registerTypedOperationCompletionTests } from "../📯️typed-operation-completion/🟦️.tsx";
import { registerUiIntakeAdmissionTests } from "../🪢️ui-intake-admission/🟦️.tsx";

export async function registerPluginRuntimeTests(vitest: PluginRuntimeVitest, dependencies: PluginRuntimeDependencies, source: PluginRuntimeSource): Promise<void> {
  await registerUiCloseAndJobAdmissionTests(vitest, dependencies);
  await registerViewModelSurfacesTests(vitest, dependencies, source);
  await registerExtensionInvocationWireTests(vitest, dependencies);
  await registerInstanceOpenLifecycleTests(vitest, dependencies);
  await registerTransactionCoordinatorTests(vitest, dependencies);
  await registerDocumentPackWireAdapterTests(vitest, dependencies, source);
  await registerTurnDispatchLifecycleTests(vitest, dependencies);
  await registerTypedOperationPagesTests(vitest, dependencies);
  await registerSettleContinuationsTests(vitest, dependencies);
  await registerTurnSchedulerLanesTests(vitest, dependencies);
  await registerShardLossAndBootTests(vitest, dependencies);
  await registerTypedOperationCompletionTests(vitest, dependencies);
  await registerUiIntakeAdmissionTests(vitest, dependencies);
}
