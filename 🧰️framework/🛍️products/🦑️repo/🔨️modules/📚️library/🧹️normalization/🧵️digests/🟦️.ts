import { writeFileSync } from "node:fs";
import { referenceDigestRecordLines } from "../🟦️.ts";

/** 🧵️ One reference digest work order: every leaf to digest (largest first), the cursor all workers claim leaves from, where to publish the
 * `key<TAB>digest` lines, and the shared status words (`[0]`: 0 running, 1 done, 2 failed; `[1]`: leaves processed). */
interface ReferenceDigestWorkOrder {
  readonly repoRoot: string;
  readonly rows: readonly (readonly [path: string, contentHash: string])[];
  readonly opaqueLeaves: readonly string[];
  readonly out: string;
  readonly status: SharedArrayBuffer;
  readonly cursor: SharedArrayBuffer;
}

const CLAIM = 8;

(globalThis as unknown as { onmessage: (event: { data: ReferenceDigestWorkOrder }) => void }).onmessage = ({ data }) => {
  const status = new Int32Array(data.status), cursor = new Int32Array(data.cursor), lines: string[] = [];
  try {
    for (let start = Atomics.add(cursor, 0, CLAIM); start < data.rows.length; start = Atomics.add(cursor, 0, CLAIM)) {
      const chunk = referenceDigestRecordLines(data.repoRoot, data.rows.slice(start, start + CLAIM), data.opaqueLeaves, () => Atomics.add(status, 1, 1));
      if (chunk) lines.push(chunk);
    }
    writeFileSync(data.out, lines.join("\n"));
    Atomics.store(status, 0, 1);
  } catch (error) {
    writeFileSync(`${data.out}.error`, error instanceof Error ? error.stack ?? error.message : String(error));
    Atomics.store(status, 0, 2);
  }
  Atomics.notify(status, 0);
};
