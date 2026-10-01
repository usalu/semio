/** 🗄️ stdio.zip TypeScript facade. */
import definition from "./📜️artifact-definition.json" with { type: "json" };

export { definition };
export type ArtifactDefinition = typeof definition;

export type {ZipSnapshot,ZipEntry,ZipEntryMetadata,ZipLocalHeaderMetadata,ZipCentralHeaderMetadata,ZipExtraField} from "./🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
export {ZIP_SQLITE_SCHEMA,zipSnapshotToSqliteDatabase,zipSnapshotFromSqliteDatabase,zipSnapshotValidateSqliteSubset} from "./🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🪶️sqlite/🟦️.ts";
export {checkZipIso21320Conformance,type ZipIso21320Diagnostic} from "./🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/🧬️schema/🟦️.ts";

export {parseZipSnapshot} from "./🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
