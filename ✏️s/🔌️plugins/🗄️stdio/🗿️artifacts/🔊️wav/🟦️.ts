/** 🎪 stdio.wav TypeScript facade. */
import definition from "./📜️artifact-definition.json" with { type: "json" };

export { definition };
export type ArtifactDefinition = typeof definition;
export type {WavSnapshot,WavFmt,WavData,RiffChunk,WavChunkRef} from "./🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts";
export {parseWavSnapshot,parseWavFmt,parseWavData,parseRiffChunk,parseWavChunkRef,validateWavSerialization} from "./🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts";
export {WAV_SQLITE_SCHEMA,wavSnapshotToSqliteDatabase,wavSnapshotFromSqliteDatabase} from "./🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🟦️.ts";
export {binary32,binary32Value} from "../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
export type {Binary32} from "../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
