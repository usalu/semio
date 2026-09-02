/** 🔺️ SemioAnimationDiff schema — real mirror of `🦀️.rs` (the source of truth). No
 * `snapshot: SemioAnimationSnapshot` full-replace slot anywhere. Collections
 * (timelines/channels/keyframes) are index-keyed removed/modified/added triples
 * (`engine::triples::IndexedTripleDiff<D,T>`), kept generic here since TS supports it. */
import type { AnimTarget, AnimInterpolation, AnimValue, AnimTimeline, AnimChannel, AnimKeyframe } from "../📸️snapshot/🟦️component";

export interface IndexModified<D> { index: number; diff: D; }
export interface IndexAdded<T> { index: number; item: T; }
export interface IndexedTripleDiff<D, T> { removed: number[]; modified: IndexModified<D>[]; added: IndexAdded<T>[]; }

export interface AnimKeyframeDiff {
  t?: number;
  value?: AnimValue;
}

export interface AnimChannelDiff {
  target?: AnimTarget;
  interpolation?: AnimInterpolation;
  keyframes?: IndexedTripleDiff<AnimKeyframeDiff, AnimKeyframe>;
}

export interface AnimTimelineDiff {
  /** tri-state: absent = unchanged, null = name cleared, string = renamed */
  name?: string | null;
  channels?: IndexedTripleDiff<AnimChannelDiff, AnimChannel>;
}

export interface SemioAnimationDiff {
  timelines?: IndexedTripleDiff<AnimTimelineDiff, AnimTimeline>;
}
