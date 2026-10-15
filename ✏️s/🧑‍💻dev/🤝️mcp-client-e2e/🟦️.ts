/** 🤝️ Specific pins of the os MCP client end-to-end journey: the one plugin, artifact kind, verb and inference service the journey dispatches against. The neutral driver (`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts`) owns the journey; this layer owns which plugins it measures. */
import type { McpClientE2ePinsV1 } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts";

/** 🎯️ A gate that picked its target from a fuzzy `capabilities_search` measured whichever plugin BM25 happened to rank first that day: the same journey read `animate` on 2026-09-20 18:00 and `energy` four hours later purely because 20 more descriptors became decodable in between, and `energy` has no compiled component at all. `🗒️note` is the plugin whose guest the two-phase prepare/apply law is proven against (the same `addBlock` verb), and its component is the smallest of the rebuilt set (64 MB against `🌍️gis`'s 202 MB, which is what wedged an earlier run behind a 240 s request wall). `🀄️wfc` declares five real inference services of its own and its component is staged and current, whereas `inference_list` taking `declared[0]` handed the gate `🌍️gis`, whose guest answers `inference instantiate: wasmtime: failed to convert function to given type`. */
export const SPECIFIC_MCP_CLIENT_E2E_PINS_V1: McpClientE2ePinsV1 = {
  pluginId: "note",
  artifactKind: "s.note.note",
  capabilityId: "note.s.note.note@1/*#editor.addBlock",
  searchQuery: "add a block to the note",
  noChangeCapabilityId: "note.s.note.note@1/*#editor.duplicateSelection",
  inferencePluginId: "wfc",
  inferenceArtifactKind: "s.wfc.bitmap",
  inferenceSchema: "s.wfc.bitmap.solve",
};
