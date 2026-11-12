/** 🎯️ Elementary functions that return the same bits on every JavaScript engine and in every language: each is evaluated in double-double arithmetic (about 106 bits, only `+ - * /` and `sqrt` of IEEE 754 binary64) and rounded once, so the result is the correctly rounded double up to hard cases of probability near 1e-15 (near 1e-7 for exp, pow and the logarithms, whose series are cut at 1e-23), whereas `Math.sin` and its siblings are engine-dependent in the last bit (V8 ports fdlibm, JavaScriptCore calls the system libm).
 * Used wherever a computed number reaches a recorded vector, digest or pinned byte string.
 * Arguments beyond 2^30 radians and subnormal results fall back to the engine and are not portable.
 * @see ../../🧫️fixtures/🎯️portable-math/🔣️.json — the language-agnostic vectors
 * @see https://en.wikipedia.org/wiki/Double-double_arithmetic
 */
type DD = readonly [number, number];

const HALF_PI: readonly [number, number, number] = [1.5707963267948966, 6.123233995736766e-17, -1.4973849048591698e-33];
const LN2: readonly [number, number, number] = [0.6931471805599453, 2.3190468138462996e-17, 5.707708438416212e-34];
const LN2_DD: DD = [LN2[0], LN2[1]];
const SPLITTER = 134217729;
const LIMIT = 2 ** 30;

const twoSum = (a: number, b: number): DD => {
  const s = a + b, v = s - a;
  return [s, a - (s - v) + (b - v)];
};
const fastSum = (a: number, b: number): DD => {
  const s = a + b;
  return [s, b - (s - a)];
};
const twoProduct = (a: number, b: number): DD => {
  const p = a * b, ta = SPLITTER * a, tb = SPLITTER * b, ah = ta - (ta - a), bh = tb - (tb - b), al = a - ah, bl = b - bh;
  return [p, ah * bh - p + ah * bl + al * bh + al * bl];
};
const add = (a: DD, b: DD): DD => {
  const [s, e] = twoSum(a[0], b[0]), [t, f] = twoSum(a[1], b[1]), [u, g] = fastSum(s, e + t);
  return fastSum(u, g + f);
};
const plus = (a: DD, b: DD): DD => {
  const [s, e] = twoSum(a[0], b[0]);
  return fastSum(s, e + a[1] + b[1]);
};
const negate = (a: DD): DD => [-a[0], -a[1]];
const subtract = (a: DD, b: DD): DD => add(a, negate(b));
const multiply = (a: DD, b: DD): DD => {
  const [p, e] = twoProduct(a[0], b[0]);
  return fastSum(p, e + a[0] * b[1] + a[1] * b[0]);
};
const scaleBy = (a: DD, s: number): DD => {
  const [p, e] = twoProduct(a[0], s);
  return fastSum(p, e + a[1] * s);
};
const divide = (a: DD, b: DD): DD => {
  const q1 = a[0] / b[0], r1 = subtract(a, scaleBy(b, q1)), q2 = r1[0] / b[0], r2 = subtract(r1, scaleBy(b, q2)), q3 = r2[0] / b[0], [s, e] = fastSum(q1, q2);
  return add([s, e], [q3, 0]);
};
const squareRoot = (a: DD): DD => {
  if (!(a[0] > 0)) return [0, 0];
  const s = Math.sqrt(a[0]), r = subtract(a, twoProduct(s, s));
  return fastSum(s, r[0] / (2 * s));
};

const inverseFactorials: DD[] = [[1, 0]];
for (let n = 1; n <= 30; n += 1) inverseFactorials.push(divide(inverseFactorials[n - 1]!, [n, 0]));

const exponentBits = new DataView(new ArrayBuffer(8));
const powerOfTwo = (k: number): number => {
  exponentBits.setUint32(0, (k + 1023) << 20);
  exponentBits.setUint32(4, 0);
  return exponentBits.getFloat64(0);
};
const exponentOf = (x: number): number => {
  exponentBits.setFloat64(0, x);
  return ((exponentBits.getUint32(0) >>> 20) & 0x7ff) - 1023;
};
const scaleByPower = (x: number, k: number): number => {
  let value = x, rest = k;
  while (rest > 1000) (value *= powerOfTwo(1000), rest -= 1000);
  while (rest < -1000) (value *= powerOfTwo(-1000), rest += 1000);
  return value * powerOfTwo(rest);
};

const tailSine: readonly number[] = inverseFactorialsOdd();
const tailCosine: readonly number[] = inverseFactorialsEven();
function inverseFactorialsOdd(): number[] {
  const values: number[] = [];
  for (let k = 14; k >= 6; k -= 1) values.push((k % 2 === 1 ? -1 : 1) * inverseFactorials[2 * k + 1]![0]);
  return values;
}
function inverseFactorialsEven(): number[] {
  const values: number[] = [];
  for (let k = 14; k >= 7; k -= 1) values.push((k % 2 === 1 ? -1 : 1) * inverseFactorials[2 * k]![0]);
  return values;
}
const horner = (coefficients: readonly number[], q: number): number => {
  let value = 0;
  for (const coefficient of coefficients) value = coefficient + value * q;
  return value;
};
const sineSeries = (r: DD): DD => {
  const square = multiply(r, r);
  let term = r, sum = r;
  for (let k = 1; k <= 5; k += 1) {
    term = multiply(term, square);
    sum = plus(sum, multiply(term, k % 2 === 1 ? negate(inverseFactorials[2 * k + 1]!) : inverseFactorials[2 * k + 1]!));
  }
  const q = square[0];
  return plus(sum, [r[0] * q * q * q * q * q * q * horner(tailSine, q), 0]);
};
const cosineSeries = (r: DD): DD => {
  const square = multiply(r, r);
  let term: DD = [1, 0], sum: DD = [1, 0];
  for (let k = 1; k <= 6; k += 1) {
    term = multiply(term, square);
    sum = plus(sum, multiply(term, k % 2 === 1 ? negate(inverseFactorials[2 * k]!) : inverseFactorials[2 * k]!));
  }
  const q = square[0];
  return plus(sum, [q * q * q * q * q * q * q * horner(tailCosine, q), 0]);
};
const reduceQuadrant = (x: DD): { quadrant: number; remainder: DD } => {
  const n = Math.round(x[0] * (2 / Math.PI));
  if (n === 0) return { quadrant: 0, remainder: x };
  let remainder = x;
  for (const part of HALF_PI) remainder = subtract(remainder, twoProduct(n, part));
  return { quadrant: ((n % 4) + 4) % 4, remainder };
};
const sineOnly = (x: DD): DD => {
  const { quadrant, remainder } = reduceQuadrant(x);
  return quadrant === 0 ? sineSeries(remainder) : quadrant === 1 ? cosineSeries(remainder) : quadrant === 2 ? negate(sineSeries(remainder)) : negate(cosineSeries(remainder));
};
const cosineOnly = (x: DD): DD => {
  const { quadrant, remainder } = reduceQuadrant(x);
  return quadrant === 0 ? cosineSeries(remainder) : quadrant === 1 ? negate(sineSeries(remainder)) : quadrant === 2 ? negate(cosineSeries(remainder)) : sineSeries(remainder);
};
const sineCosine = (x: DD): { sin: DD; cos: DD } => {
  const { quadrant, remainder } = reduceQuadrant(x), s = sineSeries(remainder), c = cosineSeries(remainder);
  return quadrant === 0 ? { sin: s, cos: c } : quadrant === 1 ? { sin: c, cos: negate(s) } : quadrant === 2 ? { sin: negate(s), cos: negate(c) } : { sin: negate(c), cos: s };
};
const portable = (x: number): boolean => Number.isFinite(x) && Math.abs(x) <= LIMIT;

const tailExponential: readonly number[] = (() => {
  const values: number[] = [];
  for (let n = 12; n >= 3; n -= 1) values.push(inverseFactorials[n]![0]);
  return values;
})();
let scratchHigh = 0, scratchLow = 0;
const multiplyScalar = (ah: number, al: number, bh: number, bl: number): void => {
  const p = ah * bh, ta = SPLITTER * ah, a1 = ta - (ta - ah), a2 = ah - a1, tb = SPLITTER * bh, b1 = tb - (tb - bh), b2 = bh - b1;
  const e = a1 * b1 - p + a1 * b2 + a2 * b1 + a2 * b2 + (ah * bl + al * bh), s = p + e;
  scratchHigh = s;
  scratchLow = e - (s - p);
};
const plusScalar = (ah: number, al: number, bh: number, bl: number): void => {
  const s = ah + bh, v = s - ah, e = ah - (s - v) + (bh - v) + al + bl, t = s + e;
  scratchHigh = t;
  scratchLow = e - (t - s);
};
const exponential = (x: DD): DD => {
  const k = Math.round(x[0] * (1 / LN2[0]));
  let r = x;
  for (const part of LN2) r = subtract(r, twoProduct(k, part));
  const sh = r[0] / 256, sl = r[1] / 256;
  let th = sh, tl = sl, uh = sh, ul = sl;
  for (let n = 2; n <= 2; n += 1) {
    multiplyScalar(th, tl, sh, sl);
    th = scratchHigh;
    tl = scratchLow;
    const factor = inverseFactorials[n]!;
    multiplyScalar(th, tl, factor[0], factor[1]);
    plusScalar(uh, ul, scratchHigh, scratchLow);
    uh = scratchHigh;
    ul = scratchLow;
  }
  const square = sh * sh;
  plusScalar(uh, ul, square * sh * horner(tailExponential, sh), 0);
  uh = scratchHigh;
  ul = scratchLow;
  for (let i = 0; i < 8; i += 1) {
    multiplyScalar(uh, ul, uh, ul);
    const mh = scratchHigh, ml = scratchLow;
    plusScalar(2 * uh, 2 * ul, mh, ml);
    uh = scratchHigh;
    ul = scratchLow;
  }
  const [hi, lo] = add([1, 0], [uh, ul]);
  return [scaleByPower(hi, k), scaleByPower(lo, k)];
};
const logarithm = (x: number): DD => {
  const guess = Math.log(x), t = subtract(multiply([x, 0], exponential([-guess, 0])), [1, 0]);
  return add([guess, 0], subtract(t, scaleBy(multiply(t, t), 0.5)));
};

const angle = (y: DD, x: DD): number => {
  const theta = Math.atan2(y[0], x[0]), { sin, cos } = sineCosine([theta, 0]);
  const h = squareRoot(add(multiply(x, x), multiply(y, y)));
  return add([theta, 0], negate(divide(subtract(multiply(sin, x), multiply(cos, y)), h)))[0];
};
const exact = (x: number, y: number): boolean => Number.isFinite(x) && Number.isFinite(y) && x !== 0 && y !== 0;
const unit = (a: number, b: number): [DD, DD] => {
  const k = -exponentOf(Math.max(Math.abs(a), Math.abs(b)));
  return [[scaleByPower(a, k), 0], [scaleByPower(b, k), 0]];
};

/** 📈️ Sine of `x` radians. */
export const sin = (x: number): number => (!portable(x) ? Math.sin(x) : Math.abs(x) < 2 ** -27 ? x : sineOnly([x, 0])[0]);
/** 📉️ Cosine of `x` radians. */
export const cos = (x: number): number => (!portable(x) ? Math.cos(x) : Math.abs(x) < 2 ** -27 ? 1 : cosineOnly([x, 0])[0]);
/** 📐️ Tangent of `x` radians. */
export const tan = (x: number): number => {
  if (!portable(x)) return Math.tan(x);
  if (Math.abs(x) < 2 ** -27) return x;
  const { sin: s, cos: c } = sineCosine([x, 0]);
  return divide(s, c)[0];
};
/** 🧭️ The angle of the point `(x, y)` from the positive x axis, in (-π, π]. */
export const atan2 = (y: number, x: number): number => {
  if (!exact(x, y)) return Math.atan2(y, x);
  const [uy, ux] = unit(y, x);
  return angle(uy, ux);
};
/** 🔝️ Arctangent of `x`, in (-π/2, π/2). */
export const atan = (x: number): number => atan2(x, 1);
/** 🔼️ Arcsine of `x`, in [-π/2, π/2]; NaN outside [-1, 1]. */
export const asin = (x: number): number => {
  if (!(Math.abs(x) < 1) || x === 0) return Math.abs(x) === 1 ? Math.sign(x) * HALF_PI[0] : Number.isNaN(x) || Math.abs(x) > 1 ? NaN : x;
  const c = squareRoot(multiply(twoSum(1, -x), twoSum(1, x)));
  return angle([x, 0], c);
};
/** 🔽️ Arccosine of `x`, in [0, π]; NaN outside [-1, 1]. */
export const acos = (x: number): number => {
  if (!(Math.abs(x) < 1)) return x === 1 ? 0 : x === -1 ? Math.PI : NaN;
  const c = squareRoot(multiply(twoSum(1, -x), twoSum(1, x)));
  return angle(c, [x, 0]);
};
const LN10: DD = logarithm(10);
/** 🪵️ Natural logarithm of `x`. */
export const log = (x: number): number => (!(x > 0) || !Number.isFinite(x) || x === 1 || x < 2 ** -1000 ? Math.log(x) : logarithm(x)[0]);
/** 🪵️ Base-2 logarithm of `x`; exact for powers of two. */
export const log2 = (x: number): number => (!(x > 0) || !Number.isFinite(x) || x < 2 ** -1000 ? Math.log2(x) : divide(logarithm(x), LN2_DD)[0]);
/** 🪵️ Base-10 logarithm of `x`; exact for powers of ten. */
export const log10 = (x: number): number => (!(x > 0) || !Number.isFinite(x) || x < 2 ** -1000 ? Math.log10(x) : divide(logarithm(x), LN10)[0]);
/** 🧊️ Cube root of `x`. */
export const cbrt = (x: number): number => {
  if (!Number.isFinite(x) || x === 0) return Math.cbrt(x);
  const y = Math.cbrt(x), cube = multiply(multiply([y, 0], [y, 0]), [y, 0]);
  return add([y, 0], negate(divide(subtract(cube, [x, 0]), scaleBy(multiply([y, 0], [y, 0]), 3))))[0];
};
/** 🚀️ `e` to the power `x`. */
export const exp = (x: number): number => (!Number.isFinite(x) || x > 709 || x < -708 ? Math.exp(x) : x === 0 ? 1 : exponential([x, 0])[0]);
/** ⚡️ `x` to the power `y`. */
export const pow = (x: number, y: number): number => {
  if (!exact(x, y) || y === 1 || x === 1) return Math.pow(x, y);
  const negative = x < 0;
  if (negative && !Number.isInteger(y)) return NaN;
  if (Number.isInteger(y) && Math.abs(y) <= 64) {
    let base: DD = [Math.abs(x), 0], result: DD = [1, 0], n = Math.abs(y);
    while (n > 0) {
      if (n % 2 === 1) result = multiply(result, base);
      n = Math.floor(n / 2);
      if (n > 0) base = multiply(base, base);
    }
    const value = y < 0 ? divide([1, 0], result)[0] : result[0];
    return negative && Math.abs(y % 2) === 1 ? -value : value;
  }
  if (y === 0.5 || y === -0.5) return y > 0 ? squareRoot([x, 0])[0] : divide([1, 0], squareRoot([x, 0]))[0];
  const log = logarithm(Math.abs(x)), product = scaleBy(log, y);
  if (!(Math.abs(product[0]) < 700)) return Math.pow(x, y);
  const magnitude = exponential(product)[0];
  return negative && Math.abs(y % 2) === 1 ? -magnitude : magnitude;
};
/** 📏️ Euclidean length of the values, rounded once from the exact sum of squares. */
export const hypot = (...inputs: readonly number[]): number => {
  const values = inputs.map(Number);
  if (values.some((value) => value === Infinity || value === -Infinity)) return Infinity;
  if (values.some(Number.isNaN)) return NaN;
  const largest = Math.max(0, ...values.map(Math.abs));
  if (largest === 0) return 0;
  const k = -exponentOf(largest);
  let sum: DD = [0, 0];
  for (const value of values) {
    const scaled = scaleByPower(value, k);
    sum = add(sum, twoProduct(scaled, scaled));
  }
  return scaleByPower(squareRoot(sum)[0], -k);
};

const FAST_LN2_HIGH = 6.93147180369123816490e-01, FAST_LN2_LOW = 1.90821492927058770002e-10, LOG2E = 1.4426950408889634;
const fastExpSeries: readonly number[] = (() => {
  const values: number[] = [];
  for (let n = 14; n >= 0; n -= 1) values.push(inverseFactorials[n]![0]);
  return values;
})();
const fastLogSeries: readonly number[] = (() => {
  const values: number[] = [];
  for (let n = 12; n >= 0; n -= 1) values.push(1 / (2 * n + 1));
  return values;
})();
/** 🏎️ `e` to the power `x` in plain doubles (Taylor on the reduced argument, no double-double): identical bits on every engine and language like the rest of this module, accurate to about two ulp instead of correctly rounded, and an order of magnitude faster; for numerical models whose pinned outputs are rounded, not for byte-exact encoders. */
export const fastExp = (x: number): number => {
  if (!Number.isFinite(x) || x > 709 || x < -708) return exp(x);
  const k = Math.round(x * LOG2E), r = x - k * FAST_LN2_HIGH - k * FAST_LN2_LOW;
  return scaleByPower(horner(fastExpSeries, r), k);
};
/** 🏎️ Natural logarithm of a positive finite `x` in plain doubles; the tier of {@link fastExp}. */
export const fastLog = (x: number): number => {
  if (!(x > 0) || !Number.isFinite(x) || x < 2 ** -1000) return log(x);
  let e = exponentOf(x), m = scaleByPower(x, -e);
  if (m > Math.SQRT2) (m /= 2, e += 1);
  const s = (m - 1) / (m + 1);
  return e * FAST_LN2_HIGH + (e * FAST_LN2_LOW + 2 * s * horner(fastLogSeries, s * s));
};
/** 🏎️ `x` to the power `y` for positive finite `x` in plain doubles; integer exponents up to 64 by repeated multiplication, the rest as `exp(y log x)`; the tier of {@link fastExp}. */
export const fastPow = (x: number, y: number): number => {
  if (!(x > 0) || !Number.isFinite(x) || !Number.isFinite(y)) return pow(x, y);
  if (Number.isInteger(y) && Math.abs(y) <= 64) {
    let result = 1, base = x, n = Math.abs(y);
    while (n > 0) {
      if (n % 2 === 1) result *= base;
      n = Math.floor(n / 2);
      base *= base;
    }
    return y < 0 ? 1 / result : result;
  }
  if (y === 0.5) return Math.sqrt(x);
  const product = y * fastLog(x);
  return Math.abs(product) < 700 ? fastExp(product) : pow(x, y);
};
