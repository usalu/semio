// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { scaleBand } from "d3-scale";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "charts-heatmap-matrix";
const DECIMALS = 9;

/** 🧪️ `demo-matrix` of `semio-viz-data`, row-major in the order the long table mentions the levels. */
const ROWS = ["R1", "R2", "R3", "R4"] as const;
const COLUMNS = ["C1", "C2", "C3", "C4"] as const;
const VALUES = [4, 7, 2, 9, 6, 1, 8, 3, 5, 9, 4, 6, 2, 3, 7, 8] as const;

const grid = (value: number): number => {
  const factor = 10 ** DECIMALS;
  return value === 0 ? 0 : Math.round(value * factor) / factor;
};

/** 🎯️ Compiles one committed fixture of this case and returns its whole projection. */
async function probe(ctx: AdapterContext, fixture: string): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbe(ctx.fixture(`shared://🔥️charts-heatmap-matrix/${fixture}`), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
  return { projection: probeProjection(roundProbeNumbers(records, DECIMALS), ctx.scenario.id) };
}
// #endregion 🧫️Vectors

// #region 🔮️Oracles
/** 🔮️ Two unpadded d3-scale band scales, inset by the padding fraction of the cell. */
function rectangles(width: number, height: number, pad: number, padding: number): number[] {
  const columns = scaleBand<string>().domain([...COLUMNS]).range([pad, width - pad]);
  const rows = scaleBand<string>().domain([...ROWS]).range([pad, height - pad]);
  const out: number[] = [];
  for (const row of ROWS) {
    for (const column of COLUMNS) {
      out.push(
        grid(columns(column)! + (columns.bandwidth() * padding) / 2),
        grid(rows(row)! + (rows.bandwidth() * padding) / 2),
        grid(columns.bandwidth() * (1 - padding)),
        grid(rows.bandwidth() * (1 - padding)),
      );
    }
  }
  return out;
}

/** 🔮️ One record per cell: its one-based row and column plus its raw and normalised value. */
function cells(): number[] {
  const min = Math.min(...VALUES);
  const max = Math.max(...VALUES);
  const out: number[] = [];
  ROWS.forEach((_, r) =>
    COLUMNS.forEach((__, c) => {
      const value = VALUES[r * COLUMNS.length + c]!;
      out.push(r + 1, c + 1, value, grid((value - min) / (max - min)));
    }),
  );
  return out;
}
// #endregion 🔮️Oracles

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "heatmap": {
      /** 🔮️ d3-scale's band scales over the frame with no inset. */
      oracle: () => ({ projection: { "geometry/rect": rectangles(64, 40, 4, 0) } }),
      /** 🎯️ The `geometry/rect` records the heatmap drew. */
      subject: async (ctx: AdapterContext) => ({ projection: { "geometry/rect": (await probe(ctx, "heatmap.tex")).projection["geometry/rect"] ?? [] } }),
    },
    "cells": {
      /** 🔮️ The raw and normalised value of every cell of the matrix extent. */
      oracle: () => ({ projection: { "geometry/cell": cells() } }),
      /** 🎯️ The `geometry/cell` records the heatmap emitted. */
      subject: async (ctx: AdapterContext) => ({ projection: { "geometry/cell": (await probe(ctx, "cells.tex")).projection["geometry/cell"] ?? [] } }),
    },
    "heatmap-padded": {
      /** 🔮️ The same band scales with every cell inset by a fifth of its own size. */
      oracle: () => ({ projection: { "geometry/rect": rectangles(64, 40, 4, 0.2) } }),
      /** 🎯️ The `geometry/rect` records the padded heatmap drew. */
      subject: async (ctx: AdapterContext) => ({ projection: { "geometry/rect": (await probe(ctx, "heatmap-padded.tex")).projection["geometry/rect"] ?? [] } }),
    },
  },
});
// #endregion 🧭️Adapter
