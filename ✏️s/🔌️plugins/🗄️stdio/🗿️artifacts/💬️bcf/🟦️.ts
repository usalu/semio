/** 🗄️ stdio.bcf TypeScript facade. */
import definition from "./📜️artifact-definition.json" with { type: "json" };

export { definition };
export type ArtifactDefinition = typeof definition;
export type {BcfSnapshot,BcfTopic,BcfViewpoint,BcfCamera,BcfPoint3,BcfComponents,BcfVisibility,BcfColoring,BcfComment,BcfRawPart} from "./🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🧬️schema/📸️snapshot/🟦️.ts";
export {parseBcfSnapshot,parseBcfSnapshotJson,parseBcfTopic,parseBcfViewpoint,parseBcfCamera,parseBcfPoint3,parseBcfComponents,parseBcfVisibility,parseBcfColoring,parseBcfComment,parseBcfRawPart} from "./🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🧬️schema/📸️snapshot/🟦️.ts";
export type {BcfArtifact} from "./🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🧬️schema/🟦️.ts";
export {parseBcfArtifact} from "./🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🧬️schema/🟦️.ts";
export {BCF_SQLITE_SCHEMA,bcfSnapshotToSqliteDatabase,bcfSnapshotFromSqliteDatabase} from "./🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts";
