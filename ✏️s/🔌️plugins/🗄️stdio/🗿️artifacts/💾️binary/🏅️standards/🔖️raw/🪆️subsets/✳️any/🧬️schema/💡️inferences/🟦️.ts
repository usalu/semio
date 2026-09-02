/** 💡️ binary inference schema — a raw opaque byte blob's honest real extent (length, emptiness,
 * content digest); deliberately not a fabricated `entries` shape. */

export interface BinaryExtent {
  byteLength: number;
  isEmpty: boolean;
  contentDigest: string;
}

export interface BinaryInference {
  /** @derived */
  extent: BinaryExtent;
}
