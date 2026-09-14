// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { scaleTemporal } from "../../🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/🟦️.ts";
import base from "../⏳️scale-temporal/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const DECIMALS = 6;

/** 🔢️ Rounds the twin's numbers onto the emission grid the LaTeX probe writes on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => Math.round(value * factor) / factor + 0);
}

/** ⏱️ An ISO timestamp read as UTC, the only reading the library knows. */
function utc(timestamp: string): Date {
  return new Date(`${timestamp}Z`);
}

/** 🔮️ The oracle of `scale-temporal`, reused so both subjects meet the same reference numbers. */
function oracle(id: string): (ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome> {
  const handler = base.scenarios[id]?.oracle;
  if (handler === undefined) throw new Error(`scale-temporal declares no oracle for ${id}`);
  return handler;
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    map: {
      oracle: oracle("map"),
      /** 🎯️ The twin's temporal scale over the same domains, forwards and inverted. */
      subject: () => {
        const days = scaleTemporal([utc("2026-01-01"), utc("2026-12-31")], [0, 364]);
        const hours = scaleTemporal([utc("2026-03-01T00:00"), utc("2026-03-02T00:00")], [0, 240]);
        return {
          projection: {
            "map/tm": grid(["2026-01-01", "2026-04-01", "2026-07-01", "2026-12-31"].map((value) => days(utc(value)))),
            "map/tmhours": grid(["2026-03-01T00:00", "2026-03-01T06:00", "2026-03-01T12:00", "2026-03-02T00:00"].map((value) => hours(utc(value)))),
            "invert/tm": grid([0, 91, 181, 364].map((value) => days.invert!(value))),
          },
        };
      },
    },
  },
});
// #endregion 🧭️Adapter
