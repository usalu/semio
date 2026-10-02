import axes from "../🎚️axes/🔣️.json";

export type LabelValue = { readonly kind: "null" } | { readonly kind: "boolean"; readonly value: boolean } | { readonly kind: "number"; readonly value: number } | { readonly kind: "string"; readonly value: string } | { readonly kind: "array"; readonly items: readonly LabelValue[] } | { readonly kind: "object"; readonly entries: readonly (readonly [string, LabelValue])[] };

/** 🚫️ Reports an owned semantic path without exposing an external validation interface. */
export class LocaleContractError extends Error {
  constructor(readonly path: string, reason: string) { super(path + ": " + reason); }
}

/** 🌐️ Requires a declared locale selected by its caller. */
export class Locale {
  private constructor(readonly id: string) {}
  static parse(value: unknown): Locale {
    if (typeof value !== "string" || !axes.locales.some(row => row.id === value)) throw new LocaleContractError("locale", "an explicit declared locale is required");
    return new Locale(value);
  }
  static fromLanguageTag(value: unknown): Locale {
    if (typeof value !== "string") throw new LocaleContractError("locale", "an explicit language tag is required");
    let language: string;
    try { language = new Intl.Locale(value).language; } catch { throw new LocaleContractError("locale", "malformed language tag"); }
    return Locale.parse(language);
  }
}

/** 🗣️ Requires a declared terminology selected by its caller. */
export class Terminology {
  private constructor(readonly id: string) {}
  static parse(value: unknown): Terminology {
    if (typeof value !== "string" || !axes.terminologies.some(row => row.id === value)) throw new LocaleContractError("terminology", "an explicit declared terminology is required");
    return new Terminology(value);
  }
}

function fields(value: LabelValue, allowed: readonly string[], path: string): Map<string, LabelValue> {
  if (value.kind !== "object") throw new LocaleContractError(path || "$", "expected an object");
  const result = new Map<string, LabelValue>();
  for (const [key, child] of value.entries) {
    const childPath = path ? path + "." + key : key;
    if (!allowed.includes(key)) throw new LocaleContractError(childPath, "unknown field");
    if (result.has(key)) throw new LocaleContractError(childPath, "duplicate field");
    result.set(key, child);
  }
  for (const key of allowed) if (!result.has(key)) throw new LocaleContractError(path ? path + "." + key : key, "missing field");
  return result;
}

/** 🏷️ Admits exactly the complete locale and terminology matrix, including duplicate-key refusal. */
export class LocalizedLabel {
  private constructor(private readonly cells: ReadonlyMap<string, string>) {}
  static fromValue(value: LabelValue): LocalizedLabel {
    const rows = fields(value, axes.terminologies.map(row => row.id), ""), cells = new Map<string, string>();
    for (const terminology of axes.terminologies) {
      const row = fields(rows.get(terminology.id)!, axes.locales.map(row => row.id), terminology.id);
      for (const locale of axes.locales) {
        const key = terminology.id + "." + locale.id, text = row.get(locale.id)!;
        if (text.kind !== "string") throw new LocaleContractError(key, "expected text");
        cells.set(key, text.value);
      }
    }
    return new LocalizedLabel(cells);
  }
  resolve(terminology: Terminology, locale: Locale): string {
    if (!(locale instanceof Locale)) throw new LocaleContractError("locale", "an explicit declared locale is required");
    if (!(terminology instanceof Terminology)) throw new LocaleContractError("terminology", "an explicit declared terminology is required");
    return this.cells.get(terminology.id + "." + locale.id)!;
  }
}
