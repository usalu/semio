// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import FFT from "fft.js";
import { defineTestAdapter, type AdapterContext } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { compileVizProbe, probeProjection, roundProbeNumbers, type ProbeProjection } from "../../🔨️modules/🧪️viz-probe/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
const CASE = "signal-dft-bode";
const FIXTURE = "local://signal-dft-bode.tex";
const DECIMALS = 5;

/** 🔢️ Rounds an oracle's numbers onto the emission grid the comparison uses. */
function grid(values: readonly number[]): number[] {
  const factor = 10 ** DECIMALS;
  return values.map((value) => Math.round(value * factor) / factor + 0);
}

/** 🌊️ The one-sided amplitude spectrum `fft.js` computes for a real signal, bin index and magnitude interleaved. */
function spectrum(signal: readonly number[]): number[] {
  const size = signal.length;
  const fft = new FFT(size);
  const transformed = fft.createComplexArray();
  fft.realTransform(transformed, [...signal]);
  fft.completeSpectrum(transformed);
  const rows: number[] = [];
  for (let bin = 0; bin <= size / 2; bin += 1) rows.push(bin, (2 / size) * Math.hypot(transformed[2 * bin]!, transformed[2 * bin + 1]!));
  return rows;
}

/** 🌊️ The biased autocorrelation through `fft.js`: the inverse transform of the zero-padded power spectrum. */
function autocorrelation(signal: readonly number[]): number[] {
  const samples = signal.length;
  const fft = new FFT(2 * samples);
  const transformed = fft.createComplexArray();
  fft.realTransform(transformed, [...signal, ...new Array<number>(samples).fill(0)]);
  fft.completeSpectrum(transformed);
  const power = fft.createComplexArray();
  for (let bin = 0; bin < 2 * samples; bin += 1) {
    power[2 * bin] = transformed[2 * bin]! ** 2 + transformed[2 * bin + 1]! ** 2;
    power[2 * bin + 1] = 0;
  }
  const back = fft.createComplexArray();
  fft.inverseTransform(back, power);
  const rows: number[] = [];
  for (let lag = 0; lag <= samples / 2; lag += 1) rows.push(lag, back[2 * lag]! / samples);
  return rows;
}

/** 🌊️ The two-tone signal of the feature's first vector, sampled at its own rate. */
function twoTones(samples: number): number[] {
  return Array.from({ length: samples }, (_, index) => Math.sin((2 * Math.PI * 3 * index) / samples) + 0.5 * Math.sin((2 * Math.PI * 7 * index) / samples));
}

/** 🌊️ The cosine of the feature's third vector, sampled at its own rate. */
function cosine(samples: number, rate: number, frequency: number): number[] {
  return Array.from({ length: samples }, (_, index) => Math.cos((2 * Math.PI * frequency * index) / rate));
}

/** 🎚️ Magnitude and phase of a rational transfer function evaluated at s = i omega. */
function transfer(numerator: readonly number[], denominator: readonly number[], omega: number): [number, number] {
  const evaluate = (coefficients: readonly number[]): [number, number] => {
    let re = 0;
    let im = 0;
    const degree = coefficients.length - 1;
    coefficients.forEach((coefficient, index) => {
      const power = degree - index;
      const magnitude = coefficient * omega ** power;
      if (power % 4 === 0) re += magnitude;
      else if (power % 4 === 1) im += magnitude;
      else if (power % 4 === 2) re -= magnitude;
      else im -= magnitude;
    });
    return [re, im];
  };
  const [nr, ni] = evaluate(numerator);
  const [dr, di] = evaluate(denominator);
  const norm = dr * dr + di * di;
  const re = (nr * dr + ni * di) / norm;
  const im = (ni * dr - nr * di) / norm;
  return [Math.hypot(re, im), (Math.atan2(im, re) * 180) / Math.PI];
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
    "dft-magnitudes": {
      /** 🔮️ `fft.js`'s radix-4 transform of the same three sampled signals. */
      oracle: () => ({
        projection: {
          "dft/two-tones": grid(spectrum(twoTones(32))),
          "dft/constant": grid(spectrum(new Array<number>(16).fill(1))),
          "correlation/cosine": grid(autocorrelation(cosine(16, 16, 2))),
        },
      }),
      subject,
    },
    "transfer-function": {
      /** 🔮️ The closed-form magnitude and phase of the same rational functions. */
      oracle: () => ({
        projection: {
          "bode/second-order": grid([0.1, 0.5, 1, 2, 10].flatMap((omega) => transfer([1], [1, 0.4, 1], omega))),
          "bode/bandpass": grid([1, 4].flatMap((omega) => transfer([1, 0], [1, 3, 2], omega))),
        },
      }),
      subject,
    },
    "polynomial-roots": {
      /** 🔮️ Roots written down in advance, in the ascending order the family sorts them into. */
      oracle: () => ({
        projection: {
          "roots/cubic": grid([1, 0, 2, 0, 3, 0]),
          "roots/quadratic": grid([-0.2, -Math.sqrt(0.96), -0.2, Math.sqrt(0.96)]),
          "roots/real-pair": grid([-2, 0, -1, 0]),
        },
      }),
      subject,
    },
  },
});
// #endregion 🧭️Adapter
