import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { classifyIconSelectorMode, type IconSelectorMode } from "../../🧱️elements/🔣️Icons/🟦️.tsx";
import { migrateIconKindToIconSelectorMode } from "../../🧱️elements/🎴️IconSelector/🟦️.tsx";

const ui = resolve(import.meta.dir, "../..");
const law = JSON.parse(readFileSync(join(ui, "🧪️tests/🧫️fixtures/🎴️icon-selector-mode/🔣️.json"), "utf8")) as { cases: { id: string; raw: string; mode: IconSelectorMode }[]; migrate: { id: string; prev: string; mode: IconSelectorMode; stored: string }[] };

for (const row of law.cases) test("the actual React classifyIconSelectorMode reads " + row.id, () => {
  expect(classifyIconSelectorMode(row.raw)).toBe(row.mode);
});

for (const row of law.migrate) test("the actual React migrateIconKindToIconSelectorMode stores " + row.id, () => {
  expect(migrateIconKindToIconSelectorMode(row.prev, row.mode, classifyIconSelectorMode)).toBe(row.stored);
});
