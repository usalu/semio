// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import semver from "semver";
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
const DIFFERENTIAL = ["exact-and-any", "caret-tiers", "tilde-and-at-least"];

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    ...Object.fromEntries(
      DIFFERENTIAL.map((scenario) => [
        scenario,
        {
          /** 🔮️ The registered `semver` reference implementation. */
          oracle: (ctx: AdapterContext) => ({ projection: pairs(ctx).map((pair) => ({ ...pair, satisfied: semver.satisfies(pair.version, pair.requirement) })) }),
          /** 🎯️ This repository's owned decision. */
          subject: (ctx: AdapterContext) => ({ projection: pairs(ctx).map((pair) => ({ ...pair, satisfied: versionSatisfies(pair.version, pair.requirement) })) }),
        },
      ]),
    ),
  },
});
// #endregion 🧭️Adapter
