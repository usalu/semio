/** 🗄️ stdio.jpg TypeScript facade. */
import definition from "./📜️artifact-definition.json" with { type: "json" };

export { definition };
export type ArtifactDefinition = typeof definition;
export type {JpgSnapshot,JfifThumbnail,JpgFrameHeader,JpgFrameComponent,JpgQuantTable,JpgHuffmanTable,JpgSegment} from "./🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/🟦️.ts";
export {JPG_SQLITE_SCHEMA,jpgSnapshotToSqliteDatabase,jpgSnapshotFromSqliteDatabase} from "./🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts";
