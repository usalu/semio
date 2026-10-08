import type { FrameworkOsBootOptions } from "@semio-tech/framework-renderer-react";
import type { SDevBrowserHostV1 } from "../🟦️.ts";

/** ⚛️ Binds one Specific browser mount to the actual General React receiver and its exact installed inventory. */
export function createSDevReactHostV1(configuration: Pick<FrameworkOsBootOptions, "rootId" | "plugin" | "appId" | "appRole" | "locks" | "defaults" | "brand">): SDevBrowserHostV1 {
  return { mount: async (installed, operation) => {
    operation.signal.throwIfAborted();
    const { bootFrameworkOs } = await import("@semio-tech/framework-renderer-react");
    operation.signal.throwIfAborted();
    return bootFrameworkOs({ ...configuration, catalog: installed.catalog, plugins: installed.plugins, surfaceSessionFactories: installed.surfaceSessionFactories }, { backboneWorkerFactory: installed.backboneWorkerFactory, documentServices: installed.documentServices });
  } };
}
