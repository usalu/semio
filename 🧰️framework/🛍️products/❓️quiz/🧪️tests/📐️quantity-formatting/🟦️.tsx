/** 📐️ Quantity formatting and parsing: the shared vectors, the SI prefix choice and locale digits against
 * `Intl.NumberFormat`'s own engineering notation as the third-party oracle, and typed guesses read back from what
 * `Intl.NumberFormat` writes (numbers, and lengths in its unit style with centi, milli and kilo); the time left on a
 * clock against `Intl.DurationFormat`'s digital style and in words against its long style; the factor a bound names
 * checked by its bounds — never above the factor, less than one step of its last digit below it, two digits at most; and
 * the counts a hint names in words, their scale words against `Intl.NumberFormat`'s long compact notation.
 *
 * @see ../../🧫️fixtures/📐️quantity-formatting/🔣️.json
 */

import { describe, expect, it } from "vitest";
import type { Scale } from "@semio-tech/quiz";
import { QUIZ_LOCALES, SIGNIFICANT_DIGITS, SI_PREFIXES, ceilSignificant, engineering, floorSignificant, formatCount, formatCountdown, formatDuration, formatFactor, formatNumber, formatPoints, formatQuantity, formatScore, formatTimes, parseQuantity, withMinusSign, withUnit, type QuizLocale, type Toward } from "@semio-tech/quiz-react";
import vectors from "../../🧫️fixtures/📐️quantity-formatting/🔣️.json";

interface Vector {
  readonly id: string;
  readonly value: number;
  readonly unit: string;
  readonly prefixed: boolean;
  readonly en: string;
  readonly de: string;
}

interface Parse {
  readonly id: string;
  readonly locale: string;
  readonly text: string;
  readonly unit: string;
  readonly prefixed: boolean;
  readonly scale: string;
  readonly value: number | null;
}

const fixture: {
  readonly parses: readonly Parse[];
  readonly vectors: readonly Vector[];
  readonly scores: readonly { readonly id: string; readonly score: number; readonly en: string; readonly de: string }[];
  readonly points: readonly { readonly id: string; readonly points: number; readonly en: string; readonly de: string }[];
} = vectors;

function oracle(value: number, unit: string, locale: QuizLocale): string {
  const parts = new Intl.NumberFormat(locale, { notation: "engineering", maximumSignificantDigits: SIGNIFICANT_DIGITS }).formatToParts(value);
  const separator = parts.findIndex((part) => part.type === "exponentSeparator");
  const mantissa = parts
    .slice(0, separator)
    .map((part) => part.value)
    .join("");
  const exponent = Number(
    parts
      .slice(separator + 1)
      .map((part) => (part.type === "exponentMinusSign" ? "-" : part.value))
      .join(""),
  );
  return `${mantissa}\u00a0${SI_PREFIXES[exponent]}${unit}`;
}

/** 🎯️ `value` rounded to {@link SIGNIFICANT_DIGITS} significant digits by `Intl.NumberFormat`'s scientific notation. */
function rounded(value: number): number {
  return Number(new Intl.NumberFormat("en", { notation: "scientific", maximumSignificantDigits: SIGNIFICANT_DIGITS }).format(value).replace("E", "e"));
}

const MANTISSAS = [1, 1.5, 2.345678, 4.2, 9.99949, 9.9996, 12.345, 42, 123.45, 500, 999.94, 999.96, 1234.5];

describe("📐️ quantity formatting", () => {
  for (const vector of fixture.parses) {
    it(`parses the shared vector ${vector.id}`, () => {
      expect(parseQuantity(vector.text, { ...vector, scale: vector.scale as Scale }, vector.locale as QuizLocale)).toBe(vector.value ?? undefined);
    });
  }

  it("reads back every number Intl.NumberFormat writes, grouped or not, in both locales", () => {
    const quantity = { unit: "W", prefixed: true, scale: "linear" } as const;
    let compared = 0;
    for (const locale of QUIZ_LOCALES) {
      const intl = new Intl.NumberFormat(locale, { maximumFractionDigits: 6 });
      for (const value of [0, 1, 7, 35, 100, 999, 1000, 1234, 12345.5, 100000, 1234567, 0.5, 0.035, 21.25, 123456.789, 99999999, -1, -1234.5]) {
        expect(parseQuantity(intl.format(value), quantity, locale), `${intl.format(value)} in ${locale}`).toBe(value);
        expect(parseQuantity(`${intl.format(value)} W`, quantity, locale), `${intl.format(value)} W in ${locale}`).toBe(value);
        compared += 2;
      }
    }
    expect(compared).toBe(2 * 2 * 18);
  });

  it("reads back every prefixed quantity written by formatQuantity within its four significant digits, in both locales", () => {
    let compared = 0;
    for (const locale of QUIZ_LOCALES) {
      for (let power = -27; power <= 26; power += 1) {
        for (const mantissa of MANTISSAS) {
          const value = mantissa * 10 ** power;
          const written = formatQuantity(value, { unit: "Wh", prefixed: true }, locale);
          expect(parseQuantity(written, { unit: "Wh", prefixed: true, scale: "logarithmic" }, locale), `${written} in ${locale}`).toBe(rounded(value));
          compared += 1;
        }
      }
    }
    expect(compared).toBe(2 * 54 * MANTISSAS.length);
  });

  it("agrees with Number() on plain scientific notation", () => {
    for (const text of ["1e3", "2.5E-4", "6.02e23", "1.5E+2", "0.001", "42"]) expect(parseQuantity(text, { unit: "W", prefixed: false, scale: "linear" }, "en"), text).toBe(Number(text));
  });

  for (const vector of fixture.vectors) {
    it(`formats the shared vector ${vector.id}`, () => {
      for (const locale of QUIZ_LOCALES) expect(formatQuantity(vector.value, vector, locale)).toBe(vector[locale]);
    });
  }

  for (const vector of fixture.scores) {
    it(`formats the shared score ${vector.id}`, () => {
      for (const locale of QUIZ_LOCALES) expect(formatScore(vector.score, locale)).toBe(vector[locale]);
    });
  }

  for (const vector of fixture.points) {
    it(`formats the shared points ${vector.id}`, () => {
      for (const locale of QUIZ_LOCALES) expect(formatPoints(vector.points, locale)).toBe(vector[locale]);
    });
  }

  it("never shows a score short of perfect as 100 % nor a score above zero as 0 %", () => {
    for (const locale of QUIZ_LOCALES) {
      const perfect = formatScore(1, locale);
      const nothing = formatScore(0, locale);
      for (let step = 1; step < 10_000; step += 1) {
        const shown = formatScore(step / 10_000, locale);
        expect(shown, `${step / 10_000}`).not.toBe(perfect);
        expect(shown, `${step / 10_000}`).not.toBe(nothing);
      }
    }
  });

  it("chooses the SI prefix and mantissa digits exactly as Intl's engineering notation does", () => {
    let compared = 0;
    for (const locale of QUIZ_LOCALES) {
      for (let power = -27; power <= 26; power += 1) {
        for (const mantissa of MANTISSAS) {
          for (const sign of [1, -1]) {
            const value = sign * mantissa * 10 ** power;
            expect(formatQuantity(value, { unit: "W", prefixed: true }, locale), `${value} in ${locale}`).toBe(oracle(value, "W", locale));
            compared += 1;
          }
        }
      }
    }
    expect(compared).toBe(2 * 54 * MANTISSAS.length * 2);
  });

  it("keeps every prefixed mantissa in [1, 1000)", () => {
    for (let power = -27; power <= 26; power += 1) {
      for (const mantissa of MANTISSAS) {
        const { mantissa: scaled } = engineering(mantissa * 10 ** power);
        expect(Math.abs(scaled)).toBeGreaterThanOrEqual(1);
        expect(Math.abs(scaled)).toBeLessThan(1000);
      }
    }
  });

  it("formats unprefixed values, scores and points like Intl.NumberFormat in the same locale", () => {
    for (const locale of QUIZ_LOCALES) {
      for (const value of [0, 0.001, 0.035, 1, 21.5, 2400, 123456.789, 99999999]) expect(formatNumber(value, locale)).toBe(new Intl.NumberFormat(locale, { maximumFractionDigits: 6 }).format(value));
      for (const value of [1e-9, 1.5e18]) expect(formatNumber(value, locale)).toBe(new Intl.NumberFormat(locale, { notation: "scientific", maximumSignificantDigits: SIGNIFICANT_DIGITS }).format(value));
      for (const score of [0, 0.125, 0.5, 0.875, 0.997, 0.4321, 1]) expect(formatScore(score, locale)).toBe(new Intl.NumberFormat(locale, { style: "percent", maximumFractionDigits: 1 }).format(score));
      expect(formatPoints(187.5, locale)).toBe(new Intl.NumberFormat(locale, { maximumFractionDigits: 1 }).format(187.5));
    }
  });

  it("separates value and unit with a no-break space except before degree and arc signs", () => {
    expect(withUnit("3", "m")).toBe("3\u00a0m");
    expect(withUnit("45", "°")).toBe("45°");
    expect(withUnit("12", "′")).toBe("12′");
    expect(withUnit("7", "")).toBe("7");
  });

  it("writes the time left on a clock as m:ss in whole seconds rounded up, like Intl.DurationFormat's digital style", () => {
    const digital = (locale: QuizLocale) => new (Intl as unknown as { readonly DurationFormat: new (locale: string, options: object) => { format(duration: object): string } }).DurationFormat(locale, { style: "digital", hoursDisplay: "auto" });
    for (const locale of QUIZ_LOCALES)
      for (const left of [0, 1, 999, 1000, 1001, 9_001, 29_500, 30_000, 59_999, 60_000, 90_000, 102_000, 599_001, 754_000]) {
        const seconds = Math.ceil(left / 1000);
        const oracle = digital(locale).format({ minutes: Math.floor(seconds / 60), seconds: seconds % 60 }).replace(/^0(?=\d:)/u, "");
        expect(formatCountdown(left), `${left} ms`).toBe(oracle);
      }
    expect(formatCountdown(-5_000)).toBe("0:00");
  });

  it("speaks the time left in words with the locale's plural forms, like Intl.DurationFormat's long style", () => {
    const long = (locale: QuizLocale) => new (Intl as unknown as { readonly DurationFormat: new (locale: string, options: object) => { format(duration: object): string } }).DurationFormat(locale, { style: "long" });
    for (const locale of QUIZ_LOCALES)
      for (const left of [1, 999, 1000, 1001, 5_000, 48_000, 59_999, 60_000, 61_000, 120_000, 125_000, 754_000]) {
        const seconds = Math.ceil(left / 1000);
        expect(formatDuration(left, locale), `${left} ms in ${locale}`).toBe(long(locale).format({ minutes: Math.floor(seconds / 60), seconds: seconds % 60 }));
      }
    expect([formatDuration(0, "en"), formatDuration(-5_000, "de"), formatDuration(125_000, "de")]).toEqual(["0 seconds", "0 Sekunden", "2 Minuten, 5 Sekunden"]);
  });

  it("names a factor bound cut down, never up, to two significant digits in the locale's digits", () => {
    const step = (shown: number): number => 10 ** (Math.floor(Math.log10(shown) + 1e-9) - 1);
    let compared = 0;
    for (const locale of QUIZ_LOCALES) {
      const group = new Intl.NumberFormat(locale).formatToParts(1000).find((part) => part.type === "group")?.value ?? "";
      const decimal = new Intl.NumberFormat(locale).formatToParts(1.5).find((part) => part.type === "decimal")?.value ?? ".";
      for (let index = 0; index < 219; index += 1) {
        const factor = Math.min(1000, 10 ** ((index * 3) / 218));
        const written = formatFactor(factor, locale);
        const shown = Number(written.replaceAll(group, "").replace(decimal, "."));
        expect(shown, `${factor} in ${locale}`).toBeLessThanOrEqual(factor);
        expect(shown + step(shown), `${factor} in ${locale}`).toBeGreaterThan(factor);
        expect(written.replace(/\D/gu, "").replace(/^0+/u, "").replace(/0+$/u, "").length, written).toBeLessThanOrEqual(2);
        compared += 1;
      }
    }
    expect(compared).toBe(2 * 219);
    expect([1.96, 1.04, 2.3, 9.99, 160.3, 1000].map((factor) => formatFactor(factor, "en"))).toEqual(["1.9", "1", "2.3", "9.9", "160", "1,000"]);
    expect([1.96, 1000].map((factor) => formatFactor(factor, "de"))).toEqual(["1,9", "1.000"]);
  });

  it("writes a count in words: plain below a million, with a scale word that agrees with its number below 10¹⁵, as a two-digit mantissa times a power of ten beyond — each cut toward its side", () => {
    const times10 = " × 10";
    const NBSP = " ";
    const cases: readonly (readonly [count: number, toward: Toward, en: string, de: string])[] = [
      [1234.5, "down", "1,200", "1.200"],
      [1234.5, "up", "1,300", "1.300"],
      [999_999, "down", "990,000", "990.000"],
      [999_999, "up", `1${NBSP}million`, `1${NBSP}Million`],
      [1e6, "down", `1${NBSP}million`, `1${NBSP}Million`],
      [1.5e6, "down", `1.5${NBSP}million`, `1,5${NBSP}Millionen`],
      [2e6, "down", `2${NBSP}million`, `2${NBSP}Millionen`],
      [12_345_678, "down", `12${NBSP}million`, `12${NBSP}Millionen`],
      [1e9, "down", `1${NBSP}billion`, `1${NBSP}Milliarde`],
      [2 ** 32, "up", `4.3${NBSP}billion`, `4,3${NBSP}Milliarden`],
      [1e12, "down", `1${NBSP}trillion`, `1${NBSP}Billion`],
      [4.97e12, "down", `4.9${NBSP}trillion`, `4,9${NBSP}Billionen`],
      [9.99e14, "down", `990${NBSP}trillion`, `990${NBSP}Billionen`],
      [9.99e14, "up", `1${times10}¹⁵`, `1${times10}¹⁵`],
      [1e15, "down", `1${times10}¹⁵`, `1${times10}¹⁵`],
      [4.97e15, "down", `4.9${times10}¹⁵`, `4,9${times10}¹⁵`],
      [4.97e15, "up", `5${times10}¹⁵`, `5${times10}¹⁵`],
      [9.96e15, "up", `1${times10}¹⁶`, `1${times10}¹⁶`],
      [1.06e16, "up", `1.1${times10}¹⁶`, `1,1${times10}¹⁶`],
      [1.09e25, "down", `1${times10}²⁵`, `1${times10}²⁵`],
      [1.09e25, "up", `1.1${times10}²⁵`, `1,1${times10}²⁵`],
      [1.1e26, "down", `1.1${times10}²⁶`, `1,1${times10}²⁶`],
      [1.1e26, "up", `1.1${times10}²⁶`, `1,1${times10}²⁶`],
      [1e100, "up", `1${times10}¹⁰⁰`, `1${times10}¹⁰⁰`],
    ];
    for (const [count, toward, en, de] of cases) expect([formatCount(count, "en", toward), formatCount(count, "de", toward)], `${count} ${toward}`).toEqual([en, de]);
    const times: readonly (readonly [count: number, en: string, de: string])[] = [
      [2.96, "2.9 times", "2,9-mal"],
      [1000, "1,000 times", "1.000-mal"],
      [1e6, `1${NBSP}million times`, `1${NBSP}Million Mal`],
      [1.2e6, `1.2${NBSP}million times`, `1,2${NBSP}Millionen Mal`],
      [1.1e26, `1.1${times10}²⁶ times`, `1,1${times10}²⁶-mal`],
    ];
    for (const [count, en, de] of times) expect([formatTimes(count, "en", "down"), formatTimes(count, "de", "down")], String(count)).toEqual([en, de]);
  });

  it("writes a count from 10¹⁵ with the mantissa and exponent of Intl.NumberFormat's scientific notation at two significant digits, never a power of ten alone", () => {
    const superscript = (digits: string): string => [...digits].map((digit) => "⁰¹²³⁴⁵⁶⁷⁸⁹"[Number(digit)]).join("");
    let compared = 0;
    for (const locale of QUIZ_LOCALES) {
      const scientific = new Intl.NumberFormat(locale, { notation: "scientific", maximumSignificantDigits: 2 });
      for (let index = 0; index <= 450; index += 1) {
        const count = 10 ** (15 + (index * 15) / 450);
        for (const toward of ["down", "up"] as const) {
          const shown = (toward === "down" ? floorSignificant : ceilSignificant)(count, 2);
          const [mantissa = "", exponent = ""] = scientific.format(shown).split("E");
          const written = formatCount(count, locale, toward);
          expect(written, `${count} ${toward} in ${locale}`).toBe(`${mantissa} × 10${superscript(exponent)}`);
          expect(written, written).toMatch(/^\d(?:[.,]\d)? × 10[¹²³⁴⁵⁶⁷⁸⁹][⁰¹²³⁴⁵⁶⁷⁸⁹]*$/u);
          compared += 1;
        }
      }
    }
    expect(compared).toBe(2 * 2 * 451);
  });

  it("names a count between a million and 10¹⁵ as Intl.NumberFormat's long compact notation does, singular and plural alike", () => {
    let compared = 0;
    for (const locale of QUIZ_LOCALES) {
      const compact = new Intl.NumberFormat(locale, { notation: "compact", compactDisplay: "long", maximumSignificantDigits: 2 });
      for (let index = 0; index <= 360; index += 1) {
        const count = Math.min(9.9e14, 10 ** (6 + (index * 9) / 360));
        for (const toward of ["down", "up"] as const) {
          const shown = (toward === "down" ? floorSignificant : ceilSignificant)(count, 2);
          if (shown >= 1e15) continue;
          expect(formatCount(count, locale, toward), `${count} ${toward} in ${locale}`).toBe(compact.format(shown).replace(/\s/gu, " "));
          compared += 1;
        }
      }
    }
    expect(compared).toBeGreaterThan(2 * 700);
  });

  it("writes a minus as U+2212 in a hint's numbers", () => {
    expect([withMinusSign(formatNumber(-2, "en")), withMinusSign(formatNumber(-2.5, "de")), withMinusSign(formatNumber(3, "de"))]).toEqual(["−2", "−2,5", "3"]);
  });

  it("reads back every prefixed length Intl.NumberFormat writes in its unit style, centi and milli included", () => {
    const quantity = { unit: "m", prefixed: true, scale: "logarithmic" } as const;
    const scales = { millimeter: 1e-3, centimeter: 1e-2, meter: 1, kilometer: 1e3 } as const;
    let compared = 0;
    for (const locale of QUIZ_LOCALES)
      for (const [unit, scale] of Object.entries(scales))
        for (const amount of [1, 4, 30, 250]) {
          const written = new Intl.NumberFormat(locale, { style: "unit", unit, unitDisplay: "short" }).format(amount);
          expect(parseQuantity(written, quantity, locale), `${written} in ${locale}`).toBe(Number(`${amount}e${Math.log10(scale)}`));
          compared += 1;
        }
    expect(compared).toBe(2 * 4 * 4);
  });
});
