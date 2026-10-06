/** 🗄️ stdio.tiff TypeScript facade. */
import definition from "./📜️artifact-definition.json" with { type: "json" };

export { definition };
export type ArtifactDefinition = typeof definition;

export type {TiffSnapshot,TiffIfd,TiffTag,TiffValues,TiffFieldType,TiffByteOrder,Binary32,Binary64} from "./🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/🟦️.ts";
export {parseTiffSnapshot,parseTiffIfd,parseTiffTag,parseTiffValues,parseTiffFieldType,parseTiffByteOrder} from "./🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/🟦️.ts";
export {TIFF_SQLITE_SCHEMA,tiffSnapshotToSqliteDatabase,tiffSnapshotFromSqliteDatabase} from "./🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts";
