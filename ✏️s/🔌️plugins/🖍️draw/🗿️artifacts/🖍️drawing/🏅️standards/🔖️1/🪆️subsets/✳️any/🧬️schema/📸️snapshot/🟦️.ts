/** 📸️ Mirrors Rust `DrawingSnapshot` (persisted drawing document snapshot — artifact-lane fields only;
 * sibling `🦀️.rs`, `#[serde(rename_all = "camelCase")]`). Nested types re-import the
 * artifact's own root schema (`../🟦️.ts`) rather than re-declaring stubs, so every facet
 * of the drawing artifact agrees on the same `DrawingLayerNode`/`DrawingImageAsset`/`DrawingArtboard`. */
import type { DrawingLayerNode, DrawingImageAsset, DrawingArtboard } from "../🟦️.ts";

export interface DrawingSnapshot {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  id: string;
  /** @state artifact */
  title?: string;
  /** @state artifact */
  layers: DrawingLayerNode[];
  /** @state artifact */
  assets: Record<string, DrawingImageAsset>;
  /** @state artifact */
  artboard?: DrawingArtboard;
}
