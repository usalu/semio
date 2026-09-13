import { readFileSync } from "node:fs";
import { join } from "node:path";

export interface UiAxisEntry {
  readonly id: string;
  readonly variant: string;
  readonly label?: string;
}

export interface UiAxes {
  readonly locales: readonly UiAxisEntry[];
  readonly terminologies: readonly UiAxisEntry[];
}

function validateAxis(name: string, value: unknown): readonly UiAxisEntry[] {
  if (!Array.isArray(value) || value.length === 0) throw new Error(`UI ${name} must be a non-empty array`);
  const entries = value.map((entry, index) => {
    if (!entry || typeof entry !== "object") throw new Error(`UI ${name}[${index}] must be an object`);
    const candidate = entry as Record<string, unknown>;
    if (typeof candidate.id !== "string" || !/^[a-z][a-z0-9-]*$/.test(candidate.id)) throw new Error(`UI ${name}[${index}].id is invalid`);
    if (typeof candidate.variant !== "string" || !/^[A-Z][A-Za-z0-9]*$/.test(candidate.variant)) throw new Error(`UI ${name}[${index}].variant is invalid`);
    if (candidate.label !== undefined && typeof candidate.label !== "string") throw new Error(`UI ${name}[${index}].label is invalid`);
    return { id: candidate.id, variant: candidate.variant, ...(candidate.label === undefined ? {} : { label: candidate.label }) };
  });
  if (new Set(entries.map(({ id }) => id)).size !== entries.length) throw new Error(`UI ${name} ids must be unique`);
  if (new Set(entries.map(({ variant }) => variant)).size !== entries.length) throw new Error(`UI ${name} variants must be unique`);
  return entries;
}

/** 🎚️ Validates the language-neutral locale and terminology source. */
export function parseUiAxes(value: unknown): UiAxes {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error("UI axes must be an object");
  const source = value as Record<string, unknown>;
  const keys = Object.keys(source).sort();
  if (keys.join("\0") !== ["locales", "terminologies"].join("\0")) throw new Error("UI axes must contain only locales and terminologies");
  return { locales: validateAxis("locales", source.locales), terminologies: validateAxis("terminologies", source.terminologies) };
}

/** 📥️ Reads the neutral UI axis catalog from its semantic owner. */
export function readUiAxes(repoRoot: string): UiAxes {
  return parseUiAxes(JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎚️axes/🔣️.json"), "utf8")));
}
