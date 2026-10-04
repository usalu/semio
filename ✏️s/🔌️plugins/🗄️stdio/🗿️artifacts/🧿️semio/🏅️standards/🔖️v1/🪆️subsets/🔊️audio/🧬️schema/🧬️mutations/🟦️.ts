/** 🧬️ SemioAudioMutation union. */
import type { SemioAudioChannel, SemioAudioFormat, SemioAudioSnapshot, SemioAudioTag } from '../📸️snapshot/🟦️.ts';
import type { SnapshotPatch } from '../../../../../../../../📇️registry/🧬️contract/✏️editing/🩹️patch/🟦️.ts';

export type SemioAudioMutation =
  | { mutation: 'setSnapshot'; snapshot: SemioAudioSnapshot }
  | { readonly mutation: 'patchSnapshot'; readonly patch: SnapshotPatch }
  | { mutation: 'setSampleRate'; sampleRate: number }
  | { mutation: 'setFormat'; format: SemioAudioFormat }
  | { mutation: 'insertChannel'; index: number; channel: SemioAudioChannel }
  | { mutation: 'removeChannel'; index: number }
  | { mutation: 'setChannelSamples'; index: number; samples: SemioAudioChannel['samples'] }
  | { mutation: 'insertTag'; index: number; tag: SemioAudioTag }
  | { mutation: 'removeTag'; index: number }
  | { mutation: 'setTagValue'; index: number; value: string };
