/** 🎪 stdio.avi TypeScript facade. */
import definition from "./📜️artifact-definition.json" with { type: "json" };

export { definition };
export type ArtifactDefinition = typeof definition;
export type {AviArtifact} from "./🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🧬️schema/🟦️.ts";
export type {AviSnapshot,AviMainHeader,AviStreamHeader,AviStreamFormat,AviChunk,AviStream,RiffChunk} from "./🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🧬️schema/📸️snapshot/🟦️.ts";
export {parseAviSnapshot,parseAviMainHeader,parseAviStreamHeader,parseAviStreamFormat,parseAviChunk,parseAviStream,parseRiffChunk} from "./🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🧬️schema/📸️snapshot/🟦️.ts";
export {AVI_SQLITE_SCHEMA,aviSnapshotToSqliteDatabase,aviSnapshotFromSqliteDatabase} from "./🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts";
