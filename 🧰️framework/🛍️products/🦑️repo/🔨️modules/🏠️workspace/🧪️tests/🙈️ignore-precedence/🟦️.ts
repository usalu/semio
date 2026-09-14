//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//#region 🔌️Adapters
import ignoreFactory from "ignore";
import { defineTestAdapter } from "../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🧭️Adapter
/**
 * 🟦️ The `ignore` package decides the verdict for the git-compatible vector set. It deliberately
 * registers NO handler for the divergence scenario: a gitignore-conformant matcher disagrees there
 * by construction, so that scenario is held by the two subjects instead.
 */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "vectors-are-ignored-the-same-way": {
      oracle: (ctx) => {
        const file = JSON.parse(new TextDecoder().decode(ctx.fixtureBytes("shared://📡️ignore-vectors.json"))) as { vectors: { name: string; rules: string[]; path: string }[] };
        return { projection: { verdicts: file.vectors.map((vector) => `${vector.name}=${ignoreFactory().add(vector.rules).ignores(vector.path)}`) } };
      },
    },
  },
});
//#endregion 🧭️Adapter
