/** 🗄️ stdio.pdf TypeScript facade. */
import definition from "./📜️artifact-definition.json" with { type: "json" };

export { definition };
export type ArtifactDefinition = typeof definition;
export { PDF17_SQLITE_SCHEMA } from "./🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧬️schema/🟦️.ts";
export { pdf17SnapshotToSqliteDatabase,pdf17SnapshotFromSqliteDatabase } from "./🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts";
export type { PdfSnapshot,PdfPage,Binary64 } from "./🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
export { PDF14_SQLITE_SCHEMA, pdf14SnapshotToSqliteDatabase, pdf14SnapshotFromSqliteDatabase } from "./🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts";
