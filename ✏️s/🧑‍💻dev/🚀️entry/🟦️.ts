import { admitPluginCatalogV1 } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🟦️.ts";
import type { FrameworkOsBootOptions, FrameworkOsBootExecution } from "@semio-tech/framework-renderer-react";
import { mountSDevInventoryV1, type SDevMountV1, type SDevOperationV1 } from "../🧩️service-composition/🟦️.ts";

export type SDevBrowserInventoryV1 = {
  readonly catalog: FrameworkOsBootOptions["catalog"];
  readonly plugins: NonNullable<FrameworkOsBootOptions["plugins"]>;
  readonly documentServices: NonNullable<FrameworkOsBootExecution["documentServices"]>;
  readonly surfaceSessionFactories: NonNullable<FrameworkOsBootOptions["surfaceSessionFactories"]>;
  readonly backboneWorkerFactory?: FrameworkOsBootExecution["backboneWorkerFactory"];
};
export interface SDevBrowserHostV1 { mount(installed: SDevBrowserInventoryV1, operation: SDevOperationV1): Promise<SDevMountV1>; }
export type { SDevMountV1, SDevOperationV1 } from "../🧩️service-composition/🟦️.ts";

/** 🚀 Boots one host with its caller's exact installed inventory, including an empty inventory. */
export async function bootSDevV1(installed: SDevBrowserInventoryV1, host: SDevBrowserHostV1, operation: SDevOperationV1): Promise<SDevMountV1> {
  if (!installed || !Array.isArray(installed.plugins) || !Array.isArray(installed.documentServices) || !Array.isArray(installed.surfaceSessionFactories)) throw new Error("s-dev.missing-installed-inventory");
  try { admitPluginCatalogV1(installed.catalog, { maxBytes: 2097152, maxRows: 128, maxEdges: 4096, maxWork: 65536, deadlineMs: Date.now() + 30000, now: () => Date.now(), cancelled: () => operation.signal.aborted, progress: event => operation.progress({ stage: "mount", completed: event.completed, total: event.total }) }); } catch { operation.signal.throwIfAborted(); throw new Error("s-dev.missing-installed-inventory"); }
  return mountSDevInventoryV1([installed], (inventory, context) => host.mount(inventory, context), operation);
}
