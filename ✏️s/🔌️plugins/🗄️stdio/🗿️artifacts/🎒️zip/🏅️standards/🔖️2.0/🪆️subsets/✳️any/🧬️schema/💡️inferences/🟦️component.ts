/** 💡️ zip inference schema — real central-directory-style census over decompressed `entries`. */

export interface ZipEntries {
  entryCount: number;
  totalUncompressedSize: number;
  contentDigest: string;
}

export interface ZipInference {
  /** @state inferred */
  entries: ZipEntries;
}
