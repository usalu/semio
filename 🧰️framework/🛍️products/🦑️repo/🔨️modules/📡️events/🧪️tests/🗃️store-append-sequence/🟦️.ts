//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { createHash } from "node:crypto";
import { defineTestAdapter } from "../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🔮️Oracle
type StoreInput = { id: string; kind: string; data: unknown };
type StoreVectors = { inputs: StoreInput[]; sequences: number[]; interruptPhases: string[] };

const SCHEMA = "semio.event/1";

/** 🔏️ OpenSSL's SHA-256 over the record preimage the implementations claim to hash. */
const recordChecksum = (sequence: number, input: StoreInput): string =>
  createHash("sha256")
    .update(`${SCHEMA}\u0000${sequence}\u0000${input.id}\u0000${input.kind}\u0000`, "utf8")
    .update(JSON.stringify(input.data), "utf8")
    .digest("hex");

/** 📄️ The exact JSONL an implementation must write for these inputs. */
const logLines = (vectors: StoreVectors): string =>
  vectors.inputs
    .map((input, index) =>
      JSON.stringify({
        schema: SCHEMA,
        sequence: index + 1,
        id: input.id,
        kind: input.kind,
        data: input.data,
        checksum: recordChecksum(index + 1, input),
      }),
    )
    .join("\n") + "\n";

const readVectors = (ctx: { fixtureBytes(uri: string): Uint8Array }): StoreVectors =>
  JSON.parse(new TextDecoder().decode(ctx.fixtureBytes("shared://🗄️store-vectors.json")));
//#endregion 🔮️Oracle

//#region 🧭️Adapter
/** 🟦️ node:crypto oracle for the append-only store: the checksums and the log bytes it implies. */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "append-then-replay-sequences": {
      oracle: (ctx) => {
        const vectors = readVectors(ctx);
        return {
          projection: {
            sequences: vectors.sequences,
            checksums: vectors.inputs.map((input, index) => recordChecksum(index + 1, input)),
            logDigest: createHash("sha256").update(logLines(vectors), "utf8").digest("hex"),
            deterministic: true,
          },
        };
      },
    },
    "duplicate-and-corrupt-are-refused": {
      oracle: () => ({ projection: { duplicateRefused: true, corruptDetected: true } }),
    },
    "interruption-preserves-committed-log": {
      oracle: (ctx) => ({
        projection: {
          phases: readVectors(ctx).interruptPhases.map((phase) => ({
            phase,
            cancelled: true,
            unchanged: true,
            stageGone: true,
            replayCount: 1,
            replayOk: true,
          })),
        },
      }),
    },
    "record-checksum-is-sha256": {
      oracle: (ctx) => {
        const vectors = readVectors(ctx);
        return {
          projection: {
            records: vectors.inputs.map((input, index) => ({
              id: input.id,
              kind: input.kind,
              sequence: index + 1,
              data: JSON.stringify(input.data),
              checksum: recordChecksum(index + 1, input),
            })),
          },
        };
      },
    },
  },
});
//#endregion 🧭️Adapter
