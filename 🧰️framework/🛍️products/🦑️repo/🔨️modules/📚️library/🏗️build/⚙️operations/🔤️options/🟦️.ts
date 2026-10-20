import catalogue from "../🔣️catalog.json" with { type: "json" };

/** 🚩️ The part of one catalogue flag that decides how its value is spelled. */
export type BuildOptionFlagV1 = Readonly<{ name: string; aliases?: readonly string[]; value: string }>;

/** 📚️ The part of the neutral operations catalogue that names the options. */
export type BuildOptionCatalogV1 = Readonly<{ flags: readonly BuildOptionFlagV1[] }>;

/** 🔤️ Rewrites every `--option value` of an option the catalogue declares with a required value (enumerations, numbers, paths, lists; the global `--tool`, `--workers` and `--lang` among them) to `--option=value`, under the canonical long name and before the `--` separator, so that every later reader sees one spelling; options that take their value only separately, optionally or not at all, and everything after `--` stay as typed. */
export function normalizeBuildOptionsV1(args: readonly string[], catalog: BuildOptionCatalogV1 = catalogue as BuildOptionCatalogV1): string[] {
  const required = new Map<string, string>();
  for (const flag of catalog.flags) if (flag.value === "required") for (const spelling of [flag.name, ...(flag.aliases ?? [])]) required.set(spelling, flag.name);
  const result: string[] = [];
  for (let index = 0; index < args.length; index++) {
    const argument = args[index]!, canonical = required.get(argument), next = args[index + 1];
    if (argument === "--") { result.push(...args.slice(index)); break; }
    if (canonical !== undefined && next !== undefined && next !== "--" && !next.startsWith("-")) { result.push(`${canonical}=${next}`); index++; }
    else result.push(argument);
  }
  return result;
}
