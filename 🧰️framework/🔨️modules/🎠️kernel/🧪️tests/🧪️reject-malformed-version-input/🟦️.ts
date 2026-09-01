// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext } from "../../../../🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";
import { versionSatisfies } from "../../🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
/** 🧫️ The scenario's `| version | requirement |` table — the feature owns the vectors. */
function pairs(ctx: AdapterContext): { version: string; requirement: string }[] {
  const table = ctx.scenario.steps.find((step) => step.dataTable !== undefined)?.dataTable;
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no vector table`);
  const [header, ...rows] = table;
  const versionColumn = header!.indexOf("version");
  const requirementColumn = header!.indexOf("requirement");
  return rows.map((row) => ({ version: row[versionColumn] ?? "", requirement: row[requirementColumn] ?? "" }));
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    /** 🚫️ Outside the frozen grammar the contract is "unsatisfied, never a throw" — this
     * repository's own error law, which no third-party matcher can adjudicate. */
    "malformed-input-is-unsatisfied-never-a-throw": {
      subject: (ctx: AdapterContext) => {
        const results = pairs(ctx).map((pair) => {
          try {
            return { ...pair, satisfied: versionSatisfies(pair.version, pair.requirement), threw: false };
          } catch (error) {
            return { ...pair, satisfied: false, threw: true, message: String(error) };
          }
        });
        const offenders = results.filter((row) => row.threw || row.satisfied);
        if (offenders.length > 0) {
          throw Object.assign(new Error(offenders.map((row) => `versionSatisfies(${JSON.stringify(row.version)}, ${JSON.stringify(row.requirement)}) ${row.threw ? "threw" : "reported satisfied"}`).join("; ")), { name: "AssertionError" });
        }
        return { projection: results.map((row) => row.satisfied) };
      },
    },
  },
});
// #endregion 🧭️Adapter
