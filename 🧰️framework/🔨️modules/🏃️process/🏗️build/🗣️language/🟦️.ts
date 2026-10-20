import { spawnSync } from "node:child_process";
import protocol from "./🔣️.json" with { type: "json" };

/** 🌍️ The environment a language is read from. */
export type LanguageEnvironmentV1 = Readonly<Record<string, string | undefined>>;

/** 🧭️ Where the chosen language came from. */
export type LanguageResolutionV1 = Readonly<{ language: string; source: string }>;

let remembered: readonly string[] | undefined;

/** 🔌️ The language tags of the operating system in preference order, or none when it names none. */
export type OsLanguageListV1 = () => readonly string[];

/** 🔤️ Reads the language subtag of a locale or language tag such as `de_CH.UTF-8@euro` or `en-US`; the neutral locales `C` and `POSIX` carry none. */
export function languageSubtagV1(value: string | undefined): string | undefined {
  const tag = value?.trim().toLowerCase().split(/[._@-]/u)[0];
  return tag === undefined || tag === "" || protocol.neutralTags.includes(tag) ? undefined : tag;
}

/** 🏳️ Reads `--lang <value>` or `--lang=<value>` before the `--` separator. */
export function languageFlagV1(args: readonly string[]): string | undefined {
  const boundary = args.indexOf("--"), head = args.slice(0, boundary < 0 ? undefined : boundary);
  let explicit: string | undefined;
  for (let index = 0; index < head.length; index++) {
    const argument = head[index]!;
    if (argument === "--lang") explicit = head[++index] ?? "";
    else if (argument.startsWith("--lang=")) explicit = argument.slice(7);
  }
  return explicit;
}

/** 🖥️ Asks the operating system for its language list: macOS preferences, the Windows user locale; other systems state theirs in the environment. */
export const osLanguageListV1: OsLanguageListV1 = () => {
  if (remembered !== undefined) return remembered;
  if (process.platform === "darwin") {
    const row = protocol.osLocale.darwin, done = spawnSync(row.command, row.args, { encoding: "utf8", timeout: row.timeoutMs });
    return (remembered = done.status === 0 ? [...done.stdout.matchAll(/"?([A-Za-z]{2,3}(?:-[A-Za-z0-9]+)*)"?\s*[,)]/gu)].map((match) => match[1]!) : []);
  }
  return (remembered = process.platform === "win32" ? [Intl.DateTimeFormat().resolvedOptions().locale] : []);
};

/** 🚫️ The refusal that names every way to choose a language, in every offered language; it never depends on a language being chosen. */
export function languageRefusalV1(languages: readonly string[]): Error {
  const fill = (text: string): string => text.replaceAll("{choice}", languages.join("|"));
  return new Error(languages.map((language) => fill((protocol.refusal as unknown as Record<string, string>)[language] ?? protocol.refusal.en)).join("\n"));
}

function refuse(row: Readonly<Record<"en" | "de", string>>, languages: readonly string[], values: Readonly<Record<string, string>>): Error {
  const fill = (text: string): string => Object.entries(values).reduce((out, [key, value]) => out.replaceAll(`{${key}}`, value), text.replaceAll("{choice}", languages.join("|")));
  return new Error(languages.map((language) => fill(row[language as "en" | "de"] ?? row.en)).join("\n"));
}

/** 🗣️ Chooses the language of texts from the offered languages; no language is assumed. Order: `--lang`, `SEMIO_LANGUAGE` (both must name an offered language), `LANGUAGE` (first offered entry), the first non-empty of `LC_ALL`, `LC_MESSAGES`, `LANG`, then the operating system language list; a source that names no offered language is skipped, and when none resolves the bilingual refusal is thrown. */
export function resolveLanguageV1(languages: readonly string[], args: readonly string[], environment: LanguageEnvironmentV1, osLanguages: OsLanguageListV1 = osLanguageListV1): LanguageResolutionV1 {
  const flag = languageFlagV1(args);
  if (flag !== undefined) {
    if (!languages.includes(flag)) throw refuse(protocol.refusal.unknownFlag, languages, { value: flag });
    return { language: flag, source: "--lang" };
  }
  const explicit = environment[protocol.explicitVariable];
  if (explicit !== undefined && explicit !== "") {
    const tag = languageSubtagV1(explicit);
    if (tag === undefined || !languages.includes(tag)) throw refuse(protocol.refusal.unknownVariable, languages, { name: protocol.explicitVariable, value: explicit });
    return { language: tag, source: protocol.explicitVariable };
  }
  for (const entry of (environment[protocol.preferenceListVariable] ?? "").split(":")) {
    const tag = languageSubtagV1(entry);
    if (tag !== undefined && languages.includes(tag)) return { language: tag, source: protocol.preferenceListVariable };
  }
  const name = protocol.environmentOrder.find((candidate) => (environment[candidate] ?? "") !== "");
  if (name !== undefined) {
    const tag = languageSubtagV1(environment[name]);
    if (tag !== undefined && languages.includes(tag)) return { language: tag, source: name };
  }
  for (const entry of osLanguages()) {
    const tag = languageSubtagV1(entry);
    if (tag !== undefined && languages.includes(tag)) return { language: tag, source: "os" };
  }
  throw languageRefusalV1(languages);
}
