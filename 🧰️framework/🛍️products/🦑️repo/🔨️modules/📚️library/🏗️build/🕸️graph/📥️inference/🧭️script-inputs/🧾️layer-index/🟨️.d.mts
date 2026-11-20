/** 🪪️ The persisted row dictionary and row-id lists of the stored closure layers. */
export type LayerListingV1 = Readonly<{ ids: Uint32Array; dictionary: object; rowSync(id: number): unknown }>;
export type LayerIndexV1 = Readonly<{
  listingSync(digest: string): LayerListingV1 | undefined;
  idsSync(digest: string): Uint32Array | undefined;
  rowSync(id: number): unknown;
  indexSync(digest: string, rows: readonly unknown[]): Uint32Array | undefined;
  sweepSync(layers: string): { removed: number; reset: boolean };
  stats(): { indexed: number; read: number; rowReads: number; full: number; refused: number; fingerprints: number };
}>;
export function createLayerIndexV1(options: { directory: string; limits?: Record<string, number> }): LayerIndexV1;
