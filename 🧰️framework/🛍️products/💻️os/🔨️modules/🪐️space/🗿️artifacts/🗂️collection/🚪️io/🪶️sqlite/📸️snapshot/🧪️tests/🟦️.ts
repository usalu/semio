import {authoredSnapshotSqliteContract,authoredSnapshotPreflightContract,authoredSnapshotSemanticContract} from "../../../../../../🧪️tests/🪶️sqlite/🔬️oracle/🟦️.ts";
import {fileURLToPath} from "node:url";
authoredSnapshotSqliteContract({sql:fileURLToPath(new URL("../🗄️.sql",import.meta.url)),fixtures:fileURLToPath(new URL("../🧫️fixtures",import.meta.url))});

authoredSnapshotPreflightContract({sql:fileURLToPath(new URL("../🗄️.sql",import.meta.url)),fixtures:fileURLToPath(new URL("../🧫️fixtures",import.meta.url))});

authoredSnapshotSemanticContract({sql:fileURLToPath(new URL("../🗄️.sql",import.meta.url)),fixtures:fileURLToPath(new URL("../🧫️fixtures",import.meta.url))});
