/** 🧬️ ZipSnapshot schema — complete per APPNOTE (local header + central directory + EOCD
 * fields this artifact models). */

export type ZipCompressionMethod = 'stored' | 'deflate';

/** 🧩️ One raw local/central "extra field" record (id + payload), kept verbatim for any id this
 * artifact doesn't specially interpret. */
export interface ZipExtraField {
  id: number;
  payload: number[];
}

/** 🎒️ One ZIP archive member. `data` is always the decompressed payload. */
export interface ZipEntry {
  name: string;
  data: number[];
  method: ZipCompressionMethod;
  /** Raw MS-DOS date (local-file-header layout, APPNOTE 4.4.6). */
  dosDate: number;
  /** Raw MS-DOS time (local-file-header layout, APPNOTE 4.4.6). */
  dosTime: number;
  /** Real-world UTC mtime decoded from an Info-ZIP `UT` (0x5455) extra-field record, if present. */
  unixMtime?: number | null;
  /** General-purpose bit flags as read from the central directory. */
  flags: number;
  versionMadeBy: number;
  versionNeeded: number;
  internalAttrs: number;
  externalAttrs: number;
  /** Extra-field records as they appeared in the local file header. */
  localExtra: ZipExtraField[];
  /** Extra-field records as they appeared in the central directory header. */
  centralExtra: ZipExtraField[];
  comment: string;
}

/** 📸️ Persisted `stdio.zip` snapshot. */
export interface ZipSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ entries: ZipEntry[];
  /** @state artifact — archive-level comment (EOCD comment field). */ comment: string;
}
