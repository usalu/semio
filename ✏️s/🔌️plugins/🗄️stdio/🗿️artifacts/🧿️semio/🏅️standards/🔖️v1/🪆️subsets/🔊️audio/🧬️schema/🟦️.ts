/** 🧬️ SemioAudioArtifact schema. */
export interface SemioAudioArtifactEntry {
  name: string;
  data: number[];
}
export interface SemioAudioArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ entries: SemioAudioArtifactEntry[];
}
