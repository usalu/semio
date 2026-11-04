/** 🏷️ Localized-label law: the shared fixture rows plus every generated stdio component descriptor. */
import { readdirSync, readFileSync, existsSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { blankLocalizedLabelPathsV1 } from "../../🏷️localized-labels/🟦️.ts";

const fixture = JSON.parse(readFileSync(new URL("../🧫️fixtures/🏷️localized-labels/🔣️.json", import.meta.url), "utf8")) as { cases: { case: string; value: unknown; blank: string[] }[] };
const stdio = fileURLToPath(new URL("../../../../../🌎️hub/🧩️compositions/🗄️stdio/", import.meta.url));

describe("localized label law", () => {
  it("answers every shared fixture row", () => {
    for (const row of fixture.cases) expect(blankLocalizedLabelPathsV1(row.value), row.case).toEqual(row.blank);
  });

  it("finds no blank en or de cell in any stdio component descriptor", () => {
    const components = [join(stdio, "🧩️component/🔣️.json"), ...readdirSync(join(stdio, "🧩️extensions")).map((name) => join(stdio, "🧩️extensions", name, "🧩️component/🔣️.json"))].filter(existsSync);
    expect(components.length).toBeGreaterThan(1);
    for (const file of components) expect(blankLocalizedLabelPathsV1(JSON.parse(readFileSync(file, "utf8"))), file).toEqual([]);
  });
});
