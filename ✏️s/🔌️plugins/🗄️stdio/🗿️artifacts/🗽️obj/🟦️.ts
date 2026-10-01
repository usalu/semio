/** 🗄️ stdio.obj TypeScript facade. */
import definition from "./📜️artifact-definition.json" with { type: "json" };

export { definition };
export type ArtifactDefinition = typeof definition;

export type { ObjSnapshot, ObjVertex, ObjTexCoord, ObjNormal, ObjFace, ObjFaceVertex, ObjGroup, ObjObject, ObjUsemtlRange, ObjSmoothingRange, ObjUnknownStatement } from "./🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/📸️snapshot/🟦️.ts";
export { OBJ_SQLITE_SCHEMA, objSnapshotToSqliteDatabase, objSnapshotFromSqliteDatabase, objSnapshotValidateSqliteSubset } from "./🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/📸️snapshot/🪶️sqlite/🟦️.ts";
