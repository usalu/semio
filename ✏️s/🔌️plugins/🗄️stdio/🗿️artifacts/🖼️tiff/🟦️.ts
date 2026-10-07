/** 🗄️ stdio.tiff TypeScript facade. */
import definition from "./📜️artifact-definition.json" with { type: "json" };

export { definition };
export type ArtifactDefinition = typeof definition;

export type {TiffSnapshot,TiffIfd,TiffTag,TiffValues,TiffFieldType,TiffWord64,TiffBinary32,TiffSampleBlock} from "./🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/🟦️.ts";
export {decodeTiffSnapshot,encodeTiffSnapshot,type TiffNativeOptions} from "./🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🚪️io/💾️binary/📸️snapshot/🟦️.ts";
export {parseTiffSnapshot,parseTiffIfd,parseTiffTag,parseTiffValues,parseTiffFieldType} from "./🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/🟦️.ts";
export {TIFF_SQLITE_SCHEMA,tiffSnapshotToSqliteDatabase,tiffSnapshotFromSqliteDatabase} from "./🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts";
