import nativeCatalog from "../🤖️generated/🦀️catalog.json";
import {readNativeCatalogV1} from "@semio-tech/plugin-registry/runtime";
import { composeSpecificOsCatalogV1 } from "../🟦️.ts";
import { PLAYGROUND_BUILD_TARGETS } from "../🤖️generated/🎮️playgrounds/🟦️.ts";
import { createCompletedWgpuConfigurationV1, type WgpuCapturedArtifactInputsV1 } from "@semio-tech/framework-renderer-wgpu/server/config";
/** 🧊️ Specific standalone assembly injects only its installed generated catalog into the neutral WGPU configuration. */
function configuration(variant: string | undefined, capture?: WgpuCapturedArtifactInputsV1) {
  const playground = PLAYGROUND_BUILD_TARGETS.find(row => row.variant === variant);
  if (!playground) throw new Error("Select a supplied WGPU playground through the build tool");
  const rows = composeSpecificOsCatalogV1("http://127.0.0.1:" + (process.env.S_OS_PORT ?? playground.ports.wgpu) + "/", { maxBytes: 2097152, maxRows: 512, maxEdges: 4096, maxWork: 65536, deadlineMs: performance.now() + 30000, now: () => performance.now(), cancelled: () => false, progress: () => {} }).rows;
  return createCompletedWgpuConfigurationV1({ variant, catalog: rows, nativeCatalog:readNativeCatalogV1(nativeCatalog), assets: playground.assets }, capture);
}

export function createCapturedWgpuConfigurationV1(capture: WgpuCapturedArtifactInputsV1, variant: string) {
  return configuration(variant, capture);
}

export default () => configuration(process.env.SEMIO_PLUGIN);
