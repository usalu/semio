/** 🧬️ SemioAudioSnapshot schema. */
export type SemioAudioFormat = 'pcm8' | 'pcm16' | 'pcm24' | 'pcm32' | 'f32' | 'f64';
export interface SemioAudioChannel {
  samples: number[];
}
export interface SemioAudioTag {
  key: string;
  value: string;
}
export interface SemioAudioSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ sampleRate: number;
  /** @state persistent */ format: SemioAudioFormat;
  /** @state persistent */ channels: SemioAudioChannel[];
  /** @state persistent */ tags: SemioAudioTag[];
}
