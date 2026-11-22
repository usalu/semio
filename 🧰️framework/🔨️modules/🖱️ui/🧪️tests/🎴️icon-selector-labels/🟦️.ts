import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { contentHostsDe, contentHostsEn } from "../../🌐️i18n/🧩️content-hosts/🟦️.ts";
import { configurationDe, configurationEn } from "../../🌐️i18n/⚙️configuration/🟦️.ts";

const ui = resolve(import.meta.dir, "../..");
const law = JSON.parse(readFileSync(join(ui, "🧪️tests/🧫️fixtures/🎴️icon-selector-labels/🔣️.json"), "utf8")) as { modes: Record<string, { en: string; de: string }>; common: Record<string, { en: string; de: string }> };

test("the shared icon selector labels equal React's en and de bundles", () => {
  for (const [mode, text] of Object.entries(law.modes)) {
    expect((contentHostsEn.iconSelector.mode as Record<string, { label: { normal: string } }>)[mode]!.label.normal).toBe(text.en);
    expect((contentHostsDe.iconSelector.mode as Record<string, { label: { normal: string } }>)[mode]!.label.normal).toBe(text.de);
  }
  for (const [key, text] of Object.entries(law.common)) {
    expect((configurationEn.common as Record<string, { label: { normal: string } }>)[key]!.label.normal).toBe(text.en);
    expect((configurationDe.common as Record<string, { label: { normal: string } }>)[key]!.label.normal).toBe(text.de);
  }
});
