// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { scaleDiverging, scaleSequential, vizInterpolateRgb } from "../../🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/🟦️.ts";
import base from "../🎨️scale-color/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const DECIMALS = 6;
const IDENTITY = (t: number) => t;

/** 🔢️ Rounds the twin's numbers onto the emission grid the LaTeX probe writes on. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => Math.round(value * factor) / factor + 0);
}

/** 🔮️ The oracle of `scale-color`, reused so both subjects meet the same reference numbers. */
function oracle(id: string): (ctx: AdapterContext) => AdapterOutcome | Promise<AdapterOutcome> {
  const handler = base.scenarios[id]?.oracle;
  if (handler === undefined) throw new Error(`scale-color declares no oracle for ${id}`);
  return handler;
}

/** 🌈️ One interpolated colour in the transport encoding the probe uses. */
function mix(from: string, to: string, position: number): string[] {
  return [`|${vizInterpolateRgb(`#${from}`, `#${to}`)(position)}`];
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "sequential-position": {
      oracle: oracle("sequential-position"),
      /** 🎯️ The twin's sequential scale with the identity interpolator isolates the domain arithmetic. */
      subject: () => {
        const plain = scaleSequential([0, 100], IDENTITY);
        const clamped = scaleSequential([0, 100], IDENTITY, { clamp: true });
        const inputs = [-20, 0, 25, 50, 75, 100, 140];
        return { projection: { "position/seq": grid(inputs.map((value) => plain(value))), "position/seqclamp": grid(inputs.map((value) => clamped(value))) } };
      },
    },
    "diverging-position": {
      oracle: oracle("diverging-position"),
      /** 🎯️ The twin's diverging scale with the identity interpolator. */
      subject: () => {
        const centred = scaleDiverging([-10, 0, 30], IDENTITY);
        const offset = scaleDiverging([0, 2, 10], IDENTITY);
        return { projection: { "position/div": grid([-10, -5, 0, 15, 30].map((value) => centred(value))), "position/divoff": grid([0, 1, 2, 6, 10].map((value) => offset(value))) } };
      },
    },
    "interpolate-rgb": {
      oracle: oracle("interpolate-rgb"),
      /** 🎯️ The twin's sRGB interpolator over the same two colour pairs. */
      subject: () => {
        const projection: Record<string, string[]> = {};
        [0, 0.25, 0.5, 0.75, 1].forEach((position, index) => {
          projection[`mix/rgb/${index}`] = mix("ff344f", "34d1bf", position);
        });
        projection["mix/rgb/5"] = mix("000000", "ffffff", 0.5);
        return { projection };
      },
    },
  },
});
// #endregion 🧭️Adapter
