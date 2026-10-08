import { composeSpecificOsCatalogV1 } from "../🟦️.ts";
import { PLAYGROUND_BUILD_TARGETS } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts";
import { createCompletedWgpuConfigurationV1 } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/🎚️config/🟦️.ts";
/** 🧊️ Specific standalone assembly injects only its installed generated catalog into the neutral WGPU configuration. */
export default () => {
  const variant = process.env.SEMIO_PLUGIN, playground = PLAYGROUND_BUILD_TARGETS.find(row => row.variant === variant);
  if (!playground) throw new Error("Select a supplied WGPU playground through Nx");
  const rows = composeSpecificOsCatalogV1("http://127.0.0.1:" + (process.env.S_OS_PORT ?? playground.ports.wgpu) + "/", { maxBytes: 2097152, maxRows: 128, maxEdges: 4096, maxWork: 65536, deadlineMs: performance.now() + 30000, now: () => performance.now(), cancelled: () => false, progress: () => {} }).rows;
  return createCompletedWgpuConfigurationV1({ variant, catalog: rows, assets: playground.assets });
};
