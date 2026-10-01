/** 🗄️ stdio.step TypeScript facade. */
import definition from "./📜️artifact-definition.json" with { type: "json" };

export { definition };
export type ArtifactDefinition = typeof definition;
export type {StepSnapshot,StepHeader,StepEntity,StepComplexType,StepValue,StepFileDescription,StepFileName,StepFileSchema} from "./🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
export type {StepArtifact} from "./🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/🟦️.ts";
export type {StepDiff} from "./🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🟦️.ts";
export type {StepMutation} from "./🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🟦️.ts";
export {STEP_SQLITE_SCHEMA,stepSnapshotToSqliteDatabase,stepSnapshotFromSqliteDatabase} from "./🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🪶️sqlite/🟦️.ts";
