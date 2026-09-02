/** 🗃 `entries` — the zip snapshot's real central-directory-style census over `entries`. */

export interface ZipEntries {
  entryCount: number;
  totalUncompressedSize: number;
  contentDigest: string;
}
