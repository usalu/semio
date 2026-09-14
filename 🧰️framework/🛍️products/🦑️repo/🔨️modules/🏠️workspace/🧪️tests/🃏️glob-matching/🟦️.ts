//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//#region 🔌️Adapters
import micromatch from "micromatch";
import { defineTestAdapter } from "../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🔖️Vectors
type GlobVector = { name: string; pattern: string; path: string };

/** 🃏️ One verdict per vector of the named set, rendered the way every implementation reports it. */
const globVerdicts = (ctx: { fixtureBytes: (reference: string) => Uint8Array }, set: string): string[] => {
  const file = JSON.parse(new TextDecoder().decode(ctx.fixtureBytes("shared://📡️glob-vectors.json"))) as Record<string, GlobVector[]>;
  return file[set].map((vector) => `${vector.name}=${micromatch.isMatch(vector.path, vector.pattern)}`);
};
//#endregion 🔖️Vectors

//#region 🧭️Adapter
/** 🟦️ micromatch decides what each pattern means; the owned matcher is judged against it. */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "vectors-match-the-same-way": {
      oracle: (ctx) => ({ projection: { verdicts: globVerdicts(ctx, "vectors") } }),
    },
    "brace-alternation-expands-the-same-way": {
      oracle: (ctx) => ({ projection: { verdicts: globVerdicts(ctx, "braceVectors") } }),
    },
  },
});
//#endregion 🧭️Adapter
