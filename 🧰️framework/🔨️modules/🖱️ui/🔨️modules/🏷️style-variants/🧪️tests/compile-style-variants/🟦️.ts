// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { cva } from "class-variance-authority";
import { defineTestAdapter, type AdapterContext } from "../../../../../../🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts";
import { styleVariants } from "../../🟦️component.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
type Program = { base: string; config: Record<string, unknown> | null; selections: (Record<string, unknown> | null)[] };

/** 🧫️ The scenario's program, read from the feature — one source of truth for both producers. */
function programOf(ctx: AdapterContext): Program {
  const docString = ctx.scenario.steps.find((step) => step.docString !== undefined)?.docString;
  if (docString === undefined) throw new Error(`scenario ${ctx.scenario.id} carries no program doc string`);
  return JSON.parse(docString) as Program;
}

/** 🧵️ Compiles every selection with one compiler, so the two producers are exercised identically. */
function compileAll(program: Program, compile: (selection?: Record<string, unknown>) => string): string[] {
  return program.selections.map((selection) => (selection === null ? compile() : compile(selection)));
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
const SCENARIOS = ["base-only-and-caller-classes", "single-variant-matrix", "boolean-choices-and-compound-conjunctions"];

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: Object.fromEntries(
    SCENARIOS.map((scenario) => [
      scenario,
      {
        /** 🔮️ The registered `class-variance-authority` reference implementation. */
        oracle: (ctx: AdapterContext) => {
          const program = programOf(ctx);
          const compile = cva(program.base, program.config as never);
          return { projection: compileAll(program, compile as never) };
        },
        /** 🎯️ This repository's owned compiler. */
        subject: (ctx: AdapterContext) => {
          const program = programOf(ctx);
          const compile = styleVariants(program.base, program.config as never);
          return { projection: compileAll(program, compile as never) };
        },
      },
    ]),
  ),
});
