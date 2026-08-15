/** 🎒️ One logical ZIP member with decompressed semantic content. */
export interface ZipEntry {
  name: string;
  data: number[];
}

/** 📸️ Logical `stdio.zip` snapshot. Entries are keyed and normalized by name. */
export interface ZipSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ entries: ZipEntry[];
  /** @state artifact */ comment: string;
}
