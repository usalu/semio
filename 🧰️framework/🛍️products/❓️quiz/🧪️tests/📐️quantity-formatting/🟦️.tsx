/** 📐️ Quantity formatting and parsing: the shared vectors, the SI prefix choice and locale digits against
 * `Intl.NumberFormat`'s own engineering notation as the third-party oracle, and typed guesses read back from what
 * `Intl.NumberFormat` writes.
 *
 * @see ../../🧫️fixtures/📐️quantity-formatting/🔣️.json
 */

import { describe, expect, it } from "vitest";
import type { Scale } from "@semio-tech/quiz";
import { QUIZ_LOCALES, SIGNIFICANT_DIGITS, SI_PREFIXES, engineering, formatNumber, formatPoints, formatQuantity, formatScore, parseQuantity, withUnit, type QuizLocale } from "@semio-tech/quiz-react";
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
});
