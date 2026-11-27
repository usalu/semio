/** 🏷️ The authoring-time law over every localized label of a manifest: each terminology row of a label carries a non-blank `en` and `de`. Twin of the Rust sweep in the stdio `shipped-fleet` test, driven by the shared `localized-labels` fixture. */

import axes from "../../🖱️ui/🎚️axes/🔣️.json" with { type: "json" };
import type { LocalizedLabel, ShellLocale, ShellTerminology } from "../🤖️generated/🎚️ui-axes/🟦️.ts";

/** 🌐️ The declared locale ids, read from the owned axes source so the closure of this module never reaches a generated output. */
const SHELL_LOCALES = axes.locales.map((row) => row.id) as readonly ShellLocale[];
/** 🗣️ The declared terminology ids, read from the owned axes source so the closure of this module never reaches a generated output. */
const SHELL_TERMINOLOGIES = axes.terminologies.map((row) => row.id) as readonly ShellTerminology[];

const isRecord = (value: unknown): value is Readonly<Record<string, unknown>> => typeof value === "object" && value !== null && !Array.isArray(value);

/** 🔎️ Whether an object is a localized label: non-empty, every value a locale map with string `en` and `de`. */
function isLocalizedLabelShape(value: Readonly<Record<string, unknown>>): boolean {
  const rows = Object.values(value);
  return rows.length > 0 && rows.every((row) => isRecord(row) && typeof row.en === "string" && typeof row.de === "string" && Object.values(row).every((cell) => typeof cell === "string"));
}

/** 🕳️ JSON-pointer-like paths of every localized label with a blank `en`/`de` cell. */
export function blankLocalizedLabelPathsV1(value: unknown, path = ""): readonly string[] {
  if (Array.isArray(value)) return value.flatMap((item, index) => blankLocalizedLabelPathsV1(item, `${path}[${index}]`));
  if (!isRecord(value)) return [];
  if (isLocalizedLabelShape(value)) return Object.values(value).some((row) => [(row as Record<string, string>).en, (row as Record<string, string>).de].some((cell) => cell!.trim() === "")) ? [path] : [];
  return Object.entries(value).flatMap(([key, child]) => blankLocalizedLabelPathsV1(child, `${path}/${key}`));
}

/** 🛂️ Admits exactly one full label matrix (every terminology × locale cell a string, no other key) — the `🖱️ui/🌐️locale/🧬️schema/🏷️localized-label` shape and the twin of Rust `LocalizedLabel`'s `Deserialize`; there is no default language. Blank cells are the authoring law's concern ({@link blankLocalizedLabelPathsV1}). */
export function parseLocalizedLabelV1(value: unknown, field: string): LocalizedLabel {
  if (!isRecord(value) || Object.keys(value).length !== SHELL_TERMINOLOGIES.length) throw new Error(`${field} must be a localized label`);
  return Object.fromEntries(SHELL_TERMINOLOGIES.map((terminology) => {
    const row = value[terminology];
    if (!isRecord(row) || Object.keys(row).length !== SHELL_LOCALES.length) throw new Error(`${field}.${terminology} must name every locale`);
    return [terminology, Object.fromEntries(SHELL_LOCALES.map((locale) => {
      const cell = row[locale];
      if (typeof cell !== "string") throw new Error(`${field}.${terminology}.${locale} must be a text`);
      return [locale, cell];
    }))];
  })) as LocalizedLabel;
}
