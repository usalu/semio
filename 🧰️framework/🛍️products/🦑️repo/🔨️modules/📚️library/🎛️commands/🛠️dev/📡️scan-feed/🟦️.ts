import { dirname, join } from "node:path";
import { changeJournalDirectoryV1, connectChangeJournalV1 } from "../../../🏗️build/🏡️resident/📜️journal/🟦️.ts";

/** 📡️ The ports of the graph-free generated-input scan for one workspace: the fenced change journal (it starts the service on demand and refuses whenever it cannot vouch) and the machine-local directory where processes share the scan snapshot, next to the journal state, which no `TMPDIR` moves.
 * @see ../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🏭️generated-owner/🗂️index/🟦️.ts */
export function generatedScanPortsV1(root: string, environment: Readonly<Record<string, string | undefined>> = process.env) {
  return Object.freeze({ changes: connectChangeJournalV1(root, environment), store: join(dirname(changeJournalDirectoryV1(root, environment)), "generated-owner") });
}
