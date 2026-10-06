/** 🗄️ stdio.json TypeScript facade. */
import definition from "./📜️artifact-definition.json" with { type: "json" };

export { definition };
export type ArtifactDefinition = typeof definition;

export { jsonSnapshotToSqliteDatabase, jsonSnapshotFromSqliteDatabase, JSON_SQLITE_SCHEMA } from "./🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts";
export { parseJsonSnapshot } from "./🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
export type { JsonSnapshot, JsonValue } from "./🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
export { checkGeoJsonConformanceControlled, validateGeoJsonSnapshotSqliteDialect } from "./🏅️standards/🔖️rfc8259/🪆️subsets/🌍️geojson/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts";
export type { GeoJsonSqliteDiagnostic } from "./🏅️standards/🔖️rfc8259/🪆️subsets/🌍️geojson/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts";

export {checkIJsonConformanceControlled,validateIJsonSnapshotSqliteDialect} from "./🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts";
export type {IJsonSqliteDiagnostic} from "./🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts";
