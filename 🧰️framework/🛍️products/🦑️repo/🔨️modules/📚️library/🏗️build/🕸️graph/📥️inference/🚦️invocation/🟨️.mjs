/** 🚦️ Routes each plugin call to the current or the reloaded implementation of its revision. */
import { inferenceInventoryReferencesHoldV1, requireCurrentInferenceInventoryReuseV1 } from "../../../../🏗️build/👁️observation/📥️reuse/🟨️.mjs";
import { join, resolve } from "node:path";
import { receiveScriptProcessInvocation, createIssuedResourceControlV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { requireCurrentObservationV1, observeDependencies, observeInference, observeInferenceSnapshotV1, settleObservedComputationV1 } from "../../../../🏗️build/👁️observation/🟨️.mjs";
import { requireCurrentSourceInventoryV1 } from "../../../../🏗️build/📥️inputs/📋️data/📥️load/🟨️.mjs";
import { withRepositoryBuildGraphFactStoreV1 } from "../../../📥️inputs/💾️storage/🟨️.mjs";

/** 🏷️ Names this component as its roster entry does. */
export const component = "invocation";
let libraryBootstrap, implementationRevision, createMetadataImplementation, workspaceDataDirectory, emojiProjectJsonNodes, settlementOptionsV1, dependencyResolutionAuthorityImplementation, moduleSourceRecordCacheRootV1, createDependenciesImplementation, cacheInternals, revisionIdentity, services;

/** 🔌️ Binds this component's dependencies from the revision registry once its needs are wired. */
export function wire(registry) {
  ({ libraryBootstrap } = registry.bootstrap);
  ({ implementationRevision, dependencyResolutionAuthorityImplementation } = registry.revision);
  ({ createMetadataImplementation, moduleSourceRecordCacheRootV1, createDependenciesImplementation } = registry.dependencyEdges);
  ({ workspaceDataDirectory } = registry.coordinates);
  ({ emojiProjectJsonNodes, settlementOptionsV1 } = registry.projectNodes);
  ({ cacheInternals } = registry.internals);
  ({ revisionIdentity, services } = registry);
}

let reloadedImplementation;

export function invokeCurrentImplementation(kind, args) {
  requireCurrentSourceInventoryV1();
  requireCurrentInferenceInventoryReuseV1();
  requireCurrentObservationV1();
  if(kind!=="nodes"&&kind!=="dependencies")return invokeCurrentImplementationV1(kind,args);
  const index=kind==="nodes"?2:1,context=args[index],run=(invocation,phaseControl)=>withRepositoryBuildGraphFactStoreV1(context.workspaceRoot,invocation,phaseControl,(_store,actualControl)=>{const current=[...args];current[index]={...context,invocation,phaseControl:actualControl};return invokeCurrentImplementationV1(kind,current);});
  return context.invocation?run(context.invocation,context.phaseControl):receiveScriptProcessInvocation(process.env,invocation=>run(invocation,createIssuedResourceControlV1(invocation,"graph:inference-entry",0)));
}

function invokeCurrentImplementationV1(kind, args) {
  return libraryBootstrap.then(() => {
    requireCurrentSourceInventoryV1();requireCurrentObservationV1();
    const revision = implementationRevision();
    if (revision === revisionIdentity.revision && kind === "metadata")return createMetadataImplementation(...args);
    if (revision === revisionIdentity.revision) return kind === "nodes" ? args[1]?.snapshot ? observeInferenceSnapshotV1(() => emojiProjectJsonNodes(...args)) : observeInference({ cachePath: join(workspaceDataDirectory(args[2].workspaceRoot), "inference", "emoji-project-json.json"), key: [revision, args[0], args[1] ?? null, resolve(args[2].workspaceRoot)],reusable:value=>inferenceInventoryReferencesHoldV1(args[2].workspaceRoot,value,args[2].phaseControl) }, () => settleObservedComputationV1(() => emojiProjectJsonNodes(...args), settlementOptionsV1(args[2]))) : kind === "authority" ? dependencyResolutionAuthorityImplementation(...args) : observeDependencies({ receiptPath: join(workspaceDataDirectory(args[1].workspaceRoot), "inference", "emoji-project-json.receipt.json"), recordDirectory:moduleSourceRecordCacheRootV1(args[1].workspaceRoot) }, () => settleObservedComputationV1(() => createDependenciesImplementation(...args), settlementOptionsV1(args[1])));
    if (reloadedImplementation?.revision !== revision) {
      const entry = { revision, module: services.loadRevision(revision).catch((error) => { if (reloadedImplementation === entry) reloadedImplementation = undefined; throw error; }) };
      reloadedImplementation = entry;
    }
    return reloadedImplementation.module.then((module) => {
      if (module.cacheInternals === cacheInternals) throw new Error("Graph runtime retained an obsolete implementation");
      return module.invokeCurrentImplementation(kind, args);
    });
  });
}
