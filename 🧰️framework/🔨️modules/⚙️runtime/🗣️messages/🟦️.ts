import {noticeLanguagesV1} from "../🗣️language/🟦️.ts";
import messages from "../🧬️schema/🗣️messages.json" with { type: "json" };
import type { RuntimeFamilyV1, RuntimeIdV1 } from "../📋️contract/🟦️.ts";

const catalog = messages as Readonly<Record<string, Readonly<{ en: string; de: string }>>>;

/** 🗣️ Renders one catalog message in the language the language protocol chooses, or in every language when it chooses none; `{detail}` may name another catalog message. */
export function runtimeMessageV1(id: string, parameters: Readonly<Record<string, string | number>>, languages: readonly ("en" | "de")[] = noticeLanguagesV1(["en", "de"] as const)): string {
  const row = catalog[id];
  if (row === undefined) return `${id} ${JSON.stringify(parameters)}`;
  const render = (language: "en" | "de", entry: Readonly<{ en: string; de: string }>): string => entry[language].replace(/\{(\w+)\}/gu, (_match, key: string) => {
    const value = parameters[key];
    if (value === undefined) return `{${key}}`;
    const nested = key === "detail" ? catalog[String(value)] : undefined;
    return nested === undefined ? String(value) : render(language, nested);
  });
  return languages.map((language) => render(language, row)).join(" / ");
}

/** 🚫️ Refusal raised when an adapter does not provide a requested family, or provides it only through a provider that is not installed. */
export class RuntimeCapabilityMissingV1 extends Error {
  readonly family: RuntimeFamilyV1;
  readonly runtime: RuntimeIdV1;
  constructor(family: RuntimeFamilyV1, runtime: RuntimeIdV1, reason?: string) { super(runtimeMessageV1(reason === undefined ? "capabilityMissing" : "capabilityMissingReason", { runtime, family, reason: reason ?? "" })); this.name = "RuntimeCapabilityMissingV1"; this.family = family; this.runtime = runtime; }
}
