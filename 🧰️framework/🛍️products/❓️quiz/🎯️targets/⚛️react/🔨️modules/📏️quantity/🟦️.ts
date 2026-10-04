/** 📏️ Locale-aware display of measured values, scores, points, counts in words and instants.
 *
 * A {@link Quantity} with `prefixed` scales its unit with SI prefixes (W → kW → MW …) so every value shows a mantissa in
 * [1, 1000) with at most {@link SIGNIFICANT_DIGITS} significant digits — the engineering notation `Intl.NumberFormat`
 * itself computes, which the tests use as the oracle. Unprefixed values keep their authored precision.
 *
 * @see https://www.bipm.org/en/publications/si-brochure — SI prefixes and the space between value and unit
 */

import type { Quantity } from "@semio-tech/quiz";
import type { QuizLocale } from "../🌐️i18n/🟦️.ts";

/** 🎚️ Significant digits of an SI-prefixed mantissa. */
export const SIGNIFICANT_DIGITS = 4;

/** 🔠️ SI prefixes by power-of-ten exponent (multiples of three from quecto to quetta). */
export const SI_PREFIXES: Readonly<Record<number, string>> = {
  [-30]: "q",
  [-27]: "r",
  [-24]: "y",
  [-21]: "z",
  [-18]: "a",
  [-15]: "f",
  [-12]: "p",
  [-9]: "n",
  [-6]: "µ",
  [-3]: "m",
  0: "",
  3: "k",
  6: "M",
  9: "G",
  12: "T",
  15: "P",
  18: "E",
  21: "Z",
  24: "Y",
  27: "R",
  30: "Q",
};

const EXPONENT_LIMIT = 30;
const UNSPACED_UNITS: ReadonlySet<string> = new Set(["°", "′", "″"]);
const formatters = new Map<string, Intl.NumberFormat>();

function formatter(locale: QuizLocale, options: Intl.NumberFormatOptions): Intl.NumberFormat {
  const key = `${locale}${JSON.stringify(options)}`;
  const cached = formatters.get(key);
  if (cached) return cached;
  const created = new Intl.NumberFormat(locale, options);
  formatters.set(key, created);
  return created;
}

/** ✂️ The shortest decimal that round-trips `magnitude` (> 0), rounded half away from zero to `digits` significant
 * digits: the digit string and the power of ten of its first digit. */
function roundedDecimal(magnitude: number, digits: number): { readonly digits: string; readonly power: number } {
  const [coefficient = "0", exponent = "0"] = magnitude.toExponential().split("e");
  const shortest = coefficient.replace(".", "");
  const kept = shortest.slice(0, digits).padEnd(digits, "0");
  if ((shortest[digits] ?? "0") < "5") return { digits: kept, power: Number(exponent) };
  const carried = (BigInt(kept) + 1n).toString();
  return carried.length > digits ? { digits: carried.slice(0, digits), power: Number(exponent) + 1 } : { digits: carried, power: Number(exponent) };
}

/** ⚖️ `value` rounded to `digits` significant digits and written as `mantissa × 10^exponent` with `exponent` a multiple
 * of three within the SI prefixes, so the mantissa lies in [1, 1000) — unless the prefixes run out or `value` is zero.
 * Rounding works on the shortest round-trip decimal, half away from zero, and happens before the exponent is chosen, so
 * 999.96 becomes 1 × 10³, never 1000 × 10⁰ — the same decimal arithmetic `Intl.NumberFormat` uses. */
export function engineering(value: number, digits: number = SIGNIFICANT_DIGITS): { readonly mantissa: number; readonly exponent: number } {
  if (value === 0 || !Number.isFinite(value)) return { mantissa: value, exponent: 0 };
  const rounded = roundedDecimal(Math.abs(value), digits);
  const exponent = Math.max(-EXPONENT_LIMIT, Math.min(EXPONENT_LIMIT, Math.floor(rounded.power / 3) * 3));
  const mantissa = Number(`${rounded.digits[0]}.${rounded.digits.slice(1)}e${rounded.power - exponent}`);
  return { mantissa: Math.sign(value) * mantissa, exponent };
}

/** 🔢️ A plain number in `locale`: authored precision (up to six decimals) between 10⁻³ and 10¹⁵, scientific beyond. */
export function formatNumber(value: number, locale: QuizLocale): string {
  const magnitude = Math.abs(value);
  if (value === 0 || (magnitude >= 1e-3 && magnitude < 1e15)) return formatter(locale, { maximumFractionDigits: 6 }).format(value);
  return formatter(locale, { notation: "scientific", maximumSignificantDigits: SIGNIFICANT_DIGITS }).format(value);
}

/** 🔗️ Joins a formatted number and a unit symbol with a no-break space (none before degree and arc signs). */
export function withUnit(number: string, unit: string): string {
  if (unit === "") return number;
  return UNSPACED_UNITS.has(unit) ? `${number}${unit}` : `${number}\u00a0${unit}`;
}

/** 🏷️ `value` of `quantity` in `locale`, SI-prefixed when the quantity asks for it. */
export function formatQuantity(value: number, quantity: Pick<Quantity, "unit" | "prefixed">, locale: QuizLocale): string {
  if (!quantity.prefixed) return withUnit(formatNumber(value, locale), quantity.unit);
  const { mantissa, exponent } = engineering(value);
  const prefix = SI_PREFIXES[exponent] ?? "";
  return withUnit(formatter(locale, { maximumSignificantDigits: SIGNIFICANT_DIGITS }).format(mantissa), `${prefix}${quantity.unit}`);
}

/** 🔤️ Every SI prefix a typed value may carry, by its power of ten: the shown ones, the four that are no multiple of
 * three (`h`, `da`, `d`, `c`) and the spellings `u` and `μ` for micro and `K` for kilo. */
const PREFIX_EXPONENTS: ReadonlyMap<string, number> = new Map([
  ...Object.entries(SI_PREFIXES)
    .filter(([, symbol]) => symbol !== "")
    .map(([exponent, symbol]): [string, number] => [symbol, Number(exponent)]),
  ["h", 2],
  ["da", 1],
  ["d", -1],
  ["c", -2],
  ["u", -6],
  ["μ", -6],
  ["K", 3],
]);

const TYPED_NUMBER = /^([+-])?(\d[\d.,\s'’]*|[.,]\d+)(?:[eE]([+-]?\d+))?\s*(.*)$/su;

/** 🔣️ The group mark of `locale`'s numbers. */
function groupMark(locale: QuizLocale): string {
  return formatter(locale, {}).formatToParts(11111.1).find((part) => part.type === "group")?.value ?? ",";
}

/** 🧹️ The digits of a typed mantissa as a plain decimal string, or `undefined` when it is no number: spaces and
 * apostrophes group by three digits; of `.` and `,` the one occurring last is the decimal mark when both occur, a
 * repeated one groups, and a single one groups only when it is the locale's group mark between three digits. */
function plainMantissa(typed: string, locale: QuizLocale): string | undefined {
  const raw = typed.trim();
  if (/[\s'’](?!\d{3}(?!\d))/u.test(raw)) return undefined;
  const compact = raw.replace(/[\s'’]/gu, "");
  const kinds = [...new Set(compact.match(/[.,]/gu))];
  const only = kinds[0] ?? "";
  const grouped = kinds.length === 1 && (compact.split(only).length > 2 || (only === groupMark(locale) && /^\d{1,3}[.,]\d{3}$/u.test(compact)));
  const mark = kinds.length === 2 ? compact[Math.max(compact.lastIndexOf("."), compact.lastIndexOf(","))]! : grouped ? "" : only;
  const split = mark === "" ? compact.length : compact.lastIndexOf(mark);
  const whole = compact.slice(0, split);
  const fraction = compact.slice(split + 1);
  if (/[.,]/u.test(whole) ? !/^\d{1,3}(?:([.,])\d{3}(?:\1\d{3})*)?$/u.test(whole) : !/^\d*$/u.test(whole)) return undefined;
  if (!/^\d*$/u.test(fraction) || (whole === "" && fraction === "")) return undefined;
  const digits = whole.replace(/[.,]/gu, "") || "0";
  return fraction === "" ? digits : `${digits}.${fraction}`;
}

/** 🧮️ The power of ten of the SI prefix in a typed unit part (`k`, `kW`, `W`, empty), or `undefined` when it is none.
 * The unit in its own case comes first, then a prefix before the unit in any case, then the unit in any case, then a
 * prefix alone — unless that prefix is also the unit in another case (`M` for metres), which reads as neither. */
function typedExponent(typed: string, quantity: Pick<Quantity, "unit" | "prefixed">): number | undefined {
  const text = typed.trim();
  if (text === "" || text === quantity.unit) return 0;
  const unit = quantity.unit.toLowerCase();
  const head = text.toLowerCase().endsWith(unit) ? text.slice(0, text.length - unit.length).trim() : undefined;
  if (head !== undefined && head !== "") return quantity.prefixed ? PREFIX_EXPONENTS.get(head) : undefined;
  const alone = quantity.prefixed ? PREFIX_EXPONENTS.get(text) : undefined;
  if (head === "") return alone === undefined ? 0 : undefined;
  return alone;
}

/** 🔍️ The value a learner typed for `quantity` in `locale`: a number with optional exponent (`1.5e3`) and optional SI
 * prefix and unit (`2 kW`, `5 M`, `3,5 MWh`), in the unit's base — the inverse of {@link formatQuantity}. The decimal
 * mark of `locale` is read as such, a lone `.` or `,` elsewhere too unless it groups three digits. `undefined` when the
 * text is no such number, or not positive on a logarithmic scale. */
export function parseQuantity(text: string, quantity: Pick<Quantity, "unit" | "prefixed" | "scale">, locale: QuizLocale): number | undefined {
  const match = TYPED_NUMBER.exec(text.trim());
  if (match === null) return undefined;
  const [, sign = "", typed = "", exponent = "0", unit = ""] = match;
  const mantissa = plainMantissa(typed, locale);
  const prefix = typedExponent(unit, quantity);
  if (mantissa === undefined || prefix === undefined) return undefined;
  const value = Number(`${sign}${mantissa}e${Number(exponent) + prefix}`);
  return Number.isFinite(value) && (quantity.scale !== "logarithmic" || value > 0) ? value : undefined;
}

/** 💯️ A score in [0, 1] as a percentage in `locale`: whole when whole, else one decimal ({@link oneDecimal}), so 0.997
 * reads 99.7 % and only a perfect score reads 100 %. */
export function formatScore(score: number, locale: QuizLocale): string {
  return formatter(locale, { style: "percent", maximumFractionDigits: 1 }).format(oneDecimal(score * 100) / 100);
}

/** 🏅️ Points (a run's score × the par of its challenge, and their sums over quizzes) in `locale`, with the rounding of
 * {@link oneDecimal}. */
export function formatPoints(points: number, locale: QuizLocale): string {
  return formatter(locale, { maximumFractionDigits: 1 }).format(oneDecimal(points));
}

/** 🪚️ A positive finite `value` cut down to `digits` significant digits of its shortest round-trip decimal, so it never
 * exceeds `value`: 1.96 becomes 1.9 at two digits, 1000 stays 1000. Other values are returned as they are. */
export function floorSignificant(value: number, digits: number): number {
  if (!(value > 0) || !Number.isFinite(value)) return value;
  const [coefficient = "0", exponent = "0"] = value.toExponential().split("e");
  const kept = coefficient.replace(".", "").slice(0, digits);
  return Number(`${kept}e${Number(exponent) - kept.length + 1}`);
}

/** 🪜️ A positive finite `value` raised to the next decimal of `digits` significant digits, so it never falls below
 * `value`: 1.91 becomes 2.0 at two digits, 1.9 stays 1.9. Other values are returned as they are. */
export function ceilSignificant(value: number, digits: number): number {
  const floored = floorSignificant(value, digits);
  if (!(value > 0) || !Number.isFinite(value) || floored >= value) return floored;
  const [coefficient = "0", exponent = "0"] = floored.toExponential().split("e");
  const kept = coefficient.replace(".", "").padEnd(digits, "0").slice(0, digits);
  return Number(`${Number(kept) + 1}e${Number(exponent) - digits + 1}`);
}

/** ✖️ A factor (such as how far a value may lie off) in `locale` at two significant digits, cut down by `round` —
 * {@link floorSignificant} for a bound, so it never claims more than it is (1000 reads 1,000, a reach of 1.96 reads
 * 1.9), {@link ceilSignificant} for a value that lies beyond a bound. */
export function formatFactor(factor: number, locale: QuizLocale, round: (value: number, digits: number) => number = floorSignificant): string {
  return formatter(locale, { maximumSignificantDigits: 2 }).format(round(factor, 2));
}

/** 🧭️ Which way a shown number is cut: `down` never shows more than it is, `up` never less. */
export type Toward = "down" | "up";

/** 🔟️ The scale words of a count by power of ten, largest first, in singular and plural: the grammar of a number in
 * each language (a German scale word agrees with its number, an English one never takes a plural), not a plural label. */
const SCALE_WORDS: { readonly [L in QuizLocale]: readonly (readonly [power: number, one: string, many: string])[] } = {
  en: [
    [12, "trillion", "trillion"],
    [9, "billion", "billion"],
    [6, "million", "million"],
  ],
  de: [
    [12, "Billion", "Billionen"],
    [9, "Milliarde", "Milliarden"],
    [6, "Million", "Millionen"],
  ],
};

/** 🔁️ A count as a multiplier: English "N times"; German "N-mal" after digits, "N Mal" after a scale word. */
const TIMES: { readonly [L in QuizLocale]: (count: string, scaled: boolean) => string } = {
  en: (count) => `${count} times`,
  de: (count, scaled) => (scaled ? `${count} Mal` : `${count}-mal`),
};

const SUPERSCRIPT_DIGITS = "⁰¹²³⁴⁵⁶⁷⁸⁹";

/** 🧾️ A count in words of `locale` and whether it ends in a scale word. */
function counted(count: number, locale: QuizLocale, toward: Toward): { readonly text: string; readonly scaled: boolean } {
  const shown = (toward === "down" ? floorSignificant : ceilSignificant)(count, 2);
  const plain = formatter(locale, { maximumSignificantDigits: 2 });
  if (!(shown >= 1e6) || !Number.isFinite(shown)) return { text: plain.format(shown), scaled: false };
  const [coefficient = "1", exponent = "0"] = shown.toExponential().split("e");
  if (shown >= 1e15) return { text: `${plain.format(Number(coefficient))} × 10${[...String(Number(exponent))].map((digit) => SUPERSCRIPT_DIGITS[Number(digit)]).join("")}`, scaled: false };
  const [power, one, many] = SCALE_WORDS[locale].find(([candidate]) => Number(exponent) >= candidate)!;
  const mantissa = Number(`${coefficient}e${Number(exponent) - power}`);
  return { text: `${plain.format(mantissa)} ${mantissa === 1 ? one : many}`, scaled: true };
}

/** 📈️ A count (how many of one thing make another, how many times as high) in words of `locale`, cut `toward` one side
 * at two significant digits: plain with the locale's grouping below 10⁶ ("1,200"), with a scale word below 10¹⁵ ("1.2
 * million", "1 Million", "4,9 Billionen"), and beyond as its two-digit mantissa times a power of ten with a superscript
 * exponent ("1.1 × 10¹⁶", "1,1 × 10¹⁶"), the parts joined by no-break spaces.
 *
 * @see https://www.duden.de/rechtschreibung/Million — German scale words agree in number
 * @see https://www.bipm.org/en/publications/si-brochure — a number times a power of ten, the × as multiplication sign */
export function formatCount(count: number, locale: QuizLocale, toward: Toward): string {
  return counted(count, locale, toward).text;
}

/** 🔂️ A count as a multiplier in `locale` ({@link formatCount}): "2.9 times", "1.2 million times"; German "2,9-mal",
 * "1,2 Millionen Mal", "1,1 × 10²⁶-mal".
 *
 * @see https://www.duden.de/rechtschreibung/Mal_Zeitpunkt — "-mal" after digits, "Mal" after a number word */
export function formatTimes(count: number, locale: QuizLocale, toward: Toward): string {
  const { text, scaled } = counted(count, locale, toward);
  return TIMES[locale](text, scaled);
}

/** ➖️ A formatted number with its minus signs as U+2212, the typographic minus. */
export function withMinusSign(number: string): string {
  return number.replace(/-/gu, "−");
}

/** ⏲️ A remaining time in milliseconds as `m:ss` in whole seconds rounded up, so the clock reads 0:00 only once the
 * time is up; never negative. */
export function formatCountdown(milliseconds: number): string {
  const seconds = Math.max(0, Math.ceil(milliseconds / 1000));
  return `${Math.floor(seconds / 60)}:${String(seconds % 60).padStart(2, "0")}`;
}

/** 🗣️ A remaining time in milliseconds in words of `locale`, in whole seconds rounded up — "2 minutes, 5 seconds",
 * "48 seconds", "1 minute" — with the plural forms of the locale's own unit names; never negative. */
export function formatDuration(milliseconds: number, locale: QuizLocale): string {
  const seconds = Math.max(0, Math.ceil(milliseconds / 1000));
  const unit = (amount: number, name: "minute" | "second"): string => formatter(locale, { style: "unit", unit: name, unitDisplay: "long" }).format(amount);
  const parts = [...(seconds >= 60 ? [unit(Math.floor(seconds / 60), "minute")] : []), ...(seconds % 60 > 0 || seconds < 60 ? [unit(seconds % 60, "second")] : [])];
  return listFormatter(locale).format(parts);
}

const listFormatters = new Map<QuizLocale, Intl.ListFormat>();

function listFormatter(locale: QuizLocale): Intl.ListFormat {
  const cached = listFormatters.get(locale) ?? new Intl.ListFormat(locale, { style: "long", type: "unit" });
  listFormatters.set(locale, cached);
  return cached;
}

/** 🎯️ `value` to one decimal, but only a whole value shows as whole: 99.96 becomes 99.9 (never 100) and 0.04 becomes
 * 0.1 (never 0), so a score short of perfect never reads as perfect. Float noise below 10⁻⁹ counts as whole. */
export function oneDecimal(value: number): number {
  const whole = Math.round(value);
  if (Math.abs(value - whole) < 1e-9) return whole;
  const rounded = Math.round(value * 10) / 10;
  if (!Number.isInteger(rounded)) return rounded;
  return rounded > value ? rounded - 0.1 : rounded + 0.1;
}

/** 🕰️ An instant (ms since the epoch) as a medium date with short time in `locale`. */
export function formatInstant(at: number, locale: QuizLocale): string {
  return new Intl.DateTimeFormat(locale, { dateStyle: "medium", timeStyle: "short" }).format(at);
}

/** ⏰️ An instant (ms since the epoch) as a short time of day in `locale`. */
export function formatClock(at: number, locale: QuizLocale): string {
  return new Intl.DateTimeFormat(locale, { timeStyle: "short" }).format(at);
}

/** 📅️ An instant (ms since the epoch) as a medium date in `locale`. */
export function formatDate(at: number, locale: QuizLocale): string {
  return new Intl.DateTimeFormat(locale, { dateStyle: "medium" }).format(at);
}
