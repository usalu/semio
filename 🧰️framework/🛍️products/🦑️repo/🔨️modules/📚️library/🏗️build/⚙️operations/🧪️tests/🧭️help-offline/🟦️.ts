import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { resolve } from "node:path";
import catalog from "../../🔣️catalog.json" with { type: "json" };

const root = resolve(import.meta.dir, "../../../../../../../../.."), entry = resolve(import.meta.dir, "../../../📜️script.ts");
const environment = Object.fromEntries(Object.entries(process.env).filter(([key]) => !key.startsWith("SEMIO_SCRIPT") && !key.startsWith("SEMIO_BUILD_INV") && key !== "SEMIO_LANGUAGE"));

const help = (operation: string, language: string): string => {
  const result = spawnSync(process.execPath, [entry, "repo", operation, "--help", `--lang=${language}`], { cwd: root, encoding: "utf8", timeout: 30_000, env: environment });
  expect(result.error, operation).toBeUndefined();
  expect(result.status, `${operation} ${language}: ${result.stderr}`).toBe(0);
  return result.stdout;
};

test("every operation of the catalogue prints its own help in every catalogue language without running anything", () => {
  for (const row of catalog.operations) {
    const texts = catalog.languages.map((language) => {
      const text = help(row.name, language);
      expect(text.split("\n")[0], `${row.name} ${language}`).toBe(`repo ${row.name} - ${(row.summary as Record<string, string>)[language]}`);
      return text;
    });
    expect(new Set(texts).size, row.name).toBe(texts.length);
  }
}, 120_000);
