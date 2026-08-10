/** 🧬️ PdfArtifact schema. */
export interface PdfEntry {
  name: string;
  data: number[];
}
export interface PdfArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: PdfEntry[];
}
