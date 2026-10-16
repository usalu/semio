/** 🗄️ TypeScript access to the portable SQLite snapshot metadata DDL, byte-identical to the text the Rust implementation includes. */
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

export const sqliteSnapshotMetadataSql: string = readFileSync(fileURLToPath(new URL("./🗄️.sql", import.meta.url)), "utf8");
