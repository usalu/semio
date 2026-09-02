/** 📏 `extent` — the ONLY honest census an opaque byte blob supports (real byte length, emptiness,
 * content digest) — deliberately not a fabricated `entries` shape. */

export interface BinaryExtent {
  byteLength: number;
  isEmpty: boolean;
  contentDigest: string;
}
