/** 🗄️ stdio.md TypeScript facade. */
import definition from "./📜️artifact-definition.json" with { type: "json" };

export { definition };
export type ArtifactDefinition = typeof definition;
export type {MdSnapshot,MdBlock,MdInline} from "./🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts";
export {parseMdSnapshot,parseMdBlock,parseMdInline} from "./🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts";
export {MD_SQLITE_SCHEMA,mdSnapshotToSqliteDatabase,mdSnapshotFromSqliteDatabase,validateMdSnapshotSqliteDialect} from "./🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts";
