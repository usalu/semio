// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext } from "../../../../../../🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";
import { cn } from "../../🟦️component.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
/** 🧫️ Reads the scenario's `| input | expected |` specification table. The table IS the
 * specification — there is no third-party merger that knows this repository's utility families, so
 * the vectors carry the contract instead of an oracle. */
function vectorsOf(ctx: AdapterContext): { input: string; expected: string }[] {
  const table = ctx.scenario.steps.find((step) => step.dataTable !== undefined)?.dataTable;
  if (table === undefined || table.length < 2) throw new Error(`scenario ${ctx.scenario.id} carries no specification table`);
  const [header, ...rows] = table;
  const inputColumn = header!.indexOf("input");
  const expectedColumn = header!.indexOf("expected");
  if (inputColumn === -1 || expectedColumn === -1) throw new Error(`scenario ${ctx.scenario.id} table needs an "input" and an "expected" column`);
  return rows.map((row) => ({ input: row[inputColumn] ?? "", expected: row[expectedColumn] ?? "" }));
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
/** 🎯️ Composes every vector and fails loudly on the first divergence from the specification. */
function conform(ctx: AdapterContext): { projection: unknown } {
  const results = vectorsOf(ctx).map((vector) => ({ input: vector.input, expected: vector.expected, actual: cn(vector.input) }));
  const divergent = results.filter((row) => row.actual !== row.expected);
  if (divergent.length > 0) {
    throw Object.assign(new Error(divergent.map((row) => `cn(${JSON.stringify(row.input)}) = ${JSON.stringify(row.actual)}, specified ${JSON.stringify(row.expected)}`).join("; ")), { name: "AssertionError" });
  }
  return { projection: results.map((row) => row.actual) };
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: Object.fromEntries(["last-surface-fill-wins", "last-utility-in-a-family-wins", "modifiers-scope-the-conflict"].map((scenario) => [scenario, { subject: conform }])),
});
// #endregion 🧭️Adapter
