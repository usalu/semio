/** 📝️ Text representation for `stdio.semio.animation.diff`. */
import type { AnimChannelDiff } from "../../../🧬️schema/🔺️diff/🟦️.ts";
import type { AnimChannel, AnimTimeline } from "../📸️snapshot/🟦️.ts";
export interface SemioAnimationDiff { timelines?: IndexedTripleDiff<AnimTimelineDiff, AnimTimeline> }
export interface IndexedTripleDiff<D, T> { removed: number[]; modified: { index: number; diff: D }[]; added: { index: number; item: T }[] }
export interface AnimTimelineDiff { name?: string | null; channels?: IndexedTripleDiff<AnimChannelDiff, AnimChannel> }
