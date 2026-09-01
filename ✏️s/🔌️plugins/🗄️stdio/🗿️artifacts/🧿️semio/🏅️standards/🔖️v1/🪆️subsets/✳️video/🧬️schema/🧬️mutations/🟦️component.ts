/** 🧬️ SemioVideoMutation — named-variant enum, discriminated on `mutation`. Mirrors
 * `🧬️mutations/🦀️component.rs` field for field. */
import type { SemioRational, SemioVideoSample, SemioVideoSnapshot, SemioVideoStream, SemioVideoStreamKind } from "../📸️snapshot/🟦️component.ts";

export type SemioVideoMutation =
  | { mutation: "setSnapshot"; snapshot: SemioVideoSnapshot }
  | { mutation: "insertStream"; index: number; stream: SemioVideoStream }
  | { mutation: "removeStream"; index: number }
  | { mutation: "setStreamMeta"; index: number; kind: SemioVideoStreamKind; codec: string; width: number; height: number; rate: SemioRational }
  | { mutation: "insertSample"; streamIndex: number; index: number; sample: SemioVideoSample }
  | { mutation: "removeSample"; streamIndex: number; index: number }
  | { mutation: "setSampleData"; streamIndex: number; index: number; data: number[] }
  | { mutation: "setSampleFlags"; streamIndex: number; index: number; pts: number; key: boolean };
