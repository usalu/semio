// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "physics-projectile-rk4";
const FIXTURE = "local://physics-projectile-rk4.tex";
const DECIMALS = 5;
const WIDTH = 60;
const HEIGHT = 40;
const MARGIN = 0.08;

/** 🔢️ Rounds an oracle's numbers onto the emission grid the comparison uses. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => Math.round(value * factor) / factor + 0);
}

/** 🚀️ The closed-form parabola, sampled the way the velocity-Verlet step samples it. */
function projectile(speed: number, angle: number, gravity: number, step: number, steps: number): Array<[number, number]> {
  const radians = (angle * Math.PI) / 180;
  const out: Array<[number, number]> = [];
  let x = 0;
  let y = 0;
  let vy = speed * Math.sin(radians);
  const vx = speed * Math.cos(radians);
  for (let index = 0; index <= steps; index += 1) {
    if (y > -1e-9) out.push([x, y]);
    x += step * vx;
    y += step * (vy - (gravity * step) / 2);
    vy -= step * gravity;
  }
  return out;
}

/** 📐️ The canvas mapping the family applies: fit to data with a relative margin, then millimetres. */
function toMillimetres(points: ReadonlyArray<readonly [number, number]>): number[] {
  const xs = points.map(([x]) => x);
  const ys = points.map(([, y]) => y);
  const xmin = Math.min(...xs);
  const xmax = Math.max(...xs);
  const ymin = Math.min(...ys);
  const ymax = Math.max(...ys);
  const dxmin = xmin - MARGIN * (xmax - xmin);
  const dxmax = xmax + MARGIN * (xmax - xmin);
  const dymin = ymin - MARGIN * (ymax - ymin);
  const dymax = ymax + MARGIN * (ymax - ymin);
  return points.flatMap(([x, y]) => [
    (WIDTH * (x - dxmin)) / (dxmax - dxmin),
    (HEIGHT * (y - dymin)) / (dymax - dymin),
  ]);
}

/** 🧮️ Fourth-order Runge–Kutta on a planar autonomous system, the same tableau the kernel uses. */
function rungeKutta(
  fx: (x: number, y: number) => number,
  fy: (x: number, y: number) => number,
  x0: number,
  y0: number,
  step: number,
  steps: number,
): number[] {
  let x = x0;
  let y = y0;
  const out: number[] = [x, y];
  for (let index = 0; index < steps; index += 1) {
    const k1x = fx(x, y);
    const k1y = fy(x, y);
    const k2x = fx(x + (step * k1x) / 2, y + (step * k1y) / 2);
    const k2y = fy(x + (step * k1x) / 2, y + (step * k1y) / 2);
    const k3x = fx(x + (step * k2x) / 2, y + (step * k2y) / 2);
    const k3y = fy(x + (step * k2x) / 2, y + (step * k2y) / 2);
    const k4x = fx(x + step * k3x, y + step * k3y);
    const k4y = fy(x + step * k3x, y + step * k3y);
    x += (step * (k1x + 2 * k2x + 2 * k3x + k4x)) / 6;
    y += (step * (k1y + 2 * k2y + 2 * k3y + k4y)) / 6;
    out.push(x, y);
  }
  return out;
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
    "projectile-path": {
      /** 🔮️ The closed-form flight, mapped onto the canvas by the documented window rule. */
      oracle: () => ({
        projection: {
          "geometry/path": grid(toMillimetres(projectile(22, 52, 9.81, 0.05, 40))),
          "geometry/kinematics/range": grid([projectile(22, 52, 9.81, 0.05, 40).length > 0 ? lastRange(22, 52, 9.81, 0.05, 40) : 0]),
        },
      }),
      subject,
    },
    "harmonic-orbit": {
      /** 🔮️ The same tableau in JavaScript; the analytic circle is cos t, -sin t. */
      oracle: () => ({
        projection: {
          "rk4/orbit": grid(rungeKutta((_x, y) => y, (x) => -x, 1, 0, 0.1, 20)),
          "rk4/orbit-fine": grid(rungeKutta((_x, y) => y, (x) => -x, 1, 0, 0.05, 40)),
        },
      }),
      subject,
    },
    "damped-decay": {
      /** 🔮️ The damped system integrated the same way; energy must never increase. */
      oracle: () => ({
        projection: {
          "rk4/damped": grid(rungeKutta((_x, y) => y, (x, y) => -x - 0.4 * y, 1, 0, 0.1, 30)),
        },
      }),
      subject,
    },
  },
});

/** 🚀️ The horizontal position the integrator has reached when it stops, which the family reports. */
function lastRange(speed: number, angle: number, gravity: number, step: number, steps: number): number {
  const radians = (angle * Math.PI) / 180;
  return step * speed * Math.cos(radians) * (steps + 1);
}
// #endregion 🧭️Adapter
