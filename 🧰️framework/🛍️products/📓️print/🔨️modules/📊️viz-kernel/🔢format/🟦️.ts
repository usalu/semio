/** 🔢️ Number and time formatting: the TypeScript twin of `semio-viz-format`. The full d3-format
 * specifier grammar (`fill align sign symbol zero width comma precision ~ type`), the SI prefix
 * table, and the d3-time-format directives, each in the document languages `en` and `de` — there
 * is no default language.
 * @see ../../../🖋️latex/semio-viz-format.sty
 */
import type { VizLanguage } from "../🧬️schema/🟦️.ts";

//#region 🔖️Specifier
/** 🔢️ A parsed format specifier, every field carrying the default d3 assigns when it is absent. */
export type VizFormatSpecifier = {
  readonly fill: string;
  readonly align: "<" | ">" | "=" | "^";
  readonly sign: "-" | "+" | "(" | " ";
  readonly symbol: "" | "$" | "#";
  readonly zero: boolean;
  readonly width?: number;
  readonly comma: boolean;
  readonly precision?: number;
  readonly trim: boolean;
  readonly type: string;
};

const SPECIFIER_PATTERN = /^(?:(.)?([<>=^]))?([+\-( ])?([$#])?(0)?(\d+)?(,)?(\.\d+)?(~)?([a-z%])?$/i;

/** 🔢️ Parses a d3-format specifier; an unparsable one is an error, never a silent default. */
export function parseVizFormatSpecifier(specifier: string): VizFormatSpecifier {
  const match = SPECIFIER_PATTERN.exec(specifier);
  if (match === null) throw new Error(`invalid format specifier: ${specifier}`);
  const [, fill, align, sign, symbol, zero, width, comma, precision, trim, type] = match;
  return {
    fill: fill ?? " ",
    align: (align ?? ">") as VizFormatSpecifier["align"],
    sign: (sign ?? "-") as VizFormatSpecifier["sign"],
    symbol: (symbol ?? "") as VizFormatSpecifier["symbol"],
    zero: zero !== undefined,
    width: width === undefined ? undefined : Number(width),
    comma: comma !== undefined,
    precision: precision === undefined ? undefined : Number(precision.slice(1)),
    trim: trim !== undefined,
    type: type ?? "",
  };
}

/** 🔢️ Renders a specifier back to its canonical text, the way d3's `formatSpecifier` prints. */
export function formatVizSpecifier(spec: VizFormatSpecifier): string {
  return `${spec.fill}${spec.align}${spec.sign}${spec.symbol}${spec.zero ? "0" : ""}${spec.width === undefined ? "" : Math.max(1, Math.min(21, spec.width))}${spec.comma ? "," : ""}${spec.precision === undefined ? "" : `.${Math.max(0, Math.min(20, spec.precision))}`}${spec.trim ? "~" : ""}${spec.type}`;
}
//#endregion 🔖️Specifier

//#region 🔖️Locale
/** 🌍️ The numeric conventions of one document language. */
export type VizNumberLocale = {
  readonly decimal: string;
  readonly thousands: string;
  readonly grouping: readonly number[];
  readonly currency: readonly [string, string];
  readonly minus: string;
  readonly percent: string;
  readonly nan: string;
};

/** 🌍️ Number locales for every language the print product ships. */
export const VIZ_NUMBER_LOCALES: Readonly<Record<VizLanguage, VizNumberLocale>> = {
  en: { decimal: ".", thousands: ",", grouping: [3], currency: ["$", ""], minus: "−", percent: "%", nan: "NaN" },
  de: { decimal: ",", thousands: ".", grouping: [3], currency: ["", " €"], minus: "−", percent: "%", nan: "NaN" },
};

/** 🌍️ The calendar names of one document language. */
export type VizTimeLocale = {
  readonly dateTime: string;
  readonly date: string;
  readonly time: string;
  readonly periods: readonly [string, string];
  readonly days: readonly string[];
  readonly shortDays: readonly string[];
  readonly months: readonly string[];
  readonly shortMonths: readonly string[];
};

/** 🌍️ Time locales for every language the print product ships. */
export const VIZ_TIME_LOCALES: Readonly<Record<VizLanguage, VizTimeLocale>> = {
  en: {
    dateTime: "%x, %X",
    date: "%-m/%-d/%Y",
    time: "%-I:%M:%S %p",
    periods: ["AM", "PM"],
    days: ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"],
    shortDays: ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"],
    months: ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"],
    shortMonths: ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"],
  },
  de: {
    dateTime: "%A, der %e. %B %Y, %X",
    date: "%d.%m.%Y",
    time: "%H:%M:%S",
    periods: ["AM", "PM"],
    days: ["Sonntag", "Montag", "Dienstag", "Mittwoch", "Donnerstag", "Freitag", "Samstag"],
    shortDays: ["So", "Mo", "Di", "Mi", "Do", "Fr", "Sa"],
    months: ["Januar", "Februar", "März", "April", "Mai", "Juni", "Juli", "August", "September", "Oktober", "November", "Dezember"],
    shortMonths: ["Jan", "Feb", "Mär", "Apr", "Mai", "Jun", "Jul", "Aug", "Sep", "Okt", "Nov", "Dez"],
  },
};
//#endregion 🔖️Locale

//#region 🔖️NumberEngine
/** 🔢️ The SI prefixes, indexed by `8 + exponent / 3` exactly like d3's table. */
export const VIZ_SI_PREFIXES: readonly string[] = ["y", "z", "a", "f", "p", "n", "µ", "m", "", "k", "M", "G", "T", "P", "E", "Z", "Y"];

function decimalParts(x: number, p?: number): [string, number] | null {
  const text = p === undefined ? x.toExponential() : x.toExponential(p - 1);
  const i = text.indexOf("e");
  if (i < 0) return null;
  const coefficient = text.slice(0, i);
  return [coefficient.length > 1 ? `${coefficient[0]}${coefficient.slice(2)}` : coefficient, Number(text.slice(i + 1))];
}

function formatDecimalInteger(x: number): string {
  const rounded = Math.round(x);
  return Math.abs(rounded) >= 1e21 ? rounded.toLocaleString("en").replace(/,/g, "") : rounded.toString(10);
}

function formatRounded(x: number, p: number): string {
  const parts = decimalParts(x, p);
  if (parts === null) return `${x}`;
  const [coefficient, exponent] = parts;
  if (exponent < 0) return `0.${"0".repeat(-exponent - 1)}${coefficient}`;
  if (coefficient.length > exponent + 1) return `${coefficient.slice(0, exponent + 1)}.${coefficient.slice(exponent + 1)}`;
  return `${coefficient}${"0".repeat(exponent - coefficient.length + 1)}`;
}

type PrefixState = { exponent: number };

function formatPrefixAuto(x: number, p: number, state: PrefixState): string {
  const parts = decimalParts(x, p);
  if (parts === null) return `${x}`;
  const [coefficient, exponent] = parts;
  state.exponent = Math.max(-8, Math.min(8, Math.floor(exponent / 3))) * 3;
  const i = exponent - state.exponent + 1;
  const n = coefficient.length;
  if (i === n) return coefficient;
  if (i > n) return `${coefficient}${"0".repeat(i - n)}`;
  if (i > 0) return `${coefficient.slice(0, i)}.${coefficient.slice(i)}`;
  return `0.${"0".repeat(-i)}${decimalParts(x, Math.max(0, p + i - 1))![0]}`;
}

function formatTrim(text: string): string {
  let i0 = -1;
  let i1 = 0;
  const n = text.length;
  for (let i = 1; i < n; i += 1) {
    const character = text[i]!;
    if (character === ".") {
      i0 = i;
      i1 = i;
    } else if (character === "0") {
      if (i0 === 0) i0 = i;
      i1 = i;
    } else {
      if (!Number(character)) break;
      if (i0 > 0) i0 = 0;
    }
  }
  return i0 > 0 ? `${text.slice(0, i0)}${text.slice(i1 + 1)}` : text;
}

function grouper(grouping: readonly number[], thousands: string): (value: string, width: number) => string {
  return (value, width) => {
    let i = value.length;
    const chunks: string[] = [];
    let j = 0;
    let g = grouping[0]!;
    let length = 0;
    while (i > 0 && g > 0) {
      if (length + g + 1 > width) g = Math.max(1, width - length);
      i -= g;
      chunks.push(value.substring(i, i + g));
      length += g + 1;
      if (length > width) break;
      j = (j + 1) % grouping.length;
      g = grouping[j]!;
    }
    return chunks.reverse().join(thousands);
  };
}

const NUMERIC_TYPES = new Set(["%", "b", "c", "d", "e", "f", "g", "o", "p", "r", "s", "X", "x"]);

/** 🔢️ Builds a number formatter for one specifier in one language. */
export function vizFormat(specifier: string, language: VizLanguage): (value: number) => string {
  const locale = VIZ_NUMBER_LOCALES[language];
  const parsed = parseVizFormatSpecifier(specifier);
  const group = grouper(locale.grouping, locale.thousands);
  const state: PrefixState = { exponent: 0 };
  let type = parsed.type;
  let comma = parsed.comma;
  let trim = parsed.trim;
  let precision = parsed.precision;
  let zero = parsed.zero;
  let fill = parsed.fill;
  let align: string = parsed.align;
  if (type === "n") {
    comma = true;
    type = "g";
  } else if (!NUMERIC_TYPES.has(type)) {
    if (precision === undefined) precision = 12;
    trim = true;
    type = "g";
  }
  if (zero || (fill === "0" && align === "=")) {
    zero = true;
    fill = "0";
    align = "=";
  }
  const prefix = parsed.symbol === "$" ? locale.currency[0] : parsed.symbol === "#" && /[boxX]/.test(type) ? `0${type.toLowerCase()}` : "";
  const suffix = parsed.symbol === "$" ? locale.currency[1] : /[%p]/.test(type) ? locale.percent : "";
  const resolved = precision === undefined ? 6 : /[gprs]/.test(type) ? Math.max(1, Math.min(21, precision)) : Math.max(0, Math.min(20, precision));
  const apply = (x: number, p: number): string => {
    switch (type) {
      case "%":
        return (x * 100).toFixed(p);
      case "b":
        return Math.round(x).toString(2);
      case "c":
        return `${x}`;
      case "d":
        return formatDecimalInteger(x);
      case "e":
        return x.toExponential(p);
      case "f":
        return x.toFixed(p);
      case "g":
        return x.toPrecision(p);
      case "o":
        return Math.round(x).toString(8);
      case "p":
        return formatRounded(x * 100, p);
      case "r":
        return formatRounded(x, p);
      case "s":
        return formatPrefixAuto(x, p, state);
      case "X":
        return Math.round(x).toString(16).toUpperCase();
      default:
        return Math.round(x).toString(16);
    }
  };
  const maybeSuffix = /[defgprs%]/.test(type);
  return (input: number) => {
    let valuePrefix = prefix;
    let valueSuffix = suffix;
    let value: string;
    if (type === "c") {
      valueSuffix = `${apply(input, resolved)}${valueSuffix}`;
      value = "";
    } else {
      const numeric = +input;
      let negative = numeric < 0 || 1 / numeric < 0;
      value = Number.isNaN(numeric) ? locale.nan : apply(Math.abs(numeric), resolved);
      if (trim) value = formatTrim(value);
      if (negative && Number(value) === 0 && parsed.sign !== "+") negative = false;
      valuePrefix = `${negative ? (parsed.sign === "(" ? "(" : locale.minus) : parsed.sign === "-" || parsed.sign === "(" ? "" : parsed.sign}${valuePrefix}`;
      valueSuffix = `${type === "s" ? VIZ_SI_PREFIXES[8 + state.exponent / 3]! : ""}${valueSuffix}${negative && parsed.sign === "(" ? ")" : ""}`;
      if (maybeSuffix) {
        for (let i = 0; i < value.length; i += 1) {
          const code = value.charCodeAt(i);
          if (code < 48 || code > 57) {
            valueSuffix = `${code === 46 ? `${locale.decimal}${value.slice(i + 1)}` : value.slice(i)}${valueSuffix}`;
            value = value.slice(0, i);
            break;
          }
        }
      }
    }
    if (comma && !zero) value = group(value, Number.POSITIVE_INFINITY);
    const width = parsed.width ?? 0;
    const length = valuePrefix.length + value.length + valueSuffix.length;
    let padding = length < width ? fill.repeat(width - length) : "";
    if (comma && zero) {
      value = group(`${padding}${value}`, padding.length > 0 ? width - valueSuffix.length : Number.POSITIVE_INFINITY);
      padding = "";
    }
    if (align === "<") return `${valuePrefix}${value}${valueSuffix}${padding}`;
    if (align === "=") return `${valuePrefix}${padding}${value}${valueSuffix}`;
    if (align === "^") {
      const half = padding.length >> 1;
      return `${padding.slice(0, half)}${valuePrefix}${value}${valueSuffix}${padding.slice(half)}`;
    }
    return `${padding}${valuePrefix}${value}${valueSuffix}`;
  };
}

/** 🔢️ The decimal exponent of a value, d3's internal `exponent`. */
export function vizExponent(value: number): number {
  return decimalParts(Math.abs(value))?.[1] ?? Number.NaN;
}

/** 🔢️ The SI-prefixed formatter pinned to one reference value, d3's `formatPrefix`. */
export function vizFormatPrefix(specifier: string, value: number, language: VizLanguage): (input: number) => string {
  const parsed = parseVizFormatSpecifier(specifier);
  const exponent = Math.max(-8, Math.min(8, Math.floor(vizExponent(value) / 3))) * 3;
  const k = 10 ** -exponent;
  const prefix = VIZ_SI_PREFIXES[8 + exponent / 3]!;
  const base = vizFormat(formatVizSpecifier({ ...parsed, type: "f" }), language);
  return (input: number) => `${base(k * input)}${prefix}`;
}

/** 🔢️ Suggested decimal precision for a fixed-notation step, d3's `precisionFixed`. */
export function vizPrecisionFixed(step: number): number {
  return Math.max(0, -vizExponent(step));
}

/** 🔢️ Suggested significant digits for a rounded step over a span, d3's `precisionRound`. */
export function vizPrecisionRound(step: number, max: number): number {
  const magnitude = Math.abs(step);
  return Math.max(0, vizExponent(Math.abs(max) - magnitude) - vizExponent(magnitude)) + 1;
}

/** 🔢️ Suggested significant digits for an SI-prefixed value, d3's `precisionPrefix`. */
export function vizPrecisionPrefix(step: number, value: number): number {
  return Math.max(0, Math.max(-8, Math.min(8, Math.floor(vizExponent(value) / 3))) * 3 - vizExponent(step));
}
//#endregion 🔖️NumberEngine

//#region 🔖️TimeEngine
function pad(value: number, fill: string, width: number): string {
  const negative = value < 0;
  const text = String(negative ? -value : value);
  return `${negative ? "-" : ""}${text.length < width ? `${fill.repeat(width - text.length)}${text}` : text}`;
}

function dayOfYear(date: Date): number {
  const start = new Date(date.getFullYear(), 0, 1);
  const days = Math.round((+new Date(date.getFullYear(), date.getMonth(), date.getDate()) - +start) / 86400000);
  return days;
}

function weekNumber(date: Date, firstDay: number): number {
  const jan1 = new Date(date.getFullYear(), 0, 1);
  const first = (7 - ((jan1.getDay() - firstDay + 7) % 7)) % 7;
  const day = dayOfYear(date);
  return day < first ? 0 : Math.floor((day - first) / 7) + 1;
}

function formatZone(date: Date): string {
  const offset = date.getTimezoneOffset();
  const minutes = Math.abs(offset);
  return `${offset > 0 ? "-" : "+"}${pad(Math.floor(minutes / 60), "0", 2)}${pad(minutes % 60, "0", 2)}`;
}

/** 🕰️ Builds a date formatter from a d3-time-format specifier in one document language. */
export function vizTimeFormat(specifier: string, language: VizLanguage): (date: Date) => string {
  const locale = VIZ_TIME_LOCALES[language];
  const directive = (code: string, padding: string, date: Date): string => {
    switch (code) {
      case "a":
        return locale.shortDays[date.getDay()]!;
      case "A":
        return locale.days[date.getDay()]!;
      case "b":
        return locale.shortMonths[date.getMonth()]!;
      case "B":
        return locale.months[date.getMonth()]!;
      case "c":
        return vizTimeFormat(locale.dateTime, language)(date);
      case "d":
        return pad(date.getDate(), padding, 2);
      case "e":
        return pad(date.getDate(), padding, 2);
      case "H":
        return pad(date.getHours(), padding, 2);
      case "I":
        return pad(date.getHours() % 12 === 0 ? 12 : date.getHours() % 12, padding, 2);
      case "j":
        return pad(dayOfYear(date) + 1, padding, 3);
      case "L":
        return pad(date.getMilliseconds(), padding, 3);
      case "m":
        return pad(date.getMonth() + 1, padding, 2);
      case "M":
        return pad(date.getMinutes(), padding, 2);
      case "p":
        return locale.periods[date.getHours() >= 12 ? 1 : 0]!;
      case "S":
        return pad(date.getSeconds(), padding, 2);
      case "u":
        return String(date.getDay() === 0 ? 7 : date.getDay());
      case "U":
        return pad(weekNumber(date, 0), padding, 2);
      case "w":
        return String(date.getDay());
      case "W":
        return pad(weekNumber(date, 1), padding, 2);
      case "x":
        return vizTimeFormat(locale.date, language)(date);
      case "X":
        return vizTimeFormat(locale.time, language)(date);
      case "y":
        return pad(((date.getFullYear() % 100) + 100) % 100, padding, 2);
      case "Y":
        return pad(date.getFullYear() % 10000, padding, 4);
      case "Z":
        return formatZone(date);
      case "%":
        return "%";
      default:
        throw new Error(`unknown time directive %${code}`);
    }
  };
  return (date: Date) => {
    let out = "";
    let i = 0;
    while (i < specifier.length) {
      if (specifier[i] !== "%") {
        out += specifier[i];
        i += 1;
        continue;
      }
      i += 1;
      let padding = specifier[i] === "e" ? " " : "0";
      if (specifier[i] === "-") {
        padding = "";
        i += 1;
      } else if (specifier[i] === "_") {
        padding = " ";
        i += 1;
      } else if (specifier[i] === "0") {
        padding = "0";
        i += 1;
      }
      const code = specifier[i]!;
      i += 1;
      out += directive(code, padding, date);
    }
    return out;
  };
}
//#endregion 🔖️TimeEngine

//#region 🔖️Dispatch
/** 🔢️ `\SemioVizFormat`: one value through one specifier in the document language. */
export function formatVizValue(specifier: string, value: number, language: VizLanguage): string {
  return vizFormat(specifier, language)(value);
}

/** 🕰️ `\SemioVizTimeFormat`: one instant through one specifier in the document language. */
export function formatVizTime(specifier: string, value: Date | number, language: VizLanguage): string {
  return vizTimeFormat(specifier, language)(value instanceof Date ? value : new Date(value));
}
//#endregion 🔖️Dispatch
