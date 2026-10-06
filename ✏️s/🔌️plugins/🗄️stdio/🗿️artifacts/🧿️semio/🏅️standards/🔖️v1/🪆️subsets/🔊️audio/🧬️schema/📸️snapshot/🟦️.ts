/** 🧬️ SemioAudioSnapshot schema. */
import type {Binary32} from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
export type SemioAudioFormat = 'pcm8' | 'pcm16' | 'pcm24' | 'pcm32' | 'f32' | 'f64';
export interface SemioAudioChannel {
  samples: Binary32[];
}
export interface SemioAudioTag {
  key: string;
  value: string;
}
export interface SemioAudioSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ sampleRate: number;
  /** @state artifact */ format: SemioAudioFormat;
  /** @state artifact */ channels: SemioAudioChannel[];
  /** @state artifact */ tags: SemioAudioTag[];
}
