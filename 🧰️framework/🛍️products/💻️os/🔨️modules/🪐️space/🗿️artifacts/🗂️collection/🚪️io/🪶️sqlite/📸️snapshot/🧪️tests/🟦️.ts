import {authoredSnapshotSqliteContract,authoredSnapshotPreflightContract,authoredSnapshotSemanticContract} from "../../../../../../🧪️tests/🪶️sqlite/🔬️oracle/🟦️.ts";
import {fileURLToPath} from "node:url";
authoredSnapshotSqliteContract(fileURLToPath(new URL("../../../../🧬️schema/📸️snapshot",import.meta.url)));

authoredSnapshotPreflightContract(fileURLToPath(new URL("../../../../🧬️schema/📸️snapshot",import.meta.url)));

authoredSnapshotSemanticContract(fileURLToPath(new URL("../../../../🧬️schema/📸️snapshot",import.meta.url)));
