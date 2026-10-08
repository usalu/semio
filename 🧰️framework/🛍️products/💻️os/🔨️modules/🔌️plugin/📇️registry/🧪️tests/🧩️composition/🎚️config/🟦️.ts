import base from "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts";
import { defineConfig } from "vitest/config";
import { resolve } from "node:path";
const paths = (rows: readonly string[]) => rows.map(path => resolve(import.meta.dirname, path));
const node = paths(["../../../../../../../../🔨️modules/🔲️pixels/🎯️selection/🧪️tests/🟦️.ts","../../✅️catalog-complete/🟦️.ts", "../../🎬️host-activation/🟦️.ts", "../../📖️generated-projection/🟦️.ts", "../../../../../📺️renderer/🧑‍🎨engine/🧪️tests/⏱️wgpu-worker-step-budget/🟦️.ts"]);
const renderer = paths(["../../../../../📺️renderer/🧑‍🎨engine/🧪️tests/🪆️embedded-mount/🟦️.ts", "../../../../../📺️renderer/🧑‍🎨engine/🧪️tests/📨️browser-frame-transport/🟦️.ts", "../../../../../📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-extension-dispatch/🟦️.ts"]);
const runtime = paths(["../../../../../📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx"]);
/** 🧪️ Retains the full original Node producer laws and actual renderer in-source runtime controls. */
export default defineConfig({ ...base, resolve: { ...base.resolve, alias: [{ find: "@semio-tech/framework-os-node-graph-rs", replacement: resolve(import.meta.dirname, "../../../../../../../../🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts") }, ...base.resolve!.alias as any[]] }, test: { ...base.test, include: [], includeSource: [], projects: [{ extends: true, test: { name: "catalog-producers", environment: "node", include: node } }, { extends: true, test: { name: "catalog-renderers", environment: "jsdom", include: renderer, includeSource: runtime } }], testTimeout: 30000, hookTimeout: 30000 } });
