/** 🧬️ ZipArtifact schema. */
export interface ZipEntry {
  name: string;
  data: number[];
}
export interface ZipArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ entries: ZipEntry[];
}
