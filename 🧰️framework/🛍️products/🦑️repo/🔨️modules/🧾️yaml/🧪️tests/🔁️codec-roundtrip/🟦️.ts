//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { parse, stringify } from "yaml";
import { defineTestAdapter } from "../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🎯️Canonical
/** 🎯️ Canonical JSON: members sorted by key, no insignificant whitespace — the shape every implementation compares in. */
function canonical(value: unknown): string {
  if (value === null || value === undefined) return "null";
  if (Array.isArray(value)) return `[${value.map(canonical).join(",")}]`;
  if (typeof value === "object") {
    if (value instanceof Map) return canonical(Object.fromEntries(value));
    const entries = Object.entries(value as Record<string, unknown>).sort(([left], [right]) => (left < right ? -1 : left > right ? 1 : 0));
    return `{${entries.map(([key, member]) => `${JSON.stringify(key)}:${canonical(member)}`).join(",")}}`;
  }
  return JSON.stringify(value);
}
//#endregion 🎯️Canonical

//#region 🧭️Adapter
/** 🟦️ The `yaml` package decides what each vector means; this repository's decoder is judged against it. */
/** 🧫️ The vectors the oracle decides, read straight from the immutable fixture. */
function vectorsOf(ctx: { fixtureBytes(uri: string): Uint8Array }): { name: string; source: string }[] {
  return (JSON.parse(new TextDecoder().decode(ctx.fixtureBytes("shared://📡️codec-vectors.json"))) as { vectors: { name: string; source: string }[] }).vectors;
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "vectors-decode-to-the-same-value": {
      oracle: (ctx) => ({
        projection: {
          decoded: vectorsOf(ctx).map((vector) => {
            try {
              return `${vector.name}=${canonical(parse(vector.source))}`;
            } catch {
              return `${vector.name}!error`;
            }
          }),
        },
      }),
    },
    "decode-encode-decode-is-idempotent": {
      oracle: (ctx) => ({
        projection: {
          reDecoded: vectorsOf(ctx).map((vector) => {
            try {
              return `${vector.name}=${canonical(parse(stringify(parse(vector.source))))}`;
            } catch {
              return `${vector.name}!decode`;
            }
          }),
        },
      }),
    },
  },
});
//#endregion 🧭️Adapter
