// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { clsx } from "clsx";
import { defineTestAdapter, type AdapterContext } from "../../../../../../🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";
import { cn, type ClassNameInput } from "../../🟦️component.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
/** 🧫️ The one input vector of a scenario, read from the feature's doc string — the feature file is
 * the single source of the vectors, so the oracle and the subject are provably given the same input. */
function inputsOf(ctx: AdapterContext): ClassNameInput[] {
  const docString = ctx.scenario.steps.find((step) => step.docString !== undefined)?.docString;
  if (docString === undefined) throw new Error(`scenario ${ctx.scenario.id} carries no input doc string`);
  // 🧭️ `undefined` is a legal ClassNameInput but not legal JSON, so the profile spells it as a bare
  // literal and it is normalized to JSON `null` — both are falsey for every implementation.
  return JSON.parse(docString.replace(/\bundefined\b/g, "null")) as ClassNameInput[];
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: Object.fromEntries(
    ["flattens-nested-arrays-and-objects", "suppresses-every-falsey-value", "preserves-unclassified-application-classes", "flattens-deeply-nested-mixed-input"].map((scenario) => [
      scenario,
      {
        /** 🔮️ The registered `clsx` reference implementation — linked only here, never by production. */
        oracle: (ctx: AdapterContext) => ({ projection: clsx(...(inputsOf(ctx) as Parameters<typeof clsx>)) }),
        /** 🎯️ This repository's owned composition. */
        subject: (ctx: AdapterContext) => ({ projection: cn(...inputsOf(ctx)) }),
      },
    ]),
  ),
});
// #endregion 🧭️Adapter
