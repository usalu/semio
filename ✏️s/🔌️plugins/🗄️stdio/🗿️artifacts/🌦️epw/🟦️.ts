/** 🎪 stdio.epw TypeScript facade. */
import definition from "./📜️artifact-definition.json" with { type: "json" };

export { definition };
export type ArtifactDefinition = typeof definition;
export type {EpwArtifact} from "./🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/🟦️.ts";

export type {EpwSnapshot,EpwLocation,EpwDataPeriod,EpwDataPeriods,EpwRecord} from "./🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts";
export {parseEpwSnapshot,parseEpwLocation,parseEpwDataPeriod,parseEpwDataPeriods,parseEpwRecord} from "./🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts";
export {EPW_SQLITE_SCHEMA,epwSnapshotToSqliteDatabase,epwSnapshotFromSqliteDatabase} from "./🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts";
