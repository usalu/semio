/** 🔊️ The artifact owns the same typed sample/channel/tag state as its native snapshot. */
import type {SemioAudioSnapshot} from "./📸️snapshot/🟦️.ts";
export type {SemioAudioChannel,SemioAudioFormat,SemioAudioTag} from "./📸️snapshot/🟦️.ts";
export interface SemioAudioArtifact extends SemioAudioSnapshot {}
