// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { scaleUtc } from "d3-scale";
import { utcDay, utcMonth, utcWeek, utcYear, type CountableTimeInterval } from "d3-time";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "scale-temporal";
const FIXTURE = "local://scale-temporal.tex";
const DECIMALS = 6;

/** ⏱️ An ISO timestamp read as UTC, the only reading the library knows. */
function utc(timestamp: string): Date {
  return new Date(`${timestamp}Z`);
}

/** 📅️ The interval boundaries inside a closed domain, as ISO dates. */
function boundaries(interval: CountableTimeInterval, from: string, to: string): string[] {
  return interval.range(utc(from), new Date(utc(to).getTime() + 1)).map((date) => date.toISOString().slice(0, 10));
}

/** 🎯️ Compiles the committed fixture and projects the records of one scenario. */
async function subject(ctx: AdapterContext): Promise<{ projection: ProbeProjection }> {
  const records = await compileVizProbe(ctx.fixture(FIXTURE), { workDir: ctx.workDir, caseName: CASE, scenario: ctx.scenario.id });
  return { projection: probeProjection(roundProbeNumbers(records, DECIMALS), ctx.scenario.id) };
}
// #endregion 🧫️Vectors

// #region 🧭️Adapter
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    map: {
      /** 🔮️ d3-scale's scaleUtc over the same domains, forwards and inverted. */
      oracle: () => {
        const days = scaleUtc().domain([utc("2026-01-01"), utc("2026-12-31")]).range([0, 364]);
        const hours = scaleUtc().domain([utc("2026-03-01T00:00"), utc("2026-03-02T00:00")]).range([0, 240]);
        return {
          projection: {
            "map/tm": ["2026-01-01", "2026-04-01", "2026-07-01", "2026-12-31"].map((value) => days(utc(value))),
            "map/tmhours": ["2026-03-01T00:00", "2026-03-01T06:00", "2026-03-01T12:00", "2026-03-02T00:00"].map((value) => hours(utc(value))),
            "invert/tm": [0, 91, 181, 364].map((value) => days.invert(value).getTime()),
          },
        };
      },
      /** 🎯️ The same temporal scales from the compiled probe. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    "day-ticks": {
      /** 🔮️ d3-time's utcDay boundaries inside the domain. */
      oracle: () => ({ projection: { "ticks/day": boundaries(utcDay, "2026-02-25", "2026-03-04") } }),
      /** 🎯️ The day ticks the probe's temporal scale produced. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    "week-ticks": {
      /** 🔮️ d3-time's utcWeek, which is Sunday based, inside the domain. */
      oracle: () => ({ projection: { "ticks/week": boundaries(utcWeek, "2026-01-01", "2026-03-01") } }),
      /** 🎯️ The week ticks the probe's temporal scale produced. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    "month-ticks": {
      /** 🔮️ d3-time's utcMonth boundaries inside the domain. */
      oracle: () => ({ projection: { "ticks/month": boundaries(utcMonth, "2026-01-01", "2026-12-31") } }),
      /** 🎯️ The month ticks the probe's temporal scale produced. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
    "year-ticks": {
      /** 🔮️ d3-time's utcYear boundaries inside the domain. */
      oracle: () => ({ projection: { "ticks/year": boundaries(utcYear, "2019-06-01", "2024-06-01") } }),
      /** 🎯️ The year ticks the probe's temporal scale produced. */
      subject: async (ctx: AdapterContext) => (await subject(ctx)),
    },
  },
});
// #endregion 🧭️Adapter
