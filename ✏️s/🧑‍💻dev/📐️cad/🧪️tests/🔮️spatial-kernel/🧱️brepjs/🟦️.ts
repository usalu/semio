// #region 🧲️Header
/// <reference types="vite/client" />
/// <reference types="vitest/importMeta" />
/** 🧭️ `@semio-tech/cad-js/brepjs` — `SpatialKernel` backed by brepjs + OpenCascade WASM, kept ONLY as a vitest differential oracle (see `import.meta.vitest`-guarded exports below and `🎫️tickets/…/BREP-KERNEL-DEPENDENCY-FREE-RUNTIME/📓️w4a-spatial-kernel-first-party.md`). The production runtime kernel is `🧠️semio/🟦️.ts`; pure preview math moved to the `🧮️preview` modules. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { kernelGeometry, type MeshTransfer } from "@semio-tech/framework-3d-js";
import { meshTransferTransferables } from "./🧱️entity-maps/🟦️.ts";
import { BrepjsWasmEngine } from "./⚙️wasm-engine/🟦️.ts";
import { type BrepjsWorkerRequest, type BrepjsWorkerResponse, deserializeWorkerArg } from "./📨️worker-and-kernel/🟦️.ts";
export { kernelGeometry };

// #endregion 🔌️Adapters
// #region 🌐️BrepjsWorkerEntry
function isBrepjsDedicatedWorker(): boolean {
  return typeof self !== "undefined" && self.constructor?.name === "DedicatedWorkerGlobalScope";
}

if (isBrepjsDedicatedWorker()) {
  let engine: BrepjsWasmEngine | null = null;
  self.addEventListener("message", (event: MessageEvent<BrepjsWorkerRequest>) => {
    const msg = event.data;
    if (msg.type === "init") {
      void (async () => {
        try {
          engine = new BrepjsWasmEngine();
          await engine.ensureInit();
          self.postMessage({ type: "init-done" } satisfies BrepjsWorkerResponse);
        } catch (e) {
          self.postMessage({
            type: "init-error",
            error: e instanceof Error ? e.message : String(e),
          } satisfies BrepjsWorkerResponse);
        }
      })();
      return;
    }
    if (msg.type === "rpc" && engine) {
      void (async () => {
        try {
          const args = msg.args.map(deserializeWorkerArg);
          const fn = (engine as unknown as Record<string, unknown>)[msg.method];
          if (typeof fn !== "function") throw new Error(`brepjs worker: unknown method ${msg.method}`);
          const result = await (fn as (...a: unknown[]) => unknown).apply(engine, args);
          const transfers = result && typeof result === "object" && "position" in result ? meshTransferTransferables(result as MeshTransfer) : undefined;
          if (transfers?.length) {
            self.postMessage({ type: "rpc-result", id: msg.id, result } satisfies BrepjsWorkerResponse, {
              transfer: transfers,
            });
          } else {
            self.postMessage({ type: "rpc-result", id: msg.id, result } satisfies BrepjsWorkerResponse);
          }
        } catch (e) {
          self.postMessage({
            type: "rpc-error",
            id: msg.id,
            error: e instanceof Error ? e.message : String(e),
          } satisfies BrepjsWorkerResponse);
        }
      })();
    }
  });
}

// #endregion 🌐️BrepjsWorkerEntry
