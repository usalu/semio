/** 🔺️ diff fragment for `CreateLayer` — one `layers.added` entry. */
export interface CreateLayerDiff {
  layers?: { added: { index: number; item: unknown }[] };
}
