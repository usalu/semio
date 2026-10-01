/** 🎪 stdio.mp3 TypeScript facade. */
import definition from "./📜️artifact-definition.json" with { type: "json" };

export { definition };
export type ArtifactDefinition = typeof definition;
export type {Mp3Snapshot,Id3Frame,Id3v2Tag,Id3v1Tag,Mp3FrameHeader,Mp3Frame} from "./🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts";
export {parseMp3Snapshot,parseId3Frame,parseId3v2Tag,parseId3v1Tag,parseMp3FrameHeader,parseMp3Frame} from "./🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts";
export {MP3_SQLITE_SCHEMA,mp3SnapshotToSqliteDatabase,mp3SnapshotFromSqliteDatabase} from "./🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🟦️.ts";
