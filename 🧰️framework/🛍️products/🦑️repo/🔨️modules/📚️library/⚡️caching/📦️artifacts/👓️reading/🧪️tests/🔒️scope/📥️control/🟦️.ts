import {expect} from "bun:test";
/** 🧬️ Exercises authentic control lineage inside the suite's already received Original. */
export async function proveArtifactControlRefinementV1(original: import("../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation): Promise<void> {
  const api = await import("../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"), base = api.createIssuedResourceControlV1(original, "reader:unrestricted-base", 0), finite = api.createIssuedResourceControlV1(original, "reader:finite-base", 5000), child = api.createRequestedIssuedResourceControlV1(finite, 0), sibling = api.createIssuedResourceControlV1(original, "reader:independent-phase", 0), signal = new AbortController(), narrowed = api.createIssuedResourceControlV1(original, "reader:signal-base", 0, signal.signal);
  expect(typeof api.requireIssuedResourceControlRefinementV1).toBe("function");
  api.requireIssuedResourceControlRefinementV1(base, finite);
  api.requireIssuedResourceControlRefinementV1(finite, child);
  api.requireIssuedResourceControlRefinementV1(narrowed, api.createRequestedIssuedResourceControlV1(narrowed, 0));
  expect(() => api.requireIssuedResourceControlRefinementV1(finite, sibling)).toThrow();
  expect(() => api.requireIssuedResourceControlRefinementV1(narrowed, sibling)).toThrow();
  expect(() => api.requireIssuedResourceControlRefinementV1({ ...base }, finite)).toThrow();
  expect(() => api.requireIssuedResourceControlRefinementV1(base, { ...finite })).toThrow();
  signal.abort(Error("Actual caller signal stopped"));
  expect(() => api.requireIssuedResourceControlRefinementV1(narrowed, narrowed)).toThrow();
  const vite=await import("../../../../../🌐️vite/🟦️.ts"),authority=(await import("../../../../📋️native-orchestration/📥️invocation/🟦️.ts")).readNativeCallerAuthorityV1(original.capabilities),mode=authority.original.buildMode==="prototype"?"release":"prototype",root=authority.repositoryRoot;
  await expect(vite.buildViteArtifact({control:base,invocation:original,root,workspace:root,config:root+"/no-config",owner:"fixture:wrong-intent",mode})).rejects.toThrow("Original mode");
  await expect(vite.serveVite({control:base,invocation:original,root,config:root+"/no-config",host:"127.0.0.1",port:0,signal:original.control.signal,mode,ready:()=>{throw Error("Wrong intent reached listener");}})).rejects.toThrow("Original mode");
  await expect(vite.serveViteCommandV1({runtimeChannels:[],control:base,invocation:original,root,config:root+"/no-config",host:"127.0.0.1",port:5000,signal:original.control.signal,mode,environment:{}})).rejects.toThrow("Original mode");
  for(const forged of[{...base},undefined]){
    const control=forged as typeof base,mode=authority.original.buildMode;
    await expect(vite.buildViteArtifact({control,invocation:original,root,workspace:root,config:root+"/must-not-open",owner:"fixture:forged-control",mode})).rejects.toThrow("Original resource control issuance");
    await expect(vite.serveVite({control,invocation:original,root,config:root+"/must-not-open",host:"127.0.0.1",port:0,signal:original.control.signal,mode,ready:()=>{throw Error("Forged control reached listener");}})).rejects.toThrow("Original resource control issuance");
    await expect(vite.serveViteCommandV1({runtimeChannels:[],control,invocation:original,root,config:root+"/must-not-open",host:"127.0.0.1",port:5000,signal:original.control.signal,mode,environment:{}})).rejects.toThrow("Original resource control issuance");
  }
  console.log("[DEBUG] Actual Original rejects copied and absent controls at all three Vite entries before effects");
  console.log("[DEBUG] Actual Original rejects all three foreign-intent Vite entries before any cache/config/listener effect");
  console.log("[DEBUG] Actual private control ancestry preserves finite and custom-signal base constraints without reconstructing clocks or authority");
}
