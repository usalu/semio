/** 🧬️ ZipSnapshot schema. */
export interface ZipEntry {
  name: string;
  data: number[];
}
export interface ZipSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: ZipEntry[];
}
