/** 🧬️ SemioAudioArtifact schema. */
export interface SemioAudioArtifactEntry {
  name: string;
  data: number[];
}
export interface SemioAudioArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioAudioArtifactEntry[];
}
