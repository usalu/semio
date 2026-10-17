/**
 * 🕸️ The wasm engine packages of the page bound to the loader contract the React renderer declares: each package is imported when a surface first asks, so a page pays for the engines its documents use.
 * @see ../../../📺️renderer/🧑‍🎨engine/🧱️elements/🪪️WasmSessionLoader/🟦️.tsx
 */
import type { WasmBindingsV1 } from "@semio-tech/framework-renderer-react";

/** 🕸️ Surface, flow, editor and node-graph engines. */
export const WASM_BINDINGS_V1: WasmBindingsV1 = Object.freeze({
  surface: () => import("@semio-tech/framework-surface-rs"),
  flow: async () => {
    const [core, browser] = await Promise.all([import("@semio-tech/flow-core"), import("@semio-tech/flow-core/🌐️flow-browser.js")]);
    return { core, browser };
  },
  editor: () => import("@semio-tech/framework-editor-rs"),
  nodeGraph: () => import("@semio-tech/framework-os-node-graph-rs/framework_os_node_graph.js"),
});
