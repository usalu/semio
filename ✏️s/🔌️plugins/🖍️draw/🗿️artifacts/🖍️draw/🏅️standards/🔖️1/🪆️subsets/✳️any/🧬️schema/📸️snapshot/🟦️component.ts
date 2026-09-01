/** 📸️ Mirrors Rust `DrawSnapshot` (persisted draw document snapshot — artifact-lane fields only;
 * sibling `🦀️component.rs`, `#[serde(rename_all = "camelCase")]`). Nested types re-import the
 * artifact's own root schema (`../🟦️component.ts`) rather than re-declaring stubs, so every facet
 * of the draw artifact agrees on the same `DrawLayerNode`/`DrawImageAsset`/`DrawArtboard`. */
import type { DrawLayerNode, DrawImageAsset, DrawArtboard } from "../🟦️component.ts";

export interface DrawSnapshot {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  id: string;
  /** @state artifact */
  title?: string;
  /** @state artifact */
  layers: DrawLayerNode[];
  /** @state artifact */
  assets: Record<string, DrawImageAsset>;
  /** @state artifact */
  artboard?: DrawArtboard;
}
