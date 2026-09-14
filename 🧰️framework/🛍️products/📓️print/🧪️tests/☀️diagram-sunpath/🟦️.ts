// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import SunCalc from "suncalc";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbeDocument, probeProjection, roundProbeNumbers, roundProjectionNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
import { DIAGRAM_PACKAGES, DIAGRAM_PREAMBLE, FRAME, rows } from "../🚦️diagram-routing/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "diagram-sunpath";
const DECIMALS = 4;
const RADIANS = Math.PI / 180;

type Sample = { latitude: number; longitude: number; date: string; hour: number };

function samples(ctx: AdapterContext): Sample[] {
  return rows(ctx).map((row) => ({
    latitude: Number(row.latitude),
    longitude: Number(row.longitude),
    date: row.date!,
    hour: Number(row.hour),
  }));
}
// #endregion 🧫️Vectors

// #region 🔮️Oracle
/** 🕰️ The sampled instant as the UTC moment suncalc takes, so both sides read the same clock. */
function instant(sample: Sample): Date {
  const [year, month, day] = sample.date.split("-").map(Number) as [number, number, number];
  return new Date(Date.UTC(year, month - 1, day, Math.trunc(sample.hour), Math.round((sample.hour % 1) * 60)));
}

/** 🔮️ suncalc's altitude and azimuth in degrees, azimuth measured from south as the family measures it. */
function solarPosition(sample: Sample): [number, number] {
  const position = SunCalc.getPosition(instant(sample), sample.latitude, sample.longitude);
  return [position.altitude / RADIANS, position.azimuth / RADIANS];
}

function referencePositions(ctx: AdapterContext): number[] {
  const flat: number[] = [];
  for (const sample of samples(ctx)) flat.push(...solarPosition(sample));
  return flat;
}
// #endregion 🔮️Oracle

// #region 🧪️Probe
/** 🎯️ One `arch-sunpath` run per sampled place and date; the family emits one record per hour. */
async function subject(ctx: AdapterContext): Promise<{ projection: ProbeProjection }> {
  const body = [] as { raw: string }[];
  for (const sample of samples(ctx)) {
    body.push({ raw: FRAME.open });
    body.push({
      raw:
        `\\SemioVizRunFamily{arch-sunpath}[latitude=${sample.latitude},longitude=${sample.longitude},` +
        `date=${sample.date},hours={${sample.hour}},width=60,height=60]`,
    });
    body.push({ raw: FRAME.close });
  }
  const records = await compileVizProbeDocument(
    { case: CASE, scenario: ctx.scenario.id, packages: DIAGRAM_PACKAGES, preamble: DIAGRAM_PREAMBLE, geometry: true, body },
    { workDir: ctx.workDir },
  );
  const projection = probeProjection(roundProbeNumbers(records, DECIMALS));
  return { projection: { "geometry/diagram-sun": projection["geometry/diagram-sun"] ?? [] } };
}
// #endregion 🧪️Probe

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "solstice-day-arc": {
      /** 🔮️ The independent implementation of the same astronomical model. */
      oracle: (ctx: AdapterContext) => ({ projection: roundProjectionNumbers({ "geometry/diagram-sun": referencePositions(ctx) }, DECIMALS) }),
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    "southern-winter-arc": {
      oracle: (ctx: AdapterContext) => ({ projection: roundProjectionNumbers({ "geometry/diagram-sun": referencePositions(ctx) }, DECIMALS) }),
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
  },
});
// #endregion 🧭️Adapter
