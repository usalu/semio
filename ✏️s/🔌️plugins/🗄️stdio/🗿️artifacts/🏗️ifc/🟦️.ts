/** 🗄️ stdio.ifc TypeScript facade. */
import definition from "./📜️artifact-definition.json" with { type: "json" };

export { definition };
export type ArtifactDefinition = typeof definition;
export type {IfcSnapshot,IfcValue,IfcEntity,IfcComplexType,IfcHeader} from "./🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts";
export type {IfcArtifact} from "./🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🟦️.ts";
export type {IfcDiff} from "./🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts";
export type {IfcMutation} from "./🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts";
export {IFC_SQLITE_SCHEMA,ifcSnapshotToSqliteDatabase,ifcSnapshotFromSqliteDatabase} from "./🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🟦️.ts";
export type {Ifc2x3Snapshot,Ifc2x3EdmPreamble,Part21Decimal,Part21Value,Part21Document,Part21Header,Part21Instance,Part21Entity} from "./🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
export type {Ifc2x3Artifact} from "./🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/🟦️.ts";
export type {Ifc2x3Diff} from "./🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🟦️.ts";
export type {Ifc2x3Mutation} from "./🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🟦️.ts";
export {IFC2X3_SQLITE_SCHEMA,ifc2x3SnapshotToSqliteDatabase,ifc2x3SnapshotFromSqliteDatabase} from "./🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🪶️sqlite/🟦️.ts";
