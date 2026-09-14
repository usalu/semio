//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { gunzipSync } from "node:zlib";
import { defineTestAdapter } from "../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🔏️Reference
/** 🔏️ FIPS 180-4 SHA-256, from the OpenSSL bindings the runtime ships. */
function digestHex(data: Uint8Array): string {
  return createHash("sha256").update(data).digest("hex");
}

/** 📦️ Splits the length-prefixed frame of gzip members the subject wrote. */
function unframe(raw: Uint8Array): Uint8Array[] {
  const view = new DataView(raw.buffer, raw.byteOffset, raw.byteLength);
  const count = view.getUint32(0);
  const members: Uint8Array[] = [];
  let cursor = 4;
  for (let index = 0; index < count; index += 1) {
    const length = view.getUint32(cursor);
    cursor += 4;
    members.push(raw.subarray(cursor, cursor + length));
    cursor += length;
  }
  return members;
}
//#endregion 🔏️Reference

//#region 🧭️Adapter
/**
 * 🟦️ RFC 1951, RFC 1952 and FIPS 180-4 decide what a gzip member and a SHA-256 digest are; the
 * hand-rolled writer, reader and hash of `📜️statutes` are judged against the runtime's own zlib and
 * OpenSSL bindings. The second scenario inflates the subject's own bytes, so a member this
 * repository wrote must be readable by an implementation that shares no code with it.
 */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "the-digest-is-sha-256": {
      oracle: (ctx) => {
        const file = JSON.parse(new TextDecoder().decode(ctx.fixtureBytes("local://🔣️vectors.json"))) as { digests: string[]; envelope: string };
        const encoder = new TextEncoder();
        return {
          projection: {
            digests: file.digests.map((vector) => digestHex(encoder.encode(vector))),
            envelopeDigest: digestHex(encoder.encode(file.envelope)),
          },
        };
      },
    },
    "a-member-inflates-anywhere": {
      oracle: (ctx) => {
        const file = JSON.parse(new TextDecoder().decode(ctx.fixtureBytes("local://🔣️vectors.json"))) as { payloads: string[] };
        const rawPath = (ctx.plan as unknown as { subjectRawInputs?: Record<string, string> }).subjectRawInputs?.rust;
        if (rawPath === undefined) throw new Error("no raw subject output from rust; run the subject phase before this byte-decoding oracle");
        const members = unframe(new Uint8Array(readFileSync(rawPath)));
        return {
          projection: {
            recovered: members.map((member) => digestHex(new Uint8Array(gunzipSync(member)))),
            count: String(file.payloads.length),
          },
        };
      },
    },
  },
});
//#endregion 🧭️Adapter
